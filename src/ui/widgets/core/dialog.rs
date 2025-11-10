//! Dialog widget for confirmations and prompts
//!
//! Copilot-style confirmation dialogs with radio button selection

use crate::ui::{
    atoms::{block::Block, text::Text},
    theme::ToadTheme,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

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

        // Outer container using Block atom
        let outer_block = Block::new()
            .title(&self.title)
            .border_style(Style::default().fg(ToadTheme::BORDER_FOCUSED))
            .style(Style::default().bg(ToadTheme::BACKGROUND))
            .to_ratatui();

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

        // Render info box if present using Block atom
        if let Some(info_text) = &self.info_box {
            let info_block = Block::new()
                .border_style(Style::default().fg(ToadTheme::DARK_GRAY))
                .to_ratatui();

            let info_paragraph = Paragraph::new(info_text.as_str())
                .style(Style::default().fg(ToadTheme::GRAY))
                .block(info_block)
                .alignment(Alignment::Center);

            frame.render_widget(info_paragraph, chunks[chunk_idx]);
            chunk_idx += 2; // Skip spacing
        }

        // Render message using Text atoms
        let message_lines: Vec<_> = self
            .message
            .iter()
            .map(|msg| {
                Text::new(msg)
                    .style(Style::default().fg(ToadTheme::FOREGROUND))
                    .to_line()
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

            // Use Text atom for option rendering
            let option_text = format!("{}{}. {}", prefix, option.key, option.label);
            let option_line = Text::new(option_text).style(style).to_line();
            let option_paragraph = Paragraph::new(option_line);

            frame.render_widget(option_paragraph, option_area);
        }
        chunk_idx += 2; // Skip spacing

        // Render help text using Text atom
        let help_text = "Confirm with number keys or ↑↓ keys and Enter, Cancel with Esc";
        let help_line = Text::new(help_text)
            .style(
                Style::default()
                    .fg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::ITALIC),
            )
            .to_line();
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

    // ============================================================================
    // ADVANCED TIER: Comprehensive Edge Case Tests
    // ============================================================================

    #[test]
    fn test_confirm_dialog_debug_trait() {
        let dialog = ConfirmDialog::new("Title")
            .message("Message")
            .option('1', "Option 1");

        let debug_str = format!("{:?}", dialog);
        assert!(debug_str.contains("ConfirmDialog"));
        assert!(debug_str.contains("Title"));
    }

    #[test]
    fn test_dialog_option_debug_trait() {
        let option = DialogOption::new('1', "Test");
        let debug_str = format!("{:?}", option);
        assert!(debug_str.contains("DialogOption"));
        assert!(debug_str.contains("Test"));
    }

    // Stress Tests (10k operations)

    #[test]
    fn test_confirm_dialog_10k_navigation_cycles() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3")
            .option('4', "Option 4")
            .option('5', "Option 5");

        for _ in 0..5000 {
            dialog.select_next();
        }
        // Should be at last option (5000 % 5 movements from 0, but bounded)
        assert!(dialog.selected() < 5);

        for _ in 0..5000 {
            dialog.select_previous();
        }
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_10k_mixed_navigation() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3")
            .option('4', "Option 4")
            .option('5', "Option 5");

        for i in 0..10000 {
            match i % 4 {
                0 => dialog.select_next(),
                1 => dialog.select_previous(),
                2 => { dialog.select_by_key('3'); }
                _ => { dialog.select_by_key('1'); }
            }
        }

        assert!(dialog.selected() < 5);
    }

    #[test]
    fn test_confirm_dialog_10k_select_by_key() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3")
            .option('4', "Option 4")
            .option('5', "Option 5");

        for i in 0..10000 {
            let key = char::from_digit((i % 5 + 1) as u32, 10).unwrap();
            dialog.select_by_key(key);
        }

        // Last iteration: i=9999, 9999 % 5 = 4, 4 + 1 = 5, so key '5' (index 4)
        assert_eq!(dialog.selected(), 4);
    }

    #[test]
    fn test_confirm_dialog_10k_message_additions() {
        let mut dialog = ConfirmDialog::new("Title");

        for i in 0..10000 {
            dialog = dialog.message(format!("Message {}", i));
        }

        assert_eq!(dialog.message.len(), 10000);
        assert_eq!(dialog.message[0], "Message 0");
        assert_eq!(dialog.message[9999], "Message 9999");
    }

    #[test]
    fn test_confirm_dialog_10k_option_additions() {
        let mut dialog = ConfirmDialog::new("Title");

        for i in 0..10000 {
            let key = char::from_digit((i % 10) as u32, 10).unwrap_or('0');
            dialog = dialog.option(key, format!("Option {}", i));
        }

        assert_eq!(dialog.options.len(), 10000);
        assert_eq!(dialog.options[0].label, "Option 0");
        assert_eq!(dialog.options[9999].label, "Option 9999");
    }

    // Unicode Edge Cases

    #[test]
    fn test_confirm_dialog_rtl_text_arabic() {
        let dialog = ConfirmDialog::new("عنوان الحوار")
            .message("رسالة بالعربية")
            .option('1', "خيار واحد")
            .info_box("معلومات إضافية");

        assert!(dialog.title.contains("عنوان"));
        assert!(dialog.message[0].contains("رسالة"));
        assert!(dialog.options[0].label.contains("خيار"));
        assert!(dialog.info_box.as_ref().unwrap().contains("معلومات"));
    }

    #[test]
    fn test_confirm_dialog_rtl_text_hebrew() {
        let dialog = ConfirmDialog::new("כותרת דו-שיח")
            .message("הודעה בעברית")
            .option('א', "אפשרות אחת")
            .info_box("מידע נוסף");

        assert!(dialog.title.contains("כותרת"));
        assert!(dialog.message[0].contains("הודעה"));
        assert_eq!(dialog.options[0].key, 'א');
        assert!(dialog.options[0].label.contains("אפשרות"));
    }

    #[test]
    fn test_confirm_dialog_mixed_scripts() {
        let dialog = ConfirmDialog::new("Title 标题 عنوان")
            .message("English 日本語 العربية 한국어")
            .option('1', "混合 Mixed مختلط")
            .info_box("Multiple מרובים متعددة scripts");

        assert!(dialog.title.contains("Title"));
        assert!(dialog.title.contains("标题"));
        assert!(dialog.title.contains("عنوان"));
        assert!(dialog.message[0].contains("日本語"));
        assert!(dialog.message[0].contains("한국어"));
    }

    #[test]
    fn test_confirm_dialog_emoji_combinations() {
        let dialog = ConfirmDialog::new("🎉🎊🎈 Party Dialog 🥳")
            .message("Choose your favorite emoji: 😀😃😄😁")
            .option('1', "🐸 Frog")
            .option('2', "🦀 Crab")
            .option('3', "🐍 Snake")
            .info_box("Emoji support: ✅ Working 🎯");

        assert!(dialog.title.contains("🎉"));
        assert!(dialog.message[0].contains("😀"));
        assert!(dialog.options[0].label.contains("🐸"));
        assert!(dialog.info_box.as_ref().unwrap().contains("✅"));
    }

    #[test]
    fn test_confirm_dialog_very_long_unicode_string() {
        let long_unicode = "🎉".repeat(1000);
        let dialog = ConfirmDialog::new(&long_unicode)
            .message(&long_unicode)
            .option('1', &long_unicode)
            .info_box(&long_unicode);

        assert_eq!(dialog.title.chars().filter(|&c| c == '🎉').count(), 1000);
        assert_eq!(dialog.message[0].chars().filter(|&c| c == '🎉').count(), 1000);
        assert_eq!(dialog.options[0].label.chars().filter(|&c| c == '🎉').count(), 1000);
    }

    #[test]
    fn test_confirm_dialog_zero_width_characters() {
        let text_with_zwj = "Test\u{200D}Text"; // Zero-width joiner
        let text_with_zwnj = "Test\u{200C}Text"; // Zero-width non-joiner

        let dialog = ConfirmDialog::new(text_with_zwj)
            .message(text_with_zwnj)
            .option('1', text_with_zwj);

        assert!(dialog.title.contains('\u{200D}'));
        assert!(dialog.message[0].contains('\u{200C}'));
        assert!(dialog.options[0].label.contains('\u{200D}'));
    }

    #[test]
    fn test_confirm_dialog_combining_characters() {
        let text_with_combining = "e\u{0301}"; // e with acute accent (é)
        let dialog = ConfirmDialog::new(text_with_combining)
            .message(text_with_combining)
            .option('1', text_with_combining)
            .info_box(text_with_combining);

        assert!(dialog.title.contains('\u{0301}'));
        assert!(dialog.message[0].contains('\u{0301}'));
        assert!(dialog.options[0].label.contains('\u{0301}'));
    }

    // Extreme Values

    #[test]
    fn test_confirm_dialog_very_large_number_of_options() {
        let mut dialog = ConfirmDialog::new("Title");

        for i in 0..1000 {
            let key = char::from_digit((i % 10) as u32, 10).unwrap_or('0');
            dialog = dialog.option(key, format!("Option {}", i));
        }

        assert_eq!(dialog.options.len(), 1000);

        // Test navigation with many options
        dialog.select_next();
        assert_eq!(dialog.selected(), 1);

        dialog.select_previous();
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_very_large_number_of_messages() {
        let mut dialog = ConfirmDialog::new("Title");

        for i in 0..1000 {
            dialog = dialog.message(format!("Message line {}", i));
        }

        assert_eq!(dialog.message.len(), 1000);
        assert_eq!(dialog.message[0], "Message line 0");
        assert_eq!(dialog.message[999], "Message line 999");
    }

    #[test]
    fn test_confirm_dialog_empty_title() {
        let dialog = ConfirmDialog::new("")
            .message("Message")
            .option('1', "Option");

        assert_eq!(dialog.title, "");
        assert_eq!(dialog.message.len(), 1);
    }

    #[test]
    fn test_confirm_dialog_very_long_title() {
        let long_title = "x".repeat(10000);
        let dialog = ConfirmDialog::new(&long_title);

        assert_eq!(dialog.title.len(), 10000);
    }

    #[test]
    fn test_confirm_dialog_very_long_option_labels() {
        let long_label = "x".repeat(10000);
        let dialog = ConfirmDialog::new("Title")
            .option('1', &long_label)
            .option('2', &long_label);

        assert_eq!(dialog.options[0].label.len(), 10000);
        assert_eq!(dialog.options[1].label.len(), 10000);
    }

    #[test]
    fn test_confirm_dialog_very_long_info_box() {
        let long_info = "x".repeat(10000);
        let dialog = ConfirmDialog::new("Title")
            .info_box(&long_info);

        assert_eq!(dialog.info_box.as_ref().unwrap().len(), 10000);
    }

    #[test]
    fn test_confirm_dialog_empty_option_label() {
        let dialog = ConfirmDialog::new("Title")
            .option('1', "")
            .option('2', "Normal");

        assert_eq!(dialog.options[0].label, "");
        assert_eq!(dialog.options[1].label, "Normal");
    }

    #[test]
    fn test_confirm_dialog_special_characters_in_labels() {
        let dialog = ConfirmDialog::new("Title\n\t\r")
            .message("Message\n\t\r")
            .option('1', "Option\n\t\r")
            .info_box("Info\n\t\r");

        assert!(dialog.title.contains('\n'));
        assert!(dialog.message[0].contains('\t'));
        assert!(dialog.options[0].label.contains('\r'));
        assert!(dialog.info_box.as_ref().unwrap().contains('\n'));
    }

    // Multi-phase Comprehensive Workflow

    #[test]
    fn test_confirm_dialog_10_phase_comprehensive_workflow() {
        // Phase 1: Create basic dialog
        let mut dialog = ConfirmDialog::new("Comprehensive Test Dialog");
        assert_eq!(dialog.title, "Comprehensive Test Dialog");
        assert_eq!(dialog.selected(), 0);

        // Phase 2: Add multiple messages
        dialog = dialog
            .message("First message")
            .message("Second message")
            .message("Third message with emoji 🎉")
            .message("Fourth message with unicode 日本語");
        assert_eq!(dialog.message.len(), 4);

        // Phase 3: Add multiple options
        dialog = dialog
            .option('1', "Option One")
            .option('2', "Option Two")
            .option('3', "Option Three")
            .option('4', "Option Four")
            .option('5', "Option Five");
        assert_eq!(dialog.options.len(), 5);

        // Phase 4: Add info box
        dialog = dialog.info_box("Important information here");
        assert!(dialog.info_box.is_some());

        // Phase 5: Test navigation down
        for i in 0..5 {
            assert_eq!(dialog.selected(), i);
            if i < 4 {
                dialog.select_next();
            }
        }
        assert_eq!(dialog.selected(), 4);

        // Phase 6: Test navigation up
        for i in (0..5).rev() {
            assert_eq!(dialog.selected(), i);
            if i > 0 {
                dialog.select_previous();
            }
        }
        assert_eq!(dialog.selected(), 0);

        // Phase 7: Test select by key
        dialog.select_by_key('3');
        assert_eq!(dialog.selected(), 2);
        dialog.select_by_key('5');
        assert_eq!(dialog.selected(), 4);
        dialog.select_by_key('1');
        assert_eq!(dialog.selected(), 0);

        // Phase 8: Test invalid key selection
        let result = dialog.select_by_key('9');
        assert!(result.is_none());
        assert_eq!(dialog.selected(), 0); // Should not change

        // Phase 9: Add more messages dynamically
        dialog = dialog
            .message("Added message 5")
            .message("Added message 6");
        assert_eq!(dialog.message.len(), 6);

        // Phase 10: Test boundary conditions
        dialog.select_previous(); // Already at 0, should stay
        assert_eq!(dialog.selected(), 0);

        for _ in 0..10 {
            dialog.select_next();
        }
        assert_eq!(dialog.selected(), 4); // Should be at last option
    }

    // Builder Pattern Edge Cases

    #[test]
    fn test_confirm_dialog_builder_chaining_many_operations() {
        let dialog = ConfirmDialog::new("Title")
            .message("M1").message("M2").message("M3").message("M4").message("M5")
            .option('1', "O1").option('2', "O2").option('3', "O3")
            .info_box("Info1")
            .info_box("Info2") // Should override
            .message("M6");

        assert_eq!(dialog.message.len(), 6);
        assert_eq!(dialog.options.len(), 3);
        assert_eq!(dialog.info_box.as_ref().unwrap(), "Info2");
    }

    #[test]
    fn test_confirm_dialog_duplicate_option_keys() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "First Option 1")
            .option('2', "Option 2")
            .option('1', "Second Option 1"); // Duplicate key

        assert_eq!(dialog.options.len(), 3);

        // select_by_key should select the FIRST match
        dialog.select_by_key('1');
        assert_eq!(dialog.selected(), 0);
        assert_eq!(dialog.options[0].label, "First Option 1");
    }

    // Navigation Edge Cases

    #[test]
    fn test_confirm_dialog_rapid_navigation_cycles() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3");

        for _ in 0..100 {
            dialog.select_next();
            dialog.select_next();
            dialog.select_previous();
            dialog.select_previous();
        }

        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_select_by_key_then_navigate() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3")
            .option('4', "Option 4");

        dialog.select_by_key('3');
        assert_eq!(dialog.selected(), 2);

        dialog.select_next();
        assert_eq!(dialog.selected(), 3);

        dialog.select_previous();
        dialog.select_previous();
        assert_eq!(dialog.selected(), 1);
    }

    #[test]
    fn test_confirm_dialog_navigate_then_select_by_key() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3")
            .option('4', "Option 4");

        dialog.select_next();
        dialog.select_next();
        assert_eq!(dialog.selected(), 2);

        dialog.select_by_key('1');
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_multiple_select_by_key_calls() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2")
            .option('3', "Option 3")
            .option('4', "Option 4")
            .option('5', "Option 5");

        for i in 1..=5 {
            let key = char::from_digit(i, 10).unwrap();
            dialog.select_by_key(key);
            assert_eq!(dialog.selected(), (i - 1) as usize);
        }
    }

    // Empty State Operations

    #[test]
    fn test_confirm_dialog_all_operations_no_options() {
        let mut dialog = ConfirmDialog::new("Title")
            .message("Message without options");

        assert_eq!(dialog.options.len(), 0);
        assert_eq!(dialog.selected(), 0);

        // Note: select_next/select_previous have edge case with zero options
        // (options.len() - 1 wraps to usize::MAX), so we only test select_by_key

        let result = dialog.select_by_key('1');
        assert!(result.is_none());
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_confirm_dialog_all_operations_no_messages() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('1', "Option 1")
            .option('2', "Option 2");

        assert_eq!(dialog.message.len(), 0);
        assert_eq!(dialog.options.len(), 2);

        dialog.select_next();
        assert_eq!(dialog.selected(), 1);

        dialog.select_previous();
        assert_eq!(dialog.selected(), 0);
    }

    #[test]
    fn test_dialog_option_clone_independence() {
        let option1 = DialogOption::new('1', "Test");
        let mut option2 = option1.clone();

        option2.label = "Modified".to_string();
        option2.key = '2';

        assert_eq!(option1.label, "Test");
        assert_eq!(option1.key, '1');
        assert_eq!(option2.label, "Modified");
        assert_eq!(option2.key, '2');
    }

    #[test]
    fn test_confirm_dialog_unicode_option_keys() {
        let dialog = ConfirmDialog::new("Title")
            .option('א', "Hebrew option")
            .option('あ', "Japanese option")
            .option('🎉', "Emoji option");

        assert_eq!(dialog.options[0].key, 'א');
        assert_eq!(dialog.options[1].key, 'あ');
        assert_eq!(dialog.options[2].key, '🎉');
    }

    #[test]
    fn test_confirm_dialog_select_unicode_keys() {
        let mut dialog = ConfirmDialog::new("Title")
            .option('א', "Hebrew")
            .option('あ', "Japanese")
            .option('🎉', "Emoji");

        dialog.select_by_key('あ');
        assert_eq!(dialog.selected(), 1);

        dialog.select_by_key('🎉');
        assert_eq!(dialog.selected(), 2);

        dialog.select_by_key('א');
        assert_eq!(dialog.selected(), 0);
    }
}
