//! Core TUI domain
//!
//! Contains the fundamental TUI architecture following Elm pattern:
//! - Model (app.rs): Application state
//! - Message (event.rs): Events and messages
//! - Update (app.rs): State transitions
//! - View (ui.rs): Rendering logic

pub mod app;
pub mod event;
pub mod tui;
pub mod ui;

pub use app::{App, AppScreen, EvaluationState};
pub use event::{EvaluationProgress, Event, EventHandler};
pub use tui::Tui;
