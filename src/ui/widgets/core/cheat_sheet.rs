//! Cheat sheet quick reference overlay
//!
//! Displays a quick reference guide for common keybindings and commands
//! in an easy-to-read overlay format.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::CheatSheet;
//!
//! let mut sheet = CheatSheet::new();
//! sheet.show();
//! assert!(sheet.is_visible());
//! ```

use crate::ui::atoms::{block::Block as AtomBlock, text::Text};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

/// Cheat sheet widget
///
/// Provides a quick reference overlay with common keybindings organized
/// by category for easy lookup.
pub struct CheatSheet {
    /// Whether the cheat sheet is visible
    visible: bool,
    /// Current category (0 = all, 1-N = specific categories)
    category: usize,
}

impl CheatSheet {
    /// Create a new cheat sheet
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::CheatSheet;
    ///
    /// let sheet = CheatSheet::new();
    /// assert!(!sheet.is_visible());
    /// ```
    pub fn new() -> Self {
        Self {
            visible: false,
            category: 0,
        }
    }

    /// Show the cheat sheet
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the cheat sheet
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Switch to next category
    pub fn next_category(&mut self) {
        self.category = (self.category + 1) % 6; // 6 categories total
    }

    /// Switch to previous category
    pub fn previous_category(&mut self) {
        self.category = if self.category == 0 { 5 } else { self.category - 1 };
    }

    /// Get cheat sheet content
    fn get_content(&self) -> Vec<(&'static str, Vec<(&'static str, &'static str)>)> {
        vec![
            ("🚀 General", vec![
                ("Ctrl+P", "Command palette"),
                ("/", "Search"),
                ("?", "Toggle help"),
                ("Ctrl+C", "Quit"),
                ("Esc", "Cancel/Exit mode"),
            ]),
            ("📁 Navigation", vec![
                ("h/j/k/l", "Move left/down/up/right"),
                ("gg", "Jump to top"),
                ("G", "Jump to bottom"),
                ("Ctrl+D/U", "Page down/up"),
                ("Tab", "Switch panes"),
            ]),
            ("✏️  Editing", vec![
                ("i", "Insert mode"),
                ("a", "Append"),
                ("v", "Visual mode"),
                ("y", "Yank (copy)"),
                ("p", "Paste"),
                ("u", "Undo"),
                ("Ctrl+R", "Redo"),
            ]),
            ("🌳 Git", vec![
                ("Space", "Stage/unstage"),
                ("c", "Commit"),
                ("P", "Push"),
                ("F", "Pull"),
                ("b", "Branches"),
                ("d", "Diff"),
            ]),
            ("🔍 Search", vec![
                ("/", "Start search"),
                ("n", "Next result"),
                ("N", "Previous result"),
                ("Ctrl+F", "Find in files"),
            ]),
            ("⚙️  Advanced", vec![
                ("Ctrl+T", "New tab"),
                ("Ctrl+W", "Close tab"),
                ("Alt+1-9", "Switch to tab N"),
                ("Ctrl+B", "Toggle sidebar"),
                ("Ctrl+\\", "Split pane"),
            ]),
        ]
    }
}

impl Default for CheatSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &CheatSheet {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        // Calculate overlay area (80% of screen)
        let width = (area.width as f32 * 0.8).min(100.0) as u16;
        let height = (area.height as f32 * 0.8).min(35.0) as u16;

        let overlay_area = Rect {
            x: (area.width.saturating_sub(width)) / 2 + area.x,
            y: (area.height.saturating_sub(height)) / 2 + area.y,
            width,
            height,
        };

        // Clear background
        Clear.render(overlay_area, buf);

        // Split into title and content
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Content
            Constraint::Length(1),  // Footer
        ])
        .split(overlay_area);

        // Render title using Text atoms
        let title_text = Text::new("TOAD Cheat Sheet")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        let separator = Text::new(" - ");
        let subtitle_text = Text::new("Quick Reference")
            .style(Style::default().fg(Color::Gray));

        let title_line = Line::from(vec![
            title_text.to_span(),
            separator.to_span(),
            subtitle_text.to_span(),
        ]);

        let title_block = AtomBlock::new()
            .border_style(Style::default().fg(Color::Cyan))
            .to_ratatui();

        let title = Paragraph::new(vec![title_line]).block(title_block);
        title.render(chunks[0], buf);

        // Render content in columns
        let content = self.get_content();
        let categories_to_show = if self.category == 0 {
            content.clone()
        } else {
            vec![content[self.category - 1].clone()]
        };

        // Calculate columns
        let cols = if categories_to_show.len() > 3 { 3 } else { categories_to_show.len() };
        let col_width = 100 / cols as u16;

        // Create column constraints
        let column_constraints: Vec<Constraint> = (0..cols)
            .map(|_| Constraint::Percentage(col_width as u16))
            .collect();

        let columns = Layout::horizontal(column_constraints).split(chunks[1]);

        // Render each category in columns using Text atoms
        for (idx, (category_name, bindings)) in categories_to_show.iter().enumerate() {
            if idx >= columns.len() {
                break;
            }

            let category_header = Text::new(*category_name)
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            let mut lines = vec![Line::from(category_header.to_span()), Line::from("")];

            for (key, desc) in bindings {
                let key_text = Text::new(format!("{:12}", key))
                    .style(Style::default().fg(Color::Green));
                let separator = Text::new(" → ");
                let desc_text =
                    Text::new(*desc).style(Style::default().fg(Color::White));

                lines.push(Line::from(vec![
                    key_text.to_span(),
                    separator.to_span(),
                    desc_text.to_span(),
                ]));
            }

            let block = AtomBlock::new().to_ratatui();
            let para = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });

            para.render(columns[idx], buf);
        }

        // Render footer
        let footer_text = "Tab: Next category | Shift+Tab: Previous | Esc/?: Close";
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::DarkGray));
        footer.render(chunks[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheat_sheet_new() {
        let sheet = CheatSheet::new();
        assert!(!sheet.is_visible());
    }

    #[test]
    fn test_cheat_sheet_visibility() {
        let mut sheet = CheatSheet::new();

        sheet.show();
        assert!(sheet.is_visible());

        sheet.hide();
        assert!(!sheet.is_visible());

        sheet.toggle();
        assert!(sheet.is_visible());
    }

    #[test]
    fn test_cheat_sheet_categories() {
        let mut sheet = CheatSheet::new();

        assert_eq!(sheet.category, 0);

        sheet.next_category();
        assert_eq!(sheet.category, 1);

        sheet.previous_category();
        assert_eq!(sheet.category, 0);
    }

    #[test]
    fn test_cheat_sheet_content() {
        let sheet = CheatSheet::new();
        let content = sheet.get_content();

        assert!(!content.is_empty());
        assert!(content.len() >= 5); // At least 5 categories
    }
}
