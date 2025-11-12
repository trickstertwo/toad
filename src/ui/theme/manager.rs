/// Theme manager for loading, storing, and hot-reloading themes
use super::{
    CustomTheme, Theme, ThemeColors,
    builtin::{DarkTheme, HighContrastTheme, LightTheme},
    catppuccin::{CatppuccinFrappe, CatppuccinLatte, CatppuccinMacchiato, CatppuccinMocha},
    nord::NordTheme,
};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Available theme names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeName {
    Dark,
    Light,
    HighContrast,
    CatppuccinMocha,
    CatppuccinMacchiato,
    CatppuccinFrappe,
    CatppuccinLatte,
    Nord,
    Custom,
}

impl ThemeName {
    /// Get all built-in theme names
    pub fn all() -> Vec<ThemeName> {
        vec![
            ThemeName::Dark,
            ThemeName::Light,
            ThemeName::HighContrast,
            ThemeName::CatppuccinMocha,
            ThemeName::CatppuccinMacchiato,
            ThemeName::CatppuccinFrappe,
            ThemeName::CatppuccinLatte,
            ThemeName::Nord,
        ]
    }

    /// Get theme name as string
    pub fn as_str(&self) -> &str {
        match self {
            ThemeName::Dark => "Dark",
            ThemeName::Light => "Light",
            ThemeName::HighContrast => "High Contrast",
            ThemeName::CatppuccinMocha => "Catppuccin Mocha",
            ThemeName::CatppuccinMacchiato => "Catppuccin Macchiato",
            ThemeName::CatppuccinFrappe => "Catppuccin Frappe",
            ThemeName::CatppuccinLatte => "Catppuccin Latte",
            ThemeName::Nord => "Nord",
            ThemeName::Custom => "Custom",
        }
    }

    /// Parse theme name from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "dark" => Some(ThemeName::Dark),
            "light" => Some(ThemeName::Light),
            "high contrast" | "highcontrast" => Some(ThemeName::HighContrast),
            "catppuccin mocha" | "mocha" => Some(ThemeName::CatppuccinMocha),
            "catppuccin macchiato" | "macchiato" => Some(ThemeName::CatppuccinMacchiato),
            "catppuccin frappe" | "frappe" => Some(ThemeName::CatppuccinFrappe),
            "catppuccin latte" | "latte" => Some(ThemeName::CatppuccinLatte),
            "nord" => Some(ThemeName::Nord),
            "custom" => Some(ThemeName::Custom),
            _ => None,
        }
    }
}

/// Theme manager
pub struct ThemeManager {
    /// Current theme name
    current_theme: ThemeName,
    /// Custom theme (if loaded)
    custom_theme: Option<CustomTheme>,
    /// Custom theme path (for hot-reload)
    custom_theme_path: Option<PathBuf>,
    /// NO_COLOR environment variable support (ANSI-compatible)
    no_color_mode: bool,
}

impl ThemeManager {
    /// Create a new theme manager with default theme
    ///
    /// Automatically detects terminal background and NO_COLOR environment variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::manager::ThemeManager;
    ///
    /// let manager = ThemeManager::new();
    /// ```
    pub fn new() -> Self {
        let no_color_mode = Self::detect_no_color();
        let current_theme = Self::auto_detect_theme();

        Self {
            current_theme,
            custom_theme: None,
            custom_theme_path: None,
            no_color_mode,
        }
    }

    /// Create a theme manager with explicit theme (bypass auto-detection)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::manager::{ThemeManager, ThemeName};
    ///
    /// let manager = ThemeManager::with_theme(ThemeName::Nord);
    /// ```
    pub fn with_theme(theme: ThemeName) -> Self {
        Self {
            current_theme: theme,
            custom_theme: None,
            custom_theme_path: None,
            no_color_mode: Self::detect_no_color(),
        }
    }

    /// Detect NO_COLOR environment variable
    ///
    /// Follows the NO_COLOR standard: https://no-color.org/
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::manager::ThemeManager;
    ///
    /// let no_color = ThemeManager::detect_no_color();
    /// ```
    pub fn detect_no_color() -> bool {
        std::env::var("NO_COLOR").is_ok()
    }

    /// Auto-detect appropriate theme based on terminal background
    ///
    /// Attempts to detect terminal background and select appropriate theme:
    /// - Light backgrounds → Light theme
    /// - Dark backgrounds → Dark theme
    /// - Unknown → Dark theme (safe default)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::manager::ThemeManager;
    ///
    /// let theme = ThemeManager::auto_detect_theme();
    /// ```
    pub fn auto_detect_theme() -> ThemeName {
        // Try environment hints first
        if let Ok(colorfgbg) = std::env::var("COLORFGBG") {
            // COLORFGBG format: "foreground;background"
            // Background color codes: 0-7 are dark, 8-15 are light
            if let Some(bg_str) = colorfgbg.split(';').nth(1) {
                if let Ok(bg_code) = bg_str.parse::<u8>() {
                    return if bg_code >= 8 {
                        ThemeName::Light
                    } else {
                        ThemeName::Dark
                    };
                }
            }
        }

        // Check TERM_PROGRAM for known terminals with default backgrounds
        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            match term_program.as_str() {
                "iTerm.app" | "Terminal.app" | "Hyper" => {
                    // These typically default to dark
                    return ThemeName::Dark;
                }
                "vscode" => {
                    // VSCode can vary, check VSCODE_THEME_VARIANT
                    if let Ok(variant) = std::env::var("VSCODE_THEME_VARIANT") {
                        return if variant == "light" {
                            ThemeName::Light
                        } else {
                            ThemeName::Dark
                        };
                    }
                }
                _ => {}
            }
        }

        // Default to Dark theme if detection fails
        ThemeName::Dark
    }

    /// Check if NO_COLOR mode is enabled
    ///
    /// When NO_COLOR is enabled, applications should use minimal colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::manager::ThemeManager;
    ///
    /// let manager = ThemeManager::new();
    /// if manager.is_no_color() {
    ///     // Use minimal colors
    /// }
    /// ```
    pub fn is_no_color(&self) -> bool {
        self.no_color_mode
    }

    /// Force enable/disable NO_COLOR mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::theme::manager::ThemeManager;
    ///
    /// let mut manager = ThemeManager::new();
    /// manager.set_no_color(true);
    /// assert!(manager.is_no_color());
    /// ```
    pub fn set_no_color(&mut self, enabled: bool) {
        self.no_color_mode = enabled;
    }

    /// Get current theme name
    pub fn current_theme_name(&self) -> ThemeName {
        self.current_theme
    }

    /// Set theme by name
    pub fn set_theme(&mut self, theme: ThemeName) {
        self.current_theme = theme;
    }

    /// Load custom theme from file
    pub fn load_custom_theme(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let colors: ThemeColors = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let theme = CustomTheme::new(
            "Custom".to_string(),
            "Custom theme from config".to_string(),
            colors,
        );

        self.custom_theme = Some(theme);
        self.custom_theme_path = Some(path.to_path_buf());
        self.current_theme = ThemeName::Custom;

        Ok(())
    }

    /// Reload custom theme from file (hot-reload)
    pub fn reload_custom_theme(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.custom_theme_path {
            self.load_custom_theme(path.clone())?;
        }
        Ok(())
    }

    /// Get a color from the current theme
    pub fn get_color<F>(&self, getter: F) -> Color
    where
        F: Fn(&dyn Theme) -> Color,
    {
        match self.current_theme {
            ThemeName::Dark => getter(&DarkTheme),
            ThemeName::Light => getter(&LightTheme),
            ThemeName::HighContrast => getter(&HighContrastTheme),
            ThemeName::CatppuccinMocha => getter(&CatppuccinMocha),
            ThemeName::CatppuccinMacchiato => getter(&CatppuccinMacchiato),
            ThemeName::CatppuccinFrappe => getter(&CatppuccinFrappe),
            ThemeName::CatppuccinLatte => getter(&CatppuccinLatte),
            ThemeName::Nord => getter(&NordTheme),
            ThemeName::Custom => {
                if let Some(custom) = &self.custom_theme {
                    getter(custom)
                } else {
                    // Fallback to dark theme
                    getter(&DarkTheme)
                }
            }
        }
    }

    /// Get primary color
    pub fn primary(&self) -> Color {
        self.get_color(|theme| theme.primary())
    }

    /// Get background color
    pub fn background(&self) -> Color {
        self.get_color(|theme| theme.background())
    }

    /// Get foreground color
    pub fn foreground(&self) -> Color {
        self.get_color(|theme| theme.foreground())
    }

    /// Get border color
    pub fn border(&self) -> Color {
        self.get_color(|theme| theme.border())
    }

    /// Get focused border color
    pub fn border_focused(&self) -> Color {
        self.get_color(|theme| theme.border_focused())
    }

    /// Get success color
    pub fn success(&self) -> Color {
        self.get_color(|theme| theme.success())
    }

    /// Get error color
    pub fn error(&self) -> Color {
        self.get_color(|theme| theme.error())
    }

    /// Get warning color
    pub fn warning(&self) -> Color {
        self.get_color(|theme| theme.warning())
    }

    /// Get info color
    pub fn info(&self) -> Color {
        self.get_color(|theme| theme.info())
    }

    /// List all available themes
    pub fn list_themes(&self) -> Vec<String> {
        ThemeName::all()
            .iter()
            .map(|name| name.as_str().to_string())
            .collect()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::new();
        assert_eq!(manager.current_theme_name(), ThemeName::Dark);
    }

    #[test]
    fn test_theme_switching() {
        let mut manager = ThemeManager::new();

        manager.set_theme(ThemeName::Light);
        assert_eq!(manager.current_theme_name(), ThemeName::Light);

        manager.set_theme(ThemeName::Nord);
        assert_eq!(manager.current_theme_name(), ThemeName::Nord);
    }

    #[test]
    fn test_get_colors() {
        let manager = ThemeManager::new();

        // Should get colors from Dark theme
        let primary = manager.primary();
        assert_eq!(primary, Color::Rgb(76, 175, 80)); // Toad green
    }

    #[test]
    fn test_theme_name_parsing() {
        assert_eq!(ThemeName::from_str("dark"), Some(ThemeName::Dark));
        assert_eq!(ThemeName::from_str("Light"), Some(ThemeName::Light));
        assert_eq!(ThemeName::from_str("nord"), Some(ThemeName::Nord));
        assert_eq!(
            ThemeName::from_str("mocha"),
            Some(ThemeName::CatppuccinMocha)
        );
        assert_eq!(ThemeName::from_str("invalid"), None);
    }

    #[test]
    fn test_theme_name_all() {
        let themes = ThemeName::all();
        assert!(themes.len() >= 8); // At least 8 built-in themes
        assert!(themes.contains(&ThemeName::Dark));
        assert!(themes.contains(&ThemeName::Nord));
    }

    #[test]
    fn test_list_themes() {
        let manager = ThemeManager::new();
        let themes = manager.list_themes();
        assert!(themes.len() >= 8);
        assert!(themes.contains(&"Dark".to_string()));
        assert!(themes.contains(&"Nord".to_string()));
    }

    #[test]
    fn test_theme_name_as_str() {
        assert_eq!(ThemeName::Dark.as_str(), "Dark");
        assert_eq!(ThemeName::Nord.as_str(), "Nord");
        assert_eq!(ThemeName::CatppuccinMocha.as_str(), "Catppuccin Mocha");
    }

    #[test]
    fn test_all_theme_colors() {
        let mut manager = ThemeManager::new();

        // Test Dark theme
        manager.set_theme(ThemeName::Dark);
        assert_eq!(manager.background(), Color::Rgb(33, 33, 33));
        assert_eq!(manager.foreground(), Color::Rgb(189, 189, 189));

        // Test Light theme
        manager.set_theme(ThemeName::Light);
        assert_eq!(manager.background(), Color::Rgb(250, 250, 250));

        // Test Nord theme
        manager.set_theme(ThemeName::Nord);
        assert_eq!(manager.background(), Color::Rgb(46, 52, 64));
    }

    #[test]
    fn test_all_color_getters() {
        let manager = ThemeManager::new();

        // Test all color getter methods
        let _ = manager.primary();
        let _ = manager.background();
        let _ = manager.foreground();
        let _ = manager.border();
        let _ = manager.border_focused();
        let _ = manager.success();
        let _ = manager.error();
        let _ = manager.warning();
        let _ = manager.info();
    }

    #[test]
    fn test_semantic_colors() {
        let manager = ThemeManager::new();

        let success = manager.success();
        let error = manager.error();
        let warning = manager.warning();
        let info = manager.info();

        // All should return valid colors
        assert!(matches!(success, Color::Rgb(..)));
        assert!(matches!(error, Color::Rgb(..)));
        assert!(matches!(warning, Color::Rgb(..)));
        assert!(matches!(info, Color::Rgb(..)));
    }

    #[test]
    fn test_all_catppuccin_variants() {
        let mut manager = ThemeManager::new();

        manager.set_theme(ThemeName::CatppuccinMocha);
        assert_eq!(manager.current_theme_name(), ThemeName::CatppuccinMocha);

        manager.set_theme(ThemeName::CatppuccinMacchiato);
        assert_eq!(manager.current_theme_name(), ThemeName::CatppuccinMacchiato);

        manager.set_theme(ThemeName::CatppuccinFrappe);
        assert_eq!(manager.current_theme_name(), ThemeName::CatppuccinFrappe);

        manager.set_theme(ThemeName::CatppuccinLatte);
        assert_eq!(manager.current_theme_name(), ThemeName::CatppuccinLatte);
    }

    #[test]
    fn test_theme_name_parsing_all_variants() {
        // Test all theme names
        assert_eq!(ThemeName::from_str("dark"), Some(ThemeName::Dark));
        assert_eq!(ThemeName::from_str("DARK"), Some(ThemeName::Dark));
        assert_eq!(ThemeName::from_str("light"), Some(ThemeName::Light));
        assert_eq!(
            ThemeName::from_str("high contrast"),
            Some(ThemeName::HighContrast)
        );
        assert_eq!(
            ThemeName::from_str("highcontrast"),
            Some(ThemeName::HighContrast)
        );
        assert_eq!(
            ThemeName::from_str("mocha"),
            Some(ThemeName::CatppuccinMocha)
        );
        assert_eq!(
            ThemeName::from_str("catppuccin mocha"),
            Some(ThemeName::CatppuccinMocha)
        );
        assert_eq!(
            ThemeName::from_str("macchiato"),
            Some(ThemeName::CatppuccinMacchiato)
        );
        assert_eq!(
            ThemeName::from_str("frappe"),
            Some(ThemeName::CatppuccinFrappe)
        );
        assert_eq!(
            ThemeName::from_str("latte"),
            Some(ThemeName::CatppuccinLatte)
        );
        assert_eq!(ThemeName::from_str("nord"), Some(ThemeName::Nord));
        assert_eq!(ThemeName::from_str("custom"), Some(ThemeName::Custom));
    }

    #[test]
    fn test_theme_name_parsing_invalid() {
        assert_eq!(ThemeName::from_str(""), None);
        assert_eq!(ThemeName::from_str("invalid"), None);
        assert_eq!(ThemeName::from_str("123"), None);
        assert_eq!(ThemeName::from_str("dark theme"), None);
    }

    #[test]
    fn test_theme_name_traits() {
        let theme1 = ThemeName::Dark;
        let theme2 = ThemeName::Dark;
        let theme3 = ThemeName::Light;

        // Test Clone
        let cloned = theme1.clone();
        assert_eq!(cloned, theme1);

        // Test PartialEq
        assert_eq!(theme1, theme2);
        assert_ne!(theme1, theme3);

        // Test Debug
        let debug_str = format!("{:?}", theme1);
        assert!(debug_str.contains("Dark"));
    }

    #[test]
    fn test_theme_manager_default() {
        let manager = ThemeManager::default();
        assert_eq!(manager.current_theme_name(), ThemeName::Dark);
    }

    #[test]
    fn test_custom_theme_fallback() {
        let mut manager = ThemeManager::new();

        // Set to Custom but don't load a custom theme
        manager.set_theme(ThemeName::Custom);

        // Should fallback to Dark theme colors
        let bg = manager.background();
        assert_eq!(bg, Color::Rgb(33, 33, 33)); // Dark theme background
    }

    #[test]
    fn test_all_themes_have_unique_names() {
        let themes = ThemeName::all();
        let mut names: Vec<String> = themes.iter().map(|t| t.as_str().to_string()).collect();
        names.sort();
        names.dedup();

        // All theme names should be unique
        assert_eq!(names.len(), themes.len());
    }

    #[test]
    fn test_theme_name_all_count() {
        let themes = ThemeName::all();
        // Should have exactly 8 built-in themes (excluding Custom)
        assert_eq!(themes.len(), 8);
    }

    #[test]
    fn test_list_themes_matches_all() {
        let manager = ThemeManager::new();
        let list = manager.list_themes();
        let all_themes = ThemeName::all();

        assert_eq!(list.len(), all_themes.len());
    }

    #[test]
    fn test_high_contrast_theme() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::HighContrast);

        // High contrast should have pure black background
        assert_eq!(manager.background(), Color::Rgb(0, 0, 0));
        // And pure white foreground
        assert_eq!(manager.foreground(), Color::Rgb(255, 255, 255));
    }

    #[test]
    fn test_theme_switching_preserves_colors() {
        let mut manager = ThemeManager::new();

        // Switch to Nord
        manager.set_theme(ThemeName::Nord);
        let nord_primary = manager.primary();

        // Switch to Light
        manager.set_theme(ThemeName::Light);
        let light_primary = manager.primary();

        // Switch back to Nord
        manager.set_theme(ThemeName::Nord);
        let nord_primary_again = manager.primary();

        // Nord primary should be the same
        assert_eq!(nord_primary, nord_primary_again);
        // But different from Light
        assert_ne!(nord_primary, light_primary);
    }

    #[test]
    fn test_border_colors_differ_from_background() {
        let manager = ThemeManager::new();

        let bg = manager.background();
        let border = manager.border();
        let border_focused = manager.border_focused();

        // Border colors should differ from background
        assert_ne!(bg, border);
        assert_ne!(bg, border_focused);
    }

    // ===== Custom Theme Loading Tests =====
    #[test]
    fn test_load_custom_theme_success() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut manager = ThemeManager::new();

        // Create temporary theme file with all required fields
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
primary = [100, 150, 200]
primary_bright = [120, 170, 220]
primary_dark = [80, 130, 180]
white = [255, 255, 255]
light_gray = [200, 200, 200]
gray = [150, 150, 150]
dark_gray = [100, 100, 100]
darker_gray = [50, 50, 50]
black = [10, 10, 10]
success = [0, 255, 0]
error = [255, 0, 0]
warning = [255, 255, 0]
info = [0, 150, 255]
red = [255, 0, 0]
yellow = [255, 255, 0]
blue = [0, 0, 255]
green = [0, 255, 0]
cyan = [0, 255, 255]
magenta = [255, 0, 255]
background = [10, 10, 10]
foreground = [240, 240, 240]
border = [50, 50, 50]
border_focused = [100, 150, 200]
title = [100, 150, 200]
accent = [100, 150, 200]
"#
        )
        .unwrap();

        // Load custom theme
        let result = manager.load_custom_theme(temp_file.path());
        assert!(result.is_ok());

        // Verify theme was loaded
        assert_eq!(manager.current_theme_name(), ThemeName::Custom);
        assert_eq!(manager.primary(), Color::Rgb(100, 150, 200));
        assert_eq!(manager.background(), Color::Rgb(10, 10, 10));
    }

    #[test]
    fn test_load_custom_theme_file_not_found() {
        let mut manager = ThemeManager::new();

        let result = manager.load_custom_theme("/nonexistent/path/theme.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_custom_theme_invalid_toml() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut manager = ThemeManager::new();

        // Create temporary file with invalid TOML
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "this is not valid toml {{{{").unwrap();

        let result = manager.load_custom_theme(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_reload_custom_theme_without_path() {
        let mut manager = ThemeManager::new();

        // Reload without loading a custom theme first
        let result = manager.reload_custom_theme();
        assert!(result.is_ok()); // Should succeed but do nothing
    }

    #[test]
    fn test_reload_custom_theme_success() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut manager = ThemeManager::new();

        // Create temporary theme file with all fields
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
primary = [100, 100, 100]
primary_bright = [120, 120, 120]
primary_dark = [80, 80, 80]
white = [255, 255, 255]
light_gray = [200, 200, 200]
gray = [150, 150, 150]
dark_gray = [100, 100, 100]
darker_gray = [50, 50, 50]
black = [10, 10, 10]
success = [0, 200, 0]
error = [200, 0, 0]
warning = [200, 200, 0]
info = [0, 100, 200]
red = [200, 0, 0]
yellow = [200, 200, 0]
blue = [0, 0, 200]
green = [0, 200, 0]
cyan = [0, 200, 200]
magenta = [200, 0, 200]
background = [20, 20, 20]
foreground = [220, 220, 220]
border = [60, 60, 60]
border_focused = [100, 100, 100]
title = [100, 100, 100]
accent = [100, 100, 100]
"#
        )
        .unwrap();

        // Load custom theme
        manager.load_custom_theme(temp_file.path()).unwrap();
        let initial_bg = manager.background();

        // Reload should succeed
        let result = manager.reload_custom_theme();
        assert!(result.is_ok());

        // Colors should remain the same
        assert_eq!(manager.background(), initial_bg);
    }

    // ===== get_color() Coverage for All Theme Variants =====
    #[test]
    fn test_get_color_all_themes() {
        let mut manager = ThemeManager::new();

        // Test get_color for each theme
        for theme in ThemeName::all() {
            manager.set_theme(theme);
            let primary = manager.get_color(|t| t.primary());
            assert!(matches!(primary, Color::Rgb(..)));
        }
    }

    #[test]
    fn test_get_color_dark_theme() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::Dark);

        let primary = manager.get_color(|t| t.primary());
        assert_eq!(primary, Color::Rgb(76, 175, 80));
    }

    #[test]
    fn test_get_color_light_theme() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::Light);

        let bg = manager.get_color(|t| t.background());
        assert_eq!(bg, Color::Rgb(250, 250, 250));
    }

    #[test]
    fn test_get_color_high_contrast_theme() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::HighContrast);

        let bg = manager.get_color(|t| t.background());
        let fg = manager.get_color(|t| t.foreground());
        assert_eq!(bg, Color::Rgb(0, 0, 0));
        assert_eq!(fg, Color::Rgb(255, 255, 255));
    }

    #[test]
    fn test_get_color_catppuccin_mocha() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::CatppuccinMocha);

        let primary = manager.get_color(|t| t.primary());
        assert!(matches!(primary, Color::Rgb(..)));
    }

    #[test]
    fn test_get_color_catppuccin_macchiato() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::CatppuccinMacchiato);

        let primary = manager.get_color(|t| t.primary());
        assert!(matches!(primary, Color::Rgb(..)));
    }

    #[test]
    fn test_get_color_catppuccin_frappe() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::CatppuccinFrappe);

        let primary = manager.get_color(|t| t.primary());
        assert!(matches!(primary, Color::Rgb(..)));
    }

    #[test]
    fn test_get_color_catppuccin_latte() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::CatppuccinLatte);

        let primary = manager.get_color(|t| t.primary());
        assert!(matches!(primary, Color::Rgb(..)));
    }

    #[test]
    fn test_get_color_nord_theme() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeName::Nord);

        let primary = manager.get_color(|t| t.primary());
        assert!(matches!(primary, Color::Rgb(..)));
    }

    #[test]
    fn test_get_color_custom_with_loaded_theme() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut manager = ThemeManager::new();

        // Create and load custom theme with all fields
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
primary = [123, 45, 67]
primary_bright = [143, 65, 87]
primary_dark = [103, 25, 47]
white = [255, 255, 255]
light_gray = [200, 200, 200]
gray = [150, 150, 150]
dark_gray = [100, 100, 100]
darker_gray = [50, 50, 50]
black = [10, 10, 10]
success = [0, 220, 0]
error = [220, 0, 0]
warning = [220, 220, 0]
info = [0, 120, 220]
red = [220, 0, 0]
yellow = [220, 220, 0]
blue = [0, 0, 220]
green = [0, 220, 0]
cyan = [0, 220, 220]
magenta = [220, 0, 220]
background = [15, 15, 15]
foreground = [230, 230, 230]
border = [55, 55, 55]
border_focused = [123, 45, 67]
title = [123, 45, 67]
accent = [123, 45, 67]
"#
        )
        .unwrap();

        manager.load_custom_theme(temp_file.path()).unwrap();

        // Test get_color with custom theme
        let primary = manager.get_color(|t| t.primary());
        assert_eq!(primary, Color::Rgb(123, 45, 67));
    }

    // ===== ThemeName Copy Trait Test =====
    #[test]
    fn test_theme_name_copy() {
        let theme1 = ThemeName::Dark;
        let theme2 = theme1; // Copy, not move

        // Both should be usable
        assert_eq!(theme1, ThemeName::Dark);
        assert_eq!(theme2, ThemeName::Dark);
    }

    // ===== Serialize/Deserialize Tests =====
    #[test]
    fn test_theme_name_serialize() {
        let theme = ThemeName::CatppuccinMocha;
        let serialized = serde_json::to_string(&theme).unwrap();
        assert!(serialized.contains("CatppuccinMocha"));
    }

    #[test]
    fn test_theme_name_deserialize() {
        let json = "\"Dark\"";
        let theme: ThemeName = serde_json::from_str(json).unwrap();
        assert_eq!(theme, ThemeName::Dark);
    }

    // ===== All Color Getters for Each Theme =====
    #[test]
    fn test_all_getters_for_each_theme() {
        let mut manager = ThemeManager::new();

        for theme in ThemeName::all() {
            manager.set_theme(theme);

            // Call all getter methods
            let _ = manager.primary();
            let _ = manager.background();
            let _ = manager.foreground();
            let _ = manager.border();
            let _ = manager.border_focused();
            let _ = manager.success();
            let _ = manager.error();
            let _ = manager.warning();
            let _ = manager.info();
        }
    }

    // ===== Custom Theme Path Tracking =====
    #[test]
    fn test_custom_theme_path_stored() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut manager = ThemeManager::new();

        // Create temporary theme file with all fields
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
primary = [50, 50, 50]
primary_bright = [70, 70, 70]
primary_dark = [30, 30, 30]
white = [255, 255, 255]
light_gray = [200, 200, 200]
gray = [150, 150, 150]
dark_gray = [100, 100, 100]
darker_gray = [50, 50, 50]
black = [5, 5, 5]
success = [0, 255, 0]
error = [255, 0, 0]
warning = [255, 255, 0]
info = [0, 150, 255]
red = [255, 0, 0]
yellow = [255, 255, 0]
blue = [0, 0, 255]
green = [0, 255, 0]
cyan = [0, 255, 255]
magenta = [255, 0, 255]
background = [5, 5, 5]
foreground = [250, 250, 250]
border = [40, 40, 40]
border_focused = [50, 50, 50]
title = [50, 50, 50]
accent = [50, 50, 50]
"#
        )
        .unwrap();

        // Load custom theme
        manager.load_custom_theme(temp_file.path()).unwrap();

        // Path should be stored (verified by successful reload)
        assert!(manager.reload_custom_theme().is_ok());
    }

    // ===== Custom Theme Name Test =====
    #[test]
    fn test_custom_theme_name_as_str() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut manager = ThemeManager::new();

        // Create temporary theme file
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
primary = [100, 100, 100]
primary_bright = [120, 120, 120]
primary_dark = [80, 80, 80]
white = [255, 255, 255]
light_gray = [200, 200, 200]
gray = [150, 150, 150]
dark_gray = [100, 100, 100]
darker_gray = [50, 50, 50]
black = [10, 10, 10]
success = [0, 255, 0]
error = [255, 0, 0]
warning = [255, 255, 0]
info = [0, 150, 255]
red = [255, 0, 0]
yellow = [255, 255, 0]
blue = [0, 0, 255]
green = [0, 255, 0]
cyan = [0, 255, 255]
magenta = [255, 0, 255]
background = [10, 10, 10]
foreground = [240, 240, 240]
border = [50, 50, 50]
border_focused = [100, 100, 100]
title = [100, 100, 100]
accent = [100, 100, 100]
"#
        )
        .unwrap();

        // Load custom theme
        manager.load_custom_theme(temp_file.path()).unwrap();

        // Verify custom theme name
        let theme_name = manager.current_theme_name();
        assert_eq!(theme_name, ThemeName::Custom);
        assert_eq!(theme_name.as_str(), "Custom");
    }
}

    // ===== NO_COLOR and Auto-Detection Tests =====
    #[test]
    fn test_detect_no_color_when_set() {
        std::env::set_var("NO_COLOR", "1");
        assert!(ThemeManager::detect_no_color());
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn test_detect_no_color_when_not_set() {
        std::env::remove_var("NO_COLOR");
        assert!(!ThemeManager::detect_no_color());
    }

    #[test]
    fn test_is_no_color() {
        std::env::set_var("NO_COLOR", "1");
        let manager = ThemeManager::new();
        assert!(manager.is_no_color());
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn test_set_no_color() {
        std::env::remove_var("NO_COLOR");
        let mut manager = ThemeManager::new();
        assert!(!manager.is_no_color());

        manager.set_no_color(true);
        assert!(manager.is_no_color());

        manager.set_no_color(false);
        assert!(!manager.is_no_color());
    }

    #[test]
    fn test_auto_detect_theme_colorfgbg_dark() {
        std::env::set_var("COLORFGBG", "15;0");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark);
        std::env::remove_var("COLORFGBG");
    }

    #[test]
    fn test_auto_detect_theme_colorfgbg_light() {
        std::env::set_var("COLORFGBG", "0;15");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Light);
        std::env::remove_var("COLORFGBG");
    }

    #[test]
    fn test_auto_detect_theme_term_program_iterm() {
        std::env::remove_var("COLORFGBG");
        std::env::set_var("TERM_PROGRAM", "iTerm.app");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark);
        std::env::remove_var("TERM_PROGRAM");
    }

    #[test]
    fn test_auto_detect_theme_term_program_vscode_light() {
        std::env::remove_var("COLORFGBG");
        std::env::set_var("TERM_PROGRAM", "vscode");
        std::env::set_var("VSCODE_THEME_VARIANT", "light");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Light);
        std::env::remove_var("TERM_PROGRAM");
        std::env::remove_var("VSCODE_THEME_VARIANT");
    }

    #[test]
    fn test_auto_detect_theme_term_program_vscode_dark() {
        std::env::remove_var("COLORFGBG");
        std::env::set_var("TERM_PROGRAM", "vscode");
        std::env::set_var("VSCODE_THEME_VARIANT", "dark");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark);
        std::env::remove_var("TERM_PROGRAM");
        std::env::remove_var("VSCODE_THEME_VARIANT");
    }

    #[test]
    fn test_auto_detect_theme_default_fallback() {
        std::env::remove_var("COLORFGBG");
        std::env::remove_var("TERM_PROGRAM");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark); // Should default to Dark
    }

    #[test]
    fn test_with_theme_constructor() {
        std::env::remove_var("NO_COLOR");
        let manager = ThemeManager::with_theme(ThemeName::Nord);
        assert_eq!(manager.current_theme_name(), ThemeName::Nord);
    }

    #[test]
    fn test_with_theme_respects_no_color() {
        std::env::set_var("NO_COLOR", "1");
        let manager = ThemeManager::with_theme(ThemeName::Light);
        assert_eq!(manager.current_theme_name(), ThemeName::Light);
        assert!(manager.is_no_color());
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn test_new_auto_detects_theme() {
        std::env::set_var("COLORFGBG", "0;15");
        std::env::remove_var("NO_COLOR");
        let manager = ThemeManager::new();
        assert_eq!(manager.current_theme_name(), ThemeName::Light);
        std::env::remove_var("COLORFGBG");
    }

    #[test]
    fn test_colorfgbg_parsing_edge_cases() {
        std::env::set_var("COLORFGBG", "invalid;7");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark);
        std::env::remove_var("COLORFGBG");
    }

    #[test]
    fn test_colorfgbg_missing_background() {
        std::env::set_var("COLORFGBG", "15");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark); // Should fallback
        std::env::remove_var("COLORFGBG");
    }

    #[test]
    fn test_term_program_hyper() {
        std::env::remove_var("COLORFGBG");
        std::env::set_var("TERM_PROGRAM", "Hyper");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark);
        std::env::remove_var("TERM_PROGRAM");
    }

    #[test]
    fn test_term_program_terminal_app() {
        std::env::remove_var("COLORFGBG");
        std::env::set_var("TERM_PROGRAM", "Terminal.app");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Dark);
        std::env::remove_var("TERM_PROGRAM");
    }

    #[test]
    fn test_colorfgbg_prioritized_over_term_program() {
        std::env::set_var("COLORFGBG", "0;15");
        std::env::set_var("TERM_PROGRAM", "iTerm.app");
        let theme = ThemeManager::auto_detect_theme();
        assert_eq!(theme, ThemeName::Light); // COLORFGBG wins
        std::env::remove_var("COLORFGBG");
        std::env::remove_var("TERM_PROGRAM");
    }

    #[test]
    fn test_no_color_empty_string() {
        std::env::set_var("NO_COLOR", "");
        assert!(ThemeManager::detect_no_color()); // ANY value enables NO_COLOR
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn test_manager_no_color_field_initialized() {
        std::env::set_var("NO_COLOR", "1");
        let manager = ThemeManager::new();
        assert_eq!(manager.no_color_mode, true);
        std::env::remove_var("NO_COLOR");
    }
