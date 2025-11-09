//! Interactive Git branch manager widget
//!
//! Provides a visual interface for managing git branches: creating, switching,
//! deleting, and viewing branch information.
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::GitBranchManager;
//! use toad::git::GitService;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let service = GitService::new(".")?;
//! let mut manager = GitBranchManager::new(service);
//! manager.refresh().await?;
//! # Ok(())
//! # }
//! ```

use crate::git::{BranchInfo, GitService};
use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

/// Branch operation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchMode {
    /// Browsing branches
    Browse,
    /// Creating new branch
    Create,
    /// Deleting branch
    Delete,
    /// Renaming branch
    Rename,
}

/// Git branch manager widget
///
/// Provides an interactive interface for branch management:
/// - View all branches with current branch highlighted
/// - Create new branches
/// - Switch between branches
/// - Delete branches (with safety checks)
/// - View ahead/behind status
///
/// # Keybindings
///
/// - `j/k`: Navigate up/down
/// - `Enter`: Switch to selected branch
/// - `n`: Create new branch
/// - `d`: Delete branch
/// - `r`: Rename branch
/// - `gg/G`: Jump to top/bottom
pub struct GitBranchManager {
    /// Git service for operations
    service: GitService,
    /// List of branches
    branches: Vec<BranchInfo>,
    /// Current branch name
    current_branch: String,
    /// List state for navigation
    list_state: ListState,
    /// Current operation mode
    mode: BranchMode,
    /// New branch name (for create/rename)
    input_buffer: String,
    /// Last operation message
    message: Option<String>,
    /// Error message
    error: Option<String>,
}

impl GitBranchManager {
    /// Create a new git branch manager
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::GitBranchManager;
    /// use toad::git::GitService;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let service = GitService::new(".")?;
    /// let manager = GitBranchManager::new(service);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(service: GitService) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            service,
            branches: Vec::new(),
            current_branch: String::new(),
            list_state,
            mode: BranchMode::Browse,
            input_buffer: String::new(),
            message: None,
            error: None,
        }
    }

    /// Refresh the branch list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::ui::widgets::GitBranchManager;
    /// # use toad::git::GitService;
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// # let service = GitService::new(".")?;
    /// let mut manager = GitBranchManager::new(service);
    /// manager.refresh().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refresh(&mut self) -> Result<()> {
        self.current_branch = self.service.current_branch().await?;
        self.branches = self.service.list_branches().await?;

        // Ensure selection is valid
        if !self.branches.is_empty() && self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }

        // Select current branch by default
        if let Some(pos) = self.branches.iter().position(|b| b.is_current) {
            self.list_state.select(Some(pos));
        }

        Ok(())
    }

    /// Get the current mode
    pub fn mode(&self) -> BranchMode {
        self.mode
    }

    /// Set the mode
    pub fn set_mode(&mut self, mode: BranchMode) {
        self.mode = mode;
        self.input_buffer.clear();
        self.error = None;

        if mode == BranchMode::Create {
            self.message = Some(String::from("Enter new branch name:"));
        } else if mode == BranchMode::Rename {
            self.message = Some(String::from("Enter new branch name:"));
        } else if mode == BranchMode::Delete {
            if let Some(selected) = self.selected_branch() {
                self.message = Some(format!("Delete branch '{}'? (y/n)", selected.name));
            }
        }
    }

    /// Cancel current operation and return to browse mode
    pub fn cancel(&mut self) {
        self.mode = BranchMode::Browse;
        self.input_buffer.clear();
        self.message = None;
        self.error = None;
    }

    /// Get the selected branch
    pub fn selected_branch(&self) -> Option<&BranchInfo> {
        self.list_state.selected().and_then(|i| self.branches.get(i))
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected > 0 {
                self.list_state.select(Some(selected - 1));
            }
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected + 1 < self.branches.len() {
                self.list_state.select(Some(selected + 1));
            }
        }
    }

    /// Jump to top of list
    pub fn jump_to_top(&mut self) {
        if !self.branches.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    /// Jump to bottom of list
    pub fn jump_to_bottom(&mut self) {
        if !self.branches.is_empty() {
            self.list_state.select(Some(self.branches.len() - 1));
        }
    }

    /// Insert character into input buffer
    pub fn insert_char(&mut self, c: char) {
        if matches!(self.mode, BranchMode::Create | BranchMode::Rename) {
            self.input_buffer.push(c);
        }
    }

    /// Delete last character from input buffer
    pub fn backspace(&mut self) {
        if matches!(self.mode, BranchMode::Create | BranchMode::Rename) {
            self.input_buffer.pop();
        }
    }

    /// Get the input buffer
    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    /// Switch to the selected branch
    pub async fn switch_branch(&mut self) -> Result<()> {
        if let Some(branch) = self.selected_branch() {
            if branch.is_current {
                self.message = Some(String::from("Already on this branch"));
                return Ok(());
            }

            // Use git checkout
            let output = tokio::process::Command::new("git")
                .current_dir(self.service.repo_path())
                .args(["checkout", &branch.name])
                .output()
                .await?;

            if output.status.success() {
                self.message = Some(format!("Switched to branch '{}'", branch.name));
                self.refresh().await?;
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                self.error = Some(format!("Failed to switch: {}", stderr));
            }
        }
        Ok(())
    }

    /// Create a new branch
    pub async fn create_branch(&mut self) -> Result<()> {
        if self.input_buffer.trim().is_empty() {
            self.error = Some(String::from("Branch name cannot be empty"));
            return Ok(());
        }

        let branch_name = self.input_buffer.trim();

        // Check if branch already exists
        if self.branches.iter().any(|b| b.name == branch_name) {
            self.error = Some(format!("Branch '{}' already exists", branch_name));
            return Ok(());
        }

        // Create branch
        let output = tokio::process::Command::new("git")
            .current_dir(self.service.repo_path())
            .args(["branch", branch_name])
            .output()
            .await?;

        if output.status.success() {
            self.message = Some(format!("Created branch '{}'", branch_name));
            self.mode = BranchMode::Browse;
            self.input_buffer.clear();
            self.refresh().await?;
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.error = Some(format!("Failed to create: {}", stderr));
        }

        Ok(())
    }

    /// Delete the selected branch
    pub async fn delete_branch(&mut self) -> Result<()> {
        if let Some(branch) = self.selected_branch() {
            if branch.is_current {
                self.error = Some(String::from("Cannot delete current branch"));
                return Ok(());
            }

            let branch_name = branch.name.clone();

            // Delete branch
            let output = tokio::process::Command::new("git")
                .current_dir(self.service.repo_path())
                .args(["branch", "-d", &branch_name])
                .output()
                .await?;

            if output.status.success() {
                self.message = Some(format!("Deleted branch '{}'", branch_name));
                self.mode = BranchMode::Browse;
                self.refresh().await?;
            } else {
                // Try force delete if normal delete failed
                let output = tokio::process::Command::new("git")
                    .current_dir(self.service.repo_path())
                    .args(["branch", "-D", &branch_name])
                    .output()
                    .await?;

                if output.status.success() {
                    self.message = Some(format!("Force deleted branch '{}'", branch_name));
                    self.mode = BranchMode::Browse;
                    self.refresh().await?;
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    self.error = Some(format!("Failed to delete: {}", stderr));
                }
            }
        }
        Ok(())
    }

    /// Rename the selected branch
    pub async fn rename_branch(&mut self) -> Result<()> {
        if self.input_buffer.trim().is_empty() {
            self.error = Some(String::from("Branch name cannot be empty"));
            return Ok(());
        }

        if let Some(branch) = self.selected_branch() {
            let old_name = branch.name.clone();
            let new_name = self.input_buffer.trim();

            // Check if new name already exists
            if self.branches.iter().any(|b| b.name == new_name) {
                self.error = Some(format!("Branch '{}' already exists", new_name));
                return Ok(());
            }

            // Rename branch
            let output = tokio::process::Command::new("git")
                .current_dir(self.service.repo_path())
                .args(["branch", "-m", &old_name, new_name])
                .output()
                .await?;

            if output.status.success() {
                self.message = Some(format!("Renamed '{}' to '{}'", old_name, new_name));
                self.mode = BranchMode::Browse;
                self.input_buffer.clear();
                self.refresh().await?;
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                self.error = Some(format!("Failed to rename: {}", stderr));
            }
        }

        Ok(())
    }

    /// Get the last message
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Get the error message
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Clear messages
    pub fn clear_messages(&mut self) {
        self.message = None;
        self.error = None;
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> &str {
        &self.current_branch
    }

    /// Get the branch count
    pub fn branch_count(&self) -> usize {
        self.branches.len()
    }
}

impl Widget for &mut GitBranchManager {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into header, list, input, footer
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Branch list
            Constraint::Length(3),  // Input/message area
            Constraint::Length(1),  // Footer
        ])
        .split(area);

        // Render header
        let header_text = vec![Line::from(vec![
            Span::styled("Current: ", Style::default().fg(Color::Gray)),
            Span::styled(&self.current_branch, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("  |  ", Style::default().fg(Color::Gray)),
            Span::styled(format!("Branches: {}", self.branches.len()), Style::default().fg(Color::Cyan)),
        ])];

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title("Branch Manager"));
        header.render(chunks[0], buf);

        // Render branch list
        let items: Vec<ListItem> = self
            .branches
            .iter()
            .map(|branch| {
                let marker = if branch.is_current { "* " } else { "  " };
                let style = if branch.is_current {
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let ahead_behind = if branch.ahead > 0 || branch.behind > 0 {
                    format!(" [↑{} ↓{}]", branch.ahead, branch.behind)
                } else {
                    String::new()
                };

                ListItem::new(Line::from(vec![
                    Span::styled(marker, style),
                    Span::styled(&branch.name, style),
                    Span::styled(ahead_behind, Style::default().fg(Color::Gray)),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Branches"))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        StatefulWidget::render(list, chunks[1], buf, &mut self.list_state);

        // Render input/message area
        let input_text = match self.mode {
            BranchMode::Create | BranchMode::Rename => {
                vec![Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Cyan)),
                    Span::raw(&self.input_buffer),
                    Span::styled("█", Style::default().bg(Color::White).fg(Color::Black)),
                ])]
            }
            _ => {
                if let Some(err) = &self.error {
                    vec![Line::from(vec![Span::styled(err, Style::default().fg(Color::Red))])]
                } else if let Some(msg) = &self.message {
                    vec![Line::from(vec![Span::styled(msg, Style::default().fg(Color::Cyan))])]
                } else {
                    vec![Line::from("")]
                }
            }
        };

        let input_widget = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL));
        input_widget.render(chunks[2], buf);

        // Render footer
        let footer_text = match self.mode {
            BranchMode::Browse => "Enter: Switch | n: New | d: Delete | r: Rename | j/k: Navigate | q: Quit",
            BranchMode::Create | BranchMode::Rename => "Enter: Confirm | Esc: Cancel",
            BranchMode::Delete => "y: Confirm | n/Esc: Cancel",
        };

        let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::Gray));
        footer.render(chunks[3], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;
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

        // Create initial commit
        tokio::fs::write(path.join("test.txt"), "content").await.unwrap();
        Command::new("git")
            .current_dir(path)
            .args(["add", "."])
            .output()
            .await
            .unwrap();
        Command::new("git")
            .current_dir(path)
            .args(["commit", "-m", "initial"])
            .output()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_git_branch_manager_new() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let manager = GitBranchManager::new(service);

        assert_eq!(manager.mode(), BranchMode::Browse);
    }

    #[tokio::test]
    async fn test_git_branch_manager_refresh() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut manager = GitBranchManager::new(service);

        manager.refresh().await.unwrap();

        assert!(manager.branch_count() > 0);
        assert!(!manager.current_branch().is_empty());
    }

    #[tokio::test]
    async fn test_git_branch_manager_navigation() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create additional branches
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["branch", "test1"])
            .output()
            .await
            .unwrap();
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["branch", "test2"])
            .output()
            .await
            .unwrap();

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut manager = GitBranchManager::new(service);
        manager.refresh().await.unwrap();

        manager.move_down();
        assert_eq!(manager.list_state.selected(), Some(1));

        manager.move_up();
        assert_eq!(manager.list_state.selected(), Some(0));

        manager.jump_to_bottom();
        assert_eq!(manager.list_state.selected(), Some(manager.branch_count() - 1));

        manager.jump_to_top();
        assert_eq!(manager.list_state.selected(), Some(0));
    }

    #[tokio::test]
    async fn test_git_branch_manager_input() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut manager = GitBranchManager::new(service);

        manager.set_mode(BranchMode::Create);
        manager.insert_char('t');
        manager.insert_char('e');
        manager.insert_char('s');
        manager.insert_char('t');

        assert_eq!(manager.input_buffer(), "test");

        manager.backspace();
        assert_eq!(manager.input_buffer(), "tes");
    }

    #[tokio::test]
    async fn test_git_branch_manager_create() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut manager = GitBranchManager::new(service);

        manager.refresh().await.unwrap();
        let initial_count = manager.branch_count();

        manager.set_mode(BranchMode::Create);
        manager.insert_char('n');
        manager.insert_char('e');
        manager.insert_char('w');
        manager.create_branch().await.unwrap();

        assert_eq!(manager.branch_count(), initial_count + 1);
    }
}
