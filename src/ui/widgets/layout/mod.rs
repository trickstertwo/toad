//! Layout management widgets
//!
//! This module contains widgets for managing window layout including
//! split panes, floating windows, panels, tabs, and window switching.

pub mod floating;
pub mod minimap;
pub mod panel;
pub mod split;
pub mod tabbar;
pub mod window_switcher;

// Re-export all types for backwards compatibility
pub use floating::*;
pub use minimap::*;
pub use panel::*;
pub use split::*;
pub use tabbar::*;
pub use window_switcher::*;
