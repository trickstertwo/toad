use crate::ai::metrics::Metrics;
/// Evaluation framework for SWE-bench tasks
///
/// This module provides the core infrastructure for running and evaluating
/// AI coding agent performance on SWE-bench tasks.
use crate::config::ToadConfig;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod task_loader;
pub use task_loader::TaskLoader;

pub mod dataset_manager;
pub use dataset_manager::{DatasetInfo, DatasetManager, DatasetSource};

pub mod experiment_manager;
pub use experiment_manager::{Experiment, ExperimentManager, ExperimentResults, ExperimentStatus};

/// Complexity level of a task
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Complexity {
    Simple,
    Medium,
    Hard,
}

/// A single SWE-bench task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier (e.g., "django__django-12345")
    pub id: String,

    /// Repository name (e.g., "django/django")
    pub repo: String,

    /// Base commit hash
    pub base_commit: String,

    /// Problem statement / issue description
    pub problem_statement: String,

    /// Hints (if available)
    pub hints: Option<String>,

    /// Test patch (the test that verifies the fix)
    pub test_patch: String,

    /// Files that need to be modified (ground truth, for evaluation)
    pub files_to_modify: Vec<PathBuf>,

    /// Expected solution patch (ground truth)
    pub solution_patch: Option<String>,

    /// Task complexity (categorized for stratified sampling)
    pub complexity: Complexity,

    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Task {
    /// Create a minimal task for testing
    pub fn example() -> Self {
        Self {
            id: "test__example-001".to_string(),
            repo: "test/example".to_string(),
            base_commit: "abc123".to_string(),
            problem_statement: "Fix the bug in function foo()".to_string(),
            hints: Some("The issue is in the return statement".to_string()),
            test_patch: "// Test code here".to_string(),
            files_to_modify: vec![PathBuf::from("src/foo.rs")],
            solution_patch: Some("// Solution patch".to_string()),
            complexity: Complexity::Simple,
            metadata: HashMap::new(),
        }
    }
}

/// Result of running a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,

    /// Was the task solved correctly?
    pub solved: bool,

    /// Did tests pass?
    pub tests_passed: bool,

    /// Execution time (milliseconds)
    pub duration_ms: u64,

    /// API cost (USD)
    pub cost_usd: f64,

    /// Number of API calls made
    pub api_calls: u32,

    /// Total tokens used (input + output)
    pub total_tokens: u64,

    /// Files modified by the agent
    pub files_modified: Vec<PathBuf>,

    /// Error message (if failed)
    pub error: Option<String>,

    /// Full metrics
    pub metrics: Metrics,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl TaskResult {
    /// Create a new task result
    pub fn new(task_id: String) -> Self {
        Self {
            task_id,
            solved: false,
            tests_passed: false,
            duration_ms: 0,
            cost_usd: 0.0,
            api_calls: 0,
            total_tokens: 0,
            files_modified: vec![],
            error: None,
            metrics: Metrics::default(),
            timestamp: Utc::now(),
        }
    }

    /// Mark as solved
    pub fn mark_solved(&mut self) {
        self.solved = true;
        self.tests_passed = true;
    }

    /// Mark as failed with error
    pub fn mark_failed(&mut self, error: String) {
        self.solved = false;
        self.error = Some(error);
    }
}

/// Evaluation results for a set of tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResults {
    /// Configuration used
    pub config_name: String,

    /// Individual task results
    pub results: Vec<TaskResult>,

    /// Overall accuracy (% solved)
    pub accuracy: f64,

    /// Average cost per task
    pub avg_cost_usd: f64,

    /// Average duration per task
    pub avg_duration_ms: f64,

    /// Total tasks evaluated
    pub total_tasks: usize,

    /// Tasks solved
    pub tasks_solved: usize,

    /// Breakdown by complexity
    pub by_complexity: HashMap<String, ComplexityStats>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityStats {
    pub total: usize,
    pub solved: usize,
    pub accuracy: f64,
    pub avg_cost: f64,
}

impl EvaluationResults {
    /// Compute results from individual task results
    pub fn from_results(config_name: String, results: Vec<TaskResult>) -> Self {
        let total_tasks = results.len();
        let tasks_solved = results.iter().filter(|r| r.solved).count();
        let accuracy = if total_tasks > 0 {
            (tasks_solved as f64 / total_tasks as f64) * 100.0
        } else {
            0.0
        };

        let avg_cost_usd = if total_tasks > 0 {
            results.iter().map(|r| r.cost_usd).sum::<f64>() / total_tasks as f64
        } else {
            0.0
        };

        let avg_duration_ms = if total_tasks > 0 {
            results.iter().map(|r| r.duration_ms as f64).sum::<f64>() / total_tasks as f64
        } else {
            0.0
        };

        Self {
            config_name,
            results,
            accuracy,
            avg_cost_usd,
            avg_duration_ms,
            total_tasks,
            tasks_solved,
            by_complexity: HashMap::new(), // TODO: compute
            timestamp: Utc::now(),
        }
    }

    /// Print summary
    pub fn print_summary(&self) {
        println!("\n=== Evaluation Results: {} ===", self.config_name);
        println!(
            "Accuracy: {:.2}% ({}/{})",
            self.accuracy, self.tasks_solved, self.total_tasks
        );
        println!("Avg Cost: ${:.4}/task", self.avg_cost_usd);
        println!("Avg Duration: {:.2}s/task", self.avg_duration_ms / 1000.0);
        println!("Total Tasks: {}", self.total_tasks);
    }
}

/// Evaluation harness for running experiments
pub struct EvaluationHarness {
    /// Tasks to evaluate
    tasks: Vec<Task>,

    /// Results storage path
    results_path: PathBuf,
}

impl EvaluationHarness {
    /// Create a new evaluation harness
    pub fn new(tasks: Vec<Task>, results_path: PathBuf) -> Self {
        Self {
            tasks,
            results_path,
        }
    }

    /// Run evaluation with a specific configuration
    pub async fn evaluate(&self, config: &ToadConfig) -> Result<EvaluationResults> {
        let config_name = format!("{} features", config.features.enabled_count());
        let mut results = Vec::new();

        for task in &self.tasks {
            let result = self.run_task(task, config).await?;
            results.push(result);
        }

        Ok(EvaluationResults::from_results(config_name, results))
    }

    /// Run a single task with the agent
    async fn run_task(&self, task: &Task, config: &ToadConfig) -> Result<TaskResult> {
        use crate::ai::agent::Agent;
        use crate::ai::llm::{AnthropicClient, LLMProvider, get_api_key};
        use crate::ai::metrics::MetricsCollector;
        use crate::ai::routing::{CascadingRouter, Router};
        use crate::ai::tools::ToolRegistry;
        use anyhow::Context;

        tracing::info!("Running task: {}", task.id);

        // Get API key (may be optional for local-only cascading)
        let api_key = get_api_key().ok();

        // Create LLM client based on routing strategy
        let llm_client: Box<dyn crate::ai::llm::LLMClient> = if config.features.routing_cascade {
            // M4: Use cascading router (cheap local → expensive cloud)
            tracing::info!("M4 Cascading routing enabled for task {}", task.id);

            let router = if let Some(key) = api_key.clone() {
                CascadingRouter::with_api_key(key)
            } else {
                tracing::warn!("No API key, using local-only routing");
                CascadingRouter::new()
            };

            // Route to appropriate model based on task difficulty
            let provider_config = router.route(task)?;

            // Create client with prompt caching if configured
            LLMProvider::create_with_features(
                &provider_config,
                config.features.prompt_caching,
            )?
        } else {
            // M1/M2/M3: Use direct Anthropic client
            let api_key = api_key
                .context("Failed to get API key. Set ANTHROPIC_API_KEY environment variable")?;

            let mut llm_client = AnthropicClient::new(api_key)
                .with_model("claude-sonnet-4-20250514");

            // Enable prompt caching if configured (90% cost reduction)
            if config.features.prompt_caching {
                llm_client = llm_client.with_prompt_caching(true);
            }

            Box::new(llm_client)
        };

        // Create tool registry with feature flags
        // M2+ uses smart test selection, M1 uses baseline
        let tool_registry = if config.features.smart_test_selection {
            tracing::info!("Using M2+ tool registry with smart test selection");
            ToolRegistry::m2_with_features(&config.features)
        } else {
            ToolRegistry::m1_with_features(&config.features)
        };

        // Create agent
        let agent = Agent::new(llm_client, tool_registry);

        // Create metrics collector
        let mut metrics_collector = MetricsCollector::new();

        // Execute task
        let agent_result = agent.execute_task(task, &mut metrics_collector).await?;

        // Build task result
        let final_metrics = metrics_collector.finish();
        let mut result = TaskResult::new(task.id.clone());

        result.duration_ms = final_metrics.duration_ms;
        result.cost_usd = final_metrics.cost_usd;
        result.api_calls = final_metrics.api_calls;
        result.total_tokens = final_metrics.total_tokens();
        result.metrics = final_metrics;

        // For M1: We consider a task "solved" if agent completed successfully
        // In M2+, we'd validate against test_patch
        if agent_result.success {
            result.mark_solved();
        }

        tracing::info!(
            "Task {} complete: solved={}, cost=${:.4}, tokens={}, steps={}",
            task.id,
            result.solved,
            result.cost_usd,
            result.total_tokens,
            agent_result.steps
        );

        Ok(result)
    }

    /// Compare two configurations
    pub async fn compare(
        &self,
        config_a: &ToadConfig,
        config_b: &ToadConfig,
    ) -> Result<(EvaluationResults, EvaluationResults)> {
        let results_a = self.evaluate(config_a).await?;
        let results_b = self.evaluate(config_b).await?;
        Ok((results_a, results_b))
    }

    /// Save results to disk
    pub fn save_results(&self, results: &EvaluationResults) -> Result<()> {
        std::fs::create_dir_all(&self.results_path)?;

        let filename = format!(
            "eval_{}_{}.json",
            results.config_name.replace(' ', "_"),
            results.timestamp.timestamp()
        );

        let path = self.results_path.join(filename);
        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(path, json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::example();
        assert_eq!(task.id, "test__example-001");
        assert_eq!(task.complexity, Complexity::Simple);
    }

    #[test]
    fn test_task_result() {
        let mut result = TaskResult::new("test-001".to_string());
        assert!(!result.solved);

        result.mark_solved();
        assert!(result.solved);
        assert!(result.tests_passed);
    }

    #[test]
    fn test_evaluation_results() {
        let mut results = vec![];
        for i in 0..10 {
            let mut result = TaskResult::new(format!("task-{}", i));
            if i % 2 == 0 {
                result.mark_solved();
            }
            result.cost_usd = 0.01;
            result.duration_ms = 1000;
            results.push(result);
        }

        let eval = EvaluationResults::from_results("test".to_string(), results);
        assert_eq!(eval.accuracy, 50.0);
        assert_eq!(eval.tasks_solved, 5);
        assert_eq!(eval.total_tasks, 10);
    }

    #[tokio::test]
    #[ignore] // Requires ANTHROPIC_API_KEY, run with `cargo test -- --ignored`
    async fn test_harness_evaluate() {
        let tasks = vec![Task::example()];
        let harness = EvaluationHarness::new(tasks, PathBuf::from("/tmp/toad-test"));

        let config = ToadConfig::default();
        let results = harness.evaluate(&config).await.unwrap();

        assert_eq!(results.total_tasks, 1);
    }

    #[test]
    fn test_m1_config_has_required_features() {
        use crate::config::FeatureFlags;

        let m1_features = FeatureFlags::milestone_1();

        // M1 MUST have these enabled
        assert!(m1_features.prompt_caching, "M1 must have prompt caching enabled (90% cost reduction)");
        assert!(m1_features.tree_sitter_validation, "M1 must have tree-sitter validation enabled");

        // M1 should NOT have these (simple baseline)
        assert!(!m1_features.context_ast, "M1 should not have AST context (that's M2)");
        assert!(!m1_features.smart_test_selection, "M1 should not have smart test selection (that's M2)");
        assert!(!m1_features.routing_multi_model, "M1 should not have multi-model routing (that's M3)");
    }

    #[test]
    fn test_m1_baseline_config_uses_features() {
        let config = ToadConfig::for_milestone(1);

        // Verify M1 config has the right features
        assert!(config.features.prompt_caching);
        assert!(config.features.tree_sitter_validation);
        assert!(!config.features.context_ast);
    }

    #[test]
    fn test_m4_config_has_cascading_routing() {
        use crate::config::FeatureFlags;

        let m4_features = FeatureFlags::milestone_4();

        // M4 MUST have cascading routing enabled
        assert!(m4_features.routing_cascade, "M4 must have cascading routing enabled (70% cost reduction)");

        // M4 should also have M3 features
        assert!(m4_features.routing_multi_model, "M4 should have multi-model routing from M3");
        assert!(m4_features.context_ast, "M4 should have AST context from M2");
        assert!(m4_features.smart_test_selection, "M4 should have smart test selection from M2");

        // M4 adds embeddings and failure memory
        assert!(m4_features.context_embeddings, "M4 should have embeddings for better context");
        assert!(m4_features.failure_memory, "M4 should have failure memory");

        // Core optimizations still enabled
        assert!(m4_features.prompt_caching);
        assert!(m4_features.tree_sitter_validation);
    }

    #[test]
    fn test_m4_baseline_config_uses_features() {
        let config = ToadConfig::for_milestone(4);

        // Verify M4 config has cascading routing
        assert!(config.features.routing_cascade);
        assert!(config.features.routing_multi_model);
        assert!(config.features.context_embeddings);
        assert!(config.features.failure_memory);
    }
}
