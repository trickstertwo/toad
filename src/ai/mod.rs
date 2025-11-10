//! AI domain
//!
//! Contains all AI-related modules for agent execution, LLM integration,
//! evaluation frameworks, and tooling.

pub mod agent;
pub mod context;
pub mod eval_commands;
pub mod eval_runner;
pub mod evaluation;
pub mod llm;
pub mod metrics;
pub mod routing;
pub mod stats;
pub mod test_selection;
pub mod tools;

// Re-exports
pub use agent::{Agent, AgentResult, PromptBuilder};
pub use context::{
    AstCache, AstContext, AstParser, ContextBuilder, ExtractorRegistry, FileContext, Import,
    Language, Symbol, SymbolKind,
};
pub use eval_commands::{
    CompareArgs, EvalArgs, EvalCommand, ParseError as EvalParseError, ShowConfigArgs,
    parse_eval_command,
};
pub use eval_runner::{EvaluationHandle, start_comparison, start_evaluation};
pub use evaluation::{
    Complexity, DatasetManager, DatasetSource, EvaluationHarness, EvaluationResults,
    ExperimentManager, ExperimentStatus, Task, TaskLoader, TaskResult, task_loader,
};
pub use llm::{
    AnthropicClient, DeterministicLLMClient, LLMClient, LLMResponse, Message, MockResponseBuilder,
    SequencedMockClient, Usage,
};
pub use metrics::{AggregateMetrics, Metrics, MetricsCollector, QualityMetrics};
pub use routing::{CascadingRouter, Difficulty, ModelTier, Router, TaskClassifier};
pub use stats::{ComparisonResult, Recommendation, StatisticalTest};
pub use test_selection::{
    DependencyMapper, TestDiscovery, TestExecutionResult, TestExecutor, TestSelection, TestSelector,
};
pub use tools::{Tool, ToolRegistry, ToolResult};
