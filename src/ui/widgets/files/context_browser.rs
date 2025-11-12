//! Context-aware file browser widget
//!
//! File browser integrated with LLM context management.
//! Shows files with token estimates and provides "Add to Context" actions.
//!
//! # Features
//!
//! - Directory tree navigation
//! - Token count estimation per file
//! - Visual indicator for files in context
//! - Quick add/remove from context
//! - File filtering (.gitignore support)
//! - Search functionality
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::files::ContextBrowser;
//! use std::path::PathBuf;
//!
//! # fn example() -> std::io::Result<()> {
//! let browser = ContextBrowser::new(PathBuf::from("."))?;
//! # Ok(())
//! # }
//! ```

use crate::services::FilesystemService;
use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// File entry with context information
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File path
    pub path: PathBuf,
    /// Display name
    pub name: String,
    /// Whether this is a directory
    pub is_dir: bool,
    /// Estimated token count (0 for directories)
    pub estimated_tokens: usize,
    /// Whether this file is in context
    pub in_context: bool,
    /// Indentation level
    pub depth: usize,
}

impl FileEntry {
    /// Create a new file entry
    pub fn new(path: PathBuf, name: String, is_dir: bool, depth: usize) -> Self {
        Self {
            path,
            name,
            is_dir,
            estimated_tokens: 0,
            in_context: false,
            depth,
        }
    }

    /// Estimate tokens for this file
    ///
    /// Rough estimate: ~4 characters per token on average
    pub fn estimate_tokens(&mut self) -> std::io::Result<()> {
        if self.is_dir {
            self.estimated_tokens = 0;
            return Ok(());
        }

        if let Ok(content) = std::fs::read_to_string(&self.path) {
            self.estimated_tokens = content.len() / 4;
        }

        Ok(())
    }

    /// Format token count as human-readable
    pub fn formatted_tokens(&self) -> String {
        if self.is_dir {
            return String::new();
        }

        if self.estimated_tokens >= 1000 {
            format!("{}K", self.estimated_tokens / 1000)
        } else {
            format!("{}", self.estimated_tokens)
        }
    }
}

/// Context-aware file browser
///
/// Integrates file browsing with LLM context management.
///
/// # Examples
///
/// ```no_run
/// use toad::ui::widgets::files::ContextBrowser;
/// use std::path::PathBuf;
///
/// # fn example() -> std::io::Result<()> {
/// let mut browser = ContextBrowser::new(PathBuf::from("."))?;
/// browser.select_next();
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ContextBrowser {
    /// Root directory
    root_path: PathBuf,
    /// File entries (flattened tree)
    entries: Vec<FileEntry>,
    /// Files currently in context
    context_files: HashSet<PathBuf>,
    /// Selected entry index
    selected_index: usize,
    /// List state
    list_state: ListState,
    /// Scroll state
    scroll_state: ScrollbarState,
    /// Filesystem service
    fs_service: FilesystemService,
    /// Show token estimates
    show_tokens: bool,
    /// Search query
    search_query: Option<String>,
}

impl ContextBrowser {
    /// Create a new context browser
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::files::ContextBrowser;
    /// use std::path::PathBuf;
    ///
    /// # fn example() -> std::io::Result<()> {
    /// let browser = ContextBrowser::new(PathBuf::from("."))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(root_path: PathBuf) -> std::io::Result<Self> {
        let fs_service = FilesystemService::new();
        let mut browser = Self {
            root_path: root_path.clone(),
            entries: Vec::new(),
            context_files: HashSet::new(),
            selected_index: 0,
            list_state: ListState::default(),
            scroll_state: ScrollbarState::default(),
            fs_service,
            show_tokens: true,
            search_query: None,
        };

        browser.load_directory(&root_path, 0)?;
        browser.list_state.select(Some(0));
        browser.scroll_state = ScrollbarState::new(browser.entries.len());

        Ok(browser)
    }

    /// Load directory contents
    fn load_directory(&mut self, path: &Path, depth: usize) -> std::io::Result<()> {
        let entries = self.fs_service.read_dir(path)?;

        for entry in entries {
            // Skip hidden files and common ignore patterns
            if entry.file_name.starts_with('.')
                || entry.file_name == "target"
                || entry.file_name == "node_modules"
                || entry.file_name.ends_with(".lock")
            {
                continue;
            }

            let mut file_entry = FileEntry::new(
                entry.path.clone(),
                entry.file_name,
                entry.is_dir,
                depth,
            );

            // Estimate tokens for files
            if !entry.is_dir {
                let _ = file_entry.estimate_tokens();
            }

            // Check if in context
            file_entry.in_context = self.context_files.contains(&entry.path);

            self.entries.push(file_entry);

            // Recursively load subdirectories (limited depth)
            if entry.is_dir && depth < 2 {
                self.load_directory(&entry.path, depth + 1)?;
            }
        }

        Ok(())
    }

    /// Reload the file tree
    pub fn reload(&mut self) -> std::io::Result<()> {
        self.entries.clear();
        let root_path = self.root_path.clone();
        self.load_directory(&root_path, 0)?;
        self.scroll_state = ScrollbarState::new(self.entries.len());

        // Adjust selection if needed
        if self.selected_index >= self.entries.len() && !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
            self.list_state.select(Some(self.selected_index));
        }

        Ok(())
    }

    /// Add file to context tracking
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::files::ContextBrowser;
    /// use std::path::PathBuf;
    ///
    /// # fn example() -> std::io::Result<()> {
    /// let mut browser = ContextBrowser::new(PathBuf::from("."))?;
    /// browser.add_to_context(PathBuf::from("src/main.rs"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_to_context(&mut self, path: PathBuf) {
        self.context_files.insert(path.clone());

        // Update entry
        for entry in &mut self.entries {
            if entry.path == path {
                entry.in_context = true;
            }
        }
    }

    /// Remove file from context tracking
    pub fn remove_from_context(&mut self, path: &Path) {
        self.context_files.remove(path);

        // Update entry
        for entry in &mut self.entries {
            if entry.path == path {
                entry.in_context = false;
            }
        }
    }

    /// Get selected file entry
    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    /// Get selected file entry (mutable)
    pub fn selected_entry_mut(&mut self) -> Option<&mut FileEntry> {
        self.entries.get_mut(self.selected_index)
    }

    /// Select next entry
    pub fn select_next(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.entries.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Select previous entry
    pub fn select_previous(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.entries.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Toggle showing token estimates
    pub fn toggle_tokens(&mut self) {
        self.show_tokens = !self.show_tokens;
    }

    /// Set search query
    pub fn set_search(&mut self, query: Option<String>) {
        self.search_query = query;
        // TODO: Filter entries based on query
    }

    /// Get file count
    pub fn file_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.is_dir).count()
    }

    /// Get context file count
    pub fn context_file_count(&self) -> usize {
        self.context_files.len()
    }

    /// Render the context browser
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Split into header and file list
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with stats
                Constraint::Min(0),    // File list
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_file_list(frame, chunks[1]);
    }

    /// Render header with statistics
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("File Browser").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let file_count = self.file_count();
        let context_count = self.context_file_count();

        let header = Line::from(vec![
            Span::styled("Files: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{}", file_count),
                Style::default().fg(ToadTheme::FOREGROUND),
            ),
            Span::raw("  │  "),
            Span::styled("In Context: ", Style::default().fg(ToadTheme::GRAY)),
            Span::styled(
                format!("{}", context_count),
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let paragraph = Paragraph::new(header);
        frame.render_widget(paragraph, inner);
    }

    /// Render file list
    fn render_file_list(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.entries.is_empty() {
            let empty_text = Paragraph::new(Line::from(Span::styled(
                "No files found",
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::ITALIC),
            )));
            frame.render_widget(empty_text, inner);
            return;
        }

        // Create list items
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|entry| {
                let mut spans = vec![];

                // Indentation
                if entry.depth > 0 {
                    spans.push(Span::raw("  ".repeat(entry.depth)));
                }

                // Context indicator
                if entry.in_context {
                    spans.push(Span::styled("✓ ", Style::default().fg(ToadTheme::TOAD_GREEN)));
                } else {
                    spans.push(Span::raw("  "));
                }

                // Icon
                let icon = if entry.is_dir { "📁 " } else { "📄 " };
                spans.push(Span::raw(icon));

                // Name
                let name_style = if entry.is_dir {
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(ToadTheme::FOREGROUND)
                };
                spans.push(Span::styled(&entry.name, name_style));

                // Token count
                if !entry.is_dir && self.show_tokens && entry.estimated_tokens > 0 {
                    spans.push(Span::styled(
                        format!(" ({})", entry.formatted_tokens()),
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
        if self.entries.len() > inner.height as usize {
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
    fn test_file_entry_new() {
        let entry = FileEntry::new(PathBuf::from("test.rs"), "test.rs".to_string(), false, 0);
        assert_eq!(entry.name, "test.rs");
        assert!(!entry.is_dir);
        assert!(!entry.in_context);
    }

    #[test]
    fn test_file_entry_formatted_tokens() {
        let mut entry = FileEntry::new(PathBuf::from("test.rs"), "test.rs".to_string(), false, 0);

        entry.estimated_tokens = 500;
        assert_eq!(entry.formatted_tokens(), "500");

        entry.estimated_tokens = 2500;
        assert_eq!(entry.formatted_tokens(), "2K");
    }

    #[test]
    fn test_add_remove_context() {
        let temp_dir = std::env::temp_dir();
        let mut browser = ContextBrowser::new(temp_dir.clone()).unwrap();

        let test_path = temp_dir.join("test.txt");
        browser.add_to_context(test_path.clone());
        assert_eq!(browser.context_file_count(), 1);

        browser.remove_from_context(&test_path);
        assert_eq!(browser.context_file_count(), 0);
    }

    #[test]
    fn test_select_next_previous() {
        let temp_dir = std::env::temp_dir();
        let mut browser = ContextBrowser::new(temp_dir).unwrap();

        if browser.entries.len() >= 2 {
            let initial = browser.selected_index;
            browser.select_next();
            assert_ne!(browser.selected_index, initial);

            browser.select_previous();
            assert_eq!(browser.selected_index, initial);
        }
    }

    #[test]
    fn test_toggle_tokens() {
        let temp_dir = std::env::temp_dir();
        let mut browser = ContextBrowser::new(temp_dir).unwrap();

        assert!(browser.show_tokens);
        browser.toggle_tokens();
        assert!(!browser.show_tokens);
    }
}
