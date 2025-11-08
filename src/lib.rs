//! TOAD - Terminal-Oriented Autonomous Developer
//! 
//! This library provides both:
//! - M0: Infrastructure and Evaluation Framework
//! - TUI: Terminal user interface following the Elm Architecture
//!   - Model (app.rs): Application state
//!   - Message (event.rs): Events and messages
//!   - Update (app.rs): State transitions
//!   - View (ui.rs): Rendering logic

// M0 modules
pub mod evaluation;
pub mod metrics;
pub mod stats;
pub mod tools;
pub mod llm;
pub mod agent;

// TUI modules
pub mod advanced_search;
pub mod app;
pub mod async_ops;
pub mod autocomplete;
pub mod background_tasks;
pub mod clipboard;
pub mod event;
pub mod fuzzy;
pub mod history;
pub mod keybinds;
pub mod lazy_render;
pub mod layout;
pub mod logo;
pub mod mouse;
pub mod multicursor;
pub mod performance;
pub mod resizable;
pub mod search;
pub mod session;
pub mod tabs;
pub mod theme;
pub mod tui;
pub mod ui;
pub mod validation;
pub mod virtual_scroll;
pub mod widgets;

// Config module contains both M0 and TUI configs
pub mod config;

// M0 re-exports
pub use config::{FeatureFlags, ToadConfig};
pub use evaluation::{Task, TaskResult, EvaluationHarness};
pub use metrics::{Metrics, MetricsCollector};
pub use stats::{ComparisonResult, StatisticalTest};

// TUI re-exports
pub use advanced_search::{
    AdvancedSearchManager, AdvancedSearchMatch, FilterCondition, FilterHistory,
    FilterHistoryEntry, FilterOperator, MultiFieldFilter, SavedFilters,
};
pub use app::App;
pub use async_ops::{AsyncOperation, AsyncOperationManager, OperationId, OperationStatus};
pub use autocomplete::{AutocompleteManager, AutocompleteProvider, CommandProvider, Suggestion, WordProvider};
pub use background_tasks::{BackgroundTask, BackgroundTaskManager, TaskId, TaskStatus};
pub use clipboard::Clipboard;
pub use config::{Config, AiConfig, EditorConfig, UiConfig};
pub use event::Event;
pub use fuzzy::{CaseMode, FuzzyFinder, FuzzyMatch, FuzzyMatcher, MatchStrategy};
pub use history::History;
pub use keybinds::{KeyBinding, KeyBindings};
pub use lazy_render::{LazyRenderManager, LazyRenderState, LazyRenderable};
pub use layout::{LayoutManager, Pane, PanelId, SplitDirection};
pub use mouse::{ClickAction, MouseAction, MouseState, ScrollDirection};
pub use multicursor::{CursorPosition, MultiCursor};
pub use performance::{FrameLimiter, PerformanceMetrics, TargetFPS};
pub use resizable::{ResizablePane, ResizablePaneManager, ResizeDirection};
pub use search::{SearchMatch, SearchState};
pub use session::Session;
pub use tabs::{Tab, TabId, TabManager};
pub use tui::Tui;
pub use validation::{
    CompositeValidator, InputValidator, LengthValidator, NotEmptyValidator, RegexValidator,
    ValidationResult, Validator,
};
pub use virtual_scroll::VirtualScrollState;

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
