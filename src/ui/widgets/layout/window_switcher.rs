//! Window switcher widget for multi-window navigation
//!
//! Provides Alt+Tab style window switching UI with previews.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::WindowSwitcher;
//! use toad::ui::multi_window::WindowManager;
//!
//! let mut manager = WindowManager::new();
//! manager.create_window("Window 1");
//! manager.create_window("Window 2");
//!
//! let switcher = WindowSwitcher::new(&manager);
//! ```

use crate::ui::layout::multi_window::{Window, WindowManager, WindowPriority, WindowState};
use crate::ui::{atoms::{block::Block as AtomBlock, text::Text as AtomText}, theme::ToadTheme};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Borders, List, ListItem, ListState, Paragraph, Wrap},
};

/// Window switcher display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitcherMode {
    /// Compact list view
    Compact,
    /// Grid view with previews
    Grid,
    /// Detailed list with metadata
    Detailed,
}

/// Window switcher widget
///
/// Displays active windows with keyboard navigation for quick switching.
#[derive(Debug)]
pub struct WindowSwitcher {
    /// Display mode
    mode: SwitcherMode,
    /// Selected window index
    selected: usize,
    /// Whether to show previews
    show_previews: bool,
    /// Whether to show only unsaved windows
    filter_unsaved: bool,
    /// ListState for rendering
    list_state: ListState,
}

impl WindowSwitcher {
    /// Create a new window switcher
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::WindowSwitcher;
    ///
    /// let switcher = WindowSwitcher::new();
    /// ```
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            mode: SwitcherMode::Compact,
            selected: 0,
            show_previews: true,
            filter_unsaved: false,
            list_state,
        }
    }

    /// Set display mode
    pub fn mode(mut self, mode: SwitcherMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set whether to show previews
    pub fn show_previews(mut self, show: bool) -> Self {
        self.show_previews = show;
        self
    }

    /// Set unsaved filter
    pub fn filter_unsaved(mut self, filter: bool) -> Self {
        self.filter_unsaved = filter;
        self
    }

    /// Get selected index
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Select next window
    pub fn next(&mut self, window_count: usize) {
        if window_count > 0 {
            self.selected = (self.selected + 1) % window_count;
            self.list_state.select(Some(self.selected));
        }
    }

    /// Select previous window
    pub fn previous(&mut self, window_count: usize) {
        if window_count > 0 {
            self.selected = if self.selected == 0 {
                window_count - 1
            } else {
                self.selected - 1
            };
            self.list_state.select(Some(self.selected));
        }
    }

    /// Select window by index
    pub fn select(&mut self, index: usize) {
        self.selected = index;
        self.list_state.select(Some(index));
    }

    /// Render the window switcher
    pub fn render(&mut self, frame: &mut Frame, area: Rect, manager: &WindowManager) {
        let windows: Vec<&Window> = if self.filter_unsaved {
            manager.unsaved_windows()
        } else {
            manager.windows_mru()
        };

        if windows.is_empty() {
            self.render_empty(frame, area);
            return;
        }

        // Ensure selected is in bounds
        if self.selected >= windows.len() {
            self.selected = 0;
            self.list_state.select(Some(0));
        }

        match self.mode {
            SwitcherMode::Compact => self.render_compact(frame, area, &windows),
            SwitcherMode::Grid => self.render_grid(frame, area, &windows),
            SwitcherMode::Detailed => self.render_detailed(frame, area, &windows),
        }
    }

    /// Render empty state
    fn render_empty(&self, frame: &mut Frame, area: Rect) {
        let block = AtomBlock::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY))
            .title(" Window Switcher ")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .to_ratatui();

        let text = Paragraph::new("No windows open")
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(ToadTheme::DARK_GRAY));

        frame.render_widget(text, area);
    }

    /// Render compact list view
    fn render_compact(&mut self, frame: &mut Frame, area: Rect, windows: &[&Window]) {
        let items: Vec<ListItem> = windows
            .iter()
            .enumerate()
            .map(|(idx, window)| {
                let priority_icon = match window.priority() {
                    WindowPriority::Low => "▼",
                    WindowPriority::Normal => "●",
                    WindowPriority::High => "▲",
                    WindowPriority::Urgent => "⚠",
                };

                let unsaved = if window.has_unsaved_changes() {
                    " ●"
                } else {
                    ""
                };

                let workspace = window
                    .workspace()
                    .and_then(|w| std::path::Path::new(w).file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                let workspace_text = if !workspace.is_empty() {
                    format!(" [{}]", workspace)
                } else {
                    String::new()
                };

                let content = format!(
                    "{} {} {}{}{unsaved}",
                    idx + 1,
                    priority_icon,
                    window.title(),
                    workspace_text
                );

                ListItem::new(content)
            })
            .collect();

        let block = AtomBlock::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title(" Windows (Ctrl+Tab) ")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .to_ratatui();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN)
                    .fg(ToadTheme::BACKGROUND)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    /// Render grid view with previews
    fn render_grid(&mut self, frame: &mut Frame, area: Rect, windows: &[&Window]) {
        // Split into list and preview
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        // Render window list
        self.render_compact(frame, chunks[0], windows);

        // Render preview of selected window
        if self.selected < windows.len() {
            self.render_preview(frame, chunks[1], windows[self.selected]);
        }
    }

    /// Render detailed list view
    fn render_detailed(&mut self, frame: &mut Frame, area: Rect, windows: &[&Window]) {
        let items: Vec<ListItem> = windows
            .iter()
            .enumerate()
            .map(|(idx, window)| {
                let priority_color = match window.priority() {
                    WindowPriority::Low => ToadTheme::DARK_GRAY,
                    WindowPriority::Normal => ToadTheme::FOREGROUND,
                    WindowPriority::High => ratatui::style::Color::Yellow,
                    WindowPriority::Urgent => ratatui::style::Color::Red,
                };

                let state_text = match window.state() {
                    WindowState::Active => "●",
                    WindowState::Background => "○",
                    WindowState::Minimized => "–",
                    WindowState::Closing => "✖",
                };

                let unsaved = if window.has_unsaved_changes() {
                    " [unsaved]"
                } else {
                    ""
                };

                let workspace = window.workspace().unwrap_or("—");

                let idle = window.idle_time().as_secs();
                let idle_text = if idle < 60 {
                    format!("{}s", idle)
                } else {
                    format!("{}m", idle / 60)
                };

                let lines = vec![
                    Line::from(vec![
                        AtomText::new(format!("{}. ", idx + 1))
                            .style(Style::default().fg(ToadTheme::DARK_GRAY))
                            .to_span(),
                        AtomText::new(state_text)
                            .style(Style::default().fg(priority_color))
                            .to_span(),
                        AtomText::new(" ").to_span(),
                        AtomText::new(window.title())
                            .style(
                                Style::default()
                                    .fg(priority_color)
                                    .add_modifier(Modifier::BOLD),
                            )
                            .to_span(),
                        AtomText::new(unsaved)
                            .style(Style::default().fg(ratatui::style::Color::Yellow))
                            .to_span(),
                    ]),
                    Line::from(vec![
                        AtomText::new("   ").to_span(),
                        AtomText::new(format!("📁 {} ", workspace))
                            .style(Style::default().fg(ToadTheme::DARK_GRAY))
                            .to_span(),
                        AtomText::new(format!("⏱ {}", idle_text))
                            .style(Style::default().fg(ToadTheme::DARK_GRAY))
                            .to_span(),
                    ]),
                ];

                ListItem::new(lines)
            })
            .collect();

        let block = AtomBlock::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title(" Windows (Detailed) ")
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .to_ratatui();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN)
                    .fg(ToadTheme::BACKGROUND),
            )
            .highlight_symbol("▶ ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    /// Render preview pane
    fn render_preview(&self, frame: &mut Frame, area: Rect, window: &Window) {
        let block = AtomBlock::new()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY))
            .title(format!(" {} ", window.title()))
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .to_ratatui();

        let preview_text = window.preview_text().unwrap_or("No preview available");

        let paragraph = Paragraph::new(preview_text)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(ToadTheme::FOREGROUND));

        frame.render_widget(paragraph, area);
    }
}

impl Default for WindowSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_switcher_creation() {
        let switcher = WindowSwitcher::new();
        assert_eq!(switcher.selected(), 0);
        assert_eq!(switcher.mode, SwitcherMode::Compact);
        assert!(switcher.show_previews);
    }

    #[test]
    fn test_switcher_navigation() {
        let mut switcher = WindowSwitcher::new();

        switcher.next(3);
        assert_eq!(switcher.selected(), 1);

        switcher.next(3);
        assert_eq!(switcher.selected(), 2);

        switcher.next(3);
        assert_eq!(switcher.selected(), 0); // Wraps around

        switcher.previous(3);
        assert_eq!(switcher.selected(), 2); // Wraps backward
    }

    #[test]
    fn test_switcher_select() {
        let mut switcher = WindowSwitcher::new();
        switcher.select(5);
        assert_eq!(switcher.selected(), 5);
    }

    #[test]
    fn test_switcher_modes() {
        let switcher = WindowSwitcher::new().mode(SwitcherMode::Grid);
        assert_eq!(switcher.mode, SwitcherMode::Grid);

        let switcher = WindowSwitcher::new().mode(SwitcherMode::Detailed);
        assert_eq!(switcher.mode, SwitcherMode::Detailed);
    }

    #[test]
    fn test_switcher_filters() {
        let switcher = WindowSwitcher::new().filter_unsaved(true);
        assert!(switcher.filter_unsaved);

        let switcher = WindowSwitcher::new().show_previews(false);
        assert!(!switcher.show_previews);
    }
}
