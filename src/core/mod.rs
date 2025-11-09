//! Core TUI domain
//!
//! Contains the fundamental TUI architecture following Elm pattern:
//! - Model (app.rs): Application state
//! - Message (event.rs): Events and messages
//! - Update (app.rs, app_event_handlers): State transitions
//! - View (ui.rs): Rendering logic

pub mod app;
pub mod app_event_handlers;
pub mod app_state;
pub mod event;
pub mod tui;
pub mod ui;

pub use app::App;
pub use app_state::{AppScreen, EvaluationState};
pub use event::{EvaluationProgress, Event, EventHandler};
pub use tui::Tui;
