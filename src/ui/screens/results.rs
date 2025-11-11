//! Results Screen - Post-evaluation summary display
//!
//! Displays final evaluation results including accuracy, cost, duration, and task statistics.
//!
//! # Architecture
//!
//! Following Atomic Design:
//! - **Screen**: Top-level UI composition
//! - **Stateful**: Holds evaluation results
//! - **Pure**: Renders final results
//!
//! # Examples
//!
//! ```
//! use toad::ui::screens::results::ResultsScreen;
//! use ratatui::{buffer::Buffer, layout::Rect};
//!
//! let screen = ResultsScreen::new()
//!     .accuracy(85.2)
//!     .cost(0.45)
//!     .duration_secs(120)
//!     .tasks_completed(8)
//!     .tasks_total(10);
//!
//! let area = Rect::new(0, 0, 80, 24);
//! let mut buf = Buffer::empty(area);
//! screen.render(area, &mut buf);
//! ```

use crate::ui::atoms::{Block, Icon, Text};
use crate::ui::molecules::MetricCard;
use crate::ui::primitives::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Results screen with final evaluation metrics
///
/// Displays post-evaluation summary including accuracy, cost, duration, and task statistics.
///
/// # Examples
///
/// ```
/// use toad::ui::screens::results::ResultsScreen;
///
/// let screen = ResultsScreen::new()
///     .accuracy(85.2)
///     .tasks_completed(8)
///     .tasks_total(10);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ResultsScreen {
    /// Final accuracy percentage (0.0 - 100.0)
    accuracy: Option<f64>,
    /// Total cost in USD
    cost: Option<f64>,
    /// Total duration in seconds
    duration_secs: Option<u64>,
    /// Number of tasks completed successfully
    tasks_completed: usize,
    /// Total number of tasks
    tasks_total: usize,
    /// Screen title
    title: String,
}

impl ResultsScreen {
    /// Create a new results screen
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new();
    /// ```
    pub fn new() -> Self {
        Self {
            accuracy: None,
            cost: None,
            duration_secs: None,
            tasks_completed: 0,
            tasks_total: 0,
            title: "Evaluation Results".to_string(),
        }
    }

    /// Set screen title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new().title("M1 Baseline Results");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set final accuracy percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new().accuracy(85.2);
    /// ```
    pub fn accuracy(mut self, accuracy: f64) -> Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Set total cost in USD
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new().cost(0.45);
    /// ```
    pub fn cost(mut self, cost: f64) -> Self {
        self.cost = Some(cost);
        self
    }

    /// Set total duration in seconds
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new().duration_secs(120);
    /// ```
    pub fn duration_secs(mut self, secs: u64) -> Self {
        self.duration_secs = Some(secs);
        self
    }

    /// Set number of tasks completed
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new().tasks_completed(8);
    /// ```
    pub fn tasks_completed(mut self, completed: usize) -> Self {
        self.tasks_completed = completed;
        self
    }

    /// Set total number of tasks
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new().tasks_total(10);
    /// ```
    pub fn tasks_total(mut self, total: usize) -> Self {
        self.tasks_total = total;
        self
    }

    /// Get accuracy
    pub fn get_accuracy(&self) -> Option<f64> {
        self.accuracy
    }

    /// Get cost
    pub fn get_cost(&self) -> Option<f64> {
        self.cost
    }

    /// Get duration
    pub fn get_duration(&self) -> Option<u64> {
        self.duration_secs
    }

    /// Get tasks completed
    pub fn get_tasks_completed(&self) -> usize {
        self.tasks_completed
    }

    /// Get tasks total
    pub fn get_tasks_total(&self) -> usize {
        self.tasks_total
    }

    /// Calculate pass rate percentage
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::screens::results::ResultsScreen;
    ///
    /// let screen = ResultsScreen::new()
    ///     .tasks_completed(8)
    ///     .tasks_total(10);
    /// assert_eq!(screen.pass_rate(), 80.0);
    /// ```
    pub fn pass_rate(&self) -> f64 {
        if self.tasks_total == 0 {
            0.0
        } else {
            (self.tasks_completed as f64 / self.tasks_total as f64) * 100.0
        }
    }

    /// Render the results screen
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create border
        let block = Block::themed(&self.title).to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        // Split layout: metrics | task summary | prompt
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Spacer
                Constraint::Length(5), // Metrics (3 lines + spacing)
                Constraint::Length(2), // Spacer
                Constraint::Length(3), // Task summary
                Constraint::Length(2), // Spacer
                Constraint::Length(1), // Prompt
                Constraint::Min(0),    // Filler
            ])
            .split(inner);

        // Render metrics
        self.render_metrics(chunks[2], buf);

        // Render task summary
        self.render_task_summary(chunks[4], buf);

        // Render prompt
        self.render_prompt(chunks[6], buf);
    }

    /// Render metrics (accuracy, cost, duration)
    fn render_metrics(&self, area: Rect, buf: &mut Buffer) {
        let mut lines = Vec::new();

        // Accuracy
        if let Some(acc) = self.accuracy {
            let card = MetricCard::new("Accuracy", format!("{:.1}%", acc))
                .icon(Icon::ui(UiIcon::Success).style(Style::default().fg(ToadTheme::TOAD_GREEN)))
                .label_style(Style::default().fg(ToadTheme::GRAY))
                .value_style(Style::default().fg(ToadTheme::TOAD_GREEN));
            lines.push(card.to_line());
        }

        // Cost
        if let Some(cost) = self.cost {
            let card = MetricCard::new("Total Cost", format!("${:.3}", cost))
                .icon(Icon::ui(UiIcon::Tag).style(Style::default().fg(ToadTheme::YELLOW)))
                .label_style(Style::default().fg(ToadTheme::GRAY))
                .value_style(Style::default().fg(ToadTheme::WHITE));
            lines.push(card.to_line());
        }

        // Duration
        if let Some(secs) = self.duration_secs {
            let duration_str = if secs < 60 {
                format!("{}s", secs)
            } else {
                format!("{}m {}s", secs / 60, secs % 60)
            };
            let card = MetricCard::new("Duration", duration_str)
                .icon(Icon::ui(UiIcon::Clock).style(Style::default().fg(ToadTheme::BLUE)))
                .label_style(Style::default().fg(ToadTheme::GRAY))
                .value_style(Style::default().fg(ToadTheme::WHITE));
            lines.push(card.to_line());
        }

        Paragraph::new(lines).render(area, buf);
    }

    /// Render task summary
    fn render_task_summary(&self, area: Rect, buf: &mut Buffer) {
        let pass_rate = self.pass_rate();
        let tasks_failed = self.tasks_total.saturating_sub(self.tasks_completed);

        let summary_lines = vec![
            Line::from(vec![
                Span::styled("Tasks: ", Style::default().fg(ToadTheme::GRAY)),
                Span::styled(
                    format!("{} / {}", self.tasks_completed, self.tasks_total),
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Pass Rate: ", Style::default().fg(ToadTheme::GRAY)),
                Span::styled(
                    format!("{:.1}%", pass_rate),
                    Style::default()
                        .fg(if pass_rate >= 80.0 {
                            ToadTheme::TOAD_GREEN
                        } else if pass_rate >= 60.0 {
                            ToadTheme::YELLOW
                        } else {
                            ToadTheme::RED
                        })
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Failed: ", Style::default().fg(ToadTheme::GRAY)),
                Span::styled(
                    format!("{}", tasks_failed),
                    Style::default().fg(if tasks_failed > 0 {
                        ToadTheme::RED
                    } else {
                        ToadTheme::GRAY
                    }),
                ),
            ]),
        ];

        Paragraph::new(summary_lines)
            .alignment(Alignment::Center)
            .render(area, buf);
    }

    /// Render "press any key" prompt
    fn render_prompt(&self, area: Rect, buf: &mut Buffer) {
        let prompt = Text::new("Press any key to return to main screen...")
            .style(
                Style::default()
                    .fg(ToadTheme::GRAY)
                    .add_modifier(Modifier::DIM),
            )
            .to_span();

        Paragraph::new(prompt)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

impl Default for ResultsScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ResultsScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        (&self).render(area, buf);
    }
}

impl Widget for &ResultsScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        ResultsScreen::render(self, area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results_screen_new() {
        let screen = ResultsScreen::new();
        assert_eq!(screen.get_accuracy(), None);
        assert_eq!(screen.get_cost(), None);
        assert_eq!(screen.get_duration(), None);
        assert_eq!(screen.get_tasks_completed(), 0);
        assert_eq!(screen.get_tasks_total(), 0);
    }

    #[test]
    fn test_results_screen_title() {
        let screen = ResultsScreen::new().title("M1 Results");
        assert_eq!(screen.title, "M1 Results");
    }

    #[test]
    fn test_results_screen_accuracy() {
        let screen = ResultsScreen::new().accuracy(85.2);
        assert_eq!(screen.get_accuracy(), Some(85.2));
    }

    #[test]
    fn test_results_screen_cost() {
        let screen = ResultsScreen::new().cost(0.45);
        assert_eq!(screen.get_cost(), Some(0.45));
    }

    #[test]
    fn test_results_screen_duration() {
        let screen = ResultsScreen::new().duration_secs(120);
        assert_eq!(screen.get_duration(), Some(120));
    }

    #[test]
    fn test_results_screen_tasks_completed() {
        let screen = ResultsScreen::new().tasks_completed(8);
        assert_eq!(screen.get_tasks_completed(), 8);
    }

    #[test]
    fn test_results_screen_tasks_total() {
        let screen = ResultsScreen::new().tasks_total(10);
        assert_eq!(screen.get_tasks_total(), 10);
    }

    #[test]
    fn test_results_screen_pass_rate() {
        let screen = ResultsScreen::new().tasks_completed(8).tasks_total(10);
        assert_eq!(screen.pass_rate(), 80.0);
    }

    #[test]
    fn test_results_screen_pass_rate_zero_total() {
        let screen = ResultsScreen::new().tasks_completed(0).tasks_total(0);
        assert_eq!(screen.pass_rate(), 0.0);
    }

    #[test]
    fn test_results_screen_pass_rate_perfect() {
        let screen = ResultsScreen::new().tasks_completed(10).tasks_total(10);
        assert_eq!(screen.pass_rate(), 100.0);
    }

    #[test]
    fn test_results_screen_pass_rate_zero_completed() {
        let screen = ResultsScreen::new().tasks_completed(0).tasks_total(10);
        assert_eq!(screen.pass_rate(), 0.0);
    }

    #[test]
    fn test_results_screen_chaining() {
        let screen = ResultsScreen::new()
            .title("Test")
            .accuracy(85.0)
            .cost(0.5)
            .duration_secs(100)
            .tasks_completed(8)
            .tasks_total(10);

        assert_eq!(screen.title, "Test");
        assert_eq!(screen.get_accuracy(), Some(85.0));
        assert_eq!(screen.get_cost(), Some(0.5));
        assert_eq!(screen.get_duration(), Some(100));
        assert_eq!(screen.get_tasks_completed(), 8);
        assert_eq!(screen.get_tasks_total(), 10);
    }

    #[test]
    fn test_results_screen_render() {
        let screen = ResultsScreen::new()
            .accuracy(85.0)
            .cost(0.45)
            .duration_secs(120)
            .tasks_completed(8)
            .tasks_total(10);

        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        screen.render(area, &mut buf);
        // Verify it doesn't panic
    }

    #[test]
    fn test_results_screen_clone() {
        let screen1 = ResultsScreen::new().accuracy(85.0);
        let screen2 = screen1.clone();
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_results_screen_equality() {
        let screen1 = ResultsScreen::new().accuracy(85.0);
        let screen2 = ResultsScreen::new().accuracy(85.0);
        assert_eq!(screen1, screen2);
    }

    #[test]
    fn test_results_screen_default() {
        let screen = ResultsScreen::default();
        assert_eq!(screen.get_accuracy(), None);
    }

    #[test]
    fn test_results_screen_widget_trait() {
        let screen = ResultsScreen::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        // Test both owned and borrowed widget rendering
        screen.clone().render(area, &mut buf);
        (&screen).render(area, &mut buf);
    }

    #[test]
    fn test_results_screen_minimal() {
        let screen = ResultsScreen::new();
        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        screen.render(area, &mut buf);
        // Verify minimal screen renders without panic
    }

    #[test]
    fn test_results_screen_full_metrics() {
        let screen = ResultsScreen::new()
            .accuracy(90.5)
            .cost(1.25)
            .duration_secs(300)
            .tasks_completed(9)
            .tasks_total(10);

        assert_eq!(screen.get_accuracy(), Some(90.5));
        assert_eq!(screen.get_cost(), Some(1.25));
        assert_eq!(screen.get_duration(), Some(300));
        assert_eq!(screen.pass_rate(), 90.0);
    }

    #[test]
    fn test_results_screen_partial_metrics() {
        let screen = ResultsScreen::new()
            .accuracy(75.0)
            .tasks_completed(7)
            .tasks_total(10);

        assert_eq!(screen.get_accuracy(), Some(75.0));
        assert_eq!(screen.get_cost(), None);
        assert_eq!(screen.get_duration(), None);
        assert_eq!(screen.pass_rate(), 70.0);
    }
}
