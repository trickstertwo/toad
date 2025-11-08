//! Welcome screen widget
//!
//! Displays the TOAD logo, version, and quick start tips

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::{logo, theme::ToadTheme};

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
        let header_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = header_block.inner(area);
        frame.render_widget(header_block, area);

        // Split into two columns: logo on left, quick start on right
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(inner);

        // Render logo on left side
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

        let logo_paragraph = Paragraph::new(logo_lines).alignment(Alignment::Center);
        frame.render_widget(logo_paragraph, columns[0]);

        // Render quick start info on right side
        let quick_start = vec![
            Line::from(Span::styled(
                "AI-Powered Coding Terminal",
                Style::default()
                    .fg(ToadTheme::WHITE)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled(
                    "Ask questions or request changes",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
            ]),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Type ", Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled(
                    "/help",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" for commands", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Press ", Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled(
                    "?",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " for keybindings",
                    Style::default().fg(ToadTheme::FOREGROUND),
                ),
            ]),
            Line::from(vec![
                Span::styled("  • ", Style::default().fg(ToadTheme::TOAD_GREEN)),
                Span::styled("Use ", Style::default().fg(ToadTheme::FOREGROUND)),
                Span::styled(
                    "Ctrl+P",
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" for palette", Style::default().fg(ToadTheme::FOREGROUND)),
            ]),
        ];

        let quick_start_paragraph = Paragraph::new(quick_start).alignment(Alignment::Left);
        frame.render_widget(quick_start_paragraph, columns[1]);
    }

    fn render_version_line(&self, frame: &mut Frame, area: Rect) {
        let version = env!("CARGO_PKG_VERSION");
        let version_text = format!("v{} · Built with Rust + Ratatui", version);
        let version_line = Line::from(Span::styled(
            version_text,
            Style::default()
                .fg(ToadTheme::DARK_GRAY)
                .add_modifier(Modifier::ITALIC),
        ));
        let version_paragraph = Paragraph::new(version_line).alignment(Alignment::Center);
        frame.render_widget(version_paragraph, area);
    }

    fn render_tips_section(&self, frame: &mut Frame, area: Rect) {
        if !self.show_tips {
            return;
        }

        let tips_block = Block::default()
            .title(" Features ")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY));

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
