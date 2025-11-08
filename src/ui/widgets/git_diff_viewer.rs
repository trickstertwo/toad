//! Git diff viewer widget for displaying file changes
//!
//! Provides a rich diff viewing experience with syntax highlighting,
//! line-by-line additions/deletions, and file headers.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::GitDiffViewer;
//!
//! let diff = r#"
//! diff --git a/src/main.rs b/src/main.rs
//! index 1234567..89abcdef 100644
//! --- a/src/main.rs
//! +++ b/src/main.rs
//! @@ -1,4 +1,5 @@
//!  fn main() {
//! +    println!("Hello, world!");
//!      println!("Goodbye!");
//!  }
//! "#;
//!
//! let mut viewer = GitDiffViewer::new();
//! viewer.set_diff(diff);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget},
};

/// Type of diff line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineType {
    /// File header (diff --git)
    FileHeader,
    /// Index line (index abc123..def456)
    Index,
    /// Old file path (---)
    OldFile,
    /// New file path (+++)
    NewFile,
    /// Hunk header (@@ -1,4 +1,5 @@)
    Hunk,
    /// Added line (+)
    Addition,
    /// Deleted line (-)
    Deletion,
    /// Context line (unchanged)
    Context,
    /// No newline at end of file
    NoNewline,
}

impl DiffLineType {
    /// Get the color for this line type
    pub fn color(&self) -> Color {
        match self {
            DiffLineType::FileHeader => Color::Yellow,
            DiffLineType::Index => Color::DarkGray,
            DiffLineType::OldFile => Color::Red,
            DiffLineType::NewFile => Color::Green,
            DiffLineType::Hunk => Color::Cyan,
            DiffLineType::Addition => Color::Green,
            DiffLineType::Deletion => Color::Red,
            DiffLineType::Context => Color::White,
            DiffLineType::NoNewline => Color::DarkGray,
        }
    }

    /// Get the background color for this line type (if any)
    pub fn bg_color(&self) -> Option<Color> {
        match self {
            DiffLineType::Addition => Some(Color::Rgb(0, 64, 0)), // Dark green
            DiffLineType::Deletion => Some(Color::Rgb(64, 0, 0)), // Dark red
            _ => None,
        }
    }
}

/// A line in a diff
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Original line text
    pub text: String,
    /// Line type
    pub line_type: DiffLineType,
    /// Old line number (if applicable)
    pub old_line_no: Option<usize>,
    /// New line number (if applicable)
    pub new_line_no: Option<usize>,
}

impl DiffLine {
    /// Create a new diff line
    pub fn new(
        text: impl Into<String>,
        line_type: DiffLineType,
        old_line_no: Option<usize>,
        new_line_no: Option<usize>,
    ) -> Self {
        Self {
            text: text.into(),
            line_type,
            old_line_no,
            new_line_no,
        }
    }

    /// Parse line type from diff line
    fn parse_type(line: &str) -> DiffLineType {
        if line.starts_with("diff --git") {
            DiffLineType::FileHeader
        } else if line.starts_with("index ") {
            DiffLineType::Index
        } else if line.starts_with("--- ") {
            DiffLineType::OldFile
        } else if line.starts_with("+++ ") {
            DiffLineType::NewFile
        } else if line.starts_with("@@ ") {
            DiffLineType::Hunk
        } else if line.starts_with('+') {
            DiffLineType::Addition
        } else if line.starts_with('-') {
            DiffLineType::Deletion
        } else if line.starts_with("\\ No newline") {
            DiffLineType::NoNewline
        } else {
            DiffLineType::Context
        }
    }
}

/// Git diff viewer widget
///
/// Displays unified diff format with syntax highlighting and line numbers.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::GitDiffViewer;
///
/// let mut viewer = GitDiffViewer::new();
/// viewer.set_diff("diff --git a/file.txt b/file.txt\n+added line");
/// viewer.with_line_numbers(true);
/// viewer.with_syntax_highlighting(true);
/// ```
#[derive(Debug, Clone)]
pub struct GitDiffViewer {
    /// Parsed diff lines
    lines: Vec<DiffLine>,
    /// Title of the viewer
    title: String,
    /// Show line numbers
    show_line_numbers: bool,
    /// Enable syntax highlighting
    syntax_highlighting: bool,
    /// Compact mode (less padding)
    compact: bool,
    /// Current file being viewed (if filtering by file)
    current_file: Option<String>,
}

impl Default for GitDiffViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl GitDiffViewer {
    /// Create a new git diff viewer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::GitDiffViewer;
    ///
    /// let viewer = GitDiffViewer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            title: "Git Diff".to_string(),
            show_line_numbers: true,
            syntax_highlighting: true,
            compact: false,
            current_file: None,
        }
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set whether to show line numbers
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Set whether to enable syntax highlighting
    pub fn with_syntax_highlighting(mut self, enabled: bool) -> Self {
        self.syntax_highlighting = enabled;
        self
    }

    /// Enable compact mode
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Set the diff content
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::GitDiffViewer;
    ///
    /// let mut viewer = GitDiffViewer::new();
    /// viewer.set_diff("diff --git a/file.txt b/file.txt\n+new line");
    /// ```
    pub fn set_diff(&mut self, diff: impl AsRef<str>) {
        self.lines = self.parse_diff(diff.as_ref());
    }

    /// Set the diff and filter to a specific file
    pub fn set_diff_for_file(&mut self, diff: impl AsRef<str>, file: impl Into<String>) {
        self.current_file = Some(file.into());
        self.lines = self.parse_diff(diff.as_ref());
        self.filter_by_current_file();
    }

    /// Clear the diff
    pub fn clear(&mut self) {
        self.lines.clear();
        self.current_file = None;
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get statistics (additions, deletions, context)
    pub fn stats(&self) -> (usize, usize, usize) {
        let additions = self
            .lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Addition)
            .count();
        let deletions = self
            .lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Deletion)
            .count();
        let context = self
            .lines
            .iter()
            .filter(|l| l.line_type == DiffLineType::Context)
            .count();
        (additions, deletions, context)
    }

    /// Parse diff text into DiffLines
    fn parse_diff(&self, diff: &str) -> Vec<DiffLine> {
        let mut lines = Vec::new();
        let mut old_line = 0;
        let mut new_line = 0;

        for line in diff.lines() {
            let line_type = DiffLine::parse_type(line);

            let (old_no, new_no) = match line_type {
                DiffLineType::Hunk => {
                    // Parse hunk header to get line numbers
                    // @@ -1,4 +1,5 @@
                    if let Some(nums) = Self::parse_hunk_header(line) {
                        old_line = nums.0;
                        new_line = nums.1;
                    }
                    (None, None)
                }
                DiffLineType::Addition => {
                    let no = new_line;
                    new_line += 1;
                    (None, Some(no))
                }
                DiffLineType::Deletion => {
                    let no = old_line;
                    old_line += 1;
                    (Some(no), None)
                }
                DiffLineType::Context => {
                    let old_no = old_line;
                    let new_no = new_line;
                    old_line += 1;
                    new_line += 1;
                    (Some(old_no), Some(new_no))
                }
                _ => (None, None),
            };

            lines.push(DiffLine::new(line, line_type, old_no, new_no));
        }

        lines
    }

    /// Parse hunk header to extract starting line numbers
    /// Returns (old_start, new_start)
    fn parse_hunk_header(line: &str) -> Option<(usize, usize)> {
        // @@ -1,4 +1,5 @@
        if !line.starts_with("@@ -") {
            return None;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        let old_part = parts[1].trim_start_matches('-');
        let new_part = parts[2].trim_start_matches('+');

        let old_start = old_part.split(',').next()?.parse().ok()?;
        let new_start = new_part.split(',').next()?.parse().ok()?;

        Some((old_start, new_start))
    }

    /// Filter lines to only show the current file
    fn filter_by_current_file(&mut self) {
        if let Some(ref file) = self.current_file {
            let mut in_file = false;
            let mut filtered_lines = Vec::new();

            for line in &self.lines {
                match line.line_type {
                    DiffLineType::FileHeader => {
                        in_file = line.text.contains(file);
                        if in_file {
                            filtered_lines.push(line.clone());
                        }
                    }
                    _ if in_file => {
                        filtered_lines.push(line.clone());
                    }
                    _ => {}
                }
            }

            self.lines = filtered_lines;
        }
    }

    /// Render lines as ListItems
    fn render_lines(&self) -> Vec<ListItem<'static>> {
        let mut items = Vec::new();

        for line in &self.lines {
            let mut spans = Vec::new();

            // Add line numbers if enabled
            if self.show_line_numbers {
                let line_no_str = match (line.old_line_no, line.new_line_no) {
                    (Some(old), Some(new)) => format!("{:4} {:4} ", old, new),
                    (Some(old), None) => format!("{:4}    - ", old),
                    (None, Some(new)) => format!("   - {:4} ", new),
                    (None, None) => "         ".to_string(),
                };

                spans.push(Span::styled(
                    line_no_str,
                    Style::default().fg(Color::DarkGray),
                ));
            }

            // Add the line text with appropriate styling
            let mut style = Style::default().fg(line.line_type.color());

            if self.syntax_highlighting {
                if let Some(bg) = line.line_type.bg_color() {
                    style = style.bg(bg);
                }

                // Bold for headers
                if matches!(
                    line.line_type,
                    DiffLineType::FileHeader | DiffLineType::Hunk
                ) {
                    style = style.add_modifier(Modifier::BOLD);
                }
            }

            spans.push(Span::styled(line.text.clone(), style));

            items.push(ListItem::new(Line::from(spans)));
        }

        if items.is_empty() {
            items.push(ListItem::new(Line::from(Span::styled(
                "No diff to display",
                Style::default().fg(Color::DarkGray),
            ))));
        }

        items
    }
}

/// Stateful widget implementation
impl StatefulWidget for &GitDiffViewer {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items = self.render_lines();

        let (additions, deletions, _context) = self.stats();
        let title = if additions > 0 || deletions > 0 {
            format!(
                "{} (+{} -{}) ",
                self.title, additions, deletions
            )
        } else {
            self.title.clone()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::Cyan));

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        StatefulWidget::render(list, area, buf, state);
    }
}

/// Regular widget implementation (without selection)
impl Widget for &GitDiffViewer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_line_type_parse() {
        assert_eq!(DiffLine::parse_type("diff --git a/file b/file"), DiffLineType::FileHeader);
        assert_eq!(DiffLine::parse_type("index 1234567..89abcdef"), DiffLineType::Index);
        assert_eq!(DiffLine::parse_type("--- a/file.txt"), DiffLineType::OldFile);
        assert_eq!(DiffLine::parse_type("+++ b/file.txt"), DiffLineType::NewFile);
        assert_eq!(DiffLine::parse_type("@@ -1,4 +1,5 @@"), DiffLineType::Hunk);
        assert_eq!(DiffLine::parse_type("+added line"), DiffLineType::Addition);
        assert_eq!(DiffLine::parse_type("-deleted line"), DiffLineType::Deletion);
        assert_eq!(DiffLine::parse_type(" context line"), DiffLineType::Context);
        assert_eq!(DiffLine::parse_type("\\ No newline at end of file"), DiffLineType::NoNewline);
    }

    #[test]
    fn test_diff_viewer_new() {
        let viewer = GitDiffViewer::new();
        assert_eq!(viewer.line_count(), 0);
        assert!(viewer.show_line_numbers);
        assert!(viewer.syntax_highlighting);
    }

    #[test]
    fn test_diff_viewer_set_diff() {
        let mut viewer = GitDiffViewer::new();
        let diff = "diff --git a/test.txt b/test.txt\n+new line\n-old line";
        viewer.set_diff(diff);

        assert_eq!(viewer.line_count(), 3);
    }

    #[test]
    fn test_diff_viewer_stats() {
        let mut viewer = GitDiffViewer::new();
        let diff = "diff --git a/test.txt b/test.txt\n+line1\n+line2\n-line3";
        viewer.set_diff(diff);

        let (additions, deletions, _context) = viewer.stats();
        assert_eq!(additions, 2);
        assert_eq!(deletions, 1);
    }

    #[test]
    fn test_parse_hunk_header() {
        assert_eq!(
            GitDiffViewer::parse_hunk_header("@@ -1,4 +1,5 @@"),
            Some((1, 1))
        );
        assert_eq!(
            GitDiffViewer::parse_hunk_header("@@ -10,7 +12,9 @@"),
            Some((10, 12))
        );
        assert_eq!(GitDiffViewer::parse_hunk_header("invalid"), None);
    }

    #[test]
    fn test_diff_viewer_clear() {
        let mut viewer = GitDiffViewer::new();
        viewer.set_diff("+line");
        assert_eq!(viewer.line_count(), 1);
        viewer.clear();
        assert_eq!(viewer.line_count(), 0);
    }

    #[test]
    fn test_diff_viewer_builder() {
        let viewer = GitDiffViewer::new()
            .with_title("Custom Diff")
            .with_line_numbers(false)
            .with_syntax_highlighting(false)
            .with_compact(true);

        assert_eq!(viewer.title, "Custom Diff");
        assert!(!viewer.show_line_numbers);
        assert!(!viewer.syntax_highlighting);
        assert!(viewer.compact);
    }

    #[test]
    fn test_diff_line_colors() {
        assert_eq!(DiffLineType::Addition.color(), Color::Green);
        assert_eq!(DiffLineType::Deletion.color(), Color::Red);
        assert_eq!(DiffLineType::Hunk.color(), Color::Cyan);
        assert_eq!(DiffLineType::FileHeader.color(), Color::Yellow);
    }

    #[test]
    fn test_diff_line_bg_colors() {
        assert_eq!(DiffLineType::Addition.bg_color(), Some(Color::Rgb(0, 64, 0)));
        assert_eq!(DiffLineType::Deletion.bg_color(), Some(Color::Rgb(64, 0, 0)));
        assert_eq!(DiffLineType::Context.bg_color(), None);
    }

    #[test]
    fn test_full_diff_parsing() {
        let diff = r#"diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcdef 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,4 +1,5 @@
 fn main() {
+    println!("Hello, world!");
     println!("Goodbye!");
 }
"#;

        let mut viewer = GitDiffViewer::new();
        viewer.set_diff(diff);

        assert!(viewer.line_count() > 0);

        let (additions, deletions, context) = viewer.stats();
        assert_eq!(additions, 1);
        assert_eq!(deletions, 0);
        assert_eq!(context, 3); // "fn main() {", "println!("Goodbye!");", "}"
    }
}
