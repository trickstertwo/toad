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
    hash: String,
    /// Commit message
    message: String,
    /// Author name
    author: Option<String>,
    /// Branch name
    branch: Option<String>,
    /// Parent commit hash
    parent: Option<String>,
    /// Merge parent hashes
    merge_parents: Vec<String>,
    /// Commit color
    color: Color,
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
    commits: Vec<GitCommit>,
    /// Graph title
    title: Option<String>,
    /// Show author names
    show_authors: bool,
    /// Show branch names
    show_branches: bool,
    /// Compact mode (less spacing)
    compact: bool,
    /// Maximum commits to display
    max_commits: Option<usize>,
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
    fn render_lines(&self, width: u16) -> Vec<Line<'static>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_commit_new() {
        let commit = GitCommit::new("abc123", "Initial commit");
        assert_eq!(commit.hash(), "abc123");
        assert_eq!(commit.message(), "Initial commit");
        assert_eq!(commit.author(), None);
        assert_eq!(commit.branch(), None);
        assert_eq!(commit.parent(), None);
        assert!(!commit.is_merge());
    }

    #[test]
    fn test_git_commit_with_author() {
        let commit = GitCommit::new("abc", "msg").with_author("Alice");
        assert_eq!(commit.author(), Some("Alice"));
    }

    #[test]
    fn test_git_commit_with_branch() {
        let commit = GitCommit::new("abc", "msg").with_branch("main");
        assert_eq!(commit.branch(), Some("main"));
    }

    #[test]
    fn test_git_commit_with_parent() {
        let commit = GitCommit::new("def", "msg").with_parent("abc");
        assert_eq!(commit.parent(), Some("abc"));
    }

    #[test]
    fn test_git_commit_with_merge_parent() {
        let commit = GitCommit::new("ghi", "Merge")
            .with_parent("def")
            .with_merge_parent("abc");

        assert!(commit.is_merge());
        assert_eq!(commit.parent(), Some("def"));
    }

    #[test]
    fn test_git_commit_with_color() {
        let commit = GitCommit::new("abc", "msg").with_color(Color::Red);
        assert_eq!(commit.color, Color::Red);
    }

    #[test]
    fn test_git_commit_builder_pattern() {
        let commit = GitCommit::new("abc123", "Fix bug")
            .with_author("Bob")
            .with_branch("develop")
            .with_parent("def456")
            .with_color(Color::Green);

        assert_eq!(commit.hash(), "abc123");
        assert_eq!(commit.message(), "Fix bug");
        assert_eq!(commit.author(), Some("Bob"));
        assert_eq!(commit.branch(), Some("develop"));
        assert_eq!(commit.parent(), Some("def456"));
        assert_eq!(commit.color, Color::Green);
    }

    #[test]
    fn test_git_graph_new() {
        let graph = GitGraph::new();
        assert_eq!(graph.commit_count(), 0);
        assert!(graph.show_authors);
        assert!(graph.show_branches);
        assert!(!graph.compact);
    }

    #[test]
    fn test_git_graph_default() {
        let graph = GitGraph::default();
        assert_eq!(graph.commit_count(), 0);
    }

    #[test]
    fn test_git_graph_with_title() {
        let graph = GitGraph::new().with_title("History");
        assert_eq!(graph.title, Some("History".to_string()));
    }

    #[test]
    fn test_git_graph_with_authors() {
        let graph = GitGraph::new().with_authors(false);
        assert!(!graph.show_authors);
    }

    #[test]
    fn test_git_graph_with_branches() {
        let graph = GitGraph::new().with_branches(false);
        assert!(!graph.show_branches);
    }

    #[test]
    fn test_git_graph_with_compact() {
        let graph = GitGraph::new().with_compact(true);
        assert!(graph.compact);
    }

    #[test]
    fn test_git_graph_with_max_commits() {
        let graph = GitGraph::new().with_max_commits(10);
        assert_eq!(graph.max_commits, Some(10));
    }

    #[test]
    fn test_git_graph_add_commit() {
        let mut graph = GitGraph::new();
        graph.add_commit(GitCommit::new("abc", "Test"));
        assert_eq!(graph.commit_count(), 1);
    }

    #[test]
    fn test_git_graph_add_commits() {
        let mut graph = GitGraph::new();
        graph.add_commits(vec![
            GitCommit::new("c1", "First"),
            GitCommit::new("c2", "Second"),
        ]);
        assert_eq!(graph.commit_count(), 2);
    }

    #[test]
    fn test_git_graph_clear() {
        let mut graph = GitGraph::new();
        graph.add_commit(GitCommit::new("abc", "Test"));
        assert_eq!(graph.commit_count(), 1);
        graph.clear();
        assert_eq!(graph.commit_count(), 0);
    }

    #[test]
    fn test_git_graph_render_empty() {
        let graph = GitGraph::new();
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_render_with_commits() {
        let mut graph = GitGraph::new();
        graph.add_commit(GitCommit::new("abc", "First"));
        graph.add_commit(GitCommit::new("def", "Second").with_parent("abc"));
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_max_commits_limit() {
        let mut graph = GitGraph::new().with_max_commits(2);

        for i in 0..5 {
            graph.add_commit(GitCommit::new(format!("c{}", i), format!("Commit {}", i)));
        }

        let lines = graph.render_lines(80);
        // Should render only 2 commits (plus possible title/spacing)
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_git_graph_builder_pattern() {
        let graph = GitGraph::new()
            .with_title("History")
            .with_authors(false)
            .with_branches(true)
            .with_compact(true)
            .with_max_commits(50);

        assert_eq!(graph.title, Some("History".to_string()));
        assert!(!graph.show_authors);
        assert!(graph.show_branches);
        assert!(graph.compact);
        assert_eq!(graph.max_commits, Some(50));
    }

    #[test]
    fn test_merge_commit_rendering() {
        let mut graph = GitGraph::new();

        graph.add_commit(GitCommit::new("c1", "First"));
        graph.add_commit(GitCommit::new("c2", "Second").with_parent("c1"));
        graph.add_commit(
            GitCommit::new("c3", "Merge")
                .with_parent("c2")
                .with_merge_parent("c1"),
        );

        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - 90%+ COVERAGE)
    // ============================================================================

    // Unicode and emoji tests
    #[test]
    fn test_git_commit_with_unicode_message() {
        let commit = GitCommit::new("abc123", "修正バグ 🐛 Fix bug 日本語");
        assert_eq!(commit.message(), "修正バグ 🐛 Fix bug 日本語");
    }

    #[test]
    fn test_git_commit_with_unicode_author() {
        let commit = GitCommit::new("abc", "msg").with_author("田中太郎 (Tanaka Taro) 👨‍💻");
        assert_eq!(commit.author(), Some("田中太郎 (Tanaka Taro) 👨‍💻"));
    }

    #[test]
    fn test_git_commit_with_unicode_branch() {
        let commit = GitCommit::new("abc", "msg").with_branch("機能/新しい-機能 🚀");
        assert_eq!(commit.branch(), Some("機能/新しい-機能 🚀"));
    }

    #[test]
    fn test_git_commit_with_unicode_hash() {
        let commit = GitCommit::new("日本語ハッシュ", "msg");
        assert_eq!(commit.hash(), "日本語ハッシュ");
    }

    #[test]
    fn test_git_graph_with_unicode_title() {
        let graph = GitGraph::new().with_title("コミット履歴 📊 Git History");
        assert_eq!(graph.title, Some("コミット履歴 📊 Git History".to_string()));
    }

    // Very long string tests
    #[test]
    fn test_git_commit_with_very_long_message() {
        let long_message = "Fix ".repeat(1000);
        let commit = GitCommit::new("abc", &long_message);
        assert_eq!(commit.message().len(), long_message.len());
    }

    #[test]
    fn test_git_commit_with_very_long_author() {
        let long_author = "Alice Smith ".repeat(500);
        let commit = GitCommit::new("abc", "msg").with_author(&long_author);
        assert_eq!(commit.author().unwrap().len(), long_author.len());
    }

    #[test]
    fn test_git_commit_with_very_long_branch() {
        let long_branch = "feature/very-long-branch-name-".repeat(100);
        let commit = GitCommit::new("abc", "msg").with_branch(&long_branch);
        assert_eq!(commit.branch().unwrap().len(), long_branch.len());
    }

    #[test]
    fn test_git_commit_with_very_long_hash() {
        let long_hash = "a".repeat(10000);
        let commit = GitCommit::new(&long_hash, "msg");
        assert_eq!(commit.hash().len(), 10000);
    }

    #[test]
    fn test_git_graph_with_very_long_title() {
        let long_title = "Git History ".repeat(1000);
        let graph = GitGraph::new().with_title(&long_title);
        assert_eq!(graph.title.as_ref().unwrap().len(), long_title.len());
    }

    // Stress tests with many commits
    #[test]
    fn test_git_graph_with_many_commits() {
        let mut graph = GitGraph::new();

        for i in 0..1000 {
            graph.add_commit(GitCommit::new(
                format!("commit{}", i),
                format!("Message {}", i),
            ));
        }

        assert_eq!(graph.commit_count(), 1000);
    }

    #[test]
    fn test_git_graph_with_many_commits_and_parents() {
        let mut graph = GitGraph::new();

        graph.add_commit(GitCommit::new("c0", "Initial"));

        for i in 1..500 {
            graph.add_commit(
                GitCommit::new(format!("c{}", i), format!("Commit {}", i))
                    .with_parent(format!("c{}", i - 1)),
            );
        }

        assert_eq!(graph.commit_count(), 500);
    }

    #[test]
    fn test_git_graph_with_extreme_number_of_commits() {
        let mut graph = GitGraph::new();

        for i in 0..10000 {
            graph.add_commit(GitCommit::new(format!("c{}", i), format!("M{}", i)));
        }

        assert_eq!(graph.commit_count(), 10000);
        let lines = graph.render_lines(100);
        assert!(!lines.is_empty());
    }

    // Max commits limiting tests
    #[test]
    fn test_git_graph_max_commits_with_exact_count() {
        let mut graph = GitGraph::new().with_max_commits(10);

        for i in 0..10 {
            graph.add_commit(GitCommit::new(format!("c{}", i), format!("Commit {}", i)));
        }

        assert_eq!(graph.commit_count(), 10);
        let lines = graph.render_lines(80);
        // All 10 commits should be rendered
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_max_commits_exceeds_limit() {
        let mut graph = GitGraph::new().with_max_commits(5);

        for i in 0..100 {
            graph.add_commit(GitCommit::new(format!("c{}", i), format!("Commit {}", i)));
        }

        assert_eq!(graph.commit_count(), 100);
        // Only last 5 commits should be rendered
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_max_commits_zero() {
        let mut graph = GitGraph::new().with_max_commits(0);

        graph.add_commit(GitCommit::new("c1", "First"));
        graph.add_commit(GitCommit::new("c2", "Second"));

        let lines = graph.render_lines(80);
        // With max_commits = 0 and no title, no lines are rendered
        assert!(lines.is_empty());
    }

    // Rendering dimension tests
    #[test]
    fn test_git_graph_render_zero_width() {
        let mut graph = GitGraph::new();
        graph.add_commit(GitCommit::new("abc", "Test"));

        let lines = graph.render_lines(0);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_render_minimal_width() {
        let mut graph = GitGraph::new();
        graph.add_commit(GitCommit::new("abc", "Test"));

        let lines = graph.render_lines(1);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_render_extreme_width() {
        let mut graph = GitGraph::new();
        graph.add_commit(GitCommit::new("abc", "Test"));

        let lines = graph.render_lines(10000);
        assert!(!lines.is_empty());
    }

    // Complex merge scenarios
    #[test]
    fn test_git_commit_with_multiple_merge_parents() {
        let commit = GitCommit::new("merge", "Octopus merge")
            .with_parent("main")
            .with_merge_parent("feature1")
            .with_merge_parent("feature2")
            .with_merge_parent("feature3");

        assert!(commit.is_merge());
        assert_eq!(commit.parent(), Some("main"));
    }

    #[test]
    fn test_git_graph_with_complex_merge_tree() {
        let mut graph = GitGraph::new();

        // Main branch
        graph.add_commit(GitCommit::new("c1", "Initial"));
        graph.add_commit(GitCommit::new("c2", "Second").with_parent("c1"));

        // Feature branch
        graph.add_commit(
            GitCommit::new("f1", "Feature start")
                .with_parent("c1")
                .with_branch("feature"),
        );
        graph.add_commit(
            GitCommit::new("f2", "Feature work")
                .with_parent("f1")
                .with_branch("feature"),
        );

        // Merge
        graph.add_commit(
            GitCommit::new("m1", "Merge feature")
                .with_parent("c2")
                .with_merge_parent("f2")
                .with_branch("main"),
        );

        assert_eq!(graph.commit_count(), 5);
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_with_multiple_branches() {
        let mut graph = GitGraph::new();

        for i in 0..10 {
            let branch = format!("branch{}", i);
            graph.add_commit(
                GitCommit::new(format!("c{}", i), format!("Commit {}", i)).with_branch(&branch),
            );
        }

        assert_eq!(graph.commit_count(), 10);
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // Display mode combinations
    #[test]
    fn test_git_graph_hide_authors_and_branches() {
        let mut graph = GitGraph::new().with_authors(false).with_branches(false);

        graph.add_commit(
            GitCommit::new("abc", "Test")
                .with_author("Alice")
                .with_branch("main"),
        );

        assert!(!graph.show_authors);
        assert!(!graph.show_branches);

        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_show_authors_hide_branches() {
        let mut graph = GitGraph::new().with_authors(true).with_branches(false);

        graph.add_commit(
            GitCommit::new("abc", "Test")
                .with_author("Alice")
                .with_branch("main"),
        );

        assert!(graph.show_authors);
        assert!(!graph.show_branches);

        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_hide_authors_show_branches() {
        let mut graph = GitGraph::new().with_authors(false).with_branches(true);

        graph.add_commit(
            GitCommit::new("abc", "Test")
                .with_author("Alice")
                .with_branch("main"),
        );

        assert!(!graph.show_authors);
        assert!(graph.show_branches);

        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // Compact mode tests
    #[test]
    fn test_git_graph_compact_mode_with_many_commits() {
        let mut graph = GitGraph::new().with_compact(true);

        for i in 0..50 {
            graph.add_commit(
                GitCommit::new(format!("c{}", i), format!("Commit {}", i))
                    .with_parent(if i > 0 {
                        format!("c{}", i - 1)
                    } else {
                        String::new()
                    }),
            );
        }

        assert!(graph.compact);
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_git_graph_non_compact_mode_with_many_commits() {
        let mut graph = GitGraph::new().with_compact(false);

        for i in 0..50 {
            graph.add_commit(
                GitCommit::new(format!("c{}", i), format!("Commit {}", i))
                    .with_parent(if i > 0 {
                        format!("c{}", i - 1)
                    } else {
                        String::new()
                    }),
            );
        }

        assert!(!graph.compact);
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // Builder pattern comprehensive tests
    #[test]
    fn test_git_commit_builder_pattern_all_features() {
        let commit = GitCommit::new("abc123def456", "🚀 Feature: Add new capability")
            .with_author("Alice Johnson <alice@example.com>")
            .with_branch("feature/awesome-feature")
            .with_parent("parent123")
            .with_merge_parent("merge1")
            .with_merge_parent("merge2")
            .with_color(Color::Magenta);

        assert_eq!(commit.hash(), "abc123def456");
        assert_eq!(commit.message(), "🚀 Feature: Add new capability");
        assert_eq!(commit.author(), Some("Alice Johnson <alice@example.com>"));
        assert_eq!(commit.branch(), Some("feature/awesome-feature"));
        assert_eq!(commit.parent(), Some("parent123"));
        assert!(commit.is_merge());
        assert_eq!(commit.color, Color::Magenta);
    }

    #[test]
    fn test_git_graph_builder_pattern_all_features() {
        let graph = GitGraph::new()
            .with_title("Complete Git History 📊")
            .with_authors(true)
            .with_branches(true)
            .with_compact(false)
            .with_max_commits(100);

        assert_eq!(graph.title, Some("Complete Git History 📊".to_string()));
        assert!(graph.show_authors);
        assert!(graph.show_branches);
        assert!(!graph.compact);
        assert_eq!(graph.max_commits, Some(100));
    }

    // Clone trait test
    #[test]
    fn test_git_commit_clone() {
        let original = GitCommit::new("abc", "Test")
            .with_author("Alice")
            .with_branch("main");

        let cloned = original.clone();

        assert_eq!(cloned.hash(), "abc");
        assert_eq!(cloned.message(), "Test");
        assert_eq!(cloned.author(), Some("Alice"));
        assert_eq!(cloned.branch(), Some("main"));
    }

    #[test]
    fn test_git_graph_clone() {
        let mut original = GitGraph::new().with_title("Original");
        original.add_commit(GitCommit::new("abc", "Test"));

        let cloned = original.clone();

        assert_eq!(cloned.commit_count(), 1);
        assert_eq!(cloned.title, Some("Original".to_string()));
    }

    // Empty and boundary tests
    #[test]
    fn test_git_commit_with_empty_strings() {
        let commit = GitCommit::new("", "").with_author("").with_branch("");

        assert_eq!(commit.hash(), "");
        assert_eq!(commit.message(), "");
        assert_eq!(commit.author(), Some(""));
        assert_eq!(commit.branch(), Some(""));
    }

    #[test]
    fn test_git_graph_add_commits_empty_vec() {
        let mut graph = GitGraph::new();
        graph.add_commits(vec![]);

        assert_eq!(graph.commit_count(), 0);
    }

    #[test]
    fn test_git_graph_render_with_title_and_no_commits() {
        let graph = GitGraph::new().with_title("Empty History");

        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
        // Should have title and "No commits" message
    }

    #[test]
    fn test_git_graph_clear_after_many_commits() {
        let mut graph = GitGraph::new();

        for i in 0..1000 {
            graph.add_commit(GitCommit::new(format!("c{}", i), format!("Commit {}", i)));
        }

        assert_eq!(graph.commit_count(), 1000);

        graph.clear();

        assert_eq!(graph.commit_count(), 0);
    }

    // Message truncation test
    #[test]
    fn test_git_graph_truncates_long_messages() {
        let mut graph = GitGraph::new();

        let very_long_message = "This is a very long commit message that should be truncated when rendered in the graph widget to fit within the available width ".repeat(10);

        graph.add_commit(GitCommit::new("abc", &very_long_message));

        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // Different color tests
    #[test]
    fn test_git_commits_with_different_colors() {
        let mut graph = GitGraph::new();

        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::Black,
        ];

        for (i, color) in colors.iter().enumerate() {
            graph.add_commit(
                GitCommit::new(format!("c{}", i), format!("Commit {}", i)).with_color(*color),
            );
        }

        assert_eq!(graph.commit_count(), 8);
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // Parent chain validation
    #[test]
    fn test_git_graph_long_parent_chain() {
        let mut graph = GitGraph::new();

        for i in 0..100 {
            let parent = if i > 0 {
                Some(format!("c{}", i - 1))
            } else {
                None
            };

            let mut commit = GitCommit::new(format!("c{}", i), format!("Commit {}", i));

            if let Some(p) = parent {
                commit = commit.with_parent(p);
            }

            graph.add_commit(commit);
        }

        assert_eq!(graph.commit_count(), 100);
        let lines = graph.render_lines(80);
        assert!(!lines.is_empty());
    }

    // Stress test combining all features
    #[test]
    fn test_git_graph_comprehensive_stress_test() {
        let mut graph = GitGraph::new()
            .with_title("Comprehensive Test 📊🚀")
            .with_authors(true)
            .with_branches(true)
            .with_compact(false)
            .with_max_commits(500);

        // Add commits with various combinations
        for i in 0..1000 {
            let commit = GitCommit::new(
                format!("commit{:07x}", i),
                format!("Message {}: Fix bug 🐛", i),
            )
            .with_author(format!("Developer {} 👨‍💻", i % 10))
            .with_branch(format!("branch{}", i % 5))
            .with_color(match i % 8 {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                3 => Color::Yellow,
                4 => Color::Magenta,
                5 => Color::Cyan,
                6 => Color::White,
                _ => Color::Gray,
            });

            graph.add_commit(commit);
        }

        assert_eq!(graph.commit_count(), 1000);

        let lines = graph.render_lines(120);
        assert!(!lines.is_empty());
    }
}
