//! Event handlers for different application screens
//!
//! This module contains keyboard event handlers organized by screen type,
//! following the Elm Architecture pattern (Update functions).
//!
//! # Module Structure
//!
//! - [`welcome`]: Welcome screen event handling
//! - [`trust_dialog`]: Trust confirmation dialog event handling
//! - [`main_screen`]: Main TUI interface event handling (largest handler)
//! - [`evaluation`]: Evaluation progress/results screen event handling
//!
//! # Architecture
//!
//! Each handler is implemented as a method on [`App`](crate::core::app::App) via `impl` blocks.
//! The main dispatcher [`App::handle_key_event`](crate::core::app::App::handle_key_event)
//! routes keyboard events to the appropriate screen handler based on current
//! [`AppScreen`](crate::core::app_state::AppScreen).
//!
//! # Event Flow
//!
//! ```text
//! KeyEvent → handle_key_event() → match screen {
//!     Welcome      → handle_welcome_key()
//!     TrustDialog  → handle_trust_dialog_key()
//!     Main         → handle_main_key()
//!     Evaluation   → handle_evaluation_key()
//! }
//! ```

mod evaluation;
mod main_screen;
mod trust_dialog;
mod welcome;
