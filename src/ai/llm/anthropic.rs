/// Anthropic API client for Claude
///
/// Implements the Messages API: https://docs.claude.com/en/api/messages
/// Supports all Anthropic API features including streaming, tool use, and advanced parameters
use super::{streaming::MessageStream, LLMClient, LLMResponse, Message, StopReason, ToolUse, Usage};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::json;

const API_BASE_URL: &str = "https://api.anthropic.com/v1";
const API_VERSION: &str = "2023-06-01";
const DEFAULT_MODEL: &str = "claude-sonnet-4-5-20250929";
const DEFAULT_MAX_TOKENS: u32 = 4096;

/// Tool choice strategy for controlling tool usage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    /// Let Claude decide whether to use tools
    Auto,
    /// Force Claude to use at least one tool
    Any,
    /// Force Claude to use a specific tool
    Tool { name: String },
}

/// Service tier for capacity allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    /// Automatically select best tier
    Auto,
    /// Use only standard tier
    StandardOnly,
}

/// Extended thinking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Thinking mode type
    #[serde(rename = "type")]
    pub mode: String,
    /// Budget in tokens (minimum 1024)
    pub budget_tokens: u32,
}

/// Request metadata for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// User ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Additional metadata fields
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Anthropic API client with full feature support
pub struct AnthropicClient {
    api_key: String,
    model: String,
    max_tokens: u32,
    http_client: reqwest::Client,

    // Optional parameters
    system: Option<String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop_sequences: Option<Vec<String>>,
    tool_choice: Option<ToolChoice>,
    metadata: Option<Metadata>,
    thinking: Option<ThinkingConfig>,
    service_tier: Option<ServiceTier>,
    beta_features: Vec<String>,
    prompt_caching_enabled: bool,
}

impl AnthropicClient {
    /// Create a new Anthropic client with API key
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: DEFAULT_MODEL.to_string(),
            max_tokens: DEFAULT_MAX_TOKENS,
            http_client: reqwest::Client::new(),
            system: None,
            temperature: None,
            top_p: None,
            top_k: None,
            stop_sequences: None,
            tool_choice: None,
            metadata: None,
            thinking: None,
            service_tier: None,
            beta_features: Vec::new(),
            prompt_caching_enabled: false,
        }
    }

    /// Set the model to use (e.g., "claude-sonnet-4-5-20250929")
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set maximum output tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Set system prompt (context and instructions)
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set temperature (0.0-1.0, default 1.0)
    /// Lower values are more analytical, higher values more creative
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature.clamp(0.0, 1.0));
        self
    }

    /// Set nucleus sampling parameter (0.0-1.0)
    /// Use alternatively with temperature, not together
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p.clamp(0.0, 1.0));
        self
    }

    /// Set top-k sampling (restricts to top K options)
    /// For advanced use cases only
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set custom stop sequences
    pub fn with_stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(sequences);
        self
    }

    /// Set tool choice strategy
    pub fn with_tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Set request metadata for tracking
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Enable extended thinking with token budget (minimum 1024)
    pub fn with_thinking(mut self, budget_tokens: u32) -> Self {
        self.thinking = Some(ThinkingConfig {
            mode: "enabled".to_string(),
            budget_tokens: budget_tokens.max(1024),
        });
        self
    }

    /// Set service tier for capacity allocation
    pub fn with_service_tier(mut self, tier: ServiceTier) -> Self {
        self.service_tier = Some(tier);
        self
    }

    /// Add a beta feature
    pub fn with_beta_feature(mut self, feature: impl Into<String>) -> Self {
        self.beta_features.push(feature.into());
        self
    }

    /// Add multiple beta features
    pub fn with_beta_features(mut self, features: Vec<String>) -> Self {
        self.beta_features.extend(features);
        self
    }

    /// Enable prompt caching (90% cost reduction)
    ///
    /// When enabled, adds cache breakpoints to system and tool messages
    /// to reuse computation across requests. Requires beta header.
    pub fn with_prompt_caching(mut self, enabled: bool) -> Self {
        self.prompt_caching_enabled = enabled;
        if enabled && !self.beta_features.contains(&"prompt-caching-2024-07-31".to_string()) {
            self.beta_features.push("prompt-caching-2024-07-31".to_string());
        }
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
        // Build request body with all parameters
        let mut body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": messages,
        });

        // Add optional parameters if set
        if let Some(ref system) = self.system {
            // Use cache-friendly system format when caching enabled
            if self.prompt_caching_enabled {
                body["system"] = json!([{
                    "type": "text",
                    "text": system,
                    "cache_control": {"type": "ephemeral"}
                }]);
            } else {
                body["system"] = json!(system);
            }
        }

        if let Some(temperature) = self.temperature {
            body["temperature"] = json!(temperature);
        }

        if let Some(top_p) = self.top_p {
            body["top_p"] = json!(top_p);
        }

        if let Some(top_k) = self.top_k {
            body["top_k"] = json!(top_k);
        }

        if let Some(ref stop_sequences) = self.stop_sequences {
            body["stop_sequences"] = json!(stop_sequences);
        }

        if let Some(ref tool_choice) = self.tool_choice {
            body["tool_choice"] = json!(tool_choice);
        }

        if let Some(ref metadata) = self.metadata {
            body["metadata"] = json!(metadata);
        }

        if let Some(ref thinking) = self.thinking {
            body["thinking"] = json!(thinking);
        }

        if let Some(ref service_tier) = self.service_tier {
            body["service_tier"] = json!(service_tier);
        }

        // Add tools if provided
        if let Some(mut tools_list) = tools {
            if !tools_list.is_empty() {
                // Add cache_control to last tool when caching enabled
                if self.prompt_caching_enabled {
                    if let Some(last_tool) = tools_list.last_mut() {
                        if let Some(obj) = last_tool.as_object_mut() {
                            obj.insert("cache_control".to_string(), json!({"type": "ephemeral"}));
                        }
                    }
                }
                body["tools"] = json!(tools_list);
            }
        }

        // Build request with headers
        let mut request = self
            .http_client
            .post(format!("{}/messages", API_BASE_URL))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .header("content-type", "application/json");

        // Add beta features header if any
        if !self.beta_features.is_empty() {
            request = request.header("anthropic-beta", self.beta_features.join(","));
        }

        // Send request
        let response = request
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
            // Try to parse Anthropic error response
            if let Ok(error_response) = serde_json::from_str::<AnthropicErrorResponse>(&response_text) {
                return Err(anyhow!(
                    "Anthropic API error ({}): {} - {}",
                    status,
                    error_response.error.error_type,
                    error_response.error.message
                ));
            }

            // Fallback to generic error
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
                "thinking" => {
                    // Extended thinking content - could be exposed separately
                    if let Some(_thinking_text) = content_block.text {
                        // For now, we skip thinking content
                        // Could add to a separate field in LLMResponse if needed
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

    async fn send_message_stream(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<MessageStream> {
        // Build request body with all parameters (same as send_message)
        let mut body = json!({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": messages,
            "stream": true,  // Enable streaming
        });

        // Add optional parameters if set
        if let Some(ref system) = self.system {
            // Use cache-friendly system format when caching enabled
            if self.prompt_caching_enabled {
                body["system"] = json!([{
                    "type": "text",
                    "text": system,
                    "cache_control": {"type": "ephemeral"}
                }]);
            } else {
                body["system"] = json!(system);
            }
        }

        if let Some(temperature) = self.temperature {
            body["temperature"] = json!(temperature);
        }

        if let Some(top_p) = self.top_p {
            body["top_p"] = json!(top_p);
        }

        if let Some(top_k) = self.top_k {
            body["top_k"] = json!(top_k);
        }

        if let Some(ref stop_sequences) = self.stop_sequences {
            body["stop_sequences"] = json!(stop_sequences);
        }

        if let Some(ref tool_choice) = self.tool_choice {
            body["tool_choice"] = json!(tool_choice);
        }

        if let Some(ref metadata) = self.metadata {
            body["metadata"] = json!(metadata);
        }

        if let Some(ref thinking) = self.thinking {
            body["thinking"] = json!(thinking);
        }

        if let Some(ref service_tier) = self.service_tier {
            body["service_tier"] = json!(service_tier);
        }

        // Add tools if provided
        if let Some(mut tools_list) = tools {
            if !tools_list.is_empty() {
                // Add cache_control to last tool when caching enabled
                if self.prompt_caching_enabled {
                    if let Some(last_tool) = tools_list.last_mut() {
                        if let Some(obj) = last_tool.as_object_mut() {
                            obj.insert("cache_control".to_string(), json!({"type": "ephemeral"}));
                        }
                    }
                }
                body["tools"] = json!(tools_list);
            }
        }

        // Build request with headers
        let mut request = self
            .http_client
            .post(format!("{}/messages", API_BASE_URL))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .header("content-type", "application/json");

        // Add beta features header if any
        if !self.beta_features.is_empty() {
            request = request.header("anthropic-beta", self.beta_features.join(","));
        }

        // Send request
        let response = request
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Anthropic API")?;

        // Check status before creating stream
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error response".to_string());

            // Try to parse Anthropic error response
            if let Ok(error_response) = serde_json::from_str::<AnthropicErrorResponse>(&error_text) {
                return Err(anyhow!(
                    "Anthropic API error ({}): {} - {}",
                    status,
                    error_response.error.error_type,
                    error_response.error.message
                ));
            }

            return Err(anyhow!(
                "Anthropic API error ({}): {}",
                status,
                error_text
            ));
        }

        // Create streaming response
        Ok(MessageStream::new(response))
    }
}

/// Anthropic error response structure
#[derive(Debug, Deserialize)]
struct AnthropicErrorResponse {
    #[serde(rename = "type")]
    response_type: String,
    error: AnthropicError,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
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
    // For text and thinking blocks
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
        assert!(client.system.is_none());
        assert!(client.temperature.is_none());
    }

    #[test]
    fn test_client_configuration() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_model("claude-opus-4-1-20250805")
            .with_max_tokens(8192);

        assert_eq!(client.model_name(), "claude-opus-4-1-20250805");
        assert_eq!(client.max_tokens, 8192);
    }

    #[test]
    fn test_system_prompt() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_system("You are a helpful coding assistant");

        assert_eq!(
            client.system.as_ref().unwrap(),
            "You are a helpful coding assistant"
        );
    }

    #[test]
    fn test_temperature_clamping() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_temperature(1.5); // Should clamp to 1.0

        assert_eq!(client.temperature.unwrap(), 1.0);

        let client = AnthropicClient::new("test-key".to_string())
            .with_temperature(-0.5); // Should clamp to 0.0

        assert_eq!(client.temperature.unwrap(), 0.0);
    }

    #[test]
    fn test_sampling_parameters() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_temperature(0.7)
            .with_top_p(0.9)
            .with_top_k(40);

        assert_eq!(client.temperature.unwrap(), 0.7);
        assert_eq!(client.top_p.unwrap(), 0.9);
        assert_eq!(client.top_k.unwrap(), 40);
    }

    #[test]
    fn test_stop_sequences() {
        let sequences = vec!["END".to_string(), "STOP".to_string()];
        let client = AnthropicClient::new("test-key".to_string())
            .with_stop_sequences(sequences.clone());

        assert_eq!(client.stop_sequences.as_ref().unwrap(), &sequences);
    }

    #[test]
    fn test_tool_choice() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_tool_choice(ToolChoice::Auto);

        assert!(matches!(client.tool_choice, Some(ToolChoice::Auto)));

        let client = AnthropicClient::new("test-key".to_string())
            .with_tool_choice(ToolChoice::Tool {
                name: "read_file".to_string(),
            });

        if let Some(ToolChoice::Tool { name }) = client.tool_choice {
            assert_eq!(name, "read_file");
        } else {
            panic!("Expected ToolChoice::Tool");
        }
    }

    #[test]
    fn test_thinking_config() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_thinking(2048);

        let thinking = client.thinking.unwrap();
        assert_eq!(thinking.mode, "enabled");
        assert_eq!(thinking.budget_tokens, 2048);

        // Test minimum budget enforcement
        let client = AnthropicClient::new("test-key".to_string())
            .with_thinking(512); // Should enforce minimum 1024

        assert_eq!(client.thinking.unwrap().budget_tokens, 1024);
    }

    #[test]
    fn test_service_tier() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_service_tier(ServiceTier::Auto);

        assert!(matches!(client.service_tier, Some(ServiceTier::Auto)));
    }

    #[test]
    fn test_beta_features() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_beta_feature("prompt-caching-2024-07-31")
            .with_beta_feature("extended-thinking-2024-12-12");

        assert_eq!(client.beta_features.len(), 2);
        assert!(client.beta_features.contains(&"prompt-caching-2024-07-31".to_string()));
    }

    #[test]
    fn test_metadata() {
        let mut extra = serde_json::Map::new();
        extra.insert("session_id".to_string(), json!("abc123"));

        let metadata = Metadata {
            user_id: Some("user123".to_string()),
            extra,
        };

        let client = AnthropicClient::new("test-key".to_string())
            .with_metadata(metadata);

        let meta = client.metadata.as_ref().unwrap();
        assert_eq!(meta.user_id.as_ref().unwrap(), "user123");
        assert_eq!(meta.extra.get("session_id").unwrap(), "abc123");
    }

    #[test]
    fn test_tool_choice_serialization() {
        // Test Auto
        let auto = ToolChoice::Auto;
        let json = serde_json::to_value(&auto).unwrap();
        assert_eq!(json["type"], "auto");

        // Test Any
        let any = ToolChoice::Any;
        let json = serde_json::to_value(&any).unwrap();
        assert_eq!(json["type"], "any");

        // Test Tool
        let tool = ToolChoice::Tool {
            name: "calculator".to_string(),
        };
        let json = serde_json::to_value(&tool).unwrap();
        assert_eq!(json["type"], "tool");
        assert_eq!(json["name"], "calculator");
    }

    #[test]
    fn test_prompt_caching_disabled_by_default() {
        let client = AnthropicClient::new("test-key".to_string());
        assert!(!client.prompt_caching_enabled);
        assert!(!client.beta_features.contains(&"prompt-caching-2024-07-31".to_string()));
    }

    #[test]
    fn test_prompt_caching_enabled() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_prompt_caching(true);

        assert!(client.prompt_caching_enabled);
        assert!(client.beta_features.contains(&"prompt-caching-2024-07-31".to_string()));
    }

    #[test]
    fn test_prompt_caching_with_system() {
        let client = AnthropicClient::new("test-key".to_string())
            .with_system("You are helpful")
            .with_prompt_caching(true);

        assert_eq!(client.system.as_ref().unwrap(), "You are helpful");
        assert!(client.prompt_caching_enabled);
    }

    // Note: Integration tests with real API would require API key
    // and should be run separately with `cargo test --ignored`
}
