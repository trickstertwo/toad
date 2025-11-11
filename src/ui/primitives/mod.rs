//! UI Primitives
//!
//! Basic building blocks for terminal UI: box drawing, Nerd Fonts icons, and logo rendering.
//!
//! # Modules
//!
//! - [`box_drawing`]: Unicode box drawing characters and utilities
//! - [`nerd_fonts`]: Nerd Fonts icon definitions
//! - [`logo`]: TOAD logo rendering

pub mod box_drawing;
pub mod logo;
pub mod nerd_fonts;

// Re-export public types
pub use box_drawing::{BoxBuilder, BoxChars, BoxStyle};
pub use nerd_fonts::{GitStatus, NerdFonts, UiIcon, supports_nerd_fonts};
