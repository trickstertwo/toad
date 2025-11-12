//! Theme selector widget
//!
//! Allows users to browse and select from available themes

use crate::ui::{
    atoms::{block::Block, text::Text},
    theme::{ToadTheme, manager::ThemeName},
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{List, ListItem, ListState, Paragraph},
};

/// Theme selector widget
#[derive(Debug)]
pub struct ThemeSelector {
    /// List state for selection
    list_state: ListState,
    /// Available themes
    themes: Vec<ThemeName>,
}

impl ThemeSelector {
    pub fn new(current_theme: ThemeName) -> Self {
        let themes = ThemeName::all();

        // Find the index of the current theme
        let current_index = themes
            .iter()
            .position(|t| *t == current_theme)
            .unwrap_or(0);

        let mut list_state = ListState::default();
        list_state.select(Some(current_index));

        Self {
            list_state,
            themes,
        }
    }

    /// Select next theme
    pub fn select_next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.themes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Select previous theme
    pub fn select_previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.themes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Get the selected theme
    pub fn selected_theme(&self) -> Option<ThemeName> {
        self.list_state
            .selected()
            .map(|i| self.themes[i])
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_theme: ThemeName) {
        // Create centered modal-style layout
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(vertical[1]);

        let selector_area = horizontal[1];

        // Render selector using Block atom
        let selector_block = Block::new()
            .title(" Select Theme ")
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK))
            .to_ratatui();

        let inner = selector_block.inner(selector_area);
        frame.render_widget(selector_block, selector_area);

        // Split inner area: theme list + help text
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),    // Theme list
                Constraint::Length(2),  // Help text
            ])
            .split(inner);

        // Render theme list
        let items: Vec<ListItem> = self
            .themes
            .iter()
            .map(|theme| {
                let mut label = theme.as_str().to_string();

                // Add indicator for current theme
                if *theme == current_theme {
                    label.push_str(" ●");
                }

                let content = Line::from(vec![
                    Text::new("  ").to_span(),
                    Text::new(label)
                        .style(Style::default().fg(ToadTheme::FOREGROUND))
                        .to_span(),
                ]);
                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, chunks[0], &mut self.list_state);

        // Render help text
        let help = "↑↓ Navigate · Enter Apply · Esc Cancel";
        let help_line = Line::from(
            Text::new(help)
                .style(
                    Style::default()
                        .fg(ToadTheme::DARK_GRAY)
                        .add_modifier(Modifier::ITALIC),
                )
                .to_span(),
        );
        let help_paragraph = Paragraph::new(help_line).alignment(Alignment::Center);
        frame.render_widget(help_paragraph, chunks[1]);
    }
}

impl Default for ThemeSelector {
    fn default() -> Self {
        Self::new(ThemeName::Dark)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_selector_new() {
        let selector = ThemeSelector::new(ThemeName::Dark);
        assert!(selector.selected_theme().is_some());
    }

    #[test]
    fn test_theme_selector_default() {
        let selector = ThemeSelector::default();
        assert!(selector.selected_theme().is_some());
    }

    #[test]
    fn test_select_next() {
        let mut selector = ThemeSelector::new(ThemeName::Dark);
        let initial = selector.selected_theme();

        selector.select_next();
        let next = selector.selected_theme();

        assert_ne!(initial, next);
    }

    #[test]
    fn test_select_previous() {
        let mut selector = ThemeSelector::new(ThemeName::Light);
        let initial = selector.selected_theme();

        selector.select_previous();
        let previous = selector.selected_theme();

        assert_ne!(initial, previous);
    }

    #[test]
    fn test_wrap_around_next() {
        let mut selector = ThemeSelector::new(ThemeName::Dark);

        // Select last theme
        let themes = ThemeName::all();
        for _ in 0..themes.len() {
            selector.select_next();
        }

        // Should wrap around to first theme
        assert!(selector.selected_theme().is_some());
    }

    #[test]
    fn test_wrap_around_previous() {
        let mut selector = ThemeSelector::new(ThemeName::Dark);

        // From first theme, going previous should wrap to last
        selector.select_previous();

        assert!(selector.selected_theme().is_some());
    }

    #[test]
    fn test_theme_selector_debug() {
        let selector = ThemeSelector::new(ThemeName::Nord);
        let debug_str = format!("{:?}", selector);
        assert!(debug_str.contains("ThemeSelector"));
    }

    #[test]
    fn test_selected_theme_matches_current() {
        for theme in ThemeName::all() {
            let selector = ThemeSelector::new(theme);
            assert_eq!(selector.selected_theme(), Some(theme));
        }
    }

    #[test]
    fn test_themes_list_not_empty() {
        let selector = ThemeSelector::new(ThemeName::Dark);
        assert!(!selector.themes.is_empty());
    }

    #[test]
    fn test_themes_list_size() {
        let selector = ThemeSelector::new(ThemeName::Dark);
        assert_eq!(selector.themes.len(), ThemeName::all().len());
    }
}
