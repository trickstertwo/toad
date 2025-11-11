//! APICallMetrics molecule - API monitoring display
//!
//! Displays API call statistics with rate limiting awareness and throttling indicators.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Icon + Text atoms
//! - **Purpose**: Show API call rates, limits, and throttling status
//! - **Visual**: Calls/min, limit status, throttle warnings
//! - **States**: Normal, Approaching Limit, Throttled
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::api_call_metrics::{APICallMetrics, ThrottleStatus};
//!
//! let metrics = APICallMetrics::new(45, 50)
//!     .status(ThrottleStatus::Normal)
//!     .show_percentage(true);
//! let line = metrics.to_line();
//! ```

use crate::ui::atoms::Icon;
use crate::ui::primitives::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// API call metrics display
///
/// Shows API call statistics with rate limiting:
/// - Current calls per minute
/// - Rate limit (calls/min)
/// - Usage percentage
/// - Throttle status indicators
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::api_call_metrics::APICallMetrics;
///
/// let metrics = APICallMetrics::new(30, 50);
/// assert_eq!(metrics.current_calls(), 30);
/// assert_eq!(metrics.rate_limit(), 50);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct APICallMetrics {
    /// Current calls per minute
    current_calls: u32,
    /// Rate limit (calls per minute)
    rate_limit: u32,
    /// Throttle status
    status: ThrottleStatus,
    /// Show usage percentage
    show_percentage: bool,
    /// Show icon
    show_icon: bool,
    /// Custom style override
    style: Option<Style>,
}

impl APICallMetrics {
    /// Create new API call metrics
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    ///
    /// let metrics = APICallMetrics::new(30, 50);
    /// ```
    pub fn new(current_calls: u32, rate_limit: u32) -> Self {
        Self {
            current_calls,
            rate_limit,
            status: ThrottleStatus::Normal,
            show_percentage: true,
            show_icon: true,
            style: None,
        }
    }

    /// Set throttle status
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::{APICallMetrics, ThrottleStatus};
    ///
    /// let metrics = APICallMetrics::new(45, 50)
    ///     .status(ThrottleStatus::ApproachingLimit);
    /// ```
    pub fn status(mut self, status: ThrottleStatus) -> Self {
        self.status = status;
        self
    }

    /// Set whether to show percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    ///
    /// let metrics = APICallMetrics::new(30, 50).show_percentage(false);
    /// ```
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set whether to show icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    ///
    /// let metrics = APICallMetrics::new(30, 50).show_icon(false);
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
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    /// use ratatui::style::Style;
    ///
    /// let metrics = APICallMetrics::new(30, 50).style(Style::default());
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get current calls per minute
    pub fn current_calls(&self) -> u32 {
        self.current_calls
    }

    /// Get rate limit
    pub fn rate_limit(&self) -> u32 {
        self.rate_limit
    }

    /// Get throttle status
    pub fn get_status(&self) -> ThrottleStatus {
        self.status
    }

    /// Calculate usage percentage (0.0 - 100.0+)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    ///
    /// let metrics = APICallMetrics::new(30, 50);
    /// assert_eq!(metrics.usage_percentage(), 60.0);
    /// ```
    pub fn usage_percentage(&self) -> f64 {
        if self.rate_limit == 0 {
            0.0
        } else {
            (self.current_calls as f64 / self.rate_limit as f64) * 100.0
        }
    }

    /// Determine throttle status from usage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::{APICallMetrics, ThrottleStatus};
    ///
    /// let normal = APICallMetrics::new(30, 100);
    /// assert_eq!(normal.determine_status(), ThrottleStatus::Normal);
    ///
    /// let approaching = APICallMetrics::new(85, 100);
    /// assert_eq!(approaching.determine_status(), ThrottleStatus::ApproachingLimit);
    ///
    /// let at_limit = APICallMetrics::new(100, 100);
    /// assert_eq!(at_limit.determine_status(), ThrottleStatus::Throttled);
    /// ```
    pub fn determine_status(&self) -> ThrottleStatus {
        let pct = self.usage_percentage();
        if pct >= 100.0 {
            ThrottleStatus::Throttled
        } else if pct >= 80.0 {
            ThrottleStatus::ApproachingLimit
        } else {
            ThrottleStatus::Normal
        }
    }

    /// Get color for current status
    fn get_color(&self) -> ratatui::style::Color {
        if let Some(style) = self.style {
            return style.fg.unwrap_or(ToadTheme::WHITE);
        }

        match self.status {
            ThrottleStatus::Normal => ToadTheme::TOAD_GREEN,
            ThrottleStatus::ApproachingLimit => ToadTheme::YELLOW,
            ThrottleStatus::Throttled => ToadTheme::RED,
        }
    }

    /// Get icon for current status
    fn get_icon(&self) -> Icon {
        let icon = match self.status {
            ThrottleStatus::Normal => Icon::ui(UiIcon::Success),
            ThrottleStatus::ApproachingLimit => Icon::ui(UiIcon::Warning),
            ThrottleStatus::Throttled => Icon::ui(UiIcon::Error),
        };
        icon.style(Style::default().fg(self.get_color()))
    }

    /// Convert to styled spans
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    ///
    /// let metrics = APICallMetrics::new(30, 50);
    /// let spans = metrics.to_spans();
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

        // Calls count
        let color = self.get_color();
        let calls_text = format!("{} / {} calls/min", self.current_calls, self.rate_limit);
        spans.push(Span::styled(calls_text, Style::default().fg(color)));

        // Percentage
        if self.show_percentage {
            let pct_text = format!(" ({:.0}%)", self.usage_percentage());
            spans.push(Span::styled(pct_text, Style::default().fg(ToadTheme::GRAY)));
        }

        spans
    }

    /// Convert to line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::api_call_metrics::APICallMetrics;
    ///
    /// let metrics = APICallMetrics::new(30, 50);
    /// let line = metrics.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

/// API throttle status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThrottleStatus {
    /// Normal usage (< 80% of rate limit)
    Normal,
    /// Approaching rate limit (80-100%)
    ApproachingLimit,
    /// At or over rate limit (throttled)
    Throttled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_call_metrics_new() {
        let metrics = APICallMetrics::new(30, 50);
        assert_eq!(metrics.current_calls(), 30);
        assert_eq!(metrics.rate_limit(), 50);
        assert_eq!(metrics.get_status(), ThrottleStatus::Normal);
    }

    #[test]
    fn test_api_call_metrics_status() {
        let metrics = APICallMetrics::new(30, 50).status(ThrottleStatus::ApproachingLimit);
        assert_eq!(metrics.get_status(), ThrottleStatus::ApproachingLimit);
    }

    #[test]
    fn test_api_call_metrics_show_percentage() {
        let metrics = APICallMetrics::new(30, 50).show_percentage(false);
        assert!(!metrics.show_percentage);
    }

    #[test]
    fn test_api_call_metrics_show_icon() {
        let metrics = APICallMetrics::new(30, 50).show_icon(false);
        assert!(!metrics.show_icon);
    }

    #[test]
    fn test_api_call_metrics_style() {
        let style = Style::default().fg(ToadTheme::RED);
        let metrics = APICallMetrics::new(30, 50).style(style);
        assert_eq!(metrics.style, Some(style));
    }

    #[test]
    fn test_api_call_metrics_usage_percentage() {
        let metrics = APICallMetrics::new(30, 50);
        assert_eq!(metrics.usage_percentage(), 60.0);
    }

    #[test]
    fn test_api_call_metrics_usage_percentage_zero_limit() {
        let metrics = APICallMetrics::new(30, 0);
        assert_eq!(metrics.usage_percentage(), 0.0);
    }

    #[test]
    fn test_api_call_metrics_usage_percentage_full() {
        let metrics = APICallMetrics::new(50, 50);
        assert_eq!(metrics.usage_percentage(), 100.0);
    }

    #[test]
    fn test_api_call_metrics_determine_status_normal() {
        let metrics = APICallMetrics::new(30, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Normal);
    }

    #[test]
    fn test_api_call_metrics_determine_status_approaching() {
        let metrics = APICallMetrics::new(85, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::ApproachingLimit);
    }

    #[test]
    fn test_api_call_metrics_determine_status_throttled() {
        let metrics = APICallMetrics::new(100, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Throttled);
    }

    #[test]
    fn test_api_call_metrics_determine_status_over_limit() {
        let metrics = APICallMetrics::new(120, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Throttled);
    }

    #[test]
    fn test_api_call_metrics_determine_status_boundary_80() {
        let metrics = APICallMetrics::new(80, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::ApproachingLimit);
    }

    #[test]
    fn test_api_call_metrics_determine_status_boundary_100() {
        let metrics = APICallMetrics::new(100, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Throttled);
    }

    #[test]
    fn test_api_call_metrics_chaining() {
        let metrics = APICallMetrics::new(45, 50)
            .status(ThrottleStatus::ApproachingLimit)
            .show_percentage(true)
            .show_icon(true);

        assert_eq!(metrics.current_calls(), 45);
        assert_eq!(metrics.rate_limit(), 50);
        assert_eq!(metrics.get_status(), ThrottleStatus::ApproachingLimit);
        assert!(metrics.show_percentage);
        assert!(metrics.show_icon);
    }

    #[test]
    fn test_api_call_metrics_to_spans() {
        let metrics = APICallMetrics::new(30, 50);
        let spans = metrics.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_api_call_metrics_to_line() {
        let metrics = APICallMetrics::new(30, 50);
        let line = metrics.to_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_api_call_metrics_clone() {
        let metrics1 = APICallMetrics::new(30, 50);
        let metrics2 = metrics1.clone();
        assert_eq!(metrics1, metrics2);
    }

    #[test]
    fn test_api_call_metrics_equality() {
        let metrics1 = APICallMetrics::new(30, 50);
        let metrics2 = APICallMetrics::new(30, 50);
        assert_eq!(metrics1, metrics2);
    }

    #[test]
    fn test_throttle_status_equality() {
        assert_eq!(ThrottleStatus::Normal, ThrottleStatus::Normal);
        assert_ne!(ThrottleStatus::Normal, ThrottleStatus::Throttled);
    }

    #[test]
    fn test_api_call_metrics_zero_calls() {
        let metrics = APICallMetrics::new(0, 50);
        assert_eq!(metrics.usage_percentage(), 0.0);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Normal);
    }

    #[test]
    fn test_api_call_metrics_high_calls() {
        let metrics = APICallMetrics::new(500, 100);
        assert_eq!(metrics.usage_percentage(), 500.0);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Throttled);
    }

    #[test]
    fn test_api_call_metrics_zero_limit() {
        let metrics = APICallMetrics::new(30, 0);
        assert_eq!(metrics.usage_percentage(), 0.0);
    }

    #[test]
    fn test_api_call_metrics_boundary_79() {
        let metrics = APICallMetrics::new(79, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::Normal);
    }

    #[test]
    fn test_api_call_metrics_boundary_99() {
        let metrics = APICallMetrics::new(99, 100);
        assert_eq!(metrics.determine_status(), ThrottleStatus::ApproachingLimit);
    }

    #[test]
    fn test_api_call_metrics_without_percentage() {
        let metrics = APICallMetrics::new(30, 50).show_percentage(false);
        let spans = metrics.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_api_call_metrics_without_icon() {
        let metrics = APICallMetrics::new(30, 50).show_icon(false);
        let spans = metrics.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_api_call_metrics_minimal() {
        let metrics = APICallMetrics::new(30, 50)
            .show_icon(false)
            .show_percentage(false);
        let spans = metrics.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_api_call_metrics_full_featured() {
        let metrics = APICallMetrics::new(45, 50)
            .status(ThrottleStatus::ApproachingLimit)
            .show_percentage(true)
            .show_icon(true);

        assert_eq!(metrics.current_calls(), 45);
        assert_eq!(metrics.rate_limit(), 50);
        assert_eq!(metrics.usage_percentage(), 90.0);
        assert_eq!(metrics.get_status(), ThrottleStatus::ApproachingLimit);

        let spans = metrics.to_spans();
        assert!(!spans.is_empty());
    }
}
