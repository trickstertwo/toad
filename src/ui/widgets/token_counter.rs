//! Token Counter widget for tracking AI API usage
//!
//! Displays token usage statistics including input/output tokens, cost estimates,
//! and remaining budget.

use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};

/// Token usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Input tokens used
    pub input_tokens: usize,
    /// Output tokens used
    pub output_tokens: usize,
    /// Cached tokens (if applicable)
    pub cached_tokens: usize,
}

impl TokenUsage {
    /// Create new token usage
    pub fn new(input: usize, output: usize) -> Self {
        Self {
            input_tokens: input,
            output_tokens: output,
            cached_tokens: 0,
        }
    }

    /// Total tokens
    pub fn total(&self) -> usize {
        self.input_tokens + self.output_tokens
    }

    /// Add usage
    pub fn add(&mut self, other: &TokenUsage) {
        self.input_tokens += other.input_tokens;
        self.output_tokens += other.output_tokens;
        self.cached_tokens += other.cached_tokens;
    }

    /// Reset usage
    pub fn reset(&mut self) {
        self.input_tokens = 0;
        self.output_tokens = 0;
        self.cached_tokens = 0;
    }
}

/// Cost calculation for a model
#[derive(Debug, Clone)]
pub struct CostModel {
    /// Price per million input tokens
    pub input_price: f64,
    /// Price per million output tokens
    pub output_price: f64,
    /// Price per million cached tokens (if applicable)
    pub cache_price: f64,
}

impl CostModel {
    /// Claude Sonnet 4.5 pricing
    pub fn claude_sonnet_4_5() -> Self {
        Self {
            input_price: 3.0,
            output_price: 15.0,
            cache_price: 0.3,
        }
    }

    /// Claude Opus 4 pricing
    pub fn claude_opus_4() -> Self {
        Self {
            input_price: 15.0,
            output_price: 75.0,
            cache_price: 1.5,
        }
    }

    /// Claude Haiku 4 pricing
    pub fn claude_haiku_4() -> Self {
        Self {
            input_price: 0.25,
            output_price: 1.25,
            cache_price: 0.025,
        }
    }

    /// GPT-4o pricing
    pub fn gpt_4o() -> Self {
        Self {
            input_price: 2.5,
            output_price: 10.0,
            cache_price: 0.0,
        }
    }

    /// Calculate cost for usage
    pub fn calculate_cost(&self, usage: &TokenUsage) -> f64 {
        let input_cost = (usage.input_tokens as f64 / 1_000_000.0) * self.input_price;
        let output_cost = (usage.output_tokens as f64 / 1_000_000.0) * self.output_price;
        let cache_cost = (usage.cached_tokens as f64 / 1_000_000.0) * self.cache_price;

        input_cost + output_cost + cache_cost
    }
}

/// Token counter widget
pub struct TokenCounter {
    /// Current session usage
    session_usage: TokenUsage,
    /// Total usage (all time)
    total_usage: TokenUsage,
    /// Current model's cost model
    cost_model: CostModel,
    /// Budget limit (in dollars, optional)
    budget: Option<f64>,
    /// Show detailed breakdown
    show_details: bool,
    /// Compact mode (single line)
    compact: bool,
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenCounter {
    /// Create a new token counter
    pub fn new() -> Self {
        Self {
            session_usage: TokenUsage::default(),
            total_usage: TokenUsage::default(),
            cost_model: CostModel::claude_sonnet_4_5(),
            budget: None,
            show_details: true,
            compact: false,
        }
    }

    /// Add token usage
    pub fn add_usage(&mut self, usage: TokenUsage) {
        self.session_usage.add(&usage);
        self.total_usage.add(&usage);
    }

    /// Reset session usage
    pub fn reset_session(&mut self) {
        self.session_usage.reset();
    }

    /// Reset total usage
    pub fn reset_total(&mut self) {
        self.total_usage.reset();
    }

    /// Set cost model
    pub fn set_cost_model(&mut self, cost_model: CostModel) {
        self.cost_model = cost_model;
    }

    /// Set budget limit
    pub fn with_budget(mut self, budget: f64) -> Self {
        self.budget = Some(budget);
        self
    }

    /// Toggle details view
    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
    }

    /// Set compact mode
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Get session cost
    pub fn session_cost(&self) -> f64 {
        self.cost_model.calculate_cost(&self.session_usage)
    }

    /// Get total cost
    pub fn total_cost(&self) -> f64 {
        self.cost_model.calculate_cost(&self.total_usage)
    }

    /// Format number with K/M suffix
    fn format_number(n: usize) -> String {
        if n >= 1_000_000 {
            format!("{:.1}M", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1}K", n as f64 / 1_000.0)
        } else {
            format!("{}", n)
        }
    }

    /// Format cost
    fn format_cost(cost: f64) -> String {
        if cost >= 1.0 {
            format!("${:.2}", cost)
        } else if cost >= 0.01 {
            format!("${:.3}", cost)
        } else if cost >= 0.001 {
            format!("${:.4}", cost)
        } else {
            format!("$< 0.001")
        }
    }

    /// Render compact view (single line)
    fn render_compact(&self, frame: &mut Frame, area: Rect) {
        let total = self.session_usage.total();
        let cost = self.session_cost();

        let text = format!(
            "Tokens: {} ({} in / {} out) | Cost: {}",
            Self::format_number(total),
            Self::format_number(self.session_usage.input_tokens),
            Self::format_number(self.session_usage.output_tokens),
            Self::format_cost(cost)
        );

        let color = if let Some(budget) = self.budget {
            if cost >= budget {
                Color::Red
            } else if cost >= budget * 0.8 {
                Color::Yellow
            } else {
                Color::Green
            }
        } else {
            Color::Cyan
        };

        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(color))
            .alignment(Alignment::Right);

        frame.render_widget(paragraph, area);
    }

    /// Render full view
    fn render_full(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Token Usage ")
            .style(Style::default().fg(Color::White));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::new();

        // Session usage
        lines.push(Line::from(vec![
            Span::styled("Session: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                Self::format_number(self.session_usage.total()),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" tokens"),
        ]));

        if self.show_details {
            lines.push(Line::from(vec![
                Span::raw("  Input: "),
                Span::styled(
                    Self::format_number(self.session_usage.input_tokens),
                    Style::default().fg(Color::Green),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Output: "),
                Span::styled(
                    Self::format_number(self.session_usage.output_tokens),
                    Style::default().fg(Color::Yellow),
                ),
            ]));

            if self.session_usage.cached_tokens > 0 {
                lines.push(Line::from(vec![
                    Span::raw("  Cached: "),
                    Span::styled(
                        Self::format_number(self.session_usage.cached_tokens),
                        Style::default().fg(Color::Blue),
                    ),
                ]));
            }
        }

        // Session cost
        let session_cost = self.session_cost();
        lines.push(Line::from(vec![
            Span::raw("  Cost: "),
            Span::styled(
                Self::format_cost(session_cost),
                Style::default().fg(Color::Yellow),
            ),
        ]));

        lines.push(Line::from(""));

        // Total usage
        lines.push(Line::from(vec![
            Span::styled("Total: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                Self::format_number(self.total_usage.total()),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" tokens"),
        ]));

        let total_cost = self.total_cost();
        lines.push(Line::from(vec![
            Span::raw("  Cost: "),
            Span::styled(
                Self::format_cost(total_cost),
                Style::default().fg(Color::Yellow),
            ),
        ]));

        // Budget gauge if set
        if let Some(budget) = self.budget {
            lines.push(Line::from(""));

            let percentage = ((session_cost / budget) * 100.0).min(100.0) as u16;
            let color = if percentage >= 100 {
                Color::Red
            } else if percentage >= 80 {
                Color::Yellow
            } else {
                Color::Green
            };

            lines.push(Line::from(vec![
                Span::styled("Budget: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(format!("{}%", percentage), Style::default().fg(color)),
                Span::raw(format!(" of {}", Self::format_cost(budget))),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }

    /// Render the token counter
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if self.compact {
            self.render_compact(frame, area);
        } else {
            self.render_full(frame, area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_usage() {
        let mut usage = TokenUsage::new(100, 50);
        assert_eq!(usage.total(), 150);

        let other = TokenUsage::new(50, 25);
        usage.add(&other);
        assert_eq!(usage.total(), 225);
        assert_eq!(usage.input_tokens, 150);
        assert_eq!(usage.output_tokens, 75);
    }

    #[test]
    fn test_cost_calculation() {
        let cost_model = CostModel::claude_sonnet_4_5();
        let usage = TokenUsage::new(1_000_000, 1_000_000);
        let cost = cost_model.calculate_cost(&usage);
        assert_eq!(cost, 18.0); // 3.0 + 15.0
    }

    #[test]
    fn test_token_counter() {
        let mut counter = TokenCounter::new();
        assert_eq!(counter.session_usage.total(), 0);

        counter.add_usage(TokenUsage::new(100, 50));
        assert_eq!(counter.session_usage.total(), 150);
        assert_eq!(counter.total_usage.total(), 150);

        counter.reset_session();
        assert_eq!(counter.session_usage.total(), 0);
        assert_eq!(counter.total_usage.total(), 150);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(TokenCounter::format_number(500), "500");
        assert_eq!(TokenCounter::format_number(1_500), "1.5K");
        assert_eq!(TokenCounter::format_number(1_500_000), "1.5M");
    }

    #[test]
    fn test_format_cost() {
        assert_eq!(TokenCounter::format_cost(1.234), "$1.23");
        assert_eq!(TokenCounter::format_cost(0.056), "$0.056");
        assert_eq!(TokenCounter::format_cost(0.0005), "$< 0.001");
    }

    #[test]
    fn test_budget_tracking() {
        let mut counter = TokenCounter::new().with_budget(1.0);
        counter.add_usage(TokenUsage::new(100_000, 50_000));

        let cost = counter.session_cost();
        assert!(cost < 1.0);
    }
}
