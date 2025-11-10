//! Git graph widget for branch visualization
//!
//! Displays commit history as a visual graph showing branches, merges,
//! and commit relationships. Inspired by lazygit's graph visualization.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{GitGraph, GitCommit};
//! use ratatui::style::Color;
//!
//! let mut graph = GitGraph::new();
//!
//! graph.add_commit(GitCommit::new("abc123", "Initial commit")
//!     .with_branch("main")
//!     .with_author("Alice"));
//!
//! graph.add_commit(GitCommit::new("def456", "Add feature")
//!     .with_branch("main")
//!     .with_parent("abc123"));
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};
use std::collections::HashMap;

/// A single commit in the graph
///
/// # Examples
///
/// ```
/// use toad::widgets::GitCommit;
///
/// let commit = GitCommit::new("a1b2c3", "Fix bug")
///     .with_author("Bob")
///     .with_branch("develop");
///
/// assert_eq!(commit.hash(), "a1b2c3");
/// assert_eq!(commit.message(), "Fix bug");
/// ```
#[derive(Debug, Clone)]
pub struct GitCommit {
    /// Commit hash (short)
    pub(super) hash: String,
    /// Commit message
    pub(super) message: String,
    /// Author name
    pub(super) author: Option<String>,
    /// Branch name
    pub(super) branch: Option<String>,
    /// Parent commit hash
    pub(super) parent: Option<String>,
    /// Merge parent hashes
    pub(super) merge_parents: Vec<String>,
    /// Commit color
    pub(super) color: Color,
}

impl GitCommit {
    /// Create a new commit
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    ///
    /// let commit = GitCommit::new("abc123", "Initial commit");
    /// assert_eq!(commit.hash(), "abc123");
    /// ```
    pub fn new(hash: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            hash: hash.into(),
            message: message.into(),
            author: None,
            branch: None,
            parent: None,
            merge_parents: Vec::new(),
            color: Color::Cyan,
        }
    }

    /// Set author name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    ///
    /// let commit = GitCommit::new("abc", "msg").with_author("Alice");
    /// assert_eq!(commit.author(), Some("Alice"));
    /// ```
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set branch name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    ///
    /// let commit = GitCommit::new("abc", "msg").with_branch("main");
    /// assert_eq!(commit.branch(), Some("main"));
    /// ```
    pub fn with_branch(mut self, branch: impl Into<String>) -> Self {
        self.branch = Some(branch.into());
        self
    }

    /// Set parent commit
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    ///
    /// let commit = GitCommit::new("def", "msg").with_parent("abc");
    /// assert_eq!(commit.parent(), Some("abc"));
    /// ```
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Add merge parent
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    ///
    /// let commit = GitCommit::new("ghi", "Merge")
    ///     .with_parent("def")
    ///     .with_merge_parent("abc");
    ///
    /// assert!(commit.is_merge());
    /// ```
    pub fn with_merge_parent(mut self, parent: impl Into<String>) -> Self {
        self.merge_parents.push(parent.into());
        self
    }

    /// Set commit color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    /// use ratatui::style::Color;
    ///
    /// let commit = GitCommit::new("abc", "msg").with_color(Color::Green);
    /// ```
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Get commit hash
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Get commit message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get author name
    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    /// Get branch name
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }

    /// Get parent commit
    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    /// Check if this is a merge commit
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitCommit;
    ///
    /// let regular = GitCommit::new("abc", "Regular");
    /// assert!(!regular.is_merge());
    ///
    /// let merge = GitCommit::new("def", "Merge").with_merge_parent("abc");
    /// assert!(merge.is_merge());
    /// ```
    pub fn is_merge(&self) -> bool {
        !self.merge_parents.is_empty()
    }
}

/// Git graph widget
///
/// Visualizes commit history as a graph with branches and merges.
///
/// # Examples
///
/// ```
/// use toad::widgets::{GitGraph, GitCommit};
///
/// let mut graph = GitGraph::new();
///
/// graph.add_commit(GitCommit::new("c3", "Third")
///     .with_parent("c2")
///     .with_branch("main"));
///
/// graph.add_commit(GitCommit::new("c2", "Second")
///     .with_parent("c1"));
///
/// graph.add_commit(GitCommit::new("c1", "First"));
///
/// assert_eq!(graph.commit_count(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct GitGraph {
    /// Commits in chronological order
    pub(super) commits: Vec<GitCommit>,
    /// Graph title
    pub(super) title: Option<String>,
    /// Show author names
    pub(super) show_authors: bool,
    /// Show branch names
    pub(super) show_branches: bool,
    /// Compact mode (less spacing)
    pub(super) compact: bool,
    /// Maximum commits to display
    pub(super) max_commits: Option<usize>,
}

impl Default for GitGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl GitGraph {
    /// Create a new git graph
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitGraph;
    ///
    /// let graph = GitGraph::new();
    /// assert_eq!(graph.commit_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            commits: Vec::new(),
            title: None,
            show_authors: true,
            show_branches: true,
            compact: false,
            max_commits: None,
        }
    }

    /// Set graph title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitGraph;
    ///
    /// let graph = GitGraph::new().with_title("Commit History");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Show or hide author names
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitGraph;
    ///
    /// let graph = GitGraph::new().with_authors(false);
    /// ```
    pub fn with_authors(mut self, show: bool) -> Self {
        self.show_authors = show;
        self
    }

    /// Show or hide branch names
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitGraph;
    ///
    /// let graph = GitGraph::new().with_branches(false);
    /// ```
    pub fn with_branches(mut self, show: bool) -> Self {
        self.show_branches = show;
        self
    }

    /// Enable compact mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::GitGraph;
    ///
    /// let graph = GitGraph::new().with_compact(true);
    /// ```
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Set maximum commits to display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{GitGraph, GitCommit};
    ///
    /// let mut graph = GitGraph::new().with_max_commits(2);
    ///
    /// for i in 0..5 {
    ///     graph.add_commit(GitCommit::new(format!("c{}", i), format!("Commit {}", i)));
    /// }
    ///
    /// // Only last 2 commits will be displayed
    /// ```
    pub fn with_max_commits(mut self, max: usize) -> Self {
        self.max_commits = Some(max);
        self
    }

    /// Add a commit to the graph
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{GitGraph, GitCommit};
    ///
    /// let mut graph = GitGraph::new();
    /// graph.add_commit(GitCommit::new("abc", "Initial"));
    /// assert_eq!(graph.commit_count(), 1);
    /// ```
    pub fn add_commit(&mut self, commit: GitCommit) {
        self.commits.push(commit);
    }

    /// Add multiple commits
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{GitGraph, GitCommit};
    ///
    /// let mut graph = GitGraph::new();
    /// graph.add_commits(vec![
    ///     GitCommit::new("c1", "First"),
    ///     GitCommit::new("c2", "Second"),
    /// ]);
    /// assert_eq!(graph.commit_count(), 2);
    /// ```
    pub fn add_commits(&mut self, commits: Vec<GitCommit>) {
        self.commits.extend(commits);
    }

    /// Clear all commits
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{GitGraph, GitCommit};
    ///
    /// let mut graph = GitGraph::new();
    /// graph.add_commit(GitCommit::new("abc", "Test"));
    /// graph.clear();
    /// assert_eq!(graph.commit_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.commits.clear();
    }

    /// Get number of commits
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{GitGraph, GitCommit};
    ///
    /// let mut graph = GitGraph::new();
    /// assert_eq!(graph.commit_count(), 0);
    /// graph.add_commit(GitCommit::new("abc", "Test"));
    /// assert_eq!(graph.commit_count(), 1);
    /// ```
    pub fn commit_count(&self) -> usize {
        self.commits.len()
    }

    /// Render graph lines
    pub(super) fn render_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // Title
        if let Some(ref title) = self.title {
            lines.push(Line::from(Span::styled(
                title.clone(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
        }

        if self.commits.is_empty() {
            lines.push(Line::from("No commits"));
            return lines;
        }

        // Determine which commits to display
        let display_commits: Vec<&GitCommit> = if let Some(max) = self.max_commits {
            self.commits.iter().rev().take(max).collect()
        } else {
            self.commits.iter().rev().collect()
        };

        // Build commit hash to index map
        let hash_to_index: HashMap<&str, usize> = display_commits
            .iter()
            .enumerate()
            .map(|(i, c)| (c.hash(), i))
            .collect();

        // Render each commit
        for (i, commit) in display_commits.iter().enumerate() {
            let graph_char = if commit.is_merge() {
                "◆"
            } else if i == display_commits.len() - 1 {
                "◉"
            } else {
                "●"
            };

            let mut spans = vec![
                Span::styled(graph_char, Style::default().fg(commit.color)),
                Span::raw(" "),
            ];

            // Hash
            spans.push(Span::styled(
                format!("{:7}", commit.hash()),
                Style::default().fg(Color::Yellow),
            ));
            spans.push(Span::raw(" "));

            // Branch name
            if self.show_branches
                && let Some(branch) = commit.branch()
            {
                spans.push(Span::styled(
                    format!("({}) ", branch),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));
            }

            // Message
            let msg_width = width.saturating_sub(20) as usize;
            let message = if commit.message().len() > msg_width {
                format!("{}...", &commit.message()[..msg_width.saturating_sub(3)])
            } else {
                commit.message().to_string()
            };

            spans.push(Span::raw(message));

            // Author
            if self.show_authors
                && let Some(author) = commit.author()
            {
                spans.push(Span::styled(
                    format!(" <{}>", author),
                    Style::default().fg(Color::DarkGray),
                ));
            }

            lines.push(Line::from(spans));

            // Connection line to parent
            if !self.compact
                && i < display_commits.len() - 1
                && let Some(parent) = commit.parent()
                && hash_to_index.contains_key(parent)
            {
                lines.push(Line::from(Span::styled(
                    "│",
                    Style::default().fg(commit.color),
                )));
            }
        }

        lines
    }
}

impl Widget for &GitGraph {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.render_lines(area.width);
        let block = Block::default().borders(Borders::ALL);
        let inner = block.inner(area);

        block.render(area, buf);

        for (i, line) in lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}

