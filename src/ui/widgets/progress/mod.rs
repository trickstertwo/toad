//! Progress indicator widgets
//!
//! This module contains widgets for showing progress including progress bars,
//! spinners, token usage counters, and multi-step operation tracking.
//!
//! # Deprecation Notice
//!
//! The `ProgressBar` widget in this module is deprecated in favor of the atomic
//! `crate::ui::molecules::ProgressBar` for composable UIs.
//!
//! See `ATOMIC_DESIGN_MIGRATION.md` for migration guide.

pub mod multi_step;
pub mod progress;
pub mod spinner;
pub mod token_counter;

// Re-export all types for backwards compatibility
#[allow(deprecated)]
pub use progress::*;
pub use multi_step::{MultiStepProgress, Step, StepStatus};
pub use spinner::*;
pub use token_counter::*;
