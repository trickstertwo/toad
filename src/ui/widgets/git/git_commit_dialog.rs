//! Interactive Git commit dialog widget
//!
//! Provides a modal dialog for creating git commits with multi-line message
//! editing, commit message preview, and validation.
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::GitCommitDialog;
//! use toad::git::GitService;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let service = GitService::new(".")?;
//! let mut dialog = GitCommitDialog::new(service);
//! dialog.show();
//! # Ok(())
//! # }
//! ```

use crate::{git::GitService, ui::atoms::{block::Block as AtomBlock, text::Text as AtomText}};
use anyhow::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Borders, Clear, Paragraph, Widget, Wrap},
};

/// Git commit dialog state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitDialogState {
    /// Dialog is hidden
    Hidden,
    /// Editing commit message
    Editing,
    /// Committing (async operation in progress)
    Committing,
    /// Commit succeeded
    Success,
    /// Commit failed
    Error,
}

/// Interactive git commit dialog
///
/// Provides a modal interface for creating commits with:
/// - Multi-line commit message editing
/// - Live character count
/// - Commit message guidelines
/// - Staged file summary
/// - Success/error feedback
///
/// # Keybindings
///
/// - `Enter`: Newline in message
/// - `Ctrl+Enter`: Commit
/// - `Esc`: Cancel
pub struct GitCommitDialog {
    /// Git service for operations
    service: GitService,
    /// Dialog state
    state: CommitDialogState,
    /// Commit message buffer
    message: String,
    /// Cursor position in message
    cursor_pos: usize,
    /// Number of staged files
    staged_count: usize,
    /// Last commit result message
    result_message: Option<String>,
    /// Error message if commit failed
    error_message: Option<String>,
}

impl GitCommitDialog {
    /// Create a new git commit dialog
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::GitCommitDialog;
    /// use toad::git::GitService;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let service = GitService::new(".")?;
    /// let dialog = GitCommitDialog::new(service);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(service: GitService) -> Self {
        Self {
            service,
            state: CommitDialogState::Hidden,
            message: String::new(),
            cursor_pos: 0,
            staged_count: 0,
            result_message: None,
            error_message: None,
        }
    }

    /// Show the dialog and refresh staged file count
    pub async fn show(&mut self) -> Result<()> {
        self.state = CommitDialogState::Editing;
        self.message.clear();
        self.cursor_pos = 0;
        self.result_message = None;
        self.error_message = None;

        // Count staged files
        let changes = self.service.status().await?;
        self.staged_count = changes
            .iter()
            .filter(|c| matches!(c, crate::git::FileChange::Staged(_)))
            .count();

        Ok(())
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.state = CommitDialogState::Hidden;
        self.message.clear();
        self.cursor_pos = 0;
        self.result_message = None;
        self.error_message = None;
    }

    /// Check if dialog is visible
    pub fn is_visible(&self) -> bool {
        !matches!(self.state, CommitDialogState::Hidden)
    }

    /// Get the current state
    pub fn state(&self) -> CommitDialogState {
        self.state
    }

    /// Insert character at cursor position
    pub fn insert_char(&mut self, c: char) {
        if self.state == CommitDialogState::Editing {
            self.message.insert(self.cursor_pos, c);
            self.cursor_pos += c.len_utf8();
        }
    }

    /// Delete character before cursor
    pub fn backspace(&mut self) {
        if self.state == CommitDialogState::Editing && self.cursor_pos > 0 {
            let mut chars: Vec<char> = self.message.chars().collect();
            let char_pos = self.message[..self.cursor_pos].chars().count();
            if char_pos > 0 {
                let removed_char = chars.remove(char_pos - 1);
                self.message = chars.into_iter().collect();
                self.cursor_pos -= removed_char.len_utf8();
            }
        }
    }

    /// Delete character at cursor
    pub fn delete(&mut self) {
        if self.state == CommitDialogState::Editing && self.cursor_pos < self.message.len() {
            let mut chars: Vec<char> = self.message.chars().collect();
            let char_pos = self.message[..self.cursor_pos].chars().count();
            if char_pos < chars.len() {
                chars.remove(char_pos);
                self.message = chars.into_iter().collect();
            }
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            let mut new_pos = self.cursor_pos - 1;
            while new_pos > 0 && !self.message.is_char_boundary(new_pos) {
                new_pos -= 1;
            }
            self.cursor_pos = new_pos;
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.message.len() {
            let mut new_pos = self.cursor_pos + 1;
            while new_pos < self.message.len() && !self.message.is_char_boundary(new_pos) {
                new_pos += 1;
            }
            self.cursor_pos = new_pos;
        }
    }

    /// Move cursor to start of line
    pub fn move_cursor_to_line_start(&mut self) {
        if let Some(line_start) = self.message[..self.cursor_pos].rfind('\n') {
            self.cursor_pos = line_start + 1;
        } else {
            self.cursor_pos = 0;
        }
    }

    /// Move cursor to end of line
    pub fn move_cursor_to_line_end(&mut self) {
        if let Some(line_end) = self.message[self.cursor_pos..].find('\n') {
            self.cursor_pos += line_end;
        } else {
            self.cursor_pos = self.message.len();
        }
    }

    /// Insert newline at cursor
    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    /// Get the commit message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Set the commit message
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
        self.cursor_pos = self.message.len();
    }

    /// Validate commit message
    pub fn is_valid(&self) -> bool {
        !self.message.trim().is_empty() && self.staged_count > 0
    }

    /// Get validation error message
    pub fn validation_error(&self) -> Option<&str> {
        if self.message.trim().is_empty() {
            Some("Commit message cannot be empty")
        } else if self.staged_count == 0 {
            Some("No files staged for commit")
        } else {
            None
        }
    }

    /// Commit with the current message
    pub async fn commit(&mut self) -> Result<()> {
        if !self.is_valid() {
            self.state = CommitDialogState::Error;
            self.error_message = self.validation_error().map(String::from);
            return Ok(());
        }

        self.state = CommitDialogState::Committing;

        match self.service.commit(&self.message).await {
            Ok(output) => {
                self.state = CommitDialogState::Success;
                self.result_message = Some(output);
            }
            Err(e) => {
                self.state = CommitDialogState::Error;
                self.error_message = Some(format!("Commit failed: {}", e));
            }
        }

        Ok(())
    }

    /// Get the result message
    pub fn result_message(&self) -> Option<&str> {
        self.result_message.as_deref()
    }

    /// Get the error message
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Get the number of staged files
    pub fn staged_count(&self) -> usize {
        self.staged_count
    }

    /// Get cursor position
    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    /// Calculate cursor line and column for rendering
    fn cursor_line_col(&self) -> (usize, usize) {
        let before_cursor = &self.message[..self.cursor_pos];
        let line = before_cursor.lines().count().saturating_sub(1);
        let col = before_cursor.lines().last().map(|l| l.len()).unwrap_or(0);
        (line, col)
    }
}

impl Widget for &GitCommitDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state == CommitDialogState::Hidden {
            return;
        }

        // Calculate centered area for dialog (80% width, 60% height)
        let dialog_width = (area.width as f32 * 0.8).min(100.0) as u16;
        let dialog_height = (area.height as f32 * 0.6).min(30.0) as u16;

        let dialog_area = Rect {
            x: (area.width.saturating_sub(dialog_width)) / 2 + area.x,
            y: (area.height.saturating_sub(dialog_height)) / 2 + area.y,
            width: dialog_width,
            height: dialog_height,
        };

        // Clear the dialog area
        Clear.render(dialog_area, buf);

        // Split dialog into header, body, footer
        let chunks = Layout::vertical([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Message editor
            Constraint::Length(3), // Guidelines
            Constraint::Length(3), // Footer
        ])
        .split(dialog_area);

        // Render title based on state
        let (title, title_style) = match self.state {
            CommitDialogState::Editing => ("Commit Changes", Style::default().fg(Color::Cyan)),
            CommitDialogState::Committing => ("Committing...", Style::default().fg(Color::Yellow)),
            CommitDialogState::Success => ("Commit Successful", Style::default().fg(Color::Green)),
            CommitDialogState::Error => ("Commit Failed", Style::default().fg(Color::Red)),
            CommitDialogState::Hidden => ("", Style::default()),
        };

        // Header with file count
        let header_text = vec![Line::from(vec![
            AtomText::new("Staged files: ")
                .style(Style::default().fg(Color::Gray))
                .to_span(),
            AtomText::new(format!("{}", self.staged_count))
                .style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .to_span(),
            AtomText::new("  |  ")
                .style(Style::default().fg(Color::Gray))
                .to_span(),
            AtomText::new("Length: ")
                .style(Style::default().fg(Color::Gray))
                .to_span(),
            AtomText::new(format!("{}", self.message.len()))
                .style(Style::default().fg(Color::Cyan))
                .to_span(),
        ])];

        let header = Paragraph::new(header_text)
            .block(
                AtomBlock::new()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_style(title_style.add_modifier(Modifier::BOLD))
                    .to_ratatui(),
            )
            .alignment(Alignment::Center);

        header.render(chunks[0], buf);

        // Message editor area
        let message_text = if self.state == CommitDialogState::Success {
            Text::from(vec![Line::from(vec![AtomText::new(
                self.result_message
                    .as_deref()
                    .unwrap_or("Committed successfully!"),
            )
            .style(Style::default().fg(Color::Green))
            .to_span()])])
        } else if self.state == CommitDialogState::Error {
            Text::from(vec![Line::from(vec![AtomText::new(
                self.error_message.as_deref().unwrap_or("An error occurred"),
            )
            .style(Style::default().fg(Color::Red))
            .to_span()])])
        } else {
            // Render message with cursor indicator
            let mut lines = vec![];
            for (i, line) in self.message.lines().enumerate() {
                let (cursor_line, cursor_col) = self.cursor_line_col();
                if i == cursor_line {
                    // Add cursor to this line
                    let before = &line[..cursor_col.min(line.len())];
                    let cursor_char = line.chars().nth(cursor_col).unwrap_or(' ');
                    let after = &line[cursor_col.min(line.len())..];

                    lines.push(Line::from(vec![
                        AtomText::new(before).to_span(),
                        AtomText::new(cursor_char.to_string())
                            .style(Style::default().bg(Color::White).fg(Color::Black))
                            .to_span(),
                        AtomText::new(after).to_span(),
                    ]));
                } else {
                    lines.push(Line::from(line.to_string()));
                }
            }

            // If message is empty or cursor is at end after newline, show cursor
            let (cursor_line, cursor_col) = self.cursor_line_col();
            if self.message.is_empty() || (cursor_line >= lines.len() && cursor_col == 0) {
                lines.push(Line::from(vec![AtomText::new(" ")
                    .style(Style::default().bg(Color::White).fg(Color::Black))
                    .to_span()]));
            }

            Text::from(lines)
        };

        let message_editor = Paragraph::new(message_text)
            .block(
                AtomBlock::new()
                    .borders(Borders::ALL)
                    .title("Message")
                    .to_ratatui(),
            )
            .wrap(Wrap { trim: false })
            .scroll((0, 0));

        message_editor.render(chunks[1], buf);

        // Guidelines
        let guidelines = vec![Line::from(vec![
            AtomText::new("Tip: ")
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .to_span(),
            AtomText::new("First line is summary (50 chars max), then blank line, then details")
                .to_span(),
        ])];

        let guidelines_widget = Paragraph::new(guidelines)
            .block(AtomBlock::new().borders(Borders::ALL).to_ratatui())
            .wrap(Wrap { trim: false });

        guidelines_widget.render(chunks[2], buf);

        // Footer with keybindings
        let footer_text = if self.state == CommitDialogState::Editing {
            if let Some(err) = self.validation_error() {
                vec![Line::from(vec![AtomText::new(err)
                    .style(Style::default().fg(Color::Red))
                    .to_span()])]
            } else {
                vec![Line::from(vec![
                    AtomText::new("Ctrl+Enter: Commit | ").to_span(),
                    AtomText::new("Enter: New line | ").to_span(),
                    AtomText::new("Esc: Cancel").to_span(),
                ])]
            }
        } else {
            vec![Line::from(vec![AtomText::new("Press Esc to close").to_span()])]
        };

        let footer = Paragraph::new(footer_text)
            .block(AtomBlock::new().borders(Borders::ALL).to_ratatui())
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);

        footer.render(chunks[3], buf);
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
    async fn test_git_commit_dialog_new() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let dialog = GitCommitDialog::new(service);

        assert_eq!(dialog.state(), CommitDialogState::Hidden);
        assert!(!dialog.is_visible());
    }

    #[tokio::test]
    async fn test_git_commit_dialog_show() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut dialog = GitCommitDialog::new(service);

        dialog.show().await.unwrap();

        assert_eq!(dialog.state(), CommitDialogState::Editing);
        assert!(dialog.is_visible());
    }

    #[tokio::test]
    async fn test_git_commit_dialog_hide() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut dialog = GitCommitDialog::new(service);

        dialog.show().await.unwrap();
        dialog.hide();

        assert_eq!(dialog.state(), CommitDialogState::Hidden);
        assert!(!dialog.is_visible());
    }

    #[tokio::test]
    async fn test_git_commit_dialog_input() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut dialog = GitCommitDialog::new(service);

        dialog.show().await.unwrap();

        dialog.insert_char('H');
        dialog.insert_char('i');
        assert_eq!(dialog.message(), "Hi");

        dialog.backspace();
        assert_eq!(dialog.message(), "H");

        dialog.insert_char('e');
        dialog.insert_char('l');
        dialog.insert_char('l');
        dialog.insert_char('o');
        assert_eq!(dialog.message(), "Hello");
    }

    #[tokio::test]
    async fn test_git_commit_dialog_cursor_movement() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut dialog = GitCommitDialog::new(service);

        dialog.show().await.unwrap();
        dialog.set_message("Hello World");

        assert_eq!(dialog.cursor_pos(), 11);

        dialog.move_cursor_left();
        assert_eq!(dialog.cursor_pos(), 10);

        dialog.move_cursor_to_line_start();
        assert_eq!(dialog.cursor_pos(), 0);

        dialog.move_cursor_to_line_end();
        assert_eq!(dialog.cursor_pos(), 11);
    }

    #[tokio::test]
    async fn test_git_commit_dialog_validation() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut dialog = GitCommitDialog::new(service);

        dialog.show().await.unwrap();

        // Empty message is invalid
        assert!(!dialog.is_valid());
        assert!(dialog.validation_error().is_some());

        dialog.set_message("Test commit");

        // No staged files is invalid
        assert!(!dialog.is_valid());
    }

    #[tokio::test]
    async fn test_git_commit_dialog_commit() {
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

        // Create and stage a file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").await.unwrap();
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "test.txt"])
            .output()
            .await
            .unwrap();

        let service = GitService::new(temp_dir.path()).unwrap();
        let mut dialog = GitCommitDialog::new(service);

        dialog.show().await.unwrap();
        dialog.set_message("Test commit message");

        dialog.commit().await.unwrap();

        assert_eq!(dialog.state(), CommitDialogState::Success);
        assert!(dialog.result_message().is_some());
    }
}
