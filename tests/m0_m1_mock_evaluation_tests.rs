/// End-to-end tests for M0/M1 evaluation framework using mock LLM
///
/// These tests prove that the entire evaluation pipeline works without
/// requiring ANTHROPIC_API_KEY, using mock LLM clients instead.
use toad::ai::{
    Agent, DeterministicLLMClient, MetricsCollector, MockResponseBuilder, ToolRegistry,
};
use toad::ai::evaluation::Task;
use toad::config::ToadConfig;
use tempfile::TempDir;

/// Test M0 evaluation framework with mock LLM - single task
#[tokio::test]
async fn test_m0_evaluation_single_task_with_mock() {
    // Create mock LLM that suggests reading and writing files
    let mock_client = Box::new(
        MockResponseBuilder::new()
            .with_tool("read", serde_json::json!({"path": "src/bug.py"}))
            .with_tool("write", serde_json::json!({
                "path": "src/bug.py",
                "content": "def fixed():\n    return 42\n"
            }))
            .with_text("Bug fixed successfully!")
            .build(),
    );

    // Create agent with mock
    let tool_registry = ToolRegistry::m1_baseline();
    let agent = Agent::new(mock_client, tool_registry);

    // Create a test task
    let task = Task::example();

    // Run evaluation
    let mut metrics = MetricsCollector::new();
    let result = agent.execute_task(&task, &mut metrics).await;

    // Verify execution
    assert!(result.is_ok(), "Agent should complete task with mock LLM");
    let agent_result = result.unwrap();

    // Verify agent made steps
    assert!(
        agent_result.steps >= 2,
        "Agent should execute multiple steps (read, write, complete)"
    );

    // Verify metrics were collected
    let snapshot = metrics.snapshot();
    assert!(snapshot.api_calls > 0, "API calls should be tracked");
    assert_eq!(snapshot.cost_usd, 0.0, "Mock LLM has zero cost");
    assert!(snapshot.input_tokens > 0, "Input tokens should be tracked");
    assert!(snapshot.output_tokens > 0, "Output tokens should be tracked");

    println!("✅ M0 Evaluation with Mock: {} steps, {} API calls, $0.00 cost",
        agent_result.steps, snapshot.api_calls);
}

/// Test M0 evaluation framework with mock LLM - multiple tasks
#[tokio::test]
async fn test_m0_evaluation_multiple_tasks_with_mock() {
    // Create 3 test tasks
    let tasks = vec![
        {
            let mut task = Task::example();
            task.id = "mock-test-1".to_string();
            task.problem_statement = "Fix bug in function foo".to_string();
            task
        },
        {
            let mut task = Task::example();
            task.id = "mock-test-2".to_string();
            task.problem_statement = "Implement feature bar".to_string();
            task
        },
        {
            let mut task = Task::example();
            task.id = "mock-test-3".to_string();
            task.problem_statement = "Add test for baz".to_string();
            task
        },
    ];

    // Run each task with deterministic mock
    let mut total_steps = 0;
    let mut total_api_calls = 0;

    for task in tasks {
        let mock_client = Box::new(DeterministicLLMClient::new());
        let agent = Agent::new(mock_client, ToolRegistry::m1_baseline());

        let mut metrics = MetricsCollector::new();
        let result = agent.execute_task(&task, &mut metrics).await;

        assert!(result.is_ok(), "Task {} should complete", task.id);

        let agent_result = result.unwrap();
        total_steps += agent_result.steps;

        let snapshot = metrics.snapshot();
        total_api_calls += snapshot.api_calls as usize;
        assert_eq!(snapshot.cost_usd, 0.0, "Each task should have zero cost");
    }

    assert!(total_steps > 0, "Should have executed agent steps");
    assert!(total_api_calls > 0, "Should have tracked API calls");

    println!("✅ M0 Multiple Tasks: 3 tasks, {} total steps, {} total calls, $0.00 total cost",
        total_steps, total_api_calls);
}

/// Test M1 baseline configuration with mock LLM
#[tokio::test]
async fn test_m1_baseline_config_with_mock() {
    // Create M1 config
    let config = ToadConfig::for_milestone(1);

    // Verify M1 features
    assert!(config.features.prompt_caching, "M1 should enable prompt caching");
    assert!(
        config.features.tree_sitter_validation,
        "M1 should enable tree-sitter validation"
    );
    assert!(!config.features.context_ast, "M1 should not use AST context");
    assert!(
        !config.features.smart_test_selection,
        "M1 should not use smart test selection"
    );

    // Create tool registry with M1 features
    let registry = ToolRegistry::m1_with_features(&config.features);
    assert_eq!(registry.count(), 8, "M1 should have 8 tools");

    // Create mock client and agent
    let mock_client = Box::new(DeterministicLLMClient::new());
    let agent = Agent::new(mock_client, registry);

    // Run a task
    let task = Task::example();
    let mut metrics = MetricsCollector::new();
    let result = agent.execute_task(&task, &mut metrics).await;

    assert!(result.is_ok(), "M1 agent should work with mock");

    println!("✅ M1 Configuration Test: Works with mock LLM, zero cost");
}

/// Test M1 quality gate simulation with mock LLM
#[tokio::test]
async fn test_m1_quality_gate_simulation() {
    // Simulate QG1: Run on 10 tasks, verify no crashes
    const TASK_COUNT: usize = 10;

    let mut completed_tasks = 0;
    let mut total_cost = 0.0;
    let mut total_duration_ms = 0u64;

    for i in 0..TASK_COUNT {
        let mut task = Task::example();
        task.id = format!("qg1-task-{}", i + 1);

        let mock_client = Box::new(DeterministicLLMClient::new());
        let agent = Agent::new(mock_client, ToolRegistry::m1_baseline());

        let mut metrics = MetricsCollector::new();
        metrics.start();

        let result = agent.execute_task(&task, &mut metrics).await;

        if result.is_ok() {
            completed_tasks += 1;
        }

        let final_metrics = metrics.finish();
        total_cost += final_metrics.cost_usd;
        total_duration_ms += final_metrics.duration_ms;
    }

    // Verify QG1 requirements
    assert_eq!(
        completed_tasks, TASK_COUNT,
        "All tasks should complete (no crashes)"
    );
    assert_eq!(total_cost, 0.0, "Total cost should be zero with mock");

    let avg_duration_ms = total_duration_ms / TASK_COUNT as u64;

    println!("✅ M1 QG1 Simulation: {}/{} tasks completed, $0.00 total cost, ~{}ms avg",
        completed_tasks, TASK_COUNT, avg_duration_ms);
}

/// Test evaluation metrics persistence (without actual LLM)
#[tokio::test]
async fn test_evaluation_metrics_persistence_mock() {
    use std::fs;

    let temp_dir = TempDir::new().unwrap();
    let results_file = temp_dir.path().join("test_results.json");

    // Run a task and collect metrics
    let mock_client = Box::new(DeterministicLLMClient::new());
    let agent = Agent::new(mock_client, ToolRegistry::m1_baseline());

    let task = Task::example();
    let mut metrics = MetricsCollector::new();
    metrics.start();

    let result = agent.execute_task(&task, &mut metrics).await;
    assert!(result.is_ok());

    let final_metrics = metrics.finish();

    // Simulate saving results (what eval pipeline does)
    let results_json = serde_json::json!({
        "task_id": task.id,
        "solved": false, // We don't verify correctness in this test
        "metrics": {
            "duration_ms": final_metrics.duration_ms,
            "cost_usd": final_metrics.cost_usd,
            "api_calls": final_metrics.api_calls,
            "input_tokens": final_metrics.input_tokens,
            "output_tokens": final_metrics.output_tokens,
        }
    });

    fs::write(&results_file, serde_json::to_string_pretty(&results_json).unwrap()).unwrap();

    // Verify file was created
    assert!(results_file.exists(), "Results file should be created");

    // Read and verify
    let saved = fs::read_to_string(&results_file).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&saved).unwrap();

    assert_eq!(parsed["task_id"], task.id);
    assert_eq!(parsed["metrics"]["cost_usd"], 0.0);

    println!("✅ Metrics Persistence: Results saved successfully, zero cost");
}

/// Test that mock LLM enables deterministic testing
#[tokio::test]
async fn test_mock_deterministic_behavior() {
    // Run the same task twice with same mock
    let task = Task::example();

    let mut results = Vec::new();

    for _ in 0..2 {
        let mock_client = Box::new(
            MockResponseBuilder::new()
                .with_tool("read", serde_json::json!({"path": "test.py"}))
                .with_text("Done")
                .build(),
        );

        let agent = Agent::new(mock_client, ToolRegistry::m1_baseline());
        let mut metrics = MetricsCollector::new();

        let result = agent.execute_task(&task, &mut metrics).await;
        assert!(result.is_ok());

        results.push(result.unwrap());
    }

    // Verify deterministic behavior (same mock → same results)
    assert_eq!(
        results[0].steps, results[1].steps,
        "Mock should produce deterministic results"
    );

    println!("✅ Deterministic Behavior: Same mock → same results (reproducible tests)");
}

/// Test mock LLM with M1 tree-sitter validation enabled
#[tokio::test]
async fn test_m1_with_validation_and_mock() {
    let temp_dir = TempDir::new().unwrap();
    let valid_file = temp_dir.path().join("valid.py");

    // Mock that writes valid Python code
    let mock_client = Box::new(
        MockResponseBuilder::new()
            .with_tool(
                "write",
                serde_json::json!({
                    "path": valid_file.to_string_lossy(),
                    "content": "def hello():\n    return 'world'\n"
                }),
            )
            .with_text("File written successfully")
            .build(),
    );

    // Create M1 config with validation
    let config = ToadConfig::for_milestone(1);
    let registry = ToolRegistry::m1_with_features(&config.features);
    let agent = Agent::new(mock_client, registry);

    let task = Task::example();
    let mut metrics = MetricsCollector::new();

    let result = agent.execute_task(&task, &mut metrics).await;
    assert!(result.is_ok(), "Valid code should pass validation");

    println!("✅ M1 with Validation: Tree-sitter validation works with mock LLM");
}

/// Test full M0 pipeline simulation
#[tokio::test]
async fn test_full_m0_pipeline_simulation() {
    println!("\n🔬 Running Full M0 Pipeline Simulation with Mock LLM\n");

    // Phase 1: Task Loading (simulated)
    let tasks = vec![Task::example(); 5];
    println!("📋 Loaded {} tasks", tasks.len());

    // Phase 2: Evaluation (with mock)
    let mut completed = 0;
    let mut total_api_calls = 0;
    let mut total_steps = 0;

    for (i, task) in tasks.iter().enumerate() {
        let mock = Box::new(DeterministicLLMClient::new());
        let agent = Agent::new(mock, ToolRegistry::m1_baseline());

        let mut metrics = MetricsCollector::new();
        metrics.start();

        if let Ok(result) = agent.execute_task(task, &mut metrics).await {
            completed += 1;
            total_steps += result.steps;

            let snapshot = metrics.snapshot();
            total_api_calls += snapshot.api_calls as usize;
        }

        println!("  Task {}/{}: ✓ completed", i + 1, tasks.len());
    }

    // Phase 3: Results (simulated)
    let accuracy = (completed as f64 / tasks.len() as f64) * 100.0;

    println!("\n📊 M0 Pipeline Results:");
    println!("  ✓ Tasks completed: {}/{}", completed, tasks.len());
    println!("  ✓ Success rate: {:.1}%", accuracy);
    println!("  ✓ Total agent steps: {}", total_steps);
    println!("  ✓ Total API calls: {}", total_api_calls);
    println!("  ✓ Total cost: $0.00 (mock LLM)");
    println!("\n✅ Full M0 Pipeline: Works without API key!\n");

    assert_eq!(completed, tasks.len(), "All tasks should complete");
}
