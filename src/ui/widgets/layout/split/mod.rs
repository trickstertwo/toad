//! Split pane widget for dividing screen space
//!
//! Provides horizontal and vertical pane splitting with configurable borders and sizes.

mod state;
#[cfg(test)]
mod tests;

// Re-export all public types
pub use state::{PaneBorderStyle, SplitDirection, SplitPane, SplitPaneError, SplitSize};
