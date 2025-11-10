/// Mock and deterministic LLM clients for testing
///
/// Provides testing utilities for agent execution without requiring API keys:
/// - DeterministicLLMClient: Simple, rule-based responses
/// - Mock support via mockall for complex test scenarios
use super::{LLMClient, LLMResponse, Message, Role, StopReason, ToolUse, Usage};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;

/// Deterministic LLM client for integration testing
///
/// Returns predefined responses based on simple rules:
/// - If user message contains "read", suggests using read tool
/// - If user message contains "write", suggests using write tool
/// - If user message contains "test", suggests using bash tool to run tests
/// - Otherwise, returns a text response
///
/// This allows testing the full agent loop without API calls.
pub struct DeterministicLLMClient {
    /// Response counter for tracking calls
    call_count: std::sync::Mutex<usize>,

    /// Whether to stop after first response (default: true)
    stop_after_first: bool,
}

impl DeterministicLLMClient {
    /// Create a new deterministic client
    pub fn new() -> Self {
        Self {
            call_count: std::sync::Mutex::new(0),
            stop_after_first: true,
        }
    }

    /// Create a deterministic client that continues for multiple turns
    pub fn multi_turn() -> Self {
        Self {
            call_count: std::sync::Mutex::new(0),
            stop_after_first: false,
        }
    }

    /// Get the number of times send_message was called
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// Determine what tool to suggest based on message content
    fn determine_tool_response(&self, messages: &[Message]) -> Option<ToolUse> {
        // Get the last user message
        let last_message = messages.iter().rev().find(|m| m.role == Role::User)?;
        let content = last_message.content.to_lowercase();

        // Simple pattern matching for tool selection
        if content.contains("read") && content.contains("file") {
            Some(ToolUse {
                id: "tool_1".to_string(),
                name: "read".to_string(),
                input: json!({
                    "path": "README.md"
                }),
            })
        } else if content.contains("write") {
            Some(ToolUse {
                id: "tool_2".to_string(),
                name: "write".to_string(),
                input: json!({
                    "path": "output.txt",
                    "content": "Test content"
                }),
            })
        } else if content.contains("list") {
            Some(ToolUse {
                id: "tool_3".to_string(),
                name: "list".to_string(),
                input: json!({
                    "path": "."
                }),
            })
        } else if content.contains("test") || content.contains("pytest") {
            Some(ToolUse {
                id: "tool_4".to_string(),
                name: "bash".to_string(),
                input: json!({
                    "command": "pytest tests/ -v"
                }),
            })
        } else {
            None
        }
    }

    /// Determine appropriate text response
    fn determine_text_response(&self, messages: &[Message]) -> String {
        let last_message = messages.iter().rev().find(|m| m.role == Role::User);

        if let Some(msg) = last_message {
            let content = msg.content.to_lowercase();

            if content.contains("fix") || content.contains("bug") {
                "I'll analyze the issue and suggest a fix.".to_string()
            } else if content.contains("implement") || content.contains("add") {
                "I'll implement the requested functionality.".to_string()
            } else if content.contains("test") {
                "I'll run the tests to verify the changes.".to_string()
            } else {
                "Task completed successfully.".to_string()
            }
        } else {
            "Ready to help.".to_string()
        }
    }
}

impl Default for DeterministicLLMClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LLMClient for DeterministicLLMClient {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse> {
        // Increment call count
        {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
        }

        let count = self.call_count();

        // Check if we should stop
        if self.stop_after_first && count > 1 {
            return Ok(LLMResponse {
                content: "Task completed.".to_string(),
                stop_reason: StopReason::EndTurn,
                tool_uses: Vec::new(),
                usage: Usage {
                    input_tokens: 100,
                    output_tokens: 10,
                    cache_creation_tokens: Some(0),
                    cache_read_tokens: Some(0),
                },
            });
        }

        // Determine response based on message content
        if let Some(tool_use) = self.determine_tool_response(&messages) {
            // Suggest tool use
            Ok(LLMResponse {
                content: "I'll use the appropriate tool.".to_string(),
                stop_reason: StopReason::ToolUse,
                tool_uses: vec![tool_use],
                usage: Usage {
                    input_tokens: 100,
                    output_tokens: 50,
                    cache_creation_tokens: Some(0),
                    cache_read_tokens: Some(0),
                },
            })
        } else {
            // Return text response
            Ok(LLMResponse {
                content: self.determine_text_response(&messages),
                stop_reason: StopReason::EndTurn,
                tool_uses: Vec::new(),
                usage: Usage {
                    input_tokens: 100,
                    output_tokens: 20,
                    cache_creation_tokens: Some(0),
                    cache_read_tokens: Some(0),
                },
            })
        }
    }

    async fn send_message_stream(
        &self,
        _messages: Vec<Message>,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<super::MessageStream> {
        // Mock doesn't support streaming
        anyhow::bail!("Streaming not supported in DeterministicLLMClient")
    }

    fn model_name(&self) -> &str {
        "mock-deterministic"
    }
}

/// Builder for creating custom mock responses
pub struct MockResponseBuilder {
    responses: Vec<LLMResponse>,
    current_index: std::sync::Mutex<usize>,
}

impl MockResponseBuilder {
    /// Create a new response builder
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            current_index: std::sync::Mutex::new(0),
        }
    }

    /// Add a text response
    pub fn with_text(mut self, content: impl Into<String>) -> Self {
        self.responses.push(LLMResponse {
            content: content.into(),
            stop_reason: StopReason::EndTurn,
            tool_uses: Vec::new(),
            usage: Usage {
                input_tokens: 100,
                output_tokens: 20,
                cache_creation_tokens: Some(0),
                cache_read_tokens: Some(0),
            },
        });
        self
    }

    /// Add a tool use response
    pub fn with_tool(mut self, tool_name: impl Into<String>, input: serde_json::Value) -> Self {
        let tool_name_string: String = tool_name.into();
        let tool_id = format!("tool_{}", self.responses.len() + 1);

        self.responses.push(LLMResponse {
            content: format!("Using tool: {}", tool_name_string),
            stop_reason: StopReason::ToolUse,
            tool_uses: vec![ToolUse {
                id: tool_id,
                name: tool_name_string,
                input,
            }],
            usage: Usage {
                input_tokens: 100,
                output_tokens: 50,
                cache_creation_tokens: Some(0),
                cache_read_tokens: Some(0),
            },
        });
        self
    }

    /// Build a sequenced mock client
    pub fn build(self) -> SequencedMockClient {
        SequencedMockClient {
            responses: self.responses,
            current_index: self.current_index,
        }
    }
}

impl Default for MockResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock client that returns responses in sequence
pub struct SequencedMockClient {
    responses: Vec<LLMResponse>,
    current_index: std::sync::Mutex<usize>,
}

#[async_trait]
impl LLMClient for SequencedMockClient {
    async fn send_message(
        &self,
        _messages: Vec<Message>,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse> {
        let mut index = self.current_index.lock().unwrap();
        let response = self.responses.get(*index).cloned().unwrap_or_else(|| {
            // Default fallback response
            LLMResponse {
                content: "No more responses configured".to_string(),
                stop_reason: StopReason::EndTurn,
                tool_uses: Vec::new(),
                usage: Usage {
                    input_tokens: 0,
                    output_tokens: 0,
                    cache_creation_tokens: Some(0),
                    cache_read_tokens: Some(0),
                },
            }
        });

        *index += 1;
        Ok(response)
    }

    async fn send_message_stream(
        &self,
        _messages: Vec<Message>,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<super::MessageStream> {
        anyhow::bail!("Streaming not supported in SequencedMockClient")
    }

    fn model_name(&self) -> &str {
        "mock-sequenced"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deterministic_client_read_tool() {
        let client = DeterministicLLMClient::new();
        let messages = vec![Message::user("Please read file README.md")];

        let response = client.send_message(messages, None).await.unwrap();

        assert_eq!(response.stop_reason, StopReason::ToolUse);
        assert_eq!(response.tool_uses.len(), 1);
        assert_eq!(response.tool_uses[0].name, "read");
    }

    #[tokio::test]
    async fn test_deterministic_client_write_tool() {
        let client = DeterministicLLMClient::new();
        let messages = vec![Message::user("Write to file output.txt")];

        let response = client.send_message(messages, None).await.unwrap();

        assert_eq!(response.stop_reason, StopReason::ToolUse);
        assert_eq!(response.tool_uses.len(), 1);
        assert_eq!(response.tool_uses[0].name, "write");
    }

    #[tokio::test]
    async fn test_deterministic_client_text_response() {
        let client = DeterministicLLMClient::new();
        let messages = vec![Message::user("Fix the bug in the code")];

        let response = client.send_message(messages, None).await.unwrap();

        assert_eq!(response.stop_reason, StopReason::EndTurn);
        assert!(response.tool_uses.is_empty());
        assert!(response.content.contains("fix"));
    }

    #[tokio::test]
    async fn test_deterministic_client_stops_after_first() {
        let client = DeterministicLLMClient::new();

        // First call
        let response1 = client
            .send_message(vec![Message::user("Hello")], None)
            .await
            .unwrap();
        assert_eq!(client.call_count(), 1);

        // Second call should stop
        let response2 = client
            .send_message(vec![Message::user("Hello again")], None)
            .await
            .unwrap();
        assert_eq!(client.call_count(), 2);
        assert_eq!(response2.stop_reason, StopReason::EndTurn);
    }

    #[tokio::test]
    async fn test_mock_response_builder() {
        let client = MockResponseBuilder::new()
            .with_tool("read", json!({"path": "test.txt"}))
            .with_text("File contents retrieved")
            .build();

        // First response: tool use
        let response1 = client
            .send_message(vec![Message::user("Read file")], None)
            .await
            .unwrap();
        assert_eq!(response1.stop_reason, StopReason::ToolUse);
        assert_eq!(response1.tool_uses[0].name, "read");

        // Second response: text
        let response2 = client
            .send_message(vec![Message::user("Thanks")], None)
            .await
            .unwrap();
        assert_eq!(response2.stop_reason, StopReason::EndTurn);
        assert!(response2.content.contains("contents"));
    }

    #[tokio::test]
    async fn test_sequenced_mock_exhaustion() {
        let client = MockResponseBuilder::new()
            .with_text("Only response")
            .build();

        // First call returns response
        let response1 = client
            .send_message(vec![Message::user("Hello")], None)
            .await
            .unwrap();
        assert!(response1.content.contains("Only response"));

        // Second call returns fallback
        let response2 = client
            .send_message(vec![Message::user("Hello")], None)
            .await
            .unwrap();
        assert!(response2.content.contains("No more responses"));
    }
}
