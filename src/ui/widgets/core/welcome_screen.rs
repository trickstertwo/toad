//! Welcome screen widget
//!
//! Displays the TOAD logo, version, and quick start tips

use crate::ui::{
    atoms::{block::Block, text::Text},
    logo,
    theme::ToadTheme,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

/// Welcome screen widget
pub struct WelcomeScreen {
    show_tips: bool,
}

impl WelcomeScreen {
    pub fn new() -> Self {
        Self { show_tips: true }
    }

    pub fn with_tips(mut self, show: bool) -> Self {
        self.show_tips = show;
        self
    }

    /// Render the welcome screen
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Main layout: header box, tips section, status section
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(11), // Header box with logo and quick start
                Constraint::Length(1),  // Spacing
                Constraint::Min(5),     // Tips section
                Constraint::Length(1),  // Spacing
                Constraint::Length(3),  // Status section
            ])
            .split(area);

        // Render main header box
        self.render_header_box(frame, chunks[0]);

        // Render version below header
        self.render_version_line(frame, chunks[1]);

        // Render tips section
        self.render_tips_section(frame, chunks[2]);

        // Render status section
        self.render_status_section(frame, chunks[4]);
    }

    fn render_header_box(&self, frame: &mut Frame, area: Rect) {
        let header_block = Block::new()
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK))
            .to_ratatui();

        let inner = header_block.inner(area);
        frame.render_widget(header_block, area);

        // Split into two columns: logo on left, quick start on right
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(inner);

        // Render logo on left side using Text atoms
        let logo_lines: Vec<Line> = logo::TOAD_COMPACT
            .lines()
            .map(|line| {
                let text = Text::new(line).style(
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                );
                Line::from(text.to_span())
            })
            .collect();

        let logo_paragraph = Paragraph::new(logo_lines).alignment(Alignment::Center);
        frame.render_widget(logo_paragraph, columns[0]);

        // Render quick start info on right side using Text atoms
        let quick_start = vec![
            Line::from(
                Text::new("AI-Powered Coding Terminal")
                    .style(
                        Style::default()
                            .fg(ToadTheme::WHITE)
                            .add_modifier(Modifier::BOLD),
                    )
                    .to_span(),
            ),
            Line::from(""),
            Line::from(vec![
                Text::new("  • ")
                    .style(Style::default().fg(ToadTheme::TOAD_GREEN))
                    .to_span(),
                Text::new("Ask questions or request changes")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
            ]),
            Line::from(vec![
                Text::new("  • ")
                    .style(Style::default().fg(ToadTheme::TOAD_GREEN))
                    .to_span(),
                Text::new("Type ")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
                Text::new("/help")
                    .style(
                        Style::default()
                            .fg(ToadTheme::TOAD_GREEN)
                            .add_modifier(Modifier::BOLD),
                    )
                    .to_span(),
                Text::new(" for commands")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
            ]),
            Line::from(vec![
                Text::new("  • ")
                    .style(Style::default().fg(ToadTheme::TOAD_GREEN))
                    .to_span(),
                Text::new("Press ")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
                Text::new("?")
                    .style(
                        Style::default()
                            .fg(ToadTheme::TOAD_GREEN)
                            .add_modifier(Modifier::BOLD),
                    )
                    .to_span(),
                Text::new(" for keybindings")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
            ]),
            Line::from(vec![
                Text::new("  • ")
                    .style(Style::default().fg(ToadTheme::TOAD_GREEN))
                    .to_span(),
                Text::new("Use ")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
                Text::new("Ctrl+P")
                    .style(
                        Style::default()
                            .fg(ToadTheme::TOAD_GREEN)
                            .add_modifier(Modifier::BOLD),
                    )
                    .to_span(),
                Text::new(" for palette")
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_span(),
            ]),
        ];

        let quick_start_paragraph = Paragraph::new(quick_start).alignment(Alignment::Left);
        frame.render_widget(quick_start_paragraph, columns[1]);
    }

    fn render_version_line(&self, frame: &mut Frame, area: Rect) {
        let version = env!("CARGO_PKG_VERSION");
        let version_text = format!("v{} · Built with Rust + Ratatui", version);
        let version_line = Line::from(
            Text::new(version_text)
                .style(
                    Style::default()
                        .fg(ToadTheme::DARK_GRAY)
                        .add_modifier(Modifier::ITALIC),
                )
                .to_span(),
        );
        let version_paragraph = Paragraph::new(version_line).alignment(Alignment::Center);
        frame.render_widget(version_paragraph, area);
    }

    fn render_tips_section(&self, frame: &mut Frame, area: Rect) {
        if !self.show_tips {
            return;
        }

        let tips_block = Block::new()
            .title(" Features ")
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY))
            .to_ratatui();

        let inner = tips_block.inner(area);
        frame.render_widget(tips_block, area);

        let tips = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  1. ",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "AI-powered code generation ",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
                Span::styled("✓", Style::default().fg(ToadTheme::TOAD_GREEN)),
            ]),
            Line::from(vec![
                Span::styled(
                    "  2. ",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Semi-autonomous coding agents ",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
                Span::styled("✓", Style::default().fg(ToadTheme::TOAD_GREEN)),
            ]),
            Line::from(vec![
                Span::styled(
                    "  3. ",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Plugin marketplace ",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
                Span::styled(
                    "(coming soon)",
                    Style::default()
                        .fg(ToadTheme::GRAY)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "  4. ",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Native terminal experience ",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
                Span::styled("✓", Style::default().fg(ToadTheme::TOAD_GREEN)),
            ]),
            Line::from(vec![
                Span::styled(
                    "  5. ",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Command palette & shortcuts ",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
                Span::styled("✓", Style::default().fg(ToadTheme::TOAD_GREEN)),
            ]),
            Line::from(""),
        ];

        let tips_paragraph = Paragraph::new(tips).alignment(Alignment::Left);
        frame.render_widget(tips_paragraph, inner);
    }

    fn render_status_section(&self, frame: &mut Frame, area: Rect) {
        let status_lines = vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(ToadTheme::GRAY)),
                Span::styled(
                    "Ready ",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("●", Style::default().fg(ToadTheme::TOAD_GREEN)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to continue...",
                Style::default()
                    .fg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::ITALIC),
            )),
        ];

        let status_paragraph = Paragraph::new(status_lines).alignment(Alignment::Center);
        frame.render_widget(status_paragraph, area);
    }
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_screen_new() {
        let screen = WelcomeScreen::new();
        assert!(screen.show_tips, "New screen should show tips by default");
    }

    #[test]
    fn test_welcome_screen_default() {
        let screen = WelcomeScreen::default();
        assert!(screen.show_tips, "Default screen should show tips");
    }

    #[test]
    fn test_welcome_screen_with_tips_true() {
        let screen = WelcomeScreen::new().with_tips(true);
        assert!(
            screen.show_tips,
            "Screen should show tips when with_tips(true)"
        );
    }

    #[test]
    fn test_welcome_screen_with_tips_false() {
        let screen = WelcomeScreen::new().with_tips(false);
        assert!(
            !screen.show_tips,
            "Screen should hide tips when with_tips(false)"
        );
    }

    #[test]
    fn test_welcome_screen_builder_pattern() {
        // Test that builder pattern can be chained
        let screen = WelcomeScreen::new().with_tips(false).with_tips(true);
        assert!(screen.show_tips, "Builder pattern should allow chaining");
    }

    #[test]
    fn test_welcome_screen_toggle_tips() {
        // Test toggling tips multiple times
        let screen = WelcomeScreen::new()
            .with_tips(false)
            .with_tips(true)
            .with_tips(false);
        assert!(
            !screen.show_tips,
            "Tips should be hidden after final toggle"
        );
    }

    #[test]
    fn test_welcome_screen_tips_default_true() {
        // Verify default behavior
        let default_screen = WelcomeScreen::default();
        let new_screen = WelcomeScreen::new();
        assert_eq!(
            default_screen.show_tips, new_screen.show_tips,
            "Default and new should have same tips state"
        );
    }

    #[test]
    fn test_welcome_screen_with_tips_idempotent() {
        // Setting the same value twice should work
        let screen1 = WelcomeScreen::new().with_tips(true).with_tips(true);
        let screen2 = WelcomeScreen::new().with_tips(true);
        assert_eq!(
            screen1.show_tips, screen2.show_tips,
            "Idempotent with_tips calls should produce same result"
        );
    }

    #[test]
    fn test_welcome_screen_struct_size() {
        // Ensure the struct is small (only contains a bool)
        use std::mem::size_of;
        assert_eq!(
            size_of::<WelcomeScreen>(),
            size_of::<bool>(),
            "WelcomeScreen should only contain show_tips bool"
        );
    }

    #[test]
    fn test_welcome_screen_multiple_instances() {
        // Test creating multiple instances with different configs
        let screen1 = WelcomeScreen::new();
        let screen2 = WelcomeScreen::new().with_tips(false);
        let screen3 = WelcomeScreen::default();

        assert!(screen1.show_tips);
        assert!(!screen2.show_tips);
        assert!(screen3.show_tips);
    }
}
