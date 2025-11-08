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
pub mod navigation;
pub mod search;
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
pub use navigation::{NavigationAction, VimNavigation};
pub use search::{Search, SearchDirection, SearchMode, SearchOptions};
pub use tui::Tui;

/// Result type alias for the application
pub type Result<T> = color_eyre::Result<T>;
