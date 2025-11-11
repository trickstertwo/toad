//! Benchmark abstraction and orchestration
//!
//! This module provides infrastructure for running multiple benchmarks
//! (SWE-bench, LiveCodeBench, HumanEval+, etc.) through a unified interface.
//!
//! # Architecture
//!
//! The benchmark system is built around the [`BenchmarkExecutor`] trait, which
//! defines a common interface for different benchmark implementations. The
//! `Orchestrator` (Phase 5) coordinates concurrent execution of multiple benchmarks and
//! emits progress events.
//!
//! # Module Structure
//!
//! - [`types`]: Core data structures (Task, BenchmarkMetadata, ProgressEvent)
//! - `orchestrator`: Multi-benchmark concurrent executor (Phase 5)
//! - [`swebench`]: SWE-bench adapter
//! - `livecodebench`: LiveCodeBench stub (future implementation)
//!
//! # Examples
//!
//! ```rust,ignore
//! use toad::benchmarks::{BenchmarkExecutor, Task, ExecutionContext};
//! use async_trait::async_trait;
//!
//! // Define a benchmark executor
//! struct MyBenchmark;
//!
//! #[async_trait]
//! impl BenchmarkExecutor for MyBenchmark {
//!     async fn setup(&mut self) -> anyhow::Result<()> {
//!         // Load dataset
//!         Ok(())
//!     }
//!
//!     async fn run_task(&self, task: &Task, ctx: &ExecutionContext) -> TaskResult {
//!         // Execute task and return result
//!         unimplemented!()
//!     }
//!
//!     async fn cleanup(&mut self) -> anyhow::Result<()> {
//!         // Clean up resources
//!         Ok(())
//!     }
//!
//!     fn get_metadata(&self) -> &BenchmarkMetadata {
//!         // Return benchmark info
//!         unimplemented!()
//!     }
//! }
//! ```

use anyhow::Result;
use async_trait::async_trait;

pub mod types;
pub mod swebench;
pub mod livecodebench;

// Re-export core types for convenience
pub use types::{BenchmarkMetadata, ExecutionContext, ProgressEvent, Task};

/// Async trait for benchmark execution
///
/// Defines the interface for running different benchmarks (SWE-bench, LiveCodeBench, etc.)
/// through a unified API. Implementations must be Send + Sync for concurrent execution
/// in the orchestrator (Phase 5).
///
/// # Lifecycle
///
/// 1. **setup()**: Load dataset, initialize resources
/// 2. **run_task()**: Execute individual tasks (called multiple times)
/// 3. **cleanup()**: Release resources, cleanup temporary files
///
/// # Thread Safety
///
/// Implementations must be Send + Sync to enable:
/// - Concurrent execution of multiple benchmarks via `tokio::spawn`
/// - Shared access to LLM clients (wrapped in `Arc<>`)
/// - Safe passing across async boundaries
///
/// # Examples
///
/// ```rust,ignore
/// use toad::benchmarks::{BenchmarkExecutor, Task, ExecutionContext, BenchmarkMetadata};
/// use async_trait::async_trait;
///
/// struct MyBenchmark {
///     metadata: BenchmarkMetadata,
///     tasks: Vec<Task>,
/// }
///
/// #[async_trait]
/// impl BenchmarkExecutor for MyBenchmark {
///     async fn setup(&mut self) -> anyhow::Result<()> {
///         // Load tasks from dataset
///         self.tasks = load_tasks()?;
///         Ok(())
///     }
///
///     async fn run_task(&self, task: &Task, ctx: &ExecutionContext) -> TaskResult {
///         // Execute task with agent
///         let result = agent.execute(task, ctx).await?;
///         Ok(result)
///     }
///
///     async fn cleanup(&mut self) -> anyhow::Result<()> {
///         // Clean up temporary files
///         Ok(())
///     }
///
///     fn get_metadata(&self) -> &BenchmarkMetadata {
///         &self.metadata
///     }
/// }
/// ```
#[async_trait]
pub trait BenchmarkExecutor: Send + Sync {
    /// Initialize the benchmark executor
    ///
    /// Called once before any tasks are run. Use this to:
    /// - Load dataset from disk or download from URL
    /// - Validate dataset format and contents
    /// - Initialize any shared resources
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Dataset cannot be loaded or is malformed
    /// - Required dependencies are missing
    /// - Initialization fails for any reason
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// async fn setup(&mut self) -> Result<()> {
    ///     self.tasks = DatasetLoader::load("swebench_verified.json").await?;
    ///     tracing::info!("Loaded {} tasks", self.tasks.len());
    ///     Ok(())
    /// }
    /// ```
    async fn setup(&mut self) -> Result<()>;

    /// Execute a single task
    ///
    /// Run the task with the provided execution context and return the result.
    /// This method is called multiple times (once per task) and must be thread-safe.
    ///
    /// # Parameters
    ///
    /// - `task`: The task to execute (immutable borrow)
    /// - `ctx`: Execution configuration (timeout, max_steps, etc.)
    ///
    /// # Returns
    ///
    /// Returns `TaskResult` with:
    /// - `solved`: Whether the task was completed successfully
    /// - `duration_ms`: Execution time in milliseconds
    /// - `cost_usd`: API cost for this task
    /// - `metrics`: Detailed performance metrics
    /// - `error`: Error message if task failed
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// async fn run_task(&self, task: &Task, ctx: &ExecutionContext) -> TaskResult {
    ///     let start = Instant::now();
    ///
    ///     let mut result = TaskResult::new(task.id.clone());
    ///
    ///     match self.agent.execute(task, ctx).await {
    ///         Ok(solution) => {
    ///             result.mark_solved();
    ///             result.duration_ms = start.elapsed().as_millis() as u64;
    ///         }
    ///         Err(e) => {
    ///             result.mark_failed(e.to_string());
    ///         }
    ///     }
    ///
    ///     result
    /// }
    /// ```
    async fn run_task(
        &self,
        task: &Task,
        ctx: &ExecutionContext,
    ) -> crate::ai::evaluation::TaskResult;

    /// Clean up resources after all tasks complete
    ///
    /// Called once after all tasks finish (or evaluation is cancelled).
    /// Use this to:
    /// - Delete temporary files
    /// - Close database connections
    /// - Release system resources
    ///
    /// # Errors
    ///
    /// Returns error if cleanup fails. Non-fatal - orchestrator will log
    /// and continue.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// async fn cleanup(&mut self) -> Result<()> {
    ///     std::fs::remove_dir_all(&self.temp_dir)?;
    ///     tracing::info!("Cleaned up temporary directory");
    ///     Ok(())
    /// }
    /// ```
    async fn cleanup(&mut self) -> Result<()>;

    /// Get metadata about this benchmark
    ///
    /// Returns static information about the benchmark (name, version,
    /// contamination risk, etc.). Called by orchestrator for reporting.
    ///
    /// # Returns
    ///
    /// Reference to `BenchmarkMetadata` with:
    /// - `name`: Benchmark name (e.g., "SWE-bench Verified")
    /// - `version`: Version or release date
    /// - `total_tasks`: Number of tasks in dataset
    /// - `contamination_risk`: LOW/MEDIUM/HIGH/CERTAIN
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// fn get_metadata(&self) -> &BenchmarkMetadata {
    ///     &self.metadata
    /// }
    /// ```
    fn get_metadata(&self) -> &BenchmarkMetadata;
}
