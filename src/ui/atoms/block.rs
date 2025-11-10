//! Block atom - Bordered container primitive
//!
//! A fundamental container widget that wraps content with borders and optional titles.
//! Wraps ratatui's Block widget with consistent styling and theming.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Atom**: Single-purpose bordered container
//! - **Pure**: No mutable state, builder pattern
//! - **Composable**: Used by molecules and organisms
//!
//! # Examples
//!
//! ```
//! use toad::ui::atoms::block::Block;
//! use toad::ui::theme::ToadTheme;
//! use ratatui::style::Style;
//!
//! // Simple block with borders
//! let block = Block::new();
//!
//! // Block with title
//! let block = Block::new()
//!     .title("Evaluation Progress");
//!
//! // Styled block
//! let block = Block::new()
//!     .title("Success")
//!     .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));
//! ```

use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    widgets::{Block as RatatuiBlock, Borders},
};

/// A bordered container primitive
///
/// Wraps ratatui's Block widget with consistent styling.
/// Used as the foundation for panels, cards, and other contained widgets.
///
/// # Examples
///
/// ```
/// use toad::ui::atoms::block::Block;
///
/// let block = Block::new()
///     .title("My Panel")
///     .borders(ratatui::widgets::Borders::ALL);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// Optional title text
    title: Option<String>,
    /// Which borders to show
    borders: Borders,
    /// Border style
    border_style: Option<Style>,
    /// Block background style
    style: Option<Style>,
}

impl Block {
    /// Create a new block with default borders (ALL)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::new();
    /// ```
    pub fn new() -> Self {
        Self {
            title: None,
            borders: Borders::ALL,
            border_style: None,
            style: None,
        }
    }

    /// Set the block title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::new().title("My Title");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set which borders to display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    /// use ratatui::widgets::Borders;
    ///
    /// let block = Block::new().borders(Borders::TOP | Borders::BOTTOM);
    /// ```
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    /// Set the border style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let block = Block::new()
    ///     .border_style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    /// Set the block background style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let block = Block::new()
    ///     .style(Style::default().bg(ToadTheme::BLACK));
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get the title text if set
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::new().title("Test");
    /// assert_eq!(block.title_text(), Some("Test"));
    /// ```
    pub fn title_text(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Convert to ratatui Block for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::new().title("Test");
    /// let ratatui_block = block.to_ratatui();
    /// ```
    pub fn to_ratatui(&self) -> RatatuiBlock<'static> {
        let mut block = RatatuiBlock::default().borders(self.borders);

        if let Some(ref title) = self.title {
            block = block.title(title.clone());
        }

        if let Some(border_style) = self.border_style {
            block = block.border_style(border_style);
        }

        if let Some(style) = self.style {
            block = block.style(style);
        }

        block
    }

    /// Create a themed block with TOAD green borders
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::themed("Panel Title");
    /// ```
    pub fn themed(title: impl Into<String>) -> Self {
        Self::new()
            .title(title)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK))
    }

    /// Create a success-themed block (bright green)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::success("✅ Complete");
    /// ```
    pub fn success(title: impl Into<String>) -> Self {
        Self::new()
            .title(title)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT))
            .style(Style::default().bg(ToadTheme::BLACK))
    }

    /// Create an error-themed block (red)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::error("❌ Failed");
    /// ```
    pub fn error(title: impl Into<String>) -> Self {
        Self::new()
            .title(title)
            .border_style(Style::default().fg(ToadTheme::RED))
            .style(Style::default().bg(ToadTheme::BLACK))
    }

    /// Create a warning-themed block (yellow)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::block::Block;
    ///
    /// let block = Block::warning("⚠ Warning");
    /// ```
    pub fn warning(title: impl Into<String>) -> Self {
        Self::new()
            .title(title)
            .border_style(Style::default().fg(ToadTheme::YELLOW))
            .style(Style::default().bg(ToadTheme::BLACK))
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_new() {
        let block = Block::new();
        assert_eq!(block.title_text(), None);
        assert_eq!(block.borders, Borders::ALL);
        assert_eq!(block.border_style, None);
        assert_eq!(block.style, None);
    }

    #[test]
    fn test_block_default() {
        let block = Block::default();
        assert_eq!(block.title_text(), None);
        assert_eq!(block.borders, Borders::ALL);
    }

    #[test]
    fn test_block_with_title() {
        let block = Block::new().title("Test Title");
        assert_eq!(block.title_text(), Some("Test Title"));
    }

    #[test]
    fn test_block_with_borders() {
        let block = Block::new().borders(Borders::TOP | Borders::BOTTOM);
        assert_eq!(block.borders, Borders::TOP | Borders::BOTTOM);
    }

    #[test]
    fn test_block_with_border_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let block = Block::new().border_style(style);
        assert_eq!(block.border_style, Some(style));
    }

    #[test]
    fn test_block_with_style() {
        let style = Style::default().bg(ToadTheme::BLACK);
        let block = Block::new().style(style);
        assert_eq!(block.style, Some(style));
    }

    #[test]
    fn test_block_chaining() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let block = Block::new()
            .title("Chained")
            .borders(Borders::ALL)
            .border_style(style);

        assert_eq!(block.title_text(), Some("Chained"));
        assert_eq!(block.borders, Borders::ALL);
        assert_eq!(block.border_style, Some(style));
    }

    #[test]
    fn test_block_themed() {
        let block = Block::themed("Themed Block");
        assert_eq!(block.title_text(), Some("Themed Block"));
        assert!(block.border_style.is_some());
        assert!(block.style.is_some());
    }

    #[test]
    fn test_block_success() {
        let block = Block::success("Success Block");
        assert_eq!(block.title_text(), Some("Success Block"));
        assert!(block.border_style.is_some());
    }

    #[test]
    fn test_block_error() {
        let block = Block::error("Error Block");
        assert_eq!(block.title_text(), Some("Error Block"));
        assert!(block.border_style.is_some());
    }

    #[test]
    fn test_block_warning() {
        let block = Block::warning("Warning Block");
        assert_eq!(block.title_text(), Some("Warning Block"));
        assert!(block.border_style.is_some());
    }

    #[test]
    fn test_block_to_ratatui() {
        let block = Block::new().title("Convert");
        let ratatui_block = block.to_ratatui();
        // Verify conversion works (can't easily test ratatui internals)
        assert!(block.title_text().is_some());
    }

    #[test]
    fn test_block_clone() {
        let block1 = Block::new().title("Clone");
        let block2 = block1.clone();
        assert_eq!(block1.title_text(), block2.title_text());
    }

    #[test]
    fn test_block_equality() {
        let block1 = Block::new().title("Same");
        let block2 = Block::new().title("Same");
        let block3 = Block::new().title("Different");

        assert_eq!(block1, block2);
        assert_ne!(block1, block3);
    }

    #[test]
    fn test_block_no_borders() {
        let block = Block::new().borders(Borders::NONE);
        assert_eq!(block.borders, Borders::NONE);
    }

    #[test]
    fn test_block_empty_title() {
        let block = Block::new().title("");
        assert_eq!(block.title_text(), Some(""));
    }

    #[test]
    fn test_block_unicode_title() {
        let block = Block::new().title("🐸 TOAD 日本");
        assert_eq!(block.title_text(), Some("🐸 TOAD 日本"));
    }

    #[test]
    fn test_block_long_title() {
        let long_title = "A".repeat(1000);
        let block = Block::new().title(&long_title);
        assert_eq!(block.title_text(), Some(long_title.as_str()));
    }
}
