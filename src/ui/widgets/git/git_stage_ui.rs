//! Interactive Git staging UI widget
//!
//! Provides a visual interface for staging and unstaging files, similar to
//! lazygit's staging interface. Integrates with GitService for operations.
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::GitStageUI;
//! use toad::git::GitService;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let service = GitService::new(".")?;
//! let mut stage_ui = GitStageUI::new(service);
//! stage_ui.refresh().await?;
//! # Ok(())
//! # }
//! ```

use crate::git::{FileChange, GitService};
use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};
use std::path::PathBuf;

/// File entry in the staging UI
#[derive(Debug, Clone)]
pub struct StageFileEntry {
    /// File path
    pub path: PathBuf,
    /// Change type
    pub change: FileChangeType,
    /// Whether file is currently staged
    pub is_staged: bool,
    /// Whether entry is selected for batch operation
    pub is_selected: bool,
}

/// Type of file change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileChangeType {
    /// File modified
    Modified,
    /// File added
    Added,
    /// File deleted
    Deleted,
    /// File renamed
    Renamed,
    /// File has conflicts
    Conflicted,
}

impl FileChangeType {
    /// Get display character
    pub fn char(&self) -> &str {
        match self {
            FileChangeType::Modified => "M",
            FileChangeType::Added => "A",
            FileChangeType::Deleted => "D",
            FileChangeType::Renamed => "R",
            FileChangeType::Conflicted => "C",
        }
    }

    /// Get color for this change type
    pub fn color(&self) -> Color {
        match self {
            FileChangeType::Modified => Color::Yellow,
            FileChangeType::Added => Color::Green,
            FileChangeType::Deleted => Color::Red,
            FileChangeType::Renamed => Color::Cyan,
            FileChangeType::Conflicted => Color::Magenta,
        }
    }
}

impl From<&FileChange> for FileChangeType {
    fn from(change: &FileChange) -> Self {
        match change {
            FileChange::Modified(_) => FileChangeType::Modified,
            FileChange::Staged(_) => FileChangeType::Added,
            FileChange::Untracked(_) => FileChangeType::Added,
            FileChange::Deleted(_) => FileChangeType::Deleted,
            FileChange::Renamed(_, _) => FileChangeType::Renamed,
            FileChange::Conflicted(_) => FileChangeType::Conflicted,
        }
    }
}

/// Interactive git staging UI
///
/// # Keybindings
///
/// - `Space`: Stage/unstage selected file
/// - `a`: Stage all unstaged files
/// - `u`: Unstage all staged files
/// - `v`: Toggle visual selection mode
/// - `j/k`: Navigate up/down
/// - `gg/G`: Jump to top/bottom
pub struct GitStageUI {
    /// Git service for operations
    service: GitService,
    /// Unstaged files
    unstaged: Vec<StageFileEntry>,
    /// Staged files
    staged: Vec<StageFileEntry>,
    /// Current focus (Unstaged or Staged pane)
    focus: StagePane,
    /// List state for unstaged pane
    unstaged_state: ListState,
    /// List state for staged pane
    staged_state: ListState,
    /// Visual selection mode
    visual_mode: bool,
    /// Last operation message
    message: Option<String>,
    /// Branch name
    branch: String,
}

/// Which pane is currently focused
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagePane {
    /// Unstaged files pane
    Unstaged,
    /// Staged files pane
    Staged,
}

impl GitStageUI {
    /// Create a new git staging UI
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::GitStageUI;
    /// use toad::git::GitService;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let service = GitService::new(".")?;
    /// let stage_ui = GitStageUI::new(service);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(service: GitService) -> Self {
        let mut unstaged_state = ListState::default();
        unstaged_state.select(Some(0));

        let mut staged_state = ListState::default();
        staged_state.select(Some(0));

        Self {
            service,
            unstaged: Vec::new(),
            staged: Vec::new(),
            focus: StagePane::Unstaged,
            unstaged_state,
            staged_state,
            visual_mode: false,
            message: None,
            branch: String::from("unknown"),
        }
    }

    /// Refresh the file list from git status
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::ui::widgets::GitStageUI;
    /// # use toad::git::GitService;
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// # let service = GitService::new(".")?;
    /// let mut stage_ui = GitStageUI::new(service);
    /// stage_ui.refresh().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refresh(&mut self) -> Result<()> {
        // Get current branch
        self.branch = self
            .service
            .current_branch()
            .await
            .unwrap_or_else(|_| String::from("unknown"));

        // Get file changes
        let changes = self.service.status().await?;

        self.unstaged.clear();
        self.staged.clear();

        for change in changes {
            let change_type = FileChangeType::from(&change);
            let path = change.path().to_path_buf();

            match change {
                FileChange::Staged(_) => {
                    self.staged.push(StageFileEntry {
                        path,
                        change: change_type,
                        is_staged: true,
                        is_selected: false,
                    });
                }
                _ => {
                    self.unstaged.push(StageFileEntry {
                        path,
                        change: change_type,
                        is_staged: false,
                        is_selected: false,
                    });
                }
            }
        }

        // Ensure selection is valid
        if !self.unstaged.is_empty() && self.unstaged_state.selected().is_none() {
            self.unstaged_state.select(Some(0));
        }
        if !self.staged.is_empty() && self.staged_state.selected().is_none() {
            self.staged_state.select(Some(0));
        }

        Ok(())
    }

    /// Stage the currently selected file
    pub async fn stage_selected(&mut self) -> Result<()> {
        if let Some(idx) = self.unstaged_state.selected()
            && let Some(entry) = self.unstaged.get(idx)
        {
            self.service.stage(&entry.path).await?;
            self.message = Some(format!("Staged: {}", entry.path.display()));
            self.refresh().await?;
        }
        Ok(())
    }

    /// Unstage the currently selected file
    pub async fn unstage_selected(&mut self) -> Result<()> {
        if let Some(idx) = self.staged_state.selected()
            && let Some(entry) = self.staged.get(idx)
        {
            self.service.unstage(&entry.path).await?;
            self.message = Some(format!("Unstaged: {}", entry.path.display()));
            self.refresh().await?;
        }
        Ok(())
    }

    /// Stage all unstaged files
    pub async fn stage_all(&mut self) -> Result<()> {
        let count = self.unstaged.len();
        for entry in &self.unstaged {
            self.service.stage(&entry.path).await?;
        }
        self.message = Some(format!("Staged {} files", count));
        self.refresh().await?;
        Ok(())
    }

    /// Unstage all staged files
    pub async fn unstage_all(&mut self) -> Result<()> {
        let count = self.staged.len();
        for entry in &self.staged {
            self.service.unstage(&entry.path).await?;
        }
        self.message = Some(format!("Unstaged {} files", count));
        self.refresh().await?;
        Ok(())
    }

    /// Toggle stage/unstage based on current focus
    pub async fn toggle_stage(&mut self) -> Result<()> {
        match self.focus {
            StagePane::Unstaged => self.stage_selected().await,
            StagePane::Staged => self.unstage_selected().await,
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        match self.focus {
            StagePane::Unstaged => {
                if let Some(selected) = self.unstaged_state.selected()
                    && selected > 0
                {
                    self.unstaged_state.select(Some(selected - 1));
                }
            }
            StagePane::Staged => {
                if let Some(selected) = self.staged_state.selected()
                    && selected > 0
                {
                    self.staged_state.select(Some(selected - 1));
                }
            }
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        match self.focus {
            StagePane::Unstaged => {
                if let Some(selected) = self.unstaged_state.selected()
                    && selected + 1 < self.unstaged.len()
                {
                    self.unstaged_state.select(Some(selected + 1));
                }
            }
            StagePane::Staged => {
                if let Some(selected) = self.staged_state.selected()
                    && selected + 1 < self.staged.len()
                {
                    self.staged_state.select(Some(selected + 1));
                }
            }
        }
    }

    /// Jump to top of current list
    pub fn jump_to_top(&mut self) {
        match self.focus {
            StagePane::Unstaged => {
                if !self.unstaged.is_empty() {
                    self.unstaged_state.select(Some(0));
                }
            }
            StagePane::Staged => {
                if !self.staged.is_empty() {
                    self.staged_state.select(Some(0));
                }
            }
        }
    }

    /// Jump to bottom of current list
    pub fn jump_to_bottom(&mut self) {
        match self.focus {
            StagePane::Unstaged => {
                if !self.unstaged.is_empty() {
                    self.unstaged_state.select(Some(self.unstaged.len() - 1));
                }
            }
            StagePane::Staged => {
                if !self.staged.is_empty() {
                    self.staged_state.select(Some(self.staged.len() - 1));
                }
            }
        }
    }

    /// Switch focus between panes
    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            StagePane::Unstaged => StagePane::Staged,
            StagePane::Staged => StagePane::Unstaged,
        };
    }

    /// Toggle visual selection mode
    pub fn toggle_visual_mode(&mut self) {
        self.visual_mode = !self.visual_mode;
        self.message = if self.visual_mode {
            Some(String::from("Visual mode enabled"))
        } else {
            Some(String::from("Visual mode disabled"))
        };
    }

    /// Get the current focus
    pub fn focus(&self) -> StagePane {
        self.focus
    }

    /// Get unstaged file count
    pub fn unstaged_count(&self) -> usize {
        self.unstaged.len()
    }

    /// Get staged file count
    pub fn staged_count(&self) -> usize {
        self.staged.len()
    }

    /// Get the last message
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Clear the message
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Get the current branch name
    pub fn branch(&self) -> &str {
        &self.branch
    }
}

impl Widget for &mut GitStageUI {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into header, panes, and footer
        let chunks = Layout::vertical([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Panes
            Constraint::Length(1), // Footer
        ])
        .split(area);

        // Render header with branch info
        let header = Paragraph::new(vec![Line::from(vec![
            Span::styled("Branch: ", Style::default().fg(Color::Gray)),
            Span::styled(
                &self.branch,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  |  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("Unstaged: {}", self.unstaged.len()),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled("  |  ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("Staged: {}", self.staged.len()),
                Style::default().fg(Color::Green),
            ),
        ])])
        .block(Block::default().borders(Borders::ALL).title("Git Stage"));
        header.render(chunks[0], buf);

        // Split panes area horizontally
        let panes = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Render unstaged pane
        let unstaged_items: Vec<ListItem> = self
            .unstaged
            .iter()
            .map(|entry| {
                let style = Style::default().fg(entry.change.color());
                let marker = if entry.is_selected { "[x] " } else { "[ ] " };
                ListItem::new(Line::from(vec![
                    Span::styled(marker, style),
                    Span::styled(entry.change.char(), style.add_modifier(Modifier::BOLD)),
                    Span::raw(" "),
                    Span::raw(entry.path.display().to_string()),
                ]))
            })
            .collect();

        let unstaged_border = if self.focus == StagePane::Unstaged {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let unstaged_list = List::new(unstaged_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Unstaged")
                    .border_style(unstaged_border),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        StatefulWidget::render(unstaged_list, panes[0], buf, &mut self.unstaged_state);

        // Render staged pane
        let staged_items: Vec<ListItem> = self
            .staged
            .iter()
            .map(|entry| {
                let style = Style::default().fg(entry.change.color());
                let marker = if entry.is_selected { "[x] " } else { "[ ] " };
                ListItem::new(Line::from(vec![
                    Span::styled(marker, style),
                    Span::styled(entry.change.char(), style.add_modifier(Modifier::BOLD)),
                    Span::raw(" "),
                    Span::raw(entry.path.display().to_string()),
                ]))
            })
            .collect();

        let staged_border = if self.focus == StagePane::Staged {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };

        let staged_list = List::new(staged_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Staged")
                    .border_style(staged_border),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        StatefulWidget::render(staged_list, panes[1], buf, &mut self.staged_state);

        // Render footer with help text and message
        let footer_text = if let Some(msg) = &self.message {
            msg.clone()
        } else {
            String::from(
                "Space: Stage/Unstage | a: Stage All | u: Unstage All | Tab: Switch Pane | v: Visual Mode | j/k: Navigate",
            )
        };
        let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
        footer.render(chunks[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;
    use tokio::fs;
    use tokio::process::Command;

    async fn init_git_repo(path: &Path) {
        Command::new("git")
            .current_dir(path)
            .args(["init"])
            .output()
            .await
            .unwrap();

        Command::new("git")
            .current_dir(path)
            .args(["config", "user.email", "test@example.com"])
            .output()
            .await
            .unwrap();

        Command::new("git")
            .current_dir(path)
            .args(["config", "user.name", "Test User"])
            .output()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_git_stage_ui_new() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let stage_ui = GitStageUI::new(service);

        assert_eq!(stage_ui.unstaged_count(), 0);
        assert_eq!(stage_ui.staged_count(), 0);
        assert_eq!(stage_ui.focus(), StagePane::Unstaged);
    }

    #[tokio::test]
    async fn test_git_stage_ui_refresh() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").await.unwrap();

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut stage_ui = GitStageUI::new(service);
        stage_ui.refresh().await.unwrap();

        assert_eq!(stage_ui.unstaged_count(), 1);
        assert_eq!(stage_ui.staged_count(), 0);
    }

    #[tokio::test]
    async fn test_git_stage_ui_stage_file() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create initial commit
        let initial_file = temp_dir.path().join("initial.txt");
        fs::write(&initial_file, "initial").await.unwrap();
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "initial.txt"])
            .output()
            .await
            .unwrap();
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["commit", "-m", "initial"])
            .output()
            .await
            .unwrap();

        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").await.unwrap();

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut stage_ui = GitStageUI::new(service);
        stage_ui.refresh().await.unwrap();

        assert_eq!(stage_ui.unstaged_count(), 1);

        // Stage the file
        stage_ui.stage_selected().await.unwrap();

        assert_eq!(stage_ui.unstaged_count(), 0);
        assert_eq!(stage_ui.staged_count(), 1);
    }

    #[tokio::test]
    async fn test_git_stage_ui_navigation() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create multiple test files
        for i in 0..5 {
            let test_file = temp_dir.path().join(format!("test{}.txt", i));
            fs::write(&test_file, "content").await.unwrap();
        }

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut stage_ui = GitStageUI::new(service);
        stage_ui.refresh().await.unwrap();

        // Test navigation
        stage_ui.move_down();
        assert_eq!(stage_ui.unstaged_state.selected(), Some(1));

        stage_ui.move_up();
        assert_eq!(stage_ui.unstaged_state.selected(), Some(0));

        stage_ui.jump_to_bottom();
        assert_eq!(stage_ui.unstaged_state.selected(), Some(4));

        stage_ui.jump_to_top();
        assert_eq!(stage_ui.unstaged_state.selected(), Some(0));
    }

    #[tokio::test]
    async fn test_git_stage_ui_focus_toggle() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut stage_ui = GitStageUI::new(service);

        assert_eq!(stage_ui.focus(), StagePane::Unstaged);

        stage_ui.toggle_focus();
        assert_eq!(stage_ui.focus(), StagePane::Staged);

        stage_ui.toggle_focus();
        assert_eq!(stage_ui.focus(), StagePane::Unstaged);
    }

    #[tokio::test]
    async fn test_git_stage_ui_stage_all() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create initial commit
        let initial_file = temp_dir.path().join("initial.txt");
        fs::write(&initial_file, "initial").await.unwrap();
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "initial.txt"])
            .output()
            .await
            .unwrap();
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["commit", "-m", "initial"])
            .output()
            .await
            .unwrap();

        // Create multiple test files
        for i in 0..3 {
            let test_file = temp_dir.path().join(format!("test{}.txt", i));
            fs::write(&test_file, "content").await.unwrap();
        }

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut stage_ui = GitStageUI::new(service);
        stage_ui.refresh().await.unwrap();

        assert_eq!(stage_ui.unstaged_count(), 3);

        // Stage all files
        stage_ui.stage_all().await.unwrap();

        assert_eq!(stage_ui.unstaged_count(), 0);
        assert_eq!(stage_ui.staged_count(), 3);
    }

    #[tokio::test]
    async fn test_file_change_type_conversion() {
        let modified = FileChange::Modified(PathBuf::from("test.txt"));
        let change_type = FileChangeType::from(&modified);
        assert_eq!(change_type, FileChangeType::Modified);
        assert_eq!(change_type.char(), "M");
        assert_eq!(change_type.color(), Color::Yellow);
    }
}
