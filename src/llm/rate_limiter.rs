/// Rate limiting for Anthropic API
///
/// Handles rate limits for Claude Sonnet 4.x:
/// - 50 requests per minute (RPM)
/// - 30,000 input tokens per minute (ITPM)
/// - 8,000 output tokens per minute (OTPM)
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Rate limiter for Anthropic API
pub struct RateLimiter {
    state: Arc<Mutex<RateLimitState>>,
    config: RateLimitConfig,
}

struct RateLimitState {
    window_start: Instant,
    requests_in_window: u32,
    input_tokens_in_window: u32,
    output_tokens_in_window: u32,
}

#[derive(Clone)]
pub struct RateLimitConfig {
    pub max_requests_per_minute: u32,
    pub max_input_tokens_per_minute: u32,
    pub max_output_tokens_per_minute: u32,
    pub window_duration: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::claude_sonnet_4()
    }
}

impl RateLimitConfig {
    /// Rate limits for Claude Sonnet 4.x
    pub fn claude_sonnet_4() -> Self {
        Self {
            max_requests_per_minute: 50,
            max_input_tokens_per_minute: 30_000,
            max_output_tokens_per_minute: 8_000,
            window_duration: Duration::from_secs(60),
        }
    }

    /// Conservative limits (80% of max) for safety
    pub fn conservative() -> Self {
        let base = Self::claude_sonnet_4();
        Self {
            max_requests_per_minute: (base.max_requests_per_minute as f64 * 0.8) as u32,
            max_input_tokens_per_minute: (base.max_input_tokens_per_minute as f64 * 0.8) as u32,
            max_output_tokens_per_minute: (base.max_output_tokens_per_minute as f64 * 0.8) as u32,
            window_duration: base.window_duration,
        }
    }
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimitState {
                window_start: Instant::now(),
                requests_in_window: 0,
                input_tokens_in_window: 0,
                output_tokens_in_window: 0,
            })),
            config,
        }
    }

    /// Create rate limiter with default Claude Sonnet 4 limits
    pub fn with_default_limits() -> Self {
        Self::new(RateLimitConfig::default())
    }

    /// Create rate limiter with conservative limits (80% of max)
    pub fn conservative() -> Self {
        Self::new(RateLimitConfig::conservative())
    }

    /// Wait if necessary to stay within rate limits
    /// Returns estimated input/output tokens that will be used
    pub async fn acquire(&self, estimated_input_tokens: u32, estimated_output_tokens: u32) {
        loop {
            let mut state = self.state.lock().await;

            // Check if window has expired
            let elapsed = state.window_start.elapsed();
            if elapsed >= self.config.window_duration {
                // Reset window
                state.window_start = Instant::now();
                state.requests_in_window = 0;
                state.input_tokens_in_window = 0;
                state.output_tokens_in_window = 0;
            }

            // Check if we can make this request
            let can_proceed = state.requests_in_window < self.config.max_requests_per_minute
                && state.input_tokens_in_window + estimated_input_tokens
                    <= self.config.max_input_tokens_per_minute
                && state.output_tokens_in_window + estimated_output_tokens
                    <= self.config.max_output_tokens_per_minute;

            if can_proceed {
                // Reserve capacity
                state.requests_in_window += 1;
                state.input_tokens_in_window += estimated_input_tokens;
                state.output_tokens_in_window += estimated_output_tokens;
                return;
            }

            // Calculate how long to wait
            let time_remaining = self.config.window_duration - elapsed;
            drop(state); // Release lock before sleeping

            // Wait until window resets
            tracing::debug!(
                "Rate limit approaching, waiting {:?} for window reset",
                time_remaining
            );
            sleep(time_remaining).await;
        }
    }

    /// Record actual usage (call after request completes)
    pub async fn record_actual_usage(&self, input_tokens: u32, output_tokens: u32) {
        let mut state = self.state.lock().await;

        // Update actual usage (may differ from estimates)
        // Note: This is for tracking, the reservation already happened in acquire()
        // We don't adjust backward, only ensure we don't overshoot
        state.input_tokens_in_window = state.input_tokens_in_window.max(input_tokens);
        state.output_tokens_in_window = state.output_tokens_in_window.max(output_tokens);
    }

    /// Get current rate limit status
    pub async fn status(&self) -> RateLimitStatus {
        let state = self.state.lock().await;
        let elapsed = state.window_start.elapsed();

        RateLimitStatus {
            requests_used: state.requests_in_window,
            requests_limit: self.config.max_requests_per_minute,
            input_tokens_used: state.input_tokens_in_window,
            input_tokens_limit: self.config.max_input_tokens_per_minute,
            output_tokens_used: state.output_tokens_in_window,
            output_tokens_limit: self.config.max_output_tokens_per_minute,
            window_remaining: self.config.window_duration.saturating_sub(elapsed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    pub requests_used: u32,
    pub requests_limit: u32,
    pub input_tokens_used: u32,
    pub input_tokens_limit: u32,
    pub output_tokens_used: u32,
    pub output_tokens_limit: u32,
    pub window_remaining: Duration,
}

impl RateLimitStatus {
    pub fn requests_percentage(&self) -> f64 {
        (self.requests_used as f64 / self.requests_limit as f64) * 100.0
    }

    pub fn input_tokens_percentage(&self) -> f64 {
        (self.input_tokens_used as f64 / self.input_tokens_limit as f64) * 100.0
    }

    pub fn output_tokens_percentage(&self) -> f64 {
        (self.output_tokens_used as f64 / self.output_tokens_limit as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests_per_minute: 5,
            max_input_tokens_per_minute: 1000,
            max_output_tokens_per_minute: 500,
            window_duration: Duration::from_secs(1),
        });

        // Should allow first request immediately
        limiter.acquire(100, 50).await;
        let status = limiter.status().await;
        assert_eq!(status.requests_used, 1);
        assert_eq!(status.input_tokens_used, 100);
    }

    #[tokio::test]
    async fn test_rate_limiter_window_reset() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests_per_minute: 2,
            max_input_tokens_per_minute: 1000,
            max_output_tokens_per_minute: 500,
            window_duration: Duration::from_millis(100),
        });

        // Use up limit
        limiter.acquire(400, 200).await;
        limiter.acquire(400, 200).await;

        let status = limiter.status().await;
        assert_eq!(status.requests_used, 2);

        // Wait for window reset
        sleep(Duration::from_millis(150)).await;

        // Should allow new request after window reset
        limiter.acquire(400, 200).await;
        let status = limiter.status().await;
        assert_eq!(status.requests_used, 1);
    }

    #[test]
    fn test_conservative_config() {
        let config = RateLimitConfig::conservative();
        assert_eq!(config.max_requests_per_minute, 40); // 80% of 50
        assert_eq!(config.max_input_tokens_per_minute, 24_000); // 80% of 30K
        assert_eq!(config.max_output_tokens_per_minute, 6_400); // 80% of 8K
    }

    #[tokio::test]
    async fn test_rate_limit_status() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests_per_minute: 10,
            max_input_tokens_per_minute: 1000,
            max_output_tokens_per_minute: 500,
            window_duration: Duration::from_secs(60),
        });

        limiter.acquire(250, 125).await;

        let status = limiter.status().await;
        assert_eq!(status.requests_percentage(), 10.0); // 1/10
        assert_eq!(status.input_tokens_percentage(), 25.0); // 250/1000
        assert_eq!(status.output_tokens_percentage(), 25.0); // 125/500
    }
}
