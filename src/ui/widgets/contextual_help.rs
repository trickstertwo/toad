//! Contextual help system
//!
//! Provides context-aware help that changes based on the current application
//! state, active widget, and user's current task.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::{ContextualHelp, HelpContext};
//!
//! let mut help = ContextualHelp::new();
//! help.set_context(HelpContext::FileTree);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Borders, Clear, List, ListItem, Paragraph, Widget},
};

use crate::ui::atoms::{block::Block as AtomBlock, text::Text};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Help context representing different parts of the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HelpContext {
    /// General application help
    General,
    /// File tree navigation
    FileTree,
    /// Text editing
    TextEditor,
    /// Git operations
    GitOperations,
    /// Search mode
    Search,
    /// Command palette
    CommandPalette,
    /// Settings
    Settings,
    /// Evaluation mode
    Evaluation,
    /// Chat panel
    Chat,
}

/// A help entry with keybinding and description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpEntry {
    /// Keybinding (e.g., "Ctrl+P")
    pub keybinding: String,
    /// Action description
    pub description: String,
    /// Category (optional)
    pub category: Option<String>,
}

impl HelpEntry {
    /// Create a new help entry
    pub fn new(keybinding: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            keybinding: keybinding.into(),
            description: description.into(),
            category: None,
        }
    }

    /// Set category
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

/// Contextual help widget
///
/// Displays help information that adapts to the current context.
///
/// # Features
///
/// - Context-aware help content
/// - Categorized keybindings
/// - Search within help
/// - Quick reference mode
pub struct ContextualHelp {
    /// Current context
    context: HelpContext,
    /// Help entries by context
    entries: HashMap<HelpContext, Vec<HelpEntry>>,
    /// Whether help is visible
    visible: bool,
    /// Search filter
    search_filter: String,
    /// Selected entry index
    selected: usize,
}

impl ContextualHelp {
    /// Create a new contextual help system
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ContextualHelp;
    ///
    /// let help = ContextualHelp::new();
    /// ```
    pub fn new() -> Self {
        let mut help = Self {
            context: HelpContext::General,
            entries: HashMap::new(),
            visible: false,
            search_filter: String::new(),
            selected: 0,
        };

        help.load_default_entries();
        help
    }

    /// Load default help entries
    fn load_default_entries(&mut self) {
        // General help
        self.add_entry(
            HelpContext::General,
            HelpEntry::new("?", "Toggle help").category("General"),
        );
        self.add_entry(
            HelpContext::General,
            HelpEntry::new("Ctrl+P", "Command palette").category("General"),
        );
        self.add_entry(
            HelpContext::General,
            HelpEntry::new("/", "Search").category("General"),
        );
        self.add_entry(
            HelpContext::General,
            HelpEntry::new("Ctrl+C", "Quit").category("General"),
        );

        // File Tree
        self.add_entry(
            HelpContext::FileTree,
            HelpEntry::new("j/k", "Navigate down/up").category("Navigation"),
        );
        self.add_entry(
            HelpContext::FileTree,
            HelpEntry::new("Enter", "Open file/folder").category("Actions"),
        );
        self.add_entry(
            HelpContext::FileTree,
            HelpEntry::new("Space", "Select/deselect").category("Actions"),
        );
        self.add_entry(
            HelpContext::FileTree,
            HelpEntry::new("d", "Delete file").category("Actions"),
        );
        self.add_entry(
            HelpContext::FileTree,
            HelpEntry::new("r", "Rename file").category("Actions"),
        );

        // Text Editor
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("i", "Insert mode").category("Modes"),
        );
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("Esc", "Normal mode").category("Modes"),
        );
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("v", "Visual mode").category("Modes"),
        );
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("y", "Yank (copy)").category("Editing"),
        );
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("p", "Paste").category("Editing"),
        );
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("u", "Undo").category("Editing"),
        );
        self.add_entry(
            HelpContext::TextEditor,
            HelpEntry::new("Ctrl+R", "Redo").category("Editing"),
        );

        // Git Operations
        self.add_entry(
            HelpContext::GitOperations,
            HelpEntry::new("Space", "Stage/unstage file").category("Staging"),
        );
        self.add_entry(
            HelpContext::GitOperations,
            HelpEntry::new("c", "Commit").category("Actions"),
        );
        self.add_entry(
            HelpContext::GitOperations,
            HelpEntry::new("P", "Push").category("Actions"),
        );
        self.add_entry(
            HelpContext::GitOperations,
            HelpEntry::new("F", "Pull").category("Actions"),
        );
        self.add_entry(
            HelpContext::GitOperations,
            HelpEntry::new("b", "Branches").category("Navigation"),
        );

        // Search
        self.add_entry(
            HelpContext::Search,
            HelpEntry::new("n", "Next result").category("Navigation"),
        );
        self.add_entry(
            HelpContext::Search,
            HelpEntry::new("N", "Previous result").category("Navigation"),
        );
        self.add_entry(
            HelpContext::Search,
            HelpEntry::new("Esc", "Exit search").category("Actions"),
        );

        // Command Palette
        self.add_entry(
            HelpContext::CommandPalette,
            HelpEntry::new("↑/↓", "Navigate commands").category("Navigation"),
        );
        self.add_entry(
            HelpContext::CommandPalette,
            HelpEntry::new("Enter", "Execute command").category("Actions"),
        );
        self.add_entry(
            HelpContext::CommandPalette,
            HelpEntry::new("Esc", "Close palette").category("Actions"),
        );

        // Chat
        self.add_entry(
            HelpContext::Chat,
            HelpEntry::new("Enter", "Send message").category("Actions"),
        );
        self.add_entry(
            HelpContext::Chat,
            HelpEntry::new("Shift+Enter", "New line").category("Editing"),
        );
        self.add_entry(
            HelpContext::Chat,
            HelpEntry::new("Ctrl+L", "Clear chat").category("Actions"),
        );
    }

    /// Add a help entry for a context
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::{ContextualHelp, HelpContext, HelpEntry};
    ///
    /// let mut help = ContextualHelp::new();
    /// help.add_entry(
    ///     HelpContext::General,
    ///     HelpEntry::new("F1", "Help"),
    /// );
    /// ```
    pub fn add_entry(&mut self, context: HelpContext, entry: HelpEntry) {
        self.entries.entry(context).or_default().push(entry);
    }

    /// Set the current context
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::{ContextualHelp, HelpContext};
    ///
    /// let mut help = ContextualHelp::new();
    /// help.set_context(HelpContext::FileTree);
    /// assert_eq!(help.context(), HelpContext::FileTree);
    /// ```
    pub fn set_context(&mut self, context: HelpContext) {
        self.context = context;
        self.selected = 0;
    }

    /// Get the current context
    pub fn context(&self) -> HelpContext {
        self.context
    }

    /// Show help
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide help
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle help visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if help is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set search filter
    pub fn set_filter(&mut self, filter: impl Into<String>) {
        self.search_filter = filter.into();
        self.selected = 0;
    }

    /// Clear search filter
    pub fn clear_filter(&mut self) {
        self.search_filter.clear();
        self.selected = 0;
    }

    /// Get filtered entries for current context
    fn get_filtered_entries(&self) -> Vec<&HelpEntry> {
        let entries = self.entries.get(&self.context).map(|v| v.as_slice()).unwrap_or(&[]);

        if self.search_filter.is_empty() {
            entries.iter().collect()
        } else {
            let filter_lower = self.search_filter.to_lowercase();
            entries
                .iter()
                .filter(|e| {
                    e.keybinding.to_lowercase().contains(&filter_lower)
                        || e.description.to_lowercase().contains(&filter_lower)
                })
                .collect()
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        let count = self.get_filtered_entries().len();
        if self.selected + 1 < count {
            self.selected += 1;
        }
    }

    /// Get entry count for current context
    pub fn entry_count(&self) -> usize {
        self.get_filtered_entries().len()
    }
}

impl Default for ContextualHelp {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &ContextualHelp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        // Calculate centered help panel
        let help_width = (area.width as f32 * 0.6).min(70.0) as u16;
        let help_height = (area.height as f32 * 0.7).min(30.0) as u16;

        let help_area = Rect {
            x: (area.width.saturating_sub(help_width)) / 2 + area.x,
            y: (area.height.saturating_sub(help_height)) / 2 + area.y,
            width: help_width,
            height: help_height,
        };

        // Clear background
        Clear.render(help_area, buf);

        // Split into title, content, footer
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Title
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(help_area);

        // Render title
        let context_name = format!("{:?}", self.context);

        // Use Text atoms for title components
        let help_text = Text::new("Help: ").style(Style::default().fg(Color::Gray));
        let context_text = Text::new(&context_name)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        let space_text = Text::new(" ");
        let bindings_text = Text::new(format!("({} bindings)", self.entry_count()))
            .style(Style::default().fg(Color::Gray));

        let title_text = vec![Line::from(vec![
            help_text.to_span(),
            context_text.to_span(),
            space_text.to_span(),
            bindings_text.to_span(),
        ])];

        // Use Block atom for title border
        let title_block = AtomBlock::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .to_ratatui();

        let title = Paragraph::new(title_text)
            .block(title_block)
            .alignment(Alignment::Center);
        title.render(chunks[0], buf);

        // Render entries
        let entries = self.get_filtered_entries();
        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let category_str = entry.category.as_deref().unwrap_or("General");

                // Use Text atoms for each component
                let key_text = Text::new(format!("{:15}", entry.keybinding))
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                let separator_text = Text::new(" │ ");
                let desc_text = Text::new(&entry.description)
                    .style(Style::default().fg(Color::White));
                let space_text = Text::new(" ");
                let category_text = Text::new(format!("[{}]", category_str))
                    .style(Style::default().fg(Color::DarkGray));

                ListItem::new(Line::from(vec![
                    key_text.to_span(),
                    separator_text.to_span(),
                    desc_text.to_span(),
                    space_text.to_span(),
                    category_text.to_span(),
                ]))
            })
            .collect();

        // Use Block atom for list border
        let list_block = AtomBlock::new()
            .borders(Borders::ALL)
            .title("Keybindings")
            .to_ratatui();

        let list = List::new(items).block(list_block);

        list.render(chunks[1], buf);

        // Render footer
        let footer_text = if self.search_filter.is_empty() {
            "↑/↓: Navigate | /: Filter | ?: Close | Tab: Change context"
        } else {
            &format!("Filtering: '{}' | Esc: Clear filter", self.search_filter)
        };

        // Use Block atom for footer border
        let footer_block = AtomBlock::new()
            .borders(Borders::ALL)
            .to_ratatui();

        let footer = Paragraph::new(footer_text)
            .block(footer_block)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        footer.render(chunks[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contextual_help_new() {
        let help = ContextualHelp::new();
        assert_eq!(help.context(), HelpContext::General);
        assert!(!help.is_visible());
    }

    #[test]
    fn test_contextual_help_visibility() {
        let mut help = ContextualHelp::new();

        help.show();
        assert!(help.is_visible());

        help.hide();
        assert!(!help.is_visible());

        help.toggle();
        assert!(help.is_visible());

        help.toggle();
        assert!(!help.is_visible());
    }

    #[test]
    fn test_contextual_help_context() {
        let mut help = ContextualHelp::new();

        help.set_context(HelpContext::FileTree);
        assert_eq!(help.context(), HelpContext::FileTree);

        help.set_context(HelpContext::GitOperations);
        assert_eq!(help.context(), HelpContext::GitOperations);
    }

    #[test]
    fn test_contextual_help_entries() {
        let mut help = ContextualHelp::new();

        help.set_context(HelpContext::General);
        let general_count = help.entry_count();
        assert!(general_count > 0);

        help.set_context(HelpContext::FileTree);
        let tree_count = help.entry_count();
        assert!(tree_count > 0);
    }

    #[test]
    fn test_contextual_help_filter() {
        let mut help = ContextualHelp::new();
        help.set_context(HelpContext::General);

        let initial_count = help.entry_count();

        help.set_filter("palette");
        let filtered_count = help.entry_count();

        assert!(filtered_count < initial_count);

        help.clear_filter();
        assert_eq!(help.entry_count(), initial_count);
    }

    #[test]
    fn test_contextual_help_navigation() {
        let mut help = ContextualHelp::new();
        help.set_context(HelpContext::General);

        assert_eq!(help.selected, 0);

        help.move_down();
        assert_eq!(help.selected, 1);

        help.move_up();
        assert_eq!(help.selected, 0);

        // Can't go below 0
        help.move_up();
        assert_eq!(help.selected, 0);
    }

    #[test]
    fn test_help_entry() {
        let entry = HelpEntry::new("Ctrl+P", "Command palette").category("General");

        assert_eq!(entry.keybinding, "Ctrl+P");
        assert_eq!(entry.description, "Command palette");
        assert_eq!(entry.category.as_deref(), Some("General"));
    }

    #[test]
    fn test_add_custom_entry() {
        let mut help = ContextualHelp::new();
        let initial_count = help.entry_count();

        help.add_entry(
            HelpContext::General,
            HelpEntry::new("F1", "Custom help"),
        );

        assert_eq!(help.entry_count(), initial_count + 1);
    }
}
