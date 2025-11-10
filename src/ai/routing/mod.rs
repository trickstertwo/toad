/// LLM routing strategies for cost optimization and performance
///
/// This module implements multiple routing strategies:
/// - Cascading: Try cheap models first, escalate to expensive (DavaJ approach)
/// - Multi-model: Race multiple models in parallel (TRAE paper)
/// - Semantic: Route based on task type (Aurelio Labs)
/// - Speculative: Run fast+premium in parallel, use fast if good enough
mod cascade;
mod classifier;

pub use cascade::{CascadingRouter, ModelTier};
pub use classifier::{Difficulty, TaskClassifier};

use crate::ai::evaluation::Task;
use crate::ai::llm::ProviderConfig;
use anyhow::Result;

/// Router trait for selecting which LLM to use for a task
pub trait Router {
    /// Select the best LLM provider config for this task
    fn route(&self, task: &Task) -> Result<ProviderConfig>;

    /// Get the name of this routing strategy
    fn name(&self) -> &str;
}
