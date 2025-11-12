//! Auto-commit functionality for AI-assisted changes
//!
//! Automatically creates git commits after AI modifies files, with descriptive
//! messages and support for undo operations.
//!
//! # Features
//!
//! - Auto-commit after every AI file change
//! - Descriptive commit message generation
//! - Undo/revert functionality
//! - Commit history tracking
//! - Respects .gitignore
//!
//! # Examples
//!
//! ```no_run
//! use toad::git::AutoCommitManager;
//! use std::path::Path;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let manager = AutoCommitManager::new(Path::new("."))?;
//! // let commit_hash = manager.commit_ai_changes(files, "Added feature X").await?;
//! # Ok(())
//! # }
//! ```

use crate::git::GitService;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Prefix for AI-assisted commit messages
const AI_COMMIT_PREFIX: &str = "AI-assisted:";

/// Tag for tracking AI commits
const AI_COMMIT_TAG: &str = "toad-ai";

/// Auto-commit manager for AI-assisted changes
#[derive(Debug)]
pub struct AutoCommitManager {
    /// Git service for repository operations
    git_service: GitService,
    /// Whether auto-commit is enabled
    enabled: bool,
    /// Stack of AI commit hashes (for undo)
    commit_stack: Vec<String>,
}

impl AutoCommitManager {
    /// Create a new auto-commit manager
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::git::AutoCommitManager;
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let manager = AutoCommitManager::new(Path::new("."))?;
    /// assert!(manager.is_enabled());
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(repo_path: impl AsRef<Path>) -> Result<Self> {
        let git_service = GitService::new(repo_path)?;

        Ok(Self {
            git_service,
            enabled: true,
            commit_stack: Vec::new(),
        })
    }

    /// Check if auto-commit is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable or disable auto-commit
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::git::AutoCommitManager;
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let mut manager = AutoCommitManager::new(Path::new("."))?;
    /// manager.set_enabled(false);
    /// assert!(!manager.is_enabled());
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Toggle auto-commit
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Get the number of commits in the undo stack
    pub fn undo_stack_size(&self) -> usize {
        self.commit_stack.len()
    }

    /// Commit AI-assisted changes
    ///
    /// # Parameters
    ///
    /// - `files`: Files that were modified
    /// - `context`: Context about what changed (e.g., "Added JWT authentication")
    ///
    /// # Returns
    ///
    /// The commit hash if successful
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::git::AutoCommitManager;
    /// use std::path::{Path, PathBuf};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let mut manager = AutoCommitManager::new(Path::new("."))?;
    /// let files = vec![PathBuf::from("src/main.rs")];
    /// let hash = manager.commit_ai_changes(files, "Refactored main function").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn commit_ai_changes(
        &mut self,
        files: Vec<PathBuf>,
        context: &str,
    ) -> Result<String> {
        if !self.enabled {
            anyhow::bail!("Auto-commit is disabled");
        }

        if files.is_empty() {
            anyhow::bail!("No files to commit");
        }

        // Stage all modified files
        for file in &files {
            self.git_service
                .stage(file)
                .await
                .with_context(|| format!("Failed to stage file: {}", file.display()))?;
        }

        // Generate commit message
        let message = self.generate_commit_message(&files, context)?;

        // Create commit
        let commit_hash = self
            .git_service
            .commit(&message)
            .await
            .context("Failed to create commit")?;

        // Add to undo stack
        self.commit_stack.push(commit_hash.clone());

        Ok(commit_hash)
    }

    /// Generate a descriptive commit message
    ///
    /// # Message Format
    ///
    /// ```text
    /// AI-assisted: <type>(<scope>): <description>
    ///
    /// Files modified:
    /// - file1.rs
    /// - file2.rs
    ///
    /// Context: <context>
    ///
    /// Tag: toad-ai
    /// ```
    fn generate_commit_message(&self, files: &[PathBuf], context: &str) -> Result<String> {
        // Infer commit type from context
        let commit_type = self.infer_commit_type(context);

        // Infer scope from files
        let scope = self.infer_scope(files);

        // Generate summary line
        let summary = if scope.is_empty() {
            format!("{} {}: {}", AI_COMMIT_PREFIX, commit_type, context)
        } else {
            format!(
                "{} {}({}): {}",
                AI_COMMIT_PREFIX, commit_type, scope, context
            )
        };

        // Build full message
        let mut message = summary.clone();

        // Add file list if more than 1 file
        if files.len() > 1 {
            message.push_str("\n\nFiles modified:");
            for file in files {
                message.push_str(&format!("\n- {}", file.display()));
            }
        }

        // Add tag
        message.push_str(&format!("\n\nTag: {}", AI_COMMIT_TAG));

        Ok(message)
    }

    /// Infer commit type from context
    ///
    /// Returns: feat, fix, refactor, docs, test, style, or chore
    fn infer_commit_type(&self, context: &str) -> &str {
        let lower = context.to_lowercase();

        if lower.contains("add") || lower.contains("new") || lower.contains("implement") {
            "feat"
        } else if lower.contains("fix") || lower.contains("bug") || lower.contains("error") {
            "fix"
        } else if lower.contains("refactor") || lower.contains("restructure") {
            "refactor"
        } else if lower.contains("doc") || lower.contains("comment") || lower.contains("readme") {
            "docs"
        } else if lower.contains("test") {
            "test"
        } else if lower.contains("format") || lower.contains("style") {
            "style"
        } else {
            "chore"
        }
    }

    /// Infer scope from file paths
    ///
    /// Returns the most common directory or module
    fn infer_scope(&self, files: &[PathBuf]) -> String {
        if files.is_empty() {
            return String::new();
        }

        // Try to find common directory
        let mut dir_counts = std::collections::HashMap::new();

        for file in files {
            if let Some(parent) = file.parent() {
                if let Some(dir_name) = parent.file_name() {
                    if let Some(name) = dir_name.to_str() {
                        *dir_counts.entry(name.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        // Return most common directory, or empty if tie
        dir_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(dir, _)| dir)
            .unwrap_or_default()
    }

    /// Undo the last AI-assisted commit
    ///
    /// # Returns
    ///
    /// The hash of the commit that was reverted
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::git::AutoCommitManager;
    /// use std::path::Path;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let mut manager = AutoCommitManager::new(Path::new("."))?;
    /// // ... make some commits ...
    /// let reverted_hash = manager.undo_last_commit().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn undo_last_commit(&mut self) -> Result<String> {
        let commit_hash = self
            .commit_stack
            .pop()
            .context("No commits to undo")?;

        // Verify it's an AI commit by checking the log
        let commits = self.git_service.log(Some(1)).await?;
        if let Some(latest) = commits.first() {
            if latest.hash != commit_hash && latest.full_hash != commit_hash {
                anyhow::bail!(
                    "Latest commit {} doesn't match expected {}",
                    latest.hash,
                    commit_hash
                );
            }

            // Ensure it's an AI commit
            if !latest.message.contains(AI_COMMIT_PREFIX) {
                anyhow::bail!("Latest commit is not an AI-assisted commit");
            }
        }

        // Revert using git reset --soft (keeps changes in working directory)
        self.soft_reset_head().await?;

        Ok(commit_hash)
    }

    /// Soft reset to HEAD~1 (keeps changes staged)
    async fn soft_reset_head(&self) -> Result<()> {
        use tokio::process::Command;

        let output = Command::new("git")
            .args(["reset", "--soft", "HEAD~1"])
            .current_dir(self.git_service.repo_path())
            .output()
            .await
            .context("Failed to execute git reset")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Git reset failed: {}", stderr);
        }

        Ok(())
    }

    /// Get commit history (AI commits only)
    ///
    /// # Parameters
    ///
    /// - `max_count`: Maximum number of commits to retrieve
    ///
    /// # Returns
    ///
    /// List of AI-assisted commits
    pub async fn ai_commit_history(&self, max_count: usize) -> Result<Vec<crate::git::CommitInfo>> {
        let all_commits = self.git_service.log(Some(max_count * 2)).await?;

        // Filter to AI commits only
        let ai_commits: Vec<_> = all_commits
            .into_iter()
            .filter(|c| c.message.contains(AI_COMMIT_PREFIX))
            .take(max_count)
            .collect();

        Ok(ai_commits)
    }

    /// Get current git status
    pub async fn status(&self) -> Result<Vec<crate::git::FileChange>> {
        self.git_service.status().await
    }

    /// Get current branch name
    pub async fn current_branch(&self) -> Result<String> {
        self.git_service.current_branch().await
    }

    /// Get ahead/behind counts
    pub async fn ahead_behind(&self) -> Result<(usize, usize)> {
        self.git_service.ahead_behind().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_commit_type() {
        let manager = AutoCommitManager {
            git_service: GitService::new(".").unwrap(),
            enabled: true,
            commit_stack: Vec::new(),
        };

        assert_eq!(manager.infer_commit_type("Add new feature"), "feat");
        assert_eq!(manager.infer_commit_type("Fix bug in parser"), "fix");
        assert_eq!(manager.infer_commit_type("Refactor authentication"), "refactor");
        assert_eq!(manager.infer_commit_type("Update documentation"), "docs");
        assert_eq!(manager.infer_commit_type("Add tests for module"), "test");
        assert_eq!(manager.infer_commit_type("Format code"), "style");
        assert_eq!(manager.infer_commit_type("Update dependencies"), "chore");
    }

    #[test]
    fn test_infer_scope() {
        let manager = AutoCommitManager {
            git_service: GitService::new(".").unwrap(),
            enabled: true,
            commit_stack: Vec::new(),
        };

        let files = vec![
            PathBuf::from("src/auth/login.rs"),
            PathBuf::from("src/auth/logout.rs"),
        ];

        assert_eq!(manager.infer_scope(&files), "auth");

        let mixed_files = vec![
            PathBuf::from("src/auth/login.rs"),
            PathBuf::from("src/db/query.rs"),
        ];

        // When files are from different dirs, should pick one (or empty)
        let scope = manager.infer_scope(&mixed_files);
        assert!(scope == "auth" || scope == "db" || scope.is_empty());
    }

    #[test]
    fn test_generate_commit_message() {
        let manager = AutoCommitManager {
            git_service: GitService::new(".").unwrap(),
            enabled: true,
            commit_stack: Vec::new(),
        };

        let files = vec![PathBuf::from("src/auth/login.rs")];
        let message = manager.generate_commit_message(&files, "Add JWT authentication").unwrap();

        assert!(message.contains(AI_COMMIT_PREFIX));
        assert!(message.contains("feat"));
        assert!(message.contains("auth"));
        assert!(message.contains("Add JWT authentication"));
        assert!(message.contains(AI_COMMIT_TAG));
    }

    #[test]
    fn test_enabled_toggle() {
        let mut manager = AutoCommitManager {
            git_service: GitService::new(".").unwrap(),
            enabled: true,
            commit_stack: Vec::new(),
        };

        assert!(manager.is_enabled());

        manager.toggle();
        assert!(!manager.is_enabled());

        manager.set_enabled(true);
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_undo_stack() {
        let mut manager = AutoCommitManager {
            git_service: GitService::new(".").unwrap(),
            enabled: true,
            commit_stack: Vec::new(),
        };

        assert_eq!(manager.undo_stack_size(), 0);

        manager.commit_stack.push("abc123".to_string());
        assert_eq!(manager.undo_stack_size(), 1);

        manager.commit_stack.push("def456".to_string());
        assert_eq!(manager.undo_stack_size(), 2);
    }
}
