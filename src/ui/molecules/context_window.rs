//! ContextWindow molecule - Context usage display
//!
//! Displays LLM context window usage with visual indicators for remaining capacity.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Text + Icon atoms
//! - **Purpose**: Show context usage (used/total tokens)
//! - **Visual**: Progress bar with color coding
//! - **States**: Normal (< 70%), Warning (70-90%), Critical (> 90%)
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::context_window::ContextWindow;
//!
//! let window = ContextWindow::new(45000, 200000);
//! let line = window.to_line();
//! ```

use crate::ui::atoms::Icon;
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// Context window usage display
///
/// Shows LLM context window usage with color-coded indicators:
/// - Green: < 70% used (plenty of space)
/// - Yellow: 70-90% used (getting full)
/// - Red: > 90% used (nearly full)
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::context_window::ContextWindow;
///
/// let window = ContextWindow::new(45000, 200000);
/// assert_eq!(window.used_tokens(), 45000);
/// assert_eq!(window.total_tokens(), 200000);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ContextWindow {
    /// Number of tokens currently used
    used: u64,
    /// Total available tokens
    total: u64,
    /// Show percentage
    show_percentage: bool,
    /// Show visual bar
    show_bar: bool,
    /// Bar width in characters
    bar_width: usize,
    /// Custom style override
    style: Option<Style>,
}

impl ContextWindow {
    /// Create a new context window display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(45000, 200000);
    /// ```
    pub fn new(used: u64, total: u64) -> Self {
        Self {
            used,
            total,
            show_percentage: true,
            show_bar: true,
            bar_width: 20,
            style: None,
        }
    }

    /// Set whether to show percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(100, 200).show_percentage(false);
    /// ```
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    /// Set whether to show visual bar
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(100, 200).show_bar(false);
    /// ```
    pub fn show_bar(mut self, show: bool) -> Self {
        self.show_bar = show;
        self
    }

    /// Set bar width
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(100, 200).bar_width(30);
    /// ```
    pub fn bar_width(mut self, width: usize) -> Self {
        self.bar_width = width;
        self
    }

    /// Set custom style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    /// use ratatui::style::Style;
    ///
    /// let window = ContextWindow::new(100, 200).style(Style::default());
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get used tokens
    pub fn used_tokens(&self) -> u64 {
        self.used
    }

    /// Get total tokens
    pub fn total_tokens(&self) -> u64 {
        self.total
    }

    /// Calculate usage percentage (0.0 - 100.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(50, 200);
    /// assert_eq!(window.percentage(), 25.0);
    /// ```
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.used as f64 / self.total as f64) * 100.0
        }
    }

    /// Get usage state based on percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::{ContextWindow, UsageState};
    ///
    /// let normal = ContextWindow::new(50, 200);
    /// assert_eq!(normal.usage_state(), UsageState::Normal);
    ///
    /// let warning = ContextWindow::new(150, 200);
    /// assert_eq!(warning.usage_state(), UsageState::Warning);
    ///
    /// let critical = ContextWindow::new(190, 200);
    /// assert_eq!(critical.usage_state(), UsageState::Critical);
    /// ```
    pub fn usage_state(&self) -> UsageState {
        let pct = self.percentage();
        if pct >= 90.0 {
            UsageState::Critical
        } else if pct >= 70.0 {
            UsageState::Warning
        } else {
            UsageState::Normal
        }
    }

    /// Format tokens with K/M suffixes
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// assert_eq!(ContextWindow::format_tokens(1500), "1.5K");
    /// assert_eq!(ContextWindow::format_tokens(1500000), "1.5M");
    /// assert_eq!(ContextWindow::format_tokens(500), "500");
    /// ```
    pub fn format_tokens(tokens: u64) -> String {
        if tokens >= 1_000_000 {
            format!("{:.1}M", tokens as f64 / 1_000_000.0)
        } else if tokens >= 1_000 {
            format!("{:.1}K", tokens as f64 / 1_000.0)
        } else {
            tokens.to_string()
        }
    }

    /// Get color for current state
    fn get_color(&self) -> ratatui::style::Color {
        if let Some(style) = self.style {
            return style.fg.unwrap_or(ToadTheme::WHITE);
        }

        match self.usage_state() {
            UsageState::Normal => ToadTheme::TOAD_GREEN,
            UsageState::Warning => ToadTheme::YELLOW,
            UsageState::Critical => ToadTheme::RED,
        }
    }

    /// Get icon for current state
    fn get_icon(&self) -> Icon {
        let icon = match self.usage_state() {
            UsageState::Normal => Icon::ui(UiIcon::Success),
            UsageState::Warning => Icon::ui(UiIcon::Warning),
            UsageState::Critical => Icon::ui(UiIcon::Error),
        };
        icon.style(Style::default().fg(self.get_color()))
    }

    /// Create visual progress bar
    fn create_bar(&self) -> String {
        if !self.show_bar {
            return String::new();
        }

        let filled_width = if self.total > 0 {
            ((self.used as f64 / self.total as f64) * self.bar_width as f64) as usize
        } else {
            0
        };

        let filled_width = filled_width.min(self.bar_width);
        let empty_width = self.bar_width.saturating_sub(filled_width);

        format!("[{}{}]", "█".repeat(filled_width), "░".repeat(empty_width))
    }

    /// Convert to styled spans
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(45000, 200000);
    /// let spans = window.to_spans();
    /// assert!(!spans.is_empty());
    /// ```
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        // Icon
        let icon = self.get_icon();
        spans.push(icon.to_text().to_span());
        spans.push(Span::raw(" "));

        // Usage text
        let used_str = Self::format_tokens(self.used);
        let total_str = Self::format_tokens(self.total);
        let usage_text = format!("{} / {}", used_str, total_str);

        let color = self.get_color();
        spans.push(Span::styled(usage_text, Style::default().fg(color)));

        // Percentage
        if self.show_percentage {
            let pct_text = format!(" ({:.1}%)", self.percentage());
            spans.push(Span::styled(pct_text, Style::default().fg(ToadTheme::GRAY)));
        }

        // Bar
        if self.show_bar {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(self.create_bar(), Style::default().fg(color)));
        }

        spans
    }

    /// Convert to line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::context_window::ContextWindow;
    ///
    /// let window = ContextWindow::new(45000, 200000);
    /// let line = window.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

/// Context window usage state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageState {
    /// Normal usage (< 70%)
    Normal,
    /// Warning - getting full (70-90%)
    Warning,
    /// Critical - nearly full (> 90%)
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_window_new() {
        let window = ContextWindow::new(100, 200);
        assert_eq!(window.used_tokens(), 100);
        assert_eq!(window.total_tokens(), 200);
    }

    #[test]
    fn test_context_window_percentage() {
        let window = ContextWindow::new(50, 200);
        assert_eq!(window.percentage(), 25.0);
    }

    #[test]
    fn test_context_window_percentage_zero_total() {
        let window = ContextWindow::new(0, 0);
        assert_eq!(window.percentage(), 0.0);
    }

    #[test]
    fn test_context_window_percentage_full() {
        let window = ContextWindow::new(200, 200);
        assert_eq!(window.percentage(), 100.0);
    }

    #[test]
    fn test_context_window_usage_state_normal() {
        let window = ContextWindow::new(50, 200);
        assert_eq!(window.usage_state(), UsageState::Normal);
    }

    #[test]
    fn test_context_window_usage_state_warning() {
        let window = ContextWindow::new(150, 200);
        assert_eq!(window.usage_state(), UsageState::Warning);
    }

    #[test]
    fn test_context_window_usage_state_critical() {
        let window = ContextWindow::new(190, 200);
        assert_eq!(window.usage_state(), UsageState::Critical);
    }

    #[test]
    fn test_context_window_usage_state_boundary_70() {
        let window = ContextWindow::new(140, 200);
        assert_eq!(window.usage_state(), UsageState::Warning);
    }

    #[test]
    fn test_context_window_usage_state_boundary_90() {
        let window = ContextWindow::new(180, 200);
        assert_eq!(window.usage_state(), UsageState::Critical);
    }

    #[test]
    fn test_context_window_format_tokens_small() {
        assert_eq!(ContextWindow::format_tokens(500), "500");
    }

    #[test]
    fn test_context_window_format_tokens_thousands() {
        assert_eq!(ContextWindow::format_tokens(1500), "1.5K");
        assert_eq!(ContextWindow::format_tokens(45000), "45.0K");
    }

    #[test]
    fn test_context_window_format_tokens_millions() {
        assert_eq!(ContextWindow::format_tokens(1500000), "1.5M");
        assert_eq!(ContextWindow::format_tokens(200000000), "200.0M");
    }

    #[test]
    fn test_context_window_format_tokens_exact_thousand() {
        assert_eq!(ContextWindow::format_tokens(1000), "1.0K");
    }

    #[test]
    fn test_context_window_format_tokens_exact_million() {
        assert_eq!(ContextWindow::format_tokens(1000000), "1.0M");
    }

    #[test]
    fn test_context_window_show_percentage() {
        let window = ContextWindow::new(100, 200).show_percentage(false);
        assert!(!window.show_percentage);
    }

    #[test]
    fn test_context_window_show_bar() {
        let window = ContextWindow::new(100, 200).show_bar(false);
        assert!(!window.show_bar);
    }

    #[test]
    fn test_context_window_bar_width() {
        let window = ContextWindow::new(100, 200).bar_width(30);
        assert_eq!(window.bar_width, 30);
    }

    #[test]
    fn test_context_window_style() {
        let style = Style::default().fg(ToadTheme::RED);
        let window = ContextWindow::new(100, 200).style(style);
        assert_eq!(window.style, Some(style));
    }

    #[test]
    fn test_context_window_chaining() {
        let window = ContextWindow::new(100, 200)
            .show_percentage(false)
            .show_bar(true)
            .bar_width(25);

        assert!(!window.show_percentage);
        assert!(window.show_bar);
        assert_eq!(window.bar_width, 25);
    }

    #[test]
    fn test_context_window_to_spans() {
        let window = ContextWindow::new(45000, 200000);
        let spans = window.to_spans();
        assert!(!spans.is_empty());
    }

    #[test]
    fn test_context_window_to_line() {
        let window = ContextWindow::new(45000, 200000);
        let line = window.to_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_context_window_clone() {
        let window1 = ContextWindow::new(100, 200);
        let window2 = window1.clone();
        assert_eq!(window1, window2);
    }

    #[test]
    fn test_context_window_equality() {
        let window1 = ContextWindow::new(100, 200);
        let window2 = ContextWindow::new(100, 200);
        assert_eq!(window1, window2);
    }

    #[test]
    fn test_usage_state_equality() {
        assert_eq!(UsageState::Normal, UsageState::Normal);
        assert_ne!(UsageState::Normal, UsageState::Warning);
    }

    #[test]
    fn test_context_window_zero_used() {
        let window = ContextWindow::new(0, 200000);
        assert_eq!(window.percentage(), 0.0);
        assert_eq!(window.usage_state(), UsageState::Normal);
    }

    #[test]
    fn test_context_window_full() {
        let window = ContextWindow::new(200000, 200000);
        assert_eq!(window.percentage(), 100.0);
        assert_eq!(window.usage_state(), UsageState::Critical);
    }

    #[test]
    fn test_context_window_over_capacity() {
        let window = ContextWindow::new(250000, 200000);
        assert!(window.percentage() > 100.0);
        assert_eq!(window.usage_state(), UsageState::Critical);
    }

    #[test]
    fn test_context_window_create_bar() {
        let window = ContextWindow::new(100, 200).bar_width(10);
        let bar = window.create_bar();
        assert!(bar.contains('['));
        assert!(bar.contains(']'));
        assert!(bar.contains('█') || bar.contains('░'));
    }

    #[test]
    fn test_context_window_create_bar_disabled() {
        let window = ContextWindow::new(100, 200).show_bar(false);
        let bar = window.create_bar();
        assert!(bar.is_empty());
    }

    #[test]
    fn test_context_window_create_bar_empty() {
        let window = ContextWindow::new(0, 200).bar_width(10);
        let bar = window.create_bar();
        assert_eq!(bar.chars().filter(|c| *c == '░').count(), 10);
    }

    #[test]
    fn test_context_window_create_bar_full() {
        let window = ContextWindow::new(200, 200).bar_width(10);
        let bar = window.create_bar();
        assert_eq!(bar.chars().filter(|c| *c == '█').count(), 10);
    }

    #[test]
    fn test_context_window_create_bar_half() {
        let window = ContextWindow::new(100, 200).bar_width(10);
        let bar = window.create_bar();
        let filled = bar.chars().filter(|c| *c == '█').count();
        assert_eq!(filled, 5);
    }
}
