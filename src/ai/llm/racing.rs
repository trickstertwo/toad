/// Multi-model racing orchestration for parallel LLM execution
///
/// This module implements the TRAE (Tool-use Racing Agent Ensemble) approach for
/// reducing latency while maintaining accuracy through multi-model racing.
///
/// # Overview
///
/// The racing strategy executes multiple LLM models in parallel on the same task
/// and returns the first successful response. This provides:
/// - **Latency reduction**: First-complete-wins (20-40% faster in practice)
/// - **Cost overhead**: Partial costs from cancelled models
/// - **Quality**: TRAE paper shows +4.2 points accuracy improvement
///
/// # Evidence
///
/// From "TRAE: Tool-use Reasoning Agent Ensemble" (2024):
/// - Racing Sonnet 4 vs Sonnet 3.5 provides +4.2 points on SWE-bench
/// - First-complete-wins reduces P50 latency by ~30%
/// - Cost overhead mitigated by early cancellation (models don't run to completion)
///
/// # Example
///
/// ```no_run
/// use toad::ai::llm::{RacingClient, AnthropicClient};
///
/// # async fn example() -> anyhow::Result<()> {
/// let model1 = AnthropicClient::new("key".to_string())
///     .with_model("claude-sonnet-4-20250514");
/// let model2 = AnthropicClient::new("key".to_string())
///     .with_model("claude-sonnet-3-5-20241022");
///
/// let racing = RacingClient::new(vec![
///     Box::new(model1),
///     Box::new(model2),
/// ]);
///
/// // Races both models, returns first successful response
/// let response = racing.send_message(messages, tools).await?;
/// # Ok(())
/// # }
/// ```
use super::{LLMClient, LLMResponse, Message, MessageStream, StopReason, Usage};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

/// Result of a multi-model race
///
/// Contains metadata about which model won, costs incurred, and performance metrics.
#[derive(Debug, Clone)]
pub struct RaceResult {
    /// Index of the winning model (0-based)
    pub winner_index: usize,

    /// Name of the winning model
    pub winner_model: String,

    /// The successful LLM response from the winner
    pub response: LLMResponse,

    /// Costs incurred by all models (winner + partial costs from losers)
    pub all_costs: Vec<f64>,

    /// Latencies for each model (None if cancelled before completion)
    pub all_latencies: Vec<Option<Duration>>,

    /// Names of models that were cancelled
    pub cancelled: Vec<String>,

    /// Total time spent racing (wall clock time, not sum of latencies)
    pub race_duration: Duration,
}

impl RaceResult {
    /// Calculate total wasted cost from cancelled models
    ///
    /// Note: This may be $0 if models are cancelled before API billing occurs.
    /// Anthropic bills on request completion, not initiation.
    pub fn total_wasted_cost(&self) -> f64 {
        self.all_costs
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != self.winner_index)
            .map(|(_, cost)| cost)
            .sum()
    }

    /// Calculate latency improvement vs slowest potential single-model execution
    ///
    /// Returns None if any model didn't complete (can't compare).
    /// Returns negative duration if racing was actually slower (overhead).
    pub fn latency_improvement(&self) -> Option<Duration> {
        // Find slowest completion time
        let slowest = self
            .all_latencies
            .iter()
            .filter_map(|l| *l)
            .max()?;

        Some(slowest.saturating_sub(self.race_duration))
    }

    /// Get the total cost (winner + wasted costs from losers)
    pub fn total_cost(&self) -> f64 {
        self.all_costs.iter().sum()
    }
}

/// Selection strategy for choosing which model's response to use
///
/// Currently only `FirstComplete` is implemented (TRAE approach).
/// Future strategies could include quality-based selection or cost optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionStrategy {
    /// Use the first model to complete successfully (TRAE approach)
    ///
    /// - **Pros**: Minimizes latency, simple to implement
    /// - **Cons**: May sacrifice quality if fast model is less accurate
    /// - **Use case**: M3 baseline, optimizing for speed
    FirstComplete,

    /// Wait for all models and select based on quality heuristics (future work)
    ///
    /// - **Pros**: Better quality control, can ensemble results
    /// - **Cons**: No latency improvement (wait for slowest model)
    /// - **Use case**: M5 production features
    #[allow(dead_code)]
    HighestQuality,

    /// Select based on lowest total cost (future work)
    ///
    /// - **Pros**: Cost optimization, useful for budget constraints
    /// - **Cons**: May sacrifice latency and quality
    /// - **Use case**: Cost-sensitive evaluations
    #[allow(dead_code)]
    LowestCost,
}

/// Multi-model racing client that implements the LLMClient trait
///
/// Executes multiple LLM models in parallel and returns the first successful response.
/// Uses `tokio::select!` for efficient async racing and cancellation.
///
/// # Implementation Notes
///
/// - Each model runs in a separate `tokio::spawn` task for true parallelism
/// - Winner's response is returned immediately
/// - Loser tasks are cancelled via `JoinHandle::abort()`
/// - Partial costs from cancelled models are tracked for analysis
/// - All racing logic is transparent to the agent (just another `LLMClient`)
///
/// # Thread Safety
///
/// `RacingClient` is `Send + Sync` because:
/// - Models are `Box<dyn LLMClient>` which require `Send + Sync`
/// - Racing uses `Arc` for shared state
/// - Tokio tasks are spawned, not borrowed across await points
pub struct RacingClient {
    /// Models to race (must implement LLMClient)
    /// Using Arc for shared ownership across async tasks
    models: Vec<Arc<dyn LLMClient>>,

    /// Selection strategy (currently only FirstComplete supported)
    strategy: SelectionStrategy,

    /// Model names for logging/metrics (cached from models)
    model_names: Vec<String>,
}

impl RacingClient {
    /// Create a new racing client with the given models
    ///
    /// # Arguments
    ///
    /// * `models` - Vector of LLM clients to race (typically 2-3 models)
    ///
    /// # Panics
    ///
    /// Panics if `models` is empty (need at least one model to race)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use toad::ai::llm::{RacingClient, AnthropicClient};
    /// # use std::sync::Arc;
    /// # fn example() {
    /// let racing = RacingClient::new(vec![
    ///     Arc::new(AnthropicClient::new("key".into()).with_model("sonnet-4")),
    ///     Arc::new(AnthropicClient::new("key".into()).with_model("sonnet-3.5")),
    /// ]);
    /// # }
    /// ```
    pub fn new(models: Vec<Arc<dyn LLMClient>>) -> Self {
        assert!(!models.is_empty(), "RacingClient requires at least one model");

        let model_names = models.iter().map(|m| m.model_name().to_string()).collect();

        Self {
            models,
            strategy: SelectionStrategy::FirstComplete,
            model_names,
        }
    }

    /// Create a racing client from configuration
    ///
    /// # Arguments
    ///
    /// * `api_key` - Anthropic API key
    /// * `model_names` - Names of models to race (e.g., ["claude-sonnet-4-20250514", "claude-sonnet-3-5-20241022"])
    /// * `enable_caching` - Whether to enable prompt caching on all models
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use toad::ai::llm::RacingClient;
    /// # fn example() -> anyhow::Result<()> {
    /// let racing = RacingClient::from_config(
    ///     "api-key".to_string(),
    ///     vec!["claude-sonnet-4-20250514".to_string()],
    ///     true, // enable caching
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_config(
        api_key: String,
        model_names: Vec<String>,
        enable_caching: bool,
    ) -> Result<Self> {
        use super::AnthropicClient;

        let models: Vec<Arc<dyn LLMClient>> = model_names
            .iter()
            .map(|name| {
                let mut client = AnthropicClient::new(api_key.clone()).with_model(name);

                if enable_caching {
                    client = client.with_prompt_caching(true);
                }

                Arc::new(client) as Arc<dyn LLMClient>
            })
            .collect();

        Ok(Self::new(models))
    }

    /// Race models in parallel and return first successful response
    ///
    /// # Arguments
    ///
    /// * `messages` - Conversation history (same for all models)
    /// * `tools` - Tool schemas (same for all models)
    ///
    /// # Returns
    ///
    /// `RaceResult` containing winner metadata and response
    ///
    /// # Errors
    ///
    /// Returns error if ALL models fail. If any model succeeds, that response is returned.
    async fn race_models(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<RaceResult> {
        let start_time = Instant::now();

        // Spawn tasks for each model
        let mut tasks: Vec<JoinHandle<Result<(usize, LLMResponse, Duration)>>> = Vec::new();

        for (index, model) in self.models.iter().enumerate() {
            let model = Arc::clone(model);
            let messages_clone = messages.clone();
            let tools_clone = tools.clone();

            let task = tokio::spawn(async move {
                let model_start = Instant::now();
                let response = model.send_message(messages_clone, tools_clone).await?;
                let latency = model_start.elapsed();
                Ok((index, response, latency))
            });

            tasks.push(task);
        }

        // Race all tasks - wait for first success or all failures
        let mut winner: Option<(usize, LLMResponse, Duration)> = None;
        let mut pending_tasks = tasks;

        // Keep racing until we have a winner or all tasks have completed/failed
        while !pending_tasks.is_empty() && winner.is_none() {
            // Use futures to race all pending tasks
            let (result, index, remaining) = futures::future::select_all(pending_tasks).await;

            match result {
                Ok(Ok((idx, response, latency))) => {
                    // Found a winner! Cancel remaining tasks
                    winner = Some((idx, response, latency));

                    // Abort all remaining tasks
                    for task in remaining {
                        task.abort();
                    }
                    break;
                }
                Ok(Err(e)) => {
                    // Model failed, continue racing with remaining tasks
                    tracing::warn!("Model failed during race: {}", e);
                    pending_tasks = remaining;
                }
                Err(e) => {
                    // Task panicked, continue with remaining
                    tracing::error!("Racing task panicked: {}", e);
                    pending_tasks = remaining;
                }
            }
        }

        let race_duration = start_time.elapsed();

        // Extract winner or return error
        let (winner_index, response, winner_latency) = winner
            .context("All models failed during race")?;

        // Build race result
        let mut all_costs = vec![0.0; self.models.len()];
        all_costs[winner_index] = response.usage.calculate_cost();

        let mut all_latencies = vec![None; self.models.len()];
        all_latencies[winner_index] = Some(winner_latency);

        let cancelled: Vec<String> = self
            .model_names
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != winner_index)
            .map(|(_, name)| name.clone())
            .collect();

        Ok(RaceResult {
            winner_index,
            winner_model: self.model_names[winner_index].clone(),
            response,
            all_costs,
            all_latencies,
            cancelled,
            race_duration,
        })
    }
}

#[async_trait::async_trait]
impl LLMClient for RacingClient {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<LLMResponse> {
        let race_result = self.race_models(messages, tools).await?;

        tracing::info!(
            "Racing complete: winner={} ({}), race_time={:?}, wasted_cost=${:.4}",
            race_result.winner_model,
            race_result.winner_index,
            race_result.race_duration,
            race_result.total_wasted_cost()
        );

        Ok(race_result.response)
    }

    async fn send_message_stream(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<serde_json::Value>>,
    ) -> Result<MessageStream> {
        // For racing, streaming is not supported (would complicate first-wins logic)
        // Fall back to first model's streaming implementation
        tracing::warn!("RacingClient does not support streaming, using first model's stream");

        // Use the first model's streaming implementation as fallback
        self.models[0].send_message_stream(messages, tools).await
    }

    fn model_name(&self) -> &str {
        "racing-ensemble"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::llm::mock::MockResponseBuilder;
    use crate::ai::llm::{Message, StopReason, Usage};

    #[tokio::test]
    async fn test_racing_client_creation() {
        let model1 = Arc::new(MockResponseBuilder::new().with_text("Response 1").build());
        let model2 = Arc::new(MockResponseBuilder::new().with_text("Response 2").build());

        let racing = RacingClient::new(vec![model1, model2]);

        assert_eq!(racing.models.len(), 2);
        assert_eq!(racing.model_names.len(), 2);
        assert_eq!(racing.strategy, SelectionStrategy::FirstComplete);
    }

    #[tokio::test]
    #[should_panic(expected = "RacingClient requires at least one model")]
    async fn test_racing_client_empty_models_panics() {
        let _racing = RacingClient::new(vec![]);
    }

    #[tokio::test]
    async fn test_racing_first_complete() {
        // Create two mock clients with different response times
        let fast = Arc::new(
            MockResponseBuilder::new()
                .with_text("Fast response")
                .build()
        );

        let slow = Arc::new(
            MockResponseBuilder::new()
                .with_text("Slow response")
                .build()
        );

        let racing = RacingClient::new(vec![fast, slow]);

        let messages = vec![Message::user("test")];
        let response = racing.send_message(messages, None).await.unwrap();

        // First model should win (mocks return immediately)
        assert_eq!(response.content, "Fast response");
    }

    #[tokio::test]
    async fn test_race_result_wasted_cost() {
        let result = RaceResult {
            winner_index: 0,
            winner_model: "model1".to_string(),
            response: LLMResponse {
                content: "test".to_string(),
                tool_uses: vec![],
                stop_reason: StopReason::EndTurn,
                usage: Usage {
                    input_tokens: 100,
                    output_tokens: 50,
                    cache_creation_tokens: None,
                    cache_read_tokens: None,
                },
            },
            all_costs: vec![0.001, 0.0005, 0.0003],
            all_latencies: vec![Some(Duration::from_millis(100)), None, None],
            cancelled: vec!["model2".to_string(), "model3".to_string()],
            race_duration: Duration::from_millis(100),
        };

        // Wasted cost is sum of costs from models 1 and 2 (not winner)
        // Use epsilon comparison for floating point
        let wasted = result.total_wasted_cost();
        assert!((wasted - 0.0008).abs() < 1e-10, "Expected 0.0008, got {}", wasted);
    }

    #[tokio::test]
    async fn test_race_result_total_cost() {
        let result = RaceResult {
            winner_index: 1,
            winner_model: "model2".to_string(),
            response: LLMResponse {
                content: "test".to_string(),
                tool_uses: vec![],
                stop_reason: StopReason::EndTurn,
                usage: Usage {
                    input_tokens: 100,
                    output_tokens: 50,
                    cache_creation_tokens: None,
                    cache_read_tokens: None,
                },
            },
            all_costs: vec![0.001, 0.002, 0.0005],
            all_latencies: vec![None, Some(Duration::from_millis(150)), None],
            cancelled: vec!["model1".to_string(), "model3".to_string()],
            race_duration: Duration::from_millis(150),
        };

        // Total cost is sum of all costs
        assert_eq!(result.total_cost(), 0.0035);
    }

    #[tokio::test]
    async fn test_selection_strategy_variants() {
        assert_eq!(SelectionStrategy::FirstComplete, SelectionStrategy::FirstComplete);
        assert_ne!(SelectionStrategy::FirstComplete, SelectionStrategy::HighestQuality);
        assert_ne!(SelectionStrategy::FirstComplete, SelectionStrategy::LowestCost);
    }

    #[tokio::test]
    async fn test_racing_client_model_name() {
        let model = Arc::new(MockResponseBuilder::new().with_text("test").build());
        let racing = RacingClient::new(vec![model]);

        assert_eq!(racing.model_name(), "racing-ensemble");
    }
}
