//! Modal dialog system
//!
//! Error, warning, info, and success modals

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Modal type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    Error,
    Warning,
    Info,
    Success,
}

impl ModalType {
    /// Get the color for this modal type
    fn color(&self) -> ratatui::style::Color {
        match self {
            ModalType::Error => ratatui::style::Color::Red,
            ModalType::Warning => ratatui::style::Color::Yellow,
            ModalType::Info => ToadTheme::TOAD_GREEN,
            ModalType::Success => ratatui::style::Color::Green,
        }
    }

    /// Get the icon for this modal type
    fn icon(&self) -> &str {
        match self {
            ModalType::Error => "✖",
            ModalType::Warning => "⚠",
            ModalType::Info => "ℹ",
            ModalType::Success => "✓",
        }
    }

    /// Get the title prefix for this modal type
    fn title_prefix(&self) -> &str {
        match self {
            ModalType::Error => "Error",
            ModalType::Warning => "Warning",
            ModalType::Info => "Information",
            ModalType::Success => "Success",
        }
    }
}

/// Modal dialog
pub struct Modal {
    modal_type: ModalType,
    title: Option<String>,
    message: String,
    details: Vec<String>,
    button_label: String,
}

impl Modal {
    /// Create a new modal
    pub fn new(modal_type: ModalType, message: impl Into<String>) -> Self {
        Self {
            modal_type,
            title: None,
            message: message.into(),
            details: Vec::new(),
            button_label: "OK".to_string(),
        }
    }

    /// Create an error modal
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(ModalType::Error, message)
    }

    /// Create a warning modal
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(ModalType::Warning, message)
    }

    /// Create an info modal
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(ModalType::Info, message)
    }

    /// Create a success modal
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(ModalType::Success, message)
    }

    /// Set custom title (overrides default)
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add detail lines
    pub fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = details;
        self
    }

    /// Add a single detail line
    pub fn add_detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    /// Set button label
    pub fn with_button(mut self, label: impl Into<String>) -> Self {
        self.button_label = label.into();
        self
    }

    /// Get the modal type
    pub fn modal_type(&self) -> ModalType {
        self.modal_type
    }

    /// Render the modal
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create centered modal
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(vertical[1]);

        let modal_area = horizontal[1];

        // Render modal block
        let color = self.modal_type.color();
        let title = self
            .title
            .as_deref()
            .unwrap_or(self.modal_type.title_prefix());

        let block = Block::default()
            .title(format!(" {} {} ", self.modal_type.icon(), title))
            .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = block.inner(modal_area);
        frame.render_widget(block, modal_area);

        // Split inner area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),    // Message + details
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Button
            ])
            .split(inner);

        // Render message
        let mut lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                &self.message,
                Style::default()
                    .fg(ToadTheme::FOREGROUND)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        if !self.details.is_empty() {
            lines.push(Line::from(""));
            for detail in &self.details {
                lines.push(Line::from(Span::styled(
                    detail,
                    Style::default().fg(ToadTheme::GRAY),
                )));
            }
        }

        let message_paragraph = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(message_paragraph, chunks[0]);

        // Render button
        let button_text = format!("[ {} ]", self.button_label);
        let button_line = Line::from(Span::styled(
            button_text,
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));
        let button_paragraph = Paragraph::new(button_line).alignment(Alignment::Center);

        frame.render_widget(button_paragraph, chunks[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_creation() {
        let modal = Modal::error("Test error");
        assert_eq!(modal.modal_type(), ModalType::Error);
    }

    #[test]
    fn test_modal_types() {
        let error = Modal::error("Error message");
        assert_eq!(error.modal_type(), ModalType::Error);

        let warning = Modal::warning("Warning message");
        assert_eq!(warning.modal_type(), ModalType::Warning);

        let info = Modal::info("Info message");
        assert_eq!(info.modal_type(), ModalType::Info);

        let success = Modal::success("Success message");
        assert_eq!(success.modal_type(), ModalType::Success);
    }

    #[test]
    fn test_modal_customization() {
        let modal = Modal::error("Error")
            .with_title("Custom Title")
            .add_detail("Detail 1")
            .add_detail("Detail 2")
            .with_button("Close");

        assert_eq!(modal.details.len(), 2);
        assert_eq!(modal.button_label, "Close");
        assert_eq!(modal.title, Some("Custom Title".to_string()));
    }

    #[test]
    fn test_modal_type_properties() {
        assert_eq!(ModalType::Error.title_prefix(), "Error");
        assert_eq!(ModalType::Warning.title_prefix(), "Warning");
        assert_eq!(ModalType::Info.title_prefix(), "Information");
        assert_eq!(ModalType::Success.title_prefix(), "Success");

        assert_eq!(ModalType::Error.icon(), "✖");
        assert_eq!(ModalType::Warning.icon(), "⚠");
        assert_eq!(ModalType::Info.icon(), "ℹ");
        assert_eq!(ModalType::Success.icon(), "✓");
    }
}
