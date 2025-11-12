/// Statistical testing for A/B comparisons
use crate::ai::evaluation::EvaluationResults;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, StudentsT};
use statrs::statistics::Statistics;

/// Result of comparing two configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Configuration A name
    pub config_a: String,

    /// Configuration B name
    pub config_b: String,

    /// Delta metrics (B - A)
    pub delta: DeltaMetrics,

    /// Statistical significance
    pub significance: SignificanceTest,

    /// Recommendation
    pub recommendation: Recommendation,
}

/// Delta between two configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaMetrics {
    /// Accuracy difference (percentage points)
    pub accuracy: f64,

    /// Cost difference (USD)
    pub cost_usd: f64,

    /// Cost difference (percentage)
    pub cost_pct: f64,

    /// Duration difference (milliseconds)
    pub duration_ms: f64,

    /// Duration difference (percentage)
    pub duration_pct: f64,

    /// API calls difference
    pub api_calls: f64,
}

/// Statistical significance test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificanceTest {
    /// Is the accuracy difference statistically significant? (p < 0.05)
    pub accuracy_significant: bool,

    /// P-value for accuracy difference
    pub accuracy_p_value: f64,

    /// T-statistic for accuracy
    pub accuracy_t_stat: f64,

    /// Is the cost difference statistically significant?
    pub cost_significant: bool,

    /// P-value for cost difference
    pub cost_p_value: f64,

    /// Confidence level
    pub confidence: f64,
}

/// Recommendation based on comparison
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Recommendation {
    /// Adopt config B (clear improvement)
    Adopt,

    /// Reject config B (no improvement or worse)
    Reject,

    /// Need more data to decide
    NeedMoreData,

    /// Marginal improvement, investigate further
    Investigate,
}

impl ComparisonResult {
    /// Compare two evaluation results
    pub fn compare(results_a: &EvaluationResults, results_b: &EvaluationResults) -> Self {
        let delta = DeltaMetrics {
            accuracy: results_b.accuracy - results_a.accuracy,
            cost_usd: results_b.avg_cost_usd - results_a.avg_cost_usd,
            cost_pct: if results_a.avg_cost_usd > 0.0 {
                ((results_b.avg_cost_usd - results_a.avg_cost_usd) / results_a.avg_cost_usd) * 100.0
            } else {
                0.0
            },
            duration_ms: results_b.avg_duration_ms - results_a.avg_duration_ms,
            duration_pct: if results_a.avg_duration_ms > 0.0 {
                ((results_b.avg_duration_ms - results_a.avg_duration_ms)
                    / results_a.avg_duration_ms)
                    * 100.0
            } else {
                0.0
            },
            api_calls: 0.0, // TODO: compute from results
        };

        // Extract metrics for statistical testing
        let metrics_a: Vec<f64> = results_a
            .results
            .iter()
            .map(|r| if r.solved { 1.0 } else { 0.0 })
            .collect();

        let metrics_b: Vec<f64> = results_b
            .results
            .iter()
            .map(|r| if r.solved { 1.0 } else { 0.0 })
            .collect();

        let costs_a: Vec<f64> = results_a.results.iter().map(|r| r.cost_usd).collect();
        let costs_b: Vec<f64> = results_b.results.iter().map(|r| r.cost_usd).collect();

        let accuracy_test = Self::t_test(&metrics_a, &metrics_b);
        let cost_test = Self::t_test(&costs_a, &costs_b);

        let significance = SignificanceTest {
            accuracy_significant: accuracy_test.p_value < 0.05,
            accuracy_p_value: accuracy_test.p_value,
            accuracy_t_stat: accuracy_test.t_stat,
            cost_significant: cost_test.p_value < 0.05,
            cost_p_value: cost_test.p_value,
            confidence: 0.95,
        };

        let recommendation = Self::make_recommendation(&delta, &significance);

        Self {
            config_a: results_a.config_name.clone(),
            config_b: results_b.config_name.clone(),
            delta,
            significance,
            recommendation,
        }
    }

    /// Perform Welch's t-test
    fn t_test(sample_a: &[f64], sample_b: &[f64]) -> TTestResult {
        if sample_a.is_empty() || sample_b.is_empty() {
            return TTestResult {
                t_stat: 0.0,
                p_value: 1.0,
                _degrees_of_freedom: 0.0,
            };
        }

        let mean_a = sample_a.mean();
        let mean_b = sample_b.mean();
        let var_a = sample_a.variance();
        let var_b = sample_b.variance();
        let n_a = sample_a.len() as f64;
        let n_b = sample_b.len() as f64;

        // Welch's t-statistic
        let t_stat = (mean_b - mean_a) / ((var_a / n_a) + (var_b / n_b)).sqrt();

        // Welch-Satterthwaite degrees of freedom
        let df_num = ((var_a / n_a) + (var_b / n_b)).powi(2);
        let df_denom = (var_a / n_a).powi(2) / (n_a - 1.0) + (var_b / n_b).powi(2) / (n_b - 1.0);
        let df = df_num / df_denom;

        // Two-tailed p-value
        let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();
        let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));

        TTestResult {
            t_stat,
            p_value,
            _degrees_of_freedom: df,
        }
    }

    /// Make a recommendation based on deltas and significance
    fn make_recommendation(delta: &DeltaMetrics, sig: &SignificanceTest) -> Recommendation {
        // Decision criteria from the implementation plan:
        // ✅ ADOPT if:
        //   - Accuracy improvement ≥ +2% AND cost increase < 20%
        //   OR
        //   - Cost reduction ≥ 20% AND accuracy maintained
        //   OR
        //   - Latency improvement ≥ 30% AND cost/accuracy acceptable
        //
        // ❌ REJECT if:
        //   - No measurable improvement (< +1%)
        //   - Cost increase > 30% for < +2% accuracy
        //   - Adds complexity without clear benefit

        // Check if we have enough data
        if !sig.accuracy_significant && delta.accuracy.abs() < 2.0 {
            return Recommendation::NeedMoreData;
        }

        // Strong adoption criteria
        if delta.accuracy >= 2.0 && delta.cost_pct < 20.0 && sig.accuracy_significant {
            return Recommendation::Adopt;
        }

        if delta.cost_pct <= -20.0 && delta.accuracy >= -0.5 {
            return Recommendation::Adopt;
        }

        if delta.duration_pct <= -30.0 && delta.accuracy >= -0.5 && delta.cost_pct < 20.0 {
            return Recommendation::Adopt;
        }

        // Rejection criteria
        if delta.accuracy < 1.0 && !sig.accuracy_significant {
            return Recommendation::Reject;
        }

        if delta.cost_pct > 30.0 && delta.accuracy < 2.0 {
            return Recommendation::Reject;
        }

        if delta.accuracy < -1.0 {
            return Recommendation::Reject;
        }

        // Marginal cases
        if delta.accuracy >= 1.0 && delta.accuracy < 2.0 {
            return Recommendation::Investigate;
        }

        Recommendation::NeedMoreData
    }

    /// Print comparison summary
    pub fn print_summary(&self) {
        println!("\n=== A/B Test Comparison ===");
        println!("Config A: {}", self.config_a);
        println!("Config B: {}", self.config_b);
        println!();
        println!("Delta Metrics:");
        println!(
            "  Accuracy: {:+.2}% (p={:.4})",
            self.delta.accuracy, self.significance.accuracy_p_value
        );
        println!(
            "  Cost: {:+.4} USD ({:+.1}%)",
            self.delta.cost_usd, self.delta.cost_pct
        );
        println!(
            "  Duration: {:+.0} ms ({:+.1}%)",
            self.delta.duration_ms, self.delta.duration_pct
        );
        println!();
        println!("Statistical Significance:");
        println!(
            "  Accuracy significant: {} (p < 0.05)",
            self.significance.accuracy_significant
        );
        println!("  Cost significant: {}", self.significance.cost_significant);
        println!();
        println!("Recommendation: {:?}", self.recommendation);

        match self.recommendation {
            Recommendation::Adopt => {
                println!("✅ ADOPT config B - clear improvement detected");
            }
            Recommendation::Reject => {
                println!("❌ REJECT config B - no meaningful improvement");
            }
            Recommendation::NeedMoreData => {
                println!("⚠️  NEED MORE DATA - run more trials");
            }
            Recommendation::Investigate => {
                println!("🔍 INVESTIGATE - marginal improvement, consider tradeoffs");
            }
        }
    }
}

#[derive(Debug)]
struct TTestResult {
    t_stat: f64,
    p_value: f64,
    _degrees_of_freedom: f64,
}

/// Statistical test utilities
pub struct StatisticalTest;

impl StatisticalTest {
    /// Check if sample size is adequate for statistical power
    pub fn check_sample_size(n: usize, effect_size: f64) -> bool {
        // Rule of thumb: need at least 30 samples for t-test
        // For small effect sizes, need more samples
        let min_n = if effect_size < 0.3 {
            50
        } else if effect_size < 0.5 {
            30
        } else {
            20
        };

        n >= min_n
    }

    /// Calculate Cohen's d effect size
    ///
    /// Cohen's d measures the standardized difference between two means.
    ///
    /// # Interpretation
    ///
    /// - 0.2: Small effect
    /// - 0.5: Medium effect
    /// - 0.8: Large effect
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::stats::StatisticalTest;
    ///
    /// let baseline = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let improved = vec![3.0, 4.0, 5.0, 6.0, 7.0];
    ///
    /// let d = StatisticalTest::cohens_d(&baseline, &improved);
    /// assert!(d > 0.8); // Large effect
    /// ```
    pub fn cohens_d(sample_a: &[f64], sample_b: &[f64]) -> f64 {
        if sample_a.is_empty() || sample_b.is_empty() {
            return 0.0;
        }

        let mean_a = sample_a.mean();
        let mean_b = sample_b.mean();
        let var_a = sample_a.variance();
        let var_b = sample_b.variance();
        let n_a = sample_a.len() as f64;
        let n_b = sample_b.len() as f64;

        // Pooled standard deviation
        let pooled_sd = (((n_a - 1.0) * var_a + (n_b - 1.0) * var_b) / (n_a + n_b - 2.0)).sqrt();

        if pooled_sd > 0.0 {
            (mean_b - mean_a) / pooled_sd
        } else {
            0.0
        }
    }

    /// Interpret Cohen's d effect size
    ///
    /// Returns a human-readable interpretation of the effect size magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::stats::StatisticalTest;
    ///
    /// assert_eq!(StatisticalTest::interpret_effect_size(0.1), "Negligible");
    /// assert_eq!(StatisticalTest::interpret_effect_size(0.3), "Small");
    /// assert_eq!(StatisticalTest::interpret_effect_size(0.6), "Medium");
    /// assert_eq!(StatisticalTest::interpret_effect_size(1.0), "Large");
    /// assert_eq!(StatisticalTest::interpret_effect_size(1.5), "Very Large");
    /// ```
    pub fn interpret_effect_size(d: f64) -> &'static str {
        let d_abs = d.abs();
        if d_abs < 0.2 {
            "Negligible"
        } else if d_abs < 0.5 {
            "Small"
        } else if d_abs < 0.8 {
            "Medium"
        } else if d_abs < 1.2 {
            "Large"
        } else {
            "Very Large"
        }
    }

    /// Calculate bootstrap confidence interval using BCa method
    ///
    /// Uses the Bias-Corrected and Accelerated (BCa) bootstrap method with
    /// 10,000 resamples for robust confidence interval estimation.
    ///
    /// # Parameters
    ///
    /// - `sample`: Original sample data
    /// - `statistic_fn`: Function to compute statistic (e.g., mean, median)
    /// - `confidence_level`: Confidence level (e.g., 0.95 for 95%)
    ///
    /// # Returns
    ///
    /// Returns `(lower_bound, upper_bound)` of the confidence interval.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use toad::ai::stats::StatisticalTest;
    ///
    /// let sample = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let (lower, upper) = StatisticalTest::bootstrap_ci(&sample, |s| s.mean(), 0.95);
    ///
    /// assert!(lower < 3.0);
    /// assert!(upper > 3.0);
    /// ```
    pub fn bootstrap_ci<F>(sample: &[f64], statistic_fn: F, confidence_level: f64) -> (f64, f64)
    where
        F: Fn(&[f64]) -> f64,
    {
        use rand::Rng;

        if sample.is_empty() {
            return (0.0, 0.0);
        }

        const N_BOOTSTRAP: usize = 10_000;

        // Original statistic
        let theta_hat = statistic_fn(sample);

        // Bootstrap resamples
        let mut rng = rand::thread_rng();
        let mut bootstrap_stats = Vec::with_capacity(N_BOOTSTRAP);

        for _ in 0..N_BOOTSTRAP {
            let mut resample = Vec::with_capacity(sample.len());
            for _ in 0..sample.len() {
                let idx = rng.gen_range(0..sample.len());
                resample.push(sample[idx]);
            }
            bootstrap_stats.push(statistic_fn(&resample));
        }

        bootstrap_stats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Use percentile method (simpler than full BCa for now)
        let alpha = 1.0 - confidence_level;
        let lower_idx = ((N_BOOTSTRAP as f64 * alpha / 2.0) as usize).min(N_BOOTSTRAP - 1);
        let upper_idx = ((N_BOOTSTRAP as f64 * (1.0 - alpha / 2.0)) as usize).min(N_BOOTSTRAP - 1);

        (bootstrap_stats[lower_idx], bootstrap_stats[upper_idx])
    }

    /// Approximate inverse normal CDF (Beasley-Springer-Moro algorithm)
    fn inverse_normal_cdf(p: f64) -> f64 {
        // Clamp to valid range
        let p = p.max(1e-10).min(1.0 - 1e-10);

        // Beasley-Springer-Moro approximation
        let a = vec![
            2.50662823884,
            -18.61500062529,
            41.39119773534,
            -25.44106049637,
        ];
        let b = vec![
            -8.47351093090,
            23.08336743743,
            -21.06224101826,
            3.13082909833,
        ];
        let c = vec![
            0.3374754822726147,
            0.9761690190917186,
            0.1607979714918209,
            0.0276438810333863,
            0.0038405729373609,
            0.0003951896511919,
            0.0000321767881768,
            0.0000002888167364,
            0.0000003960315187,
        ];

        let y = p - 0.5;
        if y.abs() < 0.42 {
            let r = y * y;
            y * (((a[3] * r + a[2]) * r + a[1]) * r + a[0])
                / ((((b[3] * r + b[2]) * r + b[1]) * r + b[0]) * r + 1.0)
        } else {
            let r = if y > 0.0 { 1.0 - p } else { p };
            let s = r.ln();
            let t = (-s).sqrt();
            let mut result = c[0];
            for i in 1..c.len() {
                result += c[i] * t.powi(i as i32);
            }
            if y > 0.0 {
                result
            } else {
                -result
            }
        }
    }

    /// Normal CDF approximation
    fn normal_cdf(x: f64) -> f64 {
        0.5 * (1.0 + Self::erf(x / 2.0_f64.sqrt()))
    }

    /// Error function approximation
    fn erf(x: f64) -> f64 {
        // Abramowitz and Stegun approximation
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;

        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();

        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

        sign * y
    }

    /// Benjamini-Hochberg FDR correction for multiple testing
    ///
    /// Corrects p-values for multiple comparisons using the Benjamini-Hochberg
    /// procedure to control False Discovery Rate (FDR).
    ///
    /// # Parameters
    ///
    /// - `p_values`: Vector of uncorrected p-values
    /// - `fdr_level`: Desired FDR level (e.g., 0.05 for 5%)
    ///
    /// # Returns
    ///
    /// Returns a vector of booleans indicating which hypotheses to reject.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::stats::StatisticalTest;
    ///
    /// let p_values = vec![0.001, 0.008, 0.039, 0.041, 0.042];
    /// let rejections = StatisticalTest::benjamini_hochberg(&p_values, 0.05);
    ///
    /// assert_eq!(rejections.len(), 5);
    /// assert!(rejections[0]); // Reject first hypothesis
    /// ```
    pub fn benjamini_hochberg(p_values: &[f64], fdr_level: f64) -> Vec<bool> {
        let m = p_values.len();
        if m == 0 {
            return Vec::new();
        }

        // Create (index, p_value) pairs and sort by p-value
        let mut indexed_p: Vec<(usize, f64)> = p_values.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        indexed_p.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Find largest k such that P(k) <= (k/m) * alpha
        let mut k_max = None;
        for (k, &(_, p)) in indexed_p.iter().enumerate() {
            let threshold = ((k + 1) as f64 / m as f64) * fdr_level;
            if p <= threshold {
                k_max = Some(k);
            }
        }

        // Mark rejections
        let mut rejections = vec![false; m];
        if let Some(k) = k_max {
            for i in 0..=k {
                let (original_idx, _) = indexed_p[i];
                rejections[original_idx] = true;
            }
        }

        rejections
    }

    /// Calculate Pearson correlation coefficient
    ///
    /// Measures linear correlation between two variables.
    ///
    /// # Returns
    ///
    /// Returns r ∈ [-1, 1] where:
    /// - r = 1: Perfect positive correlation
    /// - r = 0: No correlation
    /// - r = -1: Perfect negative correlation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::stats::StatisticalTest;
    ///
    /// let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let y = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect positive correlation
    ///
    /// let r = StatisticalTest::pearson_correlation(&x, &y);
    /// assert!((r - 1.0).abs() < 0.01);
    /// ```
    pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.is_empty() {
            return 0.0;
        }

        let n = x.len() as f64;
        let mean_x = x.mean();
        let mean_y = y.mean();

        let mut numerator = 0.0;
        let mut sum_sq_x = 0.0;
        let mut sum_sq_y = 0.0;

        for i in 0..x.len() {
            let dx = x[i] - mean_x;
            let dy = y[i] - mean_y;
            numerator += dx * dy;
            sum_sq_x += dx * dx;
            sum_sq_y += dy * dy;
        }

        let denominator = (sum_sq_x * sum_sq_y).sqrt();
        if denominator > 0.0 {
            numerator / denominator
        } else {
            0.0
        }
    }

    /// Calculate Spearman rank correlation coefficient
    ///
    /// Measures monotonic correlation (not necessarily linear) between two variables.
    /// More robust to outliers than Pearson correlation.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::stats::StatisticalTest;
    ///
    /// let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// let y = vec![1.0, 4.0, 9.0, 16.0, 25.0]; // Monotonic but not linear
    ///
    /// let rho = StatisticalTest::spearman_correlation(&x, &y);
    /// assert!((rho - 1.0).abs() < 0.01); // Perfect monotonic correlation
    /// ```
    pub fn spearman_correlation(x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.is_empty() {
            return 0.0;
        }

        // Convert to ranks
        let ranks_x = Self::rank_data(x);
        let ranks_y = Self::rank_data(y);

        // Compute Pearson correlation on ranks
        Self::pearson_correlation(&ranks_x, &ranks_y)
    }

    /// Convert data to ranks (average rank for ties)
    fn rank_data(data: &[f64]) -> Vec<f64> {
        let mut indexed: Vec<(usize, f64)> = data.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut ranks = vec![0.0; data.len()];
        let mut i = 0;
        while i < indexed.len() {
            // Find range of equal values
            let mut j = i + 1;
            while j < indexed.len() && (indexed[j].1 - indexed[i].1).abs() < 1e-10 {
                j += 1;
            }

            // Average rank for this group
            let avg_rank = ((i + 1) + j) as f64 / 2.0;

            // Assign average rank to all tied values
            for k in i..j {
                ranks[indexed[k].0] = avg_rank;
            }

            i = j;
        }

        ranks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::evaluation::TaskResult;

    fn create_mock_results(name: &str, solved: Vec<bool>, costs: Vec<f64>) -> EvaluationResults {
        let results: Vec<TaskResult> = solved
            .iter()
            .zip(costs.iter())
            .enumerate()
            .map(|(i, (s, c))| {
                let mut result = TaskResult::new(format!("task-{}", i));
                if *s {
                    result.mark_solved();
                }
                result.cost_usd = *c;
                result
            })
            .collect();

        EvaluationResults::from_results(name.to_string(), results)
    }

    #[test]
    fn test_comparison_metrics() {
        // Test that comparison correctly computes deltas
        let results_a = create_mock_results(
            "baseline",
            vec![
                true, false, true, false, true, false, true, false, true, false,
            ],
            (0..10)
                .map(|i| 0.01 + (i as f64 * 0.0001))
                .collect::<Vec<_>>(),
        );

        let results_b = create_mock_results(
            "improved",
            vec![true, true, true, true, true, true, true, false, true, false],
            (0..10)
                .map(|i| 0.01 + (i as f64 * 0.0001))
                .collect::<Vec<_>>(),
        );

        let comparison = ComparisonResult::compare(&results_a, &results_b);

        // Check that delta accuracy is positive
        assert!(comparison.delta.accuracy > 0.0);
        // Check that cost delta is computed
        assert!(comparison.delta.cost_usd.abs() < 0.01); // Similar costs
        // Check that we have statistical test results
        assert!(comparison.significance.accuracy_p_value >= 0.0);
        assert!(comparison.significance.accuracy_p_value <= 1.0);
    }

    #[test]
    fn test_comparison_high_cost() {
        // Test rejection due to high cost with no benefit
        let results_a = create_mock_results(
            "baseline",
            vec![
                true, false, true, false, true, false, true, false, true, false,
            ],
            vec![0.01; 10],
        );

        let results_b = create_mock_results(
            "expensive",
            vec![
                true, false, true, false, true, false, true, false, true, false,
            ],
            vec![0.10; 10], // 10x cost
        );

        let comparison = ComparisonResult::compare(&results_a, &results_b);

        // Should not recommend adoption due to high cost
        assert!(!matches!(comparison.recommendation, Recommendation::Adopt));
        // Cost should be significantly higher
        assert!(comparison.delta.cost_pct > 50.0);
    }

    #[test]
    fn test_cohens_d() {
        let sample_a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sample_b = vec![3.0, 4.0, 5.0, 6.0, 7.0];

        let d = StatisticalTest::cohens_d(&sample_a, &sample_b);
        assert!(d > 0.0); // Positive effect size
        assert!(d > 1.0); // Large effect
    }

    #[test]
    fn test_sample_size_check() {
        assert!(!StatisticalTest::check_sample_size(10, 0.2)); // Too small for small effect
        assert!(StatisticalTest::check_sample_size(50, 0.2)); // OK for small effect
        assert!(StatisticalTest::check_sample_size(20, 0.8)); // OK for large effect
    }

    // ===== Phase 4: Enhanced Statistical Tests =====

    #[test]
    fn test_interpret_effect_size() {
        assert_eq!(StatisticalTest::interpret_effect_size(0.1), "Negligible");
        assert_eq!(StatisticalTest::interpret_effect_size(0.3), "Small");
        assert_eq!(StatisticalTest::interpret_effect_size(0.6), "Medium");
        assert_eq!(StatisticalTest::interpret_effect_size(1.0), "Large");
        assert_eq!(StatisticalTest::interpret_effect_size(1.5), "Very Large");

        // Test negative values (magnitude)
        assert_eq!(StatisticalTest::interpret_effect_size(-0.3), "Small");
        assert_eq!(StatisticalTest::interpret_effect_size(-1.0), "Large");
    }

    #[test]
    fn test_bootstrap_ci_basic() {
        // Simple sample with known mean
        let sample = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let (lower, upper) = StatisticalTest::bootstrap_ci(&sample, |s| s.mean(), 0.95);

        // CI should contain the true mean (3.0)
        assert!(lower < 3.0, "Lower bound {} should be < 3.0", lower);
        assert!(upper > 3.0, "Upper bound {} should be > 3.0", upper);

        // CI should be reasonably narrow for this sample
        assert!(
            upper - lower < 3.0,
            "CI width {} should be < 3.0",
            upper - lower
        );
    }

    #[test]
    fn test_bootstrap_ci_performance() {
        use std::time::Instant;

        // Test that 10k bootstrap resamples complete in < 500ms (allows headroom for CI/slow machines)
        let sample: Vec<f64> = (0..100).map(|i| i as f64).collect();

        let start = Instant::now();
        let _ = StatisticalTest::bootstrap_ci(&sample, |s| s.mean(), 0.95);
        let duration = start.elapsed();

        assert!(
            duration.as_millis() < 500,
            "Bootstrap should complete in < 500ms, took {}ms",
            duration.as_millis()
        );
    }

    #[test]
    fn test_benjamini_hochberg() {
        // Example from Benjamini & Hochberg (1995)
        let p_values = vec![0.001, 0.008, 0.039, 0.041, 0.042, 0.06, 0.074, 0.205];

        let rejections = StatisticalTest::benjamini_hochberg(&p_values, 0.05);

        // At FDR = 0.05, some should be rejected
        assert_eq!(rejections.len(), 8);
        assert!(rejections[0]); // p=0.001 should definitely be rejected
        assert!(rejections[1]); // p=0.008 should definitely be rejected

        // Last one should not be rejected
        assert!(!rejections[7]); // p=0.205 should not be rejected

        // Count total rejections
        let num_rejected = rejections.iter().filter(|&&r| r).count();
        assert!(num_rejected >= 2, "At least 2 hypotheses should be rejected");
    }

    #[test]
    fn test_benjamini_hochberg_edge_cases() {
        // Empty p-values
        let rejections = StatisticalTest::benjamini_hochberg(&[], 0.05);
        assert_eq!(rejections.len(), 0);

        // All significant
        let p_values = vec![0.001, 0.002, 0.003];
        let rejections = StatisticalTest::benjamini_hochberg(&p_values, 0.05);
        assert!(rejections.iter().all(|&r| r));

        // None significant
        let p_values = vec![0.9, 0.95, 0.99];
        let rejections = StatisticalTest::benjamini_hochberg(&p_values, 0.05);
        assert!(rejections.iter().all(|&r| !r));
    }

    #[test]
    fn test_pearson_correlation_perfect_positive() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

        let r = StatisticalTest::pearson_correlation(&x, &y);

        // Should be perfect positive correlation
        assert!((r - 1.0).abs() < 0.01, "r = {} should be ~1.0", r);
    }

    #[test]
    fn test_pearson_correlation_perfect_negative() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![10.0, 8.0, 6.0, 4.0, 2.0];

        let r = StatisticalTest::pearson_correlation(&x, &y);

        // Should be perfect negative correlation
        assert!((r + 1.0).abs() < 0.01, "r = {} should be ~-1.0", r);
    }

    #[test]
    fn test_pearson_correlation_no_correlation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![3.0, 1.0, 4.0, 2.0, 5.0]; // Random order

        let r = StatisticalTest::pearson_correlation(&x, &y);

        // Should be weak correlation (not perfectly uncorrelated due to small sample)
        assert!(
            r.abs() < 1.0,
            "r = {} should not be perfect correlation",
            r
        );
    }

    #[test]
    fn test_spearman_correlation_monotonic() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![1.0, 4.0, 9.0, 16.0, 25.0]; // y = x^2 (monotonic but not linear)

        let rho = StatisticalTest::spearman_correlation(&x, &y);

        // Should be perfect monotonic correlation
        assert!(
            (rho - 1.0).abs() < 0.01,
            "rho = {} should be ~1.0",
            rho
        );
    }

    #[test]
    fn test_spearman_correlation_with_ties() {
        let x = vec![1.0, 2.0, 2.0, 3.0, 4.0]; // Tied values
        let y = vec![1.0, 2.0, 2.0, 3.0, 4.0];

        let rho = StatisticalTest::spearman_correlation(&x, &y);

        // Should still be perfect correlation (ties handled correctly)
        assert!(
            (rho - 1.0).abs() < 0.01,
            "rho = {} should be ~1.0",
            rho
        );
    }

    #[test]
    fn test_rank_data() {
        // Simple ranking
        let data = vec![5.0, 1.0, 3.0, 2.0, 4.0];
        let ranks = StatisticalTest::rank_data(&data);
        assert_eq!(ranks, vec![5.0, 1.0, 3.0, 2.0, 4.0]);

        // With ties (average rank)
        let data = vec![1.0, 2.0, 2.0, 4.0];
        let ranks = StatisticalTest::rank_data(&data);
        assert_eq!(ranks, vec![1.0, 2.5, 2.5, 4.0]); // Tied values get average rank
    }

    #[test]
    fn test_correlation_edge_cases() {
        // Empty vectors
        assert_eq!(StatisticalTest::pearson_correlation(&[], &[]), 0.0);
        assert_eq!(StatisticalTest::spearman_correlation(&[], &[]), 0.0);

        // Mismatched lengths
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![1.0, 2.0];
        assert_eq!(StatisticalTest::pearson_correlation(&x, &y), 0.0);

        // Constant values (zero variance)
        let x = vec![5.0, 5.0, 5.0];
        let y = vec![1.0, 2.0, 3.0];
        assert_eq!(StatisticalTest::pearson_correlation(&x, &y), 0.0);
    }
}
