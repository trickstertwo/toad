//! Context panel widget for managing LLM context
//!
//! Displays current context usage, files in context, and provides management actions.
//!
//! # Features
//!
//! - Token usage display with visual progress bar
//! - Files in context list with token counts
//! - Add/remove context actions
//! - Context optimization suggestions
//! - Cost estimation
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::context::ContextPanel;
//!
//! let panel = ContextPanel::new();
//! ```

use crate::ui::atoms::Block;
use crate::ui::molecules::ContextWindow;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use std::path::PathBuf;

/// Context file entry
#[derive(Debug, Clone)]
pub struct ContextFile {
    /// File path
    pub path: PathBuf,
    /// Estimated token count
    pub tokens: usize,
    /// Whether this file is pinned (always included)
    pub pinned: bool,
}

impl ContextFile {
    /// Create a new context file
    pub fn new(path: PathBuf, tokens: usize) -> Self {
        Self {
            path,
            tokens,
            pinned: false,
        }
    }

    /// Pin this file
    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }
}

/// Context panel widget
///
/// Manages and displays LLM context including token usage and file list.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::context::ContextPanel;
///
/// let panel = ContextPanel::new();
/// assert_eq!(panel.total_tokens(), 0);
/// ```
#[derive(Debug)]
pub struct ContextPanel {
    /// Files currently in context
    files: Vec<ContextFile>,
    /// Total tokens used
    used_tokens: u64,
    /// Maximum context window
    max_tokens: u64,
    /// Selected file index
    selected_index: usize,
    /// List state for rendering
    list_state: ListState,
    /// Scroll state for file list
    scroll_state: ScrollbarState,
    /// Show token counts for each file
    show_file_tokens: bool,
    /// Show cost estimate
    show_cost: bool,
}

impl Default for ContextPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextPanel {
    /// Create a new context panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::context::ContextPanel;
    ///
    /// let panel = ContextPanel::new();
    /// ```
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            files: Vec::new(),
            used_tokens: 0,
            max_tokens: 200_000, // Default Claude context window
            selected_index: 0,
            list_state,
            scroll_state: ScrollbarState::default(),
            show_file_tokens: true,
            show_cost: true,
        }
    }

    /// Add a file to context
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::context::ContextPanel;
    /// use std::path::PathBuf;
    ///
    /// let mut panel = ContextPanel::new();
    /// panel.add_file(PathBuf::from("src/main.rs"), 1500);
    /// assert_eq!(panel.file_count(), 1);
    /// ```
    pub fn add_file(&mut self, path: PathBuf, tokens: usize) {
        self.files.push(ContextFile::new(path, tokens));
        self.used_tokens += tokens as u64;
        self.scroll_state = ScrollbarState::new(self.files.len());
    }

    /// Remove a file from context
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::context::ContextPanel;
    /// use std::path::PathBuf;
    ///
    /// let mut panel = ContextPanel::new();
    /// panel.add_file(PathBuf::from("test.rs"), 1000);
    /// panel.remove_file(0);
    /// assert_eq!(panel.file_count(), 0);
    /// ```
    pub fn remove_file(&mut self, index: usize) -> Option<ContextFile> {
        if index < self.files.len() {
            let file = self.files.remove(index);
            self.used_tokens = self.used_tokens.saturating_sub(file.tokens as u64);
            self.scroll_state = ScrollbarState::new(self.files.len());

            // Adjust selection if needed
            if self.selected_index >= self.files.len() && self.selected_index > 0 {
                self.selected_index = self.files.len().saturating_sub(1);
                self.list_state.select(Some(self.selected_index));
            }

            Some(file)
        } else {
            None
        }
    }

    /// Clear all files from context
    pub fn clear(&mut self) {
        self.files.clear();
        self.used_tokens = 0;
        self.selected_index = 0;
        self.list_state.select(Some(0));
        self.scroll_state = ScrollbarState::default();
    }

    /// Get number of files in context
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Get total tokens used
    pub fn total_tokens(&self) -> u64 {
        self.used_tokens
    }

    /// Get maximum tokens
    pub fn max_tokens(&self) -> u64 {
        self.max_tokens
    }

    /// Set maximum tokens (context window size)
    pub fn set_max_tokens(&mut self, tokens: u64) {
        self.max_tokens = tokens;
    }

    /// Get usage percentage (0.0 - 100.0)
    pub fn usage_percentage(&self) -> f64 {
        if self.max_tokens == 0 {
            0.0
        } else {
            (self.used_tokens as f64 / self.max_tokens as f64) * 100.0
        }
    }

    /// Select next file
    pub fn select_next(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.files.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Select previous file
    pub fn select_previous(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.files.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Get selected file
    pub fn selected_file(&self) -> Option<&ContextFile> {
        self.files.get(self.selected_index)
    }

    /// Toggle showing file tokens
    pub fn toggle_file_tokens(&mut self) {
        self.show_file_tokens = !self.show_file_tokens;
    }

    /// Toggle showing cost estimate
    pub fn toggle_cost(&mut self) {
        self.show_cost = !self.show_cost;
    }

    /// Estimate cost in USD (based on typical Claude pricing)
    ///
    /// Uses approximate pricing: $3 per 1M input tokens
    pub fn estimate_cost(&self) -> f64 {
        (self.used_tokens as f64 / 1_000_000.0) * 3.0
    }

    /// Render the context panel
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Split into header and content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Header with usage
                Constraint::Min(0),    // File list
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_file_list(frame, chunks[1]);
    }

    /// Render header with context usage
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Context Usage").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::new();

        // Create context window molecule
        let context_window = ContextWindow::new(self.used_tokens, self.max_tokens);
        lines.push(context_window.to_line());

        lines.push(Line::from(""));

        // Show cost estimate if enabled
        if self.show_cost {
            let cost = self.estimate_cost();
            let cost_text = if cost < 0.01 {
                "<$0.01".to_string()
            } else {
                format!("${:.2}", cost)
            };

            lines.push(Line::from(vec![
                Span::styled("Est. Cost: ", Style::default().fg(ToadTheme::GRAY)),
                Span::styled(
                    cost_text,
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }

    /// Render file list
    fn render_file_list(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed(&format!("Files in Context ({})", self.files.len())).to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.files.is_empty() {
            let empty_text = Paragraph::new(Line::from(Span::styled(
                "No files in context. Press 'a' to add files.",
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::ITALIC),
            )));
            frame.render_widget(empty_text, inner);
            return;
        }

        // Create list items
        let items: Vec<ListItem> = self
            .files
            .iter()
            .map(|file| {
                let mut spans = vec![];

                // Pin indicator
                if file.pinned {
                    spans.push(Span::styled("📌 ", Style::default().fg(ToadTheme::TOAD_GREEN)));
                } else {
                    spans.push(Span::raw("   "));
                }

                // File name
                let file_name = file
                    .path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                spans.push(Span::styled(
                    file_name,
                    Style::default().fg(ToadTheme::FOREGROUND),
                ));

                // Token count (if enabled)
                if self.show_file_tokens {
                    let token_text = if file.tokens >= 1000 {
                        format!(" ({}K)", file.tokens / 1000)
                    } else {
                        format!(" ({})", file.tokens)
                    };

                    spans.push(Span::styled(
                        token_text,
                        Style::default().fg(ToadTheme::GRAY),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::DARK_GRAY)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, inner, &mut self.list_state);

        // Render scrollbar if needed
        if self.files.len() > inner.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            self.scroll_state = self.scroll_state
                .position(self.selected_index)
                .viewport_content_length(inner.height as usize);

            frame.render_stateful_widget(scrollbar, inner, &mut self.scroll_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_panel_new() {
        let panel = ContextPanel::new();
        assert_eq!(panel.file_count(), 0);
        assert_eq!(panel.total_tokens(), 0);
    }

    #[test]
    fn test_add_file() {
        let mut panel = ContextPanel::new();
        panel.add_file(PathBuf::from("test.rs"), 1500);

        assert_eq!(panel.file_count(), 1);
        assert_eq!(panel.total_tokens(), 1500);
    }

    #[test]
    fn test_remove_file() {
        let mut panel = ContextPanel::new();
        panel.add_file(PathBuf::from("test1.rs"), 1000);
        panel.add_file(PathBuf::from("test2.rs"), 2000);

        assert_eq!(panel.file_count(), 2);
        assert_eq!(panel.total_tokens(), 3000);

        let removed = panel.remove_file(0);
        assert!(removed.is_some());
        assert_eq!(panel.file_count(), 1);
        assert_eq!(panel.total_tokens(), 2000);
    }

    #[test]
    fn test_clear() {
        let mut panel = ContextPanel::new();
        panel.add_file(PathBuf::from("test1.rs"), 1000);
        panel.add_file(PathBuf::from("test2.rs"), 2000);

        panel.clear();
        assert_eq!(panel.file_count(), 0);
        assert_eq!(panel.total_tokens(), 0);
    }

    #[test]
    fn test_usage_percentage() {
        let mut panel = ContextPanel::new();
        panel.set_max_tokens(10000);
        panel.add_file(PathBuf::from("test.rs"), 5000);

        let percentage = panel.usage_percentage();
        assert!((percentage - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_select_next_previous() {
        let mut panel = ContextPanel::new();
        panel.add_file(PathBuf::from("test1.rs"), 1000);
        panel.add_file(PathBuf::from("test2.rs"), 2000);
        panel.add_file(PathBuf::from("test3.rs"), 3000);

        assert_eq!(panel.selected_index, 0);

        panel.select_next();
        assert_eq!(panel.selected_index, 1);

        panel.select_next();
        assert_eq!(panel.selected_index, 2);

        panel.select_next(); // Wraps around
        assert_eq!(panel.selected_index, 0);

        panel.select_previous();
        assert_eq!(panel.selected_index, 2);
    }

    #[test]
    fn test_estimate_cost() {
        let mut panel = ContextPanel::new();
        panel.add_file(PathBuf::from("test.rs"), 1_000_000);

        let cost = panel.estimate_cost();
        assert!((cost - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_toggle_settings() {
        let mut panel = ContextPanel::new();
        assert!(panel.show_file_tokens);
        assert!(panel.show_cost);

        panel.toggle_file_tokens();
        assert!(!panel.show_file_tokens);

        panel.toggle_cost();
        assert!(!panel.show_cost);
    }
}
