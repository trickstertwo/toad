//! Git status panel widget for displaying repository status
//!
//! Shows modified, staged, and untracked files in an interactive list
//! with file status indicators and branch information.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::{GitStatusPanel, FileStatus};
//!
//! let mut panel = GitStatusPanel::new();
//! panel.add_file("src/main.rs", FileStatus::Modified);
//! panel.add_file("README.md", FileStatus::Staged);
//! panel.set_branch("main");
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget},
};
use std::path::PathBuf;

/// File status in git repository
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    /// File is modified but not staged
    Modified,
    /// File is staged for commit
    Staged,
    /// File is untracked
    Untracked,
    /// File is deleted
    Deleted,
    /// File is renamed
    Renamed,
    /// File has conflicts
    Conflicted,
    /// File is both modified and staged (partially staged)
    ModifiedStaged,
}

impl FileStatus {
    /// Get the display character for this status
    pub fn char(&self) -> &str {
        match self {
            FileStatus::Modified => "M",
            FileStatus::Staged => "A",
            FileStatus::Untracked => "?",
            FileStatus::Deleted => "D",
            FileStatus::Renamed => "R",
            FileStatus::Conflicted => "C",
            FileStatus::ModifiedStaged => "M",
        }
    }

    /// Get the color for this status
    pub fn color(&self) -> Color {
        match self {
            FileStatus::Modified => Color::Yellow,
            FileStatus::Staged => Color::Green,
            FileStatus::Untracked => Color::Red,
            FileStatus::Deleted => Color::Red,
            FileStatus::Renamed => Color::Cyan,
            FileStatus::Conflicted => Color::Magenta,
            FileStatus::ModifiedStaged => Color::Yellow,
        }
    }
}

/// A file in the git status
#[derive(Debug, Clone)]
pub struct GitFile {
    /// File path
    pub path: PathBuf,
    /// File status
    pub status: FileStatus,
    /// Selected state
    pub selected: bool,
}

impl GitFile {
    /// Create a new git file
    pub fn new(path: impl Into<PathBuf>, status: FileStatus) -> Self {
        Self {
            path: path.into(),
            status,
            selected: false,
        }
    }
}

/// Git status panel widget
///
/// Displays repository status with file changes grouped by status.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::{GitStatusPanel, FileStatus};
///
/// let mut panel = GitStatusPanel::new();
/// panel.set_branch("main");
/// panel.set_ahead_behind(3, 1);
/// panel.add_file("src/lib.rs", FileStatus::Modified);
/// ```
#[derive(Debug, Clone)]
pub struct GitStatusPanel {
    /// Current branch name
    branch: Option<String>,
    /// Commits ahead of remote
    ahead: usize,
    /// Commits behind remote
    behind: usize,
    /// List of files with their status
    files: Vec<GitFile>,
    /// Title of the panel
    title: String,
    /// Show file counts summary
    show_summary: bool,
    /// Compact mode (less spacing)
    compact: bool,
}

impl Default for GitStatusPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl GitStatusPanel {
    /// Create a new git status panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::GitStatusPanel;
    ///
    /// let panel = GitStatusPanel::new();
    /// ```
    pub fn new() -> Self {
        Self {
            branch: None,
            ahead: 0,
            behind: 0,
            files: Vec::new(),
            title: "Git Status".to_string(),
            show_summary: true,
            compact: false,
        }
    }

    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set branch name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::GitStatusPanel;
    ///
    /// let mut panel = GitStatusPanel::new();
    /// panel.set_branch("main");
    /// ```
    pub fn set_branch(&mut self, branch: impl Into<String>) {
        self.branch = Some(branch.into());
    }

    /// Set commits ahead/behind remote
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::GitStatusPanel;
    ///
    /// let mut panel = GitStatusPanel::new();
    /// panel.set_ahead_behind(3, 1); // 3 ahead, 1 behind
    /// ```
    pub fn set_ahead_behind(&mut self, ahead: usize, behind: usize) {
        self.ahead = ahead;
        self.behind = behind;
    }

    /// Add a file with its status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::{GitStatusPanel, FileStatus};
    ///
    /// let mut panel = GitStatusPanel::new();
    /// panel.add_file("src/main.rs", FileStatus::Modified);
    /// ```
    pub fn add_file(&mut self, path: impl Into<PathBuf>, status: FileStatus) {
        self.files.push(GitFile::new(path, status));
    }

    /// Set all files at once
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::{GitStatusPanel, GitFile, FileStatus};
    ///
    /// let mut panel = GitStatusPanel::new();
    /// panel.set_files(vec![
    ///     GitFile::new("file1.rs", FileStatus::Modified),
    ///     GitFile::new("file2.rs", FileStatus::Staged),
    /// ]);
    /// ```
    pub fn set_files(&mut self, files: Vec<GitFile>) {
        self.files = files;
    }

    /// Clear all files
    pub fn clear(&mut self) {
        self.files.clear();
    }

    /// Get the number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Show or hide summary
    pub fn with_summary(mut self, show: bool) -> Self {
        self.show_summary = show;
        self
    }

    /// Enable compact mode
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Toggle file selection at index
    pub fn toggle_selection(&mut self, index: usize) {
        if let Some(file) = self.files.get_mut(index) {
            file.selected = !file.selected;
        }
    }

    /// Get selected files
    pub fn selected_files(&self) -> Vec<&GitFile> {
        self.files.iter().filter(|f| f.selected).collect()
    }

    /// Get file status counts
    fn file_counts(&self) -> (usize, usize, usize) {
        let mut modified = 0;
        let mut staged = 0;
        let mut untracked = 0;

        for file in &self.files {
            match file.status {
                FileStatus::Modified | FileStatus::ModifiedStaged => modified += 1,
                FileStatus::Staged => staged += 1,
                FileStatus::Untracked => untracked += 1,
                _ => {}
            }
        }

        (modified, staged, untracked)
    }

    /// Render header lines
    fn render_header(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Branch info
        if let Some(ref branch) = self.branch {
            let mut spans = vec![
                Span::styled("⎇ ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    branch.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ];

            // Ahead/behind info
            if self.ahead > 0 || self.behind > 0 {
                spans.push(Span::raw(" "));
                if self.ahead > 0 {
                    spans.push(Span::styled(
                        format!("↑{}", self.ahead),
                        Style::default().fg(Color::Green),
                    ));
                }
                if self.behind > 0 {
                    if self.ahead > 0 {
                        spans.push(Span::raw(" "));
                    }
                    spans.push(Span::styled(
                        format!("↓{}", self.behind),
                        Style::default().fg(Color::Red),
                    ));
                }
            }

            lines.push(Line::from(spans));
        }

        // Summary
        if self.show_summary && !self.files.is_empty() {
            let (modified, staged, untracked) = self.file_counts();
            let mut summary_parts = Vec::new();

            if modified > 0 {
                summary_parts.push(format!("{} modified", modified));
            }
            if staged > 0 {
                summary_parts.push(format!("{} staged", staged));
            }
            if untracked > 0 {
                summary_parts.push(format!("{} untracked", untracked));
            }

            if !summary_parts.is_empty() {
                lines.push(Line::from(Span::styled(
                    summary_parts.join(", "),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }

        if !self.compact && !lines.is_empty() {
            lines.push(Line::from(""));
        }

        lines
    }

    /// Render file list items
    fn render_list_items(&self) -> Vec<ListItem<'static>> {
        let mut items = Vec::new();

        for file in &self.files {
            let status_span = Span::styled(
                format!("{} ", file.status.char()),
                Style::default().fg(file.status.color()),
            );

            let selection_char = if file.selected { "☑ " } else { "☐ " };
            let selection_span = Span::styled(
                selection_char,
                Style::default().fg(if file.selected {
                    Color::Green
                } else {
                    Color::DarkGray
                }),
            );

            let path_str = file.path.to_string_lossy().to_string();
            let path_span = Span::raw(path_str);

            let line = Line::from(vec![selection_span, status_span, path_span]);
            items.push(ListItem::new(line));
        }

        if items.is_empty() {
            items.push(ListItem::new(Line::from(Span::styled(
                "No changes",
                Style::default().fg(Color::DarkGray),
            ))));
        }

        items
    }
}

/// Stateful widget implementation for interactive list
impl StatefulWidget for &GitStatusPanel {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_lines = self.render_header();
        let list_items = self.render_list_items();

        // Create block with title
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.clone())
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        block.render(area, buf);

        // Render header
        let header_height = header_lines.len() as u16;
        for (i, line) in header_lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }

        // Render list below header
        if inner.height > header_height {
            let list_area = Rect {
                x: inner.x,
                y: inner.y + header_height,
                width: inner.width,
                height: inner.height.saturating_sub(header_height),
            };

            let list = List::new(list_items)
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");

            StatefulWidget::render(list, list_area, buf, state);
        }
    }
}

/// Regular widget implementation (without selection)
impl Widget for &GitStatusPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_status_char() {
        assert_eq!(FileStatus::Modified.char(), "M");
        assert_eq!(FileStatus::Staged.char(), "A");
        assert_eq!(FileStatus::Untracked.char(), "?");
        assert_eq!(FileStatus::Deleted.char(), "D");
        assert_eq!(FileStatus::Renamed.char(), "R");
        assert_eq!(FileStatus::Conflicted.char(), "C");
    }

    #[test]
    fn test_file_status_color() {
        assert_eq!(FileStatus::Modified.color(), Color::Yellow);
        assert_eq!(FileStatus::Staged.color(), Color::Green);
        assert_eq!(FileStatus::Untracked.color(), Color::Red);
    }

    #[test]
    fn test_git_file_new() {
        let file = GitFile::new("src/main.rs", FileStatus::Modified);
        assert_eq!(file.path, PathBuf::from("src/main.rs"));
        assert_eq!(file.status, FileStatus::Modified);
        assert!(!file.selected);
    }

    #[test]
    fn test_git_status_panel_new() {
        let panel = GitStatusPanel::new();
        assert_eq!(panel.file_count(), 0);
        assert_eq!(panel.branch, None);
        assert_eq!(panel.ahead, 0);
        assert_eq!(panel.behind, 0);
    }

    #[test]
    fn test_git_status_panel_add_file() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("test.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_git_status_panel_set_branch() {
        let mut panel = GitStatusPanel::new();
        panel.set_branch("main");
        assert_eq!(panel.branch, Some("main".to_string()));
    }

    #[test]
    fn test_git_status_panel_set_ahead_behind() {
        let mut panel = GitStatusPanel::new();
        panel.set_ahead_behind(3, 1);
        assert_eq!(panel.ahead, 3);
        assert_eq!(panel.behind, 1);
    }

    #[test]
    fn test_git_status_panel_clear() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("test1.rs", FileStatus::Modified);
        panel.add_file("test2.rs", FileStatus::Staged);
        assert_eq!(panel.file_count(), 2);
        panel.clear();
        assert_eq!(panel.file_count(), 0);
    }

    #[test]
    fn test_git_status_panel_toggle_selection() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("test.rs", FileStatus::Modified);
        assert!(!panel.files[0].selected);
        panel.toggle_selection(0);
        assert!(panel.files[0].selected);
        panel.toggle_selection(0);
        assert!(!panel.files[0].selected);
    }

    #[test]
    fn test_git_status_panel_selected_files() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("test1.rs", FileStatus::Modified);
        panel.add_file("test2.rs", FileStatus::Staged);
        panel.add_file("test3.rs", FileStatus::Untracked);

        panel.toggle_selection(0);
        panel.toggle_selection(2);

        let selected = panel.selected_files();
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].path, PathBuf::from("test1.rs"));
        assert_eq!(selected[1].path, PathBuf::from("test3.rs"));
    }

    #[test]
    fn test_git_status_panel_file_counts() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("modified.rs", FileStatus::Modified);
        panel.add_file("staged.rs", FileStatus::Staged);
        panel.add_file("untracked.rs", FileStatus::Untracked);
        panel.add_file("deleted.rs", FileStatus::Deleted);

        let (modified, staged, untracked) = panel.file_counts();
        assert_eq!(modified, 1);
        assert_eq!(staged, 1);
        assert_eq!(untracked, 1);
    }

    #[test]
    fn test_git_status_panel_builder() {
        let panel = GitStatusPanel::new()
            .with_title("Custom Title")
            .with_summary(false)
            .with_compact(true);

        assert_eq!(panel.title, "Custom Title");
        assert!(!panel.show_summary);
        assert!(panel.compact);
    }

    #[test]
    fn test_git_status_panel_set_files() {
        let mut panel = GitStatusPanel::new();
        let files = vec![
            GitFile::new("file1.rs", FileStatus::Modified),
            GitFile::new("file2.rs", FileStatus::Staged),
        ];
        panel.set_files(files);
        assert_eq!(panel.file_count(), 2);
    }
}
