//! Icon atom - Nerd Font icon primitive
//!
//! Wraps the NerdFonts API with consistent styling and composability.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Atom**: Single-purpose icon primitive
//! - **Pure**: No mutable state, builder pattern
//! - **Composable**: Used by molecules and organisms
//!
//! # Examples
//!
//! ```
//! use toad::ui::atoms::icon::Icon;
//! use toad::ui::nerd_fonts::UiIcon;
//! use toad::ui::theme::ToadTheme;
//! use ratatui::style::Style;
//!
//! // Simple icon
//! let icon = Icon::ui(UiIcon::Success);
//!
//! // Styled icon
//! let icon = Icon::ui(UiIcon::Error)
//!     .style(Style::default().fg(ToadTheme::RED));
//!
//! // File icon
//! let icon = Icon::file("main.rs");
//! ```

use crate::ui::nerd_fonts::{GitStatus, NerdFonts, UiIcon};
use crate::ui::atoms::text::Text;
use ratatui::style::Style;
use std::path::Path;

/// An icon primitive
///
/// Wraps Nerd Font icons with optional styling.
/// Used as a building block for status indicators, file trees, and UI elements.
///
/// # Examples
///
/// ```
/// use toad::ui::atoms::icon::Icon;
/// use toad::ui::nerd_fonts::UiIcon;
///
/// let icon = Icon::ui(UiIcon::Success);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Icon {
    /// The icon character
    icon_char: String,
    /// Optional style
    style: Option<Style>,
}

impl Icon {
    /// Create icon from raw character
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    ///
    /// let icon = Icon::new("🐸");
    /// assert_eq!(icon.char_str(), "🐸");
    /// ```
    pub fn new(icon_char: impl Into<String>) -> Self {
        Self {
            icon_char: icon_char.into(),
            style: None,
        }
    }

    /// Create icon for a file based on its path
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    ///
    /// let icon = Icon::file("main.rs");
    /// // Returns Rust icon
    /// ```
    pub fn file<P: AsRef<Path>>(path: P) -> Self {
        Self::new(NerdFonts::file_icon(path))
    }

    /// Create icon for a folder
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    ///
    /// let icon = Icon::folder("src", false);
    /// // Returns closed folder icon
    /// ```
    pub fn folder(name: impl AsRef<str>, open: bool) -> Self {
        Self::new(NerdFonts::folder_icon(name.as_ref(), open))
    }

    /// Create icon for git status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    /// use toad::ui::nerd_fonts::GitStatus;
    ///
    /// let icon = Icon::git_status(GitStatus::Modified);
    /// ```
    pub fn git_status(status: GitStatus) -> Self {
        Self::new(NerdFonts::git_status_icon(status))
    }

    /// Create icon for UI element
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    /// use toad::ui::nerd_fonts::UiIcon;
    ///
    /// let icon = Icon::ui(UiIcon::Success);
    /// ```
    pub fn ui(icon: UiIcon) -> Self {
        Self::new(NerdFonts::ui_icon(icon))
    }

    /// Create icon for programming language
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    ///
    /// let icon = Icon::language("rust");
    /// ```
    pub fn language(language: impl AsRef<str>) -> Self {
        Self::new(NerdFonts::language_icon(language.as_ref()))
    }

    /// Set the style for this icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    /// use toad::ui::nerd_fonts::UiIcon;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let icon = Icon::ui(UiIcon::Error)
    ///     .style(Style::default().fg(ToadTheme::RED));
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get the icon character as a string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    ///
    /// let icon = Icon::new("🐸");
    /// assert_eq!(icon.char_str(), "🐸");
    /// ```
    pub fn char_str(&self) -> &str {
        &self.icon_char
    }

    /// Convert to Text atom
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::icon::Icon;
    /// use toad::ui::nerd_fonts::UiIcon;
    ///
    /// let icon = Icon::ui(UiIcon::Success);
    /// let text = icon.to_text();
    /// ```
    pub fn to_text(&self) -> Text {
        let mut text = Text::new(self.icon_char.clone());
        if let Some(style) = self.style {
            text = text.style(style);
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::ToadTheme;

    #[test]
    fn test_icon_new() {
        let icon = Icon::new("🐸");
        assert_eq!(icon.char_str(), "🐸");
        assert_eq!(icon.style, None);
    }

    #[test]
    fn test_icon_file() {
        let icon = Icon::file("main.rs");
        assert_eq!(icon.char_str(), "");
    }

    #[test]
    fn test_icon_folder() {
        let icon = Icon::folder("src", false);
        assert_eq!(icon.char_str(), "");
    }

    #[test]
    fn test_icon_folder_open() {
        let icon = Icon::folder("src", true);
        assert_eq!(icon.char_str(), "");
    }

    #[test]
    fn test_icon_git_status() {
        let icon = Icon::git_status(GitStatus::Modified);
        assert_eq!(icon.char_str(), "");
    }

    #[test]
    fn test_icon_ui() {
        let icon = Icon::ui(UiIcon::Success);
        assert_eq!(icon.char_str(), "");
    }

    #[test]
    fn test_icon_language() {
        let icon = Icon::language("rust");
        assert_eq!(icon.char_str(), "");
    }

    #[test]
    fn test_icon_with_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let icon = Icon::ui(UiIcon::Success).style(style);
        assert_eq!(icon.style, Some(style));
    }

    #[test]
    fn test_icon_to_text() {
        let icon = Icon::ui(UiIcon::Success);
        let text = icon.to_text();
        assert_eq!(text.content(), "");
    }

    #[test]
    fn test_icon_to_text_with_style() {
        let style = Style::default().fg(ToadTheme::RED);
        let icon = Icon::ui(UiIcon::Error).style(style);
        let text = icon.to_text();
        assert_eq!(text.content(), "");
    }

    #[test]
    fn test_icon_clone() {
        let icon1 = Icon::ui(UiIcon::Warning);
        let icon2 = icon1.clone();
        assert_eq!(icon1.char_str(), icon2.char_str());
    }

    #[test]
    fn test_icon_equality() {
        let icon1 = Icon::new("🐸");
        let icon2 = Icon::new("🐸");
        let icon3 = Icon::new("🦀");

        assert_eq!(icon1, icon2);
        assert_ne!(icon1, icon3);
    }

    #[test]
    fn test_icon_all_git_statuses() {
        Icon::git_status(GitStatus::Unmodified);
        Icon::git_status(GitStatus::Modified);
        Icon::git_status(GitStatus::Added);
        Icon::git_status(GitStatus::Deleted);
        Icon::git_status(GitStatus::Renamed);
        Icon::git_status(GitStatus::Copied);
        Icon::git_status(GitStatus::Untracked);
        Icon::git_status(GitStatus::Ignored);
        Icon::git_status(GitStatus::Conflicted);
    }

    #[test]
    fn test_icon_common_ui_icons() {
        Icon::ui(UiIcon::Error);
        Icon::ui(UiIcon::Warning);
        Icon::ui(UiIcon::Info);
        Icon::ui(UiIcon::Success);
        Icon::ui(UiIcon::Loading);
        Icon::ui(UiIcon::Search);
        Icon::ui(UiIcon::Edit);
    }

    #[test]
    fn test_icon_special_folders() {
        // Just verify special folders create icons without panicking
        let _git_icon = Icon::folder(".git", false);
        let _node_icon = Icon::folder("node_modules", false);
        let _target_icon = Icon::folder("target", false);
        let _src_icon = Icon::folder("src", false);
    }
}
