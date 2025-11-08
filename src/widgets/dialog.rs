//! Dialog widget for confirmations and prompts
//!
//! Copilot-style confirmation dialogs with radio button selection

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::theme::ToadTheme;

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
            .map(|idx| {
                self.selected = idx;
                idx
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
            .map(|msg| Line::from(Span::styled(msg, Style::default().fg(ToadTheme::FOREGROUND))))
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
