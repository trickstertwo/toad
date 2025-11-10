//! CostTracker molecule - Session cost accumulation display
//!
//! Displays accumulated API costs during an AI coding session with budget awareness.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Text + Icon atoms
//! - **Purpose**: Show running cost total with budget warnings
//! - **Visual**: Currency-formatted cost with status indicator
//! - **States**: Normal, Approaching Budget, Over Budget
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::cost_tracker::CostTracker;
//!
//! let tracker = CostTracker::new(0.45);
//! let line = tracker.to_line();
//! ```

use crate::ui::atoms::Icon;
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// Cost tracker for AI session
///
/// Tracks accumulated API costs with optional budget monitoring:
/// - Normal: Within budget
/// - Warning: 80-100% of budget
/// - Over: Exceeded budget
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::cost_tracker::CostTracker;
///
/// let tracker = CostTracker::new(0.45);
/// assert_eq!(tracker.total_cost(), 0.45);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CostTracker {
    /// Total accumulated cost in USD
    total_cost: f64,
    /// Optional budget limit in USD
    budget: Option<f64>,
    /// Show budget percentage
    show_budget_percentage: bool,
    /// Show cost breakdown icon
    show_icon: bool,
    /// Custom style override
    style: Option<Style>,
}

impl CostTracker {
    /// Create a new cost tracker
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45);
    /// ```
    pub fn new(total_cost: f64) -> Self {
        Self {
            total_cost,
            budget: None,
            show_budget_percentage: true,
            show_icon: true,
            style: None,
        }
    }

    /// Set budget limit
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45).budget(1.0);
    /// ```
    pub fn budget(mut self, budget: f64) -> Self {
        self.budget = Some(budget);
        self
    }

    /// Set whether to show budget percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45).show_budget_percentage(false);
    /// ```
    pub fn show_budget_percentage(mut self, show: bool) -> Self {
        self.show_budget_percentage = show;
        self
    }

    /// Set whether to show icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45).show_icon(false);
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
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    /// use ratatui::style::Style;
    ///
    /// let tracker = CostTracker::new(0.45).style(Style::default());
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get total cost
    pub fn total_cost(&self) -> f64 {
        self.total_cost
    }

    /// Get budget if set
    pub fn get_budget(&self) -> Option<f64> {
        self.budget
    }

    /// Calculate budget usage percentage (0.0 - 100.0+)
    ///
    /// Returns None if no budget is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45).budget(1.0);
    /// assert_eq!(tracker.budget_percentage(), Some(45.0));
    /// ```
    pub fn budget_percentage(&self) -> Option<f64> {
        self.budget.map(|budget| {
            if budget == 0.0 {
                0.0
            } else {
                (self.total_cost / budget) * 100.0
            }
        })
    }

    /// Get budget status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::{CostTracker, BudgetStatus};
    ///
    /// let normal = CostTracker::new(0.45).budget(2.0);
    /// assert_eq!(normal.budget_status(), BudgetStatus::Normal);
    ///
    /// let warning = CostTracker::new(0.85).budget(1.0);
    /// assert_eq!(warning.budget_status(), BudgetStatus::Approaching);
    ///
    /// let over = CostTracker::new(1.2).budget(1.0);
    /// assert_eq!(over.budget_status(), BudgetStatus::Over);
    /// ```
    pub fn budget_status(&self) -> BudgetStatus {
        match self.budget_percentage() {
            None => BudgetStatus::Normal,
            Some(pct) if pct >= 100.0 => BudgetStatus::Over,
            Some(pct) if pct >= 80.0 => BudgetStatus::Approaching,
            Some(_) => BudgetStatus::Normal,
        }
    }

    /// Format cost in USD
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// assert_eq!(CostTracker::format_cost(0.45), "$0.45");
    /// assert_eq!(CostTracker::format_cost(1.234), "$1.23");
    /// assert_eq!(CostTracker::format_cost(0.001), "$0.00");
    /// ```
    pub fn format_cost(cost: f64) -> String {
        format!("${:.2}", cost)
    }

    /// Get color for current status
    fn get_color(&self) -> ratatui::style::Color {
        if let Some(style) = self.style {
            return style.fg.unwrap_or(ToadTheme::WHITE);
        }

        match self.budget_status() {
            BudgetStatus::Normal => ToadTheme::TOAD_GREEN,
            BudgetStatus::Approaching => ToadTheme::YELLOW,
            BudgetStatus::Over => ToadTheme::RED,
        }
    }

    /// Get icon for current status
    fn get_icon(&self) -> Icon {
        let icon = match self.budget_status() {
            BudgetStatus::Normal => Icon::ui(UiIcon::Success),
            BudgetStatus::Approaching => Icon::ui(UiIcon::Warning),
            BudgetStatus::Over => Icon::ui(UiIcon::Error),
        };
        icon.style(Style::default().fg(self.get_color()))
    }

    /// Convert to styled spans
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45);
    /// let spans = tracker.to_spans();
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

        // Cost text
        let cost_text = Self::format_cost(self.total_cost);
        let color = self.get_color();
        spans.push(Span::styled(cost_text, Style::default().fg(color)));

        // Budget info
        if let Some(budget) = self.budget {
            let budget_text = format!(" / {}", Self::format_cost(budget));
            spans.push(Span::styled(
                budget_text,
                Style::default().fg(ToadTheme::GRAY),
            ));

            if self.show_budget_percentage && self.budget_percentage().is_some() {
                if let Some(pct) = self.budget_percentage() {
                    let pct_text = format!(" ({:.1}%)", pct);
                    spans.push(Span::styled(pct_text, Style::default().fg(ToadTheme::GRAY)));
                }
            }
        }

        spans
    }

    /// Convert to line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::cost_tracker::CostTracker;
    ///
    /// let tracker = CostTracker::new(0.45);
    /// let line = tracker.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

/// Budget status for cost tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetStatus {
    /// Within budget (< 80%)
    Normal,
    /// Approaching budget (80-100%)
    Approaching,
    /// Over budget (> 100%)
    Over,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_tracker_new() {
        let tracker = CostTracker::new(0.45);
        assert_eq!(tracker.total_cost(), 0.45);
        assert_eq!(tracker.get_budget(), None);
    }

    #[test]
    fn test_cost_tracker_budget() {
        let tracker = CostTracker::new(0.45).budget(1.0);
        assert_eq!(tracker.get_budget(), Some(1.0));
    }

    #[test]
    fn test_cost_tracker_budget_percentage() {
        let tracker = CostTracker::new(0.45).budget(1.0);
        assert_eq!(tracker.budget_percentage(), Some(45.0));
    }

    #[test]
    fn test_cost_tracker_budget_percentage_no_budget() {
        let tracker = CostTracker::new(0.45);
        assert_eq!(tracker.budget_percentage(), None);
    }

    #[test]
    fn test_cost_tracker_budget_percentage_zero_budget() {
        let tracker = CostTracker::new(0.45).budget(0.0);
        assert_eq!(tracker.budget_percentage(), Some(0.0));
    }

    #[test]
    fn test_cost_tracker_budget_status_normal() {
        let tracker = CostTracker::new(0.45).budget(2.0);
        assert_eq!(tracker.budget_status(), BudgetStatus::Normal);
    }

    #[test]
    fn test_cost_tracker_budget_status_approaching() {
        let tracker = CostTracker::new(0.85).budget(1.0);
        assert_eq!(tracker.budget_status(), BudgetStatus::Approaching);
    }

    #[test]
    fn test_cost_tracker_budget_status_over() {
        let tracker = CostTracker::new(1.2).budget(1.0);
        assert_eq!(tracker.budget_status(), BudgetStatus::Over);
    }

    #[test]
    fn test_cost_tracker_budget_status_no_budget() {
        let tracker = CostTracker::new(0.45);
        assert_eq!(tracker.budget_status(), BudgetStatus::Normal);
    }

    #[test]
    fn test_cost_tracker_budget_status_boundary_80() {
        let tracker = CostTracker::new(0.8).budget(1.0);
        assert_eq!(tracker.budget_status(), BudgetStatus::Approaching);
    }

    #[test]
    fn test_cost_tracker_budget_status_boundary_100() {
        let tracker = CostTracker::new(1.0).budget(1.0);
        assert_eq!(tracker.budget_status(), BudgetStatus::Over);
    }

    #[test]
    fn test_cost_tracker_format_cost() {
        assert_eq!(CostTracker::format_cost(0.45), "$0.45");
        assert_eq!(CostTracker::format_cost(1.234), "$1.23");
        assert_eq!(CostTracker::format_cost(10.5), "$10.50");
    }

    #[test]
    fn test_cost_tracker_format_cost_small() {
        assert_eq!(CostTracker::format_cost(0.001), "$0.00");
        assert_eq!(CostTracker::format_cost(0.005), "$0.01");
    }

    #[test]
    fn test_cost_tracker_format_cost_large() {
        assert_eq!(CostTracker::format_cost(123.456), "$123.46");
        assert_eq!(CostTracker::format_cost(1000.0), "$1000.00");
    }

    #[test]
    fn test_cost_tracker_format_cost_zero() {
        assert_eq!(CostTracker::format_cost(0.0), "$0.00");
    }

    #[test]
    fn test_cost_tracker_show_budget_percentage() {
        let tracker = CostTracker::new(0.45).show_budget_percentage(false);
        assert!(!tracker.show_budget_percentage);
    }

    #[test]
    fn test_cost_tracker_show_icon() {
        let tracker = CostTracker::new(0.45).show_icon(false);
        assert!(!tracker.show_icon);
    }

    #[test]
    fn test_cost_tracker_style() {
        let style = Style::default().fg(ToadTheme::RED);
        let tracker = CostTracker::new(0.45).style(style);
        assert_eq!(tracker.style, Some(style));
    }

    #[test]
    fn test_cost_tracker_chaining() {
        let tracker = CostTracker::new(0.45)
            .budget(1.0)
            .show_budget_percentage(false)
            .show_icon(true);

        assert_eq!(tracker.total_cost(), 0.45);
        assert_eq!(tracker.get_budget(), Some(1.0));
        assert!(!tracker.show_budget_percentage);
        assert!(tracker.show_icon);
    }

    #[test]
    fn test_cost_tracker_to_spans() {
        let tracker = CostTracker::new(0.45);
        let spans = tracker.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_cost_tracker_to_line() {
        let tracker = CostTracker::new(0.45);
        let line = tracker.to_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_cost_tracker_clone() {
        let tracker1 = CostTracker::new(0.45);
        let tracker2 = tracker1.clone();
        assert_eq!(tracker1, tracker2);
    }

    #[test]
    fn test_cost_tracker_equality() {
        let tracker1 = CostTracker::new(0.45);
        let tracker2 = CostTracker::new(0.45);
        assert_eq!(tracker1, tracker2);
    }

    #[test]
    fn test_budget_status_equality() {
        assert_eq!(BudgetStatus::Normal, BudgetStatus::Normal);
        assert_ne!(BudgetStatus::Normal, BudgetStatus::Over);
    }

    #[test]
    fn test_cost_tracker_zero_cost() {
        let tracker = CostTracker::new(0.0);
        assert_eq!(tracker.total_cost(), 0.0);
    }

    #[test]
    fn test_cost_tracker_high_cost() {
        let tracker = CostTracker::new(999.99);
        assert_eq!(tracker.total_cost(), 999.99);
    }

    #[test]
    fn test_cost_tracker_over_budget() {
        let tracker = CostTracker::new(2.5).budget(1.0);
        assert_eq!(tracker.budget_percentage(), Some(250.0));
        assert_eq!(tracker.budget_status(), BudgetStatus::Over);
    }

    #[test]
    fn test_cost_tracker_exact_budget() {
        let tracker = CostTracker::new(1.0).budget(1.0);
        assert_eq!(tracker.budget_percentage(), Some(100.0));
        assert_eq!(tracker.budget_status(), BudgetStatus::Over);
    }

    #[test]
    fn test_cost_tracker_negative_cost() {
        let tracker = CostTracker::new(-0.5);
        assert_eq!(tracker.total_cost(), -0.5);
    }

    #[test]
    fn test_cost_tracker_high_precision_cost() {
        let tracker = CostTracker::new(0.123456789);
        assert_eq!(tracker.total_cost(), 0.123456789);
        // Format should round to 2 decimals
        assert_eq!(CostTracker::format_cost(0.123456789), "$0.12");
    }

    #[test]
    fn test_cost_tracker_budget_edge_cases() {
        // Just under 80%
        let tracker1 = CostTracker::new(0.79).budget(1.0);
        assert_eq!(tracker1.budget_status(), BudgetStatus::Normal);

        // Just under 100%
        let tracker2 = CostTracker::new(0.99).budget(1.0);
        assert_eq!(tracker2.budget_status(), BudgetStatus::Approaching);
    }
}
