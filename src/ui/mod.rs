//! UI domain
//!
//! Contains all UI-related modules including widgets, themes, and visual components.

pub mod animations;
pub mod board_background;
pub mod box_drawing;
pub mod canvas;
pub mod enhanced_borders;
pub mod gradient;
pub mod logo;
pub mod multi_window;
pub mod nerd_fonts;
pub mod responsive_layout;
pub mod syntax;
pub mod theme;
pub mod widgets;

// Re-exports
pub use animations::{Animation, AnimationState, EasingFunction, TransitionManager};
pub use board_background::{BackgroundStyle, BoardBackground, BoardBackgrounds, PatternType};
pub use box_drawing::{BoxBuilder, BoxChars, BoxStyle};
pub use canvas::{Canvas, Pixel, Shape};
pub use enhanced_borders::{BorderEffect, BorderStyles, BorderThickness, CornerStyle, EnhancedBorder};
pub use gradient::{ColorStop, Gradient, GradientDirection, GradientType, Gradients};
pub use multi_window::{Window, WindowId, WindowManager, WindowPriority, WindowState};
pub use nerd_fonts::{GitStatus, NerdFonts, UiIcon, supports_nerd_fonts};
pub use responsive_layout::{ResponsiveLayout, ScreenSize};
pub use syntax::{HighlightTheme, HighlightedSpan, Language, SyntaxHighlighter};
pub use theme::ToadTheme;
pub use widgets::*;
