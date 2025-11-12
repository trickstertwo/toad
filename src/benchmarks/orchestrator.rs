//! Multi-benchmark orchestrator with concurrent execution
//!
//! This module provides the [`Orchestrator`] for running multiple benchmarks concurrently
//! while emitting real-time progress events and respecting resource limits.
//!
//! # Architecture
//!
//! The orchestrator follows these design principles:
//!
//! 1. **Concurrency**: Multiple benchmarks run in parallel via `tokio::spawn`
//! 2. **Resource limits**: Configurable semaphore prevents resource exhaustion
//! 3. **Progress tracking**: Unbounded channel streams [`ProgressEvent`] to consumers
//! 4. **Cancellation**: Supports graceful shutdown via `CancellationToken`
//! 5. **Fault isolation**: Benchmark panics don't crash the orchestrator
//!
//! # Execution Flow
//!
//! ```text
//! 1. Create Orchestrator with config
//! 2. Call run_evaluation() → returns (EvaluationRun, Receiver<ProgressEvent>)
//! 3. Orchestrator spawns tasks for each benchmark
//! 4. Each task:
//!    - Acquires semaphore permit (limits concurrency)
//!    - Sends BenchmarkStarted event
//!    - Runs tasks sequentially within benchmark
//!    - Sends TaskCompleted events
//!    - Sends BenchmarkCompleted event
//!    - Releases semaphore permit
//! 5. Main task waits for all benchmarks, sends EvaluationCompleted
//! 6. Channel closes, consumer receives all events
//! ```
//!
//! # Examples
//!
//! ```rust,ignore
//! use toad::benchmarks::{Orchestrator, OrchestratorConfig};
//! use tokio_util::sync::CancellationToken;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = OrchestratorConfig {
//!         benchmarks: vec!["swebench-verified".to_string()],
//!         task_limit: Some(10),
//!         max_concurrent_benchmarks: 2,
//!         execution_context: Default::default(),
//!     };
//!
//!     let cancel_token = CancellationToken::new();
//!     let orchestrator = Orchestrator::new(config, cancel_token.clone());
//!
//!     let (run, mut progress_rx) = orchestrator.run_evaluation().await?;
//!
//!     // Consume progress events
//!     while let Some(event) = progress_rx.recv().await {
//!         println!("Progress: {:?}", event);
//!     }
//!
//!     println!("Evaluation complete: {} tasks, {}% accuracy",
//!              run.aggregate_metrics.total_tasks,
//!              run.aggregate_metrics.accuracy * 100.0);
//!
//!     Ok(())
//! }
//! ```

use crate::ai::evaluation::{EvaluationRun, TaskResult, BenchmarkResult};
use crate::benchmarks::{get_executor, BenchmarkMetadata, ExecutionContext};
use crate::benchmarks::types::{ProgressEvent, Task};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Semaphore};
use tokio_util::sync::CancellationToken;
use crate::ai::evaluation::storage::StorageManager;

/// Configuration for the orchestrator
///
/// Defines which benchmarks to run, resource limits, and execution parameters.
///
/// # Examples
///
/// ```
/// use toad::benchmarks::OrchestratorConfig;
/// use toad::benchmarks::ExecutionContext;
///
/// let config = OrchestratorConfig {
///     benchmarks: vec!["swebench-verified".to_string(), "livecodebench".to_string()],
///     task_limit: Some(50), // Limit to first 50 tasks per benchmark
///     max_concurrent_benchmarks: 2, // Run 2 benchmarks at once
///     execution_context: ExecutionContext::default(),
/// };
/// ```
#[derive(Debug, Clone, serde::Serialize)]
pub struct OrchestratorConfig {
    /// Benchmark names to execute (e.g., "swebench-verified", "livecodebench")
    pub benchmarks: Vec<String>,

    /// Limit number of tasks per benchmark (None = run all tasks)
    ///
    /// Useful for quick testing or resource-constrained environments.
    pub task_limit: Option<usize>,

    /// Maximum number of benchmarks to run concurrently
    ///
    /// Default: 2 (balances speed vs resource usage)
    /// Higher values increase memory/CPU but reduce total duration.
    pub max_concurrent_benchmarks: usize,

    /// Execution context passed to each task
    ///
    /// Contains timeouts, max steps, system config, sandbox config.
    pub execution_context: ExecutionContext,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            benchmarks: vec!["swebench-verified".to_string()],
            task_limit: None,
            max_concurrent_benchmarks: 2,
            execution_context: ExecutionContext::default(),
        }
    }
}

/// Multi-benchmark orchestrator
///
/// Coordinates execution of multiple benchmarks concurrently, emitting progress events
/// and collecting results into a unified [`EvaluationRun`].
///
/// # Concurrency Model
///
/// - **Benchmark-level parallelism**: Multiple benchmarks run concurrently (controlled by semaphore)
/// - **Task-level serialization**: Within each benchmark, tasks run sequentially
/// - **Rationale**: Task-level parallelism would complicate rate limiting and resource management
///
/// # Cancellation
///
/// The orchestrator supports graceful cancellation via `CancellationToken`. When cancelled:
/// - In-flight tasks complete
/// - Remaining tasks are skipped
/// - Progress events are sent for completed work
/// - Partial results are returned
///
/// # Examples
///
/// ```rust,ignore
/// use toad::benchmarks::{Orchestrator, OrchestratorConfig};
/// use tokio_util::sync::CancellationToken;
///
/// let config = OrchestratorConfig::default();
/// let cancel_token = CancellationToken::new();
/// let orchestrator = Orchestrator::new(config, cancel_token);
///
/// let (run, progress_rx) = orchestrator.run_evaluation().await?;
/// ```
pub struct Orchestrator {
    /// Configuration for this orchestration
    config: OrchestratorConfig,

    /// Cancellation token for graceful shutdown
    cancel_token: CancellationToken,
}

impl Orchestrator {
    /// Create a new orchestrator with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Orchestrator configuration (benchmarks, limits, context)
    /// * `cancel_token` - Token for cancelling the evaluation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::benchmarks::{Orchestrator, OrchestratorConfig};
    /// use tokio_util::sync::CancellationToken;
    ///
    /// let config = OrchestratorConfig::default();
    /// let cancel_token = CancellationToken::new();
    /// let orchestrator = Orchestrator::new(config, cancel_token);
    /// ```
    pub fn new(config: OrchestratorConfig, cancel_token: CancellationToken) -> Self {
        Self {
            config,
            cancel_token,
        }
    }

    /// Run the evaluation across all configured benchmarks
    ///
    /// This method:
    /// 1. Creates a progress event channel
    /// 2. Spawns concurrent tasks for each benchmark
    /// 3. Collects results from all benchmarks
    /// 4. Aggregates metrics across benchmarks
    /// 5. Returns the complete evaluation run
    ///
    /// # Returns
    ///
    /// - `EvaluationRun`: Complete results including per-benchmark and aggregate metrics
    /// - `Receiver<ProgressEvent>`: Stream of progress events (closed when evaluation completes)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No benchmarks specified in config
    /// - Benchmark executor creation fails
    /// - Benchmark setup fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let (run, mut progress_rx) = orchestrator.run_evaluation().await?;
    ///
    /// // Consume progress in separate task
    /// tokio::spawn(async move {
    ///     while let Some(event) = progress_rx.recv().await {
    ///         println!("{:?}", event);
    ///     }
    /// });
    ///
    /// println!("Accuracy: {}%", run.aggregate_metrics.accuracy * 100.0);
    /// ```
    pub async fn run_evaluation(
        self,
    ) -> Result<(EvaluationRun, mpsc::UnboundedReceiver<ProgressEvent>)> {
        if self.config.benchmarks.is_empty() {
            anyhow::bail!("No benchmarks specified in orchestrator config");
        }

        // Create progress channel (unbounded to prevent backpressure)
        let (progress_tx, progress_rx) = mpsc::unbounded_channel::<ProgressEvent>();

        // Generate unique run ID
        let run_id = StorageManager::generate_run_id();
        let start_time = Instant::now();

        // Send initial progress event
        let _ = progress_tx.send(ProgressEvent::EvaluationStarted {
            run_id: run_id.clone(),
            benchmarks: self.config.benchmarks.clone(),
            total_tasks: 0, // Will be updated as benchmarks report their task counts
        });

        // Create semaphore for concurrency control
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_benchmarks));

        // Spawn tasks for each benchmark
        let mut benchmark_handles = Vec::new();

        for benchmark_name in &self.config.benchmarks {
            let benchmark_name = benchmark_name.clone();
            let config = self.config.clone();
            let progress_tx = progress_tx.clone();
            let cancel_token = self.cancel_token.clone();
            let semaphore = Arc::clone(&semaphore);

            let handle = tokio::spawn(async move {
                // Acquire semaphore permit (limits concurrent benchmarks)
                let _permit = semaphore.acquire().await.expect("Semaphore closed");

                // Check for cancellation before starting
                if cancel_token.is_cancelled() {
                    // Return empty result for cancelled benchmark
                    let empty_metadata = BenchmarkMetadata {
                        name: benchmark_name.clone(),
                        version: "unknown".to_string(),
                        total_tasks: 0,
                        dataset_url: None,
                        license: None,
                        contamination_risk: "UNKNOWN".to_string(),
                    };
                    return Ok(BenchmarkResult {
                        benchmark_metadata: empty_metadata,
                        task_results: vec![],
                        duration_ms: 0,
                        success_rate: 0.0,
                        total_cost_usd: 0.0,
                        avg_cost_per_task_usd: 0.0,
                        avg_duration_per_task_ms: 0.0,
                    });
                }

                Self::run_benchmark(
                    &benchmark_name,
                    &config,
                    progress_tx,
                    cancel_token,
                )
                .await
            });

            benchmark_handles.push(handle);
        }

        // Drop the original progress_tx so channel closes when all tasks are done
        drop(progress_tx);

        // Wait for all benchmarks to complete
        let mut benchmark_results = Vec::new();
        for handle in benchmark_handles {
            match handle.await {
                Ok(Ok(result)) => benchmark_results.push(result),
                Ok(Err(e)) => {
                    // Benchmark failed, log error but continue with other benchmarks
                    tracing::error!("Benchmark failed: {:#}", e);
                }
                Err(e) => {
                    // Task panicked
                    tracing::error!("Benchmark task panicked: {:#}", e);
                }
            }
        }

        // Aggregate metrics across all benchmarks
        let aggregate_metrics = Self::aggregate_results(&benchmark_results);

        // Calculate total duration
        let _duration = start_time.elapsed();

        // Create evaluation run
        let evaluation_run = EvaluationRun {
            run_id,
            timestamp: chrono::Utc::now(),
            benchmark_results,
            aggregate_metrics,
            format_version: 1,
            config_snapshot: serde_json::to_value(&self.config)?,
        };

        Ok((evaluation_run, progress_rx))
    }

    /// Run a single benchmark
    ///
    /// This method:
    /// 1. Creates the benchmark executor
    /// 2. Sets up the benchmark (downloads datasets, etc.)
    /// 3. Runs tasks sequentially
    /// 4. Sends progress events for each task
    /// 5. Returns the benchmark result
    ///
    /// # Arguments
    ///
    /// * `benchmark_name` - Name of the benchmark (e.g., "swebench-verified")
    /// * `config` - Orchestrator configuration
    /// * `progress_tx` - Channel for sending progress events
    /// * `cancel_token` - Token for checking cancellation
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Executor creation fails
    /// - Benchmark setup fails
    /// - Individual tasks fail (errors are collected, not propagated)
    async fn run_benchmark(
        benchmark_name: &str,
        config: &OrchestratorConfig,
        progress_tx: mpsc::UnboundedSender<ProgressEvent>,
        cancel_token: CancellationToken,
    ) -> Result<BenchmarkResult> {
        let benchmark_start = Instant::now();

        // Create executor
        let mut executor = get_executor(benchmark_name)
            .with_context(|| format!("Failed to create executor for benchmark '{}'", benchmark_name))?;

        // Setup benchmark (download datasets, etc.)
        executor.setup().await
            .with_context(|| format!("Failed to setup benchmark '{}'", benchmark_name))?;

        // Get metadata
        let metadata = executor.get_metadata().clone();

        // Determine task list (mock for now - actual implementation depends on executor)
        // TODO: Add get_tasks() method to BenchmarkExecutor trait in future
        let total_tasks = config.task_limit.unwrap_or(metadata.total_tasks);

        // Send benchmark started event
        let _ = progress_tx.send(ProgressEvent::BenchmarkStarted {
            benchmark_name: metadata.name.clone(),
            total_tasks,
        });

        // Run tasks sequentially
        let mut task_results = Vec::new();
        let mut tasks_solved = 0;
        let mut total_cost = 0.0;

        for task_index in 0..total_tasks {
            // Check for cancellation
            if cancel_token.is_cancelled() {
                tracing::info!("Benchmark '{}' cancelled at task {}/{}", benchmark_name, task_index, total_tasks);
                break;
            }

            // Create mock task (TODO: Get from executor)
            let task = Task {
                id: format!("{}-task-{}", benchmark_name, task_index),
                description: format!("Task {} for {}", task_index, benchmark_name),
                expected_output: None,
                metadata: std::collections::HashMap::new(),
            };

            let task_start = Instant::now();

            // Run task
            let result = executor.run_task(&task, &config.execution_context).await;

            let task_duration = task_start.elapsed();

            // Track solved status
            if result.solved {
                tasks_solved += 1;
            }

            total_cost += result.cost_usd;

            // Send progress event
            let _ = progress_tx.send(ProgressEvent::TaskCompleted {
                benchmark_name: metadata.name.clone(),
                task_id: task.id.clone(),
                task_index,
                total_tasks,
                solved: result.solved,
                duration_ms: task_duration.as_millis() as u64,
                cost_usd: result.cost_usd,
            });

            task_results.push(result);
        }

        let benchmark_duration = benchmark_start.elapsed();

        // Send benchmark completed event
        let _ = progress_tx.send(ProgressEvent::BenchmarkCompleted {
            benchmark_name: metadata.name.clone(),
            tasks_solved,
            total_tasks: task_results.len(),
            duration_ms: benchmark_duration.as_millis() as u64,
            cost_usd: total_cost,
        });

        // Cleanup
        let _ = executor.cleanup().await;

        // Calculate metrics
        let success_rate = if task_results.is_empty() {
            0.0
        } else {
            tasks_solved as f64 / task_results.len() as f64
        };

        let avg_cost_per_task_usd = if task_results.is_empty() {
            0.0
        } else {
            total_cost / task_results.len() as f64
        };

        let avg_duration_per_task_ms = if task_results.is_empty() {
            0.0
        } else {
            benchmark_duration.as_millis() as f64 / task_results.len() as f64
        };

        // Create benchmark result
        Ok(BenchmarkResult {
            benchmark_metadata: metadata,
            task_results,
            duration_ms: benchmark_duration.as_millis() as u64,
            success_rate,
            total_cost_usd: total_cost,
            avg_cost_per_task_usd,
            avg_duration_per_task_ms,
        })
    }

    /// Aggregate metrics across multiple benchmark results
    ///
    /// Computes overall accuracy, total tasks, costs, and median latency.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let results = vec![benchmark1_result, benchmark2_result];
    /// let aggregate = Orchestrator::aggregate_results(&results);
    /// println!("Overall accuracy: {}%", aggregate.mean_accuracy * 100.0);
    /// ```
    fn aggregate_results(results: &[BenchmarkResult]) -> crate::ai::evaluation::AggregateMetrics {
        // Count total tasks and solved tasks
        let total_tasks: usize = results.iter().map(|r| r.task_results.len()).sum();
        let tasks_solved: usize = results.iter()
            .map(|r| r.task_results.iter().filter(|t| t.solved).count())
            .sum();

        // Sum total cost
        let total_cost: f64 = results.iter().map(|r| r.total_cost_usd).sum();

        // Collect all task durations for median calculation
        let mut all_durations: Vec<u64> = results.iter()
            .flat_map(|r| r.task_results.iter().map(|t| t.duration_ms))
            .collect();

        // Calculate median latency
        let median_latency_ms = if all_durations.is_empty() {
            0.0
        } else {
            all_durations.sort_unstable();
            let mid = all_durations.len() / 2;
            if all_durations.len() % 2 == 0 {
                (all_durations[mid - 1] + all_durations[mid]) as f64 / 2.0
            } else {
                all_durations[mid] as f64
            }
        };

        // Calculate mean accuracy (weighted by tasks)
        let mean_accuracy = if total_tasks == 0 {
            0.0
        } else {
            tasks_solved as f64 / total_tasks as f64
        };

        crate::ai::evaluation::AggregateMetrics {
            mean_accuracy,
            median_latency_ms,
            total_cost_usd: total_cost,
            total_tasks,
            tasks_solved,
            behavioral_metrics: None, // Not computed here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_config_default() {
        let config = OrchestratorConfig::default();

        assert_eq!(config.benchmarks, vec!["swebench-verified"]);
        assert_eq!(config.task_limit, None);
        assert_eq!(config.max_concurrent_benchmarks, 2);
    }

    #[test]
    fn test_aggregate_results_empty() {
        let results = vec![];
        let aggregate = Orchestrator::aggregate_results(&results);

        assert_eq!(aggregate.total_tasks, 0);
        assert_eq!(aggregate.tasks_solved, 0);
        assert_eq!(aggregate.mean_accuracy, 0.0);
        assert_eq!(aggregate.total_cost_usd, 0.0);
        assert_eq!(aggregate.median_latency_ms, 0.0);
    }

    #[test]
    fn test_aggregate_results_multiple_benchmarks() {
        let metadata1 = BenchmarkMetadata {
            name: "Benchmark 1".to_string(),
            version: "1.0".to_string(),
            total_tasks: 2,
            dataset_url: None,
            license: None,
            contamination_risk: "LOW".to_string(),
        };

        let metadata2 = BenchmarkMetadata {
            name: "Benchmark 2".to_string(),
            version: "1.0".to_string(),
            total_tasks: 3,
            dataset_url: None,
            license: None,
            contamination_risk: "LOW".to_string(),
        };

        // Create task results using new() constructor
        let mut task1_1 = TaskResult::new("task-1-1".to_string());
        task1_1.solved = true;
        task1_1.duration_ms = 1000;

        let mut task1_2 = TaskResult::new("task-1-2".to_string());
        task1_2.solved = false;
        task1_2.duration_ms = 2000;

        let mut task2_1 = TaskResult::new("task-2-1".to_string());
        task2_1.solved = true;
        task2_1.duration_ms = 1500;

        let mut task2_2 = TaskResult::new("task-2-2".to_string());
        task2_2.solved = true;
        task2_2.duration_ms = 1500;

        let mut task2_3 = TaskResult::new("task-2-3".to_string());
        task2_3.solved = false;
        task2_3.duration_ms = 2000;

        let result1 = BenchmarkResult {
            benchmark_metadata: metadata1,
            task_results: vec![task1_1, task1_2],
            duration_ms: 3000,
            success_rate: 0.5,
            total_cost_usd: 0.50,
            avg_cost_per_task_usd: 0.25,
            avg_duration_per_task_ms: 1500.0,
        };

        let result2 = BenchmarkResult {
            benchmark_metadata: metadata2,
            task_results: vec![task2_1, task2_2, task2_3],
            duration_ms: 5000,
            success_rate: 0.666,
            total_cost_usd: 0.75,
            avg_cost_per_task_usd: 0.25,
            avg_duration_per_task_ms: 1666.67,
        };

        let aggregate = Orchestrator::aggregate_results(&[result1, result2]);

        assert_eq!(aggregate.total_tasks, 5);
        assert_eq!(aggregate.tasks_solved, 3);
        assert!((aggregate.mean_accuracy - 0.6).abs() < 0.01);
        assert_eq!(aggregate.total_cost_usd, 1.25);
        // Median of [1000, 1500, 1500, 2000, 2000] = 1500
        assert_eq!(aggregate.median_latency_ms, 1500.0);
    }
}
