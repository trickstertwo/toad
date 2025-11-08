/// Evaluation framework for SWE-bench tasks
///
/// This module provides the core infrastructure for running and evaluating
/// AI coding agent performance on SWE-bench tasks.

use crate::config::ToadConfig;
use crate::metrics::Metrics;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

pub mod task_loader;
pub use task_loader::TaskLoader;

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
        println!("Accuracy: {:.2}% ({}/{})", self.accuracy, self.tasks_solved, self.total_tasks);
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

    /// Run a single task (stub for now)
    async fn run_task(&self, task: &Task, _config: &ToadConfig) -> Result<TaskResult> {
        // TODO: Implement actual task execution
        // For now, return a placeholder result
        let mut result = TaskResult::new(task.id.clone());
        result.duration_ms = 1000;
        result.cost_usd = 0.01;
        result.api_calls = 1;
        result.total_tokens = 1000;

        // Simulate 50% success rate for testing
        if task.id.ends_with('1') || task.id.ends_with('3') || task.id.ends_with('5') {
            result.mark_solved();
        }

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
    async fn test_harness_evaluate() {
        let tasks = vec![Task::example()];
        let harness = EvaluationHarness::new(tasks, PathBuf::from("/tmp/toad-test"));

        let config = ToadConfig::default();
        let results = harness.evaluate(&config).await.unwrap();

        assert_eq!(results.total_tasks, 1);
    }
}
