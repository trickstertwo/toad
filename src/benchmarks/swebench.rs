//! SWE-bench benchmark executor adapter
//!
//! This module provides a `BenchmarkExecutor` implementation for SWE-bench that wraps
//! the existing M0 evaluation infrastructure. It delegates to the existing `Agent`,
//! `DatasetManager`, and `EvaluationHarness` code while conforming to the new
//! multi-benchmark abstraction.
//!
//! # Architecture
//!
//! The adapter pattern allows us to reuse all existing SWE-bench code without modification:
//! - **Dataset loading**: Delegates to `DatasetManager` (Verified/Lite/Full)
//! - **Task execution**: Uses existing `Agent` and tool registry
//! - **Metrics collection**: Leverages `MetricsCollector` from M0
//! - **Result formatting**: Converts between SWE-bench `Task` and generic `Task`
//!
//! # Examples
//!
//! ```rust,ignore
//! use toad::benchmarks::{BenchmarkExecutor, ExecutionContext};
//! use toad::benchmarks::swebench::SweBenchExecutor;
//! use toad::ai::evaluation::DatasetSource;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create executor for SWE-bench Verified
//!     let mut executor = SweBenchExecutor::new(DatasetSource::Verified);
//!
//!     // Setup (loads dataset)
//!     executor.setup().await?;
//!
//!     // Get tasks and run
//!     let metadata = executor.get_metadata();
//!     println!("Running {} tasks", metadata.total_tasks);
//!
//!     Ok(())
//! }
//! ```

use crate::ai::agent::Agent;
use crate::ai::evaluation::{DatasetManager, DatasetSource, TaskResult};
use crate::ai::llm::{AnthropicClient, get_api_key};
use crate::ai::metrics::MetricsCollector;
use crate::ai::tools::ToolRegistry;
use crate::benchmarks::{BenchmarkExecutor, BenchmarkMetadata, ExecutionContext, Task};
use crate::config::ToadConfig;
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::time::Instant;

/// SWE-bench benchmark executor
///
/// Wraps existing SWE-bench evaluation code (M0) into the new `BenchmarkExecutor`
/// trait for multi-benchmark abstraction. This allows SWE-bench to be run alongside
/// other benchmarks (LiveCodeBench, HumanEval+) through a unified orchestrator (Phase 5).
///
/// # Dataset Variants
///
/// - **Verified** (500 tasks): Human-verified, highest quality
/// - **Lite** (300 tasks): Representative subset
/// - **Full** (2,294 tasks): Complete dataset
///
/// # Examples
///
/// ```rust,ignore
/// use toad::benchmarks::swebench::SweBenchExecutor;
/// use toad::ai::evaluation::DatasetSource;
///
/// let mut executor = SweBenchExecutor::new(DatasetSource::Verified);
/// executor.setup().await?;
/// ```
pub struct SweBenchExecutor {
    /// Dataset source (Verified/Lite/Full)
    source: DatasetSource,

    /// Loaded tasks (populated by setup())
    tasks: Vec<crate::ai::evaluation::Task>,

    /// Dataset manager for loading/caching
    dataset_manager: DatasetManager,

    /// Metadata about this benchmark
    metadata: BenchmarkMetadata,
}

impl SweBenchExecutor {
    /// Create a new SWE-bench executor
    ///
    /// # Parameters
    ///
    /// - `source`: Dataset variant (Verified, Lite, or Full)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::benchmarks::swebench::SweBenchExecutor;
    /// use toad::ai::evaluation::DatasetSource;
    ///
    /// let executor = SweBenchExecutor::new(DatasetSource::Verified);
    /// ```
    pub fn new(source: DatasetSource) -> Self {
        // Determine metadata based on source
        let (name, total_tasks, dataset_url) = match &source {
            DatasetSource::Verified => (
                "SWE-bench Verified".to_string(),
                500,
                Some("https://huggingface.co/datasets/princeton-nlp/SWE-bench_Verified".to_string()),
            ),
            DatasetSource::Lite => (
                "SWE-bench Lite".to_string(),
                300,
                Some("https://huggingface.co/datasets/princeton-nlp/SWE-bench_Lite".to_string()),
            ),
            DatasetSource::Full => (
                "SWE-bench Full".to_string(),
                2294,
                Some("https://huggingface.co/datasets/princeton-nlp/SWE-bench".to_string()),
            ),
            DatasetSource::Local(path) => (
                format!("SWE-bench ({})", path.display()),
                0, // Unknown until loaded
                None,
            ),
        };

        let metadata = BenchmarkMetadata {
            name,
            version: "1.0".to_string(),
            total_tasks,
            dataset_url,
            license: Some("MIT".to_string()),
            contamination_risk: "LOW".to_string(), // Post-training release
        };

        Self {
            source,
            tasks: Vec::new(),
            dataset_manager: DatasetManager::default(),
            metadata,
        }
    }

    /// Convert SWE-bench Task to generic Task
    ///
    /// Maps SWE-bench-specific fields into the flexible `metadata` HashMap
    /// for cross-benchmark compatibility.
    ///
    /// Note: Currently unused but will be needed in Phase 5 for orchestrator
    #[allow(dead_code)]
    fn convert_task(swebench_task: &crate::ai::evaluation::Task) -> Task {
        use std::collections::HashMap;

        let mut metadata = HashMap::new();
        metadata.insert("repo".to_string(), serde_json::json!(swebench_task.repo));
        metadata.insert(
            "base_commit".to_string(),
            serde_json::json!(swebench_task.base_commit),
        );
        metadata.insert(
            "test_patch".to_string(),
            serde_json::json!(swebench_task.test_patch),
        );
        if let Some(hints) = &swebench_task.hints {
            metadata.insert("hints".to_string(), serde_json::json!(hints));
        }
        if let Some(solution) = &swebench_task.solution_patch {
            metadata.insert("solution_patch".to_string(), serde_json::json!(solution));
        }
        metadata.insert(
            "complexity".to_string(),
            serde_json::json!(format!("{:?}", swebench_task.complexity)),
        );

        // Convert files_to_modify from PathBuf to Vec<String> for JSON
        let files: Vec<String> = swebench_task
            .files_to_modify
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        metadata.insert("files_to_modify".to_string(), serde_json::json!(files));

        Task {
            id: swebench_task.id.clone(),
            description: swebench_task.problem_statement.clone(),
            expected_output: Some("Tests pass".to_string()),
            metadata,
        }
    }
}

#[async_trait]
impl BenchmarkExecutor for SweBenchExecutor {
    /// Initialize the benchmark executor
    ///
    /// Loads the SWE-bench dataset from cache or downloads from HuggingFace.
    /// After setup(), `get_metadata().total_tasks` reflects the actual dataset size.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Dataset download fails (network issues, invalid URL)
    /// - Dataset file is malformed (invalid Parquet or JSON)
    /// - Cache directory cannot be created
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut executor = SweBenchExecutor::new(DatasetSource::Verified);
    /// executor.setup().await?;
    /// assert_eq!(executor.get_metadata().total_tasks, 500);
    /// ```
    async fn setup(&mut self) -> Result<()> {
        tracing::info!("Setting up SWE-bench executor: {:?}", self.source);

        // Load all tasks from dataset
        // Note: In real evaluation, we'd use stratified sampling or load_sample(count)
        // For now, load all tasks to get accurate total_tasks
        let dataset_path = self
            .dataset_manager
            .get_or_download(&self.source)
            .await
            .context("Failed to load SWE-bench dataset")?;

        tracing::info!("Loading tasks from {:?}", dataset_path);

        // Load tasks using TaskLoader
        use crate::ai::evaluation::TaskLoader;
        let loader = TaskLoader::new(dataset_path);
        self.tasks = loader
            .load_all()
            .context("Failed to load tasks from dataset")?;

        // Update metadata with actual task count (important for Local sources)
        self.metadata.total_tasks = self.tasks.len();

        tracing::info!(
            "SWE-bench setup complete: {} tasks loaded",
            self.tasks.len()
        );

        Ok(())
    }

    /// Execute a single task
    ///
    /// Runs the task using the existing M0 agent infrastructure. This method:
    /// 1. Creates LLM client (Anthropic Claude)
    /// 2. Creates tool registry with configured features
    /// 3. Creates agent with max_steps from ExecutionContext
    /// 4. Executes task and collects metrics
    /// 5. Returns TaskResult with full metrics
    ///
    /// # Parameters
    ///
    /// - `task`: The task to execute (generic Task format)
    /// - `ctx`: Execution configuration (timeout, max_steps, system_config)
    ///
    /// # Returns
    ///
    /// Returns `TaskResult` with:
    /// - `solved`: Whether tests passed
    /// - `duration_ms`: Total execution time
    /// - `cost_usd`: API cost for this task
    /// - `metrics`: Detailed performance metrics
    /// - `error`: Error message if task failed
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let task = executor.tasks[0].clone();
    /// let ctx = ExecutionContext::default();
    /// let result = executor.run_task(&task, &ctx).await?;
    ///
    /// println!("Task {}: solved={}", task.id, result.solved);
    /// ```
    async fn run_task(&self, task: &Task, ctx: &ExecutionContext) -> TaskResult {
        tracing::info!("Running task: {}", task.id);

        // Find the original SWE-bench task
        let swebench_task = self
            .tasks
            .iter()
            .find(|t| t.id == task.id)
            .expect("Task not found in loaded dataset");

        // Parse system_config as ToadConfig (if possible)
        let config = serde_json::from_value::<ToadConfig>(ctx.system_config.clone())
            .unwrap_or_else(|_| ToadConfig::default());

        // Get API key (required for Anthropic client)
        let api_key = match get_api_key() {
            Ok(key) => key,
            Err(e) => {
                let mut result = TaskResult::new(task.id.clone());
                result.mark_failed(format!(
                    "Failed to get API key. Set ANTHROPIC_API_KEY environment variable: {}",
                    e
                ));
                return result;
            }
        };

        // Create LLM client
        let mut llm_client = AnthropicClient::new(api_key)
            .with_model("claude-sonnet-4-20250514");

        // Enable prompt caching if configured
        if config.features.prompt_caching {
            llm_client = llm_client.with_prompt_caching(true);
        }

        // Create tool registry with feature flags
        let tool_registry = if config.features.smart_test_selection {
            ToolRegistry::m2_with_features(&config.features)
        } else {
            ToolRegistry::m1_with_features(&config.features)
        };

        // Create agent with max_steps from ExecutionContext
        let agent = Agent::new(Box::new(llm_client), tool_registry)
            .with_max_steps(ctx.max_steps as u32);

        // Create metrics collector
        let mut metrics_collector = MetricsCollector::new();

        // Execute task
        let start = Instant::now();
        let agent_result = match tokio::time::timeout(
            ctx.timeout,
            agent.execute_task(swebench_task, &mut metrics_collector),
        )
        .await
        {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => {
                // Agent execution failed
                let mut result = TaskResult::new(task.id.clone());
                result.mark_failed(format!("Agent execution failed: {}", e));
                result.duration_ms = start.elapsed().as_millis() as u64;
                return result;
            }
            Err(_) => {
                // Timeout
                let mut result = TaskResult::new(task.id.clone());
                result.mark_failed(format!(
                    "Task exceeded timeout of {}s",
                    ctx.timeout.as_secs()
                ));
                result.duration_ms = start.elapsed().as_millis() as u64;
                return result;
            }
        };

        // Build task result from agent result and metrics
        let final_metrics = metrics_collector.finish();
        let mut result = TaskResult::new(task.id.clone());

        result.duration_ms = final_metrics.duration_ms;
        result.cost_usd = final_metrics.cost_usd;
        result.api_calls = final_metrics.api_calls;
        result.total_tokens = final_metrics.total_tokens();
        result.metrics = final_metrics;

        // Mark as solved if agent completed successfully
        if agent_result.success {
            result.mark_solved();
        } else {
            result.mark_failed(agent_result.final_response);
        }

        tracing::info!(
            "Task {} complete: solved={}, cost=${:.4}, tokens={}",
            task.id,
            result.solved,
            result.cost_usd,
            result.total_tokens
        );

        result
    }

    /// Clean up resources after all tasks complete
    ///
    /// SWE-bench executor doesn't require cleanup (datasets are cached for reuse).
    /// This method is a no-op but required by the trait.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// executor.cleanup().await?;
    /// ```
    async fn cleanup(&mut self) -> Result<()> {
        tracing::info!("SWE-bench cleanup: no-op (datasets remain cached)");
        Ok(())
    }

    /// Get metadata about this benchmark
    ///
    /// Returns information about the SWE-bench dataset variant, including:
    /// - Name (e.g., "SWE-bench Verified")
    /// - Total tasks (500/300/2,294 depending on variant)
    /// - Dataset URL (HuggingFace)
    /// - Contamination risk (LOW - post-training release)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::benchmarks::swebench::SweBenchExecutor;
    /// use toad::ai::evaluation::DatasetSource;
    ///
    /// let executor = SweBenchExecutor::new(DatasetSource::Verified);
    /// let metadata = executor.get_metadata();
    ///
    /// assert_eq!(metadata.name, "SWE-bench Verified");
    /// assert_eq!(metadata.total_tasks, 500);
    /// ```
    fn get_metadata(&self) -> &BenchmarkMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swebench_executor_metadata() {
        // Test Verified
        let verified = SweBenchExecutor::new(DatasetSource::Verified);
        let metadata = verified.get_metadata();
        assert_eq!(metadata.name, "SWE-bench Verified");
        assert_eq!(metadata.total_tasks, 500);
        assert_eq!(metadata.contamination_risk, "LOW");
        assert!(metadata.dataset_url.is_some());

        // Test Lite
        let lite = SweBenchExecutor::new(DatasetSource::Lite);
        let metadata = lite.get_metadata();
        assert_eq!(metadata.name, "SWE-bench Lite");
        assert_eq!(metadata.total_tasks, 300);

        // Test Full
        let full = SweBenchExecutor::new(DatasetSource::Full);
        let metadata = full.get_metadata();
        assert_eq!(metadata.name, "SWE-bench Full");
        assert_eq!(metadata.total_tasks, 2294);
    }

    #[test]
    fn test_task_conversion() {
        use std::path::PathBuf;

        // Create SWE-bench task
        let swebench_task = crate::ai::evaluation::Task {
            id: "django__django-12345".to_string(),
            repo: "django/django".to_string(),
            base_commit: "abc123".to_string(),
            problem_statement: "Add JSONB field support".to_string(),
            hints: Some("Check postgres backend".to_string()),
            test_patch: "def test_jsonb(): ...".to_string(),
            files_to_modify: vec![PathBuf::from("django/db/models/fields/__init__.py")],
            solution_patch: Some("+ class JSONBField: ...".to_string()),
            complexity: crate::ai::evaluation::Complexity::Medium,
            metadata: std::collections::HashMap::new(),
        };

        // Convert to generic task
        let generic_task = SweBenchExecutor::convert_task(&swebench_task);

        // Verify conversion
        assert_eq!(generic_task.id, "django__django-12345");
        assert_eq!(generic_task.description, "Add JSONB field support");
        assert_eq!(generic_task.expected_output, Some("Tests pass".to_string()));

        // Verify metadata
        assert_eq!(
            generic_task.metadata.get("repo"),
            Some(&serde_json::json!("django/django"))
        );
        assert_eq!(
            generic_task.metadata.get("base_commit"),
            Some(&serde_json::json!("abc123"))
        );
        assert_eq!(
            generic_task.metadata.get("hints"),
            Some(&serde_json::json!("Check postgres backend"))
        );
        assert_eq!(
            generic_task.metadata.get("complexity"),
            Some(&serde_json::json!("Medium"))
        );
    }

    #[tokio::test]
    #[ignore] // Requires network access to download dataset
    async fn test_swebench_executor_setup() {
        use tempfile::TempDir;

        // Create executor with temporary cache
        let temp_dir = TempDir::new().unwrap();
        let mut executor = SweBenchExecutor::new(DatasetSource::Verified);
        executor.dataset_manager = DatasetManager::new(temp_dir.path().to_path_buf());

        // Setup should download and load dataset
        let result = executor.setup().await;
        assert!(result.is_ok(), "Setup failed: {:?}", result.err());

        // Verify tasks were loaded
        assert_eq!(executor.tasks.len(), 500);
        assert_eq!(executor.get_metadata().total_tasks, 500);
    }
}
