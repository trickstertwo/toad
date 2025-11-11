//! Core foundational widgets
//!
//! Low-level UI primitives used across the application including collapsible sections,
//! contextual help, and undo/redo management.

pub mod animation;
pub mod borders;
pub mod breadcrumbs;
pub mod cheat_sheet;
pub mod collapsible;
pub mod context_display;
pub mod contextual_help;
pub mod dialog;
pub mod help;
pub mod icons;
pub mod preview;
pub mod scrollbar;
pub mod statusline;
pub mod table;
pub mod undo_redo;
pub mod vector_canvas;
pub mod welcome_screen;

// Re-export all types for backwards compatibility
pub use animation::*;
pub use borders::*;
pub use breadcrumbs::*;
pub use cheat_sheet::*;
pub use collapsible::*;
pub use context_display::*;
pub use contextual_help::*;
pub use dialog::*;
pub use help::*;
pub use icons::*;
pub use preview::*;
pub use scrollbar::*;
pub use statusline::*;
pub use table::*;
pub use undo_redo::*;
pub use vector_canvas::*;
pub use welcome_screen::*;
