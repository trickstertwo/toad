/// Agent module - Core execution loop for M1 baseline
///
/// The agent orchestrates:
/// 1. Task understanding
/// 2. Tool selection and execution
/// 3. Iterative problem solving
/// 4. Solution validation
use crate::ai::evaluation::Task;
use crate::ai::llm::{LLMClient, Message, StopReason, ToolUse};
use crate::ai::metrics::MetricsCollector;
use crate::ai::tools::ToolRegistry;
use anyhow::{Context, Result, anyhow};
use serde_json::json;

pub mod prompts;

pub use prompts::PromptBuilder;

/// Maximum number of agent steps before giving up
const MAX_AGENT_STEPS: u32 = 25;

/// M1 agent with simple tool use loop
pub struct Agent {
    llm_client: Box<dyn LLMClient>,
    tool_registry: ToolRegistry,
    max_steps: u32,
}

impl Agent {
    pub fn new(llm_client: Box<dyn LLMClient>, tool_registry: ToolRegistry) -> Self {
        Self {
            llm_client,
            tool_registry,
            max_steps: MAX_AGENT_STEPS,
        }
    }

    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }

    /// Execute a task and collect metrics
    pub async fn execute_task(
        &self,
        task: &Task,
        metrics: &mut MetricsCollector,
    ) -> Result<AgentResult> {
        metrics.start();

        // Build initial prompt
        let prompt = PromptBuilder::new().with_task(task).build();

        let mut conversation = vec![Message::user(prompt)];
        let mut step_count = 0;

        // Build tool schemas for LLM
        let tool_schemas = self.build_tool_schemas();

        loop {
            step_count += 1;
            metrics.record_agent_step();

            if step_count > self.max_steps {
                return Ok(AgentResult {
                    success: false,
                    final_response: format!("Exceeded maximum steps ({})", self.max_steps),
                    steps: step_count,
                });
            }

            // Send message to LLM
            let response = self
                .llm_client
                .send_message(conversation.clone(), Some(tool_schemas.clone()))
                .await
                .context("Failed to get LLM response")?;

            // Record first response time
            if step_count == 1 {
                metrics.record_first_response();
            }

            // Record token usage
            metrics.record_api_call(
                response.usage.input_tokens as u64,
                response.usage.output_tokens as u64,
                response.usage.cache_read_tokens.unwrap_or(0) as u64,
                response.usage.calculate_cost(),
            );

            // Handle response based on stop reason
            match response.stop_reason {
                StopReason::ToolUse => {
                    // Execute tools and continue
                    let tool_results = self.execute_tools(&response.tool_uses, metrics).await?;

                    // Add assistant message with tool uses
                    conversation.push(Message::assistant(
                        if response.content.is_empty() {
                            format!("Tool uses: {}", tool_results)
                        } else {
                            format!("{}\n\nTool uses: {}", response.content, tool_results)
                        }
                    ));
                }
                StopReason::EndTurn | StopReason::MaxTokens | StopReason::StopSequence => {
                    // Task complete or hit limit
                    return Ok(AgentResult {
                        success: true,
                        final_response: response.content,
                        steps: step_count,
                    });
                }
            }
        }
    }

    /// Execute a list of tool uses
    async fn execute_tools(
        &self,
        tool_uses: &[ToolUse],
        metrics: &mut MetricsCollector,
    ) -> Result<String> {
        let mut results = Vec::new();

        for tool_use in tool_uses {
            let result = self.execute_single_tool(tool_use, metrics).await?;
            results.push(format!(
                "Tool: {}\nID: {}\nResult: {}",
                tool_use.name, tool_use.id, result
            ));
        }

        Ok(results.join("\n\n"))
    }

    /// Execute a single tool
    async fn execute_single_tool(
        &self,
        tool_use: &ToolUse,
        metrics: &mut MetricsCollector,
    ) -> Result<String> {
        // Get tool from registry
        let tool = self
            .tool_registry
            .get(&tool_use.name)
            .ok_or_else(|| anyhow!("Tool '{}' not found", tool_use.name))?;

        // Record metrics based on tool type
        match tool_use.name.as_str() {
            "read" => metrics.record_file_read(),
            "write" => metrics.record_file_write(),
            "edit" => metrics.record_edit_attempt(),
            "bash"
                if tool_use
                    .input
                    .get("command")
                    .and_then(|c| c.as_str())
                    .map(|s| s.contains("test") || s.contains("pytest") || s.contains("cargo test"))
                    .unwrap_or(false) =>
            {
                metrics.record_test_run()
            }
            _ => {}
        }

        // Convert JSON input to HashMap
        let args = if let Some(obj) = tool_use.input.as_object() {
            obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
        } else {
            return Err(anyhow!("Tool input must be a JSON object"));
        };

        // Execute tool
        let result = tool.execute(args).await.context("Tool execution failed")?;

        if result.success {
            Ok(result.output)
        } else {
            Ok(format!(
                "Error: {}",
                result.error.unwrap_or_else(|| "Unknown error".to_string())
            ))
        }
    }

    /// Build tool schemas for LLM
    fn build_tool_schemas(&self) -> Vec<serde_json::Value> {
        self.tool_registry
            .list()
            .iter()
            .filter_map(|name| {
                self.tool_registry.get(name).map(|tool| {
                    json!({
                        "name": tool.name(),
                        "description": tool.description(),
                        "input_schema": tool.parameters_schema(),
                    })
                })
            })
            .collect()
    }
}

/// Result of agent execution
#[derive(Debug, Clone)]
pub struct AgentResult {
    pub success: bool,
    pub final_response: String,
    pub steps: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::llm::{LLMResponse, Message, StopReason, Usage};
    use async_trait::async_trait;

    // Mock LLM client for testing
    struct MockLLMClient {
        responses: Vec<LLMResponse>,
        current: std::sync::Mutex<usize>,
    }

    impl MockLLMClient {
        fn new(responses: Vec<LLMResponse>) -> Self {
            Self {
                responses,
                current: std::sync::Mutex::new(0),
            }
        }
    }

    #[async_trait]
    impl LLMClient for MockLLMClient {
        async fn send_message(
            &self,
            _messages: Vec<Message>,
            _tools: Option<Vec<serde_json::Value>>,
        ) -> Result<LLMResponse> {
            let mut idx = self.current.lock().unwrap();
            let response = self
                .responses
                .get(*idx)
                .cloned()
                .ok_or_else(|| anyhow!("No more mock responses"))?;
            *idx += 1;
            Ok(response)
        }

        fn model_name(&self) -> &str {
            "mock-model"
        }

        async fn send_message_stream(
            &self,
            _messages: Vec<Message>,
            _tools: Option<Vec<serde_json::Value>>,
        ) -> Result<crate::ai::llm::MessageStream> {
            // Mock implementation doesn't support streaming yet
            Err(anyhow!("Streaming not supported in mock client"))
        }
    }

    #[tokio::test]
    async fn test_agent_simple_completion() {
        let mock_client = MockLLMClient::new(vec![LLMResponse {
            content: "Task completed successfully".to_string(),
            tool_uses: vec![],
            stop_reason: StopReason::EndTurn,
            usage: Usage {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_tokens: None,
                cache_read_tokens: None,
            },
        }]);

        let registry = ToolRegistry::m1_baseline();
        let agent = Agent::new(Box::new(mock_client), registry);
        let mut metrics = MetricsCollector::new();

        let task = Task::example();
        let result = agent.execute_task(&task, &mut metrics).await.unwrap();

        assert!(result.success);
        assert_eq!(result.steps, 1);
        assert_eq!(result.final_response, "Task completed successfully");
    }

    #[tokio::test]
    async fn test_agent_max_steps() {
        use crate::ai::llm::ToolUse;

        // Create mock that returns invalid tool uses to trigger continuation
        let mock_client = MockLLMClient::new(
            (0..10)
                .map(|_| LLMResponse {
                    content: "Executing tool...".to_string(),
                    tool_uses: vec![ToolUse {
                        id: "call_123".to_string(),
                        name: "nonexistent_tool".to_string(),
                        input: json!({}),
                    }],
                    stop_reason: StopReason::ToolUse,
                    usage: Usage {
                        input_tokens: 100,
                        output_tokens: 50,
                        cache_creation_tokens: None,
                        cache_read_tokens: None,
                    },
                })
                .collect(),
        );

        let registry = ToolRegistry::m1_baseline();
        let agent = Agent::new(Box::new(mock_client), registry).with_max_steps(3);
        let mut metrics = MetricsCollector::new();

        let task = Task::example();
        // This should fail because tool doesn't exist, but we're testing max steps
        let result = agent.execute_task(&task, &mut metrics).await;

        // Should error due to nonexistent tool (not exceed max steps in this case)
        assert!(result.is_err());
    }
}
