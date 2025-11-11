//! ToolExecutionItem molecule - Tool call display
//!
//! Displays individual tool executions during AI agent runs with status and timing.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Icon + Text atoms
//! - **Purpose**: Show tool execution status and details
//! - **Visual**: Icon, tool name, status, duration
//! - **States**: Running, Success, Failed
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::tool_execution_item::{ToolExecutionItem, ExecutionStatus};
//!
//! let item = ToolExecutionItem::new("Read", "src/main.rs")
//!     .status(ExecutionStatus::Success)
//!     .duration_ms(125);
//! let line = item.to_line();
//! ```

use crate::ui::atoms::Icon;
use crate::ui::primitives::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// Tool execution item display
///
/// Shows individual tool call with status, timing, and optional result preview:
/// - Running: Tool is currently executing
/// - Success: Tool completed successfully
/// - Failed: Tool execution failed
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::tool_execution_item::{ToolExecutionItem, ExecutionStatus};
///
/// let item = ToolExecutionItem::new("Write", "src/lib.rs");
/// assert_eq!(item.tool_name(), "Write");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ToolExecutionItem {
    /// Tool name (Read, Write, Edit, Bash, etc.)
    tool_name: String,
    /// Tool description/arguments
    description: String,
    /// Execution status
    status: ExecutionStatus,
    /// Duration in milliseconds
    duration_ms: Option<u64>,
    /// Optional result preview
    result_preview: Option<String>,
    /// Show duration
    show_duration: bool,
    /// Show icon
    show_icon: bool,
    /// Custom style override
    style: Option<Style>,
}

impl ToolExecutionItem {
    /// Create a new tool execution item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Read", "src/main.rs");
    /// ```
    pub fn new(tool_name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            description: description.into(),
            status: ExecutionStatus::Running,
            duration_ms: None,
            result_preview: None,
            show_duration: true,
            show_icon: true,
            style: None,
        }
    }

    /// Set execution status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::{ToolExecutionItem, ExecutionStatus};
    ///
    /// let item = ToolExecutionItem::new("Write", "file.rs")
    ///     .status(ExecutionStatus::Success);
    /// ```
    pub fn status(mut self, status: ExecutionStatus) -> Self {
        self.status = status;
        self
    }

    /// Set duration in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Read", "file.rs").duration_ms(125);
    /// ```
    pub fn duration_ms(mut self, duration: u64) -> Self {
        self.duration_ms = Some(duration);
        self
    }

    /// Set result preview
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Read", "file.rs")
    ///     .result_preview("200 lines");
    /// ```
    pub fn result_preview(mut self, preview: impl Into<String>) -> Self {
        self.result_preview = Some(preview.into());
        self
    }

    /// Set whether to show duration
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Write", "file.rs").show_duration(false);
    /// ```
    pub fn show_duration(mut self, show: bool) -> Self {
        self.show_duration = show;
        self
    }

    /// Set whether to show icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Edit", "file.rs").show_icon(false);
    /// ```
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set custom style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    /// use ratatui::style::Style;
    ///
    /// let item = ToolExecutionItem::new("Bash", "cargo test").style(Style::default());
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get tool name
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    /// Get description
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Get execution status
    pub fn get_status(&self) -> ExecutionStatus {
        self.status
    }

    /// Get duration if available
    pub fn get_duration_ms(&self) -> Option<u64> {
        self.duration_ms
    }

    /// Format duration for display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// assert_eq!(ToolExecutionItem::format_duration(125), "125ms");
    /// assert_eq!(ToolExecutionItem::format_duration(1500), "1.50s");
    /// assert_eq!(ToolExecutionItem::format_duration(65000), "1m 5s");
    /// ```
    pub fn format_duration(ms: u64) -> String {
        if ms < 1000 {
            format!("{}ms", ms)
        } else if ms < 60000 {
            format!("{:.2}s", ms as f64 / 1000.0)
        } else {
            let secs = ms / 1000;
            let mins = secs / 60;
            let remaining_secs = secs % 60;
            format!("{}m {}s", mins, remaining_secs)
        }
    }

    /// Get color for current status
    fn get_color(&self) -> ratatui::style::Color {
        if let Some(style) = self.style {
            return style.fg.unwrap_or(ToadTheme::WHITE);
        }

        match self.status {
            ExecutionStatus::Running => ToadTheme::YELLOW,
            ExecutionStatus::Success => ToadTheme::TOAD_GREEN,
            ExecutionStatus::Failed => ToadTheme::RED,
        }
    }

    /// Get icon for current status
    fn get_icon(&self) -> Icon {
        let icon = match self.status {
            ExecutionStatus::Running => Icon::ui(UiIcon::Clock),
            ExecutionStatus::Success => Icon::ui(UiIcon::Success),
            ExecutionStatus::Failed => Icon::ui(UiIcon::Error),
        };
        icon.style(Style::default().fg(self.get_color()))
    }

    /// Convert to styled spans
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Read", "file.rs");
    /// let spans = item.to_spans();
    /// assert!(!spans.is_empty());
    /// ```
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        // Icon
        if self.show_icon {
            let icon = self.get_icon();
            spans.push(icon.to_text().to_span());
            spans.push(Span::raw(" "));
        }

        // Tool name
        let color = self.get_color();
        spans.push(Span::styled(
            self.tool_name.clone(),
            Style::default().fg(color),
        ));
        spans.push(Span::raw(": "));

        // Description
        spans.push(Span::styled(
            self.description.clone(),
            Style::default().fg(ToadTheme::GRAY),
        ));

        // Duration
        if self.show_duration && self.duration_ms.is_some() {
            if let Some(duration) = self.duration_ms {
                let duration_text = format!(" ({})", Self::format_duration(duration));
                spans.push(Span::styled(
                    duration_text,
                    Style::default().fg(ToadTheme::GRAY),
                ));
            }
        }

        // Result preview
        if let Some(ref preview) = self.result_preview {
            spans.push(Span::raw(" → "));
            spans.push(Span::styled(
                preview.clone(),
                Style::default().fg(ToadTheme::WHITE),
            ));
        }

        spans
    }

    /// Convert to line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::tool_execution_item::ToolExecutionItem;
    ///
    /// let item = ToolExecutionItem::new("Write", "src/lib.rs");
    /// let line = item.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

/// Tool execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// Tool is currently executing
    Running,
    /// Tool completed successfully
    Success,
    /// Tool execution failed
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_execution_item_new() {
        let item = ToolExecutionItem::new("Read", "src/main.rs");
        assert_eq!(item.tool_name(), "Read");
        assert_eq!(item.get_description(), "src/main.rs");
        assert_eq!(item.get_status(), ExecutionStatus::Running);
    }

    #[test]
    fn test_tool_execution_item_status() {
        let item = ToolExecutionItem::new("Write", "file.rs").status(ExecutionStatus::Success);
        assert_eq!(item.get_status(), ExecutionStatus::Success);
    }

    #[test]
    fn test_tool_execution_item_duration_ms() {
        let item = ToolExecutionItem::new("Edit", "file.rs").duration_ms(125);
        assert_eq!(item.get_duration_ms(), Some(125));
    }

    #[test]
    fn test_tool_execution_item_result_preview() {
        let item = ToolExecutionItem::new("Read", "file.rs").result_preview("200 lines");
        assert_eq!(item.result_preview, Some("200 lines".to_string()));
    }

    #[test]
    fn test_tool_execution_item_show_duration() {
        let item = ToolExecutionItem::new("Bash", "cmd").show_duration(false);
        assert!(!item.show_duration);
    }

    #[test]
    fn test_tool_execution_item_show_icon() {
        let item = ToolExecutionItem::new("Git", "status").show_icon(false);
        assert!(!item.show_icon);
    }

    #[test]
    fn test_tool_execution_item_style() {
        let style = Style::default().fg(ToadTheme::RED);
        let item = ToolExecutionItem::new("List", "src/").style(style);
        assert_eq!(item.style, Some(style));
    }

    #[test]
    fn test_tool_execution_item_format_duration_ms() {
        assert_eq!(ToolExecutionItem::format_duration(125), "125ms");
        assert_eq!(ToolExecutionItem::format_duration(999), "999ms");
    }

    #[test]
    fn test_tool_execution_item_format_duration_seconds() {
        assert_eq!(ToolExecutionItem::format_duration(1000), "1.00s");
        assert_eq!(ToolExecutionItem::format_duration(1500), "1.50s");
        assert_eq!(ToolExecutionItem::format_duration(59999), "60.00s");
    }

    #[test]
    fn test_tool_execution_item_format_duration_minutes() {
        assert_eq!(ToolExecutionItem::format_duration(60000), "1m 0s");
        assert_eq!(ToolExecutionItem::format_duration(65000), "1m 5s");
        assert_eq!(ToolExecutionItem::format_duration(125000), "2m 5s");
    }

    #[test]
    fn test_tool_execution_item_format_duration_zero() {
        assert_eq!(ToolExecutionItem::format_duration(0), "0ms");
    }

    #[test]
    fn test_tool_execution_item_chaining() {
        let item = ToolExecutionItem::new("Read", "file.rs")
            .status(ExecutionStatus::Success)
            .duration_ms(125)
            .result_preview("100 lines")
            .show_duration(true)
            .show_icon(true);

        assert_eq!(item.tool_name(), "Read");
        assert_eq!(item.get_status(), ExecutionStatus::Success);
        assert_eq!(item.get_duration_ms(), Some(125));
        assert!(item.show_duration);
        assert!(item.show_icon);
    }

    #[test]
    fn test_tool_execution_item_to_spans() {
        let item = ToolExecutionItem::new("Write", "src/lib.rs");
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tool_execution_item_to_line() {
        let item = ToolExecutionItem::new("Edit", "src/main.rs");
        let line = item.to_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_tool_execution_item_clone() {
        let item1 = ToolExecutionItem::new("Read", "file.rs");
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_tool_execution_item_equality() {
        let item1 = ToolExecutionItem::new("Write", "file.rs");
        let item2 = ToolExecutionItem::new("Write", "file.rs");
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_execution_status_equality() {
        assert_eq!(ExecutionStatus::Running, ExecutionStatus::Running);
        assert_ne!(ExecutionStatus::Running, ExecutionStatus::Success);
    }

    #[test]
    fn test_tool_execution_item_running_status() {
        let item = ToolExecutionItem::new("Read", "file.rs").status(ExecutionStatus::Running);
        assert_eq!(item.get_status(), ExecutionStatus::Running);
    }

    #[test]
    fn test_tool_execution_item_success_status() {
        let item = ToolExecutionItem::new("Write", "file.rs").status(ExecutionStatus::Success);
        assert_eq!(item.get_status(), ExecutionStatus::Success);
    }

    #[test]
    fn test_tool_execution_item_failed_status() {
        let item = ToolExecutionItem::new("Edit", "file.rs").status(ExecutionStatus::Failed);
        assert_eq!(item.get_status(), ExecutionStatus::Failed);
    }

    #[test]
    fn test_tool_execution_item_no_duration() {
        let item = ToolExecutionItem::new("Bash", "ls");
        assert_eq!(item.get_duration_ms(), None);
    }

    #[test]
    fn test_tool_execution_item_with_result_preview() {
        let item = ToolExecutionItem::new("Read", "file.rs")
            .status(ExecutionStatus::Success)
            .result_preview("200 lines");

        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tool_execution_item_without_icon() {
        let item = ToolExecutionItem::new("List", "src/").show_icon(false);
        let spans = item.to_spans();
        // Should still have spans for tool name and description
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tool_execution_item_without_duration_display() {
        let item = ToolExecutionItem::new("Git", "status")
            .duration_ms(125)
            .show_duration(false);
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_tool_execution_item_empty_description() {
        let item = ToolExecutionItem::new("Read", "");
        assert_eq!(item.get_description(), "");
    }

    #[test]
    fn test_tool_execution_item_long_description() {
        let desc = "a".repeat(100);
        let item = ToolExecutionItem::new("Write", desc.clone());
        assert_eq!(item.get_description(), desc);
    }

    #[test]
    fn test_tool_execution_item_format_duration_boundary() {
        // Test boundary at 1 second
        assert_eq!(ToolExecutionItem::format_duration(999), "999ms");
        assert_eq!(ToolExecutionItem::format_duration(1000), "1.00s");

        // Test boundary at 1 minute
        assert_eq!(ToolExecutionItem::format_duration(59999), "60.00s");
        assert_eq!(ToolExecutionItem::format_duration(60000), "1m 0s");
    }

    #[test]
    fn test_tool_execution_item_large_duration() {
        // 1 hour
        assert_eq!(ToolExecutionItem::format_duration(3600000), "60m 0s");
    }
}
