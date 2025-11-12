//! Settings screen widget
//!
//! Unified settings interface for theme selection, preferences, and configuration

use crate::ui::{
    atoms::{block::Block, text::Text},
    theme::{ToadTheme, manager::ThemeName, ResolvedThemeColors},
    widgets::core::theme_selector::ThemeSelector,
};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{List, ListItem, ListState, Paragraph, Tabs},
};

/// Settings category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    Theme,
    Editor,
    AI,
    Session,
}

impl SettingsCategory {
    fn as_str(&self) -> &str {
        match self {
            SettingsCategory::Theme => "Theme",
            SettingsCategory::Editor => "Editor",
            SettingsCategory::AI => "AI",
            SettingsCategory::Session => "Session",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            SettingsCategory::Theme,
            SettingsCategory::Editor,
            SettingsCategory::AI,
            SettingsCategory::Session,
        ]
    }
}

/// Settings screen widget
#[derive(Debug)]
pub struct SettingsScreen {
    /// Currently selected category
    category: SettingsCategory,
    /// Theme selector
    theme_selector: ThemeSelector,
}

impl SettingsScreen {
    pub fn new(current_theme: ThemeName) -> Self {
        Self {
            category: SettingsCategory::Theme,
            theme_selector: ThemeSelector::new(current_theme),
        }
    }

    /// Switch to next category
    pub fn next_category(&mut self) {
        let all = SettingsCategory::all();
        let current_idx = all.iter().position(|c| *c == self.category).unwrap_or(0);
        let next_idx = (current_idx + 1) % all.len();
        self.category = all[next_idx];
    }

    /// Switch to previous category
    pub fn previous_category(&mut self) {
        let all = SettingsCategory::all();
        let current_idx = all.iter().position(|c| *c == self.category).unwrap_or(0);
        let prev_idx = if current_idx == 0 {
            all.len() - 1
        } else {
            current_idx - 1
        };
        self.category = all[prev_idx];
    }

    /// Select next item in current category
    pub fn select_next(&mut self) {
        match self.category {
            SettingsCategory::Theme => self.theme_selector.select_next(),
            _ => {} // Other categories don't have selections yet
        }
    }

    /// Select previous item in current category
    pub fn select_previous(&mut self) {
        match self.category {
            SettingsCategory::Theme => self.theme_selector.select_previous(),
            _ => {} // Other categories don't have selections yet
        }
    }

    /// Get selected theme (if Theme category is active)
    pub fn selected_theme(&self) -> Option<ThemeName> {
        if self.category == SettingsCategory::Theme {
            self.theme_selector.selected_theme()
        } else {
            None
        }
    }

    /// Update theme selector with new current theme
    pub fn update_theme(&mut self, theme: ThemeName) {
        self.theme_selector = ThemeSelector::new(theme);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, current_theme: ThemeName) {
        // Create centered modal-style layout (slightly larger than theme selector)
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(vertical[1]);

        let settings_area = horizontal[1];

        // Render main settings block
        let settings_block = Block::new()
            .title(" Settings ")
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .style(Style::default().bg(ToadTheme::BLACK))
            .to_ratatui();

        let inner = settings_block.inner(settings_area);
        frame.render_widget(settings_block, settings_area);

        // Split into: tabs + content + help
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Tabs
                Constraint::Min(10),    // Content
                Constraint::Length(2),  // Help text
            ])
            .split(inner);

        // Render category tabs
        let categories = SettingsCategory::all();
        let tab_titles: Vec<Line> = categories
            .iter()
            .map(|cat| Line::from(cat.as_str()))
            .collect();

        let selected_idx = categories
            .iter()
            .position(|c| *c == self.category)
            .unwrap_or(0);

        let tabs = Tabs::new(tab_titles)
            .select(selected_idx)
            .highlight_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(tabs, chunks[0]);

        // Render content based on selected category
        match self.category {
            SettingsCategory::Theme => {
                self.render_theme_settings(frame, chunks[1], current_theme);
            }
            SettingsCategory::Editor => {
                self.render_editor_settings(frame, chunks[1]);
            }
            SettingsCategory::AI => {
                self.render_ai_settings(frame, chunks[1]);
            }
            SettingsCategory::Session => {
                self.render_session_settings(frame, chunks[1]);
            }
        }

        // Render help text
        let help = match self.category {
            SettingsCategory::Theme => "←→ Switch Tab · ↑↓ Navigate · Enter Apply · Esc Close",
            _ => "←→ Switch Tab · Esc Close",
        };

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
        frame.render_widget(help_paragraph, chunks[2]);
    }

    fn render_theme_settings(&mut self, frame: &mut Frame, area: Rect, current_theme: ThemeName) {
        // Get theme list from selector
        let all_themes = ThemeName::all();
        let theme_items: Vec<ListItem> = all_themes
            .iter()
            .map(|theme| {
                let mut label = theme.as_str().to_string();
                if *theme == current_theme {
                    label.push_str(" ●");
                }
                ListItem::new(Line::from(vec![
                    Text::new("  ").to_span(),
                    Text::new(label)
                        .style(Style::default().fg(ToadTheme::FOREGROUND))
                        .to_span(),
                ]))
            })
            .collect();

        let list = List::new(theme_items)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let mut list_state = ListState::default();
        if let Some(theme) = self.theme_selector.selected_theme() {
            let idx = all_themes.iter().position(|t| *t == theme);
            list_state.select(idx);
        }

        frame.render_stateful_widget(list, area, &mut list_state);
    }

    fn render_editor_settings(&self, frame: &mut Frame, area: Rect) {
        let content = vec![
            Line::from(""),
            Line::from(Text::new("Editor Settings").style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ).to_span()),
            Line::from(""),
            Line::from(Text::new("Coming soon...").style(
                Style::default().fg(ToadTheme::GRAY)
            ).to_span()),
            Line::from(""),
            Line::from("• Vim mode toggle"),
            Line::from("• Tab width"),
            Line::from("• Syntax highlighting"),
        ];

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, area);
    }

    fn render_ai_settings(&self, frame: &mut Frame, area: Rect) {
        let content = vec![
            Line::from(""),
            Line::from(Text::new("AI Settings").style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ).to_span()),
            Line::from(""),
            Line::from(Text::new("Coming soon...").style(
                Style::default().fg(ToadTheme::GRAY)
            ).to_span()),
            Line::from(""),
            Line::from("• Model selection"),
            Line::from("• Temperature"),
            Line::from("• Max tokens"),
            Line::from("• System prompt"),
        ];

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, area);
    }

    fn render_session_settings(&self, frame: &mut Frame, area: Rect) {
        let content = vec![
            Line::from(""),
            Line::from(Text::new("Session Settings").style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                    .add_modifier(Modifier::BOLD),
            ).to_span()),
            Line::from(""),
            Line::from(Text::new("Coming soon...").style(
                Style::default().fg(ToadTheme::GRAY)
            ).to_span()),
            Line::from(""),
            Line::from("• Auto-save"),
            Line::from("• History size"),
            Line::from("• Working directory"),
        ];

        let paragraph = Paragraph::new(content);
        frame.render_widget(paragraph, area);
    }
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self::new(ThemeName::Dark)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_screen_new() {
        let screen = SettingsScreen::new(ThemeName::Dark);
        assert_eq!(screen.category, SettingsCategory::Theme);
    }

    #[test]
    fn test_settings_screen_default() {
        let screen = SettingsScreen::default();
        assert_eq!(screen.category, SettingsCategory::Theme);
    }

    #[test]
    fn test_next_category() {
        let mut screen = SettingsScreen::new(ThemeName::Dark);
        assert_eq!(screen.category, SettingsCategory::Theme);

        screen.next_category();
        assert_eq!(screen.category, SettingsCategory::Editor);

        screen.next_category();
        assert_eq!(screen.category, SettingsCategory::AI);

        screen.next_category();
        assert_eq!(screen.category, SettingsCategory::Session);

        screen.next_category();
        assert_eq!(screen.category, SettingsCategory::Theme); // Wraps
    }

    #[test]
    fn test_previous_category() {
        let mut screen = SettingsScreen::new(ThemeName::Dark);
        assert_eq!(screen.category, SettingsCategory::Theme);

        screen.previous_category();
        assert_eq!(screen.category, SettingsCategory::Session); // Wraps

        screen.previous_category();
        assert_eq!(screen.category, SettingsCategory::AI);

        screen.previous_category();
        assert_eq!(screen.category, SettingsCategory::Editor);

        screen.previous_category();
        assert_eq!(screen.category, SettingsCategory::Theme);
    }

    #[test]
    fn test_select_theme() {
        let mut screen = SettingsScreen::new(ThemeName::Dark);
        assert_eq!(screen.category, SettingsCategory::Theme);

        screen.select_next();
        let theme = screen.selected_theme();
        assert!(theme.is_some());
    }

    #[test]
    fn test_selected_theme_none_when_not_theme_category() {
        let mut screen = SettingsScreen::new(ThemeName::Dark);
        screen.next_category(); // Switch to Editor

        assert_eq!(screen.selected_theme(), None);
    }

    #[test]
    fn test_update_theme() {
        let mut screen = SettingsScreen::new(ThemeName::Dark);

        screen.update_theme(ThemeName::Nord);
        let theme = screen.selected_theme();
        assert_eq!(theme, Some(ThemeName::Nord));
    }

    #[test]
    fn test_settings_category_as_str() {
        assert_eq!(SettingsCategory::Theme.as_str(), "Theme");
        assert_eq!(SettingsCategory::Editor.as_str(), "Editor");
        assert_eq!(SettingsCategory::AI.as_str(), "AI");
        assert_eq!(SettingsCategory::Session.as_str(), "Session");
    }

    #[test]
    fn test_settings_category_all() {
        let all = SettingsCategory::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&SettingsCategory::Theme));
        assert!(all.contains(&SettingsCategory::Editor));
        assert!(all.contains(&SettingsCategory::AI));
        assert!(all.contains(&SettingsCategory::Session));
    }
}
