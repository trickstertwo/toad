//! Accept/Reject panel for quick code change approval
//!
//! Provides a dedicated UI panel for reviewing and approving or rejecting
//! proposed code changes with keyboard shortcuts and batch operations.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::AcceptRejectPanel;
//!
//! let mut panel = AcceptRejectPanel::new();
//! panel.add_change("src/main.rs", "Add hello world");
//! panel.accept_current();
//! ```

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

/// State of a proposed change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeState {
    /// Pending review
    Pending,
    /// Accepted by user
    Accepted,
    /// Rejected by user
    Rejected,
}

impl ChangeState {
    /// Get color for this state
    pub fn color(&self) -> Color {
        match self {
            ChangeState::Pending => Color::Yellow,
            ChangeState::Accepted => Color::Green,
            ChangeState::Rejected => Color::Red,
        }
    }

    /// Get icon for this state
    pub fn icon(&self) -> &'static str {
        match self {
            ChangeState::Pending => "⏸",
            ChangeState::Accepted => "✓",
            ChangeState::Rejected => "✗",
        }
    }

    /// Get label for this state
    pub fn label(&self) -> &'static str {
        match self {
            ChangeState::Pending => "PENDING",
            ChangeState::Accepted => "ACCEPTED",
            ChangeState::Rejected => "REJECTED",
        }
    }
}

/// A proposed code change
#[derive(Debug, Clone)]
pub struct ProposedChange {
    /// File path
    pub file_path: String,
    /// Description of the change
    pub description: String,
    /// Change state
    pub state: ChangeState,
    /// Optional diff preview
    pub diff: Option<String>,
}

impl ProposedChange {
    /// Create a new proposed change
    pub fn new(file_path: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            description: description.into(),
            state: ChangeState::Pending,
            diff: None,
        }
    }

    /// Add diff content
    pub fn with_diff(mut self, diff: impl Into<String>) -> Self {
        self.diff = Some(diff.into());
        self
    }

    /// Accept this change
    pub fn accept(&mut self) {
        self.state = ChangeState::Accepted;
    }

    /// Reject this change
    pub fn reject(&mut self) {
        self.state = ChangeState::Rejected;
    }

    /// Reset to pending
    pub fn reset(&mut self) {
        self.state = ChangeState::Pending;
    }

    /// Check if pending
    pub fn is_pending(&self) -> bool {
        self.state == ChangeState::Pending
    }

    /// Check if accepted
    pub fn is_accepted(&self) -> bool {
        self.state == ChangeState::Accepted
    }

    /// Check if rejected
    pub fn is_rejected(&self) -> bool {
        self.state == ChangeState::Rejected
    }
}

/// Accept/Reject panel for code changes
pub struct AcceptRejectPanel {
    /// List of proposed changes
    changes: Vec<ProposedChange>,
    /// List state for navigation
    list_state: ListState,
    /// Whether to show diff preview
    show_preview: bool,
}

impl AcceptRejectPanel {
    /// Create a new accept/reject panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new();
    /// assert_eq!(panel.change_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            list_state: ListState::default(),
            show_preview: false,
        }
    }

    /// Add a proposed change
    pub fn add_change(&mut self, file_path: impl Into<String>, description: impl Into<String>) {
        self.changes.push(ProposedChange::new(file_path, description));

        // Select first item if this is the first change
        if self.changes.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Add a change with diff
    pub fn add_change_with_diff(
        &mut self,
        file_path: impl Into<String>,
        description: impl Into<String>,
        diff: impl Into<String>,
    ) {
        self.changes.push(
            ProposedChange::new(file_path, description).with_diff(diff)
        );

        if self.changes.len() == 1 {
            self.list_state.select(Some(0));
        }
    }

    /// Accept current change
    pub fn accept_current(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if let Some(change) = self.changes.get_mut(idx) {
                change.accept();
            }
            self.next();
        }
    }

    /// Reject current change
    pub fn reject_current(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if let Some(change) = self.changes.get_mut(idx) {
                change.reject();
            }
            self.next();
        }
    }

    /// Reset current change to pending
    pub fn reset_current(&mut self) {
        if let Some(idx) = self.list_state.selected()
            && let Some(change) = self.changes.get_mut(idx) {
                change.reset();
            }
    }

    /// Accept all pending changes
    pub fn accept_all(&mut self) {
        for change in &mut self.changes {
            if change.is_pending() {
                change.accept();
            }
        }
    }

    /// Reject all pending changes
    pub fn reject_all(&mut self) {
        for change in &mut self.changes {
            if change.is_pending() {
                change.reject();
            }
        }
    }

    /// Reset all changes to pending
    pub fn reset_all(&mut self) {
        for change in &mut self.changes {
            change.reset();
        }
    }

    /// Navigate to next change
    pub fn next(&mut self) {
        if self.changes.is_empty() {
            return;
        }

        let current = self.list_state.selected().unwrap_or(0);
        let next = if current >= self.changes.len() - 1 {
            0
        } else {
            current + 1
        };
        self.list_state.select(Some(next));
    }

    /// Navigate to previous change
    pub fn previous(&mut self) {
        if self.changes.is_empty() {
            return;
        }

        let current = self.list_state.selected().unwrap_or(0);
        let prev = if current == 0 {
            self.changes.len() - 1
        } else {
            current - 1
        };
        self.list_state.select(Some(prev));
    }

    /// Toggle diff preview
    pub fn toggle_preview(&mut self) {
        self.show_preview = !self.show_preview;
    }

    /// Get current change
    pub fn current_change(&self) -> Option<&ProposedChange> {
        self.list_state
            .selected()
            .and_then(|idx| self.changes.get(idx))
    }

    /// Get total change count
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Get pending count
    pub fn pending_count(&self) -> usize {
        self.changes.iter().filter(|c| c.is_pending()).count()
    }

    /// Get accepted count
    pub fn accepted_count(&self) -> usize {
        self.changes.iter().filter(|c| c.is_accepted()).count()
    }

    /// Get rejected count
    pub fn rejected_count(&self) -> usize {
        self.changes.iter().filter(|c| c.is_rejected()).count()
    }

    /// Clear all changes
    pub fn clear(&mut self) {
        self.changes.clear();
        self.list_state.select(None);
    }

    /// Get accepted changes
    pub fn accepted_changes(&self) -> Vec<&ProposedChange> {
        self.changes.iter().filter(|c| c.is_accepted()).collect()
    }

    /// Get rejected changes
    pub fn rejected_changes(&self) -> Vec<&ProposedChange> {
        self.changes.iter().filter(|c| c.is_rejected()).collect()
    }
}

impl Default for AcceptRejectPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for &AcceptRejectPanel {
    type State = ();

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        // Determine layout based on preview mode
        let show_diff_preview = self.show_preview
            && self.current_change().and_then(|c| c.diff.as_ref()).is_some();

        let (list_area, preview_area) = if show_diff_preview {
            let chunks = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);
            (chunks[0], Some(chunks[1]))
        } else {
            (area, None)
        };

        // Render change list
        let items: Vec<ListItem> = self
            .changes
            .iter()
            .map(|change| {
                let icon = change.state.icon();
                let label = change.state.label();
                let state_color = change.state.color();

                let content = vec![Line::from(vec![
                    Span::styled(
                        format!("{} ", icon),
                        Style::default().fg(state_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("[{}] ", label),
                        Style::default().fg(state_color),
                    ),
                    Span::styled(&change.file_path, Style::default().fg(Color::Cyan)),
                    Span::raw(" - "),
                    Span::styled(&change.description, Style::default().fg(Color::White)),
                ])];

                ListItem::new(content)
            })
            .collect();

        let title = format!(
            "Code Changes ({} pending, {} accepted, {} rejected)",
            self.pending_count(),
            self.accepted_count(),
            self.rejected_count()
        );

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        let mut list_state = self.list_state.clone();
        StatefulWidget::render(list, list_area, buf, &mut list_state);

        // Render diff preview if enabled
        if let Some(preview_rect) = preview_area
            && let Some(change) = self.current_change()
                && let Some(diff) = &change.diff {
                    let preview = Paragraph::new(diff.as_str())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title("Diff Preview")
                                .border_style(Style::default().fg(Color::Cyan)),
                        )
                        .style(Style::default().fg(Color::White));

                    preview.render(preview_rect, buf);
                }

        // Render footer with keybindings
        if area.height > 2 {
            let footer_area = Rect {
                x: area.x,
                y: area.y + area.height - 1,
                width: area.width,
                height: 1,
            };

            let footer_text = "a: Accept | r: Reject | u: Reset | A: Accept All | R: Reject All | p: Toggle Preview | ↑↓: Navigate";
            let footer = Paragraph::new(footer_text)
                .style(Style::default().fg(Color::DarkGray));

            footer.render(footer_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_reject_panel_new() {
        let panel = AcceptRejectPanel::new();
        assert_eq!(panel.change_count(), 0);
        assert_eq!(panel.pending_count(), 0);
    }

    #[test]
    fn test_add_change() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("src/main.rs", "Add feature");

        assert_eq!(panel.change_count(), 1);
        assert_eq!(panel.pending_count(), 1);
    }

    #[test]
    fn test_accept_current() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("src/main.rs", "Add feature");
        panel.accept_current();

        assert_eq!(panel.accepted_count(), 1);
        assert_eq!(panel.pending_count(), 0);
    }

    #[test]
    fn test_reject_current() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("src/main.rs", "Add feature");
        panel.reject_current();

        assert_eq!(panel.rejected_count(), 1);
        assert_eq!(panel.pending_count(), 0);
    }

    #[test]
    fn test_accept_all() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("src/main.rs", "Feature 1");
        panel.add_change("src/lib.rs", "Feature 2");
        panel.add_change("src/util.rs", "Feature 3");

        panel.accept_all();

        assert_eq!(panel.accepted_count(), 3);
        assert_eq!(panel.pending_count(), 0);
    }

    #[test]
    fn test_reject_all() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("src/main.rs", "Feature 1");
        panel.add_change("src/lib.rs", "Feature 2");

        panel.reject_all();

        assert_eq!(panel.rejected_count(), 2);
        assert_eq!(panel.pending_count(), 0);
    }

    #[test]
    fn test_navigation() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("file1.rs", "Change 1");
        panel.add_change("file2.rs", "Change 2");
        panel.add_change("file3.rs", "Change 3");

        assert_eq!(panel.list_state.selected(), Some(0));

        panel.next();
        assert_eq!(panel.list_state.selected(), Some(1));

        panel.next();
        assert_eq!(panel.list_state.selected(), Some(2));

        panel.next(); // Wrap around
        assert_eq!(panel.list_state.selected(), Some(0));

        panel.previous();
        assert_eq!(panel.list_state.selected(), Some(2));
    }

    #[test]
    fn test_reset() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("src/main.rs", "Feature");
        panel.accept_current();

        assert_eq!(panel.accepted_count(), 1);

        panel.reset_current();
        assert_eq!(panel.pending_count(), 1);
        assert_eq!(panel.accepted_count(), 0);
    }

    #[test]
    fn test_proposed_change() {
        let mut change = ProposedChange::new("src/main.rs", "Add feature");

        assert!(change.is_pending());
        assert!(!change.is_accepted());
        assert!(!change.is_rejected());

        change.accept();
        assert!(change.is_accepted());

        change.reject();
        assert!(change.is_rejected());

        change.reset();
        assert!(change.is_pending());
    }

    #[test]
    fn test_change_with_diff() {
        let panel = AcceptRejectPanel::new();
        let change = ProposedChange::new("src/main.rs", "Feature")
            .with_diff("@@ -1,3 +1,4 @@");

        assert!(change.diff.is_some());
    }

    #[test]
    fn test_clear() {
        let mut panel = AcceptRejectPanel::new();
        panel.add_change("file1.rs", "Change 1");
        panel.add_change("file2.rs", "Change 2");

        panel.clear();
        assert_eq!(panel.change_count(), 0);
        assert_eq!(panel.list_state.selected(), None);
    }
}
