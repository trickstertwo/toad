//! Theme color resolver
//!
//! Provides runtime theme color resolution, allowing UI to query colors
//! from the active theme instead of using hardcoded constants.
//!
//! This solves the separation of concerns issue where ThemeManager stores
//! theme state but UI uses hardcoded ToadTheme constants.

use super::{ThemeManager, manager::ThemeName};
use ratatui::style::Color;

/// Runtime theme color resolver
///
/// Provides methods to get colors for the current theme at runtime.
/// Should be created from ThemeManager and passed down the render chain.
///
/// # Examples
///
/// ```
/// use toad::ui::theme::{ThemeManager, ThemeColors};
///
/// let theme_manager = ThemeManager::new();
/// let colors = ThemeColors::from_manager(&theme_manager);
///
/// let fg = colors.foreground();
/// let accent = colors.accent();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    /// Foreground/text color
    pub foreground: Color,
    /// Background color
    pub background: Color,
    /// Primary accent color (Toad green)
    pub accent: Color,
    /// Bright accent variant
    pub accent_bright: Color,
    /// Dark accent variant
    pub accent_dark: Color,
    /// Gray text color
    pub gray: Color,
    /// Dark gray
    pub dark_gray: Color,
    /// Error/danger color
    pub error: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Info color
    pub info: Color,
}

impl ThemeColors {
    /// Create theme colors from ThemeManager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::{ThemeManager, ThemeColors};
    ///
    /// let manager = ThemeManager::new();
    /// let colors = ThemeColors::from_manager(&manager);
    /// ```
    pub fn from_manager(manager: &ThemeManager) -> Self {
        let theme_name = manager.current_theme_name();
        Self::from_theme_name(theme_name)
    }

    /// Create theme colors from ThemeName
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::{ThemeColors, manager::ThemeName};
    ///
    /// let colors = ThemeColors::from_theme_name(ThemeName::Dark);
    /// ```
    pub fn from_theme_name(theme: ThemeName) -> Self {
        match theme {
            ThemeName::Dark => Self::dark(),
            ThemeName::Light => Self::light(),
            ThemeName::HighContrast => Self::high_contrast(),
            ThemeName::CatppuccinMocha => Self::catppuccin_mocha(),
            ThemeName::CatppuccinMacchiato => Self::catppuccin_macchiato(),
            ThemeName::CatppuccinFrappe => Self::catppuccin_frappe(),
            ThemeName::CatppuccinLatte => Self::catppuccin_latte(),
            ThemeName::Nord => Self::nord(),
            ThemeName::Custom => Self::dark(), // Fallback for now
        }
    }

    /// Dark theme colors
    fn dark() -> Self {
        Self {
            foreground: Color::Rgb(211, 211, 211),
            background: Color::Rgb(0, 0, 0),
            accent: Color::Rgb(0, 255, 127),
            accent_bright: Color::Rgb(127, 255, 170),
            accent_dark: Color::Rgb(0, 128, 64),
            gray: Color::Rgb(128, 128, 128),
            dark_gray: Color::Rgb(64, 64, 64),
            error: Color::Rgb(255, 87, 87),
            success: Color::Rgb(0, 255, 127),
            warning: Color::Rgb(255, 191, 0),
            info: Color::Rgb(100, 149, 237),
        }
    }

    /// Light theme colors
    fn light() -> Self {
        Self {
            foreground: Color::Rgb(50, 50, 50),
            background: Color::Rgb(255, 255, 255),
            accent: Color::Rgb(0, 150, 80),
            accent_bright: Color::Rgb(0, 200, 100),
            accent_dark: Color::Rgb(0, 100, 50),
            gray: Color::Rgb(128, 128, 128),
            dark_gray: Color::Rgb(180, 180, 180),
            error: Color::Rgb(200, 0, 0),
            success: Color::Rgb(0, 150, 80),
            warning: Color::Rgb(200, 140, 0),
            info: Color::Rgb(70, 130, 220),
        }
    }

    /// High contrast theme colors
    fn high_contrast() -> Self {
        Self {
            foreground: Color::Rgb(255, 255, 255),
            background: Color::Rgb(0, 0, 0),
            accent: Color::Rgb(0, 255, 0),
            accent_bright: Color::Rgb(127, 255, 127),
            accent_dark: Color::Rgb(0, 200, 0),
            gray: Color::Rgb(192, 192, 192),
            dark_gray: Color::Rgb(64, 64, 64),
            error: Color::Rgb(255, 0, 0),
            success: Color::Rgb(0, 255, 0),
            warning: Color::Rgb(255, 255, 0),
            info: Color::Rgb(0, 255, 255),
        }
    }

    /// Catppuccin Mocha theme colors
    fn catppuccin_mocha() -> Self {
        Self {
            foreground: Color::Rgb(205, 214, 244),
            background: Color::Rgb(30, 30, 46),
            accent: Color::Rgb(166, 227, 161),
            accent_bright: Color::Rgb(148, 226, 213),
            accent_dark: Color::Rgb(137, 180, 250),
            gray: Color::Rgb(108, 112, 134),
            dark_gray: Color::Rgb(69, 71, 90),
            error: Color::Rgb(243, 139, 168),
            success: Color::Rgb(166, 227, 161),
            warning: Color::Rgb(249, 226, 175),
            info: Color::Rgb(137, 180, 250),
        }
    }

    /// Catppuccin Macchiato theme colors
    fn catppuccin_macchiato() -> Self {
        Self {
            foreground: Color::Rgb(202, 211, 245),
            background: Color::Rgb(36, 39, 58),
            accent: Color::Rgb(166, 218, 149),
            accent_bright: Color::Rgb(139, 213, 202),
            accent_dark: Color::Rgb(125, 196, 228),
            gray: Color::Rgb(110, 115, 141),
            dark_gray: Color::Rgb(73, 77, 100),
            error: Color::Rgb(237, 135, 150),
            success: Color::Rgb(166, 218, 149),
            warning: Color::Rgb(238, 212, 159),
            info: Color::Rgb(125, 196, 228),
        }
    }

    /// Catppuccin Frappe theme colors
    fn catppuccin_frappe() -> Self {
        Self {
            foreground: Color::Rgb(198, 208, 245),
            background: Color::Rgb(48, 52, 70),
            accent: Color::Rgb(166, 209, 137),
            accent_bright: Color::Rgb(129, 200, 190),
            accent_dark: Color::Rgb(140, 170, 238),
            gray: Color::Rgb(115, 121, 148),
            dark_gray: Color::Rgb(81, 87, 109),
            error: Color::Rgb(231, 130, 132),
            success: Color::Rgb(166, 209, 137),
            warning: Color::Rgb(229, 200, 144),
            info: Color::Rgb(140, 170, 238),
        }
    }

    /// Catppuccin Latte theme colors
    fn catppuccin_latte() -> Self {
        Self {
            foreground: Color::Rgb(76, 79, 105),
            background: Color::Rgb(239, 241, 245),
            accent: Color::Rgb(64, 160, 43),
            accent_bright: Color::Rgb(23, 146, 153),
            accent_dark: Color::Rgb(30, 102, 245),
            gray: Color::Rgb(140, 143, 161),
            dark_gray: Color::Rgb(188, 192, 204),
            error: Color::Rgb(210, 15, 57),
            success: Color::Rgb(64, 160, 43),
            warning: Color::Rgb(223, 142, 29),
            info: Color::Rgb(30, 102, 245),
        }
    }

    /// Nord theme colors
    fn nord() -> Self {
        Self {
            foreground: Color::Rgb(216, 222, 233),
            background: Color::Rgb(46, 52, 64),
            accent: Color::Rgb(136, 192, 208),
            accent_bright: Color::Rgb(143, 188, 187),
            accent_dark: Color::Rgb(94, 129, 172),
            gray: Color::Rgb(76, 86, 106),
            dark_gray: Color::Rgb(59, 66, 82),
            error: Color::Rgb(191, 97, 106),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            info: Color::Rgb(129, 161, 193),
        }
    }

    // Convenience getter methods

    /// Get foreground color
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    /// Get background color
    pub fn background(&self) -> Color {
        self.background
    }

    /// Get accent color
    pub fn accent(&self) -> Color {
        self.accent
    }

    /// Get bright accent color
    pub fn accent_bright(&self) -> Color {
        self.accent_bright
    }

    /// Get dark accent color
    pub fn accent_dark(&self) -> Color {
        self.accent_dark
    }

    /// Get gray color
    pub fn gray(&self) -> Color {
        self.gray
    }

    /// Get dark gray color
    pub fn dark_gray(&self) -> Color {
        self.dark_gray
    }

    /// Get error color
    pub fn error(&self) -> Color {
        self.error
    }

    /// Get success color
    pub fn success(&self) -> Color {
        self.success
    }

    /// Get warning color
    pub fn warning(&self) -> Color {
        self.warning
    }

    /// Get info color
    pub fn info(&self) -> Color {
        self.info
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors_dark() {
        let colors = ThemeColors::dark();
        assert_eq!(colors.foreground, Color::Rgb(211, 211, 211));
        assert_eq!(colors.background, Color::Rgb(0, 0, 0));
    }

    #[test]
    fn test_theme_colors_light() {
        let colors = ThemeColors::light();
        assert_eq!(colors.foreground, Color::Rgb(50, 50, 50));
        assert_eq!(colors.background, Color::Rgb(255, 255, 255));
    }

    #[test]
    fn test_theme_colors_from_theme_name() {
        let dark = ThemeColors::from_theme_name(ThemeName::Dark);
        assert_eq!(dark.accent, Color::Rgb(0, 255, 127));

        let light = ThemeColors::from_theme_name(ThemeName::Light);
        assert_eq!(light.accent, Color::Rgb(0, 150, 80));
    }

    #[test]
    fn test_theme_colors_default() {
        let colors = ThemeColors::default();
        assert_eq!(colors.foreground, Color::Rgb(211, 211, 211));
    }

    #[test]
    fn test_theme_colors_getter_methods() {
        let colors = ThemeColors::dark();
        assert_eq!(colors.foreground(), colors.foreground);
        assert_eq!(colors.background(), colors.background);
        assert_eq!(colors.accent(), colors.accent);
        assert_eq!(colors.accent_bright(), colors.accent_bright);
        assert_eq!(colors.accent_dark(), colors.accent_dark);
        assert_eq!(colors.gray(), colors.gray);
        assert_eq!(colors.dark_gray(), colors.dark_gray);
        assert_eq!(colors.error(), colors.error);
        assert_eq!(colors.success(), colors.success);
        assert_eq!(colors.warning(), colors.warning);
        assert_eq!(colors.info(), colors.info);
    }

    #[test]
    fn test_theme_colors_from_manager() {
        let manager = ThemeManager::new();
        let colors = ThemeColors::from_manager(&manager);
        // Should default to Dark theme
        assert_eq!(colors.foreground, Color::Rgb(211, 211, 211));
    }

    #[test]
    fn test_all_themes_have_colors() {
        for theme in ThemeName::all() {
            let colors = ThemeColors::from_theme_name(theme);
            // Verify all colors are set (non-default RGB values)
            assert!(colors.foreground != Color::Reset);
            assert!(colors.background != Color::Reset);
            assert!(colors.accent != Color::Reset);
        }
    }

    #[test]
    fn test_theme_colors_clone() {
        let colors1 = ThemeColors::dark();
        let colors2 = colors1.clone();
        assert_eq!(colors1.foreground, colors2.foreground);
        assert_eq!(colors1.accent, colors2.accent);
    }

    #[test]
    fn test_theme_colors_copy() {
        let colors1 = ThemeColors::dark();
        let colors2 = colors1; // Copy, not move
        assert_eq!(colors1.foreground, colors2.foreground);
    }
}
