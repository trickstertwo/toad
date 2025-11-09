//! TOAD - Terminal-Oriented Autonomous Developer
//!
//! Domain-Driven Architecture:
//! - core: TUI fundamentals (Elm Architecture: Model-Message-Update-View)
//! - ui: Widgets, themes, and visual components
//! - ai: Agent, LLM, evaluation, and tooling
//! - editor: Vim motions, undo/redo, multi-cursor
//! - workspace: Tabs, layouts, session management
//! - navigation: Search, fuzzy finding, bookmarks
//! - commands: Command mode, aliases, autocomplete
//! - performance: Lazy rendering, optimization
//! - infrastructure: Async, errors, I/O utilities
//! - config: Configuration management

// Domain modules
pub mod ai;
pub mod commands;
pub mod config;
pub mod core;
pub mod editor;
pub mod git;
pub mod infrastructure;
pub mod navigation;
pub mod performance;
pub mod ui;
pub mod workspace;

// Re-exports for convenience
pub use ai::{
    Agent, AnthropicClient, CompareArgs, ComparisonResult, EvalArgs, EvalCommand, EvaluationHandle,
    EvaluationHarness, EvaluationResults, LLMClient, Metrics, MetricsCollector, ShowConfigArgs,
    Task, TaskResult, ToolRegistry, parse_eval_command, start_comparison, start_evaluation,
};
pub use commands::{CommandMode, CommandRegistry};
pub use config::{
    AiConfig, Config, EditorConfig, FeatureFlags, SessionConfig, ToadConfig, UiConfig,
};
pub use core::{App, AppScreen, EvaluationProgress, EvaluationState, Event, EventHandler, Tui};
pub use editor::{Motion, MultiCursor, UndoStack, VimMotions};
pub use git::{BranchInfo, CommitInfo, FileChange, GitGraphService, GitService};
pub use infrastructure::{Clipboard, ErrorHandler, KeyBindings};
pub use navigation::{FuzzyFinder, SearchState};
pub use performance::PerformanceMetrics;
pub use ui::{HighlightTheme, HighlightedSpan, Language, SyntaxHighlighter, ToadTheme};
pub use workspace::{LayoutManager, SessionState, Tab, TabManager};

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
