//! UI domain
//!
//! Contains all UI-related modules including widgets, themes, and visual components.

pub mod animations;
pub mod box_drawing;
pub mod canvas;
pub mod logo;
pub mod nerd_fonts;
pub mod theme;
pub mod widgets;

// Re-exports
pub use animations::{Animation, AnimationState, EasingFunction, TransitionManager};
pub use box_drawing::{BoxBuilder, BoxChars, BoxStyle};
pub use canvas::{Canvas, Pixel, Shape};
pub use nerd_fonts::{GitStatus, NerdFonts, UiIcon, supports_nerd_fonts};
pub use theme::ToadTheme;
pub use widgets::*;
