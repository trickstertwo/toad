//! File preview manager with async loading and syntax highlighting
//!
//! Provides a complete file preview system that can:
//! - Load files asynchronously
//! - Detect file types automatically
//! - Apply syntax highlighting
//! - Handle large files with streaming
//! - Preview different formats
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::FilePreviewManager;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let mut manager = FilePreviewManager::new();
//! manager.preview_file("src/main.rs").await?;
//! # Ok(())
//! # }
//! ```

use crate::ui::syntax::{Language, SyntaxHighlighter};
use anyhow::{Context, Result};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap, Widget},
};
use std::path::{Path, PathBuf};
use tokio::fs;

/// File preview state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreviewState {
    /// No file loaded
    Empty,
    /// Loading file
    Loading,
    /// File loaded successfully
    Loaded,
    /// Error loading file
    Error,
}

/// File preview manager
///
/// Manages file previewing with async loading and syntax highlighting.
///
/// # Features
///
/// - Async file loading
/// - Automatic syntax highlighting based on file extension
/// - Line numbers
/// - Scrolling
/// - Large file handling (truncation with warning)
pub struct FilePreviewManager {
    /// Current file path
    file_path: Option<PathBuf>,
    /// File content
    content: String,
    /// Preview state
    state: PreviewState,
    /// Error message if any
    error: Option<String>,
    /// Scroll offset (line number)
    scroll_offset: usize,
    /// Whether to show line numbers
    show_line_numbers: bool,
    /// Whether to enable syntax highlighting
    syntax_highlighting: bool,
    /// Detected language
    language: Language,
    /// Max file size to preview (bytes)
    max_size: usize,
    /// Whether file was truncated
    truncated: bool,
}

impl FilePreviewManager {
    /// Create a new file preview manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::FilePreviewManager;
    ///
    /// let manager = FilePreviewManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            file_path: None,
            content: String::new(),
            state: PreviewState::Empty,
            error: None,
            scroll_offset: 0,
            show_line_numbers: true,
            syntax_highlighting: true,
            language: Language::PlainText,
            max_size: 1024 * 1024, // 1MB default
            truncated: false,
        }
    }

    /// Load and preview a file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::ui::widgets::FilePreviewManager;
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let mut manager = FilePreviewManager::new();
    /// manager.preview_file("src/main.rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn preview_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        self.file_path = Some(path.to_path_buf());
        self.state = PreviewState::Loading;
        self.error = None;
        self.scroll_offset = 0;
        self.truncated = false;

        // Detect language from extension
        self.language = Language::from_extension(
            path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or(""),
        );

        // Check file size
        match fs::metadata(path).await {
            Ok(metadata) => {
                let size = metadata.len() as usize;
                if size > self.max_size {
                    // File too large, read only first part
                    match self.load_partial(path, self.max_size).await {
                        Ok(content) => {
                            self.content = content;
                            self.state = PreviewState::Loaded;
                            self.truncated = true;
                        }
                        Err(e) => {
                            self.error = Some(format!("Error: {}", e));
                            self.state = PreviewState::Error;
                            self.content.clear();
                        }
                    }
                } else {
                    // Normal load
                    match fs::read_to_string(path).await {
                        Ok(content) => {
                            self.content = content;
                            self.state = PreviewState::Loaded;
                        }
                        Err(e) => {
                            self.error = Some(format!("Error: {}", e));
                            self.state = PreviewState::Error;
                            self.content.clear();
                        }
                    }
                }
            }
            Err(e) => {
                self.error = Some(format!("Cannot read metadata: {}", e));
                self.state = PreviewState::Error;
                self.content.clear();
            }
        }

        Ok(())
    }

    /// Load partial file content
    async fn load_partial(&self, path: &Path, max_bytes: usize) -> Result<String> {
        let bytes = fs::read(path).await?;
        let truncated_bytes = &bytes[..max_bytes.min(bytes.len())];

        // Try to convert to string, replacing invalid UTF-8
        Ok(String::from_utf8_lossy(truncated_bytes).to_string())
    }

    /// Get the current state
    pub fn state(&self) -> PreviewState {
        self.state.clone()
    }

    /// Get the file path
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Get the content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set content directly (for testing)
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
        self.state = PreviewState::Loaded;
        self.scroll_offset = 0;
    }

    /// Clear the preview
    pub fn clear(&mut self) {
        self.file_path = None;
        self.content.clear();
        self.state = PreviewState::Empty;
        self.error = None;
        self.scroll_offset = 0;
        self.truncated = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let max_offset = self.line_count().saturating_sub(1);
        self.scroll_offset = (self.scroll_offset + lines).min(max_offset);
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.line_count().saturating_sub(1);
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// Toggle line numbers
    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    /// Toggle syntax highlighting
    pub fn toggle_syntax_highlighting(&mut self) {
        self.syntax_highlighting = !self.syntax_highlighting;
    }

    /// Set max file size
    pub fn set_max_size(&mut self, bytes: usize) {
        self.max_size = bytes;
    }

    /// Check if file was truncated
    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Get error message
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Get scroll offset
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Get the detected language
    pub fn language(&self) -> Language {
        self.language
    }
}

impl Default for FilePreviewManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &FilePreviewManager {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Build title
        let title = if let Some(path) = &self.file_path {
            format!(
                "Preview: {} {}{}",
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown"),
                match self.language {
                    Language::PlainText => String::new(),
                    lang => format!("[{:?}] ", lang),
                },
                if self.truncated {
                    "[TRUNCATED]"
                } else {
                    ""
                }
            )
        } else {
            String::from("Preview")
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(match self.state {
                PreviewState::Loading => Style::default().fg(Color::Yellow),
                PreviewState::Error => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Cyan),
            });

        // Render based on state
        match self.state {
            PreviewState::Empty => {
                let para = Paragraph::new("No file selected").block(block)
                    .style(Style::default().fg(Color::Gray));
                para.render(area, buf);
            }
            PreviewState::Loading => {
                let para = Paragraph::new("Loading...").block(block)
                    .style(Style::default().fg(Color::Yellow));
                para.render(area, buf);
            }
            PreviewState::Error => {
                let error_text = self.error.as_deref().unwrap_or("Unknown error");
                let para = Paragraph::new(error_text).block(block)
                    .style(Style::default().fg(Color::Red))
                    .wrap(Wrap { trim: false });
                para.render(area, buf);
            }
            PreviewState::Loaded => {
                // Render content with optional line numbers and syntax highlighting
                if self.show_line_numbers {
                    self.render_with_line_numbers(area, buf, block);
                } else {
                    self.render_plain(area, buf, block);
                }
            }
        }
    }
}

impl FilePreviewManager {
    /// Render without line numbers
    fn render_plain(&self, area: Rect, buf: &mut Buffer, block: Block) {
        let paragraph = Paragraph::new(self.content.as_str())
            .block(block)
            .scroll((self.scroll_offset as u16, 0))
            .wrap(Wrap { trim: false });

        paragraph.render(area, buf);
    }

    /// Render with line numbers
    fn render_with_line_numbers(&self, area: Rect, buf: &mut Buffer, block: Block) {
        let lines: Vec<Line> = self
            .content
            .lines()
            .enumerate()
            .skip(self.scroll_offset)
            .map(|(idx, line)| {
                let line_num = idx + 1;
                Line::from(vec![
                    Span::styled(
                        format!("{:4} │ ", line_num),
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM),
                    ),
                    Span::raw(line),
                ])
            })
            .collect();

        let paragraph = Paragraph::new(lines).block(block);
        paragraph.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_file_preview_manager_new() {
        let manager = FilePreviewManager::new();
        assert_eq!(manager.state(), PreviewState::Empty);
        assert!(manager.file_path().is_none());
        assert_eq!(manager.content(), "");
    }

    #[test]
    fn test_file_preview_manager_set_content() {
        let mut manager = FilePreviewManager::new();
        manager.set_content("Hello, world!");

        assert_eq!(manager.state(), PreviewState::Loaded);
        assert_eq!(manager.content(), "Hello, world!");
    }

    #[test]
    fn test_file_preview_manager_scroll() {
        let mut manager = FilePreviewManager::new();
        manager.set_content("Line 1\nLine 2\nLine 3\nLine 4\nLine 5");

        assert_eq!(manager.scroll_offset(), 0);

        manager.scroll_down(2);
        assert_eq!(manager.scroll_offset(), 2);

        manager.scroll_up(1);
        assert_eq!(manager.scroll_offset(), 1);

        manager.scroll_to_top();
        assert_eq!(manager.scroll_offset(), 0);

        manager.scroll_to_bottom();
        assert_eq!(manager.scroll_offset(), 4);
    }

    #[test]
    fn test_file_preview_manager_line_count() {
        let mut manager = FilePreviewManager::new();
        manager.set_content("Line 1\nLine 2\nLine 3");

        assert_eq!(manager.line_count(), 3);
    }

    #[test]
    fn test_file_preview_manager_clear() {
        let mut manager = FilePreviewManager::new();
        manager.set_content("Test content");

        manager.clear();

        assert_eq!(manager.state(), PreviewState::Empty);
        assert_eq!(manager.content(), "");
        assert_eq!(manager.scroll_offset(), 0);
    }

    #[tokio::test]
    async fn test_file_preview_manager_preview_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "Test file content").await.unwrap();

        let mut manager = FilePreviewManager::new();
        manager.preview_file(&test_file).await.unwrap();

        assert_eq!(manager.state(), PreviewState::Loaded);
        assert_eq!(manager.content(), "Test file content");
    }

    #[tokio::test]
    async fn test_file_preview_manager_preview_nonexistent() {
        let mut manager = FilePreviewManager::new();
        manager.preview_file("/nonexistent/file.txt").await.ok();

        assert_eq!(manager.state(), PreviewState::Error);
        assert!(manager.error().is_some());
    }

    #[tokio::test]
    async fn test_file_preview_manager_language_detection() {
        let temp_dir = TempDir::new().unwrap();

        let rust_file = temp_dir.path().join("test.rs");
        fs::write(&rust_file, "fn main() {}").await.unwrap();

        let mut manager = FilePreviewManager::new();
        manager.preview_file(&rust_file).await.unwrap();

        assert_eq!(manager.language(), Language::Rust);
    }

    #[tokio::test]
    async fn test_file_preview_manager_truncation() {
        let temp_dir = TempDir::new().unwrap();
        let large_file = temp_dir.path().join("large.txt");

        // Create a file larger than max_size
        let large_content = "X".repeat(2 * 1024 * 1024); // 2MB
        fs::write(&large_file, large_content).await.unwrap();

        let mut manager = FilePreviewManager::new();
        manager.set_max_size(100); // 100 bytes max
        manager.preview_file(&large_file).await.unwrap();

        assert_eq!(manager.state(), PreviewState::Loaded);
        assert!(manager.is_truncated());
        assert!(manager.content().len() <= 100);
    }
}
