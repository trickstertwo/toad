//! Core types for benchmark abstraction
//!
//! This module provides the fundamental data structures for abstracting different
//! benchmarks (SWE-bench, LiveCodeBench, HumanEval+, etc.) behind a common interface.
//!
//! # Architecture
//!
//! The benchmark system uses a trait-based abstraction (`BenchmarkExecutor` in `mod.rs`)
//! with shared types defined here. This allows different benchmarks to be executed
//! through a unified orchestrator without coupling to specific benchmark implementations.
//!
//! # Key Types
//!
//! - [`Task`]: A generic task representation that works across all benchmarks
//! - [`BenchmarkMetadata`]: Information about a benchmark (name, version, contamination risk)
//! - [`ExecutionContext`]: Configuration for task execution (timeouts, limits, config)
//! - [`ProgressEvent`]: Real-time progress updates during evaluation
//!
//! # Examples
//!
//! ```
//! use toad::benchmarks::types::{Task, BenchmarkMetadata};
//! use std::collections::HashMap;
//!
//! // Create a simple task
//! let task = Task {
//!     id: "task-001".to_string(),
//!     description: "Fix the bug in function foo()".to_string(),
//!     expected_output: Some("Tests pass".to_string()),
//!     metadata: HashMap::new(),
//! };
//!
//! // Define benchmark metadata
//! let metadata = BenchmarkMetadata {
//!     name: "SWE-bench Verified".to_string(),
//!     version: "1.0".to_string(),
//!     total_tasks: 500,
//!     dataset_url: Some("https://huggingface.co/datasets/princeton-nlp/SWE-bench_Verified".to_string()),
//!     license: Some("MIT".to_string()),
//!     contamination_risk: "LOW".to_string(),
//! };
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A generic task representation that works across all benchmarks
///
/// This structure abstracts the common elements of tasks from different benchmarks
/// (SWE-bench, LiveCodeBench, HumanEval+, etc.) into a unified format. Benchmark-specific
/// details are stored in the `metadata` field as JSON values.
///
/// # Design Rationale
///
/// Different benchmarks have different task structures:
/// - SWE-bench: GitHub issues with patches
/// - LiveCodeBench: Competitive programming problems
/// - HumanEval+: Function synthesis with test cases
///
/// Rather than creating a separate type for each, we use a flexible `metadata` field
/// that can store benchmark-specific data while keeping the core fields uniform.
///
/// # Examples
///
/// ```
/// use toad::benchmarks::types::Task;
/// use std::collections::HashMap;
///
/// let task = Task {
///     id: "django__django-12345".to_string(),
///     description: "Add support for PostgreSQL JSONB field".to_string(),
///     expected_output: Some("All tests pass".to_string()),
///     metadata: HashMap::from([
///         ("repo".to_string(), serde_json::json!("django/django")),
///         ("base_commit".to_string(), serde_json::json!("abc123")),
///     ]),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    /// Unique task identifier (e.g., "django__django-12345", "LC-1234", "HumanEval/42")
    pub id: String,

    /// Human-readable task description or problem statement
    pub description: String,

    /// Expected output or success criteria (if applicable)
    ///
    /// Some benchmarks have clear expected outputs (HumanEval: test cases pass),
    /// while others are more subjective (SWE-bench: patch quality). This field
    /// captures the success criteria when available.
    pub expected_output: Option<String>,

    /// Benchmark-specific metadata as JSON values
    ///
    /// Common keys by benchmark:
    /// - SWE-bench: `repo`, `base_commit`, `test_patch`, `hints`
    /// - LiveCodeBench: `difficulty`, `time_limit`, `memory_limit`, `test_cases`
    /// - HumanEval+: `function_signature`, `docstring`, `canonical_solution`
    ///
    /// Use `serde_json::Value` for flexibility - allows any JSON-serializable type.
    pub metadata: HashMap<String, Value>,
}

impl Task {
    /// Create a minimal task for testing purposes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::benchmarks::types::Task;
    ///
    /// let task = Task::example();
    /// assert_eq!(task.id, "test-001");
    /// assert!(!task.description.is_empty());
    /// ```
    pub fn example() -> Self {
        Self {
            id: "test-001".to_string(),
            description: "Fix the bug in function foo()".to_string(),
            expected_output: Some("Tests pass".to_string()),
            metadata: HashMap::new(),
        }
    }
}

/// Metadata about a benchmark dataset
///
/// Contains information about the benchmark itself (not individual tasks).
/// Used for documentation, contamination risk assessment, and results reporting.
///
/// # Contamination Risk Levels
///
/// - `LOW`: Benchmark released after model training cutoff (e.g., LiveCodeBench 2024)
/// - `MEDIUM`: Benchmark may have some overlap with training data
/// - `HIGH`: Benchmark known to be in training data (e.g., HumanEval in GPT-3.5 training)
/// - `CERTAIN`: Benchmark definitely contaminated (proven via memorization tests)
///
/// # Examples
///
/// ```
/// use toad::benchmarks::types::BenchmarkMetadata;
///
/// let metadata = BenchmarkMetadata {
///     name: "SWE-bench Verified".to_string(),
///     version: "1.0".to_string(),
///     total_tasks: 500,
///     dataset_url: Some("https://huggingface.co/datasets/princeton-nlp/SWE-bench_Verified".to_string()),
///     license: Some("MIT".to_string()),
///     contamination_risk: "LOW".to_string(),
/// };
///
/// assert_eq!(metadata.total_tasks, 500);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkMetadata {
    /// Benchmark name (e.g., "SWE-bench Verified", "LiveCodeBench v6", "HumanEval+")
    pub name: String,

    /// Version or release date (e.g., "1.0", "2024-05", "v3.2")
    pub version: String,

    /// Total number of tasks in this benchmark
    pub total_tasks: usize,

    /// URL to dataset or documentation (if publicly available)
    pub dataset_url: Option<String>,

    /// License information (e.g., "MIT", "Apache-2.0", "CC-BY-4.0")
    pub license: Option<String>,

    /// Contamination risk level: LOW, MEDIUM, HIGH, CERTAIN
    ///
    /// Used for contamination detection and analysis (Phase 3).
    /// See module-level docs for risk level definitions.
    pub contamination_risk: String,
}

/// Execution context for running a task
///
/// Contains configuration and constraints for task execution. Passed to
/// `BenchmarkExecutor::run_task()` to control execution behavior.
///
/// # Examples
///
/// ```
/// use toad::benchmarks::types::ExecutionContext;
/// use std::time::Duration;
///
/// let ctx = ExecutionContext {
///     timeout: Duration::from_secs(300), // 5 minutes
///     max_steps: 25,
///     system_config: serde_json::json!({
///         "model": "claude-sonnet-3-5-20241022",
///         "temperature": 1.0,
///     }),
///     sandbox_config: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// Maximum execution time per task
    ///
    /// Tasks that exceed this timeout should be terminated gracefully with an error.
    /// Default: 5 minutes (300 seconds)
    #[serde(with = "humantime_serde")]
    pub timeout: std::time::Duration,

    /// Maximum agent steps per task
    ///
    /// Prevents infinite loops in agent execution. Agent should stop after this many
    /// steps even if task is not complete.
    /// Default: 25 steps (from existing M0 implementation)
    pub max_steps: usize,

    /// System configuration as JSON
    ///
    /// Contains model selection, feature flags, and other system-level config.
    /// Usually a serialized `ToadConfig` but kept as `Value` for flexibility.
    pub system_config: Value,

    /// Optional sandbox configuration
    ///
    /// For benchmarks that require isolated execution environments (e.g., SWE-bench
    /// running in Docker). Format is benchmark-specific.
    pub sandbox_config: Option<Value>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(300), // 5 minutes
            max_steps: 25,
            system_config: serde_json::json!({}),
            sandbox_config: None,
        }
    }
}

/// Progress events emitted during evaluation
///
/// The orchestrator sends these events through an async channel to provide real-time
/// updates to CLI/TUI. This enables live progress displays and incremental result storage.
///
/// # Event Flow
///
/// 1. `EvaluationStarted` - Orchestrator begins
/// 2. `BenchmarkStarted` - For each benchmark
/// 3. `TaskCompleted` - For each task (many events)
/// 4. `BenchmarkCompleted` - When benchmark finishes
/// 5. `EvaluationCompleted` - Orchestrator done
///
/// # Examples
///
/// ```
/// use toad::benchmarks::types::ProgressEvent;
/// use tokio::sync::mpsc;
///
/// #[tokio::main]
/// async fn main() {
///     let (tx, mut rx) = mpsc::unbounded_channel::<ProgressEvent>();
///
///     // Producer (orchestrator)
///     tokio::spawn(async move {
///         tx.send(ProgressEvent::BenchmarkStarted {
///             benchmark_name: "SWE-bench".to_string(),
///             total_tasks: 500,
///         }).unwrap();
///     });
///
///     // Consumer (TUI)
///     while let Some(event) = rx.recv().await {
///         match event {
///             ProgressEvent::BenchmarkStarted { benchmark_name, total_tasks } => {
///                 println!("Starting {}: {} tasks", benchmark_name, total_tasks);
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ProgressEvent {
    /// Evaluation started across all benchmarks
    EvaluationStarted {
        /// Unique run ID for this evaluation
        run_id: String,
        /// Names of benchmarks being run
        benchmarks: Vec<String>,
        /// Total tasks across all benchmarks
        total_tasks: usize,
    },

    /// A specific benchmark started executing
    BenchmarkStarted {
        /// Benchmark name (e.g., "SWE-bench Verified")
        benchmark_name: String,
        /// Number of tasks in this benchmark
        total_tasks: usize,
    },

    /// A single task completed (success or failure)
    TaskCompleted {
        /// Benchmark this task belongs to
        benchmark_name: String,
        /// Task identifier
        task_id: String,
        /// Task index (0-based)
        task_index: usize,
        /// Total tasks in this benchmark
        total_tasks: usize,
        /// Was the task solved successfully?
        solved: bool,
        /// Execution duration in milliseconds
        duration_ms: u64,
        /// API cost in USD
        cost_usd: f64,
    },

    /// A benchmark completed all tasks
    BenchmarkCompleted {
        /// Benchmark name
        benchmark_name: String,
        /// Number of tasks solved successfully
        tasks_solved: usize,
        /// Total tasks attempted
        total_tasks: usize,
        /// Total execution duration in milliseconds
        duration_ms: u64,
        /// Total cost in USD
        cost_usd: f64,
    },

    /// Entire evaluation completed
    EvaluationCompleted {
        /// Run ID
        run_id: String,
        /// Number of benchmarks completed
        benchmarks_completed: usize,
        /// Total tasks solved across all benchmarks
        tasks_solved: usize,
        /// Total tasks attempted
        total_tasks: usize,
        /// Total duration in milliseconds
        duration_ms: u64,
        /// Total cost in USD
        cost_usd: f64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation_and_serialization() {
        // Create a task
        let task = Task {
            id: "test-123".to_string(),
            description: "Implement feature X".to_string(),
            expected_output: Some("Tests pass".to_string()),
            metadata: HashMap::from([
                ("difficulty".to_string(), serde_json::json!("medium")),
                ("tags".to_string(), serde_json::json!(["rust", "cli"])),
            ]),
        };

        // Verify fields
        assert_eq!(task.id, "test-123");
        assert_eq!(task.description, "Implement feature X");
        assert_eq!(task.expected_output, Some("Tests pass".to_string()));
        assert_eq!(task.metadata.len(), 2);

        // Test JSON serialization roundtrip
        let json = serde_json::to_string(&task).expect("Failed to serialize");
        let deserialized: Task = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(task, deserialized);
    }

    #[test]
    fn test_benchmark_metadata_validation() {
        let metadata = BenchmarkMetadata {
            name: "Test Benchmark".to_string(),
            version: "1.0".to_string(),
            total_tasks: 100,
            dataset_url: Some("https://example.com/dataset".to_string()),
            license: Some("MIT".to_string()),
            contamination_risk: "LOW".to_string(),
        };

        // Verify fields
        assert_eq!(metadata.name, "Test Benchmark");
        assert_eq!(metadata.total_tasks, 100);
        assert_eq!(metadata.contamination_risk, "LOW");

        // Test serialization
        let json = serde_json::to_string(&metadata).expect("Failed to serialize");
        let deserialized: BenchmarkMetadata =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(metadata, deserialized);
    }

    #[test]
    fn test_execution_context_defaults() {
        let ctx = ExecutionContext::default();

        // Verify default values
        assert_eq!(ctx.timeout, std::time::Duration::from_secs(300));
        assert_eq!(ctx.max_steps, 25);
        assert_eq!(ctx.system_config, serde_json::json!({}));
        assert!(ctx.sandbox_config.is_none());

        // Test custom context
        let custom_ctx = ExecutionContext {
            timeout: std::time::Duration::from_secs(600),
            max_steps: 50,
            system_config: serde_json::json!({"model": "claude-opus-3"}),
            sandbox_config: Some(serde_json::json!({"docker_image": "python:3.11"})),
        };

        // Test serialization roundtrip
        let json = serde_json::to_string(&custom_ctx).expect("Failed to serialize");
        let deserialized: ExecutionContext =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(custom_ctx.timeout, deserialized.timeout);
        assert_eq!(custom_ctx.max_steps, deserialized.max_steps);
    }
}
