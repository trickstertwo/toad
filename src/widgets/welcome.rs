//! Welcome screen widget
//!
//! Displays the TOAD logo, version, and quick start tips

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{logo, theme::ToadTheme};

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
        // Main outer block
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title(format!(
                " {} {} ",
                logo::TOAD_MINIMAL,
                logo::version_string()
            ))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(ToadTheme::BACKGROUND));

        let inner = outer_block.inner(area);
        frame.render_widget(outer_block, inner);

        // Split into left (logo) and right (tips) if tips are enabled
        if self.show_tips {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(inner);

            self.render_logo(frame, chunks[0]);
            self.render_tips(frame, chunks[1]);
        } else {
            self.render_logo(frame, inner);
        }
    }

    fn render_logo(&self, frame: &mut Frame, area: Rect) {
        let logo_lines: Vec<Line> = logo::TOAD_COMPACT
            .lines()
            .map(|line| {
                Line::from(Span::styled(
                    line,
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ))
            })
            .collect();

        // Add tagline and subtitle
        let mut all_lines = logo_lines;
        all_lines.push(Line::from(""));
        all_lines.push(Line::from(Span::styled(
            logo::TAGLINE,
            Style::default().fg(ToadTheme::WHITE),
        )));
        all_lines.push(Line::from(Span::styled(
            logo::SUBTITLE,
            Style::default().fg(ToadTheme::GRAY),
        )));
        all_lines.push(Line::from(""));
        all_lines.push(Line::from(Span::styled(
            "Built with Rust + Ratatui",
            Style::default()
                .fg(ToadTheme::DARK_GRAY)
                .add_modifier(Modifier::ITALIC),
        )));

        let logo_paragraph = Paragraph::new(all_lines).alignment(Alignment::Center);
        frame.render_widget(logo_paragraph, area);
    }

    fn render_tips(&self, frame: &mut Frame, area: Rect) {
        let tips_block = Block::default()
            .title(" Quick Start ")
            .title_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY));

        let inner = tips_block.inner(area);
        frame.render_widget(tips_block, area);

        let tips = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Getting Started:",
                Style::default()
                    .fg(ToadTheme::WHITE)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Ask questions or request code changes", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Type ", Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled("/help", Style::default().fg(ToadTheme::TOAD_GREEN).add_modifier(Modifier::BOLD)),
                Span::styled(" for available commands", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Press ", Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled("?", Style::default().fg(ToadTheme::TOAD_GREEN).add_modifier(Modifier::BOLD)),
                Span::styled(" for keybindings", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Use ", Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled("Ctrl+P", Style::default().fg(ToadTheme::TOAD_GREEN).add_modifier(Modifier::BOLD)),
                Span::styled(" for command palette", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Features:",
                Style::default()
                    .fg(ToadTheme::WHITE)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("AI-powered code generation", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Semi-autonomous agents", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Plugin marketplace (coming soon)", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Native terminal experience", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to continue...",
                Style::default()
                    .fg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::ITALIC),
            )),
        ];

        let tips_paragraph = Paragraph::new(tips).alignment(Alignment::Left);
        frame.render_widget(tips_paragraph, inner);
    }
}

impl Default for WelcomeScreen {
    fn default() -> Self {
        Self::new()
    }
}
