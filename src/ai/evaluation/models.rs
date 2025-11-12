//! Enhanced data models for multi-benchmark evaluation
//!
//! This module provides rich data structures for storing and analyzing evaluation
//! results across multiple benchmarks. These models extend the existing SWE-bench
//! specific structures in `mod.rs` to support cross-benchmark comparison and
//! statistical analysis.
//!
//! # Architecture
//!
//! The evaluation data model hierarchy:
//!
//! ```text
//! EvaluationRun (entire evaluation session)
//!   ├─ BenchmarkResult (per benchmark)
//!   │   └─ Vec<TaskResult> (per task - from mod.rs)
//!   ├─ AggregateMetrics (cross-benchmark statistics)
//!   │   └─ BehavioralMetrics (quality signals)
//!   └─ config_snapshot (system configuration)
//! ```
//!
//! # Versioning
//!
//! The `EvaluationRun` struct includes a `format_version` field for schema evolution.
//! Current version: 1
//!
//! When adding new fields:
//! - Use `#[serde(default)]` for backward compatibility
//! - Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
//! - Increment `format_version` only for breaking changes
//!
//! # Examples
//!
//! ```rust
//! use toad::ai::evaluation::models::{EvaluationRun, BenchmarkResult, AggregateMetrics};
//! use chrono::Utc;
//!
//! // Create an evaluation run
//! let run = EvaluationRun {
//!     run_id: "eval-2024-11-11-001".to_string(),
//!     timestamp: Utc::now(),
//!     benchmark_results: vec![],
//!     aggregate_metrics: AggregateMetrics::default(),
//!     config_snapshot: serde_json::json!({"model": "claude-sonnet-3-5"}),
//!     format_version: 1,
//! };
//!
//! // Serialize to JSON
//! let json = serde_json::to_string_pretty(&run).unwrap();
//! ```

use crate::ai::evaluation::TaskResult;
use crate::benchmarks::BenchmarkMetadata;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A complete evaluation session across multiple benchmarks
///
/// Represents the results of running one or more benchmarks with a specific
/// system configuration. This is the top-level structure stored to disk after
/// an evaluation completes.
///
/// # Storage Format
///
/// Stored as JSON in `./results/evaluation_{timestamp}_{run_id}.json`:
/// ```json
/// {
///   "run_id": "20241111-183042-a3f5",
///   "timestamp": "2024-11-11T18:30:42Z",
///   "format_version": 1,
///   "benchmark_results": [...],
///   "aggregate_metrics": {...},
///   "config_snapshot": {...}
/// }
/// ```
///
/// # Examples
///
/// ```rust
/// use toad::ai::evaluation::models::EvaluationRun;
/// use chrono::Utc;
///
/// let run = EvaluationRun::new("eval-001".to_string(), serde_json::json!({}));
/// assert_eq!(run.run_id, "eval-001");
/// assert_eq!(run.format_version, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationRun {
    /// Unique run identifier (format: "{timestamp}-{short_uuid}")
    ///
    /// Example: "20241111-183042-a3f5"
    pub run_id: String,

    /// When this evaluation started
    pub timestamp: DateTime<Utc>,

    /// Results from each benchmark that was run
    ///
    /// Order matches the execution order. Empty if evaluation was cancelled
    /// before any benchmarks completed.
    pub benchmark_results: Vec<BenchmarkResult>,

    /// Aggregate statistics across all benchmarks
    ///
    /// Computed after all benchmarks complete. May be partial if evaluation
    /// was cancelled.
    pub aggregate_metrics: AggregateMetrics,

    /// Snapshot of system configuration at evaluation time
    ///
    /// Stores the complete `ToadConfig` serialized as JSON. Enables reproducing
    /// results and comparing different configurations.
    pub config_snapshot: Value,

    /// Data format version for schema migration
    ///
    /// Current version: 1
    /// Increment only for breaking schema changes.
    pub format_version: u32,
}

impl EvaluationRun {
    /// Create a new evaluation run
    ///
    /// # Examples
    ///
    /// ```rust
    /// use toad::ai::evaluation::models::EvaluationRun;
    ///
    /// let config = serde_json::json!({"model": "claude-sonnet-3-5"});
    /// let run = EvaluationRun::new("eval-001".to_string(), config);
    /// ```
    pub fn new(run_id: String, config_snapshot: Value) -> Self {
        Self {
            run_id,
            timestamp: Utc::now(),
            benchmark_results: vec![],
            aggregate_metrics: AggregateMetrics::default(),
            config_snapshot,
            format_version: 1,
        }
    }
}

/// Results from a single benchmark execution
///
/// Contains all tasks executed for one benchmark (e.g., SWE-bench Verified)
/// along with benchmark-specific metadata and summary statistics.
///
/// # Examples
///
/// ```rust
/// use toad::ai::evaluation::models::BenchmarkResult;
/// use toad::benchmarks::BenchmarkMetadata;
///
/// let metadata = BenchmarkMetadata {
///     name: "SWE-bench Verified".to_string(),
///     version: "1.0".to_string(),
///     total_tasks: 500,
///     dataset_url: None,
///     license: None,
///     contamination_risk: "LOW".to_string(),
/// };
///
/// let result = BenchmarkResult::new(metadata);
/// assert_eq!(result.benchmark_metadata.name, "SWE-bench Verified");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Metadata about this benchmark
    pub benchmark_metadata: BenchmarkMetadata,

    /// Results for each task in this benchmark
    ///
    /// Order matches task execution order. Length may be less than
    /// `benchmark_metadata.total_tasks` if evaluation was cancelled.
    pub task_results: Vec<TaskResult>,

    /// Total execution duration for this benchmark (milliseconds)
    pub duration_ms: u64,

    /// Success rate: tasks_solved / tasks_attempted
    ///
    /// Range: [0.0, 1.0]
    pub success_rate: f64,

    /// Total API cost for this benchmark (USD)
    pub total_cost_usd: f64,

    /// Average cost per task (USD)
    #[serde(default)]
    pub avg_cost_per_task_usd: f64,

    /// Average duration per task (milliseconds)
    #[serde(default)]
    pub avg_duration_per_task_ms: f64,
}

impl BenchmarkResult {
    /// Create a new benchmark result with metadata
    pub fn new(benchmark_metadata: BenchmarkMetadata) -> Self {
        Self {
            benchmark_metadata,
            task_results: vec![],
            duration_ms: 0,
            success_rate: 0.0,
            total_cost_usd: 0.0,
            avg_cost_per_task_usd: 0.0,
            avg_duration_per_task_ms: 0.0,
        }
    }

    /// Compute summary statistics from task results
    ///
    /// Updates success_rate, total_cost_usd, and averages based on task_results.
    pub fn compute_statistics(&mut self) {
        if self.task_results.is_empty() {
            return;
        }

        let tasks_solved = self.task_results.iter().filter(|t| t.solved).count();
        self.success_rate = tasks_solved as f64 / self.task_results.len() as f64;

        self.total_cost_usd = self.task_results.iter().map(|t| t.cost_usd).sum();
        self.avg_cost_per_task_usd = self.total_cost_usd / self.task_results.len() as f64;

        let total_duration: u64 = self.task_results.iter().map(|t| t.duration_ms).sum();
        self.avg_duration_per_task_ms = total_duration as f64 / self.task_results.len() as f64;
    }
}

/// Aggregate metrics across multiple benchmarks
///
/// Provides summary statistics computed from all benchmark results in an
/// evaluation run. Used for cross-benchmark comparison and overall performance
/// assessment.
///
/// # Examples
///
/// ```rust
/// use toad::ai::evaluation::models::AggregateMetrics;
///
/// let metrics = AggregateMetrics {
///     mean_accuracy: 0.68,
///     median_latency_ms: 2500.0,
///     total_cost_usd: 1687.50,
///     total_tasks: 1518,
///     tasks_solved: 1032,
///     behavioral_metrics: None,
/// };
///
/// assert_eq!(metrics.mean_accuracy, 0.68);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregateMetrics {
    /// Mean accuracy across all benchmarks
    ///
    /// Weighted average: (sum of tasks_solved) / (sum of total_tasks)
    /// Range: [0.0, 1.0]
    pub mean_accuracy: f64,

    /// Median task latency across all benchmarks (milliseconds)
    pub median_latency_ms: f64,

    /// Total cost across all benchmarks (USD)
    pub total_cost_usd: f64,

    /// Total number of tasks attempted
    pub total_tasks: usize,

    /// Total number of tasks solved successfully
    pub tasks_solved: usize,

    /// Behavioral and quality metrics (optional)
    ///
    /// Computed if sufficient data is available. Requires tracking additional
    /// quality signals during task execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavioral_metrics: Option<BehavioralMetrics>,
}

impl Default for AggregateMetrics {
    fn default() -> Self {
        Self {
            mean_accuracy: 0.0,
            median_latency_ms: 0.0,
            total_cost_usd: 0.0,
            total_tasks: 0,
            tasks_solved: 0,
            behavioral_metrics: None,
        }
    }
}

/// Behavioral and quality metrics
///
/// Tracks higher-level quality signals beyond simple accuracy metrics. These
/// measure agent behavior, reliability, and autonomy.
///
/// # Metric Definitions
///
/// - **Hallucination Rate**: Fraction of tasks where agent made factually incorrect
///   claims or referenced non-existent code/APIs
/// - **Tool Use Efficiency**: Average tools used per solved task (lower is better)
/// - **Autonomy Score**: Fraction of tasks completed without errors or retries
/// - **Error Recovery Rate**: Fraction of failed operations that were successfully
///   recovered from
///
/// # Examples
///
/// ```rust
/// use toad::ai::evaluation::models::BehavioralMetrics;
///
/// let metrics = BehavioralMetrics {
///     hallucination_rate: 0.12,
///     tool_use_efficiency: 8.5,
///     autonomy_score: 0.73,
///     error_recovery_rate: 0.45,
/// };
///
/// // Low hallucination is good
/// assert!(metrics.hallucination_rate < 0.15);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BehavioralMetrics {
    /// Fraction of tasks with hallucinations
    ///
    /// Range: [0.0, 1.0], lower is better
    pub hallucination_rate: f64,

    /// Average number of tool calls per solved task
    ///
    /// Range: [0.0, ∞), lower is better (more efficient)
    pub tool_use_efficiency: f64,

    /// Fraction of tasks completed without errors/retries
    ///
    /// Range: [0.0, 1.0], higher is better
    pub autonomy_score: f64,

    /// Fraction of errors that were successfully recovered
    ///
    /// Range: [0.0, 1.0], higher is better
    pub error_recovery_rate: f64,
}

impl Default for BehavioralMetrics {
    fn default() -> Self {
        Self {
            hallucination_rate: 0.0,
            tool_use_efficiency: 0.0,
            autonomy_score: 1.0,
            error_recovery_rate: 0.0,
        }
    }
}

/// Statistical summary for A/B comparison
///
/// Contains results from statistical hypothesis testing (Welch's t-test) and
/// effect size calculations (Cohen's d). Used when comparing two configurations
/// or benchmarks.
///
/// # Interpretation
///
/// - **p_value < 0.05**: Statistically significant difference
/// - **Cohen's d**:
///   - |d| < 0.2: Negligible effect
///   - 0.2 ≤ |d| < 0.5: Small effect
///   - 0.5 ≤ |d| < 0.8: Medium effect
///   - |d| ≥ 0.8: Large effect
///
/// # Examples
///
/// ```rust
/// use toad::ai::evaluation::models::StatisticalSummary;
///
/// let summary = StatisticalSummary {
///     t_statistic: 2.87,
///     p_value: 0.005,
///     degrees_of_freedom: 48.3,
///     effect_size_cohens_d: 0.64,
///     confidence_interval_95: (0.018, 0.072),
/// };
///
/// // Significant difference (p < 0.05) with medium effect size
/// assert!(summary.p_value < 0.05);
/// assert!(summary.effect_size_cohens_d.abs() >= 0.5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatisticalSummary {
    /// Welch's t-test statistic
    pub t_statistic: f64,

    /// Two-tailed p-value
    ///
    /// Probability of observing this difference by chance.
    /// p < 0.05 typically considered statistically significant.
    pub p_value: f64,

    /// Degrees of freedom for the t-test
    ///
    /// Computed using Welch-Satterthwaite equation for unequal variances.
    pub degrees_of_freedom: f64,

    /// Cohen's d effect size
    ///
    /// Standardized mean difference: (mean1 - mean2) / pooled_std
    /// Interpretation: Small (0.2), Medium (0.5), Large (0.8)
    pub effect_size_cohens_d: f64,

    /// 95% confidence interval for the mean difference
    ///
    /// Format: (lower_bound, upper_bound)
    /// If interval includes 0, difference may not be meaningful.
    pub confidence_interval_95: (f64, f64),
}

impl StatisticalSummary {
    /// Create statistical summary from two samples
    ///
    /// Computes all statistical tests comparing two samples:
    /// - Welch's t-test (unequal variances)
    /// - Cohen's d effect size
    /// - 95% bootstrap confidence interval
    ///
    /// # Parameters
    ///
    /// - `sample_a`: Baseline/control group measurements
    /// - `sample_b`: Treatment/experimental group measurements
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use toad::ai::evaluation::models::StatisticalSummary;
    ///
    /// let baseline = vec![0.5, 0.6, 0.55, 0.58, 0.52];
    /// let improved = vec![0.68, 0.72, 0.70, 0.75, 0.69];
    ///
    /// let summary = StatisticalSummary::from_samples(&baseline, &improved);
    ///
    /// println!("p-value: {}", summary.p_value);
    /// println!("Effect size: {} ({})",
    ///     summary.effect_size_cohens_d,
    ///     if summary.effect_size_cohens_d > 0.8 { "Large" } else { "Medium" }
    /// );
    /// ```
    pub fn from_samples(sample_a: &[f64], sample_b: &[f64]) -> Self {
        use crate::ai::stats::StatisticalTest;
        use statrs::distribution::{ContinuousCDF, StudentsT};
        use statrs::statistics::Statistics;

        // Welch's t-test
        let mean_a = sample_a.mean();
        let mean_b = sample_b.mean();
        let var_a = sample_a.variance();
        let var_b = sample_b.variance();
        let n_a = sample_a.len() as f64;
        let n_b = sample_b.len() as f64;

        let t_statistic = (mean_b - mean_a) / ((var_a / n_a) + (var_b / n_b)).sqrt();

        // Welch-Satterthwaite degrees of freedom
        let df_num = ((var_a / n_a) + (var_b / n_b)).powi(2);
        let df_denom = (var_a / n_a).powi(2) / (n_a - 1.0) + (var_b / n_b).powi(2) / (n_b - 1.0);
        let degrees_of_freedom = df_num / df_denom;

        // Two-tailed p-value
        let t_dist = StudentsT::new(0.0, 1.0, degrees_of_freedom).unwrap();
        let p_value = 2.0 * (1.0 - t_dist.cdf(t_statistic.abs()));

        // Cohen's d effect size
        let effect_size_cohens_d = StatisticalTest::cohens_d(sample_a, sample_b);

        // Bootstrap 95% confidence interval for mean difference
        let combined: Vec<f64> = sample_a.iter().chain(sample_b.iter()).copied().collect();
        let (lower, upper) = StatisticalTest::bootstrap_ci(
            &combined,
            |s| {
                let mid = sample_a.len();
                s[..mid].mean() - s[mid..].mean()
            },
            0.95,
        );

        Self {
            t_statistic,
            p_value,
            degrees_of_freedom,
            effect_size_cohens_d,
            confidence_interval_95: (lower, upper),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::BenchmarkMetadata;

    #[test]
    fn test_evaluation_run_serialization_roundtrip() {
        // Create an evaluation run
        let config = serde_json::json!({
            "model": "claude-sonnet-3-5-20241022",
            "temperature": 1.0,
        });

        let mut run = EvaluationRun::new("test-run-123".to_string(), config);
        run.benchmark_results.push(BenchmarkResult::new(
            BenchmarkMetadata {
                name: "Test Benchmark".to_string(),
                version: "1.0".to_string(),
                total_tasks: 100,
                dataset_url: None,
                license: None,
                contamination_risk: "LOW".to_string(),
            },
        ));

        // Serialize to JSON
        let json = serde_json::to_string(&run).expect("Failed to serialize");

        // Deserialize back
        let deserialized: EvaluationRun =
            serde_json::from_str(&json).expect("Failed to deserialize");

        // Verify roundtrip
        assert_eq!(run.run_id, deserialized.run_id);
        assert_eq!(run.format_version, deserialized.format_version);
        assert_eq!(run.benchmark_results.len(), 1);
        assert_eq!(
            run.benchmark_results[0].benchmark_metadata.name,
            "Test Benchmark"
        );
    }

    #[test]
    fn test_aggregate_metrics_computation() {
        // Create benchmark results with known statistics
        let mut benchmark1 = BenchmarkResult::new(BenchmarkMetadata {
            name: "Benchmark 1".to_string(),
            version: "1.0".to_string(),
            total_tasks: 100,
            dataset_url: None,
            license: None,
            contamination_risk: "LOW".to_string(),
        });

        // Add mock task results
        for i in 0..100 {
            use crate::ai::evaluation::TaskResult;
            let mut task = TaskResult::new(format!("task-{}", i));
            task.solved = i < 70; // 70% success rate
            task.cost_usd = 2.5;
            task.duration_ms = 3000;
            benchmark1.task_results.push(task);
        }

        benchmark1.compute_statistics();

        // Verify computed statistics
        assert_eq!(benchmark1.success_rate, 0.70);
        assert_eq!(benchmark1.total_cost_usd, 250.0); // 100 * 2.5
        assert_eq!(benchmark1.avg_cost_per_task_usd, 2.5);
        assert_eq!(benchmark1.avg_duration_per_task_ms, 3000.0);
    }

    #[test]
    fn test_behavioral_metrics_validation() {
        // Create behavioral metrics
        let metrics = BehavioralMetrics {
            hallucination_rate: 0.12,
            tool_use_efficiency: 8.5,
            autonomy_score: 0.73,
            error_recovery_rate: 0.45,
        };

        // Verify ranges
        assert!(metrics.hallucination_rate >= 0.0 && metrics.hallucination_rate <= 1.0);
        assert!(metrics.autonomy_score >= 0.0 && metrics.autonomy_score <= 1.0);
        assert!(metrics.error_recovery_rate >= 0.0 && metrics.error_recovery_rate <= 1.0);
        assert!(metrics.tool_use_efficiency >= 0.0);

        // Verify serialization
        let json = serde_json::to_string(&metrics).expect("Failed to serialize");
        let deserialized: BehavioralMetrics =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(metrics.hallucination_rate, deserialized.hallucination_rate);
        assert_eq!(metrics.autonomy_score, deserialized.autonomy_score);
    }
}
