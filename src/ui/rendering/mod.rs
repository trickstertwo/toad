//! UI Rendering Utilities
//!
//! Low-level rendering utilities for terminal graphics.
//!
//! # Modules
//!
//! - [`board_background`]: Background patterns and styles
//! - [`pixel_canvas`]: Pixel-level canvas for custom graphics
//! - [`enhanced_borders`]: Advanced border styles and effects

pub mod board_background;
pub mod enhanced_borders;
pub mod pixel_canvas;

// Re-export public types
pub use board_background::{BackgroundStyle, BoardBackground, BoardBackgrounds, PatternType};
pub use enhanced_borders::{
    BorderEffect, BorderStyles, BorderThickness, CornerStyle, EnhancedBorder,
};
pub use pixel_canvas::{Canvas, Pixel, Shape};
