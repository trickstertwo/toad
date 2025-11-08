/// Anthropic API client for Claude
///
/// Implements the Messages API: https://docs.anthropic.com/claude/reference/messages_post

use super::{LLMClient, LLMResponse, Message, StopReason, ToolUse, Usage};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::json;

const API_BASE_URL: &str = "https://api.anthropic.com/v1";
const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";
const DEFAULT_MAX_TOKENS: u32 = 4096;

pub struct AnthropicClient {
    api_key: String,
    model: String,
    max_tokens: u32,
    http_client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: DEFAULT_MODEL.to_string(),
            max_tokens: DEFAULT_MAX_TOKENS,
            http_client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

#[async_trait::async_trait]
impl LLMClient for AnthropicClient {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse> {
        // Build request body
        let mut body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": messages,
        });

        // Add tools if provided
        if let Some(tools_list) = tools {
            if !tools_list.is_empty() {
                body["tools"] = json!(tools_list);
            }
        }

        // Make API request
        let response = self
            .http_client
            .post(format!("{}/messages", API_BASE_URL))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Anthropic API")?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .context("Failed to read response body")?;

        if !status.is_success() {
            return Err(anyhow!(
                "Anthropic API error ({}): {}",
                status,
                response_text
            ));
        }

        // Parse response
        let api_response: ApiResponse =
            serde_json::from_str(&response_text).context("Failed to parse API response")?;

        // Extract content and tool uses
        let mut text_content = String::new();
        let mut tool_uses = Vec::new();

        for content_block in api_response.content {
            match content_block.r#type.as_str() {
                "text" => {
                    if let Some(text) = content_block.text {
                        if !text_content.is_empty() {
                            text_content.push('\n');
                        }
                        text_content.push_str(&text);
                    }
                }
                "tool_use" => {
                    if let (Some(id), Some(name), Some(input)) =
                        (content_block.id, content_block.name, content_block.input)
                    {
                        tool_uses.push(ToolUse { id, name, input });
                    }
                }
                _ => {}
            }
        }

        Ok(LLMResponse {
            content: text_content,
            tool_uses,
            stop_reason: api_response.stop_reason,
            usage: Usage {
                input_tokens: api_response.usage.input_tokens,
                output_tokens: api_response.usage.output_tokens,
                cache_creation_tokens: api_response.usage.cache_creation_input_tokens,
                cache_read_tokens: api_response.usage.cache_read_input_tokens,
            },
        })
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

/// API response structure
#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
    stop_reason: StopReason,
    usage: ApiUsage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    r#type: String,
    // For text blocks
    text: Option<String>,
    // For tool_use blocks
    id: Option<String>,
    name: Option<String>,
    input: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct ApiUsage {
    input_tokens: u32,
    output_tokens: u32,
    #[serde(default)]
    cache_creation_input_tokens: Option<u32>,
    #[serde(default)]
    cache_read_input_tokens: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = AnthropicClient::new("test-key".to_string());
        assert_eq!(client.model_name(), DEFAULT_MODEL);
        assert_eq!(client.max_tokens, DEFAULT_MAX_TOKENS);
    }

    #[test]
    fn test_client_configuration() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_model("claude-opus-4")
            .with_max_tokens(8192);

        assert_eq!(client.model_name(), "claude-opus-4");
        assert_eq!(client.max_tokens, 8192);
    }

    // Note: Integration tests with real API would require API key
    // and should be run separately with `cargo test --ignored`
}
