/// Theme manager for loading, storing, and hot-reloading themes

use super::{
    builtin::{DarkTheme, HighContrastTheme, LightTheme},
    catppuccin::{CatppuccinFrappe, CatppuccinLatte, CatppuccinMacchiato, CatppuccinMocha},
    nord::NordTheme,
    CustomTheme, Theme, ThemeColors,
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
}

impl ThemeManager {
    /// Create a new theme manager with default theme
    pub fn new() -> Self {
        Self {
            current_theme: ThemeName::Dark,
            custom_theme: None,
            custom_theme_path: None,
        }
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
        assert_eq!(ThemeName::from_str("mocha"), Some(ThemeName::CatppuccinMocha));
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
}
