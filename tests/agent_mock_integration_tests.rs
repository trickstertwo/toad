use toad::ai::evaluation::Task;
/// Integration tests for Agent using mock LLM clients
///
/// These tests demonstrate how to test the agent execution loop
/// without requiring an ANTHROPIC_API_KEY.
use toad::ai::{
    Agent, DeterministicLLMClient, LLMClient, Message, MetricsCollector, MockResponseBuilder,
    ToolRegistry,
};

#[tokio::test]
async fn test_agent_with_deterministic_mock_read_tool() {
    // Create mock client that suggests read tool
    let llm_client = Box::new(DeterministicLLMClient::new());
    let tool_registry = ToolRegistry::m1_baseline();
    let agent = Agent::new(llm_client, tool_registry);

    // Create a simple task using the helper
    let mut task = Task::example();
    task.problem_statement = "Please read file README.md".to_string();

    let mut metrics = MetricsCollector::new();

    // Execute task - mock will suggest read tool, then stop
    let result = agent.execute_task(&task, &mut metrics).await;

    assert!(result.is_ok());
    let agent_result = result.unwrap();

    // Verify agent executed tools
    assert!(agent_result.steps > 0);

    let snapshot = metrics.snapshot();
    assert_eq!(snapshot.cost_usd, 0.0); // Mock has zero cost

    // Verify tool was used
    assert!(
        agent_result.steps >= 1,
        "Agent should have made at least one step"
    );
}

#[tokio::test]
async fn test_agent_with_sequenced_mock() {
    // Create mock with specific response sequence
    let llm_client = Box::new(
        MockResponseBuilder::new()
            .with_tool("read", serde_json::json!({"path": "src/main.rs"}))
            .with_tool(
                "write",
                serde_json::json!({
                    "path": "src/fixed.rs",
                    "content": "// Fixed code"
                }),
            )
            .with_text("Task completed successfully!")
            .build(),
    );

    let tool_registry = ToolRegistry::m1_baseline();
    let agent = Agent::new(llm_client, tool_registry);

    let mut task = Task::example();
    task.id = "test-2".to_string();
    task.problem_statement = "Fix the bug in main.rs".to_string();

    let mut metrics = MetricsCollector::new();
    let result = agent.execute_task(&task, &mut metrics).await;

    assert!(result.is_ok());
    let agent_result = result.unwrap();

    // Verify agent executed multiple steps (read, write, complete)
    assert!(
        agent_result.steps >= 2,
        "Agent should have executed at least 2 tool steps"
    );

    // Metrics should track API calls
    let snapshot = metrics.snapshot();
    assert!(snapshot.api_calls > 0);
}

#[tokio::test]
async fn test_agent_deterministic_multi_turn() {
    // Test multi-turn conversation
    let llm_client = Box::new(DeterministicLLMClient::multi_turn());
    let tool_registry = ToolRegistry::m1_baseline();
    let agent = Agent::new(llm_client, tool_registry);

    let mut task = Task::example();
    task.id = "test-3".to_string();
    task.problem_statement = "List files and write summary".to_string();

    let mut metrics = MetricsCollector::new();
    let result = agent.execute_task(&task, &mut metrics).await;

    assert!(result.is_ok());

    // Multi-turn client can execute multiple steps
    let agent_result = result.unwrap();
    assert!(agent_result.steps > 0);
}

#[tokio::test]
async fn test_mock_performance_metrics() {
    let llm_client = Box::new(DeterministicLLMClient::new());
    let tool_registry = ToolRegistry::m1_baseline();
    let agent = Agent::new(llm_client, tool_registry);

    let mut task = Task::example();
    task.id = "test-4".to_string();
    task.problem_statement = "Test metrics tracking".to_string();

    let mut metrics = MetricsCollector::new();
    let _ = agent.execute_task(&task, &mut metrics).await;

    // Verify metrics were collected
    let snapshot = metrics.snapshot();
    assert!(snapshot.api_calls > 0);
    assert_eq!(snapshot.cost_usd, 0.0); // Mock has zero cost

    // Input/output tokens should be tracked
    assert!(snapshot.input_tokens > 0);
    assert!(snapshot.output_tokens > 0);
}

#[tokio::test]
async fn test_mock_call_counting() {
    let mock = DeterministicLLMClient::new();

    // Make some calls
    let _ = mock.send_message(vec![Message::user("Hello")], None).await;
    assert_eq!(mock.call_count(), 1);

    let _ = mock.send_message(vec![Message::user("World")], None).await;
    assert_eq!(mock.call_count(), 2);
}
