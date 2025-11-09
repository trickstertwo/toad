//! Dialog widget for confirmations and prompts
//!
//! Copilot-style confirmation dialogs with radio button selection

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::theme::ToadTheme;

/// A single option in a dialog
#[derive(Debug, Clone)]
pub struct DialogOption {
    pub label: String,
    pub key: char,
}

impl DialogOption {
    pub fn new(key: char, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            key,
        }
    }
}

/// A confirmation dialog with radio button options
#[derive(Debug)]
pub struct ConfirmDialog {
    title: String,
    message: Vec<String>,
    options: Vec<DialogOption>,
    selected: usize,
    info_box: Option<String>,
}

impl ConfirmDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: Vec::new(),
            options: Vec::new(),
            selected: 0,
            info_box: None,
        }
    }

    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.message.push(msg.into());
        self
    }

    pub fn info_box(mut self, text: impl Into<String>) -> Self {
        self.info_box = Some(text.into());
        self
    }

    pub fn option(mut self, key: char, label: impl Into<String>) -> Self {
        self.options.push(DialogOption::new(key, label));
        self
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select_next(&mut self) {
        if self.selected < self.options.len() - 1 {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn select_by_key(&mut self, key: char) -> Option<usize> {
        self.options
            .iter()
            .position(|opt| opt.key == key)
            .inspect(|&idx| {
                self.selected = idx;
            })
    }

    /// Render the dialog centered on the screen
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Calculate dialog size
        let dialog_width = 100.min(area.width.saturating_sub(4));
        let dialog_height = (8 + self.message.len() as u16 + self.options.len() as u16)
            .min(area.height.saturating_sub(4));

        // Center the dialog
        let vertical_margin = (area.height.saturating_sub(dialog_height)) / 2;
        let horizontal_margin = (area.width.saturating_sub(dialog_width)) / 2;

        let dialog_area = Rect {
            x: area.x + horizontal_margin,
            y: area.y + vertical_margin,
            width: dialog_width,
            height: dialog_height,
        };

        // Outer container
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::BORDER_FOCUSED))
            .title(self.title.as_str())
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(ToadTheme::BACKGROUND));

        let inner = outer_block.inner(dialog_area);
        frame.render_widget(outer_block, dialog_area);

        // Layout: info box (if present), message, options, help text
        let mut constraints = vec![Constraint::Length(1)]; // Padding

        if self.info_box.is_some() {
            constraints.push(Constraint::Length(3)); // Info box
            constraints.push(Constraint::Length(1)); // Spacing
        }

        constraints.push(Constraint::Length(self.message.len() as u16)); // Message
        constraints.push(Constraint::Length(1)); // Spacing
        constraints.push(Constraint::Length(self.options.len() as u16)); // Options
        constraints.push(Constraint::Length(1)); // Spacing
        constraints.push(Constraint::Length(1)); // Help text
        constraints.push(Constraint::Min(0)); // Fill remaining

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner);

        let mut chunk_idx = 1;

        // Render info box if present
        if let Some(info_text) = &self.info_box {
            let info_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ToadTheme::DARK_GRAY));

            let info_paragraph = Paragraph::new(info_text.as_str())
                .style(Style::default().fg(ToadTheme::GRAY))
                .block(info_block)
                .alignment(Alignment::Center);

            frame.render_widget(info_paragraph, chunks[chunk_idx]);
            chunk_idx += 2; // Skip spacing
        }

        // Render message
        let message_lines: Vec<Line> = self
            .message
            .iter()
            .map(|msg| {
                Line::from(Span::styled(
                    msg,
                    Style::default().fg(ToadTheme::FOREGROUND),
                ))
            })
            .collect();

        let message_paragraph = Paragraph::new(message_lines).alignment(Alignment::Left);
        frame.render_widget(message_paragraph, chunks[chunk_idx]);
        chunk_idx += 2; // Skip spacing

        // Render options
        let options_area = chunks[chunk_idx];
        for (idx, option) in self.options.iter().enumerate() {
            let option_area = Rect {
                x: options_area.x,
                y: options_area.y + idx as u16,
                width: options_area.width,
                height: 1,
            };

            let is_selected = idx == self.selected;
            let prefix = if is_selected { "❯ " } else { "  " };
            let style = if is_selected {
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ToadTheme::GRAY)
            };

            let option_text = format!("{}{}. {}", prefix, option.key, option.label);
            let option_line = Line::from(Span::styled(option_text, style));
            let option_paragraph = Paragraph::new(option_line);

            frame.render_widget(option_paragraph, option_area);
        }
        chunk_idx += 2; // Skip spacing

        // Render help text
        let help_text = "Confirm with number keys or ↑↓ keys and Enter, Cancel with Esc";
        let help_line = Line::from(Span::styled(
            help_text,
            Style::default()
                .fg(ToadTheme::DARK_GRAY)
                .add_modifier(Modifier::ITALIC),
        ));
        let help_paragraph = Paragraph::new(help_line).alignment(Alignment::Center);
        frame.render_widget(help_paragraph, chunks[chunk_idx]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_option_new() {
        let option = DialogOption::new('1', "Test Option");
        assert_eq!(option.key, '1');
        assert_eq!(option.label, "Test Option");
    }

    #[test]
    fn test_confirm_dialog_new() {
        let dialog = ConfirmDialog::new("Test Title");
        assert_eq!(dialog.title, "Test Title");
        assert!(dialog.message.is_empty());
        assert!(dialog.options.is_empty());
        assert_eq!(dialog.selected, 0);
        assert!(dialog.info_box.is_none());
    }

    #[test]
    fn test_confirm_dialog_builder_pattern() {
        let dialog = ConfirmDialog::new("Title")
            .message("Line 1")
            .message("Line 2")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .info_box("Info text");

        assert_eq!(dialog.message.len(), 2);
        assert_eq!(dialog.options.len(), 2);
        assert!(dialog.info_box.is_some());
    }

    #[test]
    fn test_confirm_dialog_single_message() {
        let dialog = ConfirmDialog::new("Title").message("Single message");
        assert_eq!(dialog.message.len(), 1);
        assert_eq!(dialog.message[0], "Single message");
    }

    #[test]
    fn test_confirm_dialog_multiple_messages() {
        let dialog = ConfirmDialog::new("Title")
            .message("Line 1")
            .message("Line 2")
            .message("Line 3");

        assert_eq!(dialog.message.len(), 3);
        assert_eq!(dialog.message[0], "Line 1");
        assert_eq!(dialog.message[1], "Line 2");
        assert_eq!(dialog.message[2], "Line 3");
    }

    #[test]
    fn test_confirm_dialog_options() {
        let dialog = ConfirmDialog::new("Title")
            .option('1', "Yes")
            .option('2', "No")
            .option('3', "Cancel");

        assert_eq!(dialog.options.len(), 3);
        assert_eq!(dialog.options[0].key, '1');
        assert_eq!(dialog.options[0].label, "Yes");
        assert_eq!(dialog.options[1].key, '2');
        assert_eq!(dialog.options[1].label, "No");
        assert_eq!(dialog.options[2].key, '3');
        assert_eq!(dialog.options[2].label, "Cancel");
    }

    #[test]
    fn test_confirm_dialog_selected_default() {
        let dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2");

        assert_eq!(dialog.selected(), 0, "First option should be selected by default");
    }

    #[test]
    fn test_confirm_dialog_select_next() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3");

        assert_eq!(dialog.selected(), 0);

        dialog.select_next();
        assert_eq!(dialog.selected(), 1);

        dialog.select_next();
        assert_eq!(dialog.selected(), 2);
    }

    #[test]
    fn test_confirm_dialog_select_next_boundary() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2");

        dialog.select_next(); // Move to option 2
        assert_eq!(dialog.selected(), 1);

        dialog.select_next(); // Try to move past last option
        assert_eq!(dialog.selected(), 1, "Should not go past last option");
    }

    #[test]
    fn test_confirm_dialog_select_previous() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3");

        dialog.select_next();
        dialog.select_next();
        assert_eq!(dialog.selected(), 2);

        dialog.select_previous();
        assert_eq!(dialog.selected(), 1);

        dialog.select_previous();
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_select_previous_boundary() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2");

        assert_eq!(dialog.selected(), 0);

        dialog.select_previous(); // Try to move before first option
        assert_eq!(dialog.selected(), 0, "Should not go before first option");
    }

    #[test]
    fn test_confirm_dialog_select_by_key_found() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Yes")
            .option('2', "No")
            .option('3', "Cancel");

        let result = dialog.select_by_key('2');
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(dialog.selected(), 1);
    }

    #[test]
    fn test_confirm_dialog_select_by_key_not_found() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Yes")
            .option('2', "No");

        let result = dialog.select_by_key('9');
        assert!(result.is_none());
        assert_eq!(dialog.selected(), 0, "Selection should not change if key not found");
    }

    #[test]
    fn test_confirm_dialog_select_by_key_updates_selection() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Yes")
            .option('2', "No")
            .option('3', "Cancel");

        dialog.select_by_key('3');
        assert_eq!(dialog.selected(), 2);

        dialog.select_by_key('1');
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_info_box() {
        let dialog = ConfirmDialog::new("Title").info_box("Important info");

        assert!(dialog.info_box.is_some());
        assert_eq!(dialog.info_box.unwrap(), "Important info");
    }

    #[test]
    fn test_confirm_dialog_info_box_override() {
        let dialog = ConfirmDialog::new("Title")
            .info_box("First info")
            .info_box("Second info");

        assert_eq!(dialog.info_box.unwrap(), "Second info", "Last info_box call should override");
    }

    #[test]
    fn test_confirm_dialog_navigation_cycle() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3");

        // Navigate down
        for i in 0..3 {
            assert_eq!(dialog.selected(), i);
            if i < 2 {
                dialog.select_next();
            }
        }

        // Navigate up
        for i in (0..3).rev() {
            assert_eq!(dialog.selected(), i);
            if i > 0 {
                dialog.select_previous();
            }
        }
    }

    #[test]
    fn test_confirm_dialog_empty_options() {
        let dialog = ConfirmDialog::new("Title")
            .message("No options provided");

        assert_eq!(dialog.options.len(), 0);
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_single_option() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Only option");

        assert_eq!(dialog.selected(), 0);

        dialog.select_next();
        assert_eq!(dialog.selected(), 0, "Should stay on single option");

        dialog.select_previous();
        assert_eq!(dialog.selected(), 0, "Should stay on single option");
    }

    #[test]
    fn test_confirm_dialog_with_unicode() {
        let dialog = ConfirmDialog::new("标题 Title")
            .message("Message with emoji 🎉")
            .option('1', "Option with emoji 🐸")
            .info_box("Info with Unicode 日本語");

        assert!(dialog.title.contains("标题"));
        assert!(dialog.message[0].contains("🎉"));
        assert!(dialog.options[0].label.contains("🐸"));
        assert!(dialog.info_box.unwrap().contains("日本語"));
    }

    #[test]
    fn test_confirm_dialog_long_message() {
        let long_msg = "x".repeat(1000);
        let dialog = ConfirmDialog::new("Title").message(long_msg.clone());

        assert_eq!(dialog.message[0].len(), 1000);
        assert_eq!(dialog.message[0], long_msg);
    }

    #[test]
    fn test_dialog_option_clone() {
        let option1 = DialogOption::new('1', "Test");
        let option2 = option1.clone();

        assert_eq!(option1.key, option2.key);
        assert_eq!(option1.label, option2.label);
    }

    #[test]
    fn test_trust_dialog_typical_usage() {
        // Simulate the typical trust dialog usage
        let mut dialog = ConfirmDialog::new("Do you trust this folder?")
            .message("This folder wants to access your files.")
            .option('1', "Yes, for this session")
            .option('2', "Yes, and remember my choice")
            .option('3', "No, quit");

        assert_eq!(dialog.options.len(), 3);
        assert_eq!(dialog.selected(), 0);

        // User presses '2' to select second option
        let result = dialog.select_by_key('2');
        assert!(result.is_some());
        assert_eq!(dialog.selected(), 1);
    }
}
