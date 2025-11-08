//! Configuration management
//!
//! Handles loading, parsing, and managing application configuration from TOML files.
//!
//! # Examples
//!
//! ```
//! use toad::config::Config;
//!
//! // Load default configuration
//! let config = Config::default();
//! assert_eq!(config.ui.theme, "default");
//! ```
//!
//! # Configuration File Format
//!
//! The configuration file is in TOML format and typically located at
//! `~/.config/toad/config.toml` or `$XDG_CONFIG_HOME/toad/config.toml`.
//!
//! Example configuration:
//!
//! ```toml
//! [ui]
//! theme = "default"
//! show_line_numbers = true
//! fps = 60
//!
//! [editor]
//! tab_width = 4
//! auto_save = false
//!
//! [ai]
//! model = "claude-sonnet-4.5"
//! max_tokens = 4096
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Main application configuration
///
/// # Examples
///
/// ```
/// use toad::config::Config;
///
/// let config = Config::default();
/// assert_eq!(config.ui.theme, "default");
/// assert_eq!(config.editor.tab_width, 4);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    /// UI-related settings
    #[serde(default)]
    pub ui: UiConfig,
    /// Editor settings
    #[serde(default)]
    pub editor: EditorConfig,
    /// AI model settings
    #[serde(default)]
    pub ai: AiConfig,
}

impl Config {
    /// Load configuration from a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::config::Config;
    /// use std::path::Path;
    ///
    /// let config = Config::load(Path::new("config.toml")).unwrap();
    /// ```
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Save configuration to a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::config::Config;
    /// use std::path::Path;
    ///
    /// let config = Config::default();
    /// config.save(Path::new("config.toml")).unwrap();
    /// ```
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        std::fs::write(path, contents)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Get the default configuration file path
    ///
    /// Returns `~/.config/toad/config.toml` on Unix-like systems,
    /// or `%APPDATA%\toad\config.toml` on Windows.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::config::Config;
    ///
    /// let path = Config::default_path();
    /// assert!(path.ends_with("toad/config.toml") || path.ends_with("toad\\config.toml"));
    /// ```
    pub fn default_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        } else {
            std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                    PathBuf::from(home).join(".config")
                })
        };

        config_dir.join("toad").join("config.toml")
    }

    /// Load configuration from default path, or return default config if file doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::config::Config;
    ///
    /// let config = Config::load_or_default();
    /// assert_eq!(config.ui.theme, "default");
    /// ```
    pub fn load_or_default() -> Self {
        let path = Self::default_path();

        if path.exists() {
            Self::load(&path).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

/// UI configuration settings
///
/// # Examples
///
/// ```
/// use toad::config::UiConfig;
///
/// let ui = UiConfig::default();
/// assert_eq!(ui.theme, "default");
/// assert_eq!(ui.fps, 60);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme name (e.g., "default", "catppuccin", "nord")
    pub theme: String,
    /// Show line numbers in editor
    pub show_line_numbers: bool,
    /// Target frames per second
    pub fps: u16,
    /// Show welcome screen on startup
    pub show_welcome: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            show_line_numbers: true,
            fps: 60,
            show_welcome: true,
        }
    }
}

/// Editor configuration settings
///
/// # Examples
///
/// ```
/// use toad::config::EditorConfig;
///
/// let editor = EditorConfig::default();
/// assert_eq!(editor.tab_width, 4);
/// assert!(!editor.auto_save);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Tab width in spaces
    pub tab_width: usize,
    /// Use spaces instead of tabs
    pub use_spaces: bool,
    /// Automatically save files
    pub auto_save: bool,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval: u64,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_width: 4,
            use_spaces: true,
            auto_save: false,
            auto_save_interval: 30,
        }
    }
}

/// AI model configuration settings
///
/// # Examples
///
/// ```
/// use toad::config::AiConfig;
///
/// let ai = AiConfig::default();
/// assert_eq!(ai.model, "claude-sonnet-4.5");
/// assert_eq!(ai.max_tokens, 4096);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// AI model identifier
    pub model: String,
    /// Maximum tokens per request
    pub max_tokens: usize,
    /// Temperature for sampling (0.0 - 1.0)
    pub temperature: f32,
    /// Enable streaming responses
    pub streaming: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model: "claude-sonnet-4.5".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
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
        assert_eq!(config.ui.theme, "default");
        assert_eq!(config.editor.tab_width, 4);
        assert_eq!(config.ai.model, "claude-sonnet-4.5");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();

        assert!(toml_str.contains("[ui]"));
        assert!(toml_str.contains("[editor]"));
        assert!(toml_str.contains("[ai]"));
        assert!(toml_str.contains("theme = \"default\""));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [ui]
            theme = "catppuccin"
            show_line_numbers = false
            fps = 120
            show_welcome = false

            [editor]
            tab_width = 2
            use_spaces = false
            auto_save = true
            auto_save_interval = 60

            [ai]
            model = "claude-opus"
            max_tokens = 8192
            temperature = 0.5
            streaming = false
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.ui.theme, "catppuccin");
        assert_eq!(config.ui.fps, 120);
        assert_eq!(config.editor.tab_width, 2);
        assert!(config.editor.auto_save);
        assert_eq!(config.ai.model, "claude-opus");
        assert_eq!(config.ai.max_tokens, 8192);
    }

    #[test]
    fn test_ui_config_default() {
        let ui = UiConfig::default();
        assert_eq!(ui.theme, "default");
        assert!(ui.show_line_numbers);
        assert_eq!(ui.fps, 60);
    }

    #[test]
    fn test_editor_config_default() {
        let editor = EditorConfig::default();
        assert_eq!(editor.tab_width, 4);
        assert!(editor.use_spaces);
        assert!(!editor.auto_save);
    }

    #[test]
    fn test_ai_config_default() {
        let ai = AiConfig::default();
        assert_eq!(ai.model, "claude-sonnet-4.5");
        assert_eq!(ai.max_tokens, 4096);
        assert_eq!(ai.temperature, 0.7);
    }

    #[test]
    fn test_default_path() {
        let path = Config::default_path();
        let path_str = path.to_string_lossy();

        // Should contain "toad" and "config.toml"
        assert!(path_str.contains("toad"));
        assert!(path_str.contains("config.toml"));
    }

    #[test]
    fn test_load_or_default() {
        // Should return default config when file doesn't exist
        let config = Config::load_or_default();
        assert_eq!(config.ui.theme, "default");
    }
}
