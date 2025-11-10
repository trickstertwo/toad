/// Cascading router: Try cheap models first, escalate to expensive
///
/// Based on DavaJ research: 70% cost reduction with local-first approach
/// - Easy tasks: Ollama 7B/13B (~$0.01)
/// - Medium tasks: Ollama 32B (~$0.10-0.50)
/// - Hard tasks: Claude Sonnet/Opus ($1-5)
use super::{Difficulty, Router, TaskClassifier};
use crate::ai::evaluation::Task;
use crate::ai::llm::{ProviderConfig, ProviderType};
use anyhow::Result;

/// Model tier for cascading routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    /// Cheap local model: Ollama qwen2.5-coder:7b (~$0.01)
    Local7B,

    /// Better local model: Ollama qwen2.5-coder:32b (~$0.10-0.50)
    Local32B,

    /// Premium cloud model: Claude Sonnet 4 ($1-5)
    CloudPremium,

    /// Best cloud model: Claude Opus ($5-15)
    CloudBest,
}

impl ModelTier {
    /// Get estimated cost per task in USD
    pub fn estimated_cost_usd(&self) -> f64 {
        match self {
            ModelTier::Local7B => 0.0,      // Local = free
            ModelTier::Local32B => 0.0,     // Local = free
            ModelTier::CloudPremium => 2.0, // Anthropic Sonnet
            ModelTier::CloudBest => 10.0,   // Anthropic Opus
        }
    }

    /// Get model name
    pub fn model_name(&self) -> &str {
        match self {
            ModelTier::Local7B => "qwen2.5-coder:7b",
            ModelTier::Local32B => "qwen2.5-coder:32b",
            ModelTier::CloudPremium => "claude-sonnet-4-20250514",
            ModelTier::CloudBest => "claude-opus-4-20250514",
        }
    }
}

/// Cascading router implementation
pub struct CascadingRouter {
    classifier: TaskClassifier,
    /// Whether to use local models (requires Ollama)
    pub use_local: bool,
    /// API key for cloud fallback
    pub api_key: Option<String>,
}

impl CascadingRouter {
    /// Create new cascading router
    pub fn new() -> Self {
        Self {
            classifier: TaskClassifier::new(),
            use_local: true,
            api_key: None,
        }
    }

    /// Create router with API key for cloud fallback
    pub fn with_api_key(api_key: String) -> Self {
        Self {
            classifier: TaskClassifier::new(),
            use_local: true,
            api_key: Some(api_key),
        }
    }

    /// Create cloud-only router (no local models)
    pub fn cloud_only(api_key: String) -> Self {
        Self {
            classifier: TaskClassifier::new(),
            use_local: false,
            api_key: Some(api_key),
        }
    }

    /// Select model tier based on difficulty
    pub fn select_tier(&self, difficulty: Difficulty) -> ModelTier {
        if !self.use_local {
            // Cloud-only mode
            return match difficulty {
                Difficulty::Easy => ModelTier::CloudPremium,
                Difficulty::Medium => ModelTier::CloudPremium,
                Difficulty::Hard => ModelTier::CloudBest,
            };
        }

        // Local-first mode (DavaJ approach)
        match difficulty {
            Difficulty::Easy => ModelTier::Local7B,
            Difficulty::Medium => ModelTier::Local32B,
            Difficulty::Hard => {
                // Use cloud for hard tasks if available
                if self.api_key.is_some() {
                    ModelTier::CloudPremium
                } else {
                    ModelTier::Local32B
                }
            }
        }
    }

    /// Create provider config for a model tier
    pub fn tier_to_config(&self, tier: ModelTier) -> Result<ProviderConfig> {
        match tier {
            ModelTier::Local7B | ModelTier::Local32B => Ok(ProviderConfig {
                provider: ProviderType::Ollama,
                model: tier.model_name().to_string(),
                api_key: None,
                base_url: Some("http://localhost:11434".to_string()),
                max_tokens: 4096,
                temperature: Some(0.3),
            }),

            ModelTier::CloudPremium | ModelTier::CloudBest => {
                let api_key = self
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("API key required for cloud models"))?;

                Ok(ProviderConfig {
                    provider: ProviderType::Anthropic,
                    model: tier.model_name().to_string(),
                    api_key: Some(api_key),
                    base_url: None,
                    max_tokens: 8192,
                    temperature: Some(0.3),
                })
            }
        }
    }
}

impl Router for CascadingRouter {
    fn route(&self, task: &Task) -> Result<ProviderConfig> {
        // 1. Classify task difficulty
        let difficulty = self.classifier.classify(task)?;

        // 2. Select appropriate tier
        let tier = self.select_tier(difficulty);

        tracing::info!(
            "Cascading router: Task {} classified as {:?}, routing to {:?} (est. cost: ${:.2})",
            task.id,
            difficulty,
            tier,
            tier.estimated_cost_usd()
        );

        // 3. Create provider config
        self.tier_to_config(tier)
    }

    fn name(&self) -> &str {
        "cascading"
    }
}

impl Default for CascadingRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_first_easy_task() {
        let router = CascadingRouter::new();
        let tier = router.select_tier(Difficulty::Easy);
        assert_eq!(tier, ModelTier::Local7B);
        assert_eq!(tier.estimated_cost_usd(), 0.0);
    }

    #[test]
    fn test_local_first_medium_task() {
        let router = CascadingRouter::new();
        let tier = router.select_tier(Difficulty::Medium);
        assert_eq!(tier, ModelTier::Local32B);
        assert_eq!(tier.estimated_cost_usd(), 0.0);
    }

    #[test]
    fn test_local_first_hard_task_with_api_key() {
        let router = CascadingRouter::with_api_key("sk-test".to_string());
        let tier = router.select_tier(Difficulty::Hard);
        assert_eq!(tier, ModelTier::CloudPremium);
        assert!(tier.estimated_cost_usd() > 0.0);
    }

    #[test]
    fn test_local_first_hard_task_no_api_key() {
        let router = CascadingRouter::new();
        let tier = router.select_tier(Difficulty::Hard);
        assert_eq!(tier, ModelTier::Local32B); // Fallback to local
    }

    #[test]
    fn test_cloud_only_mode() {
        let router = CascadingRouter::cloud_only("sk-test".to_string());
        assert_eq!(
            router.select_tier(Difficulty::Easy),
            ModelTier::CloudPremium
        );
        assert_eq!(
            router.select_tier(Difficulty::Medium),
            ModelTier::CloudPremium
        );
        assert_eq!(router.select_tier(Difficulty::Hard), ModelTier::CloudBest);
    }

    #[test]
    fn test_tier_to_config_local() {
        let router = CascadingRouter::new();
        let config = router.tier_to_config(ModelTier::Local7B).unwrap();
        assert_eq!(config.provider, ProviderType::Ollama);
        assert_eq!(config.model, "qwen2.5-coder:7b");
    }

    #[test]
    fn test_tier_to_config_cloud_without_key() {
        let router = CascadingRouter::new();
        let result = router.tier_to_config(ModelTier::CloudPremium);
        assert!(result.is_err());
    }

    #[test]
    fn test_tier_to_config_cloud_with_key() {
        let router = CascadingRouter::with_api_key("sk-test".to_string());
        let config = router.tier_to_config(ModelTier::CloudPremium).unwrap();
        assert_eq!(config.provider, ProviderType::Anthropic);
        assert_eq!(config.model, "claude-sonnet-4-20250514");
    }

    #[test]
    fn test_route_integration() {
        let router = CascadingRouter::with_api_key("sk-test".to_string());
        let task = Task {
            id: "test-1".to_string(),
            problem_statement: "Fix typo in README.md".to_string(),
            ..Task::example()
        };

        let config = router.route(&task).unwrap();
        // Easy task should route to Local7B
        assert_eq!(config.provider, ProviderType::Ollama);
    }
}
