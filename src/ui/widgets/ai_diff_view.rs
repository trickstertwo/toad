//! AI diff view widget for proposed code changes
//!
//! Displays AI-proposed code changes with accept/reject functionality,
//! syntax highlighting, and side-by-side or unified diff views.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::AIDiffView;
//!
//! let diff = r#"
//! @@ -1,3 +1,4 @@
//!  fn main() {
//! +    println!("Hello from AI!");
//!      println!("Original code");
//!  }
//! "#;
//!
//! let mut view = AIDiffView::new();
//! view.set_proposed_changes("src/main.rs", diff);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

/// Diff hunk with accept/reject state
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Hunk header (e.g., "@@ -1,3 +1,4 @@")
    pub header: String,
    /// Lines in this hunk
    pub lines: Vec<AIDiffLine>,
    /// Whether this hunk is accepted
    pub accepted: bool,
    /// Whether this hunk is rejected
    pub rejected: bool,
}

impl DiffHunk {
    /// Create a new diff hunk
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            lines: Vec::new(),
            accepted: false,
            rejected: false,
        }
    }

    /// Add a line to the hunk
    pub fn add_line(&mut self, line: AIDiffLine) {
        self.lines.push(line);
    }

    /// Accept this hunk
    pub fn accept(&mut self) {
        self.accepted = true;
        self.rejected = false;
    }

    /// Reject this hunk
    pub fn reject(&mut self) {
        self.accepted = false;
        self.rejected = true;
    }

    /// Clear accept/reject state
    pub fn clear_state(&mut self) {
        self.accepted = false;
        self.rejected = false;
    }

    /// Check if hunk has a decision
    pub fn has_decision(&self) -> bool {
        self.accepted || self.rejected
    }
}

/// Single line in a diff
#[derive(Debug, Clone)]
pub struct AIDiffLine {
    /// Line content
    pub content: String,
    /// Line type
    pub line_type: AIDiffLineType,
    /// Old line number (if applicable)
    pub old_line_num: Option<usize>,
    /// New line number (if applicable)
    pub new_line_num: Option<usize>,
}

/// Type of diff line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIDiffLineType {
    /// Added line (+)
    Addition,
    /// Deleted line (-)
    Deletion,
    /// Context line (unchanged)
    Context,
}

impl AIDiffLineType {
    /// Get color for this line type
    pub fn color(&self) -> Color {
        match self {
            AIDiffLineType::Addition => Color::Green,
            AIDiffLineType::Deletion => Color::Red,
            AIDiffLineType::Context => Color::Gray,
        }
    }

    /// Get background color
    pub fn bg_color(&self) -> Option<Color> {
        match self {
            AIDiffLineType::Addition => Some(Color::Rgb(0, 50, 0)),
            AIDiffLineType::Deletion => Some(Color::Rgb(50, 0, 0)),
            AIDiffLineType::Context => None,
        }
    }
}

/// AI diff view mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffViewMode {
    /// Unified diff (traditional)
    Unified,
    /// Side-by-side diff
    SideBySide,
}

/// AI diff view widget
///
/// Displays proposed code changes from AI with accept/reject functionality.
///
/// # Features
///
/// - Unified or side-by-side diff views
/// - Accept/reject individual hunks or entire files
/// - Syntax highlighting
/// - Keyboard navigation
/// - Summary of changes
///
/// # Keybindings
///
/// - `j/k`: Navigate hunks
/// - `a`: Accept current hunk
/// - `r`: Reject current hunk
/// - `A`: Accept all hunks
/// - `R`: Reject all hunks
/// - `c`: Clear decision for current hunk
/// - `Tab`: Toggle view mode
pub struct AIDiffView {
    /// File path being modified
    file_path: String,
    /// Diff hunks
    hunks: Vec<DiffHunk>,
    /// Current hunk index
    current_hunk: usize,
    /// View mode
    view_mode: DiffViewMode,
    /// List state for navigation
    list_state: ListState,
    /// AI explanation of changes
    explanation: Option<String>,
    /// Show line numbers
    show_line_numbers: bool,
}

impl AIDiffView {
    /// Create a new AI diff view
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::AIDiffView;
    ///
    /// let view = AIDiffView::new();
    /// assert_eq!(view.hunk_count(), 0);
    /// ```
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            file_path: String::new(),
            hunks: Vec::new(),
            current_hunk: 0,
            view_mode: DiffViewMode::Unified,
            list_state,
            explanation: None,
            show_line_numbers: true,
        }
    }

    /// Set proposed changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::AIDiffView;
    ///
    /// let mut view = AIDiffView::new();
    /// view.set_proposed_changes("main.rs", "@@ -1,1 +1,2 @@\n fn main() {}\n+println!(\"hi\");");
    /// ```
    pub fn set_proposed_changes(&mut self, file_path: impl Into<String>, diff: impl AsRef<str>) {
        self.file_path = file_path.into();
        self.hunks = self.parse_diff(diff.as_ref());
        self.current_hunk = 0;
        if !self.hunks.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    /// Parse unified diff format
    fn parse_diff(&self, diff: &str) -> Vec<DiffHunk> {
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;
        let mut old_line = 0;
        let mut new_line = 0;

        for line in diff.lines() {
            if line.starts_with("@@") {
                // Save previous hunk
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }

                // Parse hunk header to get line numbers
                if let Some((old_part, new_part)) = line.split_once("@@").and_then(|(_, rest)| {
                    rest.trim().split_once("@@").map(|(header, _)| header)
                }).and_then(|header| {
                    let parts: Vec<&str> = header.split_whitespace().collect();
                    if parts.len() >= 2 {
                        Some((parts[0], parts[1]))
                    } else {
                        None
                    }
                }) {
                    old_line = old_part.trim_start_matches('-').split(',').next()
                        .and_then(|s| s.parse().ok()).unwrap_or(1);
                    new_line = new_part.trim_start_matches('+').split(',').next()
                        .and_then(|s| s.parse().ok()).unwrap_or(1);
                }

                current_hunk = Some(DiffHunk::new(line));
            } else if let Some(hunk) = &mut current_hunk {
                let (_line_type, content) = if line.starts_with('+') {
                    let content = line[1..].to_string();
                    let diff_line = AIDiffLine {
                        content,
                        line_type: AIDiffLineType::Addition,
                        old_line_num: None,
                        new_line_num: Some(new_line),
                    };
                    new_line += 1;
                    (AIDiffLineType::Addition, diff_line)
                } else if line.starts_with('-') {
                    let content = line[1..].to_string();
                    let diff_line = AIDiffLine {
                        content,
                        line_type: AIDiffLineType::Deletion,
                        old_line_num: Some(old_line),
                        new_line_num: None,
                    };
                    old_line += 1;
                    (AIDiffLineType::Deletion, diff_line)
                } else {
                    // Context line
                    let content = if line.starts_with(' ') {
                        line[1..].to_string()
                    } else {
                        line.to_string()
                    };
                    let diff_line = AIDiffLine {
                        content,
                        line_type: AIDiffLineType::Context,
                        old_line_num: Some(old_line),
                        new_line_num: Some(new_line),
                    };
                    old_line += 1;
                    new_line += 1;
                    (AIDiffLineType::Context, diff_line)
                };

                hunk.add_line(content);
            }
        }

        // Save last hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        hunks
    }

    /// Set AI explanation
    pub fn set_explanation(&mut self, explanation: impl Into<String>) {
        self.explanation = Some(explanation.into());
    }

    /// Get current hunk
    pub fn current_hunk(&self) -> Option<&DiffHunk> {
        self.hunks.get(self.current_hunk)
    }

    /// Get current hunk mutably
    pub fn current_hunk_mut(&mut self) -> Option<&mut DiffHunk> {
        self.hunks.get_mut(self.current_hunk)
    }

    /// Navigate to next hunk
    pub fn next_hunk(&mut self) {
        if self.current_hunk + 1 < self.hunks.len() {
            self.current_hunk += 1;
            self.list_state.select(Some(self.current_hunk));
        }
    }

    /// Navigate to previous hunk
    pub fn previous_hunk(&mut self) {
        if self.current_hunk > 0 {
            self.current_hunk -= 1;
            self.list_state.select(Some(self.current_hunk));
        }
    }

    /// Accept current hunk
    pub fn accept_current(&mut self) {
        if let Some(hunk) = self.current_hunk_mut() {
            hunk.accept();
        }
    }

    /// Reject current hunk
    pub fn reject_current(&mut self) {
        if let Some(hunk) = self.current_hunk_mut() {
            hunk.reject();
        }
    }

    /// Accept all hunks
    pub fn accept_all(&mut self) {
        for hunk in &mut self.hunks {
            hunk.accept();
        }
    }

    /// Reject all hunks
    pub fn reject_all(&mut self) {
        for hunk in &mut self.hunks {
            hunk.reject();
        }
    }

    /// Clear decision for current hunk
    pub fn clear_current(&mut self) {
        if let Some(hunk) = self.current_hunk_mut() {
            hunk.clear_state();
        }
    }

    /// Toggle view mode
    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            DiffViewMode::Unified => DiffViewMode::SideBySide,
            DiffViewMode::SideBySide => DiffViewMode::Unified,
        };
    }

    /// Get hunk count
    pub fn hunk_count(&self) -> usize {
        self.hunks.len()
    }

    /// Get acceptance summary
    pub fn get_summary(&self) -> (usize, usize, usize) {
        let accepted = self.hunks.iter().filter(|h| h.accepted).count();
        let rejected = self.hunks.iter().filter(|h| h.rejected).count();
        let pending = self.hunks.len() - accepted - rejected;
        (accepted, rejected, pending)
    }

    /// Check if all hunks have decisions
    pub fn all_decided(&self) -> bool {
        self.hunks.iter().all(|h| h.has_decision())
    }
}

impl Default for AIDiffView {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &mut AIDiffView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Split into header, diff view, footer
        let chunks = Layout::vertical([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Diff view
            Constraint::Length(3),  // Footer
        ])
        .split(area);

        // Render header
        let (accepted, rejected, pending) = self.get_summary();
        let header_text = vec![Line::from(vec![
            Span::styled("File: ", Style::default().fg(Color::Gray)),
            Span::styled(&self.file_path, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  |  "),
            Span::styled(format!("✓ {}", accepted), Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled(format!("✗ {}", rejected), Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled(format!("? {}", pending), Style::default().fg(Color::Yellow)),
        ])];

        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).title("AI Proposed Changes"));
        header.render(chunks[0], buf);

        // Render diff hunks
        if !self.hunks.is_empty() {
            let items: Vec<ListItem> = self.hunks.iter().map(|hunk| {
                let mut lines = vec![Line::from(vec![
                    Span::styled(&hunk.header, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                ])];

                // Add lines from hunk
                for line in &hunk.lines {
                    let prefix = match line.line_type {
                        AIDiffLineType::Addition => "+",
                        AIDiffLineType::Deletion => "-",
                        AIDiffLineType::Context => " ",
                    };

                    let mut style = Style::default().fg(line.line_type.color());
                    if let Some(bg) = line.line_type.bg_color() {
                        style = style.bg(bg);
                    }

                    lines.push(Line::from(vec![
                        Span::styled(prefix, style),
                        Span::styled(&line.content, style),
                    ]));
                }

                // Add status indicator
                let status = if hunk.accepted {
                    Span::styled(" [ACCEPTED]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                } else if hunk.rejected {
                    Span::styled(" [REJECTED]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(" [PENDING]", Style::default().fg(Color::Yellow))
                };
                lines.push(Line::from(vec![status]));

                ListItem::new(lines)
            }).collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Changes"))
                .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

            StatefulWidget::render(list, chunks[1], buf, &mut self.list_state);
        }

        // Render footer with keybindings
        let footer_text = "j/k: Navigate | a: Accept | r: Reject | A: Accept All | R: Reject All | c: Clear | Tab: Toggle View | Esc: Close";
        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        footer.render(chunks[2], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_diff_view_new() {
        let view = AIDiffView::new();
        assert_eq!(view.hunk_count(), 0);
        assert_eq!(view.file_path, "");
    }

    #[test]
    fn test_ai_diff_view_set_changes() {
        let mut view = AIDiffView::new();
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+println!(\"hi\");";
        view.set_proposed_changes("main.rs", diff);

        assert_eq!(view.file_path, "main.rs");
        assert!(view.hunk_count() > 0);
    }

    #[test]
    fn test_ai_diff_view_accept_reject() {
        let mut view = AIDiffView::new();
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+println!(\"hi\");";
        view.set_proposed_changes("main.rs", diff);

        view.accept_current();
        let (accepted, rejected, pending) = view.get_summary();
        assert_eq!(accepted, 1);
        assert_eq!(rejected, 0);

        view.reject_current();
        let (accepted, rejected, pending) = view.get_summary();
        assert_eq!(accepted, 0);
        assert_eq!(rejected, 1);
    }

    #[test]
    fn test_ai_diff_view_navigation() {
        let mut view = AIDiffView::new();
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+println!(\"hi\");\n@@ -5,1 +6,2 @@\n fn test() {}\n+println!(\"test\");";
        view.set_proposed_changes("main.rs", diff);

        assert_eq!(view.current_hunk, 0);
        view.next_hunk();
        assert_eq!(view.current_hunk, 1);
        view.previous_hunk();
        assert_eq!(view.current_hunk, 0);
    }

    #[test]
    fn test_ai_diff_view_accept_all() {
        let mut view = AIDiffView::new();
        let diff = "@@ -1,1 +1,2 @@\n fn main() {}\n+println!(\"hi\");\n@@ -5,1 +6,2 @@\n fn test() {}\n+println!(\"test\");";
        view.set_proposed_changes("main.rs", diff);

        view.accept_all();
        let (accepted, rejected, pending) = view.get_summary();
        assert_eq!(accepted, view.hunk_count());
        assert_eq!(pending, 0);
    }

    #[test]
    fn test_ai_diff_view_toggle_view_mode() {
        let mut view = AIDiffView::new();
        assert_eq!(view.view_mode, DiffViewMode::Unified);

        view.toggle_view_mode();
        assert_eq!(view.view_mode, DiffViewMode::SideBySide);

        view.toggle_view_mode();
        assert_eq!(view.view_mode, DiffViewMode::Unified);
    }

    #[test]
    fn test_diff_hunk() {
        let mut hunk = DiffHunk::new("@@ -1,1 +1,2 @@");

        assert!(!hunk.has_decision());

        hunk.accept();
        assert!(hunk.accepted);
        assert!(!hunk.rejected);
        assert!(hunk.has_decision());

        hunk.reject();
        assert!(!hunk.accepted);
        assert!(hunk.rejected);

        hunk.clear_state();
        assert!(!hunk.has_decision());
    }
}
