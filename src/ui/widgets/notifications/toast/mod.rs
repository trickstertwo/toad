//! Toast notification system
//!
//! Non-blocking notification messages that appear temporarily and auto-dismiss.
//! Toasts can be stacked and support different severity levels.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{Toast, ToastLevel};
//!
//! let toast = Toast::info("Operation completed successfully");
//! assert_eq!(toast.message(), "Operation completed successfully");
//! ```

mod state;
mod render;
#[cfg(test)]
mod tests;

// Re-export all public types
pub use state::{Toast, ToastLevel, ToastManager};
