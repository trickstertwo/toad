/// LLM client errors with specific handling for different failure modes
///
/// This module provides comprehensive error types for:
/// - API errors (authentication, rate limits, validation)
/// - Network errors (timeouts, connectivity)
/// - Parse errors (invalid responses)
/// - Configuration errors (missing keys, invalid models)

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    /// API key is missing or invalid
    #[error("API key error: {0}")]
    ApiKey(String),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },

    /// Rate limit exceeded (429)
    #[error("Rate limit exceeded. Retry after {retry_after:?} seconds")]
    RateLimit { retry_after: Option<u64> },

    /// Request timed out
    #[error("Request timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    /// Network/connection error
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// Failed to parse API response
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Token limit exceeded
    #[error("Token limit exceeded: {used} tokens used, max is {max}")]
    TokenLimit { used: u32, max: u32 },

    /// Model not found or unsupported
    #[error("Model '{0}' not found or unsupported")]
    ModelNotFound(String),

    /// Other errors
    #[error("LLM error: {0}")]
    Other(#[from] anyhow::Error),
}

impl LLMError {
    /// Check if error is retryable (transient failure)
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            LLMError::RateLimit { .. }
                | LLMError::Timeout { .. }
                | LLMError::Network(_)
                | LLMError::ApiError { status: 500..=599, .. }
        )
    }

    /// Get suggested retry delay in seconds
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            LLMError::RateLimit { retry_after } => *retry_after,
            LLMError::Timeout { .. } => Some(2),
            LLMError::Network(_) => Some(1),
            LLMError::ApiError { status: 500..=599, .. } => Some(5),
            _ => None,
        }
    }

    /// Check if error is permanent (should not retry)
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            LLMError::ApiKey(_)
                | LLMError::Configuration(_)
                | LLMError::ModelNotFound(_)
                | LLMError::TokenLimit { .. }
                | LLMError::ApiError { status: 400..=499, .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_errors() {
        let rate_limit = LLMError::RateLimit { retry_after: Some(60) };
        assert!(rate_limit.is_retryable());
        assert!(!rate_limit.is_permanent());
        assert_eq!(rate_limit.retry_delay(), Some(60));

        let timeout = LLMError::Timeout { seconds: 30 };
        assert!(timeout.is_retryable());
        assert_eq!(timeout.retry_delay(), Some(2));

        let server_error = LLMError::ApiError {
            status: 503,
            message: "Service unavailable".to_string(),
        };
        assert!(server_error.is_retryable());
        assert_eq!(server_error.retry_delay(), Some(5));
    }

    #[test]
    fn test_permanent_errors() {
        let api_key = LLMError::ApiKey("Missing key".to_string());
        assert!(!api_key.is_retryable());
        assert!(api_key.is_permanent());
        assert_eq!(api_key.retry_delay(), None);

        let client_error = LLMError::ApiError {
            status: 400,
            message: "Bad request".to_string(),
        };
        assert!(!client_error.is_retryable());
        assert!(client_error.is_permanent());

        let token_limit = LLMError::TokenLimit { used: 100000, max: 50000 };
        assert!(!token_limit.is_retryable());
        assert!(token_limit.is_permanent());
    }

    #[test]
    fn test_error_messages() {
        let error = LLMError::ApiKey("test".to_string());
        assert!(error.to_string().contains("API key error"));

        let error = LLMError::Timeout { seconds: 30 };
        assert!(error.to_string().contains("30 seconds"));

        let error = LLMError::RateLimit { retry_after: Some(60) };
        assert!(error.to_string().contains("Rate limit"));
        assert!(error.to_string().contains("60"));
    }
}
