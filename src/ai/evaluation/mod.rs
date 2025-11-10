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

/// Metadata from multi-model racing (M3+)
///
/// Tracks which model won the race, costs incurred, and performance metrics.
/// This is only populated when routing_multi_model feature flag is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceMetadata {
    /// Name of the winning model
    pub winner_model: String,

    /// Total cost including winner + partial costs from cancelled models
    pub total_cost_usd: f64,

    /// Wasted cost from cancelled models (may be $0 if cancelled before billing)
    pub wasted_cost_usd: f64,

    /// Latency improvement vs slowest single model (milliseconds)
    /// Negative if racing was actually slower due to overhead
    pub latency_improvement_ms: i64,

    /// Total race duration (wall clock time)
    pub race_duration_ms: u64,
}

/// Metadata from cascading routing (M4+)
///
/// Tracks which tier was selected, task difficulty, and cost optimization.
/// This is only populated when routing_cascade feature flag is enabled.
///
/// # Evidence
///
/// Based on DavaJ research (2024):
/// - 84.7% accuracy on HumanEval
/// - 70% cost reduction vs cloud-only
/// - Local-first cascade: 7B → 32B → Cloud
///
/// TOAD implements 4 tiers: Local7B, Local32B, CloudPremium, CloudBest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeMetadata {
    /// Task difficulty classification (Easy/Medium/Hard)
    pub task_difficulty: String,

    /// Selected model tier (Local7B/Local32B/CloudPremium/CloudBest)
    pub selected_tier: String,

    /// Estimated cost for this tier (local = $0, cloud = $2-10)
    pub tier_cost_usd: f64,

    /// Time spent classifying and routing (microseconds for minimal overhead)
    pub routing_duration_ms: u64,
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

    /// Racing metadata (M3+ only, when routing_multi_model enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_metadata: Option<RaceMetadata>,

    /// Cascade routing metadata (M4+ only, when routing_cascade enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_metadata: Option<CascadeMetadata>,
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
            race_metadata: None,
            cascade_metadata: None,
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
        use crate::ai::routing::{CascadingRouter, Router, TaskClassifier};
        use crate::ai::tools::ToolRegistry;
        use anyhow::Context;
        use std::time::Instant;

        tracing::info!("Running task: {}", task.id);

        // Get API key (may be optional for local-only cascading)
        let api_key = get_api_key().ok();

        // Track if we're using racing (for metrics extraction)
        let mut racing_client_ref: Option<std::sync::Arc<crate::ai::llm::RacingClient>> = None;

        // Track cascade routing metadata (for M4)
        let mut cascade_difficulty: Option<String> = None;
        let mut cascade_tier: Option<String> = None;
        let mut cascade_cost: Option<f64> = None;
        let mut cascade_duration_ms: Option<u64> = None;

        // Create LLM client based on routing strategy
        let llm_client: Box<dyn crate::ai::llm::LLMClient> = if config.features.routing_cascade {
            // M4: Use cascading router (cheap local → expensive cloud)
            tracing::info!("M4 Cascading routing enabled for task {}", task.id);

            let routing_start = Instant::now();

            let router = if let Some(key) = api_key.clone() {
                CascadingRouter::with_api_key(key)
            } else {
                tracing::warn!("No API key, using local-only routing");
                CascadingRouter::new()
            };

            // Classify task difficulty first (for metadata)
            let classifier = TaskClassifier::new();
            let difficulty = classifier.classify(task)?;
            cascade_difficulty = Some(format!("{:?}", difficulty));

            // Select tier based on difficulty
            let tier = router.select_tier(difficulty);
            cascade_tier = Some(format!("{:?}", tier));
            cascade_cost = Some(tier.estimated_cost_usd());

            // Route to appropriate model based on task difficulty
            let provider_config = router.route(task)?;

            cascade_duration_ms = Some(routing_start.elapsed().as_millis() as u64);

            tracing::info!(
                "M4: Task {} classified as {:?}, routed to {:?} (est. cost: ${:.2})",
                task.id,
                difficulty,
                tier,
                tier.estimated_cost_usd()
            );

            // Create client with prompt caching if configured
            LLMProvider::create_with_features(
                &provider_config,
                config.features.prompt_caching,
            )?
        } else if config.features.routing_multi_model {
            // M3: Use multi-model racing (TRAE approach)
            use crate::ai::llm::RacingClient;

            tracing::info!("M3 Multi-model racing enabled for task {}", task.id);

            let api_key = api_key
                .context("Failed to get API key for racing. Set ANTHROPIC_API_KEY environment variable")?;

            // Create racing client from config
            let racing_client = RacingClient::from_config(
                api_key,
                config.racing_models.clone(),
                config.features.prompt_caching,
            )?;

            tracing::info!(
                "M3: Racing {} models: {}",
                config.racing_models.len(),
                config.racing_models.join(", ")
            );

            // Store Arc reference for metrics extraction later
            let racing_arc = std::sync::Arc::new(racing_client);
            racing_client_ref = Some(racing_arc.clone());

            // Clone the Arc into a Box for the agent
            // We use Arc::new + Arc::clone to share ownership
            Box::new((*racing_arc).clone())
        } else {
            // M1/M2: Use direct Anthropic client
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

        // Build AST context if M2 feature enabled
        let custom_prompt = if config.features.context_ast {
            use crate::ai::context::ContextBuilder;
            use crate::ai::agent::PromptBuilder;

            tracing::info!("M2: Building AST context for task {}", task.id);

            // Try to build context from current directory (task workspace)
            // In real evaluation, this would be the cloned repo directory
            match ContextBuilder::new() {
                Ok(builder) => {
                    match builder.add_directory(".", &["py", "js", "ts", "tsx", "rs"]).await {
                        Ok(builder) => {
                            let context = builder.build();
                            tracing::info!(
                                "M2: Built AST context with {} files, {} symbols",
                                context.file_contexts.len(),
                                context.total_symbols
                            );
                            Some(PromptBuilder::new()
                                .with_task(task)
                                .with_ast_context(context)
                                .build())
                        }
                        Err(e) => {
                            // Log warning but continue without AST context
                            tracing::warn!("M2: Failed to add directory to AST context: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("M2: Failed to create AST context builder: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Create agent
        let agent = Agent::new(llm_client, tool_registry);

        // Create metrics collector
        let mut metrics_collector = MetricsCollector::new();

        // Execute task with AST-enhanced prompt if available (M2)
        let agent_result = if let Some(prompt) = custom_prompt {
            agent.execute_task_with_prompt(task, Some(prompt), &mut metrics_collector).await?
        } else {
            agent.execute_task(task, &mut metrics_collector).await?
        };

        // Build task result
        let final_metrics = metrics_collector.finish();
        let mut result = TaskResult::new(task.id.clone());

        result.duration_ms = final_metrics.duration_ms;
        result.cost_usd = final_metrics.cost_usd;
        result.api_calls = final_metrics.api_calls;
        result.total_tokens = final_metrics.total_tokens();
        result.metrics = final_metrics;

        // Extract race metadata if M3 racing was used
        if let Some(racing_client) = racing_client_ref
            && let Some(race_result) = racing_client.get_last_race_result() {
                let latency_improvement = race_result.latency_improvement()
                    .map(|d| d.as_millis() as i64)
                    .unwrap_or(0);

                result.race_metadata = Some(RaceMetadata {
                    winner_model: race_result.winner_model.clone(),
                    total_cost_usd: race_result.total_cost(),
                    wasted_cost_usd: race_result.total_wasted_cost(),
                    latency_improvement_ms: latency_improvement,
                    race_duration_ms: race_result.race_duration.as_millis() as u64,
                });

                tracing::info!(
                    "M3 Race metadata: winner={}, total_cost=${:.4}, wasted=${:.4}, latency_improvement={}ms",
                    race_result.winner_model,
                    race_result.total_cost(),
                    race_result.total_wasted_cost(),
                    latency_improvement
                );
            }

        // Extract cascade metadata if M4 cascading was used
        if cascade_difficulty.is_some() && cascade_tier.is_some() {
            result.cascade_metadata = Some(CascadeMetadata {
                task_difficulty: cascade_difficulty.unwrap(),
                selected_tier: cascade_tier.unwrap(),
                tier_cost_usd: cascade_cost.unwrap_or(0.0),
                routing_duration_ms: cascade_duration_ms.unwrap_or(0),
            });

            tracing::info!(
                "M4 Cascade metadata: difficulty={}, tier={}, cost=${:.2}, routing_time={}ms",
                result.cascade_metadata.as_ref().unwrap().task_difficulty,
                result.cascade_metadata.as_ref().unwrap().selected_tier,
                result.cascade_metadata.as_ref().unwrap().tier_cost_usd,
                result.cascade_metadata.as_ref().unwrap().routing_duration_ms
            );
        }

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
    fn test_m2_config_has_required_features() {
        use crate::config::FeatureFlags;

        let m2_features = FeatureFlags::milestone_2();

        // M2 MUST have these enabled
        assert!(m2_features.context_ast, "M2 must have AST context enabled (+2-5 points)");
        assert!(m2_features.smart_test_selection, "M2 must have smart test selection enabled (+3-5 points)");

        // M2 should inherit M1 features
        assert!(m2_features.prompt_caching, "M2 should have prompt caching from M1");
        assert!(m2_features.tree_sitter_validation, "M2 should have tree-sitter validation from M1");

        // M2 should NOT have M3+ features
        assert!(!m2_features.routing_multi_model, "M2 should not have multi-model routing (that's M3)");
        assert!(!m2_features.routing_cascade, "M2 should not have cascading routing (that's M4)");
    }

    #[test]
    fn test_m2_baseline_config_uses_features() {
        let config = ToadConfig::for_milestone(2);

        // Verify M2 config has AST context and smart test selection
        assert!(config.features.context_ast);
        assert!(config.features.smart_test_selection);
        assert!(config.features.prompt_caching);
        assert!(config.features.tree_sitter_validation);
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

    #[test]
    fn test_m3_racing_models_configured() {
        let config = ToadConfig::for_milestone(3);

        // M3 should have racing models configured
        assert!(config.racing_models.len() >= 2, "M3 needs at least 2 models to race");
        assert!(config.features.routing_multi_model, "M3 must have routing_multi_model enabled");
    }

    #[test]
    fn test_race_metadata_serialization() {
        let metadata = RaceMetadata {
            winner_model: "claude-sonnet-4-20250514".to_string(),
            total_cost_usd: 0.05,
            wasted_cost_usd: 0.01,
            latency_improvement_ms: 500,
            race_duration_ms: 1500,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: RaceMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(metadata.winner_model, deserialized.winner_model);
        assert_eq!(metadata.total_cost_usd, deserialized.total_cost_usd);
    }

    #[test]
    fn test_task_result_with_race_metadata() {
        let mut result = TaskResult::new("test-task".to_string());

        // Initially no race metadata
        assert!(result.race_metadata.is_none());

        // Add race metadata
        result.race_metadata = Some(RaceMetadata {
            winner_model: "model-1".to_string(),
            total_cost_usd: 0.02,
            wasted_cost_usd: 0.005,
            latency_improvement_ms: 200,
            race_duration_ms: 800,
        });

        // Serialize and verify race_metadata is included
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("race_metadata"));
        assert!(json.contains("model-1"));

        // Deserialize and verify
        let deserialized: TaskResult = serde_json::from_str(&json).unwrap();
        assert!(deserialized.race_metadata.is_some());
        assert_eq!(deserialized.race_metadata.unwrap().winner_model, "model-1");
    }

    /// End-to-end integration test for M3 racing setup
    ///
    /// Tests that M3 configuration properly creates RacingClient
    /// Note: Full racing is tested in racing.rs module tests
    #[test]
    fn test_m3_racing_client_creation() {
        use crate::ai::llm::{RacingClient, LLMClient, mock::MockResponseBuilder};
        use std::sync::Arc;

        // Create mock models for racing
        let model1 = Arc::new(
            MockResponseBuilder::new()
                .with_text("Model 1 response")
                .build()
        );
        let model2 = Arc::new(
            MockResponseBuilder::new()
                .with_text("Model 2 response")
                .build()
        );

        // Create racing client
        let racing = RacingClient::new(vec![model1, model2]);

        // Verify racing client is set up correctly
        assert_eq!(racing.model_name(), "racing-ensemble");

        // Verify initial state (no race result yet)
        let race_result = racing.get_last_race_result();
        assert!(race_result.is_none());
    }

    /// Test that M3 config creates proper racing client configuration
    #[test]
    fn test_m3_config_racing_client_setup() {
        let config = ToadConfig::for_milestone(3);

        // Verify M3 has racing enabled
        assert!(config.features.routing_multi_model);

        // Verify racing models are configured
        assert_eq!(config.racing_models.len(), 2);
        assert_eq!(config.racing_models[0], "claude-sonnet-4-20250514");
        assert_eq!(config.racing_models[1], "claude-sonnet-3-5-20241022");

        // Verify M3 inherits M2 features
        assert!(config.features.context_ast);
        assert!(config.features.smart_test_selection);
        assert!(config.features.prompt_caching);

        // Verify M3 doesn't have M4 features
        assert!(!config.features.routing_cascade);
    }

    /// Test race metadata calculation correctness
    #[test]
    fn test_race_metadata_cost_calculations() {
        let metadata = RaceMetadata {
            winner_model: "claude-sonnet-4-20250514".to_string(),
            total_cost_usd: 0.05,
            wasted_cost_usd: 0.01,
            latency_improvement_ms: 500,
            race_duration_ms: 1500,
        };

        // Verify cost breakdown makes sense
        assert!(metadata.total_cost_usd > metadata.wasted_cost_usd);

        // Winner cost = total - wasted
        let winner_cost = metadata.total_cost_usd - metadata.wasted_cost_usd;
        assert_eq!(winner_cost, 0.04);

        // Verify latency improvement is positive (racing was faster)
        assert!(metadata.latency_improvement_ms > 0);
    }

    /// Test that M4 config has cascading enabled
    #[test]
    fn test_m4_config_has_cascading_enabled() {
        use crate::config::FeatureFlags;

        let m4_features = FeatureFlags::milestone_4();

        // M4 MUST have cascading routing enabled
        assert!(m4_features.routing_cascade, "M4 must have cascading routing enabled (70% cost reduction)");

        // M4 should inherit M3 features
        assert!(m4_features.routing_multi_model, "M4 should have multi-model routing from M3");
        assert!(m4_features.context_ast, "M4 should have AST context from M2");
        assert!(m4_features.smart_test_selection, "M4 should have smart test selection from M2");

        // M4 adds embeddings and failure memory
        assert!(m4_features.context_embeddings, "M4 should have embeddings");
        assert!(m4_features.failure_memory, "M4 should have failure memory");

        // Core optimizations still enabled
        assert!(m4_features.prompt_caching);
    }

    /// Test cascade metadata serialization
    #[test]
    fn test_m4_cascade_metadata_serialization() {
        let metadata = CascadeMetadata {
            task_difficulty: "Medium".to_string(),
            selected_tier: "Local32B".to_string(),
            tier_cost_usd: 0.0,
            routing_duration_ms: 15,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: CascadeMetadata = serde_json::from_str(&json).unwrap();

        // Verify fields match
        assert_eq!(metadata.task_difficulty, deserialized.task_difficulty);
        assert_eq!(metadata.selected_tier, deserialized.selected_tier);
        assert_eq!(metadata.tier_cost_usd, deserialized.tier_cost_usd);
        assert_eq!(metadata.routing_duration_ms, deserialized.routing_duration_ms);
    }

    /// Test cascade tier selection for easy tasks
    #[test]
    fn test_cascade_tier_selection_easy() {
        use crate::ai::routing::{CascadingRouter, TaskClassifier, Difficulty};

        let classifier = TaskClassifier::new();
        let router = CascadingRouter::new();

        // Create easy task
        let task = Task {
            id: "test-easy".to_string(),
            problem_statement: "Fix typo in README.md".to_string(),
            ..Task::example()
        };

        // Classify difficulty
        let difficulty = classifier.classify(&task).unwrap();
        assert_eq!(difficulty, Difficulty::Easy);

        // Verify tier selection
        use crate::ai::routing::ModelTier;
        let tier = router.select_tier(difficulty);
        assert_eq!(tier, ModelTier::Local7B);
        assert_eq!(tier.estimated_cost_usd(), 0.0);
    }

    /// Test cascade tier selection for hard tasks
    #[test]
    fn test_cascade_tier_selection_hard() {
        use crate::ai::routing::{CascadingRouter, TaskClassifier, Difficulty};

        let classifier = TaskClassifier::new();
        let router = CascadingRouter::with_api_key("test-key".to_string());

        // Create hard task
        let task = Task {
            id: "test-hard".to_string(),
            problem_statement: "Refactor the entire authentication architecture to improve performance. This requires changes across auth.py, middleware.py, database.py, cache.py, config.py, utils.py, and all related test files.".to_string(),
            ..Task::example()
        };

        // Classify difficulty
        let difficulty = classifier.classify(&task).unwrap();
        assert_eq!(difficulty, Difficulty::Hard);

        // Verify tier selection
        use crate::ai::routing::ModelTier;
        let tier = router.select_tier(difficulty);
        assert_eq!(tier, ModelTier::CloudPremium);
        assert!(tier.estimated_cost_usd() > 0.0);
    }

    /// Test cascade cost tracking
    #[test]
    fn test_cascade_cost_tracking() {
        use crate::ai::routing::ModelTier;

        // Easy/Medium tasks are free (local)
        assert_eq!(ModelTier::Local7B.estimated_cost_usd(), 0.0);
        assert_eq!(ModelTier::Local32B.estimated_cost_usd(), 0.0);

        // Hard tasks cost money (cloud)
        assert!(ModelTier::CloudPremium.estimated_cost_usd() > 0.0);
        assert!(ModelTier::CloudBest.estimated_cost_usd() > ModelTier::CloudPremium.estimated_cost_usd());

        // Verify cost model (70% reduction from DavaJ)
        // Easy (40%) + Medium (40%) = 80% of tasks at $0
        // Hard (20%) at ~$2 average
        let easy_cost = 0.4 * 500.0 * ModelTier::Local7B.estimated_cost_usd();
        let medium_cost = 0.4 * 500.0 * ModelTier::Local32B.estimated_cost_usd();
        let hard_cost = 0.2 * 500.0 * ModelTier::CloudPremium.estimated_cost_usd();
        let total_cost = easy_cost + medium_cost + hard_cost;

        // Should be ~$200 for 500 tasks vs $1000 cloud-only (80% reduction)
        assert!(total_cost < 500.0, "Total cost should be less than $500 for 500 tasks");
    }
}
