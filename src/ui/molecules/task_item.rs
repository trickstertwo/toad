//! TaskItem molecule - Displays a task with icon and status
//!
//! Composes Icon and Text atoms to create a task list item.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Icon and Text atoms
//! - **Pure**: No mutable state, builder pattern
//! - **Composable**: Used by organisms for task lists and progress displays
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::task_item::TaskItem;
//! use toad::ui::atoms::icon::Icon;
//! use toad::ui::nerd_fonts::UiIcon;
//!
//! // Simple task
//! let task = TaskItem::new(Icon::ui(UiIcon::Loading), "Running tests");
//!
//! // With status text
//! let task = TaskItem::completed("Build project")
//!     .status("2.3s");
//!
//! // Themed constructors
//! let task = TaskItem::in_progress("Processing");
//! let task = TaskItem::completed("Done");
//! let task = TaskItem::failed("Failed");
//! ```

use crate::ui::atoms::{Icon, Text};
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// A task list item
///
/// Composes atoms to show a task with icon, name, and optional status.
/// Used for displaying task progress, to-do items, and evaluation steps.
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::task_item::TaskItem;
///
/// let task = TaskItem::completed("Build project");
/// let line = task.to_line();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TaskItem {
    /// The task icon
    icon: Icon,
    /// The task name
    name: String,
    /// Optional status text
    status_text: Option<String>,
    /// Name styling
    name_style: Option<Style>,
    /// Status styling
    status_style: Option<Style>,
}

impl TaskItem {
    /// Create a new task item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    /// use toad::ui::atoms::icon::Icon;
    /// use toad::ui::nerd_fonts::UiIcon;
    ///
    /// let task = TaskItem::new(Icon::ui(UiIcon::Loading), "Running tests");
    /// ```
    pub fn new(icon: Icon, name: impl Into<String>) -> Self {
        Self {
            icon,
            name: name.into(),
            status_text: None,
            name_style: None,
            status_style: None,
        }
    }

    /// Add status text to the task
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::completed("Build").status("2.3s");
    /// ```
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status_text = Some(status.into());
        self
    }

    /// Set name styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let task = TaskItem::completed("Build")
    ///     .name_style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn name_style(mut self, style: Style) -> Self {
        self.name_style = Some(style);
        self
    }

    /// Set status styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let task = TaskItem::completed("Build")
    ///     .status("2.3s")
    ///     .status_style(Style::default().fg(ToadTheme::GRAY));
    /// ```
    pub fn status_style(mut self, style: Style) -> Self {
        self.status_style = Some(style);
        self
    }

    /// Create a pending task item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::pending("Waiting to start");
    /// ```
    pub fn pending(name: impl Into<String>) -> Self {
        Self::new(
            Icon::ui(UiIcon::RadioUnchecked).style(Style::default().fg(ToadTheme::GRAY)),
            name,
        )
        .name_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Create an in-progress task item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::in_progress("Running tests");
    /// ```
    pub fn in_progress(name: impl Into<String>) -> Self {
        Self::new(
            Icon::ui(UiIcon::Loading).style(Style::default().fg(ToadTheme::TOAD_GREEN)),
            name,
        )
        .name_style(Style::default().fg(ToadTheme::WHITE))
    }

    /// Create a completed task item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::completed("Build finished");
    /// ```
    pub fn completed(name: impl Into<String>) -> Self {
        Self::new(
            Icon::ui(UiIcon::Success).style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT)),
            name,
        )
        .name_style(Style::default().fg(ToadTheme::TOAD_GREEN))
    }

    /// Create a failed task item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::failed("Tests failed");
    /// ```
    pub fn failed(name: impl Into<String>) -> Self {
        Self::new(
            Icon::ui(UiIcon::Error).style(Style::default().fg(ToadTheme::RED)),
            name,
        )
        .name_style(Style::default().fg(ToadTheme::RED))
    }

    /// Get the task name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::completed("Build");
    /// assert_eq!(task.name(), "Build");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the status text if set
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::completed("Build").status("2.3s");
    /// assert_eq!(task.status_text(), Some("2.3s"));
    /// ```
    pub fn status_text(&self) -> Option<&str> {
        self.status_text.as_deref()
    }

    /// Convert to spans for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::completed("Build");
    /// let spans = task.to_spans();
    /// ```
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        // Add icon
        spans.push(self.icon.to_text().to_span());
        spans.push(Span::raw(" "));

        // Add name
        let mut name_text = Text::new(&self.name);
        if let Some(style) = self.name_style {
            name_text = name_text.style(style);
        }
        spans.push(name_text.to_span());

        // Add status if present
        if let Some(ref status) = self.status_text {
            spans.push(Span::raw(" "));
            let mut status_text = Text::new(status);
            if let Some(style) = self.status_style {
                status_text = status_text.style(style);
            }
            spans.push(status_text.to_span());
        }

        spans
    }

    /// Convert to a line for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::task_item::TaskItem;
    ///
    /// let task = TaskItem::completed("Build");
    /// let line = task.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_item_new() {
        let icon = Icon::ui(UiIcon::Loading);
        let task = TaskItem::new(icon, "Test task");
        assert_eq!(task.name(), "Test task");
        assert_eq!(task.status_text(), None);
    }

    #[test]
    fn test_task_item_with_status() {
        let task = TaskItem::completed("Build").status("2.3s");
        assert_eq!(task.status_text(), Some("2.3s"));
    }

    #[test]
    fn test_task_item_name_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let task = TaskItem::completed("Build").name_style(style);
        assert_eq!(task.name_style, Some(style));
    }

    #[test]
    fn test_task_item_status_style() {
        let style = Style::default().fg(ToadTheme::GRAY);
        let task = TaskItem::completed("Build")
            .status("2.3s")
            .status_style(style);
        assert_eq!(task.status_style, Some(style));
    }

    #[test]
    fn test_task_item_pending() {
        let task = TaskItem::pending("Waiting");
        assert_eq!(task.name(), "Waiting");
        assert!(task.name_style.is_some());
    }

    #[test]
    fn test_task_item_in_progress() {
        let task = TaskItem::in_progress("Running");
        assert_eq!(task.name(), "Running");
        assert!(task.name_style.is_some());
    }

    #[test]
    fn test_task_item_completed() {
        let task = TaskItem::completed("Done");
        assert_eq!(task.name(), "Done");
        assert!(task.name_style.is_some());
    }

    #[test]
    fn test_task_item_failed() {
        let task = TaskItem::failed("Error");
        assert_eq!(task.name(), "Error");
        assert!(task.name_style.is_some());
    }

    #[test]
    fn test_task_item_to_spans() {
        let task = TaskItem::completed("Build");
        let spans = task.to_spans();
        // Should have: icon + " " + name = 3 spans
        assert_eq!(spans.len(), 3);
    }

    #[test]
    fn test_task_item_to_spans_with_status() {
        let task = TaskItem::completed("Build").status("2.3s");
        let spans = task.to_spans();
        // Should have: icon + " " + name + " " + status = 5 spans
        assert_eq!(spans.len(), 5);
    }

    #[test]
    fn test_task_item_to_line() {
        let task = TaskItem::completed("Build");
        let _line = task.to_line();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_task_item_chaining() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let task = TaskItem::completed("Build")
            .status("2.3s")
            .name_style(style)
            .status_style(style);

        assert_eq!(task.status_text(), Some("2.3s"));
        assert_eq!(task.name_style, Some(style));
        assert_eq!(task.status_style, Some(style));
    }

    #[test]
    fn test_task_item_clone() {
        let task1 = TaskItem::completed("Build");
        let task2 = task1.clone();
        assert_eq!(task1.name(), task2.name());
    }

    #[test]
    fn test_task_item_equality() {
        let icon = Icon::ui(UiIcon::Success);
        let task1 = TaskItem::new(icon.clone(), "Test");
        let task2 = TaskItem::new(icon.clone(), "Test");
        let task3 = TaskItem::new(icon, "Other");

        assert_eq!(task1, task2);
        assert_ne!(task1, task3);
    }

    #[test]
    fn test_task_item_empty_name() {
        let task = TaskItem::completed("");
        assert_eq!(task.name(), "");
    }

    #[test]
    fn test_task_item_unicode() {
        let task = TaskItem::completed("テスト");
        assert_eq!(task.name(), "テスト");
    }

    #[test]
    fn test_task_item_long_text() {
        let long_name = "A".repeat(100);
        let task = TaskItem::completed(&long_name);
        assert_eq!(task.name(), long_name);
    }
}
