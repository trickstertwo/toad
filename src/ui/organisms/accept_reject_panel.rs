//! AcceptRejectPanel organism - Code approval interface
//!
//! Composes MetricCard, TaskItem, and ProgressBar molecules into a comprehensive
//! code review and approval panel.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Organism**: Composes multiple molecules (MetricCard, TaskItem, ProgressBar)
//! - **Complex**: Displays complete code review status
//! - **Stateful**: Holds code change approval data
//! - **Composable**: Used by code review screens
//!
//! # Examples
//!
//! ```
//! use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
//!
//! let panel = AcceptRejectPanel::new()
//!     .title("Code Review: feature/new-api")
//!     .total_changes(10)
//!     .accepted(7)
//!     .rejected(2)
//!     .pending(1);
//!
//! let widget = panel.render(area, buf);
//! ```

use crate::ui::atoms::{Block, Icon};
use crate::ui::molecules::{MetricCard, ProgressBar, TaskItem};
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Paragraph, Widget},
};

/// Code approval panel
///
/// Composes molecules to display comprehensive code review status including:
/// - Overall approval progress bar
/// - Metrics (accepted, rejected, pending)
/// - Change list with approval status
///
/// # Examples
///
/// ```
/// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
///
/// let panel = AcceptRejectPanel::new()
///     .total_changes(10)
///     .accepted(7);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AcceptRejectPanel {
    /// Total number of code changes
    pub total_changes: usize,
    /// Number of accepted changes
    pub accepted: usize,
    /// Number of rejected changes
    pub rejected: usize,
    /// Number of pending changes
    pub pending: usize,
    /// List of change statuses
    pub changes: Vec<ChangeStatus>,
    /// Panel title
    pub title: String,
}

/// Code change status for display
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeStatus {
    /// Change description/file path
    pub description: String,
    /// Change approval state
    pub state: ChangeState,
    /// Optional details (line count, etc.)
    pub details: Option<String>,
}

/// Change approval state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeState {
    /// Change is pending review
    Pending,
    /// Change has been accepted
    Accepted,
    /// Change has been rejected
    Rejected,
}

impl AcceptRejectPanel {
    /// Create a new accept/reject panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new();
    /// ```
    pub fn new() -> Self {
        Self {
            total_changes: 0,
            accepted: 0,
            rejected: 0,
            pending: 0,
            changes: Vec::new(),
            title: "Code Review".to_string(),
        }
    }

    /// Set the panel title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new().title("PR #123: Add feature");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set total number of changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new().total_changes(10);
    /// ```
    pub fn total_changes(mut self, total: usize) -> Self {
        self.total_changes = total;
        self
    }

    /// Set number of accepted changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new().accepted(7);
    /// ```
    pub fn accepted(mut self, count: usize) -> Self {
        self.accepted = count;
        self
    }

    /// Set number of rejected changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new().rejected(2);
    /// ```
    pub fn rejected(mut self, count: usize) -> Self {
        self.rejected = count;
        self
    }

    /// Set number of pending changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new().pending(3);
    /// ```
    pub fn pending(mut self, count: usize) -> Self {
        self.pending = count;
        self
    }

    /// Add a change status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::{AcceptRejectPanel, ChangeStatus, ChangeState};
    ///
    /// let panel = AcceptRejectPanel::new().add_change(ChangeStatus {
    ///     description: "src/main.rs".to_string(),
    ///     state: ChangeState::Accepted,
    ///     details: Some("+42 -10 lines".to_string()),
    /// });
    /// ```
    pub fn add_change(mut self, change: ChangeStatus) -> Self {
        self.changes.push(change);
        self
    }

    /// Set all changes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::{AcceptRejectPanel, ChangeStatus, ChangeState};
    ///
    /// let changes = vec![
    ///     ChangeStatus {
    ///         description: "file1.rs".to_string(),
    ///         state: ChangeState::Accepted,
    ///         details: None,
    ///     },
    /// ];
    /// let panel = AcceptRejectPanel::new().changes(changes);
    /// ```
    pub fn changes(mut self, changes: Vec<ChangeStatus>) -> Self {
        self.changes = changes;
        self
    }

    /// Get approval percentage (accepted / total)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    ///
    /// let panel = AcceptRejectPanel::new().total_changes(10).accepted(7);
    /// assert_eq!(panel.approval_percentage(), 70.0);
    /// ```
    pub fn approval_percentage(&self) -> f64 {
        if self.total_changes == 0 {
            0.0
        } else {
            (self.accepted as f64 / self.total_changes as f64) * 100.0
        }
    }

    /// Render the panel to a buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::organisms::accept_reject_panel::AcceptRejectPanel;
    /// use ratatui::{buffer::Buffer, layout::Rect};
    ///
    /// let panel = AcceptRejectPanel::new().total_changes(10).accepted(7);
    /// let area = Rect::new(0, 0, 80, 24);
    /// let mut buf = Buffer::empty(area);
    /// panel.render(area, &mut buf);
    /// ```
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create border
        let block = Block::themed(&self.title).to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        // Split layout: progress bar | metrics | changes
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Spacer
                Constraint::Length(3), // Metrics
                Constraint::Length(1), // Spacer
                Constraint::Min(0),    // Changes
            ])
            .split(inner);

        // Render progress bar
        self.render_progress(chunks[0], buf);

        // Render metrics
        self.render_metrics(chunks[2], buf);

        // Render changes
        self.render_changes(chunks[4], buf);
    }

    /// Render approval progress bar
    fn render_progress(&self, area: Rect, buf: &mut Buffer) {
        let progress = if self.total_changes > 0 {
            ProgressBar::new("Review Progress", self.accepted, self.total_changes)
                .width(20)
                .bar_style(Style::default().fg(ToadTheme::TOAD_GREEN))
                .label_style(Style::default().fg(ToadTheme::WHITE))
        } else {
            ProgressBar::new("Review Progress", 0, 1)
                .width(20)
                .bar_style(Style::default().fg(ToadTheme::GRAY))
                .label_style(Style::default().fg(ToadTheme::GRAY))
        };

        let line = progress.to_line();
        Paragraph::new(line).render(area, buf);
    }

    /// Render approval metrics
    fn render_metrics(&self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();

        // Accepted changes
        let accepted_card = MetricCard::new("Accepted", format!("{}", self.accepted))
            .icon(Icon::ui(UiIcon::Success).style(Style::default().fg(ToadTheme::TOAD_GREEN)))
            .label_style(Style::default().fg(ToadTheme::GRAY))
            .value_style(Style::default().fg(ToadTheme::TOAD_GREEN));
        lines.push(accepted_card.to_line());

        // Rejected changes
        let rejected_card = MetricCard::new("Rejected", format!("{}", self.rejected))
            .icon(Icon::ui(UiIcon::Error).style(Style::default().fg(ToadTheme::RED)))
            .label_style(Style::default().fg(ToadTheme::GRAY))
            .value_style(Style::default().fg(ToadTheme::RED));
        lines.push(rejected_card.to_line());

        // Pending changes
        let pending_card = MetricCard::new("Pending", format!("{}", self.pending))
            .icon(Icon::ui(UiIcon::Clock).style(Style::default().fg(ToadTheme::YELLOW)))
            .label_style(Style::default().fg(ToadTheme::GRAY))
            .value_style(Style::default().fg(ToadTheme::WHITE));
        lines.push(pending_card.to_line());

        Paragraph::new(lines).render(area, buf);
    }

    /// Render change list
    fn render_changes(&self, area: Rect, buf: &mut Buffer) {
        let lines: Vec<Line> = self
            .changes
            .iter()
            .map(|change| {
                let item = match change.state {
                    ChangeState::Pending => TaskItem::pending(&change.description),
                    ChangeState::Accepted => TaskItem::completed(&change.description),
                    ChangeState::Rejected => TaskItem::failed(&change.description),
                };

                if let Some(ref details) = change.details {
                    item.status(details).to_line()
                } else {
                    item.to_line()
                }
            })
            .collect();

        Paragraph::new(lines).render(area, buf);
    }
}

impl Default for AcceptRejectPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for AcceptRejectPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &AcceptRejectPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        AcceptRejectPanel::render(self, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_reject_panel_new() {
        let panel = AcceptRejectPanel::new();
        assert_eq!(panel.total_changes, 0);
        assert_eq!(panel.accepted, 0);
        assert_eq!(panel.rejected, 0);
        assert_eq!(panel.pending, 0);
    }

    #[test]
    fn test_accept_reject_panel_title() {
        let panel = AcceptRejectPanel::new().title("PR #123");
        assert_eq!(panel.title, "PR #123");
    }

    #[test]
    fn test_accept_reject_panel_total_changes() {
        let panel = AcceptRejectPanel::new().total_changes(10);
        assert_eq!(panel.total_changes, 10);
    }

    #[test]
    fn test_accept_reject_panel_accepted() {
        let panel = AcceptRejectPanel::new().accepted(7);
        assert_eq!(panel.accepted, 7);
    }

    #[test]
    fn test_accept_reject_panel_rejected() {
        let panel = AcceptRejectPanel::new().rejected(2);
        assert_eq!(panel.rejected, 2);
    }

    #[test]
    fn test_accept_reject_panel_pending() {
        let panel = AcceptRejectPanel::new().pending(3);
        assert_eq!(panel.pending, 3);
    }

    #[test]
    fn test_accept_reject_panel_add_change() {
        let panel = AcceptRejectPanel::new().add_change(ChangeStatus {
            description: "file.rs".to_string(),
            state: ChangeState::Accepted,
            details: None,
        });
        assert_eq!(panel.changes.len(), 1);
    }

    #[test]
    fn test_accept_reject_panel_changes() {
        let changes = vec![
            ChangeStatus {
                description: "file1.rs".to_string(),
                state: ChangeState::Accepted,
                details: None,
            },
            ChangeStatus {
                description: "file2.rs".to_string(),
                state: ChangeState::Rejected,
                details: Some("+10 -5".to_string()),
            },
        ];
        let panel = AcceptRejectPanel::new().changes(changes.clone());
        assert_eq!(panel.changes, changes);
    }

    #[test]
    fn test_accept_reject_panel_approval_percentage() {
        let panel = AcceptRejectPanel::new().total_changes(10).accepted(7);
        assert_eq!(panel.approval_percentage(), 70.0);
    }

    #[test]
    fn test_accept_reject_panel_approval_percentage_zero_total() {
        let panel = AcceptRejectPanel::new().total_changes(0).accepted(0);
        assert_eq!(panel.approval_percentage(), 0.0);
    }

    #[test]
    fn test_accept_reject_panel_approval_percentage_all_accepted() {
        let panel = AcceptRejectPanel::new().total_changes(10).accepted(10);
        assert_eq!(panel.approval_percentage(), 100.0);
    }

    #[test]
    fn test_accept_reject_panel_approval_percentage_none_accepted() {
        let panel = AcceptRejectPanel::new().total_changes(10).accepted(0);
        assert_eq!(panel.approval_percentage(), 0.0);
    }

    #[test]
    fn test_accept_reject_panel_chaining() {
        let panel = AcceptRejectPanel::new()
            .title("Test")
            .total_changes(10)
            .accepted(7)
            .rejected(2)
            .pending(1);

        assert_eq!(panel.title, "Test");
        assert_eq!(panel.total_changes, 10);
        assert_eq!(panel.accepted, 7);
        assert_eq!(panel.rejected, 2);
        assert_eq!(panel.pending, 1);
    }

    #[test]
    fn test_accept_reject_panel_render() {
        let panel = AcceptRejectPanel::new()
            .total_changes(10)
            .accepted(7)
            .rejected(2)
            .pending(1);

        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_accept_reject_panel_render_with_changes() {
        let changes = vec![
            ChangeStatus {
                description: "src/main.rs".to_string(),
                state: ChangeState::Accepted,
                details: Some("+42 -10 lines".to_string()),
            },
            ChangeStatus {
                description: "src/lib.rs".to_string(),
                state: ChangeState::Pending,
                details: None,
            },
        ];

        let panel = AcceptRejectPanel::new().changes(changes);
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_accept_reject_panel_widget_trait() {
        let panel = AcceptRejectPanel::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        // Test both owned and borrowed widget rendering
        panel.clone().render(area, &mut buf);
        (&panel).render(area, &mut buf);
    }

    #[test]
    fn test_accept_reject_panel_default() {
        let panel = AcceptRejectPanel::default();
        assert_eq!(panel.total_changes, 0);
        assert_eq!(panel.accepted, 0);
    }

    #[test]
    fn test_change_status() {
        let status = ChangeStatus {
            description: "test.rs".to_string(),
            state: ChangeState::Accepted,
            details: Some("+5 -2".to_string()),
        };
        assert_eq!(status.description, "test.rs");
        assert_eq!(status.state, ChangeState::Accepted);
        assert_eq!(status.details, Some("+5 -2".to_string()));
    }

    #[test]
    fn test_change_state_equality() {
        assert_eq!(ChangeState::Pending, ChangeState::Pending);
        assert_ne!(ChangeState::Pending, ChangeState::Accepted);
    }

    #[test]
    fn test_accept_reject_panel_clone() {
        let panel1 = AcceptRejectPanel::new().total_changes(10);
        let panel2 = panel1.clone();
        assert_eq!(panel1, panel2);
    }

    #[test]
    fn test_accept_reject_panel_empty_render() {
        let panel = AcceptRejectPanel::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        panel.render(area, &mut buf);
        // Verify empty panel renders without panic
    }

    #[test]
    fn test_accept_reject_panel_full_workflow() {
        let mut panel = AcceptRejectPanel::new()
            .title("PR #456: Refactor API")
            .total_changes(15);

        // Add changes incrementally
        panel = panel.add_change(ChangeStatus {
            description: "src/api.rs".to_string(),
            state: ChangeState::Accepted,
            details: Some("+100 -50".to_string()),
        });

        panel = panel.add_change(ChangeStatus {
            description: "src/db.rs".to_string(),
            state: ChangeState::Rejected,
            details: Some("+20 -10".to_string()),
        });

        panel = panel.add_change(ChangeStatus {
            description: "src/utils.rs".to_string(),
            state: ChangeState::Pending,
            details: None,
        });

        // Update counts
        panel = panel.accepted(10).rejected(3).pending(2);

        assert_eq!(panel.changes.len(), 3);
        assert_eq!(panel.accepted, 10);
        assert_eq!(panel.rejected, 3);
        assert_eq!(panel.pending, 2);
        assert_eq!(panel.approval_percentage(), (10.0 / 15.0) * 100.0);
    }

    #[test]
    fn test_accept_reject_panel_multiple_changes() {
        let changes = vec![
            ChangeStatus {
                description: "file1.rs".to_string(),
                state: ChangeState::Accepted,
                details: None,
            },
            ChangeStatus {
                description: "file2.rs".to_string(),
                state: ChangeState::Accepted,
                details: None,
            },
            ChangeStatus {
                description: "file3.rs".to_string(),
                state: ChangeState::Rejected,
                details: None,
            },
            ChangeStatus {
                description: "file4.rs".to_string(),
                state: ChangeState::Pending,
                details: None,
            },
        ];

        let panel = AcceptRejectPanel::new()
            .total_changes(4)
            .accepted(2)
            .rejected(1)
            .pending(1)
            .changes(changes);

        assert_eq!(panel.changes.len(), 4);
        assert_eq!(panel.approval_percentage(), 50.0);
    }
}
