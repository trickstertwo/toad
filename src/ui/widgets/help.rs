//! Help screen widget
//!
//! Displays keybindings and command reference

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Help screen widget
#[derive(Debug)]
pub struct HelpScreen;

impl HelpScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create centered modal-style layout
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(vertical[1]);

        let help_area = horizontal[1];

        // Render help content
        let help_block = Block::default()
            .title(" TOAD - Keyboard Shortcuts ")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = help_block.inner(help_area);
        frame.render_widget(help_block, help_area);

        // Split into two columns for keybindings
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        // Left column: General & Navigation
        let left_content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "GENERAL",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            self.keybinding_line("Ctrl+c", "Quit application"),
            self.keybinding_line("?", "Toggle help screen"),
            self.keybinding_line("Esc", "Go back / Close dialog"),
            Line::from(""),
            Line::from(Span::styled(
                "INPUT & EDITING",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            self.keybinding_line("Enter", "Submit command"),
            self.keybinding_line("Ctrl+u", "Clear input"),
            self.keybinding_line("Ctrl+a / Home", "Move to start"),
            self.keybinding_line("Ctrl+e / End", "Move to end"),
            self.keybinding_line("← / →", "Move cursor"),
            self.keybinding_line("Backspace", "Delete character"),
            Line::from(""),
            Line::from(Span::styled(
                "NAVIGATION",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            self.keybinding_line("↑ / ↓", "Navigate lists"),
            self.keybinding_line("Tab", "Next panel"),
            self.keybinding_line("Shift+Tab", "Previous panel"),
        ];

        // Right column: Commands & Features
        let right_content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "COMMANDS",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            self.keybinding_line("/help", "Show command reference"),
            self.keybinding_line("/commands", "List available commands"),
            self.keybinding_line("/clear", "Clear screen"),
            self.keybinding_line("@filename", "Mention file in context"),
            Line::from(""),
            Line::from(Span::styled(
                "FEATURES",
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            self.keybinding_line("Ctrl+p", "Command palette"),
            self.keybinding_line("Ctrl+r", "Expand recent"),
            self.keybinding_line("/", "Start command"),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "Press ESC or ? to close this help screen",
                Style::default()
                    .fg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::ITALIC),
            )),
        ];

        let left_paragraph = Paragraph::new(left_content).alignment(Alignment::Left);
        let right_paragraph = Paragraph::new(right_content).alignment(Alignment::Left);

        frame.render_widget(left_paragraph, columns[0]);
        frame.render_widget(right_paragraph, columns[1]);
    }

    fn keybinding_line<'a>(&self, key: &'a str, description: &'a str) -> Line<'a> {
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                format!("{:<15}", key),
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default()),
            Span::styled(
                description.to_string(),
                Style::default().fg(ToadTheme::FOREGROUND),
            ),
        ])
    }
}

impl Default for HelpScreen {
    fn default() -> Self {
        Self::new()
    }
}
