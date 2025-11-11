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
use crate::core::event::{EvaluationProgress, ToolExecution};
use anyhow::{Context, Result, anyhow};
use serde_json::json;
use std::sync::Arc;

pub mod prompts;

pub use prompts::PromptBuilder;

/// Maximum number of agent steps before giving up
const MAX_AGENT_STEPS: u32 = 25;

/// Progress callback type for real-time updates
pub type ProgressCallback = Arc<dyn Fn(EvaluationProgress) + Send + Sync>;

/// M1 agent with simple tool use loop
pub struct Agent {
    llm_client: Box<dyn LLMClient>,
    tool_registry: ToolRegistry,
    max_steps: u32,
    progress_callback: Option<ProgressCallback>,
}

impl Agent {
    pub fn new(llm_client: Box<dyn LLMClient>, tool_registry: ToolRegistry) -> Self {
        Self {
            llm_client,
            tool_registry,
            max_steps: MAX_AGENT_STEPS,
            progress_callback: None,
        }
    }

    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn with_progress_callback(mut self, callback: ProgressCallback) -> Self {
        self.progress_callback = Some(callback);
        self
    }

    /// Execute a task and collect metrics
    pub async fn execute_task(
        &self,
        task: &Task,
        metrics: &mut MetricsCollector,
    ) -> Result<AgentResult> {
        self.execute_task_with_prompt(task, None, metrics).await
    }

    /// Execute a task with optional custom initial prompt
    pub async fn execute_task_with_prompt(
        &self,
        task: &Task,
        custom_prompt: Option<String>,
        metrics: &mut MetricsCollector,
    ) -> Result<AgentResult> {
        metrics.start();

        // Use custom prompt if provided, otherwise build default
        let prompt = custom_prompt.unwrap_or_else(|| PromptBuilder::new().with_task(task).build());

        let mut conversation = vec![Message::user(prompt.clone())];
        let mut step_count = 0;
        let mut tool_executions = Vec::new();
        let mut files_modified = Vec::new();

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

            let step_start = std::time::Instant::now();

            // Send message to LLM
            let response = self
                .llm_client
                .send_message(conversation.clone(), Some(tool_schemas.clone()))
                .await
                .context("Failed to get LLM response")?;

            let api_latency = step_start.elapsed().as_millis() as u64;

            // Record first response time
            if step_count == 1 {
                metrics.record_first_response();
            }

            // Record token usage
            let step_cost = response.usage.calculate_cost();
            metrics.record_api_call(
                response.usage.input_tokens as u64,
                response.usage.output_tokens as u64,
                response.usage.cache_read_tokens.unwrap_or(0) as u64,
                step_cost,
            );

            // Send progress update if callback is set
            if let Some(ref callback) = self.progress_callback {
                let current_metrics = metrics.snapshot();
                let mut progress = EvaluationProgress::new(0, 0, task.id.clone());
                progress.current_step = Some(step_count as usize);
                progress.max_steps = Some(self.max_steps as usize);
                progress.total_tokens = current_metrics.total_tokens();
                progress.total_cost = current_metrics.cost_usd;
                progress.conversation = conversation.clone();
                progress.tool_executions = tool_executions.clone();
                progress.problem_statement = Some(task.problem_statement.clone());
                progress.expected_solution = task.solution_patch.clone();
                progress.current_thinking = Some(response.content.clone());
                progress.files_modified = files_modified.clone();
                progress.step_duration_ms = Some(step_start.elapsed().as_millis() as u64);
                progress.api_latencies_ms = vec![api_latency];
                progress.step_input_tokens = Some(response.usage.input_tokens);
                progress.step_output_tokens = Some(response.usage.output_tokens);
                progress.cache_read_tokens = response.usage.cache_read_tokens;
                progress.message = Some(format!("Step {}/{}: {}", step_count, self.max_steps,
                    if !response.content.is_empty() {
                        &response.content[..std::cmp::min(100, response.content.len())]
                    } else {
                        "Processing tools..."
                    }));

                callback(progress);
            }

            // Handle response based on stop reason
            match response.stop_reason {
                StopReason::ToolUse => {
                    // Execute tools and continue
                    let (tool_results, tool_execution_records) = self.execute_tools_with_tracking(&response.tool_uses, metrics, &mut files_modified).await?;

                    // Add tool executions to tracking
                    tool_executions.extend(tool_execution_records);

                    // Add assistant message with tool uses
                    conversation.push(Message::assistant(if response.content.is_empty() {
                        format!("Tool uses: {}", tool_results)
                    } else {
                        format!("{}\n\nTool uses: {}", response.content, tool_results)
                    }));
                }
                StopReason::EndTurn | StopReason::MaxTokens | StopReason::StopSequence => {
                    // Add final assistant message
                    conversation.push(Message::assistant(response.content.clone()));

                    // Send final progress update
                    if let Some(ref callback) = self.progress_callback {
                        let current_metrics = metrics.snapshot();
                        let mut progress = EvaluationProgress::new(0, 0, task.id.clone());
                        progress.current_step = Some(step_count as usize);
                        progress.max_steps = Some(self.max_steps as usize);
                        progress.total_tokens = current_metrics.total_tokens();
                        progress.total_cost = current_metrics.cost_usd;
                        progress.conversation = conversation.clone();
                        progress.tool_executions = tool_executions.clone();
                        progress.files_modified = files_modified.clone();
                        progress.message = Some("Task complete".to_string());

                        callback(progress);
                    }

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

    /// Execute a list of tool uses with tracking
    async fn execute_tools_with_tracking(
        &self,
        tool_uses: &[ToolUse],
        metrics: &mut MetricsCollector,
        files_modified: &mut Vec<std::path::PathBuf>,
    ) -> Result<(String, Vec<ToolExecution>)> {
        let mut results = Vec::new();
        let mut executions = Vec::new();

        for tool_use in tool_uses {
            let tool_start = std::time::Instant::now();
            let result = self.execute_single_tool(tool_use, metrics).await;
            let duration_ms = tool_start.elapsed().as_millis() as u64;

            let (success, output, error) = match result {
                Ok(output) => {
                    // Track file modifications for Write and Edit tools
                    if tool_use.name == "write" || tool_use.name == "edit" {
                        if let Some(path_value) = tool_use.input.get("file_path")
                            .or_else(|| tool_use.input.get("path"))
                        {
                            if let Some(path_str) = path_value.as_str() {
                                files_modified.push(std::path::PathBuf::from(path_str));
                            }
                        }
                    }

                    results.push(format!(
                        "Tool: {}\nID: {}\nResult: {}",
                        tool_use.name, tool_use.id, output
                    ));
                    (true, output, None)
                }
                Err(e) => {
                    let error_msg = format!("Error: {}", e);
                    results.push(format!(
                        "Tool: {}\nID: {}\nResult: {}",
                        tool_use.name, tool_use.id, error_msg
                    ));
                    (false, String::new(), Some(error_msg))
                }
            };

            executions.push(ToolExecution {
                tool_name: tool_use.name.clone(),
                input: tool_use.input.clone(),
                output,
                success,
                error,
                duration_ms,
                timestamp: chrono::Utc::now(),
            });
        }

        Ok((results.join("\n\n"), executions))
    }

    /// Execute a list of tool uses (legacy method)
    async fn execute_tools(
        &self,
        tool_uses: &[ToolUse],
        metrics: &mut MetricsCollector,
    ) -> Result<String> {
        let mut files_modified = Vec::new();
        let (results, _) = self.execute_tools_with_tracking(tool_uses, metrics, &mut files_modified).await?;
        Ok(results)
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
