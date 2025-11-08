//! Git service for interacting with repositories
//!
//! Provides async methods for common git operations like status checking,
//! commit history, branch management, and file staging.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

/// Git service for repository operations
#[derive(Debug, Clone)]
pub struct GitService {
    /// Repository root path
    repo_path: PathBuf,
}

/// File change status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChange {
    /// Modified file
    Modified(PathBuf),
    /// Staged file
    Staged(PathBuf),
    /// Untracked file
    Untracked(PathBuf),
    /// Deleted file
    Deleted(PathBuf),
    /// Renamed file (old, new)
    Renamed(PathBuf, PathBuf),
    /// Conflicted file
    Conflicted(PathBuf),
}

impl FileChange {
    /// Get the file path
    pub fn path(&self) -> &Path {
        match self {
            FileChange::Modified(p)
            | FileChange::Staged(p)
            | FileChange::Untracked(p)
            | FileChange::Deleted(p)
            | FileChange::Conflicted(p) => p,
            FileChange::Renamed(_, new) => new,
        }
    }
}

/// Branch information
#[derive(Debug, Clone)]
pub struct BranchInfo {
    /// Branch name
    pub name: String,
    /// Is current branch
    pub is_current: bool,
    /// Commits ahead of remote
    pub ahead: usize,
    /// Commits behind remote
    pub behind: usize,
}

/// Commit information
#[derive(Debug, Clone)]
pub struct CommitInfo {
    /// Commit hash (short)
    pub hash: String,
    /// Full commit hash
    pub full_hash: String,
    /// Commit message
    pub message: String,
    /// Author name
    pub author: String,
    /// Author email
    pub email: String,
    /// Commit timestamp
    pub timestamp: i64,
    /// Parent commit hashes
    pub parents: Vec<String>,
}

impl GitService {
    /// Create a new git service for the given repository
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::git::GitService;
    /// use std::path::Path;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = GitService::new(Path::new("."))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(repo_path: impl AsRef<Path>) -> Result<Self> {
        let repo_path = repo_path.as_ref().to_path_buf();

        // Verify it's a git repository (basic check)
        if !repo_path.join(".git").exists() {
            anyhow::bail!("Not a git repository: {}", repo_path.display());
        }

        Ok(Self { repo_path })
    }

    /// Get the repository path
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Get repository status (file changes)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::git::GitService;
    /// # use std::path::Path;
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = GitService::new(Path::new("."))?;
    /// let changes = service.status().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn status(&self) -> Result<Vec<FileChange>> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["status", "--porcelain", "--untracked-files=all"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git status")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git status failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut changes = Vec::new();

        for line in stdout.lines() {
            if line.len() < 4 {
                continue;
            }

            let index_status = &line[0..1];
            let work_status = &line[1..2];
            let path = &line[3..];

            // Parse status codes
            let change = match (index_status, work_status) {
                ("M", " ") => FileChange::Staged(PathBuf::from(path)),
                ("A", " ") => FileChange::Staged(PathBuf::from(path)),
                (" ", "M") => FileChange::Modified(PathBuf::from(path)),
                ("M", "M") => FileChange::Modified(PathBuf::from(path)),
                ("?", "?") => FileChange::Untracked(PathBuf::from(path)),
                ("D", " ") | (" ", "D") => FileChange::Deleted(PathBuf::from(path)),
                ("U", "U") | ("A", "A") | ("D", "D") => {
                    FileChange::Conflicted(PathBuf::from(path))
                }
                ("R", " ") => {
                    // Renamed files are shown as "old -> new"
                    if let Some((old, new)) = path.split_once(" -> ") {
                        FileChange::Renamed(PathBuf::from(old), PathBuf::from(new))
                    } else {
                        FileChange::Modified(PathBuf::from(path))
                    }
                }
                _ => FileChange::Modified(PathBuf::from(path)),
            };

            changes.push(change);
        }

        Ok(changes)
    }

    /// Get current branch name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use toad::git::GitService;
    /// # use std::path::Path;
    /// #
    /// # async fn example() -> anyhow::Result<()> {
    /// let service = GitService::new(Path::new("."))?;
    /// let branch = service.current_branch().await?;
    /// println!("Current branch: {}", branch);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn current_branch(&self) -> Result<String> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["branch", "--show-current"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git branch")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git branch failed: {}", stderr);
        }

        let branch = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();

        Ok(branch)
    }

    /// Get commits ahead/behind remote
    ///
    /// Returns (ahead, behind) counts
    pub async fn ahead_behind(&self) -> Result<(usize, usize)> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git rev-list")?;

        if !output.status.success() {
            // No upstream configured, return (0, 0)
            return Ok((0, 0));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = stdout.trim().split_whitespace().collect();

        if parts.len() != 2 {
            return Ok((0, 0));
        }

        let ahead = parts[0].parse().unwrap_or(0);
        let behind = parts[1].parse().unwrap_or(0);

        Ok((ahead, behind))
    }

    /// Get list of branches
    pub async fn list_branches(&self) -> Result<Vec<BranchInfo>> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["branch", "-vv"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git branch")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git branch failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut branches = Vec::new();

        for line in stdout.lines() {
            let is_current = line.starts_with('*');
            let line = line.trim_start_matches('*').trim();

            if let Some(name) = line.split_whitespace().next() {
                branches.push(BranchInfo {
                    name: name.to_string(),
                    is_current,
                    ahead: 0,
                    behind: 0,
                });
            }
        }

        Ok(branches)
    }

    /// Get commit history
    ///
    /// # Arguments
    ///
    /// * `max_count` - Maximum number of commits to fetch (None for all)
    pub async fn log(&self, max_count: Option<usize>) -> Result<Vec<CommitInfo>> {
        let mut args = vec!["log", "--pretty=format:%H|%h|%s|%an|%ae|%ct|%P"];

        let max_str;
        if let Some(max) = max_count {
            max_str = format!("-{}", max);
            args.push(&max_str);
        }

        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git log")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git log failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut commits = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 6 {
                continue;
            }

            let parents = if parts.len() > 6 && !parts[6].is_empty() {
                parts[6].split_whitespace().map(|s| s.to_string()).collect()
            } else {
                Vec::new()
            };

            commits.push(CommitInfo {
                full_hash: parts[0].to_string(),
                hash: parts[1].to_string(),
                message: parts[2].to_string(),
                author: parts[3].to_string(),
                email: parts[4].to_string(),
                timestamp: parts[5].parse().unwrap_or(0),
                parents,
            });
        }

        Ok(commits)
    }

    /// Stage a file
    pub async fn stage(&self, path: impl AsRef<Path>) -> Result<()> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["add", path.as_ref().to_str().unwrap_or("")])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git add")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git add failed: {}", stderr);
        }

        Ok(())
    }

    /// Unstage a file
    pub async fn unstage(&self, path: impl AsRef<Path>) -> Result<()> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["reset", "HEAD", path.as_ref().to_str().unwrap_or("")])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git reset")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git reset failed: {}", stderr);
        }

        Ok(())
    }

    /// Create a commit
    pub async fn commit(&self, message: &str) -> Result<String> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&["commit", "-m", message])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git commit")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git commit failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    /// Get diff for a file
    pub async fn diff(&self, path: Option<&Path>, staged: bool) -> Result<String> {
        let mut args = vec!["diff"];

        if staged {
            args.push("--cached");
        }

        if let Some(p) = path {
            args.push(p.to_str().unwrap_or(""));
        }

        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to run git diff")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("git diff failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    async fn init_git_repo(path: &Path) {
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
    }

    #[tokio::test]
    async fn test_git_service_new() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path());
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_git_service_new_not_repo() {
        let temp_dir = TempDir::new().unwrap();
        let service = GitService::new(temp_dir.path());
        assert!(service.is_err());
    }

    #[tokio::test]
    async fn test_git_service_status_empty() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let status = service.status().await.unwrap();
        assert_eq!(status.len(), 0);
    }

    #[tokio::test]
    async fn test_git_service_status_with_changes() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").await.unwrap();

        let service = GitService::new(temp_dir.path()).unwrap();
        let status = service.status().await.unwrap();
        assert_eq!(status.len(), 1);
        assert!(matches!(status[0], FileChange::Untracked(_)));
    }

    #[tokio::test]
    async fn test_git_service_current_branch() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let service = GitService::new(temp_dir.path()).unwrap();
        let branch = service.current_branch().await.unwrap();
        // New repos might have "main" or "master"
        assert!(!branch.is_empty());
    }

    #[tokio::test]
    async fn test_git_service_stage_unstage() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "content").await.unwrap();

        let service = GitService::new(temp_dir.path()).unwrap();

        // Stage file
        service.stage(&test_file).await.unwrap();
        let status = service.status().await.unwrap();
        assert_eq!(status.len(), 1);
        assert!(matches!(status[0], FileChange::Staged(_)));

        // Unstage file
        service.unstage(&test_file).await.unwrap();
        let status = service.status().await.unwrap();
        assert_eq!(status.len(), 1);
        assert!(matches!(status[0], FileChange::Untracked(_)));
    }
}
