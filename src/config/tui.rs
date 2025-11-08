/// TUI-specific configuration
///
/// This module contains configuration for the terminal user interface,
/// including theming, keybindings, and layout preferences.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main TUI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// UI-specific settings
    pub ui: UiConfig,

    /// Editor-specific settings
    pub editor: EditorConfig,

    /// AI-specific settings
    pub ai: AiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ui: UiConfig::default(),
            editor: EditorConfig::default(),
            ai: AiConfig::default(),
        }
    }
}

impl Config {
    /// Load config from a TOML file
    pub fn load_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save config to a TOML file
    pub fn save_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Get default config file path
    pub fn default_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("toad").join("config.toml")
        } else {
            PathBuf::from(".toad.toml")
        }
    }

    /// Load config from default location or create default
    pub fn load_or_default() -> Self {
        let path = Self::default_path();
        Self::load_from_file(&path).unwrap_or_default()
    }
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme name
    pub theme: String,

    /// Enable mouse support
    pub enable_mouse: bool,

    /// Show line numbers
    pub show_line_numbers: bool,

    /// Tab size for indentation
    pub tab_size: usize,

    /// Use spaces instead of tabs
    pub use_spaces: bool,

    /// Frame rate (FPS)
    pub fps: u8,

    /// Enable animations
    pub animations: bool,

    /// Enable Vim-style keybindings
    pub vim_mode: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "toad".to_string(),
            enable_mouse: true,
            show_line_numbers: true,
            tab_size: 4,
            use_spaces: true,
            fps: 60,
            animations: true,
            vim_mode: false,
        }
    }
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Auto-save on focus lost
    pub auto_save: bool,

    /// Format on save
    pub format_on_save: bool,

    /// Trim trailing whitespace
    pub trim_whitespace: bool,

    /// Insert final newline
    pub insert_final_newline: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            auto_save: false,
            format_on_save: false,
            trim_whitespace: true,
            insert_final_newline: true,
        }
    }
}

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Model to use
    pub model: String,

    /// Temperature for responses
    pub temperature: f32,

    /// Maximum tokens in response
    pub max_tokens: usize,

    /// Enable streaming responses
    pub streaming: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model: "claude-sonnet-4.5".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            streaming: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.ui.theme, "toad");
        assert_eq!(config.ai.model, "claude-sonnet-4.5");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml).unwrap();
        assert_eq!(config.ui.theme, deserialized.ui.theme);
    }
}
