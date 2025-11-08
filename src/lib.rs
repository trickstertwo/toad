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
pub mod app;
pub mod clipboard;
pub mod event;
pub mod history;
pub mod keybinds;
pub mod layout;
pub mod logo;
pub mod performance;
pub mod search;
pub mod theme;
pub mod tui;
pub mod ui;
pub mod widgets;

// Config module contains both M0 and TUI configs
pub mod config;

// M0 re-exports
pub use config::{FeatureFlags, ToadConfig};
pub use evaluation::{Task, TaskResult, EvaluationHarness};
pub use metrics::{Metrics, MetricsCollector};
pub use stats::{ComparisonResult, StatisticalTest};

// TUI re-exports
pub use app::App;
pub use clipboard::Clipboard;
pub use config::{Config, AiConfig, EditorConfig, UiConfig};
pub use event::Event;
pub use history::History;
pub use keybinds::{KeyBinding, KeyBindings};
pub use layout::{LayoutManager, Pane, PanelId, SplitDirection};
pub use performance::PerformanceMetrics;
pub use search::{SearchMatch, SearchState};
pub use tui::Tui;

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
