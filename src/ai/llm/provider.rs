/// Provider configuration and factory for multi-LLM support
///
/// Supports Anthropic, OpenAI, Ollama (local), and GitHub Models
use super::{AnthropicClient, GitHubClient, LLMClient, OllamaClient};
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::env;

/// LLM provider type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Anthropic,
    GitHub,
    Ollama,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Which provider to use
    pub provider: ProviderType,

    /// Model name
    pub model: String,

    /// API key (for Anthropic, GitHub)
    /// Can be set via environment variable or directly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Base URL (for Ollama, or custom endpoints)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Maximum tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    /// Temperature (0.0-1.0 or 0.0-2.0 depending on provider)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

fn default_max_tokens() -> u32 {
    4096
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider: ProviderType::Anthropic,
            model: "claude-sonnet-4-5-20250929".to_string(),
            api_key: None,
            base_url: None,
            max_tokens: 4096,
            temperature: None,
        }
    }
}

impl ProviderConfig {
    /// Create configuration for Anthropic
    pub fn anthropic(model: impl Into<String>) -> Self {
        Self {
            provider: ProviderType::Anthropic,
            model: model.into(),
            ..Default::default()
        }
    }

    /// Create configuration for GitHub Models
    pub fn github(model: impl Into<String>) -> Self {
        Self {
            provider: ProviderType::GitHub,
            model: model.into(),
            ..Default::default()
        }
    }

    /// Create configuration for Ollama (local)
    pub fn ollama(model: impl Into<String>) -> Self {
        Self {
            provider: ProviderType::Ollama,
            model: model.into(),
            base_url: Some("http://localhost:11434".to_string()),
            ..Default::default()
        }
    }

    /// Set API key
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set base URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Get API key from config or environment
    fn resolve_api_key(&self, env_var: &str) -> Result<String> {
        if let Some(ref key) = self.api_key {
            return Ok(key.clone());
        }

        env::var(env_var).context(format!("{} environment variable not set", env_var))
    }
}

/// Provider factory for creating LLM clients
pub struct LLMProvider;

impl LLMProvider {
    /// Create an LLM client based on configuration
    pub fn create(config: &ProviderConfig) -> Result<Box<dyn LLMClient>> {
        match config.provider {
            ProviderType::Anthropic => {
                let api_key = config.resolve_api_key("ANTHROPIC_API_KEY")?;
                let mut client = AnthropicClient::new(api_key)
                    .with_model(&config.model)
                    .with_max_tokens(config.max_tokens);

                if let Some(temp) = config.temperature {
                    client = client.with_temperature(temp);
                }

                Ok(Box::new(client))
            }

            ProviderType::GitHub => {
                let api_key = config.resolve_api_key("GITHUB_TOKEN")?;
                let mut client = GitHubClient::new(api_key)
                    .with_model(&config.model)
                    .with_max_tokens(config.max_tokens);

                if let Some(temp) = config.temperature {
                    client = client.with_temperature(temp);
                }

                Ok(Box::new(client))
            }

            ProviderType::Ollama => {
                let base_url = config
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "http://localhost:11434".to_string());

                let mut client = OllamaClient::new(&config.model)
                    .with_base_url(base_url);

                if let Some(temp) = config.temperature {
                    client = client.with_temperature(temp);
                }

                if config.max_tokens > 0 {
                    client = client.with_num_predict(config.max_tokens);
                }

                Ok(Box::new(client))
            }
        }
    }

    /// Create with Anthropic (convenience method)
    pub fn anthropic(api_key: String, model: &str) -> Result<Box<dyn LLMClient>> {
        let config = ProviderConfig::anthropic(model).with_api_key(api_key);
        Self::create(&config)
    }

    /// Create with GitHub Models (convenience method)
    pub fn github(token: String, model: &str) -> Result<Box<dyn LLMClient>> {
        let config = ProviderConfig::github(model).with_api_key(token);
        Self::create(&config)
    }

    /// Create with Ollama (convenience method)
    pub fn ollama(model: &str) -> Result<Box<dyn LLMClient>> {
        let config = ProviderConfig::ollama(model);
        Self::create(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_config_defaults() {
        let config = ProviderConfig::default();
        assert_eq!(config.provider, ProviderType::Anthropic);
        assert_eq!(config.max_tokens, 4096);
    }

    #[test]
    fn test_anthropic_config() {
        let config = ProviderConfig::anthropic("claude-sonnet-4-5-20250929")
            .with_api_key("test-key")
            .with_temperature(0.7);

        assert_eq!(config.provider, ProviderType::Anthropic);
        assert_eq!(config.model, "claude-sonnet-4-5-20250929");
        assert_eq!(config.api_key.unwrap(), "test-key");
        assert_eq!(config.temperature.unwrap(), 0.7);
    }

    #[test]
    fn test_github_config() {
        let config = ProviderConfig::github("gpt-4o")
            .with_api_key("ghp_test123");

        assert_eq!(config.provider, ProviderType::GitHub);
        assert_eq!(config.model, "gpt-4o");
        assert_eq!(config.api_key.unwrap(), "ghp_test123");
    }

    #[test]
    fn test_ollama_config() {
        let config = ProviderConfig::ollama("llama2")
            .with_base_url("http://192.168.1.100:11434");

        assert_eq!(config.provider, ProviderType::Ollama);
        assert_eq!(config.model, "llama2");
        assert_eq!(config.base_url.unwrap(), "http://192.168.1.100:11434");
    }

    #[test]
    fn test_provider_type_serialization() {
        let json = serde_json::to_string(&ProviderType::Anthropic).unwrap();
        assert_eq!(json, "\"anthropic\"");

        let json = serde_json::to_string(&ProviderType::GitHub).unwrap();
        assert_eq!(json, "\"github\"");

        let json = serde_json::to_string(&ProviderType::Ollama).unwrap();
        assert_eq!(json, "\"ollama\"");
    }
}
