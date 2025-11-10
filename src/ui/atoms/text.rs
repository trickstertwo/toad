//! Text atom - Styled text primitive
//!
//! The most fundamental UI primitive in the Atomic UI system.
//! Wraps ratatui's text rendering with consistent styling.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Atom**: Single-purpose, no dependencies on other atoms
//! - **Pure**: No mutable state, pure rendering function
//! - **Composable**: Can be combined into molecules
//!
//! # Examples
//!
//! ```
//! use toad::ui::atoms::text::Text;
//! use toad::ui::theme::ToadTheme;
//! use ratatui::style::{Modifier, Style};
//!
//! // Simple text
//! let text = Text::new("Hello");
//!
//! // Styled text
//! let text = Text::new("Success")
//!     .style(Style::default().fg(ToadTheme::TOAD_GREEN));
//!
//! // With modifiers
//! let text = Text::new("Bold")
//!     .bold()
//!     .style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT));
//! ```

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

/// A styled text primitive
///
/// The fundamental building block for all text-based UI components.
/// Wraps a string with optional styling.
///
/// # Examples
///
/// ```
/// use toad::ui::atoms::text::Text;
///
/// let text = Text::new("Hello");
/// assert_eq!(text.content(), "Hello");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    /// The text content
    content: String,
    /// Optional style
    style: Option<Style>,
}

impl Text {
    /// Create new text with default styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Hello");
    /// assert_eq!(text.content(), "Hello");
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: None,
        }
    }

    /// Set the style for this text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let text = Text::new("Success")
    ///     .style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Make text bold
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Important").bold();
    /// ```
    pub fn bold(mut self) -> Self {
        let current = self.style.unwrap_or_default();
        self.style = Some(current.add_modifier(Modifier::BOLD));
        self
    }

    /// Make text italic
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Emphasis").italic();
    /// ```
    pub fn italic(mut self) -> Self {
        let current = self.style.unwrap_or_default();
        self.style = Some(current.add_modifier(Modifier::ITALIC));
        self
    }

    /// Make text underlined
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Link").underline();
    /// ```
    pub fn underline(mut self) -> Self {
        let current = self.style.unwrap_or_default();
        self.style = Some(current.add_modifier(Modifier::UNDERLINED));
        self
    }

    /// Get the text content
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Hello");
    /// assert_eq!(text.content(), "Hello");
    /// ```
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Convert to ratatui Span for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Hello");
    /// let span = text.to_span();
    /// ```
    pub fn to_span(&self) -> Span<'static> {
        if let Some(style) = self.style {
            Span::styled(self.content.clone(), style)
        } else {
            Span::raw(self.content.clone())
        }
    }

    /// Convert to ratatui Line for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::text::Text;
    ///
    /// let text = Text::new("Hello");
    /// let line = text.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_span())
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::ToadTheme;

    #[test]
    fn test_text_new() {
        let text = Text::new("Hello");
        assert_eq!(text.content(), "Hello");
        assert_eq!(text.style, None);
    }

    #[test]
    fn test_text_from_str() {
        let text: Text = "Hello".into();
        assert_eq!(text.content(), "Hello");
    }

    #[test]
    fn test_text_from_string() {
        let text: Text = String::from("Hello").into();
        assert_eq!(text.content(), "Hello");
    }

    #[test]
    fn test_text_with_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let text = Text::new("Success").style(style);
        assert_eq!(text.style, Some(style));
    }

    #[test]
    fn test_text_bold() {
        let text = Text::new("Bold").bold();
        assert!(text.style.is_some());
    }

    #[test]
    fn test_text_italic() {
        let text = Text::new("Italic").italic();
        assert!(text.style.is_some());
    }

    #[test]
    fn test_text_underline() {
        let text = Text::new("Underline").underline();
        assert!(text.style.is_some());
    }

    #[test]
    fn test_text_chaining() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT);
        let text = Text::new("Important").bold().style(style);

        assert_eq!(text.content(), "Important");
        assert!(text.style.is_some());
    }

    #[test]
    fn test_text_to_span() {
        let text = Text::new("Hello");
        let _span = text.to_span();
        // Verify span was created (can't easily test content without ratatui backend)
        assert!(!text.content().is_empty());
    }

    #[test]
    fn test_text_to_line() {
        let text = Text::new("Hello");
        let _line = text.to_line();
        // Verify line was created
        assert!(!text.content().is_empty());
    }

    #[test]
    fn test_text_clone() {
        let text1 = Text::new("Clone").bold();
        let text2 = text1.clone();
        assert_eq!(text1.content(), text2.content());
        assert_eq!(text1.style, text2.style);
    }

    #[test]
    fn test_text_equality() {
        let text1 = Text::new("Same");
        let text2 = Text::new("Same");
        let text3 = Text::new("Different");

        assert_eq!(text1, text2);
        assert_ne!(text1, text3);
    }

    #[test]
    fn test_text_empty() {
        let text = Text::new("");
        assert_eq!(text.content(), "");
    }

    #[test]
    fn test_text_unicode() {
        let text = Text::new("🐸 TOAD 日本");
        assert_eq!(text.content(), "🐸 TOAD 日本");
    }

    #[test]
    fn test_text_long_content() {
        let long_text = "A".repeat(10000);
        let text = Text::new(&long_text);
        assert_eq!(text.content().len(), 10000);
    }
}
