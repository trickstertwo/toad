//! UI Layout Utilities
//!
//! Layout management and responsive design for terminal UI.
//!
//! # Modules
//!
//! - [`responsive_layout`]: Responsive layout system that adapts to terminal size
//! - [`multi_window`]: Multi-window management and prioritization

pub mod multi_window;
pub mod responsive_layout;

// Re-export public types
pub use multi_window::{Window, WindowId, WindowManager, WindowPriority, WindowState};
pub use responsive_layout::{ResponsiveLayout, ScreenSize};
