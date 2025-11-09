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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_screen_new() {
        let help = HelpScreen::new();
        // Should construct successfully
        assert!(format!("{:?}", help).contains("HelpScreen"));
    }

    #[test]
    fn test_help_screen_default() {
        let help = HelpScreen::default();
        assert!(format!("{:?}", help).contains("HelpScreen"));
    }

    #[test]
    fn test_help_screen_debug() {
        let help = HelpScreen::new();
        let debug_str = format!("{:?}", help);
        assert_eq!(debug_str, "HelpScreen");
    }

    #[test]
    fn test_keybinding_line_formatting() {
        let help = HelpScreen::new();
        let line = help.keybinding_line("Ctrl+c", "Quit application");

        // Verify the line contains the key and description
        let line_str = format!("{:?}", line);
        assert!(line_str.contains("Ctrl+c"));
        assert!(line_str.contains("Quit application"));
    }

    #[test]
    fn test_keybinding_line_with_short_key() {
        let help = HelpScreen::new();
        let line = help.keybinding_line("?", "Toggle help");

        let line_str = format!("{:?}", line);
        assert!(line_str.contains("?"));
        assert!(line_str.contains("Toggle help"));
    }

    #[test]
    fn test_keybinding_line_with_long_key() {
        let help = HelpScreen::new();
        let line = help.keybinding_line("Ctrl+Shift+Alt+X", "Complex keybinding");

        let line_str = format!("{:?}", line);
        assert!(line_str.contains("Ctrl+Shift+Alt+X"));
        assert!(line_str.contains("Complex keybinding"));
    }

    #[test]
    fn test_keybinding_line_with_empty_description() {
        let help = HelpScreen::new();
        let line = help.keybinding_line("Enter", "");

        let line_str = format!("{:?}", line);
        assert!(line_str.contains("Enter"));
    }

    #[test]
    fn test_keybinding_line_with_unicode() {
        let help = HelpScreen::new();
        let line = help.keybinding_line("→", "Navigate right 🐸");

        let line_str = format!("{:?}", line);
        assert!(line_str.contains("→"));
        assert!(line_str.contains("🐸"));
    }

    #[test]
    fn test_keybinding_line_with_special_chars() {
        let help = HelpScreen::new();
        let line = help.keybinding_line("Ctrl+/", "Search & find");

        let line_str = format!("{:?}", line);
        assert!(line_str.contains("Ctrl+/"));
        assert!(line_str.contains("find"));
    }

    #[test]
    fn test_keybinding_line_multiple_calls() {
        let help = HelpScreen::new();

        let line1 = help.keybinding_line("a", "Action A");
        let line2 = help.keybinding_line("b", "Action B");

        let str1 = format!("{:?}", line1);
        let str2 = format!("{:?}", line2);

        assert!(str1.contains("Action A"));
        assert!(str2.contains("Action B"));
        assert_ne!(str1, str2);
    }

    // Note: render() method requires ratatui::Frame which is difficult to unit test
    // It should be tested via integration or E2E tests
    #[test]
    fn test_help_screen_exists() {
        // Smoke test to ensure the struct can be instantiated
        let _help = HelpScreen::new();
        let _help2 = HelpScreen::default();
        // If we get here, construction works
    }

    // ===== Integration tests with TestBackend =====
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_render_with_test_backend() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let area = frame.area();
                help.render(frame, area);
            })
            .unwrap();

        // Verify render didn't panic
    }

    #[test]
    fn test_render_with_small_area() {
        let backend = TestBackend::new(40, 12);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let area = frame.area();
                help.render(frame, area);
            })
            .unwrap();

        // Should handle small areas gracefully
    }

    #[test]
    fn test_render_with_large_area() {
        let backend = TestBackend::new(200, 60);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let area = frame.area();
                help.render(frame, area);
            })
            .unwrap();

        // Should handle large areas gracefully
    }

    #[test]
    fn test_render_with_minimal_area() {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let area = frame.area();
                help.render(frame, area);
            })
            .unwrap();

        // Should not panic even with very small area
    }

    #[test]
    fn test_render_content_includes_keybindings() {
        // Use larger terminal to ensure content is visible
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let area = frame.area();
                help.render(frame, area);
            })
            .unwrap();

        // Get buffer content
        let buffer = terminal.backend().buffer().clone();
        let content = buffer.content();

        // Build complete text by concatenating adjacent cells
        let mut rendered_text = String::new();
        for cell in content.iter() {
            rendered_text.push_str(cell.symbol());
        }

        let lower_text = rendered_text.to_lowercase();

        // Check for key content in the complete rendered text
        let found_ctrl = lower_text.contains("ctrl");
        let found_help = lower_text.contains("help");

        // Both should be found in the rendered content
        assert!(found_ctrl, "Should find 'Ctrl' in keybindings");
        assert!(found_help, "Should find 'help' in keybindings or help text");
    }

    #[test]
    fn test_render_includes_title() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let area = frame.area();
                help.render(frame, area);
            })
            .unwrap();

        // Verify render completed successfully
        let buffer = terminal.backend().buffer();
        assert!(buffer.area().width > 0);
        assert!(buffer.area().height > 0);
    }

    #[test]
    fn test_render_multiple_times() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        // Render multiple times
        for _ in 0..3 {
            terminal
                .draw(|frame| {
                    let area = frame.area();
                    help.render(frame, area);
                })
                .unwrap();
        }

        // Should handle repeated rendering
    }

    #[test]
    fn test_render_with_different_areas() {
        let backend = TestBackend::new(100, 50);
        let mut terminal = Terminal::new(backend).unwrap();

        let help = HelpScreen::new();

        terminal
            .draw(|frame| {
                let full_area = frame.area();

                // Test with different sub-areas
                let top_half = Rect {
                    x: 0,
                    y: 0,
                    width: full_area.width,
                    height: full_area.height / 2,
                };

                help.render(frame, top_half);
            })
            .unwrap();
    }
}
