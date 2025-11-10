//! ProgressBar molecule - Displays progress with visual bar
//!
//! Composes Text atoms to create a progress bar display.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Text atoms for label, bar, and percentage
//! - **Pure**: No mutable state, builder pattern
//! - **Composable**: Used by organisms for progress tracking
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::progress_bar::ProgressBar;
//!
//! // Simple progress bar
//! let bar = ProgressBar::new("Tasks", 7, 10);
//!
//! // With custom width and styling
//! let bar = ProgressBar::new("Downloading", 50, 100)
//!     .width(30);
//!
//! // Themed constructors
//! let bar = ProgressBar::success("Complete", 10, 10);
//! ```

use crate::ui::atoms::Text;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// A progress bar display
///
/// Composes Text atoms to show a labeled progress bar with percentage.
/// Used for displaying task progress, download progress, and completion status.
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::progress_bar::ProgressBar;
///
/// let bar = ProgressBar::new("Tasks", 7, 10);
/// assert_eq!(bar.percentage(), 70.0);
/// let line = bar.to_line();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ProgressBar {
    /// The progress label
    label: String,
    /// Current progress value
    current: usize,
    /// Total/maximum value
    total: usize,
    /// Bar width in characters
    width: u16,
    /// Label styling
    label_style: Option<Style>,
    /// Filled bar styling
    bar_style: Option<Style>,
    /// Empty bar background styling
    background_style: Option<Style>,
}

impl ProgressBar {
    /// Default bar width
    const DEFAULT_WIDTH: u16 = 20;

    /// Create a new progress bar
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 7, 10);
    /// assert_eq!(bar.current(), 7);
    /// assert_eq!(bar.total(), 10);
    /// ```
    pub fn new(label: impl Into<String>, current: usize, total: usize) -> Self {
        Self {
            label: label.into(),
            current,
            total,
            width: Self::DEFAULT_WIDTH,
            label_style: None,
            bar_style: None,
            background_style: None,
        }
    }

    /// Set the bar width in characters
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10).width(30);
    /// ```
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set label styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10)
    ///     .label_style(Style::default().fg(ToadTheme::GRAY));
    /// ```
    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }

    /// Set filled bar styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10)
    ///     .bar_style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn bar_style(mut self, style: Style) -> Self {
        self.bar_style = Some(style);
        self
    }

    /// Set empty bar background styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10)
    ///     .background_style(Style::default().fg(ToadTheme::GRAY));
    /// ```
    pub fn background_style(mut self, style: Style) -> Self {
        self.background_style = Some(style);
        self
    }

    /// Create a success-themed progress bar (complete)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::success("Complete", 10, 10);
    /// ```
    pub fn success(label: impl Into<String>, current: usize, total: usize) -> Self {
        Self::new(label, current, total)
            .label_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .bar_style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT))
            .background_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Create a warning-themed progress bar (partial)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::warning("In Progress", 5, 10);
    /// ```
    pub fn warning(label: impl Into<String>, current: usize, total: usize) -> Self {
        Self::new(label, current, total)
            .label_style(Style::default().fg(ToadTheme::YELLOW))
            .bar_style(Style::default().fg(ToadTheme::YELLOW))
            .background_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Create an error-themed progress bar (failed)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::error("Failed", 3, 10);
    /// ```
    pub fn error(label: impl Into<String>, current: usize, total: usize) -> Self {
        Self::new(label, current, total)
            .label_style(Style::default().fg(ToadTheme::RED))
            .bar_style(Style::default().fg(ToadTheme::RED))
            .background_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Get the label text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10);
    /// assert_eq!(bar.label(), "Tasks");
    /// ```
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current progress value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10);
    /// assert_eq!(bar.current(), 5);
    /// ```
    pub fn current(&self) -> usize {
        self.current
    }

    /// Get the total/maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 5, 10);
    /// assert_eq!(bar.total(), 10);
    /// ```
    pub fn total(&self) -> usize {
        self.total
    }

    /// Calculate the progress percentage (0.0 - 100.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 7, 10);
    /// assert_eq!(bar.percentage(), 70.0);
    /// ```
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }

    /// Convert to spans for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 7, 10);
    /// let spans = bar.to_spans();
    /// ```
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        // Add label
        let mut label_text = Text::new(&self.label);
        if let Some(style) = self.label_style {
            label_text = label_text.style(style);
        }
        spans.push(label_text.to_span());
        spans.push(Span::raw(": "));

        // Calculate bar dimensions
        let filled_width = if self.total == 0 {
            0
        } else {
            ((self.current as f64 / self.total as f64) * self.width as f64) as u16
        };
        let empty_width = self.width.saturating_sub(filled_width);

        // Add filled portion
        if filled_width > 0 {
            let filled_str = "█".repeat(filled_width as usize);
            let mut filled_text = Text::new(filled_str);
            if let Some(style) = self.bar_style {
                filled_text = filled_text.style(style);
            }
            spans.push(filled_text.to_span());
        }

        // Add empty portion
        if empty_width > 0 {
            let empty_str = "░".repeat(empty_width as usize);
            let mut empty_text = Text::new(empty_str);
            if let Some(style) = self.background_style {
                empty_text = empty_text.style(style);
            }
            spans.push(empty_text.to_span());
        }

        // Add percentage
        let percentage_str = format!(" {:.1}%", self.percentage());
        spans.push(Span::raw(percentage_str));

        spans
    }

    /// Convert to a line for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::progress_bar::ProgressBar;
    ///
    /// let bar = ProgressBar::new("Tasks", 7, 10);
    /// let line = bar.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_new() {
        let bar = ProgressBar::new("Tasks", 7, 10);
        assert_eq!(bar.label(), "Tasks");
        assert_eq!(bar.current(), 7);
        assert_eq!(bar.total(), 10);
        assert_eq!(bar.width, ProgressBar::DEFAULT_WIDTH);
    }

    #[test]
    fn test_progress_bar_percentage() {
        let bar = ProgressBar::new("Tasks", 7, 10);
        assert_eq!(bar.percentage(), 70.0);

        let bar = ProgressBar::new("Tasks", 0, 10);
        assert_eq!(bar.percentage(), 0.0);

        let bar = ProgressBar::new("Tasks", 10, 10);
        assert_eq!(bar.percentage(), 100.0);
    }

    #[test]
    fn test_progress_bar_percentage_zero_total() {
        let bar = ProgressBar::new("Tasks", 5, 0);
        assert_eq!(bar.percentage(), 0.0);
    }

    #[test]
    fn test_progress_bar_width() {
        let bar = ProgressBar::new("Tasks", 5, 10).width(30);
        assert_eq!(bar.width, 30);
    }

    #[test]
    fn test_progress_bar_label_style() {
        let style = Style::default().fg(ToadTheme::GRAY);
        let bar = ProgressBar::new("Tasks", 5, 10).label_style(style);
        assert_eq!(bar.label_style, Some(style));
    }

    #[test]
    fn test_progress_bar_bar_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let bar = ProgressBar::new("Tasks", 5, 10).bar_style(style);
        assert_eq!(bar.bar_style, Some(style));
    }

    #[test]
    fn test_progress_bar_background_style() {
        let style = Style::default().fg(ToadTheme::GRAY);
        let bar = ProgressBar::new("Tasks", 5, 10).background_style(style);
        assert_eq!(bar.background_style, Some(style));
    }

    #[test]
    fn test_progress_bar_success() {
        let bar = ProgressBar::success("Complete", 10, 10);
        assert_eq!(bar.percentage(), 100.0);
        assert!(bar.label_style.is_some());
        assert!(bar.bar_style.is_some());
    }

    #[test]
    fn test_progress_bar_warning() {
        let bar = ProgressBar::warning("Partial", 5, 10);
        assert!(bar.label_style.is_some());
        assert!(bar.bar_style.is_some());
    }

    #[test]
    fn test_progress_bar_error() {
        let bar = ProgressBar::error("Failed", 3, 10);
        assert!(bar.label_style.is_some());
        assert!(bar.bar_style.is_some());
    }

    #[test]
    fn test_progress_bar_to_spans() {
        let bar = ProgressBar::new("Tasks", 5, 10).width(10);
        let spans = bar.to_spans();
        // Should have: label + ": " + filled + empty + percentage
        assert!(spans.len() >= 4);
    }

    #[test]
    fn test_progress_bar_to_spans_zero_progress() {
        let bar = ProgressBar::new("Tasks", 0, 10).width(10);
        let spans = bar.to_spans();
        // Should have: label + ": " + empty + percentage
        assert!(spans.len() >= 3);
    }

    #[test]
    fn test_progress_bar_to_spans_full_progress() {
        let bar = ProgressBar::new("Tasks", 10, 10).width(10);
        let spans = bar.to_spans();
        // Should have: label + ": " + filled + percentage
        assert!(spans.len() >= 3);
    }

    #[test]
    fn test_progress_bar_to_line() {
        let bar = ProgressBar::new("Tasks", 7, 10);
        let _line = bar.to_line();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_progress_bar_chaining() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let bar = ProgressBar::new("Tasks", 5, 10)
            .width(30)
            .label_style(style)
            .bar_style(style)
            .background_style(style);

        assert_eq!(bar.width, 30);
        assert_eq!(bar.label_style, Some(style));
        assert_eq!(bar.bar_style, Some(style));
        assert_eq!(bar.background_style, Some(style));
    }

    #[test]
    fn test_progress_bar_clone() {
        let bar1 = ProgressBar::new("Tasks", 5, 10);
        let bar2 = bar1.clone();
        assert_eq!(bar1.label(), bar2.label());
        assert_eq!(bar1.current(), bar2.current());
        assert_eq!(bar1.total(), bar2.total());
    }

    #[test]
    fn test_progress_bar_equality() {
        let bar1 = ProgressBar::new("Tasks", 5, 10);
        let bar2 = ProgressBar::new("Tasks", 5, 10);
        let bar3 = ProgressBar::new("Other", 5, 10);

        assert_eq!(bar1, bar2);
        assert_ne!(bar1, bar3);
    }

    #[test]
    fn test_progress_bar_empty_label() {
        let bar = ProgressBar::new("", 5, 10);
        assert_eq!(bar.label(), "");
    }

    #[test]
    fn test_progress_bar_unicode_label() {
        let bar = ProgressBar::new("進捗", 5, 10);
        assert_eq!(bar.label(), "進捗");
    }

    #[test]
    fn test_progress_bar_large_values() {
        let bar = ProgressBar::new("Tasks", 1000, 10000);
        assert_eq!(bar.percentage(), 10.0);
    }
}
