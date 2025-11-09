/// LLM client module for communicating with Claude Sonnet 4
///
/// This module provides:
/// - API client for Anthropic's Claude API
/// - Message formatting and parsing
/// - Token counting and cost tracking
/// - Tool use (function calling) support
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

pub mod anthropic;
pub mod errors;
pub mod github;
pub mod mock;
pub mod ollama;
pub mod provider;
pub mod rate_limiter;
pub mod streaming;

pub use anthropic::AnthropicClient;
pub use errors::LLMError;
pub use github::GitHubClient;
pub use mock::{DeterministicLLMClient, MockResponseBuilder, SequencedMockClient};
pub use ollama::OllamaClient;
pub use provider::{LLMProvider, ProviderConfig, ProviderType};
pub use rate_limiter::{RateLimitConfig, RateLimitStatus, RateLimiter};
pub use streaming::{MessageStream, StreamAccumulator, StreamEvent};

/// Represents a message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// Message role (user or assistant)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// Tool use request from the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    pub id: String,
    pub name: String,
    pub input: serde_json::Value,
}

/// LLM response containing text and/or tool uses
#[derive(Debug, Clone)]
pub struct LLMResponse {
    pub content: String,
    pub tool_uses: Vec<ToolUse>,
    pub stop_reason: StopReason,
    pub usage: Usage,
}

/// Why the LLM stopped generating
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_creation_tokens: Option<u32>,
    pub cache_read_tokens: Option<u32>,
}

impl Usage {
    pub fn total_tokens(&self) -> u32 {
        self.input_tokens + self.output_tokens
    }

    pub fn effective_tokens(&self) -> u32 {
        let cached = self.cache_read_tokens.unwrap_or(0);
        self.input_tokens - cached + self.output_tokens
    }

    /// Calculate cost in USD based on Claude Sonnet 4 pricing
    /// Input: $3/MTok, Output: $15/MTok
    /// Cache write: $3.75/MTok, Cache read: $0.30/MTok
    pub fn calculate_cost(&self) -> f64 {
        let input_cost = (self.input_tokens as f64 / 1_000_000.0) * 3.0;
        let output_cost = (self.output_tokens as f64 / 1_000_000.0) * 15.0;

        let cache_creation_cost = self
            .cache_creation_tokens
            .map(|t| (t as f64 / 1_000_000.0) * 3.75)
            .unwrap_or(0.0);

        let cache_read_cost = self
            .cache_read_tokens
            .map(|t| (t as f64 / 1_000_000.0) * 0.30)
            .unwrap_or(0.0);

        input_cost + output_cost + cache_creation_cost + cache_read_cost
    }
}

/// Trait for LLM clients
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    /// Send a message and get a response
    async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse>;

    /// Send a message and get a streaming response
    /// Returns a stream of events for real-time processing
    async fn send_message_stream(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<MessageStream>;

    /// Get the model name
    fn model_name(&self) -> &str;
}

/// Get API key from environment variable
pub fn get_api_key() -> Result<String> {
    env::var("ANTHROPIC_API_KEY").context("ANTHROPIC_API_KEY environment variable not set")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.role, Role::User);
        assert_eq!(user_msg.content, "Hello");

        let assistant_msg = Message::assistant("Hi");
        assert_eq!(assistant_msg.role, Role::Assistant);
        assert_eq!(assistant_msg.content, "Hi");
    }

    #[test]
    fn test_usage_calculations() {
        let usage = Usage {
            input_tokens: 1000,
            output_tokens: 500,
            cache_creation_tokens: Some(200),
            cache_read_tokens: Some(100),
        };

        assert_eq!(usage.total_tokens(), 1500);
        assert_eq!(usage.effective_tokens(), 1400); // 1000 - 100 + 500

        let cost = usage.calculate_cost();
        // Input: 1000/1M * $3 = $0.003
        // Output: 500/1M * $15 = $0.0075
        // Cache creation: 200/1M * $3.75 = $0.00075
        // Cache read: 100/1M * $0.30 = $0.00003
        // Total: $0.01128
        assert!((cost - 0.01128).abs() < 0.00001);
    }

    #[test]
    fn test_usage_no_cache() {
        let usage = Usage {
            input_tokens: 1000,
            output_tokens: 500,
            cache_creation_tokens: None,
            cache_read_tokens: None,
        };

        let cost = usage.calculate_cost();
        // Input: $0.003, Output: $0.0075
        // Total: $0.0105
        assert!((cost - 0.0105).abs() < 0.00001);
    }
}
