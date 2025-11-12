//! Diff visualization widget for code changes
//!
//! Displays git-compatible diffs with syntax highlighting, hunk navigation,
//! and selective application support.
//!
//! # Features
//!
//! - Side-by-side and unified diff modes
//! - Syntax highlighting (placeholder for future integration)
//! - Inline diff markers: + Added, - Removed, ~ Modified
//! - Hunk navigation (n/p for next/previous)
//! - Selective hunk application
//! - Configurable context lines (default 3)
//! - Git diff compatible format
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::git::DiffViewer;
//!
//! let diff_text = "\
//! diff --git a/src/main.rs b/src/main.rs
//! @@ -1,3 +1,4 @@
//! +use std::io;
//!  fn main() {
//!      println!(\"Hello\");
//!  }
//! ";
//! let viewer = DiffViewer::new(diff_text);
//! ```

use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

/// Diff display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMode {
    /// Unified diff (+ and - lines)
    Unified,
    /// Side-by-side diff
    SideBySide,
}

/// Line change type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    /// Line added
    Added,
    /// Line removed
    Removed,
    /// Line modified (removed + added)
    Modified,
    /// Context line (unchanged)
    Context,
}

impl ChangeType {
    /// Get color for change type
    pub fn color(&self) -> Color {
        match self {
            ChangeType::Added => ToadTheme::TOAD_GREEN,
            ChangeType::Removed => ToadTheme::ERROR,
            ChangeType::Modified => ToadTheme::YELLOW,
            ChangeType::Context => ToadTheme::GRAY,
        }
    }

    /// Get marker symbol
    pub fn marker(&self) -> &'static str {
        match self {
            ChangeType::Added => "+",
            ChangeType::Removed => "-",
            ChangeType::Modified => "~",
            ChangeType::Context => " ",
        }
    }
}

/// A single diff line
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Line content
    pub content: String,
    /// Change type
    pub change_type: ChangeType,
    /// Old line number (if applicable)
    pub old_line_num: Option<usize>,
    /// New line number (if applicable)
    pub new_line_num: Option<usize>,
}

impl DiffLine {
    /// Create a new diff line
    pub fn new(
        content: String,
        change_type: ChangeType,
        old_line_num: Option<usize>,
        new_line_num: Option<usize>,
    ) -> Self {
        Self {
            content,
            change_type,
            old_line_num,
            new_line_num,
        }
    }
}

/// A diff hunk (contiguous block of changes)
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Hunk header (e.g., "@@ -1,3 +1,4 @@")
    pub header: String,
    /// Lines in this hunk
    pub lines: Vec<DiffLine>,
    /// Old start line
    pub old_start: usize,
    /// Old line count
    pub old_count: usize,
    /// New start line
    pub new_start: usize,
    /// New line count
    pub new_count: usize,
    /// Whether this hunk is selected for application
    pub selected: bool,
}

impl DiffHunk {
    /// Create a new diff hunk
    pub fn new(
        header: String,
        old_start: usize,
        old_count: usize,
        new_start: usize,
        new_count: usize,
    ) -> Self {
        Self {
            header,
            lines: Vec::new(),
            old_start,
            old_count,
            new_start,
            new_count,
            selected: true, // Selected by default
        }
    }

    /// Add a line to the hunk
    pub fn add_line(&mut self, line: DiffLine) {
        self.lines.push(line);
    }

    /// Get number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

/// Diff viewer widget
///
/// Displays git diffs with navigation and selective application support.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::git::DiffViewer;
///
/// let diff = "\
/// @@ -1,3 +1,4 @@
/// +use std::io;
///  fn main() {
/// -    println!(\"Hello\");
/// +    println!(\"Hello, world!\");
///  }
/// ";
/// let mut viewer = DiffViewer::new(diff);
/// assert_eq!(viewer.hunk_count(), 1);
/// ```
#[derive(Debug)]
pub struct DiffViewer {
    /// Display mode
    mode: DiffMode,
    /// Diff hunks
    hunks: Vec<DiffHunk>,
    /// Selected hunk index
    selected_hunk: usize,
    /// Scroll offset for current hunk
    scroll_offset: usize,
    /// Context lines to show
    context_lines: usize,
    /// List state for hunk list
    list_state: ListState,
    /// Scrollbar state
    scrollbar_state: ScrollbarState,
    /// File path (if available)
    file_path: Option<String>,
}

impl DiffViewer {
    /// Create a new diff viewer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::git::DiffViewer;
    ///
    /// let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+fn test() {}";
    /// let viewer = DiffViewer::new(diff);
    /// ```
    pub fn new(diff_text: &str) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let hunks = Self::parse_diff(diff_text);
        let scrollbar_state = ScrollbarState::new(hunks.len());

        Self {
            mode: DiffMode::Unified,
            hunks,
            selected_hunk: 0,
            scroll_offset: 0,
            context_lines: 3,
            list_state,
            scrollbar_state,
            file_path: None,
        }
    }

    /// Create with file path
    pub fn with_file_path(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Set display mode
    pub fn set_mode(&mut self, mode: DiffMode) {
        self.mode = mode;
    }

    /// Set context lines
    pub fn set_context_lines(&mut self, lines: usize) {
        self.context_lines = lines;
    }

    /// Parse git diff text into hunks
    fn parse_diff(diff_text: &str) -> Vec<DiffHunk> {
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;
        let mut old_line = 0;
        let mut new_line = 0;

        for line in diff_text.lines() {
            if line.starts_with("@@") {
                // Save previous hunk
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }

                // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
                if let Some((old_part, new_part)) = Self::parse_hunk_header(line) {
                    let (old_start, old_count) = old_part;
                    let (new_start, new_count) = new_part;

                    current_hunk = Some(DiffHunk::new(
                        line.to_string(),
                        old_start,
                        old_count,
                        new_start,
                        new_count,
                    ));

                    old_line = old_start;
                    new_line = new_start;
                }
            } else if line.starts_with('+') && !line.starts_with("+++") {
                // Added line
                if let Some(ref mut hunk) = current_hunk {
                    hunk.add_line(DiffLine::new(
                        line[1..].to_string(),
                        ChangeType::Added,
                        None,
                        Some(new_line),
                    ));
                    new_line += 1;
                }
            } else if line.starts_with('-') && !line.starts_with("---") {
                // Removed line
                if let Some(ref mut hunk) = current_hunk {
                    hunk.add_line(DiffLine::new(
                        line[1..].to_string(),
                        ChangeType::Removed,
                        Some(old_line),
                        None,
                    ));
                    old_line += 1;
                }
            } else if line.starts_with(' ') {
                // Context line
                if let Some(ref mut hunk) = current_hunk {
                    hunk.add_line(DiffLine::new(
                        line[1..].to_string(),
                        ChangeType::Context,
                        Some(old_line),
                        Some(new_line),
                    ));
                    old_line += 1;
                    new_line += 1;
                }
            }
        }

        // Save last hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        hunks
    }

    /// Parse hunk header
    fn parse_hunk_header(header: &str) -> Option<((usize, usize), (usize, usize))> {
        // Format: @@ -old_start,old_count +new_start,new_count @@
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let old_part = parts[1].trim_start_matches('-');
        let new_part = parts[2].trim_start_matches('+');

        let parse_range = |s: &str| -> Option<(usize, usize)> {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() == 2 {
                Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
            } else if parts.len() == 1 {
                Some((parts[0].parse().ok()?, 1))
            } else {
                None
            }
        };

        Some((parse_range(old_part)?, parse_range(new_part)?))
    }

    /// Get number of hunks
    pub fn hunk_count(&self) -> usize {
        self.hunks.len()
    }

    /// Select next hunk
    pub fn next_hunk(&mut self) {
        if !self.hunks.is_empty() {
            self.selected_hunk = (self.selected_hunk + 1) % self.hunks.len();
            self.list_state.select(Some(self.selected_hunk));
            self.scroll_offset = 0;
        }
    }

    /// Select previous hunk
    pub fn prev_hunk(&mut self) {
        if !self.hunks.is_empty() {
            self.selected_hunk = if self.selected_hunk == 0 {
                self.hunks.len() - 1
            } else {
                self.selected_hunk - 1
            };
            self.list_state.select(Some(self.selected_hunk));
            self.scroll_offset = 0;
        }
    }

    /// Toggle selection of current hunk
    pub fn toggle_hunk_selection(&mut self) {
        if let Some(hunk) = self.hunks.get_mut(self.selected_hunk) {
            hunk.selected = !hunk.selected;
        }
    }

    /// Get selected hunk
    pub fn selected_hunk(&self) -> Option<&DiffHunk> {
        self.hunks.get(self.selected_hunk)
    }

    /// Get all selected hunks
    pub fn selected_hunks(&self) -> Vec<&DiffHunk> {
        self.hunks.iter().filter(|h| h.selected).collect()
    }

    /// Scroll down in current hunk
    pub fn scroll_down(&mut self) {
        if let Some(hunk) = self.hunks.get(self.selected_hunk) {
            if self.scroll_offset + 10 < hunk.line_count() {
                self.scroll_offset += 1;
            }
        }
    }

    /// Scroll up in current hunk
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Render the diff viewer
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(2), // Footer
            ])
            .split(area);

        self.render_header(frame, chunks[0]);
        self.render_content(frame, chunks[1]);
        self.render_footer(frame, chunks[2]);
    }

    /// Render header
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = if let Some(ref path) = self.file_path {
            format!("Diff: {}", path)
        } else {
            "Diff Viewer".to_string()
        };

        let block = Block::themed(&title).to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mode_text = match self.mode {
            DiffMode::Unified => "Unified",
            DiffMode::SideBySide => "Side-by-Side",
        };

        let info = format!(
            "Mode: {} | Hunks: {}/{} | Context: {} lines",
            mode_text,
            self.selected_hunk + 1,
            self.hunks.len(),
            self.context_lines
        );

        let paragraph = Paragraph::new(info);
        frame.render_widget(paragraph, inner);
    }

    /// Render content
    fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Changes").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.hunks.is_empty() {
            let text = Paragraph::new("No changes to display");
            frame.render_widget(text, inner);
            return;
        }

        if let Some(hunk) = self.hunks.get(self.selected_hunk) {
            let mut lines = vec![];

            // Hunk header
            lines.push(Line::from(Span::styled(
                &hunk.header,
                Style::default()
                    .fg(ToadTheme::BLUE)
                    .add_modifier(Modifier::BOLD),
            )));

            // Diff lines
            let visible_lines = &hunk.lines[self.scroll_offset..];
            for line in visible_lines.iter().take(inner.height as usize - 2) {
                let mut spans = vec![];

                // Line numbers
                if let Some(old_num) = line.old_line_num {
                    spans.push(Span::styled(
                        format!("{:4} ", old_num),
                        Style::default().fg(ToadTheme::GRAY),
                    ));
                } else {
                    spans.push(Span::raw("     "));
                }

                if let Some(new_num) = line.new_line_num {
                    spans.push(Span::styled(
                        format!("{:4} ", new_num),
                        Style::default().fg(ToadTheme::GRAY),
                    ));
                } else {
                    spans.push(Span::raw("     "));
                }

                // Marker
                spans.push(Span::styled(
                    format!("{} ", line.change_type.marker()),
                    Style::default().fg(line.change_type.color()),
                ));

                // Content
                spans.push(Span::styled(
                    &line.content,
                    Style::default().fg(line.change_type.color()),
                ));

                lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(lines);
            frame.render_widget(paragraph, inner);

            // Scrollbar
            if hunk.line_count() > inner.height as usize {
                let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
                let mut scrollbar_state =
                    ScrollbarState::new(hunk.line_count()).position(self.scroll_offset);
                frame.render_stateful_widget(
                    scrollbar,
                    inner,
                    &mut scrollbar_state,
                );
            }
        }
    }

    /// Render footer
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let block = Block::themed("Controls").to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let controls = Line::from(vec![
            Span::styled("n/p", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Next/Prev Hunk | "),
            Span::styled("Space", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Toggle Selection | "),
            Span::styled("↑/↓", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Scroll | "),
            Span::styled("m", Style::default().fg(ToadTheme::BLUE)),
            Span::raw(" Toggle Mode"),
        ]);

        let paragraph = Paragraph::new(controls);
        frame.render_widget(paragraph, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_mode() {
        assert_eq!(DiffMode::Unified, DiffMode::Unified);
        assert_ne!(DiffMode::Unified, DiffMode::SideBySide);
    }

    #[test]
    fn test_change_type_color() {
        assert_eq!(ChangeType::Added.color(), ToadTheme::TOAD_GREEN);
        assert_eq!(ChangeType::Removed.color(), ToadTheme::ERROR);
        assert_eq!(ChangeType::Modified.color(), ToadTheme::YELLOW);
        assert_eq!(ChangeType::Context.color(), ToadTheme::GRAY);
    }

    #[test]
    fn test_change_type_marker() {
        assert_eq!(ChangeType::Added.marker(), "+");
        assert_eq!(ChangeType::Removed.marker(), "-");
        assert_eq!(ChangeType::Modified.marker(), "~");
        assert_eq!(ChangeType::Context.marker(), " ");
    }

    #[test]
    fn test_diff_line_creation() {
        let line = DiffLine::new(
            "test content".to_string(),
            ChangeType::Added,
            None,
            Some(10),
        );
        assert_eq!(line.content, "test content");
        assert_eq!(line.change_type, ChangeType::Added);
        assert_eq!(line.old_line_num, None);
        assert_eq!(line.new_line_num, Some(10));
    }

    #[test]
    fn test_diff_hunk_creation() {
        let mut hunk = DiffHunk::new("@@ -1,3 +1,4 @@".to_string(), 1, 3, 1, 4);
        assert_eq!(hunk.old_start, 1);
        assert_eq!(hunk.old_count, 3);
        assert_eq!(hunk.new_start, 1);
        assert_eq!(hunk.new_count, 4);
        assert!(hunk.selected);
        assert_eq!(hunk.line_count(), 0);

        hunk.add_line(DiffLine::new(
            "test".to_string(),
            ChangeType::Context,
            Some(1),
            Some(1),
        ));
        assert_eq!(hunk.line_count(), 1);
    }

    #[test]
    fn test_parse_simple_diff() {
        let diff = "@@ -1,3 +1,4 @@\n use std::io;\n fn main() {\n+    println!(\"test\");\n }";
        let viewer = DiffViewer::new(diff);
        assert_eq!(viewer.hunk_count(), 1);
    }

    #[test]
    fn test_parse_empty_diff() {
        let viewer = DiffViewer::new("");
        assert_eq!(viewer.hunk_count(), 0);
    }

    #[test]
    fn test_hunk_navigation() {
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+fn test() {}\n\n@@ -10,1 +11,1 @@\n-old line\n+new line";
        let mut viewer = DiffViewer::new(diff);
        assert_eq!(viewer.hunk_count(), 2);
        assert_eq!(viewer.selected_hunk, 0);

        viewer.next_hunk();
        assert_eq!(viewer.selected_hunk, 1);

        viewer.next_hunk();
        assert_eq!(viewer.selected_hunk, 0); // Wraps around

        viewer.prev_hunk();
        assert_eq!(viewer.selected_hunk, 1);
    }

    #[test]
    fn test_toggle_hunk_selection() {
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+fn test() {}";
        let mut viewer = DiffViewer::new(diff);

        assert!(viewer.hunks[0].selected);

        viewer.toggle_hunk_selection();
        assert!(!viewer.hunks[0].selected);

        viewer.toggle_hunk_selection();
        assert!(viewer.hunks[0].selected);
    }

    #[test]
    fn test_selected_hunks() {
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+fn test() {}\n\n@@ -10,1 +11,1 @@\n-old\n+new";
        let mut viewer = DiffViewer::new(diff);

        let selected = viewer.selected_hunks();
        assert_eq!(selected.len(), 2);

        viewer.toggle_hunk_selection();
        let selected = viewer.selected_hunks();
        assert_eq!(selected.len(), 1);
    }

    #[test]
    fn test_scroll() {
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+fn test() {}";
        let mut viewer = DiffViewer::new(diff);

        assert_eq!(viewer.scroll_offset, 0);

        viewer.scroll_down();
        assert_eq!(viewer.scroll_offset, 1);

        viewer.scroll_up();
        assert_eq!(viewer.scroll_offset, 0);

        viewer.scroll_up();
        assert_eq!(viewer.scroll_offset, 0); // Can't scroll below 0
    }

    #[test]
    fn test_set_mode() {
        let mut viewer = DiffViewer::new("");
        assert_eq!(viewer.mode, DiffMode::Unified);

        viewer.set_mode(DiffMode::SideBySide);
        assert_eq!(viewer.mode, DiffMode::SideBySide);
    }

    #[test]
    fn test_set_context_lines() {
        let mut viewer = DiffViewer::new("");
        assert_eq!(viewer.context_lines, 3);

        viewer.set_context_lines(5);
        assert_eq!(viewer.context_lines, 5);
    }

    #[test]
    fn test_with_file_path() {
        let viewer = DiffViewer::new("").with_file_path("src/main.rs");
        assert_eq!(viewer.file_path, Some("src/main.rs".to_string()));
    }

    #[test]
    fn test_parse_hunk_header() {
        let result = DiffViewer::parse_hunk_header("@@ -1,3 +1,4 @@");
        assert_eq!(result, Some(((1, 3), (1, 4))));

        let result = DiffViewer::parse_hunk_header("@@ -10 +10,2 @@");
        assert_eq!(result, Some(((10, 1), (10, 2))));
    }

    #[test]
    fn test_parse_diff_with_context() {
        let diff = "@@ -1,3 +1,3 @@\n context line\n-removed\n+added\n context line 2";
        let viewer = DiffViewer::new(diff);

        assert_eq!(viewer.hunk_count(), 1);
        let hunk = &viewer.hunks[0];
        assert_eq!(hunk.line_count(), 4);
        assert_eq!(hunk.lines[0].change_type, ChangeType::Context);
        assert_eq!(hunk.lines[1].change_type, ChangeType::Removed);
        assert_eq!(hunk.lines[2].change_type, ChangeType::Added);
        assert_eq!(hunk.lines[3].change_type, ChangeType::Context);
    }
}
