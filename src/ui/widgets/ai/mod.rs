//! AI-powered widgets
//!
//! This module contains widgets that leverage AI capabilities including
//! diff views for code changes and smart context-aware suggestions.

pub mod diff_view;
pub mod suggestions;

// Re-export all types for backwards compatibility
pub use diff_view::*;
pub use suggestions::*;
