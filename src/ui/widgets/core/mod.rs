//! Core foundational widgets
//!
//! Low-level UI primitives used across the application including collapsible sections,
//! contextual help, feature flags management, and undo/redo management.

pub mod animation;
pub mod approval_dialog;
pub mod borders;
pub mod breadcrumbs;
pub mod cheat_sheet;
pub mod collapsible;
pub mod context_display;
pub mod contextual_help;
pub mod dialog;
pub mod error_dialog;
pub mod feature_flags;
pub mod help;
pub mod icons;
pub mod preview;
pub mod scrollbar;
pub mod settings_screen;
pub mod statusline;
pub mod table;
pub mod theme_selector;
pub mod undo_redo;
pub mod vector_canvas;
pub mod welcome_screen;

pub use feature_flags::{
    FlagCategory, FlagEntry, FeatureFlagsPanel, Impact, Stability,
};
