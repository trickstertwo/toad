/// Statistical testing for A/B comparisons

use crate::evaluation::EvaluationResults;
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
                ((results_b.avg_duration_ms - results_a.avg_duration_ms) / results_a.avg_duration_ms) * 100.0
            } else {
                0.0
            },
            api_calls: 0.0, // TODO: compute from results
        };

        // Extract metrics for statistical testing
        let metrics_a: Vec<f64> = results_a.results.iter()
            .map(|r| if r.solved { 1.0 } else { 0.0 })
            .collect();

        let metrics_b: Vec<f64> = results_b.results.iter()
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
        println!("  Accuracy: {:+.2}% (p={:.4})", self.delta.accuracy, self.significance.accuracy_p_value);
        println!("  Cost: {:+.4} USD ({:+.1}%)", self.delta.cost_usd, self.delta.cost_pct);
        println!("  Duration: {:+.0} ms ({:+.1}%)", self.delta.duration_ms, self.delta.duration_pct);
        println!();
        println!("Statistical Significance:");
        println!("  Accuracy significant: {} (p < 0.05)", self.significance.accuracy_significant);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluation::TaskResult;

    fn create_mock_results(name: &str, solved: Vec<bool>, costs: Vec<f64>) -> EvaluationResults {
        let results: Vec<TaskResult> = solved.iter().zip(costs.iter())
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
            vec![true, false, true, false, true, false, true, false, true, false],
            (0..10).map(|i| 0.01 + (i as f64 * 0.0001)).collect::<Vec<_>>(),
        );

        let results_b = create_mock_results(
            "improved",
            vec![true, true, true, true, true, true, true, false, true, false],
            (0..10).map(|i| 0.01 + (i as f64 * 0.0001)).collect::<Vec<_>>(),
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
            vec![true, false, true, false, true, false, true, false, true, false],
            vec![0.01; 10],
        );

        let results_b = create_mock_results(
            "expensive",
            vec![true, false, true, false, true, false, true, false, true, false],
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
        assert!(StatisticalTest::check_sample_size(50, 0.2));  // OK for small effect
        assert!(StatisticalTest::check_sample_size(20, 0.8));  // OK for large effect
    }
}
