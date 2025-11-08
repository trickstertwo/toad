// TOAD - Terminal-Oriented Autonomous Developer
// Milestone 0: Infrastructure and Evaluation Framework

pub mod config;
pub mod evaluation;
pub mod metrics;
pub mod stats;
pub mod tools;
pub mod llm;
pub mod agent;

// Re-exports for convenience
pub use config::{FeatureFlags, ToadConfig};
pub use evaluation::{Task, TaskResult, EvaluationHarness};
pub use metrics::{Metrics, MetricsCollector};
pub use stats::{ComparisonResult, StatisticalTest};

/// Current TOAD version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
//! Toad - AI-powered coding terminal with semi-autonomous agents
//!
//! This library provides the core TUI functionality following the Elm Architecture:
//! - Model (app.rs): Application state
//! - Message (event.rs): Events and messages
//! - Update (app.rs): State transitions
//! - View (ui.rs): Rendering logic

pub mod app;
pub mod clipboard;
pub mod config;
pub mod event;
pub mod history;
pub mod keybinds;
pub mod logo;
pub mod theme;
pub mod tui;
pub mod ui;
pub mod widgets;

// Re-export commonly used types
pub use app::App;
pub use clipboard::Clipboard;
pub use config::Config;
pub use event::Event;
pub use history::History;
pub use keybinds::{KeyBinding, KeyBindings};
pub use tui::Tui;

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
