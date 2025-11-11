//! Context display widget showing what AI sees
//!
//! Displays the current context being sent to the AI, including files,
//! conversation history, and system prompts. Useful for debugging and
//! understanding AI behavior.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::ContextDisplay;
//!
//! let mut display = ContextDisplay::new();
//! display.add_file_context("src/main.rs", "fn main() {}");
//! display.add_message("user", "Fix the bug");
//! ```

use crate::ui::atoms::{block::Block as AtomBlock, text::Text as AtomText};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Tabs, Widget},
};

/// Type of context item
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextType {
    /// File content
    File,
    /// Conversation message
    Message,
    /// System prompt
    SystemPrompt,
    /// Code snippet
    CodeSnippet,
    /// Tool output
    ToolOutput,
}

impl ContextType {
    /// Get color for this context type
    pub fn color(&self) -> Color {
        match self {
            ContextType::File => Color::Cyan,
            ContextType::Message => Color::Green,
            ContextType::SystemPrompt => Color::Yellow,
            ContextType::CodeSnippet => Color::Magenta,
            ContextType::ToolOutput => Color::Blue,
        }
    }

    /// Get icon for this context type
    pub fn icon(&self) -> &'static str {
        match self {
            ContextType::File => "📄",
            ContextType::Message => "💬",
            ContextType::SystemPrompt => "⚙️",
            ContextType::CodeSnippet => "📝",
            ContextType::ToolOutput => "🔧",
        }
    }

    /// Get label for this context type
    pub fn label(&self) -> &'static str {
        match self {
            ContextType::File => "FILE",
            ContextType::Message => "MESSAGE",
            ContextType::SystemPrompt => "SYSTEM",
            ContextType::CodeSnippet => "SNIPPET",
            ContextType::ToolOutput => "TOOL",
        }
    }
}

/// A context item
#[derive(Debug, Clone)]
pub struct ContextItem {
    /// Type of context
    pub context_type: ContextType,
    /// Name or identifier
    pub name: String,
    /// Content
    pub content: String,
    /// Token count (if known)
    pub token_count: Option<usize>,
    /// Metadata (e.g., role for messages)
    pub metadata: Option<String>,
}

impl ContextItem {
    /// Create a new context item
    pub fn new(
        context_type: ContextType,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            context_type,
            name: name.into(),
            content: content.into(),
            token_count: None,
            metadata: None,
        }
    }

    /// Add token count
    pub fn with_tokens(mut self, count: usize) -> Self {
        self.token_count = Some(count);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// Get preview of content (first 100 chars)
    pub fn preview(&self) -> String {
        let preview_len = 100;
        if self.content.len() > preview_len {
            format!("{}...", &self.content[..preview_len])
        } else {
            self.content.clone()
        }
    }

    /// Get content length
    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}

/// Tab in context display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextTab {
    /// All context items
    All,
    /// Only files
    Files,
    /// Only messages
    Messages,
    /// Only system prompts
    System,
}

impl ContextTab {
    const ALL_TABS: [ContextTab; 4] = [
        ContextTab::All,
        ContextTab::Files,
        ContextTab::Messages,
        ContextTab::System,
    ];

    fn title(&self) -> &'static str {
        match self {
            ContextTab::All => "All",
            ContextTab::Files => "Files",
            ContextTab::Messages => "Messages",
            ContextTab::System => "System",
        }
    }

    fn matches(&self, item: &ContextItem) -> bool {
        match self {
            ContextTab::All => true,
            ContextTab::Files => item.context_type == ContextType::File,
            ContextTab::Messages => item.context_type == ContextType::Message,
            ContextTab::System => item.context_type == ContextType::SystemPrompt,
        }
    }
}

/// Context display widget
pub struct ContextDisplay {
    /// All context items
    items: Vec<ContextItem>,
    /// List state for navigation
    list_state: ListState,
    /// Current tab
    current_tab: ContextTab,
    /// Show content preview
    show_preview: bool,
}

impl ContextDisplay {
    /// Create a new context display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ContextDisplay;
    ///
    /// let display = ContextDisplay::new();
    /// assert_eq!(display.item_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            list_state: ListState::default(),
            current_tab: ContextTab::All,
            show_preview: true,
        }
    }

    /// Add a file context item
    pub fn add_file_context(&mut self, path: impl Into<String>, content: impl Into<String>) {
        let item = ContextItem::new(ContextType::File, path, content);
        self.items.push(item);

        if self.items.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Add a message context item
    pub fn add_message(&mut self, role: impl Into<String>, content: impl Into<String>) {
        let role_str: String = role.into();
        let item = ContextItem::new(ContextType::Message, role_str.clone(), content)
            .with_metadata(role_str);
        self.items.push(item);

        if self.items.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Add a system prompt
    pub fn add_system_prompt(&mut self, name: impl Into<String>, content: impl Into<String>) {
        let item = ContextItem::new(ContextType::SystemPrompt, name, content);
        self.items.push(item);

        if self.items.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Add a code snippet
    pub fn add_code_snippet(&mut self, name: impl Into<String>, content: impl Into<String>) {
        let item = ContextItem::new(ContextType::CodeSnippet, name, content);
        self.items.push(item);

        if self.items.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Add tool output
    pub fn add_tool_output(&mut self, tool_name: impl Into<String>, output: impl Into<String>) {
        let item = ContextItem::new(ContextType::ToolOutput, tool_name, output);
        self.items.push(item);

        if self.items.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Switch to next tab
    pub fn next_tab(&mut self) {
        let current_idx = ContextTab::ALL_TABS
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % ContextTab::ALL_TABS.len();
        self.current_tab = ContextTab::ALL_TABS[next_idx];
        self.list_state.select(Some(0));
    }

    /// Switch to previous tab
    pub fn previous_tab(&mut self) {
        let current_idx = ContextTab::ALL_TABS
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);
        let prev_idx = if current_idx == 0 {
            ContextTab::ALL_TABS.len() - 1
        } else {
            current_idx - 1
        };
        self.current_tab = ContextTab::ALL_TABS[prev_idx];
        self.list_state.select(Some(0));
    }

    /// Toggle preview
    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
    }

    /// Navigate to next item
    pub fn next(&mut self) {
        let filtered = self.filtered_items();
        if filtered.is_empty() {
            return;
        }

        let current = self.list_state.selected().unwrap_or(0);
        let next = if current >= filtered.len() - 1 {
            0
        } else {
            current + 1
        };
        self.list_state.select(Some(next));
    }

    /// Navigate to previous item
    pub fn previous(&mut self) {
        let filtered = self.filtered_items();
        if filtered.is_empty() {
            return;
        }

        let current = self.list_state.selected().unwrap_or(0);
        let prev = if current == 0 {
            filtered.len() - 1
        } else {
            current - 1
        };
        self.list_state.select(Some(prev));
    }

    /// Get filtered items based on current tab
    fn filtered_items(&self) -> Vec<&ContextItem> {
        self.items
            .iter()
            .filter(|item| self.current_tab.matches(item))
            .collect()
    }

    /// Get current item
    pub fn current_item(&self) -> Option<&ContextItem> {
        let filtered = self.filtered_items();
        self.list_state
            .selected()
            .and_then(|idx| filtered.get(idx).copied())
    }

    /// Get total item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get total token count
    pub fn total_tokens(&self) -> usize {
        self.items.iter().filter_map(|item| item.token_count).sum()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.list_state.select(None);
    }

    /// Get items by type
    pub fn items_by_type(&self, context_type: ContextType) -> Vec<&ContextItem> {
        self.items
            .iter()
            .filter(|item| item.context_type == context_type)
            .collect()
    }
}

impl Default for ContextDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &ContextDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into tabs, list, and preview
        let chunks = Layout::vertical([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // List or split list/preview
        ])
        .split(area);

        // Render tabs
        let tab_titles: Vec<&str> = ContextTab::ALL_TABS.iter().map(|t| t.title()).collect();
        let tab_index = ContextTab::ALL_TABS
            .iter()
            .position(|&t| t == self.current_tab)
            .unwrap_or(0);

        let tabs = Tabs::new(tab_titles)
            .block(
                AtomBlock::new()
                    .borders(Borders::ALL)
                    .title("AI Context View")
                    .border_style(Style::default().fg(Color::Cyan))
                    .to_ratatui(),
            )
            .select(tab_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        tabs.render(chunks[0], buf);

        // Determine if we should split for preview
        let (list_area, preview_area) = if self.show_preview
            && self
                .current_item()
                .map(|i| !i.content.is_empty())
                .unwrap_or(false)
        {
            let preview_chunks =
                Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                    .split(chunks[1]);
            (preview_chunks[0], Some(preview_chunks[1]))
        } else {
            (chunks[1], None)
        };

        // Render list
        let filtered = self.filtered_items();
        let items: Vec<ListItem> = filtered
            .iter()
            .map(|item| {
                let icon = item.context_type.icon();
                let color = item.context_type.color();
                let label = item.context_type.label();

                let mut spans = vec![
                    AtomText::new(format!("{} ", icon))
                        .style(Style::default())
                        .to_span(),
                    AtomText::new(format!("[{}] ", label))
                        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                        .to_span(),
                    AtomText::new(&item.name)
                        .style(Style::default().fg(Color::White))
                        .to_span(),
                ];

                if let Some(tokens) = item.token_count {
                    spans.push(
                        AtomText::new(format!(" ({} tokens)", tokens))
                            .style(Style::default().fg(Color::DarkGray))
                            .to_span(),
                    );
                }

                if let Some(metadata) = &item.metadata {
                    spans.push(
                        AtomText::new(format!(" [{}]", metadata))
                            .style(Style::default().fg(Color::Gray))
                            .to_span(),
                    );
                }

                ListItem::new(vec![Line::from(spans)])
            })
            .collect();

        let list_title = format!(
            "{} ({} items, {} tokens)",
            self.current_tab.title(),
            filtered.len(),
            self.total_tokens()
        );

        let list = List::new(items)
            .block(
                AtomBlock::new()
                    .borders(Borders::ALL)
                    .title(list_title)
                    .border_style(Style::default().fg(Color::Yellow))
                    .to_ratatui(),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        let mut list_state = self.list_state.clone();
        StatefulWidget::render(list, list_area, buf, &mut list_state);

        // Render preview if enabled
        if let Some(preview_rect) = preview_area
            && let Some(item) = self.current_item()
        {
            let preview = Paragraph::new(item.content.as_str())
                .block(
                    AtomBlock::new()
                        .borders(Borders::ALL)
                        .title(format!("Preview: {}", item.name))
                        .border_style(Style::default().fg(Color::Cyan))
                        .to_ratatui(),
                )
                .style(Style::default().fg(Color::White))
                .wrap(ratatui::widgets::Wrap { trim: false });

            preview.render(preview_rect, buf);
        }

        // Render footer
        if area.height > 5 {
            let footer_area = Rect {
                x: area.x,
                y: area.y + area.height - 1,
                width: area.width,
                height: 1,
            };

            let footer_text = "Tab: Switch view | ↑↓: Navigate | p: Toggle preview | c: Clear";
            let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::DarkGray));

            footer.render(footer_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_display_new() {
        let display = ContextDisplay::new();
        assert_eq!(display.item_count(), 0);
        assert_eq!(display.total_tokens(), 0);
    }

    #[test]
    fn test_add_file_context() {
        let mut display = ContextDisplay::new();
        display.add_file_context("src/main.rs", "fn main() {}");

        assert_eq!(display.item_count(), 1);
        assert_eq!(display.items_by_type(ContextType::File).len(), 1);
    }

    #[test]
    fn test_add_message() {
        let mut display = ContextDisplay::new();
        display.add_message("user", "Hello");

        assert_eq!(display.item_count(), 1);
        assert_eq!(display.items_by_type(ContextType::Message).len(), 1);
    }

    #[test]
    fn test_add_system_prompt() {
        let mut display = ContextDisplay::new();
        display.add_system_prompt("base", "You are a helpful assistant");

        assert_eq!(display.item_count(), 1);
        assert_eq!(display.items_by_type(ContextType::SystemPrompt).len(), 1);
    }

    #[test]
    fn test_tabs() {
        let mut display = ContextDisplay::new();
        display.add_file_context("file.rs", "content");
        display.add_message("user", "message");

        assert_eq!(display.current_tab, ContextTab::All);

        display.next_tab();
        assert_eq!(display.current_tab, ContextTab::Files);

        display.next_tab();
        assert_eq!(display.current_tab, ContextTab::Messages);

        display.previous_tab();
        assert_eq!(display.current_tab, ContextTab::Files);
    }

    #[test]
    fn test_navigation() {
        let mut display = ContextDisplay::new();
        display.add_file_context("file1.rs", "content1");
        display.add_file_context("file2.rs", "content2");
        display.add_file_context("file3.rs", "content3");

        assert_eq!(display.list_state.selected(), Some(0));

        display.next();
        assert_eq!(display.list_state.selected(), Some(1));

        display.next();
        assert_eq!(display.list_state.selected(), Some(2));

        display.next(); // Wrap
        assert_eq!(display.list_state.selected(), Some(0));

        display.previous();
        assert_eq!(display.list_state.selected(), Some(2));
    }

    #[test]
    fn test_filtered_items() {
        let mut display = ContextDisplay::new();
        display.add_file_context("file.rs", "code");
        display.add_message("user", "msg");
        display.add_system_prompt("sys", "prompt");

        assert_eq!(display.filtered_items().len(), 3);

        display.current_tab = ContextTab::Files;
        assert_eq!(display.filtered_items().len(), 1);

        display.current_tab = ContextTab::Messages;
        assert_eq!(display.filtered_items().len(), 1);
    }

    #[test]
    fn test_context_item() {
        let item = ContextItem::new(ContextType::File, "test.rs", "fn test() {}")
            .with_tokens(50)
            .with_metadata("readonly");

        assert_eq!(item.token_count, Some(50));
        assert_eq!(item.metadata.as_deref(), Some("readonly"));
        assert_eq!(item.content_length(), 12);
    }

    #[test]
    fn test_clear() {
        let mut display = ContextDisplay::new();
        display.add_file_context("file.rs", "content");
        display.add_message("user", "msg");

        display.clear();
        assert_eq!(display.item_count(), 0);
        assert_eq!(display.list_state.selected(), None);
    }

    #[test]
    fn test_total_tokens() {
        let mut display = ContextDisplay::new();
        let item1 = ContextItem::new(ContextType::File, "file1", "content").with_tokens(100);
        let item2 = ContextItem::new(ContextType::File, "file2", "content").with_tokens(200);

        display.items.push(item1);
        display.items.push(item2);

        assert_eq!(display.total_tokens(), 300);
    }
}
