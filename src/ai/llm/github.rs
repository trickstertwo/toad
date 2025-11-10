/// GitHub Models API client
///
/// Implements the LLMClient trait for GitHub Models
/// API Documentation: https://docs.github.com/en/rest/models/inference
use super::{LLMClient, LLMResponse, Message, StopReason, ToolUse, Usage};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::json;

const API_BASE_URL: &str = "https://models.github.ai/inference";
const DEFAULT_MODEL: &str = "gpt-4o-mini";

/// GitHub Models API client
pub struct GitHubClient {
    access_token: String,
    model: String,
    http_client: reqwest::Client,

    // Optional parameters
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    top_p: Option<f32>,
}

impl GitHubClient {
    /// Create a new GitHub Models client with access token
    ///
    /// # Authentication
    /// You need a GitHub Personal Access Token (PAT) with `models:read` permission.
    /// Get one at: https://github.com/settings/tokens
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            model: DEFAULT_MODEL.to_string(),
            http_client: reqwest::Client::new(),
            temperature: None,
            max_tokens: None,
            top_p: None,
        }
    }

    /// Set the model to use
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set temperature (0.0-2.0)
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 2.0));
        self
    }

    /// Set maximum tokens to generate
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set nucleus sampling parameter
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p.clamp(0.0, 1.0));
        self
    }

    /// Convert our messages to OpenAI/GitHub format
    fn to_github_messages(messages: &[Message]) -> Vec<GitHubMessage> {
        messages
            .iter()
            .map(|msg| GitHubMessage {
                role: match msg.role {
                    super::Role::User => "user".to_string(),
                    super::Role::Assistant => "assistant".to_string(),
                },
                content: msg.content.clone(),
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl LLMClient for GitHubClient {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse> {
        // Build request body (OpenAI-compatible format)
        let mut body = json!({
            "model": self.model,
            "messages": Self::to_github_messages(&messages),
        });

        // Add optional parameters
        if let Some(temperature) = self.temperature {
            body["temperature"] = json!(temperature);
        }

        if let Some(max_tokens) = self.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        if let Some(top_p) = self.top_p {
            body["top_p"] = json!(top_p);
        }

        // Add tools if provided
        if let Some(tools_list) = tools
            && !tools_list.is_empty()
        {
            body["tools"] = json!(tools_list);
        }

        // Make API request
        let response = self
            .http_client
            .post(format!("{}/chat/completions", API_BASE_URL))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to GitHub Models API")?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .context("Failed to read response body")?;

        if !status.is_success() {
            return Err(anyhow!(
                "GitHub Models API error ({}): {}",
                status,
                response_text
            ));
        }

        // Parse response (OpenAI format)
        let api_response: GitHubResponse =
            serde_json::from_str(&response_text).context("Failed to parse GitHub response")?;

        let choice = api_response
            .choices
            .first()
            .ok_or_else(|| anyhow!("No choices in response"))?;

        // Extract tool uses if present
        let mut tool_uses = Vec::new();
        if let Some(ref tool_calls) = choice.message.tool_calls {
            for tool_call in tool_calls {
                tool_uses.push(ToolUse {
                    id: tool_call.id.clone(),
                    name: tool_call.function.name.clone(),
                    input: serde_json::from_str(&tool_call.function.arguments).unwrap_or(json!({})),
                });
            }
        }

        Ok(LLMResponse {
            content: choice.message.content.clone().unwrap_or_default(),
            tool_uses,
            stop_reason: match choice.finish_reason.as_str() {
                "stop" => StopReason::EndTurn,
                "length" => StopReason::MaxTokens,
                "tool_calls" => StopReason::ToolUse,
                _ => StopReason::EndTurn,
            },
            usage: Usage {
                input_tokens: api_response.usage.prompt_tokens,
                output_tokens: api_response.usage.completion_tokens,
                cache_creation_tokens: None,
                cache_read_tokens: None,
            },
        })
    }

    async fn send_message_stream(
        &self,
        _messages: Vec<Message>,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<super::streaming::MessageStream> {
        // TODO: Implement GitHub Models streaming
        Err(anyhow!("GitHub Models streaming not yet implemented"))
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

/// GitHub message format (OpenAI-compatible)
#[derive(Debug, Serialize, Deserialize)]
struct GitHubMessage {
    role: String,
    content: String,
}

/// GitHub API response (OpenAI-compatible)
#[derive(Debug, Deserialize)]
struct GitHubResponse {
    choices: Vec<GitHubChoice>,
    usage: GitHubUsage,
}

#[derive(Debug, Deserialize)]
struct GitHubChoice {
    message: GitHubResponseMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct GitHubResponseMessage {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<GitHubToolCall>>,
}

#[derive(Debug, Deserialize)]
struct GitHubToolCall {
    id: String,
    function: GitHubFunction,
}

#[derive(Debug, Deserialize)]
struct GitHubFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct GitHubUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GitHubClient::new("ghp_test123".to_string());
        assert_eq!(client.model_name(), DEFAULT_MODEL);
    }

    #[test]
    fn test_client_configuration() {
        let client = GitHubClient::new("ghp_test123".to_string())
            .with_model("gpt-4o")
            .with_temperature(0.7)
            .with_max_tokens(2048)
            .with_top_p(0.9);

        assert_eq!(client.model_name(), "gpt-4o");
        assert_eq!(client.temperature.unwrap(), 0.7);
        assert_eq!(client.max_tokens.unwrap(), 2048);
        assert_eq!(client.top_p.unwrap(), 0.9);
    }

    #[test]
    fn test_temperature_clamping() {
        let client = GitHubClient::new("ghp_test123".to_string()).with_temperature(3.0);
        assert_eq!(client.temperature.unwrap(), 2.0);

        let client = GitHubClient::new("ghp_test123".to_string()).with_temperature(-0.5);
        assert_eq!(client.temperature.unwrap(), 0.0);
    }

    #[test]
    fn test_message_conversion() {
        let messages = vec![Message::user("Hello"), Message::assistant("Hi there!")];

        let github_msgs = GitHubClient::to_github_messages(&messages);
        assert_eq!(github_msgs.len(), 2);
        assert_eq!(github_msgs[0].role, "user");
        assert_eq!(github_msgs[0].content, "Hello");
        assert_eq!(github_msgs[1].role, "assistant");
        assert_eq!(github_msgs[1].content, "Hi there!");
    }
}
