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

use crate::ui::atoms::{block::Block as AtomBlock, text::Text};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
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

    /// Render header lines using Text atoms
    fn render_header(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Branch info
        if let Some(ref branch) = self.branch {
            let mut spans = vec![
                Text::new("⎇ ")
                    .style(Style::default().fg(Color::Cyan))
                    .to_span(),
                Text::new(branch)
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .to_span(),
            ];

            // Ahead/behind info
            if self.ahead > 0 || self.behind > 0 {
                spans.push(Text::new(" ").to_span());
                if self.ahead > 0 {
                    spans.push(
                        Text::new(format!("↑{}", self.ahead))
                            .style(Style::default().fg(Color::Green))
                            .to_span(),
                    );
                }
                if self.behind > 0 {
                    if self.ahead > 0 {
                        spans.push(Text::new(" ").to_span());
                    }
                    spans.push(
                        Text::new(format!("↓{}", self.behind))
                            .style(Style::default().fg(Color::Red))
                            .to_span(),
                    );
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
                let summary_text =
                    Text::new(summary_parts.join(", ")).style(Style::default().fg(Color::DarkGray));
                lines.push(Line::from(summary_text.to_span()));
            }
        }

        if !self.compact && !lines.is_empty() {
            lines.push(Line::from(""));
        }

        lines
    }

    /// Render file list items using Text atoms
    fn render_list_items(&self) -> Vec<ListItem<'static>> {
        let mut items = Vec::new();

        for file in &self.files {
            // Use Text atoms for each component
            let status_text = Text::new(format!("{} ", file.status.char()))
                .style(Style::default().fg(file.status.color()));

            let selection_char = if file.selected { "☑ " } else { "☐ " };
            let selection_text =
                Text::new(selection_char).style(Style::default().fg(if file.selected {
                    Color::Green
                } else {
                    Color::DarkGray
                }));

            let path_str = file.path.to_string_lossy().to_string();
            let path_text = Text::new(path_str);

            let line = Line::from(vec![
                selection_text.to_span(),
                status_text.to_span(),
                path_text.to_span(),
            ]);
            items.push(ListItem::new(line));
        }

        if items.is_empty() {
            let empty_text = Text::new("No changes").style(Style::default().fg(Color::DarkGray));
            items.push(ListItem::new(Line::from(empty_text.to_span())));
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

        // Create block with title using Block atom
        let block = AtomBlock::new()
            .title(&self.title)
            .border_style(Style::default().fg(Color::Cyan))
            .to_ratatui();

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

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Git Integration)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_panel_10000_files() {
        let mut panel = GitStatusPanel::new();
        for i in 0..10000 {
            panel.add_file(format!("file{}.rs", i), FileStatus::Modified);
        }
        assert_eq!(panel.file_count(), 10000);
    }

    #[test]
    fn test_panel_rapid_add_1000() {
        let mut panel = GitStatusPanel::new();
        for i in 0..1000 {
            panel.add_file(format!("file{}.rs", i), FileStatus::Modified);
        }
        assert_eq!(panel.file_count(), 1000);
    }

    #[test]
    fn test_panel_rapid_toggle_1000() {
        let mut panel = GitStatusPanel::new();
        for i in 0..100 {
            panel.add_file(format!("file{}.rs", i), FileStatus::Modified);
        }
        for _ in 0..1000 {
            panel.toggle_selection(0);
        }
        // Should end up not selected (1000 toggles = even number)
        assert!(!panel.files[0].selected);
    }

    #[test]
    fn test_panel_very_long_file_path() {
        let mut panel = GitStatusPanel::new();
        let long_path = "a/".repeat(1000) + "file.rs";
        panel.add_file(&long_path, FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_file_with_emoji() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("🚀_rocket.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_rtl_arabic() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("مرحبا.txt", FileStatus::Staged);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_rtl_hebrew() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("שלום.txt", FileStatus::Untracked);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_japanese() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("日本語.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_mixed_scripts() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("Hello_مرحبا_שלום_こんにちは.txt", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_branch_unicode() {
        let mut panel = GitStatusPanel::new();
        panel.set_branch("feature/🚀-rocket");
        assert_eq!(panel.branch, Some("feature/🚀-rocket".to_string()));
    }

    #[test]
    fn test_branch_japanese() {
        let mut panel = GitStatusPanel::new();
        panel.set_branch("日本語-branch");
        assert_eq!(panel.branch, Some("日本語-branch".to_string()));
    }

    // ============ Extreme Values ============

    #[test]
    fn test_ahead_behind_max() {
        let mut panel = GitStatusPanel::new();
        panel.set_ahead_behind(usize::MAX, usize::MAX);
        assert_eq!(panel.ahead, usize::MAX);
        assert_eq!(panel.behind, usize::MAX);
    }

    #[test]
    fn test_empty_file_list() {
        let panel = GitStatusPanel::new();
        assert_eq!(panel.file_count(), 0);
        let (modified, staged, untracked) = panel.file_counts();
        assert_eq!(modified, 0);
        assert_eq!(staged, 0);
        assert_eq!(untracked, 0);
    }

    #[test]
    fn test_very_long_branch_name() {
        let mut panel = GitStatusPanel::new();
        let long_branch = "feature/".to_string() + &"very-long-name-".repeat(1000);
        panel.set_branch(&long_branch);
        assert_eq!(panel.branch, Some(long_branch));
    }

    #[test]
    fn test_file_path_1000_directories() {
        let mut panel = GitStatusPanel::new();
        let deep_path = "dir/".repeat(1000) + "file.rs";
        panel.add_file(&deep_path, FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    // ============ Selection Edge Cases ============

    #[test]
    fn test_toggle_selection_out_of_bounds() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("test.rs", FileStatus::Modified);
        panel.toggle_selection(999);
        // Should not panic, just do nothing
        assert!(!panel.files[0].selected);
    }

    #[test]
    fn test_select_all_files() {
        let mut panel = GitStatusPanel::new();
        for i in 0..10 {
            panel.add_file(format!("file{}.rs", i), FileStatus::Modified);
        }
        for i in 0..10 {
            panel.toggle_selection(i);
        }
        let selected = panel.selected_files();
        assert_eq!(selected.len(), 10);
    }

    #[test]
    fn test_select_none() {
        let mut panel = GitStatusPanel::new();
        for i in 0..10 {
            panel.add_file(format!("file{}.rs", i), FileStatus::Modified);
        }
        let selected = panel.selected_files();
        assert_eq!(selected.len(), 0);
    }

    #[test]
    fn test_selected_files_empty_list() {
        let panel = GitStatusPanel::new();
        let selected = panel.selected_files();
        assert_eq!(selected.len(), 0);
    }

    // ============ File Status Edge Cases ============

    #[test]
    fn test_all_file_statuses() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("modified.rs", FileStatus::Modified);
        panel.add_file("staged.rs", FileStatus::Staged);
        panel.add_file("untracked.rs", FileStatus::Untracked);
        panel.add_file("deleted.rs", FileStatus::Deleted);
        panel.add_file("renamed.rs", FileStatus::Renamed);
        panel.add_file("conflicted.rs", FileStatus::Conflicted);
        panel.add_file("modified_staged.rs", FileStatus::ModifiedStaged);
        assert_eq!(panel.file_count(), 7);
    }

    #[test]
    fn test_file_status_modified_staged() {
        assert_eq!(FileStatus::ModifiedStaged.char(), "M");
        assert_eq!(FileStatus::ModifiedStaged.color(), Color::Yellow);
    }

    #[test]
    fn test_file_counts_all_statuses() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("modified.rs", FileStatus::Modified);
        panel.add_file("staged.rs", FileStatus::Staged);
        panel.add_file("untracked.rs", FileStatus::Untracked);
        panel.add_file("deleted.rs", FileStatus::Deleted);
        panel.add_file("renamed.rs", FileStatus::Renamed);
        panel.add_file("conflicted.rs", FileStatus::Conflicted);
        panel.add_file("modified_staged.rs", FileStatus::ModifiedStaged);

        let (modified, staged, untracked) = panel.file_counts();
        assert_eq!(modified, 2); // Modified + ModifiedStaged
        assert_eq!(staged, 1);
        assert_eq!(untracked, 1);
    }

    #[test]
    fn test_file_status_deleted_color() {
        assert_eq!(FileStatus::Deleted.color(), Color::Red);
    }

    #[test]
    fn test_file_status_renamed_color() {
        assert_eq!(FileStatus::Renamed.color(), Color::Cyan);
    }

    #[test]
    fn test_file_status_conflicted_color() {
        assert_eq!(FileStatus::Conflicted.color(), Color::Magenta);
    }

    // ============ Trait Coverage ============

    #[test]
    fn test_file_status_clone() {
        let status = FileStatus::Modified;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_file_status_copy() {
        let status = FileStatus::Staged;
        let copied = status;
        assert_eq!(status, copied);
    }

    #[test]
    fn test_file_status_equality() {
        assert_eq!(FileStatus::Modified, FileStatus::Modified);
        assert_ne!(FileStatus::Modified, FileStatus::Staged);
    }

    #[test]
    fn test_file_status_debug() {
        let status = FileStatus::Untracked;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Untracked"));
    }

    #[test]
    fn test_git_file_clone() {
        let file = GitFile::new("test.rs", FileStatus::Modified);
        let cloned = file.clone();
        assert_eq!(file.path, cloned.path);
        assert_eq!(file.status, cloned.status);
    }

    #[test]
    fn test_git_file_debug() {
        let file = GitFile::new("test.rs", FileStatus::Modified);
        let debug_str = format!("{:?}", file);
        assert!(debug_str.contains("GitFile"));
    }

    #[test]
    fn test_panel_clone() {
        let panel = GitStatusPanel::new().with_title("Test");
        let cloned = panel.clone();
        assert_eq!(panel.title, cloned.title);
    }

    #[test]
    fn test_panel_debug() {
        let panel = GitStatusPanel::new();
        let debug_str = format!("{:?}", panel);
        assert!(debug_str.contains("GitStatusPanel"));
    }

    #[test]
    fn test_panel_default() {
        let panel = GitStatusPanel::default();
        assert_eq!(panel.file_count(), 0);
        assert_eq!(panel.title, "Git Status");
        assert!(panel.show_summary);
        assert!(!panel.compact);
    }

    // ============ Complex Workflows ============

    #[test]
    fn test_add_select_clear_add_workflow() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("file1.rs", FileStatus::Modified);
        panel.add_file("file2.rs", FileStatus::Staged);
        panel.toggle_selection(0);
        assert_eq!(panel.selected_files().len(), 1);

        panel.clear();
        assert_eq!(panel.file_count(), 0);
        assert_eq!(panel.selected_files().len(), 0);

        panel.add_file("file3.rs", FileStatus::Untracked);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_toggle_summary_with_without_files() {
        let panel = GitStatusPanel::new().with_summary(true);
        assert!(panel.show_summary);

        let panel = panel.with_summary(false);
        assert!(!panel.show_summary);
    }

    #[test]
    fn test_set_branch_files_ahead_behind_order() {
        let mut panel = GitStatusPanel::new();

        // Set in different order
        panel.add_file("file.rs", FileStatus::Modified);
        panel.set_ahead_behind(5, 2);
        panel.set_branch("develop");

        assert_eq!(panel.file_count(), 1);
        assert_eq!(panel.ahead, 5);
        assert_eq!(panel.behind, 2);
        assert_eq!(panel.branch, Some("develop".to_string()));
    }

    #[test]
    fn test_multiple_set_files_calls() {
        let mut panel = GitStatusPanel::new();

        let files1 = vec![GitFile::new("file1.rs", FileStatus::Modified)];
        panel.set_files(files1);
        assert_eq!(panel.file_count(), 1);

        let files2 = vec![
            GitFile::new("file2.rs", FileStatus::Staged),
            GitFile::new("file3.rs", FileStatus::Untracked),
        ];
        panel.set_files(files2);
        assert_eq!(panel.file_count(), 2);
    }

    // ============ Branch Edge Cases ============

    #[test]
    fn test_no_branch_detached_head() {
        let panel = GitStatusPanel::new();
        assert_eq!(panel.branch, None);
    }

    #[test]
    fn test_branch_with_slashes() {
        let mut panel = GitStatusPanel::new();
        panel.set_branch("feature/TOAD-123/some-feature");
        assert_eq!(
            panel.branch,
            Some("feature/TOAD-123/some-feature".to_string())
        );
    }

    #[test]
    fn test_branch_empty_string() {
        let mut panel = GitStatusPanel::new();
        panel.set_branch("");
        assert_eq!(panel.branch, Some("".to_string()));
    }

    // ============ Ahead/Behind Edge Cases ============

    #[test]
    fn test_ahead_only() {
        let mut panel = GitStatusPanel::new();
        panel.set_ahead_behind(5, 0);
        assert_eq!(panel.ahead, 5);
        assert_eq!(panel.behind, 0);
    }

    #[test]
    fn test_behind_only() {
        let mut panel = GitStatusPanel::new();
        panel.set_ahead_behind(0, 3);
        assert_eq!(panel.ahead, 0);
        assert_eq!(panel.behind, 3);
    }

    #[test]
    fn test_both_ahead_and_behind() {
        let mut panel = GitStatusPanel::new();
        panel.set_ahead_behind(10, 5);
        assert_eq!(panel.ahead, 10);
        assert_eq!(panel.behind, 5);
    }

    #[test]
    fn test_neither_ahead_nor_behind() {
        let mut panel = GitStatusPanel::new();
        panel.set_ahead_behind(0, 0);
        assert_eq!(panel.ahead, 0);
        assert_eq!(panel.behind, 0);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_git_status_panel_stress() {
        let mut panel = GitStatusPanel::new()
            .with_title("Comprehensive Test")
            .with_summary(true)
            .with_compact(false);

        // Phase 1: Set branch and ahead/behind
        panel.set_branch("feature/comprehensive-test");
        panel.set_ahead_behind(10, 3);
        assert_eq!(panel.branch, Some("feature/comprehensive-test".to_string()));
        assert_eq!(panel.ahead, 10);
        assert_eq!(panel.behind, 3);

        // Phase 2: Add files with all status types
        panel.add_file("modified.rs", FileStatus::Modified);
        panel.add_file("staged.rs", FileStatus::Staged);
        panel.add_file("untracked.rs", FileStatus::Untracked);
        panel.add_file("deleted.rs", FileStatus::Deleted);
        panel.add_file("renamed.rs", FileStatus::Renamed);
        panel.add_file("conflicted.rs", FileStatus::Conflicted);
        panel.add_file("modified_staged.rs", FileStatus::ModifiedStaged);
        assert_eq!(panel.file_count(), 7);

        // Phase 3: Add unicode files
        panel.add_file("🚀_rocket.rs", FileStatus::Modified);
        panel.add_file("日本語.txt", FileStatus::Staged);
        panel.add_file("مرحبا.rs", FileStatus::Untracked);
        assert_eq!(panel.file_count(), 10);

        // Phase 4: Select some files
        panel.toggle_selection(0);
        panel.toggle_selection(2);
        panel.toggle_selection(5);
        let selected = panel.selected_files();
        assert_eq!(selected.len(), 3);

        // Phase 5: Verify file counts
        let (modified, staged, untracked) = panel.file_counts();
        assert_eq!(modified, 3); // Modified + ModifiedStaged + 🚀
        assert_eq!(staged, 2); // Staged + 日本語
        assert_eq!(untracked, 2); // Untracked + مرحبا

        // Phase 6: Toggle features
        panel = panel.with_summary(false).with_compact(true);
        assert!(!panel.show_summary);
        assert!(panel.compact);

        // Phase 7: Clear and reset
        panel.clear();
        assert_eq!(panel.file_count(), 0);
        assert_eq!(panel.selected_files().len(), 0);

        // Phase 8: Add new files with set_files
        let new_files = vec![
            GitFile::new("new1.rs", FileStatus::Modified),
            GitFile::new("new2.rs", FileStatus::Staged),
        ];
        panel.set_files(new_files);
        assert_eq!(panel.file_count(), 2);
    }

    // ============ File Path Edge Cases ============

    #[test]
    fn test_file_absolute_path() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("/absolute/path/to/file.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_relative_path() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("./relative/path/file.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_parent_directory() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("../parent/file.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_with_spaces() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("file with spaces.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }

    #[test]
    fn test_file_with_special_chars() {
        let mut panel = GitStatusPanel::new();
        panel.add_file("file-name_123.test.rs", FileStatus::Modified);
        assert_eq!(panel.file_count(), 1);
    }
}
