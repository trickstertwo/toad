/// Configuration module for TOAD
/// Contains both M0 (evaluation framework) and TUI configurations
mod tui;

pub use tui::{AiConfig, Config, EditorConfig, SessionConfig, UiConfig};

use crate::ai::llm::{ProviderConfig, ProviderType};
use serde::{Deserialize, Serialize};

/// Feature flags for experimental A/B testing
/// Each feature can be toggled on/off to measure its impact

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureFlags {
    // === Context Strategies ===
    /// Use AST-based context (tree-sitter parsing)
    /// Evidence: Aider proven, +2-5 points (cAST paper)
    pub context_ast: bool,

    /// Add vector embeddings for semantic search
    /// Evidence: Standard RAG technique, untested for combo
    pub context_embeddings: bool,

    /// Add code graph analysis (imports, calls, dependencies)
    /// Evidence: CodexGraph SIGKDD 2024, untested for combo
    pub context_graph: bool,

    /// Use re-ranking for retrieved context
    /// Evidence: Cohere Rerank improves precision, moderate cost
    pub context_reranking: bool,

    // === Routing Strategies ===
    /// Use semantic router for model selection
    /// Evidence: Aurelio Labs - 50x faster, 85-90% accuracy
    pub routing_semantic: bool,

    /// Use multi-model ensemble (race multiple models)
    /// Evidence: TRAE 75.2% vs Warp 71% = +4.2 points PROVEN
    pub routing_multi_model: bool,

    /// Use cascading routing (cheap → expensive)
    /// Evidence: DavaJ 84.7% HumanEval, 70% cost reduction
    /// Try local models first (Ollama 7B/32B), escalate to cloud if needed
    pub routing_cascade: bool,

    /// Use speculative execution (fast + premium in parallel)
    /// Evidence: Novel for LLMs, could save 24% cost if effective
    pub routing_speculative: bool,

    // === Intelligence Features ===
    /// Smart test selection using coverage + SBFL
    /// Evidence: AutoCodeRover proven, +3-5 points
    pub smart_test_selection: bool,

    /// Learn from past failures (persistent memory)
    /// Evidence: RL experience replay concept, untested for coding
    pub failure_memory: bool,

    /// Opportunistic planning (fast plan + execute + refine)
    /// Evidence: Anytime algorithms, uncertain for coding agents
    pub opportunistic_planning: bool,

    // === Optimization Features ===
    /// Use prompt caching (Anthropic/OpenAI)
    /// Evidence: 90% cost reduction PROVEN
    pub prompt_caching: bool,

    /// Use semantic caching (cache by semantic similarity)
    /// Evidence: GPTCache 68.8% API reduction, >97% accuracy
    pub semantic_caching: bool,

    /// Use tree-sitter for syntax validation
    /// Evidence: Production-proven, prevents syntax errors
    pub tree_sitter_validation: bool,
}

impl Default for FeatureFlags {
    /// Default configuration: Only proven features enabled
    fn default() -> Self {
        Self {
            // Context: Start with AST only (proven)
            context_ast: true,
            context_embeddings: false,
            context_graph: false,
            context_reranking: false,

            // Routing: Start simple (no routing)
            routing_semantic: false,
            routing_multi_model: false,
            routing_cascade: false,
            routing_speculative: false,

            // Intelligence: Start with proven features
            smart_test_selection: false, // Enable in M2
            failure_memory: false,
            opportunistic_planning: false,

            // Optimization: Enable proven optimizations
            prompt_caching: true,
            semantic_caching: false, // Test in M2
            tree_sitter_validation: true,
        }
    }
}

impl FeatureFlags {
    /// Minimal configuration: Only essentials
    pub fn minimal() -> Self {
        Self {
            context_ast: true,
            prompt_caching: true,
            tree_sitter_validation: true,
            ..Default::default()
        }
    }

    /// M1 baseline: Simple agent (55-60% target)
    pub fn milestone_1() -> Self {
        Self {
            context_ast: false, // Just basic context
            prompt_caching: true,
            tree_sitter_validation: true,
            ..Default::default()
        }
    }

    /// M2 enhanced: + AST + Smart tests (61-66% target)
    pub fn milestone_2() -> Self {
        Self {
            context_ast: true,
            smart_test_selection: true,
            prompt_caching: true,
            tree_sitter_validation: true,
            ..Default::default()
        }
    }

    /// M3 advanced: + Multi-model racing (63-68% target)
    pub fn milestone_3() -> Self {
        Self {
            context_ast: true,
            smart_test_selection: true,
            routing_multi_model: true,
            prompt_caching: true,
            tree_sitter_validation: true,
            ..Default::default()
        }
    }

    /// M4 cost-optimized: + Cascading routing (65-70% target, 70% cost reduction)
    pub fn milestone_4() -> Self {
        Self {
            context_ast: true,
            smart_test_selection: true,
            routing_multi_model: true,
            routing_cascade: true,
            context_embeddings: true,
            failure_memory: true,
            prompt_caching: true,
            tree_sitter_validation: true,
            ..Default::default()
        }
    }

    /// Count enabled features
    pub fn enabled_count(&self) -> usize {
        let mut count = 0;
        if self.context_ast {
            count += 1;
        }
        if self.context_embeddings {
            count += 1;
        }
        if self.context_graph {
            count += 1;
        }
        if self.context_reranking {
            count += 1;
        }
        if self.routing_semantic {
            count += 1;
        }
        if self.routing_multi_model {
            count += 1;
        }
        if self.routing_cascade {
            count += 1;
        }
        if self.routing_speculative {
            count += 1;
        }
        if self.smart_test_selection {
            count += 1;
        }
        if self.failure_memory {
            count += 1;
        }
        if self.opportunistic_planning {
            count += 1;
        }
        if self.prompt_caching {
            count += 1;
        }
        if self.semantic_caching {
            count += 1;
        }
        if self.tree_sitter_validation {
            count += 1;
        }
        count
    }

    /// Get a human-readable description
    pub fn description(&self) -> String {
        format!(
            "Context: AST={}, Embed={}, Graph={}, Rerank={} | \
             Routing: Semantic={}, Multi={}, Cascade={}, Spec={} | \
             Intel: Tests={}, Memory={}, Plan={} | \
             Opt: PCache={}, SCache={}, Validate={}",
            self.context_ast,
            self.context_embeddings,
            self.context_graph,
            self.context_reranking,
            self.routing_semantic,
            self.routing_multi_model,
            self.routing_cascade,
            self.routing_speculative,
            self.smart_test_selection,
            self.failure_memory,
            self.opportunistic_planning,
            self.prompt_caching,
            self.semantic_caching,
            self.tree_sitter_validation,
        )
    }
}

/// Main TOAD configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadConfig {
    /// Feature flags for A/B testing
    pub features: FeatureFlags,

    /// LLM provider configuration (Anthropic, GitHub, Ollama)
    pub provider: ProviderConfig,

    /// Maximum tokens for context
    pub max_context_tokens: usize,

    /// Timeout per task (seconds)
    pub task_timeout_secs: u64,

    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for ToadConfig {
    fn default() -> Self {
        Self {
            features: FeatureFlags::default(),
            provider: ProviderConfig::default(),
            max_context_tokens: 200_000,
            task_timeout_secs: 600, // 10 minutes
            verbose: false,
        }
    }
}

impl ToadConfig {
    /// Create a config for a specific milestone
    pub fn for_milestone(milestone: u8) -> Self {
        let features = match milestone {
            1 => FeatureFlags::milestone_1(),
            2 => FeatureFlags::milestone_2(),
            3 => FeatureFlags::milestone_3(),
            _ => FeatureFlags::default(),
        };

        Self {
            features,
            ..Default::default()
        }
    }

    /// Create a minimal config for testing
    pub fn minimal() -> Self {
        Self {
            features: FeatureFlags::minimal(),
            max_context_tokens: 50_000,
            task_timeout_secs: 300,
            ..Default::default()
        }
    }

    /// Set provider configuration
    pub fn with_provider(mut self, provider: ProviderConfig) -> Self {
        self.provider = provider;
        self
    }

    /// Use Anthropic provider
    pub fn with_anthropic(mut self, model: impl Into<String>) -> Self {
        self.provider = ProviderConfig::anthropic(model);
        self
    }

    /// Use GitHub Models provider
    pub fn with_github(mut self, model: impl Into<String>) -> Self {
        self.provider = ProviderConfig::github(model);
        self
    }

    /// Use Ollama (local) provider
    pub fn with_ollama(mut self, model: impl Into<String>) -> Self {
        self.provider = ProviderConfig::ollama(model);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_flags_default() {
        let flags = FeatureFlags::default();
        assert!(flags.context_ast);
        assert!(flags.prompt_caching);
        assert!(flags.tree_sitter_validation);
        assert!(!flags.routing_multi_model);
    }

    #[test]
    fn test_milestone_progression() {
        let m1 = FeatureFlags::milestone_1();
        let m2 = FeatureFlags::milestone_2();
        let m3 = FeatureFlags::milestone_3();

        // M1 is simplest
        assert!(m1.enabled_count() < m2.enabled_count());
        // M2 adds features
        assert!(m2.enabled_count() < m3.enabled_count());
        // M3 includes multi-model
        assert!(m3.routing_multi_model);
    }

    #[test]
    fn test_config_serialization() {
        let config = ToadConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ToadConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.provider.model, deserialized.provider.model);
    }

    #[test]
    fn test_config_with_providers() {
        let config = ToadConfig::default()
            .with_anthropic("claude-sonnet-4-5-20250929");
        assert_eq!(config.provider.provider, ProviderType::Anthropic);

        let config = ToadConfig::default()
            .with_github("gpt-4o");
        assert_eq!(config.provider.provider, ProviderType::GitHub);

        let config = ToadConfig::default()
            .with_ollama("llama2");
        assert_eq!(config.provider.provider, ProviderType::Ollama);
    }
}
