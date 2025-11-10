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
    pub(super) query: String,
    /// Cursor position in search
    pub(super) cursor_position: usize,
    /// All available commands
    pub(super) commands: Vec<PaletteCommand>,
    /// Filtered commands based on query
    pub(super) filtered: Vec<usize>,
    /// List state for selection
    pub(super) list_state: ListState,
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
