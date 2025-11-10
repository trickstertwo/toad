//! Welcome Screen - Minimal welcome interface with logo and quick tips
//!
//! Simple splash screen shown on startup with TOAD logo and quick start tips.
//!
//! # Architecture
//!
//! Following Atomic Design:
//! - **Screen**: Top-level UI composition
//! - **Components**: Uses Text and Block atoms
//! - **Pure**: Stateless rendering
//!
//! # Examples
//!
//! ```
//! use toad::ui::screens::welcome::WelcomeScreen;
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! let screen = WelcomeScreen::new();
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! screen.render(area, &mut buf);
//! ```

use crate::ui::atoms::{Block, Text};
use crate::ui::logo::{TAGLINE, TOAD_LOGO};
use crate::ui::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Welcome screen with logo and quick tips
///
/// Displays TOAD logo, tagline, and quick start tips.
///
/// # Examples
///
/// ```
/// use toad::ui::screens::welcome::WelcomeScreen;
///
/// let screen = WelcomeScreen::new();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct WelcomeScreen {
    /// Quick start tips to display
    tips: Vec<String>,
}

impl WelcomeScreen {
    /// Create a new welcome screen with default tips
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::welcome::WelcomeScreen;
    ///
    /// let screen = WelcomeScreen::new();
    /// ```
    pub fn new() -> Self {
        Self {
            tips: vec![
                "Type 'eval --count 10 --milestone 1' to run evaluation".to_string(),
                "Use 'compare --baseline 1 --test 2' for A/B testing".to_string(),
                "Press 'q' or 'Esc' to quit anytime".to_string(),
            ],
        }
    }

    /// Set custom tips
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::welcome::WelcomeScreen;
    ///
    /// let screen = WelcomeScreen::new().tips(vec![
    ///     "Custom tip 1".to_string(),
    ///     "Custom tip 2".to_string(),
    /// ]);
    /// ```
    pub fn tips(mut self, tips: Vec<String>) -> Self {
        self.tips = tips;
        self
    }

    /// Get current tips
    pub fn get_tips(&self) -> &[String] {
        &self.tips
    }

    /// Render the welcome screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::welcome::WelcomeScreen;
    /// use ratatui::{buffer::Buffer, layout::Rect};
    ///
    /// let screen = WelcomeScreen::new();
    /// let area = Rect::new(0, 0, 80, 24);
    /// let mut buf = Buffer::empty(area);
    /// screen.render(area, &mut buf);
    /// ```
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create border
        let block = Block::themed("Welcome to TOAD").to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        // Split layout: logo | tagline | tips | prompt
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),                      // Logo
                Constraint::Length(1),                      // Spacer
                Constraint::Length(1),                      // Tagline
                Constraint::Length(2),                      // Spacer
                Constraint::Length(self.tips.len() as u16), // Tips
                Constraint::Length(2),                      // Spacer
                Constraint::Length(1),                      // Prompt
                Constraint::Min(0),                         // Filler
            ])
            .split(inner);

        // Render logo
        self.render_logo(chunks[0], buf);

        // Render tagline
        self.render_tagline(chunks[2], buf);

        // Render tips
        self.render_tips(chunks[4], buf);

        // Render prompt
        self.render_prompt(chunks[6], buf);
    }

    /// Render TOAD logo
    fn render_logo(&self, area: Rect, buf: &mut Buffer) {
        let logo_span = Text::new(TOAD_LOGO)
            .style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .to_span();

        Paragraph::new(logo_span)
            .alignment(Alignment::Center)
            .render(area, buf);
    }

    /// Render tagline
    fn render_tagline(&self, area: Rect, buf: &mut Buffer) {
        let tagline = Text::new(TAGLINE)
            .style(
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::ITALIC),
            )
            .to_span();

        Paragraph::new(tagline)
            .alignment(Alignment::Center)
            .render(area, buf);
    }

    /// Render quick tips
    fn render_tips(&self, area: Rect, buf: &mut Buffer) {
        let tip_lines: Vec<Line> = self
            .tips
            .iter()
            .map(|tip| {
                Line::from(vec![
                    Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                    Span::styled(tip.clone(), Style::default().fg(ToadTheme::WHITE)),
                ])
            })
            .collect();

        Paragraph::new(tip_lines).render(area, buf);
    }

    /// Render "press any key" prompt
    fn render_prompt(&self, area: Rect, buf: &mut Buffer) {
        let prompt = Text::new("Press any key to continue...")
            .style(
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::DIM),
            )
            .to_span();

        Paragraph::new(prompt)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for WelcomeScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &WelcomeScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        WelcomeScreen::render(self, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_screen_new() {
        let screen = WelcomeScreen::new();
        assert!(!screen.get_tips().is_empty());
    }

    #[test]
    fn test_welcome_screen_default_tips() {
        let screen = WelcomeScreen::new();
        let tips = screen.get_tips();
        assert_eq!(tips.len(), 3);
        assert!(tips[0].contains("eval"));
    }

    #[test]
    fn test_welcome_screen_custom_tips() {
        let custom_tips = vec!["Tip 1".to_string(), "Tip 2".to_string()];
        let screen = WelcomeScreen::new().tips(custom_tips.clone());
        assert_eq!(screen.get_tips(), custom_tips.as_slice());
    }

    #[test]
    fn test_welcome_screen_empty_tips() {
        let screen = WelcomeScreen::new().tips(vec![]);
        assert_eq!(screen.get_tips().len(), 0);
    }

    #[test]
    fn test_welcome_screen_render() {
        let screen = WelcomeScreen::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        screen.render(area, &mut buf);
        // Verify it doesn't panic
    }

    #[test]
    fn test_welcome_screen_clone() {
        let screen1 = WelcomeScreen::new();
        let screen2 = screen1.clone();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_welcome_screen_equality() {
        let screen1 = WelcomeScreen::new();
        let screen2 = WelcomeScreen::new();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_welcome_screen_default() {
        let screen = WelcomeScreen::default();
        assert!(!screen.get_tips().is_empty());
    }

    #[test]
    fn test_welcome_screen_widget_trait() {
        let screen = WelcomeScreen::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        // Test both owned and borrowed widget rendering
        screen.clone().render(area, &mut buf);
        (&screen).render(area, &mut buf);
    }

    #[test]
    fn test_welcome_screen_many_tips() {
        let tips = vec!["Tip".to_string(); 10];
        let screen = WelcomeScreen::new().tips(tips.clone());
        assert_eq!(screen.get_tips().len(), 10);
    }

    #[test]
    fn test_welcome_screen_long_tip() {
        let long_tip = "a".repeat(200);
        let screen = WelcomeScreen::new().tips(vec![long_tip.clone()]);
        assert_eq!(screen.get_tips()[0], long_tip);
    }

    #[test]
    fn test_welcome_screen_unicode_tips() {
        let unicode_tips = vec!["🐸 Emoji tip".to_string(), "日本語 tip".to_string()];
        let screen = WelcomeScreen::new().tips(unicode_tips.clone());
        assert_eq!(screen.get_tips(), unicode_tips.as_slice());
    }
}
