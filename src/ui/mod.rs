//! UI domain
//!
//! Contains all UI-related modules including widgets, themes, and visual components.
//!
//! # Atomic UI Design
//!
//! Following Atomic Design methodology for Phase 1 refactoring:
//! - **atoms**: Fundamental primitives (text, block, button, icon)
//! - **molecules**: Composite components (metric_card, task_item, progress_bar)
//! - **organisms**: Complex compositions (eval_panel)
//! - **screens**: Full screen layouts (evaluation, welcome, main)

pub mod animations;
pub mod atoms;
pub mod board_background;
pub mod box_drawing;
pub mod enhanced_borders;
pub mod gradient;
pub mod logo;
pub mod molecules;
pub mod multi_window;
pub mod nerd_fonts;
pub mod organisms;
pub mod pixel_canvas;
pub mod responsive_layout;
pub mod screens;
pub mod syntax;
pub mod theme;
pub mod widgets;

// Re-exports
pub use animations::{Animation, AnimationState, EasingFunction, TransitionManager};
pub use board_background::{BackgroundStyle, BoardBackground, BoardBackgrounds, PatternType};
pub use box_drawing::{BoxBuilder, BoxChars, BoxStyle};
pub use pixel_canvas::{Canvas, Pixel, Shape};
pub use enhanced_borders::{BorderEffect, BorderStyles, BorderThickness, CornerStyle, EnhancedBorder};
pub use gradient::{ColorStop, Gradient, GradientDirection, GradientType, Gradients};
pub use multi_window::{Window, WindowId, WindowManager, WindowPriority, WindowState};
pub use nerd_fonts::{GitStatus, NerdFonts, UiIcon, supports_nerd_fonts};
pub use responsive_layout::{ResponsiveLayout, ScreenSize};
pub use syntax::{HighlightTheme, HighlightedSpan, Language, SyntaxHighlighter};
pub use theme::ToadTheme;
pub use widgets::*;
