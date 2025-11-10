//! Main Screen - Primary interface with input and status
//!
//! The main command interface for TOAD with status display, input bar, and shortcuts.
//!
//! # Architecture
//!
//! Following Atomic Design:
//! - **Screen**: Top-level UI composition
//! - **Components**: Uses Text and Block atoms
//! - **Stateful**: Holds current input and status
//!
//! # Examples
//!
//! ```
//! use toad::ui::screens::main_screen::MainScreen;
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! let mut screen = MainScreen::new();
//! screen.set_status("Ready to evaluate");
//! screen.set_input("eval --count 10 --milestone 1");
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! screen.render(area, &mut buf);
//! ```

use crate::ui::atoms::{Block, Text};
use crate::ui::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Main screen with command input and status
///
/// The primary interface for entering commands and viewing status.
///
/// # Examples
///
/// ```
/// use toad::ui::screens::main_screen::MainScreen;
///
/// let screen = MainScreen::new();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MainScreen {
    /// Current status message
    status: String,
    /// Current input text
    input: String,
    /// Working directory path
    path: Option<String>,
    /// Current model name
    model: Option<String>,
    /// Input placeholder text
    placeholder: String,
}

impl MainScreen {
    /// Create a new main screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::main_screen::MainScreen;
    ///
    /// let screen = MainScreen::new();
    /// ```
    pub fn new() -> Self {
        Self {
            status: "Ready".to_string(),
            input: String::new(),
            path: None,
            model: None,
            placeholder: "eval --count 10 --milestone 1".to_string(),
        }
    }

    /// Set status message
    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status = status.into();
    }

    /// Set input text
    pub fn set_input(&mut self, input: impl Into<String>) {
        self.input = input.into();
    }

    /// Set working directory path
    pub fn set_path(&mut self, path: impl Into<String>) {
        self.path = Some(path.into());
    }

    /// Set model name
    pub fn set_model(&mut self, model: impl Into<String>) {
        self.model = Some(model.into());
    }

    /// Set placeholder text
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder = placeholder.into();
    }

    /// Get current status
    pub fn status(&self) -> &str {
        &self.status
    }

    /// Get current input
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Render the main screen
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Split layout: status | content | input | shortcuts
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Status bar
                Constraint::Min(0),    // Content area (empty for now)
                Constraint::Length(3), // Input bar
                Constraint::Length(1), // Shortcuts
            ])
            .split(area);

        // Render status bar
        self.render_status(chunks[0], buf);

        // Render input bar
        self.render_input(chunks[2], buf);

        // Render shortcuts
        self.render_shortcuts(chunks[3], buf);
    }

    /// Render status bar with metadata
    fn render_status(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::themed("Status").to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        let mut status_spans = vec![
            Span::styled(
                self.status.clone(),
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
        ];

        // Add path if set
        if let Some(ref path) = self.path {
            status_spans.push(Span::raw("  |  "));
            status_spans.push(Span::styled(
                format!("📁 {}", path),
                Style::default().fg(ToadTheme::GRAY),
            ));
        }

        // Add model if set
        if let Some(ref model) = self.model {
            status_spans.push(Span::raw("  |  "));
            status_spans.push(Span::styled(
                format!("🤖 {}", model),
                Style::default().fg(ToadTheme::BLUE),
            ));
        }

        Paragraph::new(Line::from(status_spans)).render(inner, buf);
    }

    /// Render input bar
    fn render_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::themed("Command").to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        let input_text = if self.input.is_empty() {
            Text::new(&self.placeholder)
                .style(Style::default().fg(ToadTheme::GRAY).add_modifier(Modifier::DIM))
                .to_span()
        } else {
            Text::new(&self.input)
                .style(Style::default().fg(ToadTheme::WHITE))
                .to_span()
        };

        Paragraph::new(input_text).render(inner, buf);
    }

    /// Render keyboard shortcuts
    fn render_shortcuts(&self, area: Rect, buf: &mut Buffer) {
        let shortcuts = vec![
            Span::styled("Esc", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::raw("/"),
            Span::styled("q", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::raw(": Quit  "),
            Span::styled("Enter", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::raw(": Execute  "),
            Span::styled("Tab", Style::default().fg(ToadTheme::TOAD_GREEN)),
            Span::raw(": Autocomplete"),
        ];

        Paragraph::new(Line::from(shortcuts))
            .alignment(Alignment::Center)
            .style(Style::default().fg(ToadTheme::GRAY))
            .render(area, buf);
    }
}

impl Default for MainScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for MainScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &MainScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        MainScreen::render(self, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_screen_new() {
        let screen = MainScreen::new();
        assert_eq!(screen.status(), "Ready");
        assert_eq!(screen.input(), "");
    }

    #[test]
    fn test_main_screen_set_status() {
        let mut screen = MainScreen::new();
        screen.set_status("Evaluating...");
        assert_eq!(screen.status(), "Evaluating...");
    }

    #[test]
    fn test_main_screen_set_input() {
        let mut screen = MainScreen::new();
        screen.set_input("eval --count 5");
        assert_eq!(screen.input(), "eval --count 5");
    }

    #[test]
    fn test_main_screen_set_path() {
        let mut screen = MainScreen::new();
        screen.set_path("/home/user/project");
        assert_eq!(screen.path, Some("/home/user/project".to_string()));
    }

    #[test]
    fn test_main_screen_set_model() {
        let mut screen = MainScreen::new();
        screen.set_model("claude-sonnet-3.5");
        assert_eq!(screen.model, Some("claude-sonnet-3.5".to_string()));
    }

    #[test]
    fn test_main_screen_set_placeholder() {
        let mut screen = MainScreen::new();
        screen.set_placeholder("Type command here...");
        assert_eq!(screen.placeholder, "Type command here...");
    }

    #[test]
    fn test_main_screen_render() {
        let screen = MainScreen::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        screen.render(area, &mut buf);
        // Verify it doesn't panic
    }

    #[test]
    fn test_main_screen_render_with_metadata() {
        let mut screen = MainScreen::new();
        screen.set_status("Running");
        screen.set_input("eval --count 10");
        screen.set_path("/home/user/toad");
        screen.set_model("claude-sonnet-3.5");

        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        screen.render(area, &mut buf);
        // Verify it doesn't panic
    }

    #[test]
    fn test_main_screen_clone() {
        let screen1 = MainScreen::new();
        let screen2 = screen1.clone();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_main_screen_equality() {
        let screen1 = MainScreen::new();
        let screen2 = MainScreen::new();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_main_screen_default() {
        let screen = MainScreen::default();
        assert_eq!(screen.status(), "Ready");
    }

    #[test]
    fn test_main_screen_widget_trait() {
        let screen = MainScreen::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        // Test both owned and borrowed widget rendering
        screen.clone().render(area, &mut buf);
        (&screen).render(area, &mut buf);
    }

    #[test]
    fn test_main_screen_empty_input_shows_placeholder() {
        let screen = MainScreen::new();
        assert_eq!(screen.input(), "");
        assert!(!screen.placeholder.is_empty());
    }

    #[test]
    fn test_main_screen_long_status() {
        let mut screen = MainScreen::new();
        screen.set_status("a".repeat(100));
        assert_eq!(screen.status().len(), 100);
    }

    #[test]
    fn test_main_screen_long_input() {
        let mut screen = MainScreen::new();
        screen.set_input("a".repeat(200));
        assert_eq!(screen.input().len(), 200);
    }

    #[test]
    fn test_main_screen_unicode_input() {
        let mut screen = MainScreen::new();
        screen.set_input("eval 🐸 --test");
        assert_eq!(screen.input(), "eval 🐸 --test");
    }
}
