//! ModelSelector molecule - AI model selection display
//!
//! Composes Text and Icon atoms to display available AI models with selection indicator.
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - **Molecule**: Composes Text and Icon atoms
//! - **Pure**: No mutable state for rendering, data passed in
//! - **Composable**: Used by organisms for model management
//!
//! # Examples
//!
//! ```
//! use toad::ui::molecules::model_selector::ModelSelector;
//!
//! let models = vec!["claude-3-sonnet", "claude-3-opus", "gpt-4"];
//! let selector = ModelSelector::new(&models, 0);
//! ```

use crate::ui::atoms::{Icon, Text};
use crate::ui::nerd_fonts::UiIcon;
use crate::ui::theme::ToadTheme;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

/// AI model selector
///
/// Displays a list of available AI models with a visual indicator for the currently selected model.
/// Supports multiple display modes (compact, detailed) and custom styling.
///
/// # Examples
///
/// ```
/// use toad::ui::molecules::model_selector::ModelSelector;
///
/// let models = vec!["claude-3-sonnet", "claude-3-opus"];
/// let selector = ModelSelector::new(&models, 0);
/// let lines = selector.to_lines();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ModelSelector<'a> {
    /// Available models
    models: &'a [&'a str],
    /// Currently selected index
    selected_index: usize,
    /// Show detailed info (costs, capabilities)
    show_details: bool,
    /// Highlight selected model
    highlight: bool,
    /// Custom style for selected model
    selected_style: Option<Style>,
    /// Custom style for unselected models
    unselected_style: Option<Style>,
}

impl<'a> ModelSelector<'a> {
    /// Create a new model selector
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    ///
    /// let models = vec!["claude-3-sonnet", "gpt-4"];
    /// let selector = ModelSelector::new(&models, 0);
    /// assert_eq!(selector.selected_index(), 0);
    /// ```
    pub fn new(models: &'a [&'a str], selected_index: usize) -> Self {
        Self {
            models,
            selected_index: selected_index.min(models.len().saturating_sub(1)),
            show_details: false,
            highlight: true,
            selected_style: None,
            unselected_style: None,
        }
    }

    /// Show detailed information
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    ///
    /// let models = vec!["claude-3-sonnet"];
    /// let selector = ModelSelector::new(&models, 0).with_details(true);
    /// ```
    pub fn with_details(mut self, show: bool) -> Self {
        self.show_details = show;
        self
    }

    /// Enable/disable highlighting
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    ///
    /// let models = vec!["claude-3-sonnet"];
    /// let selector = ModelSelector::new(&models, 0).with_highlight(false);
    /// ```
    pub fn with_highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Set custom style for selected model
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let models = vec!["claude-3-sonnet"];
    /// let selector = ModelSelector::new(&models, 0)
    ///     .selected_style(Style::default().fg(ToadTheme::TOAD_GREEN));
    /// ```
    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = Some(style);
        self
    }

    /// Set custom style for unselected models
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    /// use toad::ui::theme::ToadTheme;
    /// use ratatui::style::Style;
    ///
    /// let models = vec!["claude-3-sonnet"];
    /// let selector = ModelSelector::new(&models, 0)
    ///     .unselected_style(Style::default().fg(ToadTheme::GRAY));
    /// ```
    pub fn unselected_style(mut self, style: Style) -> Self {
        self.unselected_style = Some(style);
        self
    }

    /// Get selected model index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get selected model name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    ///
    /// let models = vec!["claude-3-sonnet", "gpt-4"];
    /// let selector = ModelSelector::new(&models, 1);
    /// assert_eq!(selector.selected_model(), Some("gpt-4"));
    /// ```
    pub fn selected_model(&self) -> Option<&'a str> {
        self.models.get(self.selected_index).copied()
    }

    /// Get total number of models
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Convert to lines for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    ///
    /// let models = vec!["claude-3-sonnet", "gpt-4"];
    /// let selector = ModelSelector::new(&models, 0);
    /// let lines = selector.to_lines();
    /// assert_eq!(lines.len(), 2);
    /// ```
    pub fn to_lines(&self) -> Vec<Line<'static>> {
        self.models
            .iter()
            .enumerate()
            .map(|(index, model)| {
                let is_selected = index == self.selected_index;
                self.render_model_line(model, is_selected)
            })
            .collect()
    }

    /// Convert to a single line (compact mode)
    ///
    /// Shows only the selected model with an indicator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::molecules::model_selector::ModelSelector;
    ///
    /// let models = vec!["claude-3-sonnet", "gpt-4"];
    /// let selector = ModelSelector::new(&models, 0);
    /// let line = selector.to_compact_line();
    /// ```
    pub fn to_compact_line(&self) -> Line<'static> {
        let model = self.selected_model().unwrap_or("None");
        let mut spans = Vec::new();

        // Icon
        let icon = Icon::ui(UiIcon::Star).style(Style::default().fg(ToadTheme::TOAD_GREEN));
        spans.push(icon.to_text().to_span());
        spans.push(Span::raw(" "));

        // Model name
        let style = self
            .selected_style
            .unwrap_or(Style::default().fg(ToadTheme::TOAD_GREEN).add_modifier(Modifier::BOLD));
        spans.push(Text::new(model).style(style).to_span());

        Line::from(spans)
    }

    /// Render a single model line
    fn render_model_line(&self, model: &str, is_selected: bool) -> Line<'static> {
        let mut spans = Vec::new();

        // Selection indicator
        if is_selected {
            let icon = Icon::ui(UiIcon::RadioChecked)
                .style(Style::default().fg(ToadTheme::TOAD_GREEN));
            spans.push(icon.to_text().to_span());
        } else {
            let icon =
                Icon::ui(UiIcon::RadioUnchecked).style(Style::default().fg(ToadTheme::GRAY));
            spans.push(icon.to_text().to_span());
        }
        spans.push(Span::raw(" "));

        // Model name
        let style = if is_selected {
            self.selected_style.unwrap_or(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            self.unselected_style
                .unwrap_or(Style::default().fg(ToadTheme::GRAY))
        };

        spans.push(Text::new(model).style(style).to_span());

        Line::from(spans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_selector_new() {
        let models = vec!["claude-3-sonnet", "gpt-4"];
        let selector = ModelSelector::new(&models, 0);
        assert_eq!(selector.selected_index(), 0);
        assert_eq!(selector.model_count(), 2);
    }

    #[test]
    fn test_model_selector_selected_model() {
        let models = vec!["claude-3-sonnet", "gpt-4", "claude-3-opus"];
        let selector = ModelSelector::new(&models, 1);
        assert_eq!(selector.selected_model(), Some("gpt-4"));
    }

    #[test]
    fn test_model_selector_invalid_index() {
        let models = vec!["claude-3-sonnet"];
        let selector = ModelSelector::new(&models, 100);
        // Should clamp to last valid index
        assert_eq!(selector.selected_index(), 0);
    }

    #[test]
    fn test_model_selector_empty_models() {
        let models: Vec<&str> = vec![];
        let selector = ModelSelector::new(&models, 0);
        assert_eq!(selector.model_count(), 0);
        assert_eq!(selector.selected_model(), None);
    }

    #[test]
    fn test_model_selector_with_details() {
        let models = vec!["claude-3-sonnet"];
        let selector = ModelSelector::new(&models, 0).with_details(true);
        assert!(selector.show_details);
    }

    #[test]
    fn test_model_selector_with_highlight() {
        let models = vec!["claude-3-sonnet"];
        let selector = ModelSelector::new(&models, 0).with_highlight(false);
        assert!(!selector.highlight);
    }

    #[test]
    fn test_model_selector_selected_style() {
        let models = vec!["claude-3-sonnet"];
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let selector = ModelSelector::new(&models, 0).selected_style(style);
        assert_eq!(selector.selected_style, Some(style));
    }

    #[test]
    fn test_model_selector_unselected_style() {
        let models = vec!["claude-3-sonnet"];
        let style = Style::default().fg(ToadTheme::GRAY);
        let selector = ModelSelector::new(&models, 0).unselected_style(style);
        assert_eq!(selector.unselected_style, Some(style));
    }

    #[test]
    fn test_model_selector_to_lines() {
        let models = vec!["claude-3-sonnet", "gpt-4", "claude-3-opus"];
        let selector = ModelSelector::new(&models, 1);
        let lines = selector.to_lines();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_model_selector_to_compact_line() {
        let models = vec!["claude-3-sonnet", "gpt-4"];
        let selector = ModelSelector::new(&models, 0);
        let _line = selector.to_compact_line();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_model_selector_chaining() {
        let models = vec!["claude-3-sonnet"];
        let style = Style::default().fg(ToadTheme::TOAD_GREEN);
        let selector = ModelSelector::new(&models, 0)
            .with_details(true)
            .with_highlight(false)
            .selected_style(style)
            .unselected_style(style);

        assert!(selector.show_details);
        assert!(!selector.highlight);
        assert_eq!(selector.selected_style, Some(style));
        assert_eq!(selector.unselected_style, Some(style));
    }

    #[test]
    fn test_model_selector_clone() {
        let models = vec!["claude-3-sonnet"];
        let selector1 = ModelSelector::new(&models, 0);
        let selector2 = selector1.clone();
        assert_eq!(selector1, selector2);
    }

    #[test]
    fn test_model_selector_equality() {
        let models1 = vec!["claude-3-sonnet"];
        let models2 = vec!["claude-3-sonnet"];
        let selector1 = ModelSelector::new(&models1, 0);
        let selector2 = ModelSelector::new(&models2, 0);
        assert_eq!(selector1, selector2);
    }

    #[test]
    fn test_model_selector_single_model() {
        let models = vec!["claude-3-sonnet"];
        let selector = ModelSelector::new(&models, 0);
        assert_eq!(selector.model_count(), 1);
        assert_eq!(selector.selected_model(), Some("claude-3-sonnet"));
    }

    #[test]
    fn test_model_selector_many_models() {
        let models = vec![
            "claude-3-sonnet",
            "claude-3-opus",
            "gpt-4",
            "gpt-3.5-turbo",
            "llama-2",
        ];
        let selector = ModelSelector::new(&models, 2);
        assert_eq!(selector.model_count(), 5);
        assert_eq!(selector.selected_model(), Some("gpt-4"));
        let lines = selector.to_lines();
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_model_selector_last_model() {
        let models = vec!["model1", "model2", "model3"];
        let selector = ModelSelector::new(&models, 2);
        assert_eq!(selector.selected_index(), 2);
        assert_eq!(selector.selected_model(), Some("model3"));
    }
}
