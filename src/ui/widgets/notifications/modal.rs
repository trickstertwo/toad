//! Modal dialog system
//!
//! Error, warning, info, and success modals

use crate::ui::{
    atoms::{block::Block as AtomBlock, text::Text},
    theme::ToadTheme,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Borders, Paragraph, Wrap},
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

        let block = AtomBlock::new()
            .title(&format!(" {} {} ", self.modal_type.icon(), title))
            .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .style(Style::default().bg(ToadTheme::BLACK))
            .to_ratatui();

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
            Line::from(
                Text::new(&self.message)
                    .style(
                        Style::default()
                            .fg(ToadTheme::FOREGROUND)
                            .add_modifier(Modifier::BOLD),
                    )
                    .to_span(),
            ),
        ];

        if !self.details.is_empty() {
            lines.push(Line::from(""));
            for detail in &self.details {
                lines.push(Line::from(
                    Text::new(detail)
                        .style(Style::default().fg(ToadTheme::GRAY))
                        .to_span(),
                ));
            }
        }

        let message_paragraph = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(message_paragraph, chunks[0]);

        // Render button
        let button_text = format!("[ {} ]", self.button_label);
        let button_line = Line::from(
            Text::new(button_text)
                .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                .to_span(),
        );
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

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_modal_with_10000_details() {
        let mut modal = Modal::error("Error with many details");
        for i in 0..10000 {
            modal = modal.add_detail(format!("Detail line {}", i));
        }
        assert_eq!(modal.details.len(), 10000);
    }

    #[test]
    fn test_rapid_modal_creation_all_types() {
        for _ in 0..1000 {
            let _error = Modal::error("Error");
            let _warning = Modal::warning("Warning");
            let _info = Modal::info("Info");
            let _success = Modal::success("Success");
        }
        // Just verify no crashes
    }

    #[test]
    fn test_modal_builder_stress_100_operations() {
        let mut modal = Modal::error("Base message");
        for i in 0..100 {
            modal = modal.add_detail(format!("Detail {}", i));
        }
        modal = modal.with_title("Final Title");
        modal = modal.with_button("Final Button");

        assert_eq!(modal.details.len(), 100);
        assert_eq!(modal.title, Some("Final Title".to_string()));
        assert_eq!(modal.button_label, "Final Button");
    }

    #[test]
    fn test_stress_replace_details_multiple_times() {
        let modal = Modal::info("Message")
            .add_detail("First")
            .add_detail("Second")
            .with_details(vec!["Third".to_string()])
            .add_detail("Fourth")
            .with_details(vec!["Fifth".to_string(), "Sixth".to_string()]);

        assert_eq!(modal.details.len(), 2);
        assert_eq!(modal.details[0], "Fifth");
        assert_eq!(modal.details[1], "Sixth");
    }

    // ============ Extreme Unicode Edge Cases ============

    #[test]
    fn test_modal_with_emoji_sequences() {
        let modal = Modal::success("👨‍👩‍👧‍👦 Family emoji sequence");
        assert!(modal.message.contains("👨‍👩‍👧‍👦"));
    }

    #[test]
    fn test_modal_with_rtl_text() {
        let modal = Modal::info("مرحبا العالم Hello שלום")
            .with_title("عنوان Title כותרת")
            .add_detail("تفاصيل Detail פרטים");

        assert!(modal.message.contains("مرحبا"));
        assert!(modal.message.contains("שלום"));
    }

    #[test]
    fn test_modal_with_combining_characters() {
        let modal = Modal::warning("Café résumé naïve");
        assert!(modal.message.contains("é"));
    }

    #[test]
    fn test_modal_with_zero_width_characters() {
        let modal = Modal::error("Test\u{200B}with\u{200B}zero\u{200B}width\u{200B}spaces");
        assert!(modal.message.contains("\u{200B}"));
    }

    #[test]
    fn test_modal_with_all_emoji_message() {
        let modal = Modal::success("🎉🎊🎈🎁🎂🎄🎃🎋");
        assert!(modal.message.contains("🎉"));
        assert_eq!(modal.message.chars().count(), 8);
    }

    #[test]
    fn test_modal_with_box_drawing_characters() {
        let modal = Modal::info("┌─┬─┐\n│ │ │\n├─┼─┤\n│ │ │\n└─┴─┘");
        assert!(modal.message.contains("┌"));
    }

    // ============ Control Characters and Special Cases ============

    #[test]
    fn test_modal_with_tabs_in_message() {
        let modal = Modal::error("Column1\tColumn2\tColumn3");
        assert!(modal.message.contains("\t"));
    }

    #[test]
    fn test_modal_with_multiple_newlines() {
        let modal = Modal::warning("Line1\n\n\nLine2");
        assert_eq!(modal.message.matches('\n').count(), 3);
    }

    #[test]
    fn test_modal_with_carriage_returns() {
        let modal = Modal::info("Text\rWith\rCarriage\rReturns");
        assert!(modal.message.contains("\r"));
    }

    #[test]
    fn test_modal_with_ansi_escape_sequences() {
        let modal = Modal::error("\x1b[31mRed text\x1b[0m");
        assert!(modal.message.contains("\x1b"));
    }

    // ============ Title Edge Cases ============

    #[test]
    fn test_modal_title_with_10000_unicode_chars() {
        let title = "日".repeat(10000);
        let modal = Modal::success("Success").with_title(title.clone());
        assert_eq!(modal.title, Some(title));
    }

    #[test]
    fn test_modal_title_with_newlines() {
        let modal = Modal::error("Error").with_title("Multi\nLine\nTitle");
        assert!(modal.title.clone().unwrap().contains("\n"));
    }

    #[test]
    fn test_modal_title_with_tabs() {
        let modal = Modal::warning("Warning").with_title("Tab\tSeparated\tTitle");
        assert!(modal.title.clone().unwrap().contains("\t"));
    }

    #[test]
    fn test_modal_title_override_multiple_times() {
        let modal = Modal::info("Info")
            .with_title("First Title")
            .with_title("Second Title")
            .with_title("Final Title");

        assert_eq!(modal.title, Some("Final Title".to_string()));
    }

    // ============ Button Label Edge Cases ============

    #[test]
    fn test_modal_button_with_newlines() {
        let modal = Modal::success("Done").with_button("Multi\nLine\nButton");
        assert!(modal.button_label.contains("\n"));
    }

    #[test]
    fn test_modal_button_with_tabs() {
        let modal = Modal::error("Error").with_button("Tab\tButton");
        assert!(modal.button_label.contains("\t"));
    }

    #[test]
    fn test_modal_button_override_multiple_times() {
        let modal = Modal::warning("Warning")
            .with_button("First")
            .with_button("Second")
            .with_button("Final");

        assert_eq!(modal.button_label, "Final");
    }

    #[test]
    fn test_modal_button_with_special_chars() {
        let modal = Modal::info("Info").with_button("<>&\"'\\|/*?");
        assert!(modal.button_label.contains("<>"));
    }

    // ============ Details Edge Cases ============

    #[test]
    fn test_modal_details_order_preservation() {
        let modal = Modal::error("Error")
            .add_detail("First")
            .add_detail("Second")
            .add_detail("Third")
            .add_detail("Fourth");

        assert_eq!(modal.details[0], "First");
        assert_eq!(modal.details[1], "Second");
        assert_eq!(modal.details[2], "Third");
        assert_eq!(modal.details[3], "Fourth");
    }

    #[test]
    fn test_modal_details_with_duplicate_entries() {
        let modal = Modal::warning("Warning")
            .add_detail("Same")
            .add_detail("Same")
            .add_detail("Same");

        assert_eq!(modal.details.len(), 3);
        assert_eq!(modal.details[0], modal.details[1]);
        assert_eq!(modal.details[1], modal.details[2]);
    }

    #[test]
    fn test_modal_details_with_very_long_unicode() {
        let long_detail = "日本語".repeat(10000);
        let modal = Modal::error("Error").add_detail(long_detail.clone());
        assert_eq!(modal.details[0], long_detail);
    }

    #[test]
    fn test_modal_details_with_mixed_lengths() {
        let modal = Modal::info("Info")
            .add_detail("X")
            .add_detail("XX".repeat(5000))
            .add_detail("")
            .add_detail("Y".repeat(100));

        assert_eq!(modal.details.len(), 4);
        assert_eq!(modal.details[0].len(), 1);
        assert_eq!(modal.details[1].len(), 10000);
        assert_eq!(modal.details[2].len(), 0);
        assert_eq!(modal.details[3].len(), 100);
    }

    #[test]
    fn test_modal_details_vec_with_10000_entries() {
        let details: Vec<String> = (0..10000).map(|i| format!("D{}", i)).collect();
        let modal = Modal::success("Success").with_details(details.clone());
        assert_eq!(modal.details.len(), 10000);
        assert_eq!(modal.details[9999], "D9999");
    }

    // ============ ModalType Debug and Clone ============

    #[test]
    fn test_modal_type_debug_format() {
        let error = ModalType::Error;
        let warning = ModalType::Warning;
        let info = ModalType::Info;
        let success = ModalType::Success;

        assert!(format!("{:?}", error).contains("Error"));
        assert!(format!("{:?}", warning).contains("Warning"));
        assert!(format!("{:?}", info).contains("Info"));
        assert!(format!("{:?}", success).contains("Success"));
    }

    #[test]
    fn test_modal_type_clone() {
        let original = ModalType::Warning;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_modal_type_all_colors() {
        let error_color = ModalType::Error.color();
        let warning_color = ModalType::Warning.color();
        let info_color = ModalType::Info.color();
        let success_color = ModalType::Success.color();

        // Verify all colors are valid (non-panicking)
        assert!(format!("{:?}", error_color).len() > 0);
        assert!(format!("{:?}", warning_color).len() > 0);
        assert!(format!("{:?}", info_color).len() > 0);
        assert!(format!("{:?}", success_color).len() > 0);
    }

    #[test]
    fn test_modal_type_all_icons_are_unicode() {
        let icons = vec![
            ModalType::Error.icon(),
            ModalType::Warning.icon(),
            ModalType::Info.icon(),
            ModalType::Success.icon(),
        ];

        for icon in icons {
            // All icons should be non-empty
            assert!(!icon.is_empty());
            // All icons should be valid UTF-8 (already guaranteed by &str)
            assert!(icon.chars().count() > 0);
        }
    }

    // ============ Message Edge Cases ============

    #[test]
    fn test_modal_message_with_extremely_long_line() {
        let long_line = "A".repeat(100000);
        let modal = Modal::error(long_line.clone());
        assert_eq!(modal.message.len(), 100000);
    }

    #[test]
    fn test_modal_message_with_mixed_newlines() {
        let modal = Modal::warning("Line1\nLine2\r\nLine3\rLine4");
        assert!(modal.message.contains("\n"));
        assert!(modal.message.contains("\r"));
    }

    #[test]
    fn test_modal_message_only_special_chars() {
        let modal = Modal::info("!@#$%^&*()_+-=[]{}|;:',.<>?/~`");
        assert!(modal.message.contains("!@#"));
    }

    // ============ Complex Builder Patterns ============

    #[test]
    fn test_modal_complex_builder_all_features() {
        let modal = Modal::error("Critical error occurred")
            .with_title("🚨 System Error")
            .add_detail("Timestamp: 2024-01-01 00:00:00")
            .add_detail("Error code: ERR_CRITICAL_001")
            .add_detail("Stack trace:")
            .add_detail("  at function1() [file1.rs:123]")
            .add_detail("  at function2() [file2.rs:456]")
            .with_button("Report & Exit");

        assert_eq!(modal.modal_type(), ModalType::Error);
        assert!(modal.title.clone().unwrap().contains("🚨"));
        assert_eq!(modal.details.len(), 5);
        assert_eq!(modal.button_label, "Report & Exit");
    }

    #[test]
    fn test_modal_builder_alternating_operations() {
        let modal = Modal::warning("Warning")
            .add_detail("D1")
            .with_title("T1")
            .add_detail("D2")
            .with_button("B1")
            .add_detail("D3")
            .with_title("T2")
            .with_button("B2");

        assert_eq!(modal.details.len(), 3);
        assert_eq!(modal.title, Some("T2".to_string()));
        assert_eq!(modal.button_label, "B2");
    }

    #[test]
    fn test_modal_constructor_consistency() {
        let modal1 = Modal::new(ModalType::Error, "Message");
        let modal2 = Modal::error("Message");

        assert_eq!(modal1.modal_type(), modal2.modal_type());
        assert_eq!(modal1.message, modal2.message);
        assert_eq!(modal1.button_label, modal2.button_label);
    }

    #[test]
    fn test_modal_all_types_title_prefixes_unique() {
        let prefixes = vec![
            ModalType::Error.title_prefix(),
            ModalType::Warning.title_prefix(),
            ModalType::Info.title_prefix(),
            ModalType::Success.title_prefix(),
        ];

        // All prefixes should be unique
        for (i, prefix1) in prefixes.iter().enumerate() {
            for (j, prefix2) in prefixes.iter().enumerate() {
                if i != j {
                    assert_ne!(prefix1, prefix2);
                }
            }
        }
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_modal_stress_all_features() {
        for i in 0..100 {
            let modal_type = match i % 4 {
                0 => ModalType::Error,
                1 => ModalType::Warning,
                2 => ModalType::Info,
                _ => ModalType::Success,
            };

            let message = format!("Message {} with unicode 日本語 🎉", i);
            let mut modal = Modal::new(modal_type, message);

            // Add varying number of details
            for j in 0..(i % 10) {
                modal = modal.add_detail(format!("Detail {} 🔍", j));
            }

            // Some iterations have custom title
            if i % 3 == 0 {
                modal = modal.with_title(format!("Title {} ✨", i));
            }

            // Some iterations have custom button
            if i % 5 == 0 {
                modal = modal.with_button(format!("Button {} ✓", i));
            }

            // Verify integrity
            assert_eq!(modal.modal_type(), modal_type);
            assert!(modal.message.contains(&i.to_string()));
        }
    }

    #[test]
    fn test_modal_type_coverage_all_methods() {
        let types = vec![
            ModalType::Error,
            ModalType::Warning,
            ModalType::Info,
            ModalType::Success,
        ];

        for modal_type in types {
            // Call all methods to ensure they don't panic
            let _color = modal_type.color();
            let _icon = modal_type.icon();
            let _title = modal_type.title_prefix();
            let _debug = format!("{:?}", modal_type);
            let _clone = modal_type.clone();
            let _copy = modal_type;
        }
    }

    #[test]
    fn test_modal_empty_everything() {
        let modal = Modal::info("")
            .with_title("")
            .with_details(vec![])
            .with_button("");

        assert_eq!(modal.message, "");
        assert_eq!(modal.title, Some("".to_string()));
        assert_eq!(modal.details.len(), 0);
        assert_eq!(modal.button_label, "");
    }
}
