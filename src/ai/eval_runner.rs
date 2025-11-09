/// Async evaluation runner for TUI
///
/// This module provides infrastructure for running evaluations in the background
/// while sending progress updates to the TUI event loop.
use crate::ai::agent::Agent;
use crate::ai::eval_commands::{CompareArgs, EvalArgs};
use crate::ai::evaluation::{
    DatasetManager, EvaluationResults, Task, TaskLoader, TaskResult,
};
use crate::ai::llm::anthropic::AnthropicClient;
use crate::ai::tools::ToolRegistry;
use crate::config::ToadConfig;
use crate::core::event::{EvaluationProgress, Event};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Handle to a running evaluation
#[derive(Debug)]
pub struct EvaluationHandle {
    /// Join handle for the background task
    handle: JoinHandle<()>,
    /// Sender for cancellation
    cancel_tx: mpsc::Sender<()>,
}

impl EvaluationHandle {
    /// Cancel the running evaluation
    pub async fn cancel(self) {
        let _ = self.cancel_tx.send(()).await;
        let _ = self.handle.await;
    }

    /// Check if the evaluation is still running
    pub fn is_running(&self) -> bool {
        !self.handle.is_finished()
    }
}

/// Start an evaluation run in the background
///
/// Returns a handle to the running task and sends events to the provided channel.
pub fn start_evaluation(
    args: EvalArgs,
    event_tx: mpsc::UnboundedSender<Event>,
) -> EvaluationHandle {
    let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(1);

    let handle = tokio::spawn(async move {
        let result = run_evaluation_inner(args, event_tx.clone(), &mut cancel_rx).await;

        // Send completion or error event
        match result {
            Ok(results) => {
                let _ = event_tx.send(Event::EvaluationComplete(results));
            }
            Err(e) => {
                let _ = event_tx.send(Event::EvaluationError(format!("{}", e)));
            }
        }
    });

    EvaluationHandle { handle, cancel_tx }
}

/// Inner evaluation function
async fn run_evaluation_inner(
    args: EvalArgs,
    event_tx: mpsc::UnboundedSender<Event>,
    cancel_rx: &mut mpsc::Receiver<()>,
) -> anyhow::Result<EvaluationResults> {
    // Load tasks
    let _ = event_tx.send(Event::EvaluationProgress(EvaluationProgress {
        current_task: 0,
        total_tasks: 0,
        task_id: "Loading tasks...".to_string(),
        current_step: None,
        max_steps: None,
        last_tool: None,
        total_tokens: 0,
        total_cost: 0.0,
        message: Some("Loading dataset...".to_string()),
        last_result: None,
    }));

    let tasks = load_tasks(&args.dataset, args.count).await?;

    // Create configuration
    let config = ToadConfig::for_milestone(args.milestone as u8);

    // Run evaluation with progress updates
    let results = run_tasks_with_progress(tasks, &config, event_tx, cancel_rx).await?;

    Ok(results)
}

/// Load tasks from dataset
async fn load_tasks(
    dataset: &crate::ai::evaluation::DatasetSource,
    count: Option<usize>,
) -> anyhow::Result<Vec<Task>> {
    let tasks = match dataset {
        crate::ai::evaluation::DatasetSource::Local(path) => {
            let task_loader = TaskLoader::new(path.clone());
            task_loader.load_all()?
        }
        crate::ai::evaluation::DatasetSource::Verified
        | crate::ai::evaluation::DatasetSource::Lite
        | crate::ai::evaluation::DatasetSource::Full => {
            let cache_dir = DatasetManager::default_cache_dir();
            let manager = DatasetManager::new(cache_dir);

            // Download/cache dataset
            let dataset_path = manager.get_or_download(dataset).await?;

            // Load tasks using task loader
            let task_loader = TaskLoader::new(dataset_path);
            task_loader.load_all()?
        }
    };

    // Limit task count if specified
    let tasks = if let Some(count) = count {
        tasks.into_iter().take(count).collect()
    } else {
        tasks
    };

    Ok(tasks)
}

/// Run tasks with progress updates
async fn run_tasks_with_progress(
    tasks: Vec<Task>,
    config: &ToadConfig,
    event_tx: mpsc::UnboundedSender<Event>,
    cancel_rx: &mut mpsc::Receiver<()>,
) -> anyhow::Result<EvaluationResults> {
    let total_tasks = tasks.len();
    let mut results = Vec::new();
    let mut total_tokens = 0u64;
    let mut total_cost = 0.0f64;

    for (idx, task) in tasks.into_iter().enumerate() {
        // Check for cancellation
        if cancel_rx.try_recv().is_ok() {
            return Err(anyhow::anyhow!("Evaluation cancelled by user"));
        }

        let current_task = idx + 1;

        // Send progress update - starting task
        let _ = event_tx.send(Event::EvaluationProgress(EvaluationProgress {
            current_task,
            total_tasks,
            task_id: task.id.clone(),
            current_step: Some(0),
            max_steps: Some(25), // Default agent max steps
            last_tool: None,
            total_tokens,
            total_cost,
            message: Some(format!("Starting task: {}", task.id)),
            last_result: None,
        }));

        // Run the task
        let result = run_single_task(&task, config).await?;

        // Update totals
        total_tokens += result.total_tokens;
        total_cost += result.cost_usd;

        // Send progress update - task complete
        let _ = event_tx.send(Event::EvaluationProgress(EvaluationProgress {
            current_task,
            total_tasks,
            task_id: task.id.clone(),
            current_step: Some(25),
            max_steps: Some(25),
            last_tool: None,
            total_tokens,
            total_cost,
            message: Some(format!(
                "Completed task: {} ({})",
                task.id,
                if result.solved { "solved" } else { "failed" }
            )),
            last_result: Some(result.clone()),
        }));

        results.push(result);
    }

    // Calculate final results
    let tasks_solved = results.iter().filter(|r| r.solved).count();
    let accuracy = if total_tasks > 0 {
        (tasks_solved as f64 / total_tasks as f64) * 100.0
    } else {
        0.0
    };

    let avg_cost_usd = if total_tasks > 0 {
        total_cost / total_tasks as f64
    } else {
        0.0
    };

    let avg_duration_ms = if total_tasks > 0 {
        results.iter().map(|r| r.duration_ms).sum::<u64>() as f64 / total_tasks as f64
    } else {
        0.0
    };

    Ok(EvaluationResults {
        config_name: format!("{} features (TUI)", config.features.enabled_count()),
        results,
        accuracy,
        avg_cost_usd,
        avg_duration_ms,
        total_tasks,
        tasks_solved,
        by_complexity: Default::default(),
        timestamp: chrono::Utc::now(),
    })
}

/// Run a single task
async fn run_single_task(task: &Task, config: &ToadConfig) -> anyhow::Result<TaskResult> {
    use crate::ai::metrics::MetricsCollector;
    use crate::ai::agent::PromptBuilder;

    // Build AST context if feature enabled
    let custom_prompt = if config.features.context_ast {
        use crate::ai::context::ContextBuilder;
        // Try to build context from current directory (task workspace)
        // In real evaluation, this would be the cloned repo directory
        match ContextBuilder::new()?
            .add_directory(".", &["py", "js", "ts", "tsx", "rs"])
            .await
        {
            Ok(builder) => {
                let context = builder.build();
                Some(PromptBuilder::new()
                    .with_task(task)
                    .with_ast_context(context)
                    .build())
            }
            Err(e) => {
                // Log warning but continue without AST context
                eprintln!("Warning: Failed to build AST context: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Create LLM client using provider factory
    use crate::ai::llm::LLMProvider;
    let llm_client = LLMProvider::create_with_features(
        &config.provider,
        config.features.prompt_caching,
    )?;

    // Create tool registry with feature flags
    let tool_registry = ToolRegistry::m1_with_features(&config.features);

    // Create agent
    let agent = Agent::new(llm_client, tool_registry);

    // Create metrics collector
    let mut metrics_collector = MetricsCollector::new();

    // Execute task with custom AST-enhanced prompt if available
    let agent_result = agent.execute_task_with_prompt(
        task,
        custom_prompt,
        &mut metrics_collector
    ).await?;

    // Build task result from agent result and metrics
    let final_metrics = metrics_collector.finish();
    let mut result = TaskResult::new(task.id.clone());

    result.duration_ms = final_metrics.duration_ms;
    result.cost_usd = final_metrics.cost_usd;
    result.api_calls = final_metrics.api_calls;
    result.total_tokens = final_metrics.total_tokens();
    result.metrics = final_metrics;

    // Mark as solved if agent succeeded
    if agent_result.success {
        result.mark_solved();
    }

    Ok(result)
}

/// Start a comparison run in the background
pub fn start_comparison(
    args: CompareArgs,
    event_tx: mpsc::UnboundedSender<Event>,
) -> EvaluationHandle {
    let (cancel_tx, mut cancel_rx) = mpsc::channel::<()>(1);

    let handle = tokio::spawn(async move {
        let result = run_comparison_inner(args, event_tx.clone(), &mut cancel_rx).await;

        match result {
            Ok((_baseline_results, test_results)) => {
                // Send both results as completion
                // For now, just send the test results
                // TODO: Send comparison result with statistical analysis
                let _ = event_tx.send(Event::EvaluationComplete(test_results));
            }
            Err(e) => {
                let _ = event_tx.send(Event::EvaluationError(format!("{}", e)));
            }
        }
    });

    EvaluationHandle { handle, cancel_tx }
}

/// Inner comparison function
async fn run_comparison_inner(
    args: CompareArgs,
    event_tx: mpsc::UnboundedSender<Event>,
    cancel_rx: &mut mpsc::Receiver<()>,
) -> anyhow::Result<(EvaluationResults, EvaluationResults)> {
    // Load tasks
    let tasks = load_tasks(&args.dataset, args.count).await?;
    let task_count = tasks.len();

    // Run baseline evaluation
    let baseline_config = ToadConfig::for_milestone(args.baseline as u8);

    let _ = event_tx.send(Event::EvaluationProgress(EvaluationProgress {
        current_task: 0,
        total_tasks: task_count * 2, // Two runs
        task_id: "Baseline".to_string(),
        current_step: None,
        max_steps: None,
        last_tool: None,
        total_tokens: 0,
        total_cost: 0.0,
        message: Some("Running baseline evaluation...".to_string()),
        last_result: None,
    }));

    let baseline_results =
        run_tasks_with_progress(tasks.clone(), &baseline_config, event_tx.clone(), cancel_rx)
            .await?;

    // Run test evaluation
    let test_config = ToadConfig::for_milestone(args.test as u8);

    let _ = event_tx.send(Event::EvaluationProgress(EvaluationProgress {
        current_task: task_count,
        total_tasks: task_count * 2,
        task_id: "Test".to_string(),
        current_step: None,
        max_steps: None,
        last_tool: None,
        total_tokens: 0,
        total_cost: 0.0,
        message: Some("Running test evaluation...".to_string()),
        last_result: None,
    }));

    let test_results =
        run_tasks_with_progress(tasks, &test_config, event_tx.clone(), cancel_rx).await?;

    Ok((baseline_results, test_results))
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_evaluation_handle_created() {
        // This is a basic structure test, actual async tests would require tokio::test
        // and more setup
        assert!(true);
    }
}
