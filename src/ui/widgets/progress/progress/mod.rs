//! Multi-stage progress indicator widget
//!
//! Displays progress through multiple stages with visual indicators.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::MultiStageProgress;
//!
//! let mut progress = MultiStageProgress::new();
//! progress.add_stage("Build", 10);
//! progress.add_stage("Test", 5);
//! ```

mod state;
mod render;
#[cfg(test)]
mod tests;

// Re-export all public types
pub use state::{MultiStageProgress, ProgressBar, StageStatus};
