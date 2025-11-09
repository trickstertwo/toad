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

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_modal_with_very_long_message() {
        let long_message = "A".repeat(10000);
        let modal = Modal::error(long_message.clone());
        assert_eq!(modal.modal_type(), ModalType::Error);
        assert_eq!(modal.message, long_message);
    }

    #[test]
    fn test_modal_with_unicode_message() {
        let modal = Modal::success("🎉 成功！ Operation completed 日本語");
        assert!(modal.message.contains("🎉"));
        assert!(modal.message.contains("成功"));
    }

    #[test]
    fn test_modal_with_empty_message() {
        let modal = Modal::info("");
        assert_eq!(modal.message, "");
    }

    #[test]
    fn test_modal_with_whitespace_only_message() {
        let modal = Modal::warning("     ");
        assert_eq!(modal.message, "     ");
    }

    #[test]
    fn test_modal_with_newlines_in_message() {
        let modal = Modal::error("Error on line 1\nError on line 2\nError on line 3");
        assert!(modal.message.contains("\n"));
    }

    #[test]
    fn test_modal_with_special_characters() {
        let modal = Modal::info("Test<>&\"'\\|/*?");
        assert!(modal.message.contains("<>"));
    }

    #[test]
    fn test_modal_with_very_long_title() {
        let long_title = "B".repeat(1000);
        let modal = Modal::error("Message").with_title(long_title.clone());
        assert_eq!(modal.title, Some(long_title));
    }

    #[test]
    fn test_modal_with_unicode_title() {
        let modal = Modal::success("Done").with_title("✅ 完了しました");
        assert!(modal.title.clone().unwrap().contains("✅"));
    }

    #[test]
    fn test_modal_with_empty_title() {
        let modal = Modal::info("Message").with_title("");
        assert_eq!(modal.title, Some("".to_string()));
    }

    #[test]
    fn test_modal_with_many_details() {
        let mut modal = Modal::error("Main error");
        for i in 0..100 {
            modal = modal.add_detail(format!("Detail {}", i));
        }
        assert_eq!(modal.details.len(), 100);
    }

    #[test]
    fn test_modal_with_very_long_details() {
        let long_detail = "C".repeat(10000);
        let modal = Modal::warning("Warning").add_detail(long_detail.clone());
        assert_eq!(modal.details[0], long_detail);
    }

    #[test]
    fn test_modal_with_unicode_details() {
        let modal = Modal::error("Error")
            .add_detail("原因: ファイルが見つかりません")
            .add_detail("🔍 詳細情報");

        assert_eq!(modal.details.len(), 2);
        assert!(modal.details[0].contains("原因"));
    }

    #[test]
    fn test_modal_with_empty_details() {
        let modal = Modal::info("Message")
            .add_detail("")
            .add_detail("   ")
            .add_detail("\n");

        assert_eq!(modal.details.len(), 3);
    }

    #[test]
    fn test_modal_with_details_vec() {
        let details = vec![
            "Detail 1".to_string(),
            "Detail 2".to_string(),
            "Detail 3".to_string(),
        ];
        let modal = Modal::success("Success").with_details(details.clone());
        assert_eq!(modal.details, details);
    }

    #[test]
    fn test_modal_with_empty_details_vec() {
        let modal = Modal::info("Message").with_details(vec![]);
        assert_eq!(modal.details.len(), 0);
    }

    #[test]
    fn test_modal_with_very_long_button_label() {
        let long_label = "D".repeat(1000);
        let modal = Modal::error("Error").with_button(long_label.clone());
        assert_eq!(modal.button_label, long_label);
    }

    #[test]
    fn test_modal_with_unicode_button_label() {
        let modal = Modal::success("Done").with_button("閉じる ✕");
        assert!(modal.button_label.contains("閉じる"));
    }

    #[test]
    fn test_modal_with_empty_button_label() {
        let modal = Modal::warning("Warning").with_button("");
        assert_eq!(modal.button_label, "");
    }

    #[test]
    fn test_modal_type_colors_unique() {
        let error_color = ModalType::Error.color();
        let warning_color = ModalType::Warning.color();
        let info_color = ModalType::Info.color();
        let success_color = ModalType::Success.color();

        // Colors should be distinct (at least some of them)
        assert_ne!(error_color, success_color);
        assert_ne!(warning_color, info_color);
    }

    #[test]
    fn test_modal_type_icons_unique() {
        let icons = vec![
            ModalType::Error.icon(),
            ModalType::Warning.icon(),
            ModalType::Info.icon(),
            ModalType::Success.icon(),
        ];

        // All icons should be unique
        for (i, icon1) in icons.iter().enumerate() {
            for (j, icon2) in icons.iter().enumerate() {
                if i != j {
                    assert_ne!(icon1, icon2);
                }
            }
        }
    }

    #[test]
    fn test_modal_type_equality() {
        assert_eq!(ModalType::Error, ModalType::Error);
        assert_eq!(ModalType::Success, ModalType::Success);
        assert_ne!(ModalType::Error, ModalType::Warning);
        assert_ne!(ModalType::Info, ModalType::Success);
    }

    #[test]
    fn test_modal_type_copy() {
        let original = ModalType::Info;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_modal_builder_pattern_chaining() {
        let modal = Modal::error("Error message")
            .with_title("Custom Error")
            .add_detail("Detail 1")
            .add_detail("Detail 2")
            .with_button("Dismiss");

        assert_eq!(modal.modal_type(), ModalType::Error);
        assert_eq!(modal.title, Some("Custom Error".to_string()));
        assert_eq!(modal.details.len(), 2);
        assert_eq!(modal.button_label, "Dismiss");
    }

    #[test]
    fn test_modal_replacing_details() {
        let modal = Modal::info("Message")
            .add_detail("First")
            .with_details(vec!["Replaced".to_string()]);

        assert_eq!(modal.details.len(), 1);
        assert_eq!(modal.details[0], "Replaced");
    }

    #[test]
    fn test_modal_multiple_detail_additions() {
        let modal = Modal::warning("Warning")
            .add_detail("Detail 1")
            .add_detail("Detail 2")
            .add_detail("Detail 3")
            .add_detail("Detail 4");

        assert_eq!(modal.details.len(), 4);
    }

    #[test]
    fn test_modal_default_button_label() {
        let modal = Modal::success("Done");
        assert_eq!(modal.button_label, "OK");
    }

    #[test]
    fn test_modal_title_override() {
        let modal = Modal::error("Error").with_title("Custom Title");

        // Modal should use custom title instead of default
        assert_eq!(modal.title, Some("Custom Title".to_string()));
    }

    #[test]
    fn test_modal_no_title_uses_default() {
        let modal = Modal::warning("Warning");

        // No custom title set
        assert_eq!(modal.title, None);
    }

    #[test]
    fn test_modal_all_types_constructors() {
        let error = Modal::error("Error");
        let warning = Modal::warning("Warning");
        let info = Modal::info("Info");
        let success = Modal::success("Success");

        assert_eq!(error.modal_type(), ModalType::Error);
        assert_eq!(warning.modal_type(), ModalType::Warning);
        assert_eq!(info.modal_type(), ModalType::Info);
        assert_eq!(success.modal_type(), ModalType::Success);
    }

    #[test]
    fn test_modal_with_mixed_unicode_and_ascii() {
        let modal = Modal::info("ASCII Message 🎉")
            .with_title("タイトル Title")
            .add_detail("Detail with emoji 🔍")
            .add_detail("純粋な日本語")
            .with_button("OK ✓");

        assert!(modal.message.contains("ASCII"));
        assert!(modal.message.contains("🎉"));
    }
}
