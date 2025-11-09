/// Metrics collection and analysis for TOAD evaluations
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Quality metrics for a task solution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityMetrics {
    /// Syntax correctness (0.0-1.0)
    pub syntax_valid: f64,

    /// Test pass rate (0.0-1.0)
    pub test_pass_rate: f64,

    /// Code coverage (0.0-1.0)
    pub code_coverage: f64,

    /// Files correctly modified (0.0-1.0)
    pub file_accuracy: f64,
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            syntax_valid: 0.0,
            test_pass_rate: 0.0,
            code_coverage: 0.0,
            file_accuracy: 0.0,
        }
    }
}

/// Comprehensive metrics for a single task execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metrics {
    // === Accuracy Metrics ===
    /// Was the task solved?
    pub solved: bool,

    /// Quality scores
    pub quality: QualityMetrics,

    // === Cost Metrics ===
    /// Total cost in USD
    pub cost_usd: f64,

    /// API calls made
    pub api_calls: u32,

    /// Input tokens
    pub input_tokens: u64,

    /// Output tokens
    pub output_tokens: u64,

    /// Cached tokens (saved from prompt cache)
    pub cached_tokens: u64,

    // === Performance Metrics ===
    /// Wall clock time (milliseconds)
    pub duration_ms: u64,

    /// Time to first response (milliseconds)
    pub time_to_first_response_ms: u64,

    /// Context retrieval time (milliseconds)
    pub context_retrieval_ms: u64,

    // === Behavioral Metrics ===
    /// Number of edit attempts
    pub edit_attempts: u32,

    /// Number of files read
    pub files_read: u32,

    /// Number of files written
    pub files_written: u32,

    /// Number of test runs
    pub test_runs: u32,

    /// Agent steps taken
    pub agent_steps: u32,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            solved: false,
            quality: QualityMetrics::default(),
            cost_usd: 0.0,
            api_calls: 0,
            input_tokens: 0,
            output_tokens: 0,
            cached_tokens: 0,
            duration_ms: 0,
            time_to_first_response_ms: 0,
            context_retrieval_ms: 0,
            edit_attempts: 0,
            files_read: 0,
            files_written: 0,
            test_runs: 0,
            agent_steps: 0,
        }
    }
}

impl Metrics {
    /// Calculate total tokens
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens
    }

    /// Calculate effective tokens (after caching)
    pub fn effective_tokens(&self) -> u64 {
        self.total_tokens().saturating_sub(self.cached_tokens)
    }

    /// Calculate cost per token
    pub fn cost_per_token(&self) -> f64 {
        let total = self.total_tokens();
        if total > 0 {
            self.cost_usd / total as f64
        } else {
            0.0
        }
    }

    /// Calculate efficiency score (accuracy / cost)
    pub fn efficiency(&self) -> f64 {
        if self.cost_usd > 0.0 {
            if self.solved {
                1.0 / self.cost_usd
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

/// Metrics collector that tracks metrics during execution
pub struct MetricsCollector {
    metrics: Metrics,
    start_time: Option<Instant>,
    first_response_time: Option<Instant>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Metrics::default(),
            start_time: None,
            first_response_time: None,
        }
    }

    /// Start timing
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Record first response
    pub fn record_first_response(&mut self) {
        if self.first_response_time.is_none() {
            self.first_response_time = Some(Instant::now());

            if let Some(start) = self.start_time {
                self.metrics.time_to_first_response_ms =
                    Instant::now().duration_since(start).as_millis() as u64;
            }
        }
    }

    /// Stop timing and finalize
    pub fn finish(&mut self) -> Metrics {
        if let Some(start) = self.start_time {
            self.metrics.duration_ms = Instant::now().duration_since(start).as_millis() as u64;
        }

        self.metrics.clone()
    }

    /// Record an API call
    pub fn record_api_call(
        &mut self,
        input_tokens: u64,
        output_tokens: u64,
        cached_tokens: u64,
        cost: f64,
    ) {
        self.metrics.api_calls += 1;
        self.metrics.input_tokens += input_tokens;
        self.metrics.output_tokens += output_tokens;
        self.metrics.cached_tokens += cached_tokens;
        self.metrics.cost_usd += cost;
    }

    /// Record a file operation
    pub fn record_file_read(&mut self) {
        self.metrics.files_read += 1;
    }

    pub fn record_file_write(&mut self) {
        self.metrics.files_written += 1;
    }

    pub fn record_edit_attempt(&mut self) {
        self.metrics.edit_attempts += 1;
    }

    /// Record a test run
    pub fn record_test_run(&mut self) {
        self.metrics.test_runs += 1;
    }

    /// Record an agent step
    pub fn record_agent_step(&mut self) {
        self.metrics.agent_steps += 1;
    }

    /// Mark task as solved
    pub fn mark_solved(&mut self, quality: QualityMetrics) {
        self.metrics.solved = true;
        self.metrics.quality = quality;
    }

    /// Get current metrics snapshot
    pub fn snapshot(&self) -> &Metrics {
        &self.metrics
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate metrics across multiple runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateMetrics {
    /// Number of runs
    pub count: usize,

    /// Accuracy (% solved)
    pub accuracy: f64,

    /// Mean metrics
    pub mean_cost_usd: f64,
    pub mean_duration_ms: f64,
    pub mean_api_calls: f64,
    pub mean_tokens: f64,

    /// Standard deviation
    pub std_cost_usd: f64,
    pub std_duration_ms: f64,

    /// Min/max
    pub min_cost_usd: f64,
    pub max_cost_usd: f64,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,

    /// Percentiles
    pub p50_cost_usd: f64,
    pub p95_cost_usd: f64,
    pub p99_cost_usd: f64,
}

impl AggregateMetrics {
    /// Compute aggregate metrics from a collection of metrics
    pub fn from_metrics(metrics: &[Metrics]) -> Self {
        let count = metrics.len();
        if count == 0 {
            return Self::default();
        }

        let solved_count = metrics.iter().filter(|m| m.solved).count();
        let accuracy = (solved_count as f64 / count as f64) * 100.0;

        let costs: Vec<f64> = metrics.iter().map(|m| m.cost_usd).collect();
        let durations: Vec<u64> = metrics.iter().map(|m| m.duration_ms).collect();

        let mean_cost_usd = costs.iter().sum::<f64>() / count as f64;
        let mean_duration_ms = durations.iter().sum::<u64>() as f64 / count as f64;
        let mean_api_calls = metrics.iter().map(|m| m.api_calls as f64).sum::<f64>() / count as f64;
        let mean_tokens =
            metrics.iter().map(|m| m.total_tokens() as f64).sum::<f64>() / count as f64;

        // Calculate standard deviations
        let variance_cost = costs
            .iter()
            .map(|c| (c - mean_cost_usd).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_cost_usd = variance_cost.sqrt();

        let variance_duration = durations
            .iter()
            .map(|d| (*d as f64 - mean_duration_ms).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_duration_ms = variance_duration.sqrt();

        // Min/max
        let min_cost_usd = costs.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_cost_usd = costs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_duration_ms = *durations.iter().min().unwrap_or(&0);
        let max_duration_ms = *durations.iter().max().unwrap_or(&0);

        // Percentiles
        let mut sorted_costs = costs.clone();
        sorted_costs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p50_cost_usd = Self::percentile(&sorted_costs, 50.0);
        let p95_cost_usd = Self::percentile(&sorted_costs, 95.0);
        let p99_cost_usd = Self::percentile(&sorted_costs, 99.0);

        Self {
            count,
            accuracy,
            mean_cost_usd,
            mean_duration_ms,
            mean_api_calls,
            mean_tokens,
            std_cost_usd,
            std_duration_ms,
            min_cost_usd,
            max_cost_usd,
            min_duration_ms,
            max_duration_ms,
            p50_cost_usd,
            p95_cost_usd,
            p99_cost_usd,
        }
    }

    fn percentile(sorted: &[f64], p: f64) -> f64 {
        if sorted.is_empty() {
            return 0.0;
        }
        let idx = (p / 100.0 * (sorted.len() - 1) as f64).round() as usize;
        sorted[idx.min(sorted.len() - 1)]
    }
}

impl Default for AggregateMetrics {
    fn default() -> Self {
        Self {
            count: 0,
            accuracy: 0.0,
            mean_cost_usd: 0.0,
            mean_duration_ms: 0.0,
            mean_api_calls: 0.0,
            mean_tokens: 0.0,
            std_cost_usd: 0.0,
            std_duration_ms: 0.0,
            min_cost_usd: 0.0,
            max_cost_usd: 0.0,
            min_duration_ms: 0,
            max_duration_ms: 0,
            p50_cost_usd: 0.0,
            p95_cost_usd: 0.0,
            p99_cost_usd: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_default() {
        let metrics = Metrics::default();
        assert!(!metrics.solved);
        assert_eq!(metrics.cost_usd, 0.0);
        assert_eq!(metrics.total_tokens(), 0);
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();
        collector.start();

        collector.record_api_call(100, 50, 20, 0.01);
        collector.record_file_read();
        collector.record_file_write();
        collector.record_agent_step();

        let metrics = collector.finish();
        assert_eq!(metrics.input_tokens, 100);
        assert_eq!(metrics.output_tokens, 50);
        assert_eq!(metrics.cached_tokens, 20);
        assert_eq!(metrics.total_tokens(), 150);
        assert_eq!(metrics.effective_tokens(), 130);
        assert_eq!(metrics.files_read, 1);
        assert_eq!(metrics.files_written, 1);
    }

    #[test]
    fn test_aggregate_metrics() {
        let metrics = vec![
            Metrics {
                solved: true,
                cost_usd: 0.01,
                duration_ms: 1000,
                api_calls: 1,
                input_tokens: 100,
                output_tokens: 50,
                ..Default::default()
            },
            Metrics {
                solved: false,
                cost_usd: 0.02,
                duration_ms: 2000,
                api_calls: 2,
                input_tokens: 200,
                output_tokens: 100,
                ..Default::default()
            },
            Metrics {
                solved: true,
                cost_usd: 0.015,
                duration_ms: 1500,
                api_calls: 1,
                input_tokens: 150,
                output_tokens: 75,
                ..Default::default()
            },
        ];

        let agg = AggregateMetrics::from_metrics(&metrics);
        assert_eq!(agg.count, 3);
        assert!((agg.accuracy - 66.67).abs() < 0.1);
        assert!((agg.mean_cost_usd - 0.015).abs() < 0.001);
    }
}
