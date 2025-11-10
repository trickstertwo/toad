//! TokenCounter molecule - API usage display
//!
//! Composes Text and Icon atoms to display token consumption and cost.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Text and Icon atoms
//! - **Pure**: No mutable state, builder pattern
//! - **Composable**: Used by organisms for API usage tracking
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::token_counter::TokenCounter;
//!
//! // Simple counter
//! let counter = TokenCounter::new(1500, 0.045);
//!
//! // With prompt/completion breakdown
//! let counter = TokenCounter::new_detailed(1200, 300, 0.045);
//! ```

use crate::ui::atoms::{Icon, Text};
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::Style,
    text::{Line, Span},
};

/// API token usage counter
///
/// Displays token consumption and associated costs for LLM API calls.
/// Supports both simple (total only) and detailed (prompt + completion) views.
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::token_counter::TokenCounter;
///
/// let counter = TokenCounter::new(1500, 0.045);
/// let line = counter.to_line();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TokenCounter {
    /// Total tokens consumed
    total_tokens: u64,
    /// Prompt tokens (if detailed)
    prompt_tokens: Option<u64>,
    /// Completion tokens (if detailed)
    completion_tokens: Option<u64>,
    /// Total cost in USD
    cost_usd: f64,
    /// Show detailed breakdown
    show_details: bool,
    /// Custom styling
    style: Option<Style>,
}

impl TokenCounter {
    /// Create a simple token counter (total only)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new(1500, 0.045);
    /// assert_eq!(counter.total_tokens(), 1500);
    /// assert_eq!(counter.cost_usd(), 0.045);
    /// ```
    pub fn new(total_tokens: u64, cost_usd: f64) -> Self {
        Self {
            total_tokens,
            prompt_tokens: None,
            completion_tokens: None,
            cost_usd,
            show_details: false,
            style: None,
        }
    }

    /// Create a detailed token counter (prompt + completion)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new_detailed(1200, 300, 0.045);
    /// assert_eq!(counter.prompt_tokens(), Some(1200));
    /// assert_eq!(counter.completion_tokens(), Some(300));
    /// ```
    pub fn new_detailed(prompt_tokens: u64, completion_tokens: u64, cost_usd: f64) -> Self {
        Self {
            total_tokens: prompt_tokens + completion_tokens,
            prompt_tokens: Some(prompt_tokens),
            completion_tokens: Some(completion_tokens),
            cost_usd,
            show_details: true,
            style: None,
        }
    }

    /// Show detailed breakdown
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new_detailed(1200, 300, 0.045)
    ///     .with_details(true);
    /// ```
    pub fn with_details(mut self, show: bool) -> Self {
        self.show_details = show;
        self
    }

    /// Set custom styling
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let counter = TokenCounter::new(1500, 0.045)
    ///     .style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Get total tokens
    pub fn total_tokens(&self) -> u64 {
        self.total_tokens
    }

    /// Get prompt tokens (if detailed)
    pub fn prompt_tokens(&self) -> Option<u64> {
        self.prompt_tokens
    }

    /// Get completion tokens (if detailed)
    pub fn completion_tokens(&self) -> Option<u64> {
        self.completion_tokens
    }

    /// Get cost in USD
    pub fn cost_usd(&self) -> f64 {
        self.cost_usd
    }

    /// Format tokens with K/M suffix for readability
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new(1500, 0.045);
    /// assert_eq!(TokenCounter::format_tokens(1500), "1.5K");
    /// assert_eq!(TokenCounter::format_tokens(1_500_000), "1.5M");
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

    /// Convert to spans for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new(1500, 0.045);
    /// let spans = counter.to_spans();
    /// ```
    pub fn to_spans(&self) -> Vec<Span<'static>> {
        let mut spans = Vec::new();
        let style = self.style.unwrap_or_default();

        // Icon
        let icon = Icon::ui(UiIcon::Tag).style(Style::default().fg(ToadTheme::YELLOW));
        spans.push(icon.to_text().to_span());
        spans.push(Span::raw(" "));

        if self.show_details && self.prompt_tokens.is_some() && self.completion_tokens.is_some() {
            // Detailed view: "1.2K + 300 tokens ($0.045)"
            let prompt = Self::format_tokens(self.prompt_tokens.unwrap());
            let completion = Self::format_tokens(self.completion_tokens.unwrap());

            spans.push(Text::new(prompt).style(style).to_span());
            spans.push(Span::raw(" + "));
            spans.push(Text::new(completion).style(style).to_span());
            spans.push(Span::raw(" tokens "));
        } else {
            // Simple view: "1.5K tokens ($0.045)"
            let total = Self::format_tokens(self.total_tokens);
            spans.push(Text::new(total).style(style).to_span());
            spans.push(Span::raw(" tokens "));
        }

        // Cost
        let cost_text = format!("(${:.3})", self.cost_usd);
        spans.push(
            Text::new(cost_text)
                .style(Style::default().fg(ToadTheme::GRAY))
                .to_span(),
        );

        spans
    }

    /// Convert to a line for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new(1500, 0.045);
    /// let line = counter.to_line();
    /// ```
    pub fn to_line(&self) -> Line<'static> {
        Line::from(self.to_spans())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_counter_new() {
        let counter = TokenCounter::new(1500, 0.045);
        assert_eq!(counter.total_tokens(), 1500);
        assert_eq!(counter.cost_usd(), 0.045);
        assert_eq!(counter.prompt_tokens(), None);
        assert_eq!(counter.completion_tokens(), None);
    }

    #[test]
    fn test_token_counter_new_detailed() {
        let counter = TokenCounter::new_detailed(1200, 300, 0.045);
        assert_eq!(counter.total_tokens(), 1500);
        assert_eq!(counter.prompt_tokens(), Some(1200));
        assert_eq!(counter.completion_tokens(), Some(300));
        assert_eq!(counter.cost_usd(), 0.045);
    }

    #[test]
    fn test_token_counter_with_details() {
        let counter = TokenCounter::new(1500, 0.045).with_details(true);
        assert!(counter.show_details);

        let counter = TokenCounter::new(1500, 0.045).with_details(false);
        assert!(!counter.show_details);
    }

    #[test]
    fn test_token_counter_style() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let counter = TokenCounter::new(1500, 0.045).style(style);
        assert_eq!(counter.style, Some(style));
    }

    #[test]
    fn test_format_tokens_small() {
        assert_eq!(TokenCounter::format_tokens(0), "0");
        assert_eq!(TokenCounter::format_tokens(100), "100");
        assert_eq!(TokenCounter::format_tokens(999), "999");
    }

    #[test]
    fn test_format_tokens_thousands() {
        assert_eq!(TokenCounter::format_tokens(1_000), "1.0K");
        assert_eq!(TokenCounter::format_tokens(1_500), "1.5K");
        assert_eq!(TokenCounter::format_tokens(10_000), "10.0K");
        assert_eq!(TokenCounter::format_tokens(999_999), "1000.0K");
    }

    #[test]
    fn test_format_tokens_millions() {
        assert_eq!(TokenCounter::format_tokens(1_000_000), "1.0M");
        assert_eq!(TokenCounter::format_tokens(1_500_000), "1.5M");
        assert_eq!(TokenCounter::format_tokens(10_000_000), "10.0M");
    }

    #[test]
    fn test_token_counter_to_spans_simple() {
        let counter = TokenCounter::new(1500, 0.045);
        let spans = counter.to_spans();
        // Should have: icon + " " + tokens + " tokens " + cost
        assert!(spans.len() >= 5);
    }

    #[test]
    fn test_token_counter_to_spans_detailed() {
        let counter = TokenCounter::new_detailed(1200, 300, 0.045);
        let spans = counter.to_spans();
        // Should have: icon + " " + prompt + " + " + completion + " tokens " + cost
        assert!(spans.len() >= 7);
    }

    #[test]
    fn test_token_counter_to_line() {
        let counter = TokenCounter::new(1500, 0.045);
        let _line = counter.to_line();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_token_counter_zero_tokens() {
        let counter = TokenCounter::new(0, 0.0);
        assert_eq!(counter.total_tokens(), 0);
        assert_eq!(counter.cost_usd(), 0.0);
    }

    #[test]
    fn test_token_counter_zero_cost() {
        let counter = TokenCounter::new(1500, 0.0);
        assert_eq!(counter.cost_usd(), 0.0);
    }

    #[test]
    fn test_token_counter_large_values() {
        let counter = TokenCounter::new(10_000_000, 100.5);
        assert_eq!(counter.total_tokens(), 10_000_000);
        assert_eq!(counter.cost_usd(), 100.5);
    }

    #[test]
    fn test_token_counter_clone() {
        let counter1 = TokenCounter::new(1500, 0.045);
        let counter2 = counter1.clone();
        assert_eq!(counter1, counter2);
    }

    #[test]
    fn test_token_counter_equality() {
        let counter1 = TokenCounter::new(1500, 0.045);
        let counter2 = TokenCounter::new(1500, 0.045);
        let counter3 = TokenCounter::new(2000, 0.045);

        assert_eq!(counter1, counter2);
        assert_ne!(counter1, counter3);
    }

    #[test]
    fn test_token_counter_chaining() {
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let counter = TokenCounter::new(1500, 0.045)
            .with_details(true)
            .style(style);

        assert!(counter.show_details);
        assert_eq!(counter.style, Some(style));
    }

    #[test]
    fn test_token_counter_detailed_auto_total() {
        let counter = TokenCounter::new_detailed(1200, 300, 0.045);
        assert_eq!(counter.total_tokens(), 1500);
    }

    #[test]
    fn test_token_counter_high_precision_cost() {
        let counter = TokenCounter::new(1500, 0.0012345);
        assert_eq!(counter.cost_usd(), 0.0012345);
    }
}
