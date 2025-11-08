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
pub mod aliases;
pub mod animations;
pub mod app;
pub mod async_ops;
pub mod autocomplete;
pub mod background_tasks;
pub mod batch_ops;
pub mod bookmarks;
pub mod box_drawing;
pub mod canvas;
pub mod clipboard;
pub mod command_mode;
pub mod config;
pub mod custom_keybindings;
pub mod diff;
pub mod errors;
pub mod event;
pub mod file_ops;
pub mod fuzzy;
pub mod history;
pub mod keybinds;
pub mod key_sequences;
pub mod lazy_render;
pub mod layout;
pub mod live_graphs;
pub mod logo;
pub mod macros;
pub mod marks;
pub mod mouse;
pub mod multicursor;
pub mod navigation;
pub mod nerd_fonts;
pub mod performance;
pub mod quick_actions;
pub mod recent_files;
pub mod resizable;
pub mod search;
pub mod session;
pub mod smart_suggestions;
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
pub mod workspaces;

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
pub use aliases::{Alias, AliasManager};
pub use app::App;
pub use async_ops::{AsyncOperation, AsyncOperationManager, OperationId, OperationStatus};
pub use autocomplete::{AutocompleteManager, AutocompleteProvider, CommandProvider, Suggestion, WordProvider};
pub use background_tasks::{BackgroundTask, BackgroundTaskManager, TaskId, TaskStatus};
pub use box_drawing::{BoxBuilder, BoxChars, BoxStyle};
pub use clipboard::Clipboard;
pub use command_mode::{Command, CommandMode, CommandRegistry, CommandHandler, CommandResult};
pub use config::{Config, AiConfig, EditorConfig, UiConfig};
pub use diff::{
    ChunkHeader, DiffHunk, DiffLine, DiffLineType, DiffParser, DiffStats, FileDiff,
};
pub use errors::{ErrorEntry, ErrorHandler, ErrorSeverity};
pub use event::Event;
pub use fuzzy::{CaseMode, FuzzyFinder, FuzzyMatch, FuzzyMatcher, MatchStrategy};
pub use history::History;
pub use keybinds::{KeyBinding, KeyBindings};
pub use key_sequences::{KeySequence, KeySequenceManager};
pub use custom_keybindings::{ContextualBinding, CustomKeybindings, KeybindingContext};
pub use lazy_render::{LazyRenderManager, LazyRenderState, LazyRenderable};
pub use live_graphs::{DataPoint, GraphType, LiveGraph, LiveGraphManager, UpdateFrequency};
pub use layout::{LayoutManager, Pane, PanelId, SplitDirection};
pub use marks::{Mark, MarksManager, MarkType};
pub use mouse::{ClickAction, MouseAction, MouseState, ScrollDirection};
pub use multicursor::{CursorPosition, MultiCursor};
pub use navigation::{NavigationAction, VimNavigation};
pub use performance::{FrameLimiter, PerformanceMetrics, TargetFPS};
pub use quick_actions::{ActionCategory, QuickAction, QuickActionManager};
pub use resizable::{ResizablePane, ResizablePaneManager, ResizeDirection};
pub use search::{SearchMatch, SearchState};
pub use session::SessionState;
pub use smart_suggestions::{ContextBuilder, SmartSuggestions, Suggestion as SmartSuggestion, SuggestionContext, SuggestionType};
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
pub use workspaces::{Workspace, WorkspaceManager};
pub use macros::{Macro, MacroAction, MacroManager};
pub use file_ops::{FileOps, FileOpResult};
pub use recent_files::{RecentFile, RecentFiles};
pub use bookmarks::{Bookmark, BookmarkManager};
pub use batch_ops::{BatchHandler, BatchManager, BatchOperation, BatchResult, BatchStats, OpResult};
pub use animations::{Animation, AnimationState, EasingFunction, TransitionManager};
pub use canvas::{Canvas, Pixel, Shape};
pub use nerd_fonts::{GitStatus, NerdFonts, UiIcon, supports_nerd_fonts};

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
