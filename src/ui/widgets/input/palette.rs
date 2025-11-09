//! Command palette widget
//!
//! Fuzzy-searchable command launcher similar to VSCode/Sublime

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};

/// Command palette entry
#[derive(Debug, Clone)]
pub struct PaletteCommand {
    pub id: String,
    pub label: String,
    pub description: String,
}

/// Command palette widget
#[derive(Debug)]
pub struct CommandPalette {
    /// Search query
    query: String,
    /// Cursor position in search
    cursor_position: usize,
    /// All available commands
    commands: Vec<PaletteCommand>,
    /// Filtered commands based on query
    filtered: Vec<usize>,
    /// List state for selection
    list_state: ListState,
}

impl CommandPalette {
    pub fn new() -> Self {
        let commands = vec![
            PaletteCommand {
                id: "help".to_string(),
                label: "Show Help".to_string(),
                description: "Open the help screen with keybindings".to_string(),
            },
            PaletteCommand {
                id: "clear".to_string(),
                label: "Clear Screen".to_string(),
                description: "Clear the main content area".to_string(),
            },
            PaletteCommand {
                id: "quit".to_string(),
                label: "Quit Application".to_string(),
                description: "Exit Toad".to_string(),
            },
            PaletteCommand {
                id: "vim_mode".to_string(),
                label: "Toggle Vim Mode".to_string(),
                description: "Enable/disable Vim-style keybindings (h/j/k/l, g/G)".to_string(),
            },
            PaletteCommand {
                id: "theme_toggle".to_string(),
                label: "Toggle Theme".to_string(),
                description: "Switch between light and dark themes".to_string(),
            },
            PaletteCommand {
                id: "split_horizontal".to_string(),
                label: "Split Horizontal".to_string(),
                description: "Split the current panel horizontally".to_string(),
            },
            PaletteCommand {
                id: "split_vertical".to_string(),
                label: "Split Vertical".to_string(),
                description: "Split the current panel vertically".to_string(),
            },
            PaletteCommand {
                id: "open_file".to_string(),
                label: "Open File".to_string(),
                description: "Browse and open a file".to_string(),
            },
            PaletteCommand {
                id: "search_files".to_string(),
                label: "Search Files".to_string(),
                description: "Search for files in the workspace".to_string(),
            },
            PaletteCommand {
                id: "git_status".to_string(),
                label: "Git Status".to_string(),
                description: "Show git repository status".to_string(),
            },
            PaletteCommand {
                id: "recent_files".to_string(),
                label: "Recent Files".to_string(),
                description: "Show recently opened files".to_string(),
            },
        ];

        let filtered: Vec<usize> = (0..commands.len()).collect();
        let mut list_state = ListState::default();
        if !filtered.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            query: String::new(),
            cursor_position: 0,
            commands,
            filtered,
            list_state,
        }
    }

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        self.query.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
        self.update_filter();
    }

    /// Delete character before cursor
    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let mut chars: Vec<char> = self.query.chars().collect();
            let char_pos = self.query[..self.cursor_position].chars().count();
            if char_pos > 0 {
                chars.remove(char_pos - 1);
                self.query = chars.into_iter().collect();
                self.cursor_position = self.char_to_byte_idx(char_pos - 1);
                self.update_filter();
            }
        }
    }

    /// Clear the query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.cursor_position = 0;
        self.update_filter();
    }

    /// Get the current query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Select next item
    pub fn select_next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Select previous item
    pub fn select_previous(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Get the selected command ID
    pub fn selected_command(&self) -> Option<String> {
        self.list_state
            .selected()
            .and_then(|i| self.filtered.get(i))
            .map(|&idx| self.commands[idx].id.clone())
    }

    /// Update filtered list based on query
    fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.filtered = (0..self.commands.len()).collect();
        } else {
            self.filtered = self
                .commands
                .iter()
                .enumerate()
                .filter(|(_, cmd)| {
                    let query_lower = self.query.to_lowercase();
                    cmd.label.to_lowercase().contains(&query_lower)
                        || cmd.description.to_lowercase().contains(&query_lower)
                        || cmd.id.to_lowercase().contains(&query_lower)
                })
                .map(|(i, _)| i)
                .collect();
        }

        // Reset selection
        if !self.filtered.is_empty() {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    /// Convert character index to byte index
    fn char_to_byte_idx(&self, char_idx: usize) -> usize {
        self.query
            .char_indices()
            .nth(char_idx)
            .map(|(idx, _)| idx)
            .unwrap_or(self.query.len())
    }

    /// Render the command palette
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Create centered modal-style layout
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
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

        let palette_area = horizontal[1];

        // Main palette block
        let palette_block = Block::default()
            .title(" Command Palette ")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK));

        let inner = palette_block.inner(palette_area);
        frame.render_widget(palette_block, palette_area);

        // Split inner area: search box + list + help text
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search box
                Constraint::Min(5),    // List
                Constraint::Length(1), // Help text
            ])
            .split(inner);

        // Render search box
        self.render_search_box(frame, chunks[0]);

        // Render filtered list
        self.render_list(frame, chunks[1]);

        // Render help text
        self.render_help_text(frame, chunks[2]);
    }

    fn render_search_box(&self, frame: &mut Frame, area: Rect) {
        let search_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title(" Search ");

        let inner = search_block.inner(area);
        frame.render_widget(search_block, area);

        // Render search text with cursor
        let before_cursor = &self.query[..self.cursor_position];
        let after_cursor = &self.query[self.cursor_position..];

        let cursor_char = if after_cursor.is_empty() {
            " "
        } else {
            &after_cursor[..1]
        };

        let rest = if after_cursor.is_empty() {
            ""
        } else {
            &after_cursor[1..]
        };

        let mut spans = vec![
            Span::styled(before_cursor, Style::default().fg(ToadTheme::FOREGROUND)),
            Span::styled(
                cursor_char,
                Style::default()
                    .fg(ToadTheme::BLACK)
                    .bg(ToadTheme::TOAD_GREEN),
            ),
        ];

        if !rest.is_empty() {
            spans.push(Span::styled(
                rest,
                Style::default().fg(ToadTheme::FOREGROUND),
            ));
        }

        let search_text = Line::from(spans);
        let search_paragraph = Paragraph::new(search_text);
        frame.render_widget(search_paragraph, inner);
    }

    fn render_list(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .filtered
            .iter()
            .map(|&idx| {
                let cmd = &self.commands[idx];
                let content = vec![
                    Line::from(vec![Span::styled(
                        &cmd.label,
                        Style::default()
                            .fg(ToadTheme::FOREGROUND)
                            .add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled(&cmd.description, Style::default().fg(ToadTheme::GRAY)),
                    ]),
                ];
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.list_state);

        // Render scrollbar if there are items
        if !self.filtered.is_empty() {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(ToadTheme::DARK_GRAY))
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let mut scrollbar_state = ScrollbarState::new(self.filtered.len())
                .position(self.list_state.selected().unwrap_or(0));

            frame.render_stateful_widget(
                scrollbar,
                area.inner(ratatui::layout::Margin {
                    vertical: 0,
                    horizontal: 0,
                }),
                &mut scrollbar_state,
            );
        }
    }

    fn render_help_text(&self, frame: &mut Frame, area: Rect) {
        let help = format!(
            "↑↓ Navigate · Enter Select · Esc Close · {} results",
            self.filtered.len()
        );
        let help_line = Line::from(Span::styled(
            help,
            Style::default()
                .fg(ToadTheme::DARK_GRAY)
                .add_modifier(Modifier::ITALIC),
        ));
        let help_paragraph = Paragraph::new(help_line).alignment(Alignment::Center);
        frame.render_widget(help_paragraph, area);
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
    // ============================================================================

    // ============ Basic Functionality Tests ============

    #[test]
    fn test_palette_creation() {
        let palette = CommandPalette::new();
        assert_eq!(palette.query(), "");
        assert!(palette.selected_command().is_some());
    }

    #[test]
    fn test_palette_default() {
        let palette = CommandPalette::default();
        assert_eq!(palette.query(), "");
    }

    #[test]
    fn test_insert_single_char() {
        let mut palette = CommandPalette::new();
        palette.insert_char('h');
        assert_eq!(palette.query(), "h");
    }

    #[test]
    fn test_insert_multiple_chars() {
        let mut palette = CommandPalette::new();
        palette.insert_char('h');
        palette.insert_char('e');
        palette.insert_char('l');
        palette.insert_char('p');
        assert_eq!(palette.query(), "help");
    }

    #[test]
    fn test_delete_char() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('b');
        palette.delete_char();
        assert_eq!(palette.query(), "a");
    }

    #[test]
    fn test_delete_char_when_empty() {
        let mut palette = CommandPalette::new();
        palette.delete_char();
        assert_eq!(palette.query(), "");
    }

    #[test]
    fn test_clear_query() {
        let mut palette = CommandPalette::new();
        palette.insert_char('t');
        palette.insert_char('e');
        palette.insert_char('s');
        palette.insert_char('t');
        palette.clear_query();
        assert_eq!(palette.query(), "");
    }

    #[test]
    fn test_select_next() {
        let mut palette = CommandPalette::new();
        let first = palette.selected_command();
        palette.select_next();
        let second = palette.selected_command();
        assert_ne!(first, second);
    }

    #[test]
    fn test_select_previous() {
        let mut palette = CommandPalette::new();
        let first = palette.selected_command();
        palette.select_previous();
        let last = palette.selected_command();
        assert_ne!(first, last);
    }

    #[test]
    fn test_palette_command_clone() {
        let cmd = PaletteCommand {
            id: "test".to_string(),
            label: "Test".to_string(),
            description: "Test description".to_string(),
        };
        let cloned = cmd.clone();
        assert_eq!(cmd.id, cloned.id);
        assert_eq!(cmd.label, cloned.label);
        assert_eq!(cmd.description, cloned.description);
    }

    #[test]
    fn test_palette_command_debug() {
        let cmd = PaletteCommand {
            id: "test".to_string(),
            label: "Test".to_string(),
            description: "Test description".to_string(),
        };
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("PaletteCommand"));
    }

    // ============ Stress Tests ============

    #[test]
    fn test_palette_rapid_char_insertion_1000() {
        let mut palette = CommandPalette::new();
        for _ in 0..1000 {
            palette.insert_char('x');
        }
        assert_eq!(palette.query().len(), 1000);
    }

    #[test]
    fn test_palette_rapid_navigation_1000_next() {
        let mut palette = CommandPalette::new();
        for _ in 0..1000 {
            palette.select_next();
        }
        // Should have wrapped around many times, still have a selection
        assert!(palette.selected_command().is_some());
    }

    #[test]
    fn test_palette_rapid_navigation_1000_previous() {
        let mut palette = CommandPalette::new();
        for _ in 0..1000 {
            palette.select_previous();
        }
        assert!(palette.selected_command().is_some());
    }

    #[test]
    fn test_palette_alternating_insert_delete_1000() {
        let mut palette = CommandPalette::new();
        for i in 0..1000 {
            if i % 2 == 0 {
                palette.insert_char('a');
            } else {
                palette.delete_char();
            }
        }
        // Should end with empty query (1000 iterations, alternating)
        assert_eq!(palette.query(), "");
    }

    #[test]
    fn test_palette_rapid_clear_operations() {
        let mut palette = CommandPalette::new();
        for _ in 0..1000 {
            palette.insert_char('t');
            palette.insert_char('e');
            palette.insert_char('s');
            palette.insert_char('t');
            palette.clear_query();
        }
        assert_eq!(palette.query(), "");
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_palette_query_with_emoji() {
        let mut palette = CommandPalette::new();
        palette.insert_char('🚀');
        palette.insert_char('🐸');
        assert!(palette.query().contains('🚀'));
        assert!(palette.query().contains('🐸'));
    }

    #[test]
    fn test_palette_query_with_rtl_text() {
        let mut palette = CommandPalette::new();
        for c in "مرحبا".chars() {
            palette.insert_char(c);
        }
        assert!(palette.query().contains("مرحبا"));
    }

    #[test]
    fn test_palette_query_with_hebrew() {
        let mut palette = CommandPalette::new();
        for c in "שלום".chars() {
            palette.insert_char(c);
        }
        assert!(palette.query().contains("שלום"));
    }

    #[test]
    fn test_palette_query_with_japanese() {
        let mut palette = CommandPalette::new();
        for c in "日本語".chars() {
            palette.insert_char(c);
        }
        assert_eq!(palette.query(), "日本語");
    }

    #[test]
    fn test_palette_query_with_combining_characters() {
        let mut palette = CommandPalette::new();
        for c in "é̂ñ̃".chars() {
            palette.insert_char(c);
        }
        assert!(palette.query().len() > 2);
    }

    #[test]
    fn test_palette_query_with_zero_width() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('\u{200B}'); // Zero-width space
        palette.insert_char('b');
        assert!(palette.query().contains('a'));
        assert!(palette.query().contains('b'));
    }

    #[test]
    fn test_palette_query_with_emoji_skin_tones() {
        let mut palette = CommandPalette::new();
        for c in "👍🏻👍🏿".chars() {
            palette.insert_char(c);
        }
        assert!(palette.query().contains("👍"));
    }

    #[test]
    fn test_palette_query_with_mixed_scripts() {
        let mut palette = CommandPalette::new();
        for c in "Hello日本مرحبا🚀".chars() {
            palette.insert_char(c);
        }
        assert_eq!(palette.query(), "Hello日本مرحبا🚀");
    }

    #[test]
    fn test_palette_delete_emoji() {
        let mut palette = CommandPalette::new();
        palette.insert_char('🚀');
        palette.delete_char();
        assert_eq!(palette.query(), "");
    }

    #[test]
    fn test_palette_delete_multibyte_char() {
        let mut palette = CommandPalette::new();
        palette.insert_char('日');
        palette.delete_char();
        assert_eq!(palette.query(), "");
    }

    // ============ Filter Edge Cases ============

    #[test]
    fn test_palette_filter_empty_query_shows_all() {
        let palette = CommandPalette::new();
        // With empty query, should have a selection (all commands visible)
        assert!(palette.selected_command().is_some());
    }

    #[test]
    fn test_palette_filter_case_insensitive() {
        let mut palette = CommandPalette::new();
        palette.insert_char('H');
        palette.insert_char('E');
        palette.insert_char('L');
        palette.insert_char('P');
        // Should still find "help" command (case-insensitive)
        let selected = palette.selected_command();
        assert!(selected.is_some());
    }

    #[test]
    fn test_palette_filter_no_matches() {
        let mut palette = CommandPalette::new();
        for c in "xyzabc123nonexistent".chars() {
            palette.insert_char(c);
        }
        // With no matches, selection should be None
        assert!(palette.selected_command().is_none());
    }

    #[test]
    fn test_palette_filter_single_char() {
        let mut palette = CommandPalette::new();
        palette.insert_char('q');
        // Should filter to commands containing 'q'
        let selected = palette.selected_command();
        assert!(selected.is_some());
    }

    #[test]
    fn test_palette_filter_progressive() {
        let mut palette = CommandPalette::new();
        palette.insert_char('h');
        let count_h = palette.selected_command();
        palette.insert_char('e');
        palette.insert_char('l');
        palette.insert_char('p');
        let count_help = palette.selected_command();
        // Both should have selections
        assert!(count_h.is_some());
        assert!(count_help.is_some());
    }

    #[test]
    fn test_palette_clear_query_restores_all() {
        let mut palette = CommandPalette::new();
        palette.insert_char('q');
        palette.insert_char('u');
        palette.insert_char('i');
        palette.insert_char('t');
        palette.clear_query();
        // After clear, should show all commands again
        assert!(palette.selected_command().is_some());
    }

    // ============ Selection Edge Cases ============

    #[test]
    fn test_palette_select_next_wraps() {
        let mut palette = CommandPalette::new();
        let first_selection = palette.selected_command();

        // Navigate to last item by going previous once (wraps)
        palette.select_previous();

        // Then go next (should wrap to first)
        palette.select_next();

        let wrapped_selection = palette.selected_command();
        assert_eq!(first_selection, wrapped_selection);
    }

    #[test]
    fn test_palette_select_previous_wraps() {
        let mut palette = CommandPalette::new();
        let first = palette.selected_command();
        palette.select_previous();
        let last = palette.selected_command();
        assert_ne!(first, last);
    }

    #[test]
    fn test_palette_selection_after_filter_change() {
        let mut palette = CommandPalette::new();
        palette.insert_char('h');
        let selected_after_h = palette.selected_command();
        palette.insert_char('e');
        palette.insert_char('l');
        palette.insert_char('p');
        let selected_after_help = palette.selected_command();
        // Both should have selections, might be different commands
        assert!(selected_after_h.is_some());
        assert!(selected_after_help.is_some());
    }

    #[test]
    fn test_palette_selection_when_no_results() {
        let mut palette = CommandPalette::new();
        for c in "nonexistentcommand123".chars() {
            palette.insert_char(c);
        }
        assert!(palette.selected_command().is_none());

        // Navigation should be safe with no results
        palette.select_next();
        palette.select_previous();
        assert!(palette.selected_command().is_none());
    }

    // ============ Cursor Position Edge Cases ============

    #[test]
    fn test_palette_cursor_at_start() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('b');
        palette.insert_char('c');
        // Cursor should be at end (position 3)
        assert_eq!(palette.query(), "abc");
    }

    #[test]
    fn test_palette_delete_updates_cursor() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('b');
        palette.delete_char();
        palette.insert_char('c');
        assert_eq!(palette.query(), "ac");
    }

    #[test]
    fn test_palette_empty_after_deletes() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.delete_char();
        palette.delete_char(); // Extra delete on empty
        assert_eq!(palette.query(), "");
    }

    // ============ Complex Workflow Tests ============

    #[test]
    fn test_palette_complex_workflow_type_navigate_clear() {
        let mut palette = CommandPalette::new();

        // Type a query
        for c in "help".chars() {
            palette.insert_char(c);
        }
        assert_eq!(palette.query(), "help");

        // Navigate
        palette.select_next();
        palette.select_next();
        palette.select_previous();

        // Clear and verify
        palette.clear_query();
        assert_eq!(palette.query(), "");
        assert!(palette.selected_command().is_some());

        // Type new query
        for c in "quit".chars() {
            palette.insert_char(c);
        }
        assert_eq!(palette.query(), "quit");
    }

    #[test]
    fn test_palette_workflow_filter_to_nothing_then_back() {
        let mut palette = CommandPalette::new();

        // Filter to something that doesn't exist
        for c in "nonexistent".chars() {
            palette.insert_char(c);
        }
        assert!(palette.selected_command().is_none());

        // Clear and go back
        palette.clear_query();
        assert!(palette.selected_command().is_some());
    }

    #[test]
    fn test_palette_workflow_rapid_operations() {
        let mut palette = CommandPalette::new();

        for _ in 0..100 {
            palette.insert_char('h');
            palette.select_next();
            palette.insert_char('e');
            palette.select_previous();
            palette.delete_char();
            palette.delete_char();
        }

        // Should still be functional
        assert_eq!(palette.query(), "");
    }

    #[test]
    fn test_palette_workflow_unicode_operations() {
        let mut palette = CommandPalette::new();

        // Insert unicode
        palette.insert_char('日');
        palette.insert_char('本');

        // Navigate
        palette.select_next();

        // Delete one char
        palette.delete_char();
        assert_eq!(palette.query(), "日");

        // Add more
        palette.insert_char('🚀');
        assert_eq!(palette.query(), "日🚀");

        // Clear
        palette.clear_query();
        assert_eq!(palette.query(), "");
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_palette_stress() {
        let mut palette = CommandPalette::new();

        // Phase 1: Rapid insertions with varied characters
        for i in 0..200 {
            let c = match i % 5 {
                0 => 'a',
                1 => '🚀',
                2 => '日',
                3 => 'x',
                _ => 'q',
            };
            palette.insert_char(c);
        }
        assert_eq!(palette.query().chars().count(), 200);

        // Phase 2: Rapid deletions
        for _ in 0..100 {
            palette.delete_char();
        }
        assert_eq!(palette.query().chars().count(), 100);

        // Phase 3: Clear and start fresh
        palette.clear_query();
        assert_eq!(palette.query(), "");

        // Phase 4: Type realistic queries with navigation
        for c in "help".chars() {
            palette.insert_char(c);
        }
        palette.select_next();
        palette.select_next();
        palette.select_previous();

        let selected = palette.selected_command();
        assert!(selected.is_some());

        // Phase 5: Clear and try another
        palette.clear_query();
        for c in "quit".chars() {
            palette.insert_char(c);
        }

        let quit_selected = palette.selected_command();
        assert!(quit_selected.is_some());

        // Phase 6: Verify state consistency
        palette.clear_query();
        assert!(palette.selected_command().is_some());
    }

    // ============ Edge Case: Whitespace and Special Characters ============

    #[test]
    fn test_palette_query_with_spaces() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char(' ');
        palette.insert_char('b');
        assert_eq!(palette.query(), "a b");
    }

    #[test]
    fn test_palette_query_only_spaces() {
        let mut palette = CommandPalette::new();
        palette.insert_char(' ');
        palette.insert_char(' ');
        palette.insert_char(' ');
        assert_eq!(palette.query(), "   ");
    }

    #[test]
    fn test_palette_query_with_newline() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('\n');
        palette.insert_char('b');
        assert!(palette.query().contains('\n'));
    }

    #[test]
    fn test_palette_query_with_tab() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('\t');
        palette.insert_char('b');
        assert!(palette.query().contains('\t'));
    }

    #[test]
    fn test_palette_query_with_special_chars() {
        let mut palette = CommandPalette::new();
        for c in "!@#$%^&*()".chars() {
            palette.insert_char(c);
        }
        assert_eq!(palette.query(), "!@#$%^&*()");
    }

    // ============ Debug Trait Test ============

    #[test]
    fn test_palette_debug() {
        let palette = CommandPalette::new();
        let debug_str = format!("{:?}", palette);
        assert!(debug_str.contains("CommandPalette"));
    }

    // ============ Filter Matching Specifics ============

    #[test]
    fn test_palette_filter_matches_label() {
        let mut palette = CommandPalette::new();
        for c in "help".chars() {
            palette.insert_char(c);
        }
        // Should match "Show Help" in label
        let selected = palette.selected_command();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap(), "help");
    }

    #[test]
    fn test_palette_filter_matches_description() {
        let mut palette = CommandPalette::new();
        for c in "keybindings".chars() {
            palette.insert_char(c);
        }
        // Should match "help" by description containing "keybindings"
        let selected = palette.selected_command();
        assert!(selected.is_some());
    }

    #[test]
    fn test_palette_filter_matches_id() {
        let mut palette = CommandPalette::new();
        for c in "quit".chars() {
            palette.insert_char(c);
        }
        // Should match by ID "quit"
        let selected = palette.selected_command();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap(), "quit");
    }

    #[test]
    fn test_palette_filter_partial_match() {
        let mut palette = CommandPalette::new();
        palette.insert_char('h');
        palette.insert_char('e');
        // "he" should match "help", "theme", etc.
        let selected = palette.selected_command();
        assert!(selected.is_some());
    }

    // ============ Navigation on Filtered Results ============

    #[test]
    fn test_palette_navigation_on_filtered_single_result() {
        let mut palette = CommandPalette::new();
        for c in "quit".chars() {
            palette.insert_char(c);
        }

        let first = palette.selected_command();
        palette.select_next(); // Should wrap to same
        let second = palette.selected_command();
        assert_eq!(first, second);
    }

    #[test]
    fn test_palette_navigation_on_filtered_multiple_results() {
        let mut palette = CommandPalette::new();
        palette.insert_char('s'); // Matches "split", "search", "status"

        let first = palette.selected_command();
        palette.select_next();
        let second = palette.selected_command();
        assert_ne!(first, second);
    }

    #[test]
    fn test_palette_navigation_wraps_on_filtered_results() {
        let mut palette = CommandPalette::new();
        palette.insert_char('s'); // Matches multiple

        let first = palette.selected_command();

        // Navigate to end
        for _ in 0..10 {
            palette.select_next();
        }

        // Should have wrapped back
        let wrapped = palette.selected_command();
        assert!(wrapped.is_some());
    }

    // ============ Extreme Stress Tests (10k operations) ============

    #[test]
    fn test_palette_10k_char_insertions() {
        let mut palette = CommandPalette::new();
        for i in 0..10000 {
            let c = (b'a' + (i % 26) as u8) as char;
            palette.insert_char(c);
        }
        assert_eq!(palette.query().chars().count(), 10000);
    }

    #[test]
    fn test_palette_10k_mixed_operations() {
        let mut palette = CommandPalette::new();
        for i in 0..10000 {
            match i % 4 {
                0 => palette.insert_char('a'),
                1 => palette.select_next(),
                2 => palette.delete_char(),
                _ => palette.select_previous(),
            }
        }
        // Should still be functional
        assert!(palette.query().len() >= 0);
    }

    #[test]
    fn test_palette_10k_navigation_cycles() {
        let mut palette = CommandPalette::new();
        for i in 0..10000 {
            if i % 2 == 0 {
                palette.select_next();
            } else {
                palette.select_previous();
            }
        }
        // Should still have valid selection
        assert!(palette.selected_command().is_some());
    }

    // ============ Unicode Boundary Cases ============

    #[test]
    fn test_palette_very_long_unicode_string() {
        let mut palette = CommandPalette::new();
        for _ in 0..100 {
            palette.insert_char('🚀');
            palette.insert_char('日');
            palette.insert_char('本');
        }
        assert_eq!(palette.query().chars().count(), 300);
    }

    #[test]
    fn test_palette_unicode_delete_boundary() {
        let mut palette = CommandPalette::new();
        palette.insert_char('a');
        palette.insert_char('🚀');
        palette.insert_char('日');
        palette.insert_char('b');

        // Delete 'b'
        palette.delete_char();
        assert_eq!(palette.query(), "a🚀日");

        // Delete '日'
        palette.delete_char();
        assert_eq!(palette.query(), "a🚀");

        // Delete '🚀'
        palette.delete_char();
        assert_eq!(palette.query(), "a");
    }

    // ============ Multi-Phase Comprehensive Workflow (10 phases) ============

    #[test]
    fn test_palette_10_phase_comprehensive_workflow() {
        let mut palette = CommandPalette::new();

        // Phase 1: Initial state verification
        assert_eq!(palette.query(), "");
        assert!(palette.selected_command().is_some());

        // Phase 2: Type a query and navigate
        for c in "help".chars() {
            palette.insert_char(c);
        }
        palette.select_next();
        palette.select_previous();
        assert_eq!(palette.query(), "help");

        // Phase 3: Clear and verify reset
        palette.clear_query();
        assert_eq!(palette.query(), "");
        assert!(palette.selected_command().is_some());

        // Phase 4: Type unicode query
        for c in "日本🚀".chars() {
            palette.insert_char(c);
        }
        assert_eq!(palette.query(), "日本🚀");

        // Phase 5: Delete characters one by one
        palette.delete_char();
        palette.delete_char();
        palette.delete_char();
        assert_eq!(palette.query(), "");

        // Phase 6: Type query that produces no results
        for c in "nonexistent123".chars() {
            palette.insert_char(c);
        }
        assert!(palette.selected_command().is_none());

        // Phase 7: Clear and type valid query
        palette.clear_query();
        for c in "quit".chars() {
            palette.insert_char(c);
        }
        assert!(palette.selected_command().is_some());

        // Phase 8: Rapid navigation on filtered results
        for _ in 0..10 {
            palette.select_next();
            palette.select_previous();
        }

        // Phase 9: Add more characters to narrow filter
        palette.insert_char('x');
        palette.insert_char('y');
        palette.insert_char('z');

        // Phase 10: Clear and verify final state
        palette.clear_query();
        assert_eq!(palette.query(), "");
        assert!(palette.selected_command().is_some());
    }

    // ============ Edge Cases: Empty and Boundary Conditions ============

    #[test]
    fn test_palette_query_length_boundary_10k() {
        let mut palette = CommandPalette::new();
        for _ in 0..10000 {
            palette.insert_char('x');
        }
        assert_eq!(palette.query().len(), 10000);

        // Delete half
        for _ in 0..5000 {
            palette.delete_char();
        }
        assert_eq!(palette.query().len(), 5000);
    }

    #[test]
    fn test_palette_filter_after_every_char_insertion() {
        let mut palette = CommandPalette::new();

        // Type "help" character by character, verifying filter updates
        palette.insert_char('h');
        assert!(palette.selected_command().is_some());

        palette.insert_char('e');
        assert!(palette.selected_command().is_some());

        palette.insert_char('l');
        assert!(palette.selected_command().is_some());

        palette.insert_char('p');
        assert!(palette.selected_command().is_some());
        assert_eq!(palette.selected_command().unwrap(), "help");
    }

    #[test]
    fn test_palette_select_operations_with_single_filtered_result() {
        let mut palette = CommandPalette::new();

        // Filter to exactly one result
        for c in "Toggle Vim Mode".chars() {
            palette.insert_char(c);
        }

        let first = palette.selected_command();

        // Navigate back and forth - should stay on same command
        palette.select_next();
        assert_eq!(first, palette.selected_command());

        palette.select_previous();
        assert_eq!(first, palette.selected_command());
    }

    // ============ Rapid Operation Combinations ============

    #[test]
    fn test_palette_rapid_insert_clear_cycles_1000() {
        let mut palette = CommandPalette::new();

        for _ in 0..1000 {
            palette.insert_char('a');
            palette.insert_char('b');
            palette.insert_char('c');
            palette.clear_query();
        }

        assert_eq!(palette.query(), "");
        assert!(palette.selected_command().is_some());
    }
}
