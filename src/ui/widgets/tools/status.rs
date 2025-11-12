//! Tool execution status panel
//!
//! Displays real-time status of AI tool executions with visual indicators.
//!
//! # Features
//!
//! - Visual indicators: ⏳ Queued, ⟳ Running, ✓ Complete, ❌ Error
//! - Duration tracking for completed tools
//! - Scrollable log of all tool executions
//! - Color-coded status
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::widgets::tools::ToolStatusPanel;
//! use toad::core::event::ToolExecution;
//!
//! let mut panel = ToolStatusPanel::new();
//! // panel.add_execution(execution);
//! ```

use crate::core::event::ToolExecution;
use crate::ui::atoms::Block;
use crate::ui::theme::ToadTheme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
    Frame,
};

/// Status of a tool execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolStatus {
    /// Tool is queued but not yet running
    Queued,
    /// Tool is currently executing
    Running,
    /// Tool completed successfully
    Complete,
    /// Tool failed with error
    Error,
}

impl ToolStatus {
    /// Get the visual indicator for this status
    pub fn indicator(&self) -> &'static str {
        match self {
            ToolStatus::Queued => "⏳",
            ToolStatus::Running => "⟳",
            ToolStatus::Complete => "✓",
            ToolStatus::Error => "❌",
        }
    }

    /// Get the color for this status
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            ToolStatus::Queued => ToadTheme::GRAY,
            ToolStatus::Running => ToadTheme::TOAD_GREEN_BRIGHT,
            ToolStatus::Complete => ToadTheme::TOAD_GREEN,
            ToolStatus::Error => ToadTheme::RED,
        }
    }
}

/// Tool execution record with computed status
#[derive(Debug, Clone)]
pub struct ToolExecutionRecord {
    /// The underlying tool execution data
    pub execution: ToolExecution,
    /// Computed status
    pub status: ToolStatus,
}

impl ToolExecutionRecord {
    /// Create a new record from execution data
    pub fn new(execution: ToolExecution) -> Self {
        let status = if execution.success {
            ToolStatus::Complete
        } else {
            ToolStatus::Error
        };

        Self { execution, status }
    }

    /// Format duration as human-readable string
    pub fn duration_str(&self) -> String {
        let ms = self.execution.duration_ms;
        if ms < 1000 {
            format!("{}ms", ms)
        } else {
            format!("{:.1}s", ms as f64 / 1000.0)
        }
    }

    /// Get truncated output for display
    pub fn output_preview(&self) -> String {
        const MAX_LEN: usize = 50;
        let output = &self.execution.output;

        if output.len() <= MAX_LEN {
            output.clone()
        } else {
            format!("{}...", &output[..MAX_LEN])
        }
    }
}

/// Tool status panel widget
///
/// Displays a scrollable list of tool executions with status indicators.
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::tools::ToolStatusPanel;
///
/// let panel = ToolStatusPanel::new();
/// assert_eq!(panel.execution_count(), 0);
/// ```
#[derive(Debug)]
pub struct ToolStatusPanel {
    /// All tool execution records
    executions: Vec<ToolExecutionRecord>,
    /// Scroll state
    scroll_state: ScrollbarState,
    /// Scroll offset (number of items scrolled)
    scroll_offset: usize,
    /// Currently running tool name (if any)
    current_tool: Option<String>,
}

impl ToolStatusPanel {
    /// Create a new tool status panel
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::tools::ToolStatusPanel;
    ///
    /// let panel = ToolStatusPanel::new();
    /// ```
    pub fn new() -> Self {
        Self {
            executions: Vec::new(),
            scroll_state: ScrollbarState::default(),
            scroll_offset: 0,
            current_tool: None,
        }
    }

    /// Add a tool execution record
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::widgets::tools::ToolStatusPanel;
    /// use toad::core::event::ToolExecution;
    ///
    /// let mut panel = ToolStatusPanel::new();
    /// // panel.add_execution(execution);
    /// ```
    pub fn add_execution(&mut self, execution: ToolExecution) {
        let record = ToolExecutionRecord::new(execution);
        self.executions.push(record);

        // Update scroll state
        self.scroll_state = ScrollbarState::new(self.executions.len());

        // Auto-scroll to bottom to show latest
        self.scroll_to_bottom();
    }

    /// Mark a tool as currently running
    pub fn set_running_tool(&mut self, tool_name: String) {
        self.current_tool = Some(tool_name);
    }

    /// Clear the currently running tool
    pub fn clear_running_tool(&mut self) {
        self.current_tool = None;
    }

    /// Get the number of executions
    pub fn execution_count(&self) -> usize {
        self.executions.len()
    }

    /// Clear all executions
    pub fn clear(&mut self) {
        self.executions.clear();
        self.scroll_offset = 0;
        self.scroll_state = ScrollbarState::default();
        self.current_tool = None;
    }

    /// Scroll up by one item
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scroll down by one item
    pub fn scroll_down(&mut self) {
        let max_offset = self.executions.len().saturating_sub(1);
        self.scroll_offset = self.scroll_offset.saturating_add(1).min(max_offset);
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.executions.len().saturating_sub(1);
    }

    /// Render the tool status panel
    ///
    /// # Display Format
    ///
    /// ```text
    /// ┌─ Tool Executions ─────────────────┐
    /// │ Status | Tool   | Duration | Result│
    /// │ ✓      | Read   | 45ms     | OK    │
    /// │ ⟳      | Write  | ...      | ...   │
    /// │ ❌      | Bash   | 1.2s     | Error │
    /// └───────────────────────────────────┘
    /// ```
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Create border
        let title = if let Some(ref tool) = self.current_tool {
            format!("Tool Executions (running: {})", tool)
        } else {
            "Tool Executions".to_string()
        };

        let block = Block::themed(&title).to_ratatui();
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // If no executions, show empty state
        if self.executions.is_empty() {
            let empty_text = Paragraph::new(Line::from(vec![
                Span::styled(
                    "No tool executions yet",
                    Style::default()
                        .fg(ToadTheme::GRAY)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));
            frame.render_widget(empty_text, inner);
            return;
        }

        // Create table rows
        let rows: Vec<Row> = self
            .executions
            .iter()
            .skip(self.scroll_offset)
            .take(inner.height as usize)
            .map(|record| {
                let status_span = Span::styled(
                    record.status.indicator(),
                    Style::default().fg(record.status.color()),
                );

                let tool_span = Span::raw(&record.execution.tool_name);

                let duration_span = Span::raw(record.duration_str());

                let result_span = if record.status == ToolStatus::Error {
                    Span::styled(
                        record.execution.error.as_deref().unwrap_or("Error"),
                        Style::default().fg(ToadTheme::RED),
                    )
                } else {
                    Span::styled(
                        record.output_preview(),
                        Style::default().fg(ToadTheme::FOREGROUND),
                    )
                };

                Row::new(vec![status_span, tool_span, duration_span, result_span])
            })
            .collect();

        // Create table
        let table = Table::new(
            rows,
            [
                Constraint::Length(3),  // Status indicator
                Constraint::Length(10), // Tool name
                Constraint::Length(8),  // Duration
                Constraint::Min(20),    // Result/output
            ],
        )
        .header(
            Row::new(vec!["", "Tool", "Duration", "Result"])
                .style(Style::default().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        );

        frame.render_widget(table, inner);

        // Render scrollbar if needed
        if self.executions.len() > inner.height as usize {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            self.scroll_state = self.scroll_state
                .position(self.scroll_offset)
                .viewport_content_length(inner.height as usize);

            frame.render_stateful_widget(scrollbar, inner, &mut self.scroll_state);
        }
    }
}

impl Default for ToolStatusPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_execution(tool_name: &str, success: bool, duration_ms: u64) -> ToolExecution {
        ToolExecution {
            tool_name: tool_name.to_string(),
            input: serde_json::json!({}),
            output: "Test output".to_string(),
            success,
            error: if success { None } else { Some("Test error".to_string()) },
            duration_ms,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_tool_status_panel_new() {
        let panel = ToolStatusPanel::new();
        assert_eq!(panel.execution_count(), 0);
    }

    #[test]
    fn test_add_execution() {
        let mut panel = ToolStatusPanel::new();
        let exec = create_test_execution("Read", true, 100);

        panel.add_execution(exec);
        assert_eq!(panel.execution_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut panel = ToolStatusPanel::new();
        panel.add_execution(create_test_execution("Read", true, 100));
        panel.add_execution(create_test_execution("Write", true, 200));

        panel.clear();
        assert_eq!(panel.execution_count(), 0);
    }

    #[test]
    fn test_set_running_tool() {
        let mut panel = ToolStatusPanel::new();
        panel.set_running_tool("Bash".to_string());
        assert_eq!(panel.current_tool, Some("Bash".to_string()));

        panel.clear_running_tool();
        assert_eq!(panel.current_tool, None);
    }

    #[test]
    fn test_scroll_operations() {
        let mut panel = ToolStatusPanel::new();
        for i in 0..10 {
            panel.add_execution(create_test_execution(&format!("Tool{}", i), true, 100));
        }

        panel.scroll_offset = 5;
        panel.scroll_up();
        assert_eq!(panel.scroll_offset, 4);

        panel.scroll_down();
        assert_eq!(panel.scroll_offset, 5);

        panel.scroll_to_bottom();
        assert_eq!(panel.scroll_offset, 9);
    }

    #[test]
    fn test_tool_status_indicator() {
        assert_eq!(ToolStatus::Queued.indicator(), "⏳");
        assert_eq!(ToolStatus::Running.indicator(), "⟳");
        assert_eq!(ToolStatus::Complete.indicator(), "✓");
        assert_eq!(ToolStatus::Error.indicator(), "❌");
    }

    #[test]
    fn test_duration_formatting() {
        let exec_ms = create_test_execution("Test", true, 500);
        let record_ms = ToolExecutionRecord::new(exec_ms);
        assert_eq!(record_ms.duration_str(), "500ms");

        let exec_s = create_test_execution("Test", true, 1500);
        let record_s = ToolExecutionRecord::new(exec_s);
        assert_eq!(record_s.duration_str(), "1.5s");
    }

    #[test]
    fn test_output_preview_truncation() {
        let mut exec = create_test_execution("Test", true, 100);
        exec.output = "a".repeat(100);

        let record = ToolExecutionRecord::new(exec);
        let preview = record.output_preview();

        assert!(preview.len() <= 53); // 50 chars + "..."
        assert!(preview.ends_with("..."));
    }
}
