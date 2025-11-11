//! Smart suggestions widget for context-aware hints
//!
//! Provides intelligent suggestions based on current context and user patterns.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::SmartSuggestions;
//!
//! let suggestions = SmartSuggestions::new()
//!     .add_suggestion("Save file", "save", 0.95, Some("Based on unsaved changes"))
//!     .add_suggestion("Run tests", "test", 0.85, None::<&str>);
//!
//! assert_eq!(suggestions.suggestion_count(), 2);
//! ```

use crate::ui::atoms::{block::Block as AtomBlock, text::Text as AtomText};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Borders, Widget},
};

/// Suggestion relevance score
pub type RelevanceScore = f64;

/// Smart suggestion item
///
/// Represents a context-aware suggestion with relevance scoring.
///
/// # Examples
///
/// ```
/// use toad::widgets::Suggestion;
///
/// let suggestion = Suggestion::new("Save file", "save", 0.95)
///     .with_reason("Unsaved changes detected");
///
/// assert_eq!(suggestion.label(), "Save file");
/// assert_eq!(suggestion.action(), "save");
/// assert_eq!(suggestion.relevance(), 0.95);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Suggestion {
    /// Suggestion label
    label: String,
    /// Action identifier
    action: String,
    /// Relevance score (0.0 to 1.0)
    relevance: RelevanceScore,
    /// Optional reason for suggestion
    reason: Option<String>,
    /// Optional category
    category: Option<String>,
    /// Icon/emoji
    icon: Option<String>,
}

impl Suggestion {
    /// Create a new suggestion
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Suggestion;
    ///
    /// let suggestion = Suggestion::new("Save file", "save", 0.95);
    /// assert_eq!(suggestion.label(), "Save file");
    /// assert_eq!(suggestion.relevance(), 0.95);
    /// ```
    pub fn new(label: impl Into<String>, action: impl Into<String>, relevance: f64) -> Self {
        Self {
            label: label.into(),
            action: action.into(),
            relevance: relevance.clamp(0.0, 1.0),
            reason: None,
            category: None,
            icon: None,
        }
    }

    /// Add reason for suggestion
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Suggestion;
    ///
    /// let suggestion = Suggestion::new("Save", "save", 0.9)
    ///     .with_reason("Unsaved changes");
    /// assert_eq!(suggestion.reason(), Some("Unsaved changes"));
    /// ```
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Add category
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Suggestion;
    ///
    /// let suggestion = Suggestion::new("Save", "save", 0.9)
    ///     .with_category("File");
    /// assert_eq!(suggestion.category(), Some("File"));
    /// ```
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Add icon
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::Suggestion;
    ///
    /// let suggestion = Suggestion::new("Save", "save", 0.9)
    ///     .with_icon("💾");
    /// assert_eq!(suggestion.icon(), Some("💾"));
    /// ```
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Get label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get action ID
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Get relevance score
    pub fn relevance(&self) -> f64 {
        self.relevance
    }

    /// Get reason
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Get category
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    /// Get icon
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// Set relevance score (mutable)
    pub fn set_relevance(&mut self, score: f64) {
        self.relevance = score.clamp(0.0, 1.0);
    }

    /// Get relevance color for display
    pub fn relevance_color(&self) -> Color {
        if self.relevance >= 0.8 {
            Color::Green
        } else if self.relevance >= 0.5 {
            Color::Yellow
        } else {
            Color::DarkGray
        }
    }
}

/// Smart suggestions widget
///
/// Displays context-aware suggestions sorted by relevance.
///
/// # Examples
///
/// ```
/// use toad::widgets::SmartSuggestions;
///
/// let mut suggestions = SmartSuggestions::new()
///     .add_suggestion("Save", "save", 0.95, Some("Unsaved changes"))
///     .add_suggestion("Test", "test", 0.85, None::<&str>)
///     .with_max_suggestions(5);
///
/// suggestions.next();
/// assert_eq!(suggestions.selected_action(), Some("test"));
/// ```
#[derive(Debug, Clone)]
pub struct SmartSuggestions {
    /// Suggestion items (sorted by relevance)
    suggestions: Vec<Suggestion>,
    /// Selected suggestion index
    selected: usize,
    /// Maximum suggestions to show
    max_suggestions: usize,
    /// Minimum relevance threshold
    min_relevance: f64,
    /// Show relevance scores
    show_scores: bool,
    /// Show reasons
    show_reasons: bool,
    /// Show categories
    show_categories: bool,
    /// Title
    title: Option<String>,
    /// Auto-sort by relevance
    auto_sort: bool,
}

impl Default for SmartSuggestions {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartSuggestions {
    /// Create a new smart suggestions widget
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SmartSuggestions;
    ///
    /// let suggestions = SmartSuggestions::new();
    /// assert_eq!(suggestions.suggestion_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            suggestions: Vec::new(),
            selected: 0,
            max_suggestions: 5,
            min_relevance: 0.0,
            show_scores: true,
            show_reasons: true,
            show_categories: false,
            title: Some("Suggestions".to_string()),
            auto_sort: true,
        }
    }

    /// Add a suggestion
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SmartSuggestions;
    ///
    /// let suggestions = SmartSuggestions::new()
    ///     .add_suggestion("Save", "save", 0.95, Some("Unsaved changes"));
    ///
    /// assert_eq!(suggestions.suggestion_count(), 1);
    /// ```
    pub fn add_suggestion(
        mut self,
        label: impl Into<String>,
        action: impl Into<String>,
        relevance: f64,
        reason: Option<impl Into<String>>,
    ) -> Self {
        let mut suggestion = Suggestion::new(label, action, relevance);
        if let Some(r) = reason {
            suggestion = suggestion.with_reason(r);
        }
        self.suggestions.push(suggestion);
        if self.auto_sort {
            self.sort_by_relevance();
        }
        self
    }

    /// Set maximum suggestions to display
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SmartSuggestions;
    ///
    /// let suggestions = SmartSuggestions::new()
    ///     .with_max_suggestions(10);
    /// ```
    pub fn with_max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max;
        self
    }

    /// Set minimum relevance threshold
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SmartSuggestions;
    ///
    /// let suggestions = SmartSuggestions::new()
    ///     .with_min_relevance(0.5);
    /// ```
    pub fn with_min_relevance(mut self, min: f64) -> Self {
        self.min_relevance = min.clamp(0.0, 1.0);
        self
    }

    /// Set whether to show relevance scores
    pub fn with_scores(mut self, show: bool) -> Self {
        self.show_scores = show;
        self
    }

    /// Set whether to show reasons
    pub fn with_reasons(mut self, show: bool) -> Self {
        self.show_reasons = show;
        self
    }

    /// Set whether to show categories
    pub fn with_categories(mut self, show: bool) -> Self {
        self.show_categories = show;
        self
    }

    /// Set title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set auto-sort behavior
    pub fn with_auto_sort(mut self, auto: bool) -> Self {
        self.auto_sort = auto;
        self
    }

    /// Get number of suggestions
    pub fn suggestion_count(&self) -> usize {
        self.filtered_suggestions().len()
    }

    /// Get all suggestions (filtered)
    pub fn suggestions(&self) -> Vec<&Suggestion> {
        self.filtered_suggestions()
    }

    /// Get filtered suggestions based on threshold and max count
    fn filtered_suggestions(&self) -> Vec<&Suggestion> {
        self.suggestions
            .iter()
            .filter(|s| s.relevance >= self.min_relevance)
            .take(self.max_suggestions)
            .collect()
    }

    /// Move selection down
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SmartSuggestions;
    ///
    /// let mut suggestions = SmartSuggestions::new()
    ///     .add_suggestion("First", "first", 0.9, None::<&str>)
    ///     .add_suggestion("Second", "second", 0.8, None::<&str>);
    ///
    /// suggestions.next();
    /// assert_eq!(suggestions.selected_index(), 1);
    /// ```
    pub fn next(&mut self) {
        let count = self.suggestion_count();
        if count > 0 {
            self.selected = (self.selected + 1) % count;
        }
    }

    /// Move selection up
    pub fn previous(&mut self) {
        let count = self.suggestion_count();
        if count > 0 {
            self.selected = if self.selected == 0 {
                count - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get selected suggestion
    pub fn selected_suggestion(&self) -> Option<&Suggestion> {
        self.filtered_suggestions().get(self.selected).copied()
    }

    /// Get selected action ID
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SmartSuggestions;
    ///
    /// let suggestions = SmartSuggestions::new()
    ///     .add_suggestion("Save", "save", 0.95, None::<&str>);
    ///
    /// assert_eq!(suggestions.selected_action(), Some("save"));
    /// ```
    pub fn selected_action(&self) -> Option<&str> {
        self.selected_suggestion().map(|s| s.action())
    }

    /// Add suggestion (mutable)
    pub fn push_suggestion(&mut self, suggestion: Suggestion) {
        self.suggestions.push(suggestion);
        if self.auto_sort {
            self.sort_by_relevance();
        }
    }

    /// Clear all suggestions
    pub fn clear(&mut self) {
        self.suggestions.clear();
        self.selected = 0;
    }

    /// Sort suggestions by relevance (descending)
    pub fn sort_by_relevance(&mut self) {
        self.suggestions
            .sort_by(|a, b| b.relevance.total_cmp(&a.relevance));
    }

    /// Update suggestion relevance by action
    pub fn update_relevance(&mut self, action: &str, new_relevance: f64) -> bool {
        if let Some(suggestion) = self.suggestions.iter_mut().find(|s| s.action == action) {
            suggestion.set_relevance(new_relevance);
            if self.auto_sort {
                self.sort_by_relevance();
            }
            true
        } else {
            false
        }
    }

    /// Render suggestion lines
    fn render_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let filtered = self.filtered_suggestions();

        for (i, suggestion) in filtered.iter().enumerate() {
            let is_selected = i == self.selected;
            let mut spans = Vec::new();

            // Selection indicator
            if is_selected {
                spans.push(
                    AtomText::new("> ")
                        .style(
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )
                        .to_span(),
                );
            } else {
                spans.push(AtomText::new("  ").to_span());
            }

            // Icon
            if let Some(icon) = suggestion.icon() {
                spans.push(AtomText::new(format!("{} ", icon)).to_span());
            }

            // Category
            if self.show_categories
                && let Some(category) = suggestion.category()
            {
                spans.push(
                    AtomText::new(format!("[{}] ", category))
                        .style(Style::default().fg(Color::DarkGray))
                        .to_span(),
                );
            }

            // Label
            let label_style = if is_selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(suggestion.relevance_color())
            };
            spans.push(
                AtomText::new(suggestion.label())
                    .style(label_style)
                    .to_span(),
            );

            // Relevance score
            if self.show_scores {
                let score_text = format!(" ({:.0}%)", suggestion.relevance() * 100.0);
                spans.push(
                    AtomText::new(score_text)
                        .style(Style::default().fg(suggestion.relevance_color()))
                        .to_span(),
                );
            }

            lines.push(Line::from(spans));

            // Reason on next line
            if self.show_reasons
                && let Some(reason) = suggestion.reason()
            {
                lines.push(Line::from(vec![
                    AtomText::new("    ").to_span(),
                    AtomText::new(reason)
                        .style(Style::default().fg(Color::DarkGray))
                        .to_span(),
                ]));
            }
        }

        if lines.is_empty() {
            lines.push(Line::from(vec![AtomText::new("  No suggestions available")
                .style(Style::default().fg(Color::DarkGray))
                .to_span()]));
        }

        lines
    }
}

impl Widget for &SmartSuggestions {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = AtomBlock::new()
            .borders(Borders::ALL)
            .style(Style::default());

        if let Some(title) = &self.title {
            block = block.title(title.clone());
        }

        let block = block.to_ratatui();
        let inner = block.inner(area);
        block.render(area, buf);

        let lines = self.render_lines();
        for (i, line) in lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_new() {
        let suggestion = Suggestion::new("Save", "save", 0.95);
        assert_eq!(suggestion.label(), "Save");
        assert_eq!(suggestion.action(), "save");
        assert_eq!(suggestion.relevance(), 0.95);
        assert_eq!(suggestion.reason(), None);
        assert_eq!(suggestion.category(), None);
        assert_eq!(suggestion.icon(), None);
    }

    #[test]
    fn test_suggestion_with_reason() {
        let suggestion = Suggestion::new("Save", "save", 0.9).with_reason("Unsaved changes");
        assert_eq!(suggestion.reason(), Some("Unsaved changes"));
    }

    #[test]
    fn test_suggestion_with_category() {
        let suggestion = Suggestion::new("Save", "save", 0.9).with_category("File");
        assert_eq!(suggestion.category(), Some("File"));
    }

    #[test]
    fn test_suggestion_with_icon() {
        let suggestion = Suggestion::new("Save", "save", 0.9).with_icon("💾");
        assert_eq!(suggestion.icon(), Some("💾"));
    }

    #[test]
    fn test_suggestion_relevance_clamp() {
        let suggestion1 = Suggestion::new("Test", "test", 1.5);
        assert_eq!(suggestion1.relevance(), 1.0);

        let suggestion2 = Suggestion::new("Test", "test", -0.5);
        assert_eq!(suggestion2.relevance(), 0.0);
    }

    #[test]
    fn test_suggestion_set_relevance() {
        let mut suggestion = Suggestion::new("Test", "test", 0.5);
        suggestion.set_relevance(0.8);
        assert_eq!(suggestion.relevance(), 0.8);

        suggestion.set_relevance(1.5);
        assert_eq!(suggestion.relevance(), 1.0);
    }

    #[test]
    fn test_suggestion_relevance_color() {
        let high = Suggestion::new("High", "high", 0.9);
        assert_eq!(high.relevance_color(), Color::Green);

        let medium = Suggestion::new("Medium", "medium", 0.6);
        assert_eq!(medium.relevance_color(), Color::Yellow);

        let low = Suggestion::new("Low", "low", 0.3);
        assert_eq!(low.relevance_color(), Color::DarkGray);
    }

    #[test]
    fn test_smart_suggestions_new() {
        let suggestions = SmartSuggestions::new();
        assert_eq!(suggestions.suggestion_count(), 0);
        assert_eq!(suggestions.selected_index(), 0);
        assert_eq!(suggestions.max_suggestions, 5);
        assert!(suggestions.show_scores);
        assert!(suggestions.show_reasons);
        assert!(suggestions.auto_sort);
    }

    #[test]
    fn test_smart_suggestions_default() {
        let suggestions = SmartSuggestions::default();
        assert_eq!(suggestions.suggestion_count(), 0);
    }

    #[test]
    fn test_smart_suggestions_add() {
        let suggestions = SmartSuggestions::new()
            .add_suggestion("Save", "save", 0.95, Some("Unsaved changes"))
            .add_suggestion("Test", "test", 0.85, None::<&str>);

        assert_eq!(suggestions.suggestion_count(), 2);
    }

    #[test]
    fn test_smart_suggestions_auto_sort() {
        let suggestions = SmartSuggestions::new()
            .add_suggestion("Low", "low", 0.3, None::<&str>)
            .add_suggestion("High", "high", 0.9, None::<&str>)
            .add_suggestion("Medium", "medium", 0.6, None::<&str>);

        let filtered = suggestions.suggestions();
        assert_eq!(filtered[0].action(), "high");
        assert_eq!(filtered[1].action(), "medium");
        assert_eq!(filtered[2].action(), "low");
    }

    #[test]
    fn test_smart_suggestions_with_max() {
        let suggestions = SmartSuggestions::new()
            .with_max_suggestions(2)
            .add_suggestion("First", "first", 0.9, None::<&str>)
            .add_suggestion("Second", "second", 0.8, None::<&str>)
            .add_suggestion("Third", "third", 0.7, None::<&str>);

        assert_eq!(suggestions.suggestion_count(), 2);
    }

    #[test]
    fn test_smart_suggestions_with_min_relevance() {
        let suggestions = SmartSuggestions::new()
            .with_min_relevance(0.5)
            .add_suggestion("High", "high", 0.9, None::<&str>)
            .add_suggestion("Medium", "medium", 0.6, None::<&str>)
            .add_suggestion("Low", "low", 0.3, None::<&str>);

        assert_eq!(suggestions.suggestion_count(), 2);
        let filtered = suggestions.suggestions();
        assert_eq!(filtered[0].action(), "high");
        assert_eq!(filtered[1].action(), "medium");
    }

    #[test]
    fn test_smart_suggestions_navigation() {
        let mut suggestions = SmartSuggestions::new()
            .add_suggestion("First", "first", 0.9, None::<&str>)
            .add_suggestion("Second", "second", 0.8, None::<&str>)
            .add_suggestion("Third", "third", 0.7, None::<&str>);

        assert_eq!(suggestions.selected_index(), 0);

        suggestions.next();
        assert_eq!(suggestions.selected_index(), 1);

        suggestions.next();
        assert_eq!(suggestions.selected_index(), 2);

        suggestions.next();
        assert_eq!(suggestions.selected_index(), 0); // Wrap around

        suggestions.previous();
        assert_eq!(suggestions.selected_index(), 2); // Wrap around
    }

    #[test]
    fn test_smart_suggestions_selected_action() {
        let suggestions =
            SmartSuggestions::new().add_suggestion("Save", "save", 0.95, None::<&str>);

        assert_eq!(suggestions.selected_action(), Some("save"));
    }

    #[test]
    fn test_smart_suggestions_push() {
        let mut suggestions = SmartSuggestions::new();
        suggestions.push_suggestion(Suggestion::new("Test", "test", 0.9));
        assert_eq!(suggestions.suggestion_count(), 1);
    }

    #[test]
    fn test_smart_suggestions_clear() {
        let mut suggestions = SmartSuggestions::new()
            .add_suggestion("First", "first", 0.9, None::<&str>)
            .add_suggestion("Second", "second", 0.8, None::<&str>);

        assert_eq!(suggestions.suggestion_count(), 2);
        suggestions.clear();
        assert_eq!(suggestions.suggestion_count(), 0);
        assert_eq!(suggestions.selected_index(), 0);
    }

    #[test]
    fn test_smart_suggestions_update_relevance() {
        let mut suggestions = SmartSuggestions::new()
            .add_suggestion("First", "first", 0.5, None::<&str>)
            .add_suggestion("Second", "second", 0.8, None::<&str>);

        assert!(suggestions.update_relevance("first", 0.9));

        // Should be auto-sorted
        let filtered = suggestions.suggestions();
        assert_eq!(filtered[0].action(), "first");
        assert_eq!(filtered[0].relevance(), 0.9);
    }

    #[test]
    fn test_smart_suggestions_builder_pattern() {
        let suggestions = SmartSuggestions::new()
            .add_suggestion("Save", "save", 0.95, Some("Unsaved"))
            .with_max_suggestions(10)
            .with_min_relevance(0.5)
            .with_scores(false)
            .with_reasons(false)
            .with_categories(true)
            .with_title("My Suggestions")
            .with_auto_sort(false);

        assert_eq!(suggestions.max_suggestions, 10);
        assert_eq!(suggestions.min_relevance, 0.5);
        assert!(!suggestions.show_scores);
        assert!(!suggestions.show_reasons);
        assert!(suggestions.show_categories);
        assert_eq!(suggestions.title, Some("My Suggestions".to_string()));
        assert!(!suggestions.auto_sort);
    }

    #[test]
    fn test_smart_suggestions_render_lines() {
        let suggestions =
            SmartSuggestions::new().add_suggestion("Save", "save", 0.95, Some("Unsaved changes"));

        let lines = suggestions.render_lines();
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_smart_suggestions_empty_render() {
        let suggestions = SmartSuggestions::new();
        let lines = suggestions.render_lines();
        assert_eq!(lines.len(), 1); // "No suggestions available"
    }
}
