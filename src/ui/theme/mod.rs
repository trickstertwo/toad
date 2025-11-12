//! Theming system for TOAD
//!
//! Provides multiple color schemes, built-in themes, and custom theme support
pub mod builtin;
pub mod catppuccin;
pub mod manager;
pub mod nord;
pub mod resolver;

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
pub use resolver::ThemeColors as ResolvedThemeColors;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test CustomTheme creation and basic properties
    #[test]
    fn test_custom_theme_creation() {
        let colors = ThemeColors {
            primary: (100, 150, 200),
            primary_bright: (120, 170, 220),
            primary_dark: (80, 130, 180),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (100, 150, 200),
        };

        let theme = CustomTheme::new("TestTheme".to_string(), "A test theme".to_string(), colors);

        assert_eq!(theme.name(), "TestTheme");
        assert_eq!(theme.description(), "A test theme");
    }

    /// Test CustomTheme primary colors
    #[test]
    fn test_custom_theme_primary_colors() {
        let colors = ThemeColors {
            primary: (100, 150, 200),
            primary_bright: (120, 170, 220),
            primary_dark: (80, 130, 180),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (100, 150, 200),
        };

        let theme = CustomTheme::new("Test".to_string(), "Test".to_string(), colors);

        assert_eq!(theme.primary(), Color::Rgb(100, 150, 200));
        assert_eq!(theme.primary_bright(), Color::Rgb(120, 170, 220));
        assert_eq!(theme.primary_dark(), Color::Rgb(80, 130, 180));
    }

    /// Test CustomTheme grayscale colors
    #[test]
    fn test_custom_theme_grayscale() {
        let colors = ThemeColors {
            primary: (0, 0, 0),
            primary_bright: (0, 0, 0),
            primary_dark: (0, 0, 0),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 0, 0),
            error: (0, 0, 0),
            warning: (0, 0, 0),
            info: (0, 0, 0),
            red: (0, 0, 0),
            yellow: (0, 0, 0),
            blue: (0, 0, 0),
            green: (0, 0, 0),
            cyan: (0, 0, 0),
            magenta: (0, 0, 0),
            background: (0, 0, 0),
            foreground: (0, 0, 0),
            border: (0, 0, 0),
            border_focused: (0, 0, 0),
            title: (0, 0, 0),
            accent: (0, 0, 0),
        };

        let theme = CustomTheme::new("Test".to_string(), "Test".to_string(), colors);

        assert_eq!(theme.white(), Color::Rgb(255, 255, 255));
        assert_eq!(theme.light_gray(), Color::Rgb(200, 200, 200));
        assert_eq!(theme.gray(), Color::Rgb(128, 128, 128));
        assert_eq!(theme.dark_gray(), Color::Rgb(64, 64, 64));
        assert_eq!(theme.darker_gray(), Color::Rgb(32, 32, 32));
        assert_eq!(theme.black(), Color::Rgb(0, 0, 0));
    }

    /// Test CustomTheme semantic colors
    #[test]
    fn test_custom_theme_semantic_colors() {
        let colors = ThemeColors {
            primary: (0, 0, 0),
            primary_bright: (0, 0, 0),
            primary_dark: (0, 0, 0),
            white: (0, 0, 0),
            light_gray: (0, 0, 0),
            gray: (0, 0, 0),
            dark_gray: (0, 0, 0),
            darker_gray: (0, 0, 0),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (0, 0, 0),
            yellow: (0, 0, 0),
            blue: (0, 0, 0),
            green: (0, 0, 0),
            cyan: (0, 0, 0),
            magenta: (0, 0, 0),
            background: (0, 0, 0),
            foreground: (0, 0, 0),
            border: (0, 0, 0),
            border_focused: (0, 0, 0),
            title: (0, 0, 0),
            accent: (0, 0, 0),
        };

        let theme = CustomTheme::new("Test".to_string(), "Test".to_string(), colors);

        assert_eq!(theme.success(), Color::Rgb(0, 255, 0));
        assert_eq!(theme.error(), Color::Rgb(255, 0, 0));
        assert_eq!(theme.warning(), Color::Rgb(255, 255, 0));
        assert_eq!(theme.info(), Color::Rgb(0, 255, 255));
    }

    /// Test CustomTheme additional colors
    #[test]
    fn test_custom_theme_additional_colors() {
        let colors = ThemeColors {
            primary: (0, 0, 0),
            primary_bright: (0, 0, 0),
            primary_dark: (0, 0, 0),
            white: (0, 0, 0),
            light_gray: (0, 0, 0),
            gray: (0, 0, 0),
            dark_gray: (0, 0, 0),
            darker_gray: (0, 0, 0),
            black: (0, 0, 0),
            success: (0, 0, 0),
            error: (0, 0, 0),
            warning: (0, 0, 0),
            info: (0, 0, 0),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (0, 0, 0),
            foreground: (0, 0, 0),
            border: (0, 0, 0),
            border_focused: (0, 0, 0),
            title: (0, 0, 0),
            accent: (0, 0, 0),
        };

        let theme = CustomTheme::new("Test".to_string(), "Test".to_string(), colors);

        assert_eq!(theme.red(), Color::Rgb(255, 0, 0));
        assert_eq!(theme.yellow(), Color::Rgb(255, 255, 0));
        assert_eq!(theme.blue(), Color::Rgb(0, 0, 255));
        assert_eq!(theme.green(), Color::Rgb(0, 255, 0));
        assert_eq!(theme.cyan(), Color::Rgb(0, 255, 255));
        assert_eq!(theme.magenta(), Color::Rgb(255, 0, 255));
    }

    /// Test CustomTheme UI element colors
    #[test]
    fn test_custom_theme_ui_colors() {
        let colors = ThemeColors {
            primary: (0, 0, 0),
            primary_bright: (0, 0, 0),
            primary_dark: (0, 0, 0),
            white: (0, 0, 0),
            light_gray: (0, 0, 0),
            gray: (0, 0, 0),
            dark_gray: (0, 0, 0),
            darker_gray: (0, 0, 0),
            black: (0, 0, 0),
            success: (0, 0, 0),
            error: (0, 0, 0),
            warning: (0, 0, 0),
            info: (0, 0, 0),
            red: (0, 0, 0),
            yellow: (0, 0, 0),
            blue: (0, 0, 0),
            green: (0, 0, 0),
            cyan: (0, 0, 0),
            magenta: (0, 0, 0),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (200, 100, 50),
        };

        let theme = CustomTheme::new("Test".to_string(), "Test".to_string(), colors);

        assert_eq!(theme.background(), Color::Rgb(10, 10, 10));
        assert_eq!(theme.foreground(), Color::Rgb(240, 240, 240));
        assert_eq!(theme.border(), Color::Rgb(128, 128, 128));
        assert_eq!(theme.border_focused(), Color::Rgb(100, 150, 200));
        assert_eq!(theme.title(), Color::Rgb(255, 255, 255));
        assert_eq!(theme.accent(), Color::Rgb(200, 100, 50));
    }

    /// Test ToadTheme constants - primary colors
    #[test]
    fn test_toad_theme_constants_primary() {
        assert_eq!(ToadTheme::TOAD_GREEN, Color::Rgb(76, 175, 80));
        assert_eq!(ToadTheme::TOAD_GREEN_BRIGHT, Color::Rgb(129, 199, 132));
        assert_eq!(ToadTheme::TOAD_GREEN_DARK, Color::Rgb(56, 142, 60));
    }

    /// Test ToadTheme constants - grayscale
    #[test]
    fn test_toad_theme_constants_grayscale() {
        assert_eq!(ToadTheme::WHITE, Color::Rgb(255, 255, 255));
        assert_eq!(ToadTheme::LIGHT_GRAY, Color::Rgb(189, 189, 189));
        assert_eq!(ToadTheme::GRAY, Color::Rgb(158, 158, 158));
        assert_eq!(ToadTheme::DARK_GRAY, Color::Rgb(97, 97, 97));
        assert_eq!(ToadTheme::DARKER_GRAY, Color::Rgb(66, 66, 66));
        assert_eq!(ToadTheme::BLACK, Color::Rgb(33, 33, 33));
    }

    /// Test ToadTheme constants - semantic colors
    #[test]
    fn test_toad_theme_constants_semantic() {
        assert_eq!(ToadTheme::SUCCESS, ToadTheme::TOAD_GREEN);
        assert_eq!(ToadTheme::ERROR, Color::Rgb(244, 67, 54));
        assert_eq!(ToadTheme::WARNING, Color::Rgb(255, 152, 0));
        assert_eq!(ToadTheme::INFO, ToadTheme::LIGHT_GRAY);
    }

    /// Test ToadTheme constants - additional colors
    #[test]
    fn test_toad_theme_constants_additional() {
        assert_eq!(ToadTheme::RED, Color::Rgb(244, 67, 54));
        assert_eq!(ToadTheme::YELLOW, Color::Rgb(255, 193, 7));
        assert_eq!(ToadTheme::BLUE, Color::Rgb(33, 150, 243));
    }

    /// Test ToadTheme constants - UI elements
    #[test]
    fn test_toad_theme_constants_ui() {
        assert_eq!(ToadTheme::BACKGROUND, ToadTheme::BLACK);
        assert_eq!(ToadTheme::FOREGROUND, ToadTheme::LIGHT_GRAY);
        assert_eq!(ToadTheme::BORDER, ToadTheme::DARK_GRAY);
        assert_eq!(ToadTheme::BORDER_FOCUSED, ToadTheme::TOAD_GREEN);
        assert_eq!(ToadTheme::TITLE, ToadTheme::WHITE);
        assert_eq!(ToadTheme::ACCENT, ToadTheme::TOAD_GREEN);
    }

    /// Test ThemeColors Clone trait
    #[test]
    fn test_theme_colors_clone() {
        let colors = ThemeColors {
            primary: (100, 150, 200),
            primary_bright: (120, 170, 220),
            primary_dark: (80, 130, 180),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (100, 150, 200),
        };

        let cloned = colors.clone();
        assert_eq!(cloned.primary, colors.primary);
        assert_eq!(cloned.background, colors.background);
    }

    /// Test ThemeColors Debug trait
    #[test]
    fn test_theme_colors_debug() {
        let colors = ThemeColors {
            primary: (100, 150, 200),
            primary_bright: (120, 170, 220),
            primary_dark: (80, 130, 180),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (100, 150, 200),
        };

        let debug_str = format!("{:?}", colors);
        assert!(debug_str.contains("ThemeColors"));
    }

    /// Test CustomTheme Clone trait
    #[test]
    fn test_custom_theme_clone() {
        let colors = ThemeColors {
            primary: (100, 150, 200),
            primary_bright: (120, 170, 220),
            primary_dark: (80, 130, 180),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (100, 150, 200),
        };

        let theme = CustomTheme::new("Original".to_string(), "Original theme".to_string(), colors);

        let cloned = theme.clone();
        assert_eq!(cloned.name(), theme.name());
        assert_eq!(cloned.description(), theme.description());
        assert_eq!(cloned.primary(), theme.primary());
    }

    /// Test CustomTheme Debug trait
    #[test]
    fn test_custom_theme_debug() {
        let colors = ThemeColors {
            primary: (100, 150, 200),
            primary_bright: (120, 170, 220),
            primary_dark: (80, 130, 180),
            white: (255, 255, 255),
            light_gray: (200, 200, 200),
            gray: (128, 128, 128),
            dark_gray: (64, 64, 64),
            darker_gray: (32, 32, 32),
            black: (0, 0, 0),
            success: (0, 255, 0),
            error: (255, 0, 0),
            warning: (255, 255, 0),
            info: (0, 255, 255),
            red: (255, 0, 0),
            yellow: (255, 255, 0),
            blue: (0, 0, 255),
            green: (0, 255, 0),
            cyan: (0, 255, 255),
            magenta: (255, 0, 255),
            background: (10, 10, 10),
            foreground: (240, 240, 240),
            border: (128, 128, 128),
            border_focused: (100, 150, 200),
            title: (255, 255, 255),
            accent: (100, 150, 200),
        };

        let theme = CustomTheme::new("Test".to_string(), "Test theme".to_string(), colors);

        let debug_str = format!("{:?}", theme);
        assert!(debug_str.contains("CustomTheme"));
    }
}
