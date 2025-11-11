//! Benchmark abstraction and orchestration
//!
//! This module provides infrastructure for running multiple benchmarks
//! (SWE-bench, LiveCodeBench, HumanEval+, etc.) through a unified interface.
//!
//! # Architecture
//!
//! The benchmark system is built around the `BenchmarkExecutor` trait (Phase 2), which
//! defines a common interface for different benchmark implementations. The
//! `Orchestrator` (Phase 5) coordinates concurrent execution of multiple benchmarks and
//! emits progress events.
//!
//! # Module Structure
//!
//! - `types`: Core data structures (Task, BenchmarkMetadata, ProgressEvent)
//! - `orchestrator`: Multi-benchmark concurrent executor (Phase 5)
//! - `swebench`: SWE-bench adapter (Phase 2)
//! - `livecodebench`: LiveCodeBench stub (Phase 2 implementation)
//!
//! # Examples
//!
//! ```rust,ignore
//! use toad::benchmarks::{BenchmarkExecutor, Task, ExecutionContext};
//!
//! // Define a benchmark executor
//! struct MyBenchmark;
//!
//! #[async_trait]
//! impl BenchmarkExecutor for MyBenchmark {
//!     async fn setup(&mut self) -> Result<()> {
//!         // Load dataset
//!         Ok(())
//!     }
//!
//!     async fn run_task(&self, task: &Task, ctx: &ExecutionContext) -> TaskResult {
//!         // Execute task and return result
//!         unimplemented!()
//!     }
//!
//!     async fn cleanup(&mut self) -> Result<()> {
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

pub mod types;

// Re-export core types for convenience
pub use types::{BenchmarkMetadata, ExecutionContext, ProgressEvent, Task};
