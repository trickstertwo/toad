//! UI domain
//!
//! Contains all UI-related modules including widgets, themes, and visual components.
//!
//! # Atomic UI Design
//!
//! Following Atomic Design methodology:
//! - **atoms**: Fundamental primitives (text, block, button, icon)
//! - **molecules**: Composite components (metric_card, task_item, progress_bar)
//! - **organisms**: Complex compositions (eval_panel)
//! - **screens**: Full screen layouts (evaluation, welcome, main)
//!
//! # Module Organization
//!
//! - **primitives**: Basic building blocks (box_drawing, nerd_fonts, logo)
//! - **effects**: Visual effects (animations, gradients)
//! - **rendering**: Rendering utilities (backgrounds, canvas, borders)
//! - **layout**: Layout management (responsive, multi-window)
//! - **syntax**: Syntax highlighting
//! - **theme**: Theme system
//! - **widgets**: Stateful UI components

pub mod atoms;
pub mod effects;
pub mod layout;
pub mod molecules;
pub mod organisms;
pub mod primitives;
pub mod rendering;
pub mod screens;
pub mod syntax;
pub mod theme;
pub mod widgets;

// Re-exports for convenience
pub use effects::{
    Animation, AnimationState, ColorStop, EasingFunction, Gradient, GradientDirection,
    GradientType, Gradients, TransitionManager,
};
pub use layout::{
    ResponsiveLayout, ScreenSize, Window, WindowId, WindowManager, WindowPriority, WindowState,
};
pub use primitives::{BoxBuilder, BoxChars, BoxStyle, GitStatus, NerdFonts, UiIcon, supports_nerd_fonts};
pub use rendering::{
    BackgroundStyle, BoardBackground, BoardBackgrounds, BorderEffect, BorderStyles,
    BorderThickness, Canvas, CornerStyle, EnhancedBorder, PatternType, Pixel, Shape,
};
pub use syntax::{HighlightTheme, HighlightedSpan, Language, SyntaxHighlighter};
pub use theme::ToadTheme;
pub use widgets::*;
