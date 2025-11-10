//! Floating window widget for overlay dialogs and popups
//!
//! Provides z-ordered floating windows with configurable positioning and styling.

mod state;
#[cfg(test)]
mod tests;

// Re-export all public types
pub use state::{FloatingWindow, FloatingWindowManager, WindowPosition};
