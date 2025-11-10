/// Ollama local LLM client
///
/// Implements the LLMClient trait for Ollama local models
/// API Documentation: https://github.com/ollama/ollama/blob/main/docs/api.md
use super::{LLMClient, LLMResponse, Message, StopReason, Usage};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::json;

const DEFAULT_BASE_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "llama2";

/// Ollama API client for local models
pub struct OllamaClient {
    base_url: String,
    model: String,
    http_client: reqwest::Client,

    // Optional parameters
    temperature: Option<f32>,
    num_predict: Option<u32>, // Ollama's equivalent to max_tokens
}

impl OllamaClient {
    /// Create a new Ollama client
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            model: model.into(),
            http_client: reqwest::Client::new(),
            temperature: None,
            num_predict: None,
        }
    }

    /// Create with custom base URL (for remote Ollama instances)
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set temperature (0.0-1.0)
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Set max tokens to generate
    pub fn with_num_predict(mut self, num_predict: u32) -> Self {
        self.num_predict = Some(num_predict);
        self
    }

    /// Convert our messages to Ollama format
    fn to_ollama_messages(messages: &[Message]) -> Vec<OllamaMessage> {
        messages
            .iter()
            .map(|msg| OllamaMessage {
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
impl LLMClient for OllamaClient {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        _tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse> {
        // Build request body
        let mut body = json!({
            "model": self.model,
            "messages": Self::to_ollama_messages(&messages),
            "stream": false,
        });

        // Add optional parameters
        if let Some(temperature) = self.temperature {
            body["options"] = json!({
                "temperature": temperature,
            });
        }

        if let Some(num_predict) = self.num_predict {
            if let Some(options) = body.get_mut("options") {
                options["num_predict"] = json!(num_predict);
            } else {
                body["options"] = json!({
                    "num_predict": num_predict,
                });
            }
        }

        // Make API request
        let response = self
            .http_client
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Ollama API")?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .context("Failed to read response body")?;

        if !status.is_success() {
            return Err(anyhow!(
                "Ollama API error ({}): {}",
                status,
                response_text
            ));
        }

        // Parse response
        let api_response: OllamaResponse =
            serde_json::from_str(&response_text).context("Failed to parse Ollama response")?;

        Ok(LLMResponse {
            content: api_response.message.content,
            tool_uses: Vec::new(), // Ollama doesn't support tool use yet
            stop_reason: if api_response.done {
                StopReason::EndTurn
            } else {
                StopReason::MaxTokens
            },
            usage: Usage {
                input_tokens: api_response.prompt_eval_count.unwrap_or(0),
                output_tokens: api_response.eval_count.unwrap_or(0),
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
        // TODO: Implement Ollama streaming
        Err(anyhow!("Ollama streaming not yet implemented"))
    }

    fn model_name(&self) -> &str {
        &self.model
    }
}

/// Ollama message format
#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

/// Ollama API response
#[derive(Debug, Deserialize)]
struct OllamaResponse {
    model: String,
    message: OllamaMessage,
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = OllamaClient::new("llama2");
        assert_eq!(client.model_name(), "llama2");
        assert_eq!(client.base_url, DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_configuration() {
        let client = OllamaClient::new("codellama")
            .with_base_url("http://192.168.1.100:11434")
            .with_temperature(0.7)
            .with_num_predict(2048);

        assert_eq!(client.model_name(), "codellama");
        assert_eq!(client.base_url, "http://192.168.1.100:11434");
        assert_eq!(client.temperature.unwrap(), 0.7);
        assert_eq!(client.num_predict.unwrap(), 2048);
    }

    #[test]
    fn test_temperature_clamping() {
        let client = OllamaClient::new("llama2")
            .with_temperature(1.5);
        assert_eq!(client.temperature.unwrap(), 1.0);

        let client = OllamaClient::new("llama2")
            .with_temperature(-0.5);
        assert_eq!(client.temperature.unwrap(), 0.0);
    }

    #[test]
    fn test_message_conversion() {
        let messages = vec![
            Message::user("Hello"),
            Message::assistant("Hi there!"),
        ];

        let ollama_msgs = OllamaClient::to_ollama_messages(&messages);
        assert_eq!(ollama_msgs.len(), 2);
        assert_eq!(ollama_msgs[0].role, "user");
        assert_eq!(ollama_msgs[0].content, "Hello");
        assert_eq!(ollama_msgs[1].role, "assistant");
        assert_eq!(ollama_msgs[1].content, "Hi there!");
    }
}
