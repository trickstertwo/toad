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
pub mod box_drawing;
pub mod clipboard;
pub mod command_mode;
pub mod diff;
pub mod event;
pub mod file_ops;
pub mod fuzzy;
pub mod history;
pub mod keybinds;
pub mod lazy_render;
pub mod layout;
pub mod logo;
pub mod macros;
pub mod marks;
pub mod mouse;
pub mod multicursor;
pub mod performance;
pub mod recent_files;
pub mod resizable;
pub mod search;
pub mod session;
pub mod tabs;
pub mod theme;
pub mod tui;
pub mod ui;
pub mod undo;
pub mod validation;
pub mod vim_motions;
pub mod virtual_scroll;
pub mod visual_selection;
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
pub use box_drawing::{BoxBuilder, BoxChars, BoxStyle};
pub use clipboard::Clipboard;
pub use command_mode::{Command, CommandMode, CommandRegistry, CommandHandler, CommandResult};
pub use diff::{
    ChunkHeader, DiffHunk, DiffLine, DiffLineType, DiffParser, DiffStats, FileDiff,
};
pub use config::{Config, AiConfig, EditorConfig, UiConfig};
pub use event::Event;
pub use fuzzy::{CaseMode, FuzzyFinder, FuzzyMatch, FuzzyMatcher, MatchStrategy};
pub use history::History;
pub use keybinds::{KeyBinding, KeyBindings};
pub use lazy_render::{LazyRenderManager, LazyRenderState, LazyRenderable};
pub use layout::{LayoutManager, Pane, PanelId, SplitDirection};
pub use marks::{Mark, MarksManager, MarkType};
pub use mouse::{ClickAction, MouseAction, MouseState, ScrollDirection};
pub use multicursor::{CursorPosition, MultiCursor};
pub use performance::{FrameLimiter, PerformanceMetrics, TargetFPS};
pub use resizable::{ResizablePane, ResizablePaneManager, ResizeDirection};
pub use search::{SearchMatch, SearchState};
pub use session::Session;
pub use tabs::{Tab, TabId, TabManager};
pub use tui::Tui;
pub use undo::{Action, UndoStack, HistoryNavigator, TextInsert, TextDelete};
pub use validation::{
    CompositeValidator, InputValidator, LengthValidator, NotEmptyValidator, RegexValidator,
    ValidationResult, Validator,
};
pub use vim_motions::{Motion, VimMotions};
pub use virtual_scroll::VirtualScrollState;
pub use visual_selection::{Position, SelectionMode, SelectionRange, VisualSelection};
pub use macros::{Macro, MacroAction, MacroManager};
pub use file_ops::{FileOps, FileOpResult};
pub use recent_files::{RecentFile, RecentFiles};

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
