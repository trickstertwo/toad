//! MetricCard molecule - Displays a labeled metric with optional icon
//!
//! Composes Text and Icon atoms to create a metric display card.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Text and Icon atoms
//! - **Pure**: No mutable state, builder pattern
//! - **Composable**: Used by organisms for metric displays
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::metric_card::MetricCard;
//! use toad::ui::nerd_fonts::UiIcon;
//! use toad::ui::atoms::icon::Icon;
//!
//! // Simple metric
//! let card = MetricCard::new("Accuracy", "85.2%");
//!
//! // With icon
//! let card = MetricCard::new("Tasks", "10/20")
//!     .icon(Icon::ui(UiIcon::Success));
//!
//! // Success themed
//! let card = MetricCard::success("Tests", "✓ All Passed");
//! ```

use crate::ui::atoms::{Icon, Text};
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// A metric display card
///
/// Composes atoms to show a labeled value with optional icon.
/// Used for displaying statistics, progress, and status information.
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::metric_card::MetricCard;
///
/// let card = MetricCard::new("Accuracy", "85.2%");
/// let line = card.to_line();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MetricCard {
    /// The metric label
    label: String,
    /// The metric value
    value: String,
    /// Optional icon
    icon: Option<Icon>,
    /// Label styling
    label_style: Option<Style>,
    /// Value styling
    value_style: Option<Style>,
}

impl MetricCard {
    /// Create a new metric card
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::new("Accuracy", "85.2%");
    /// ```
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            icon: None,
            label_style: None,
            value_style: None,
        }
    }

    /// Add an icon to the metric card
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    /// use toad::ui::atoms::icon::Icon;
    /// use toad::ui::nerd_fonts::UiIcon;
    ///
    /// let card = MetricCard::new("Status", "Success")
    ///     .icon(Icon::ui(UiIcon::Success));
    /// ```
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set label styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let card = MetricCard::new("Label", "Value")
    ///     .label_style(Style::default().fg(ToadTheme::GRAY));
    /// ```
    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = Some(style);
        self
    }

    /// Set value styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let card = MetricCard::new("Label", "Value")
    ///     .value_style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn value_style(mut self, style: Style) -> Self {
        self.value_style = Some(style);
        self
    }

    /// Create a success-themed metric card
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::success("Tests", "All Passed");
    /// ```
    pub fn success(label: impl Into<String>, value: impl Into<String>) -> Self {
        use crate::ui::nerd_fonts::UiIcon;

        Self::new(label, value)
            .icon(
                Icon::ui(UiIcon::Success).style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT)),
            )
            .value_style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT))
            .label_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Create an error-themed metric card
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::error("Tests", "3 Failed");
    /// ```
    pub fn error(label: impl Into<String>, value: impl Into<String>) -> Self {
        use crate::ui::nerd_fonts::UiIcon;

        Self::new(label, value)
            .icon(Icon::ui(UiIcon::Error).style(Style::default().fg(ToadTheme::RED)))
            .value_style(Style::default().fg(ToadTheme::RED))
            .label_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Create a warning-themed metric card
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::warning("Performance", "Slow");
    /// ```
    pub fn warning(label: impl Into<String>, value: impl Into<String>) -> Self {
        use crate::ui::nerd_fonts::UiIcon;

        Self::new(label, value)
            .icon(Icon::ui(UiIcon::Warning).style(Style::default().fg(ToadTheme::YELLOW)))
            .value_style(Style::default().fg(ToadTheme::YELLOW))
            .label_style(Style::default().fg(ToadTheme::GRAY))
    }

    /// Get the label text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::new("Accuracy", "85%");
    /// assert_eq!(card.label(), "Accuracy");
    /// ```
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the value text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::new("Accuracy", "85%");
    /// assert_eq!(card.value(), "85%");
    /// ```
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Convert to spans for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::new("Accuracy", "85%");
    /// let spans = card.to_spans();
    /// ```
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();

        // Add icon if present
        if let Some(ref icon) = self.icon {
            spans.push(icon.to_text().to_span());
            spans.push(Span::raw(" "));
        }

        // Add label
        let mut label_text = Text::new(&self.label);
        if let Some(style) = self.label_style {
            label_text = label_text.style(style);
        }
        spans.push(label_text.to_span());
        spans.push(Span::raw(": "));

        // Add value
        let mut value_text = Text::new(&self.value);
        if let Some(style) = self.value_style {
            value_text = value_text.style(style);
        }
        spans.push(value_text.to_span());

        spans
    }

    /// Convert to a line for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::metric_card::MetricCard;
    ///
    /// let card = MetricCard::new("Accuracy", "85%");
    /// let line = card.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::nerd_fonts::UiIcon;

    #[test]
    fn test_metric_card_new() {
        let card = MetricCard::new("Accuracy", "85%");
        assert_eq!(card.label(), "Accuracy");
        assert_eq!(card.value(), "85%");
        assert!(card.icon.is_none());
    }

    #[test]
    fn test_metric_card_with_icon() {
        let icon = Icon::ui(UiIcon::Success);
        let card = MetricCard::new("Status", "OK").icon(icon.clone());
        assert_eq!(card.icon, Some(icon));
    }

    #[test]
    fn test_metric_card_label_style() {
        let style = Style::default().fg(ToadTheme::GRAY);
        let card = MetricCard::new("Label", "Value").label_style(style);
        assert_eq!(card.label_style, Some(style));
    }

    #[test]
    fn test_metric_card_value_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let card = MetricCard::new("Label", "Value").value_style(style);
        assert_eq!(card.value_style, Some(style));
    }

    #[test]
    fn test_metric_card_success() {
        let card = MetricCard::success("Tests", "Passed");
        assert_eq!(card.label(), "Tests");
        assert_eq!(card.value(), "Passed");
        assert!(card.icon.is_some());
        assert!(card.value_style.is_some());
    }

    #[test]
    fn test_metric_card_error() {
        let card = MetricCard::error("Tests", "Failed");
        assert_eq!(card.label(), "Tests");
        assert_eq!(card.value(), "Failed");
        assert!(card.icon.is_some());
        assert!(card.value_style.is_some());
    }

    #[test]
    fn test_metric_card_warning() {
        let card = MetricCard::warning("Performance", "Slow");
        assert_eq!(card.label(), "Performance");
        assert_eq!(card.value(), "Slow");
        assert!(card.icon.is_some());
        assert!(card.value_style.is_some());
    }

    #[test]
    fn test_metric_card_to_spans() {
        let card = MetricCard::new("Accuracy", "85%");
        let spans = card.to_spans();
        // Should have: label + ": " + value = 3 spans
        assert_eq!(spans.len(), 3);
    }

    #[test]
    fn test_metric_card_to_spans_with_icon() {
        let card = MetricCard::new("Status", "OK").icon(Icon::ui(UiIcon::Success));
        let spans = card.to_spans();
        // Should have: icon + " " + label + ": " + value = 5 spans
        assert_eq!(spans.len(), 5);
    }

    #[test]
    fn test_metric_card_to_line() {
        let card = MetricCard::new("Accuracy", "85%");
        let _line = card.to_line();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_metric_card_chaining() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let card = MetricCard::new("Test", "Value")
            .icon(Icon::ui(UiIcon::Info))
            .label_style(style)
            .value_style(style);

        assert!(card.icon.is_some());
        assert_eq!(card.label_style, Some(style));
        assert_eq!(card.value_style, Some(style));
    }

    #[test]
    fn test_metric_card_clone() {
        let card1 = MetricCard::new("Test", "Value");
        let card2 = card1.clone();
        assert_eq!(card1.label(), card2.label());
        assert_eq!(card1.value(), card2.value());
    }

    #[test]
    fn test_metric_card_equality() {
        let card1 = MetricCard::new("Test", "Value");
        let card2 = MetricCard::new("Test", "Value");
        let card3 = MetricCard::new("Other", "Value");

        assert_eq!(card1, card2);
        assert_ne!(card1, card3);
    }

    #[test]
    fn test_metric_card_empty_strings() {
        let card = MetricCard::new("", "");
        assert_eq!(card.label(), "");
        assert_eq!(card.value(), "");
    }

    #[test]
    fn test_metric_card_unicode() {
        let card = MetricCard::new("精度", "85%");
        assert_eq!(card.label(), "精度");
    }

    #[test]
    fn test_metric_card_long_text() {
        let long_label = "A".repeat(100);
        let long_value = "B".repeat(100);
        let card = MetricCard::new(&long_label, &long_value);
        assert_eq!(card.label(), long_label);
        assert_eq!(card.value(), long_value);
    }
}
