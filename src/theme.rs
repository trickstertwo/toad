//! Theme and color palette module
//!
//! Defines the Toad color scheme: toad green accent + grayscale palette

use ratatui::style::Color;

/// Toad color palette - toad green accent with grayscale
pub struct ToadTheme;

impl ToadTheme {
    /// Toad green - primary accent color (vibrant lime green)
    pub const TOAD_GREEN: Color = Color::Rgb(76, 175, 80);

    /// Bright toad green for highlights
    pub const TOAD_GREEN_BRIGHT: Color = Color::Rgb(129, 199, 132);

    /// Dark toad green for borders
    pub const TOAD_GREEN_DARK: Color = Color::Rgb(56, 142, 60);

    // Grayscale palette
    pub const WHITE: Color = Color::Rgb(255, 255, 255);
    pub const LIGHT_GRAY: Color = Color::Rgb(189, 189, 189);
    pub const GRAY: Color = Color::Rgb(158, 158, 158);
    pub const DARK_GRAY: Color = Color::Rgb(97, 97, 97);
    pub const DARKER_GRAY: Color = Color::Rgb(66, 66, 66);
    pub const BLACK: Color = Color::Rgb(33, 33, 33);

    // Semantic colors
    pub const SUCCESS: Color = Self::TOAD_GREEN;
    pub const ERROR: Color = Color::Rgb(244, 67, 54); // Red
    pub const WARNING: Color = Color::Rgb(255, 152, 0); // Orange
    pub const INFO: Color = Self::LIGHT_GRAY;

    // Additional colors
    pub const RED: Color = Color::Rgb(244, 67, 54);
    pub const YELLOW: Color = Color::Rgb(255, 193, 7);
    pub const BLUE: Color = Color::Rgb(33, 150, 243);

    // UI element colors
    pub const BACKGROUND: Color = Self::BLACK;
    pub const FOREGROUND: Color = Self::LIGHT_GRAY;
    pub const BORDER: Color = Self::DARK_GRAY;
    pub const BORDER_FOCUSED: Color = Self::TOAD_GREEN;
    pub const TITLE: Color = Self::WHITE;
    pub const ACCENT: Color = Self::TOAD_GREEN;
}
