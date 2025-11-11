//! AgentStepItem molecule - Agent reasoning step display
//!
//! Displays individual agent reasoning steps during autonomous execution with thinking preview.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Icon + Text atoms
//! - **Purpose**: Show agent step number, action, and reasoning
//! - **Visual**: Step counter, action description, optional thinking preview
//! - **States**: Pending, In Progress, Completed
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::agent_step_item::{AgentStepItem, StepStatus};
//!
//! let item = AgentStepItem::new(1, "Reading file src/main.rs")
//!     .status(StepStatus::Completed)
//!     .thinking("Need to check imports");
//! let line = item.to_line();
//! ```

use crate::ui::atoms::Icon;
use crate::ui::primitives::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

/// Agent reasoning step item display
///
/// Shows individual agent step with:
/// - Step number and status
/// - Action description
/// - Optional thinking/reasoning preview
/// - Color-coded status indicators
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::agent_step_item::{AgentStepItem, StepStatus};
///
/// let item = AgentStepItem::new(1, "Analyzing code structure");
/// assert_eq!(item.step_number(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AgentStepItem {
    /// Step number (1-indexed)
    step_number: usize,
    /// Action description
    action: String,
    /// Step status
    status: StepStatus,
    /// Optional thinking/reasoning preview
    thinking: Option<String>,
    /// Show step number
    show_number: bool,
    /// Show thinking preview
    show_thinking: bool,
    /// Show icon
    show_icon: bool,
    /// Custom style override
    style: Option<Style>,
}

impl AgentStepItem {
    /// Create a new agent step item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "Reading configuration");
    /// ```
    pub fn new(step_number: usize, action: impl Into<String>) -> Self {
        Self {
            step_number,
            action: action.into(),
            status: StepStatus::Pending,
            thinking: None,
            show_number: true,
            show_thinking: true,
            show_icon: true,
            style: None,
        }
    }

    /// Set step status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::{AgentStepItem, StepStatus};
    ///
    /// let item = AgentStepItem::new(1, "action").status(StepStatus::InProgress);
    /// ```
    pub fn status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }

    /// Set thinking/reasoning preview
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "action").thinking("Need to verify...");
    /// ```
    pub fn thinking(mut self, thinking: impl Into<String>) -> Self {
        self.thinking = Some(thinking.into());
        self
    }

    /// Set whether to show step number
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "action").show_number(false);
    /// ```
    pub fn show_number(mut self, show: bool) -> Self {
        self.show_number = show;
        self
    }

    /// Set whether to show thinking preview
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "action").show_thinking(false);
    /// ```
    pub fn show_thinking(mut self, show: bool) -> Self {
        self.show_thinking = show;
        self
    }

    /// Set whether to show icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "action").show_icon(false);
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
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    /// use ratatui::style::Style;
    ///
    /// let item = AgentStepItem::new(1, "action").style(Style::default());
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get step number
    pub fn step_number(&self) -> usize {
        self.step_number
    }

    /// Get action description
    pub fn get_action(&self) -> &str {
        &self.action
    }

    /// Get step status
    pub fn get_status(&self) -> StepStatus {
        self.status
    }

    /// Get thinking preview if available
    pub fn get_thinking(&self) -> Option<&str> {
        self.thinking.as_deref()
    }

    /// Get color for current status
    fn get_color(&self) -> ratatui::style::Color {
        if let Some(style) = self.style {
            return style.fg.unwrap_or(ToadTheme::WHITE);
        }

        match self.status {
            StepStatus::Pending => ToadTheme::GRAY,
            StepStatus::InProgress => ToadTheme::YELLOW,
            StepStatus::Completed => ToadTheme::TOAD_GREEN,
        }
    }

    /// Get icon for current status
    fn get_icon(&self) -> Icon {
        let icon = match self.status {
            StepStatus::Pending => Icon::ui(UiIcon::Clock),
            StepStatus::InProgress => Icon::ui(UiIcon::Loading),
            StepStatus::Completed => Icon::ui(UiIcon::Success),
        };
        icon.style(Style::default().fg(self.get_color()))
    }

    /// Convert to styled spans
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "action");
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

        // Step number
        if self.show_number {
            let color = self.get_color();
            let number_text = format!("[Step {}]", self.step_number);
            spans.push(Span::styled(
                number_text,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw(" "));
        }

        // Action
        let color = self.get_color();
        spans.push(Span::styled(
            self.action.clone(),
            Style::default().fg(color),
        ));

        // Thinking preview
        if self.show_thinking && self.thinking.is_some() {
            if let Some(ref thinking) = self.thinking {
                spans.push(Span::raw(" → "));
                spans.push(Span::styled(
                    format!("\"{}\"", thinking),
                    Style::default()
                        .fg(ToadTheme::GRAY)
                        .add_modifier(Modifier::ITALIC),
                ));
            }
        }

        spans
    }

    /// Convert to line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::agent_step_item::AgentStepItem;
    ///
    /// let item = AgentStepItem::new(1, "Reading file");
    /// let line = item.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

/// Agent step status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    /// Step is pending execution
    Pending,
    /// Step is currently executing
    InProgress,
    /// Step completed successfully
    Completed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_step_item_new() {
        let item = AgentStepItem::new(1, "Reading config");
        assert_eq!(item.step_number(), 1);
        assert_eq!(item.get_action(), "Reading config");
        assert_eq!(item.get_status(), StepStatus::Pending);
    }

    #[test]
    fn test_agent_step_item_status() {
        let item = AgentStepItem::new(1, "action").status(StepStatus::InProgress);
        assert_eq!(item.get_status(), StepStatus::InProgress);
    }

    #[test]
    fn test_agent_step_item_thinking() {
        let item = AgentStepItem::new(1, "action").thinking("Need to verify");
        assert_eq!(item.get_thinking(), Some("Need to verify"));
    }

    #[test]
    fn test_agent_step_item_show_number() {
        let item = AgentStepItem::new(1, "action").show_number(false);
        assert!(!item.show_number);
    }

    #[test]
    fn test_agent_step_item_show_thinking() {
        let item = AgentStepItem::new(1, "action").show_thinking(false);
        assert!(!item.show_thinking);
    }

    #[test]
    fn test_agent_step_item_show_icon() {
        let item = AgentStepItem::new(1, "action").show_icon(false);
        assert!(!item.show_icon);
    }

    #[test]
    fn test_agent_step_item_style() {
        let style = Style::default().fg(ToadTheme::RED);
        let item = AgentStepItem::new(1, "action").style(style);
        assert_eq!(item.style, Some(style));
    }

    #[test]
    fn test_agent_step_item_chaining() {
        let item = AgentStepItem::new(5, "Analyzing code")
            .status(StepStatus::Completed)
            .thinking("Looks correct")
            .show_number(true)
            .show_thinking(true);

        assert_eq!(item.step_number(), 5);
        assert_eq!(item.get_status(), StepStatus::Completed);
        assert_eq!(item.get_thinking(), Some("Looks correct"));
        assert!(item.show_number);
        assert!(item.show_thinking);
    }

    #[test]
    fn test_agent_step_item_to_spans() {
        let item = AgentStepItem::new(1, "Reading file");
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_to_line() {
        let item = AgentStepItem::new(1, "Writing output");
        let line = item.to_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_clone() {
        let item1 = AgentStepItem::new(1, "action");
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_agent_step_item_equality() {
        let item1 = AgentStepItem::new(1, "action");
        let item2 = AgentStepItem::new(1, "action");
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_step_status_equality() {
        assert_eq!(StepStatus::Pending, StepStatus::Pending);
        assert_ne!(StepStatus::Pending, StepStatus::InProgress);
    }

    #[test]
    fn test_agent_step_item_pending_status() {
        let item = AgentStepItem::new(1, "action").status(StepStatus::Pending);
        assert_eq!(item.get_status(), StepStatus::Pending);
    }

    #[test]
    fn test_agent_step_item_in_progress_status() {
        let item = AgentStepItem::new(1, "action").status(StepStatus::InProgress);
        assert_eq!(item.get_status(), StepStatus::InProgress);
    }

    #[test]
    fn test_agent_step_item_completed_status() {
        let item = AgentStepItem::new(1, "action").status(StepStatus::Completed);
        assert_eq!(item.get_status(), StepStatus::Completed);
    }

    #[test]
    fn test_agent_step_item_no_thinking() {
        let item = AgentStepItem::new(1, "action");
        assert_eq!(item.get_thinking(), None);
    }

    #[test]
    fn test_agent_step_item_with_thinking() {
        let item = AgentStepItem::new(1, "action").thinking("Checking requirements");
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_without_number() {
        let item = AgentStepItem::new(1, "action").show_number(false);
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_without_icon() {
        let item = AgentStepItem::new(1, "action").show_icon(false);
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_without_thinking_display() {
        let item = AgentStepItem::new(1, "action")
            .thinking("Some thought")
            .show_thinking(false);
        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_empty_action() {
        let item = AgentStepItem::new(1, "");
        assert_eq!(item.get_action(), "");
    }

    #[test]
    fn test_agent_step_item_long_action() {
        let action = "a".repeat(200);
        let item = AgentStepItem::new(1, action.clone());
        assert_eq!(item.get_action(), action);
    }

    #[test]
    fn test_agent_step_item_step_number_zero() {
        let item = AgentStepItem::new(0, "action");
        assert_eq!(item.step_number(), 0);
    }

    #[test]
    fn test_agent_step_item_step_number_large() {
        let item = AgentStepItem::new(999, "action");
        assert_eq!(item.step_number(), 999);
    }

    #[test]
    fn test_agent_step_item_long_thinking() {
        let thinking = "a".repeat(300);
        let item = AgentStepItem::new(1, "action").thinking(thinking.clone());
        assert_eq!(item.get_thinking(), Some(thinking.as_str()));
    }

    #[test]
    fn test_agent_step_item_minimal() {
        let item = AgentStepItem::new(1, "action")
            .show_number(false)
            .show_icon(false)
            .show_thinking(false);

        let spans = item.to_spans();
        // Should still have action span
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_agent_step_item_full_featured() {
        let item = AgentStepItem::new(3, "Analyzing dependencies")
            .status(StepStatus::InProgress)
            .thinking("Need to check version compatibility")
            .show_number(true)
            .show_icon(true)
            .show_thinking(true);

        assert_eq!(item.step_number(), 3);
        assert_eq!(item.get_action(), "Analyzing dependencies");
        assert_eq!(item.get_status(), StepStatus::InProgress);
        assert!(item.get_thinking().is_some());

        let spans = item.to_spans();
        assert!(!spans.is_empty());
    }
}
