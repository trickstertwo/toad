///! Theming system for TOAD
///!
///! Provides multiple color schemes, built-in themes, and custom theme support
pub mod builtin;
pub mod catppuccin;
pub mod manager;
pub mod nord;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Theme trait defining all color properties
pub trait Theme {
    /// Theme name
    fn name(&self) -> &str;

    /// Theme description
    fn description(&self) -> &str;

    // Primary colors
    fn primary(&self) -> Color;
    fn primary_bright(&self) -> Color;
    fn primary_dark(&self) -> Color;

    // Grayscale
    fn white(&self) -> Color;
    fn light_gray(&self) -> Color;
    fn gray(&self) -> Color;
    fn dark_gray(&self) -> Color;
    fn darker_gray(&self) -> Color;
    fn black(&self) -> Color;

    // Semantic colors
    fn success(&self) -> Color;
    fn error(&self) -> Color;
    fn warning(&self) -> Color;
    fn info(&self) -> Color;

    // Additional colors
    fn red(&self) -> Color;
    fn yellow(&self) -> Color;
    fn blue(&self) -> Color;
    fn green(&self) -> Color;
    fn cyan(&self) -> Color;
    fn magenta(&self) -> Color;

    // UI element colors
    fn background(&self) -> Color;
    fn foreground(&self) -> Color;
    fn border(&self) -> Color;
    fn border_focused(&self) -> Color;
    fn title(&self) -> Color;
    fn accent(&self) -> Color;
}

/// Color configuration for custom themes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    // Primary
    pub primary: (u8, u8, u8),
    pub primary_bright: (u8, u8, u8),
    pub primary_dark: (u8, u8, u8),

    // Grayscale
    pub white: (u8, u8, u8),
    pub light_gray: (u8, u8, u8),
    pub gray: (u8, u8, u8),
    pub dark_gray: (u8, u8, u8),
    pub darker_gray: (u8, u8, u8),
    pub black: (u8, u8, u8),

    // Semantic
    pub success: (u8, u8, u8),
    pub error: (u8, u8, u8),
    pub warning: (u8, u8, u8),
    pub info: (u8, u8, u8),

    // Additional
    pub red: (u8, u8, u8),
    pub yellow: (u8, u8, u8),
    pub blue: (u8, u8, u8),
    pub green: (u8, u8, u8),
    pub cyan: (u8, u8, u8),
    pub magenta: (u8, u8, u8),

    // UI elements
    pub background: (u8, u8, u8),
    pub foreground: (u8, u8, u8),
    pub border: (u8, u8, u8),
    pub border_focused: (u8, u8, u8),
    pub title: (u8, u8, u8),
    pub accent: (u8, u8, u8),
}

/// Custom theme from configuration
#[derive(Debug, Clone)]
pub struct CustomTheme {
    name: String,
    description: String,
    colors: ThemeColors,
}

impl CustomTheme {
    /// Create a new custom theme
    pub fn new(name: String, description: String, colors: ThemeColors) -> Self {
        Self {
            name,
            description,
            colors,
        }
    }

    /// Convert RGB tuple to Color
    fn rgb(rgb: (u8, u8, u8)) -> Color {
        Color::Rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl Theme for CustomTheme {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn primary(&self) -> Color {
        Self::rgb(self.colors.primary)
    }

    fn primary_bright(&self) -> Color {
        Self::rgb(self.colors.primary_bright)
    }

    fn primary_dark(&self) -> Color {
        Self::rgb(self.colors.primary_dark)
    }

    fn white(&self) -> Color {
        Self::rgb(self.colors.white)
    }

    fn light_gray(&self) -> Color {
        Self::rgb(self.colors.light_gray)
    }

    fn gray(&self) -> Color {
        Self::rgb(self.colors.gray)
    }

    fn dark_gray(&self) -> Color {
        Self::rgb(self.colors.dark_gray)
    }

    fn darker_gray(&self) -> Color {
        Self::rgb(self.colors.darker_gray)
    }

    fn black(&self) -> Color {
        Self::rgb(self.colors.black)
    }

    fn success(&self) -> Color {
        Self::rgb(self.colors.success)
    }

    fn error(&self) -> Color {
        Self::rgb(self.colors.error)
    }

    fn warning(&self) -> Color {
        Self::rgb(self.colors.warning)
    }

    fn info(&self) -> Color {
        Self::rgb(self.colors.info)
    }

    fn red(&self) -> Color {
        Self::rgb(self.colors.red)
    }

    fn yellow(&self) -> Color {
        Self::rgb(self.colors.yellow)
    }

    fn blue(&self) -> Color {
        Self::rgb(self.colors.blue)
    }

    fn green(&self) -> Color {
        Self::rgb(self.colors.green)
    }

    fn cyan(&self) -> Color {
        Self::rgb(self.colors.cyan)
    }

    fn magenta(&self) -> Color {
        Self::rgb(self.colors.magenta)
    }

    fn background(&self) -> Color {
        Self::rgb(self.colors.background)
    }

    fn foreground(&self) -> Color {
        Self::rgb(self.colors.foreground)
    }

    fn border(&self) -> Color {
        Self::rgb(self.colors.border)
    }

    fn border_focused(&self) -> Color {
        Self::rgb(self.colors.border_focused)
    }

    fn title(&self) -> Color {
        Self::rgb(self.colors.title)
    }

    fn accent(&self) -> Color {
        Self::rgb(self.colors.accent)
    }
}

/// Legacy ToadTheme struct for backward compatibility
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

// Re-exports
pub use builtin::{DarkTheme, HighContrastTheme, LightTheme};
pub use catppuccin::{CatppuccinFrappe, CatppuccinLatte, CatppuccinMacchiato, CatppuccinMocha};
pub use manager::ThemeManager;
pub use nord::NordTheme;
