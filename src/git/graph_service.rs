//! Git graph service for integrating GitService with GitGraph widget
//!
//! Provides a bridge between the GitService backend and GitGraph UI widget,
//! fetching real commit history and converting it to the widget format.

use crate::git::{CommitInfo, GitService};
use crate::ui::widgets::{GitCommit, GitGraph};
use anyhow::Result;
use ratatui::style::Color;
use std::collections::HashMap;

/// Service for populating GitGraph with real git data
pub struct GitGraphService {
    git_service: GitService,
}

impl GitGraphService {
    /// Create a new git graph service
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::git::{GitService, GitGraphService};
    /// use std::path::Path;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let git_service = GitService::new(Path::new("."))?;
    /// let graph_service = GitGraphService::new(git_service);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(git_service: GitService) -> Self {
        Self { git_service }
    }

    /// Fetch commit history and populate a GitGraph
    ///
    /// # Arguments
    ///
    /// * `max_commits` - Maximum number of commits to fetch (None for default 100)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::git::{GitService, GitGraphService};
    /// # use std::path::Path;
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let git_service = GitService::new(Path::new("."))?;
    /// let graph_service = GitGraphService::new(git_service);
    /// let graph = graph_service.fetch_graph(Some(50)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_graph(&self, max_commits: Option<usize>) -> Result<GitGraph> {
        // Fetch commits from git service (handle empty repo gracefully)
        let commits = match self.git_service.log(max_commits.or(Some(100))).await {
            Ok(commits) => commits,
            Err(e) => {
                // If git log fails (likely empty repo), return empty graph
                let err_msg = e.to_string().to_lowercase();
                if err_msg.contains("does not have any commits")
                    || err_msg.contains("no commits")
                    || err_msg.contains("bad default revision")
                    || err_msg.contains("unknown revision") {
                    Vec::new()
                } else {
                    return Err(e);
                }
            }
        };

        // Get current branch
        let current_branch = self.git_service.current_branch().await.ok();

        // Create git graph
        let mut graph = GitGraph::new();

        if let Some(branch) = current_branch {
            graph = graph.with_title(format!("Commit History ({})", branch));
        } else {
            graph = graph.with_title("Commit History");
        }

        // Convert commits to GitGraph format
        let git_commits = self.convert_commits(&commits);

        // Add commits to graph
        graph.add_commits(git_commits);

        Ok(graph)
    }

    /// Fetch commit history with branch information
    ///
    /// This enriched version includes branch colors and better visualization
    pub async fn fetch_graph_enriched(
        &self,
        max_commits: Option<usize>,
        show_authors: bool,
        show_branches: bool,
        compact: bool,
    ) -> Result<GitGraph> {
        let mut graph = self.fetch_graph(max_commits).await?;

        graph = graph
            .with_authors(show_authors)
            .with_branches(show_branches)
            .with_compact(compact);

        if let Some(max) = max_commits {
            graph = graph.with_max_commits(max);
        }

        Ok(graph)
    }

    /// Convert CommitInfo to GitCommit with intelligent coloring
    fn convert_commits(&self, commits: &[CommitInfo]) -> Vec<GitCommit> {
        let mut git_commits = Vec::new();
        let mut branch_colors = HashMap::new();
        let mut color_index = 0;

        // Color palette for branches
        let colors = [
            Color::Cyan,
            Color::Green,
            Color::Yellow,
            Color::Magenta,
            Color::Blue,
            Color::Red,
        ];

        for commit in commits {
            let mut git_commit =
                GitCommit::new(commit.hash.clone(), commit.message.clone())
                    .with_author(commit.author.clone());

            // Set parent if exists
            if let Some(parent) = commit.parents.first() {
                git_commit = git_commit.with_parent(parent.clone());
            }

            // Add additional parents for merge commits
            if commit.parents.len() > 1 {
                for parent in &commit.parents[1..] {
                    git_commit = git_commit.with_merge_parent(parent.clone());
                }
            }

            // Assign color based on branch pattern (simplified heuristic)
            let color = if commit.parents.is_empty() {
                // Root commit
                Color::Green
            } else if commit.parents.len() > 1 {
                // Merge commit
                Color::Magenta
            } else {
                // Regular commit - try to infer branch from message or use rotation
                let branch_key = extract_branch_hint(&commit.message);
                *branch_colors.entry(branch_key).or_insert_with(|| {
                    let color = colors[color_index % colors.len()];
                    color_index += 1;
                    color
                })
            };

            git_commit = git_commit.with_color(color);

            git_commits.push(git_commit);
        }

        git_commits
    }
}

/// Extract branch hint from commit message (e.g., "[feature]", "feat:", etc.)
fn extract_branch_hint(message: &str) -> String {
    // Look for bracketed tags first (e.g., "[feature] Add login")
    if let Some(start) = message.find('[') {
        if let Some(end_offset) = message[start + 1..].find(']') {
            return message[start + 1..start + 1 + end_offset].to_lowercase();
        }
    }

    // Look for conventional commit prefixes (e.g., "feat: Add feature")
    if let Some(colon_pos) = message.find(':') {
        let prefix = &message[..colon_pos];
        if prefix.len() < 20 && prefix.len() > 0 {
            return prefix.to_lowercase();
        }
    }

    // Default to "main" for unclassified commits
    "main".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;
    use tokio::process::Command;

    async fn init_git_repo(path: &std::path::Path) {
        Command::new("git")
            .current_dir(path)
            .args(&["init"])
            .output()
            .await
            .unwrap();

        Command::new("git")
            .current_dir(path)
            .args(&["config", "user.email", "test@example.com"])
            .output()
            .await
            .unwrap();

        Command::new("git")
            .current_dir(path)
            .args(&["config", "user.name", "Test User"])
            .output()
            .await
            .unwrap();

        // Disable GPG signing for tests
        Command::new("git")
            .current_dir(path)
            .args(&["config", "commit.gpgsign", "false"])
            .output()
            .await
            .unwrap();
    }

    async fn create_commit(path: &std::path::Path, file: &str, message: &str) {
        let file_path = path.join(file);
        fs::write(&file_path, format!("content of {}", file))
            .await
            .unwrap();

        let add_output = Command::new("git")
            .current_dir(path)
            .args(&["add", file])
            .output()
            .await
            .unwrap();

        assert!(add_output.status.success(), "git add failed: {}", String::from_utf8_lossy(&add_output.stderr));

        let commit_output = Command::new("git")
            .current_dir(path)
            .args(&["commit", "-m", message])
            .output()
            .await
            .unwrap();

        assert!(commit_output.status.success(), "git commit failed: {}", String::from_utf8_lossy(&commit_output.stderr));
    }

    #[tokio::test]
    async fn test_git_graph_service_new() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let git_service = GitService::new(temp_dir.path()).unwrap();
        let _graph_service = GitGraphService::new(git_service);
    }

    #[tokio::test]
    async fn test_fetch_graph_empty_repo() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let git_service = GitService::new(temp_dir.path()).unwrap();
        let graph_service = GitGraphService::new(git_service);

        let graph = graph_service.fetch_graph(None).await.unwrap();
        assert_eq!(graph.commit_count(), 0);
    }

    #[tokio::test]
    async fn test_fetch_graph_with_commits() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create some commits
        create_commit(temp_dir.path(), "file1.txt", "Initial commit").await;
        create_commit(temp_dir.path(), "file2.txt", "Second commit").await;
        create_commit(temp_dir.path(), "file3.txt", "Third commit").await;

        let git_service = GitService::new(temp_dir.path()).unwrap();

        // First verify that git service can read the commits
        let commits = git_service.log(None).await.unwrap();
        assert_eq!(commits.len(), 3, "GitService should find 3 commits");

        let graph_service = GitGraphService::new(git_service);

        let graph = graph_service.fetch_graph(None).await.unwrap();
        assert_eq!(graph.commit_count(), 3);
    }

    #[tokio::test]
    async fn test_fetch_graph_max_commits() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create 5 commits
        for i in 1..=5 {
            create_commit(
                temp_dir.path(),
                &format!("file{}.txt", i),
                &format!("Commit {}", i),
            )
            .await;
        }

        let git_service = GitService::new(temp_dir.path()).unwrap();
        let graph_service = GitGraphService::new(git_service);

        let graph = graph_service.fetch_graph(Some(3)).await.unwrap();
        assert_eq!(graph.commit_count(), 3);
    }

    #[tokio::test]
    async fn test_fetch_graph_enriched() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        create_commit(temp_dir.path(), "file1.txt", "feat: Add feature").await;
        create_commit(temp_dir.path(), "file2.txt", "fix: Fix bug").await;

        let git_service = GitService::new(temp_dir.path()).unwrap();
        let graph_service = GitGraphService::new(git_service);

        let graph = graph_service
            .fetch_graph_enriched(Some(10), true, true, false)
            .await
            .unwrap();

        assert_eq!(graph.commit_count(), 2);
    }

    #[test]
    fn test_extract_branch_hint() {
        assert_eq!(extract_branch_hint("feat: Add feature"), "feat");
        assert_eq!(extract_branch_hint("fix: Fix bug"), "fix");
        assert_eq!(extract_branch_hint("[feature] Add login"), "feature");
        assert_eq!(extract_branch_hint("[FEATURE] Add login"), "feature");
        assert_eq!(extract_branch_hint("[feat] Add login"), "feat");
        assert_eq!(extract_branch_hint("Regular commit message"), "main");
        assert_eq!(extract_branch_hint("WIP changes"), "main");
    }

    #[tokio::test]
    async fn test_convert_commits() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        create_commit(temp_dir.path(), "file1.txt", "Initial").await;
        create_commit(temp_dir.path(), "file2.txt", "Second").await;

        let git_service = GitService::new(temp_dir.path()).unwrap();
        let graph_service = GitGraphService::new(git_service);

        let commits = graph_service.git_service.log(None).await.unwrap();
        let git_commits = graph_service.convert_commits(&commits);

        assert_eq!(git_commits.len(), commits.len());

        // Check that commits have proper parent relationships
        if git_commits.len() > 1 {
            // Most recent commit should have a parent
            assert!(git_commits[0].parent().is_some());
        }
    }
}
