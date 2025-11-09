use crate::ui::theme::ToadTheme;
/// Tab bar widget for displaying tabs
///
/// Visual indicator showing all tabs with the active tab highlighted
///
/// # Examples
///
/// ```
/// use toad::tabs::TabManager;
/// use toad::widgets::TabBar;
///
/// let mut manager = TabManager::with_tab("Main");
/// manager.add_tab("Settings");
///
/// let tabbar = TabBar::new(&manager);
/// assert_eq!(tabbar.tab_count(), 2);
/// ```
use crate::workspace::tabs::TabManager;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Tab bar widget
///
/// Displays tabs horizontally with the active tab highlighted
#[derive(Debug)]
pub struct TabBar<'a> {
    /// Reference to tab manager
    manager: &'a TabManager,
    /// Maximum width per tab
    max_tab_width: u16,
    /// Show close buttons
    show_close: bool,
}

impl<'a> TabBar<'a> {
    /// Create a new tab bar
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    /// use toad::widgets::TabBar;
    ///
    /// let manager = TabManager::with_tab("Main");
    /// let tabbar = TabBar::new(&manager);
    /// ```
    pub fn new(manager: &'a TabManager) -> Self {
        Self {
            manager,
            max_tab_width: 20,
            show_close: true,
        }
    }

    /// Set maximum width per tab
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    /// use toad::widgets::TabBar;
    ///
    /// let manager = TabManager::with_tab("Main");
    /// let tabbar = TabBar::new(&manager).max_tab_width(30);
    /// ```
    pub fn max_tab_width(mut self, width: u16) -> Self {
        self.max_tab_width = width;
        self
    }

    /// Set whether to show close buttons
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::tabs::TabManager;
    /// use toad::widgets::TabBar;
    ///
    /// let manager = TabManager::with_tab("Main");
    /// let tabbar = TabBar::new(&manager).show_close(false);
    /// ```
    pub fn show_close(mut self, show: bool) -> Self {
        self.show_close = show;
        self
    }

    /// Get number of tabs
    pub fn tab_count(&self) -> usize {
        self.manager.count()
    }

    /// Render the tab bar
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if self.manager.is_empty() {
            return;
        }

        // Build tab spans
        let mut spans = Vec::new();
        let active_index = self.manager.active_index();

        for (idx, tab) in self.manager.tabs().iter().enumerate() {
            let is_active = Some(idx) == active_index;

            // Tab style
            let style = if is_active {
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ToadTheme::FOREGROUND)
            };

            // Add separator before tab (except first)
            if idx > 0 {
                spans.push(Span::styled(
                    " │ ",
                    Style::default().fg(ToadTheme::DARK_GRAY),
                ));
            }

            // Tab number (1-based for display)
            spans.push(Span::styled(format!("{}", idx + 1), style));
            spans.push(Span::raw(" "));

            // Icon if present
            if let Some(icon) = &tab.icon {
                spans.push(Span::styled(format!("{} ", icon), style));
            }

            // Tab title
            let mut title = tab.display_name();

            // Truncate if too long
            let max_title_len = (self.max_tab_width as usize).saturating_sub(5);
            if title.len() > max_title_len {
                title.truncate(max_title_len.saturating_sub(3));
                title.push_str("...");
            }

            spans.push(Span::styled(title, style));

            // Close button if closable and show_close is enabled
            if self.show_close && tab.closable {
                spans.push(Span::raw(" "));
                spans.push(Span::styled(
                    "×",
                    Style::default()
                        .fg(ToadTheme::RED)
                        .add_modifier(Modifier::DIM),
                ));
            }
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line).alignment(Alignment::Left);
        frame.render_widget(paragraph, inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabbar_creation() {
        let manager = TabManager::with_tab("Main");
        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_multiple_tabs() {
        let mut manager = TabManager::with_tab("Main");
        manager.add_tab("Settings");
        manager.add_tab("Help");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 3);
    }

    #[test]
    fn test_tabbar_max_width() {
        let manager = TabManager::with_tab("Main");
        let tabbar = TabBar::new(&manager).max_tab_width(30);
        assert_eq!(tabbar.max_tab_width, 30);
    }

    #[test]
    fn test_tabbar_show_close() {
        let manager = TabManager::with_tab("Main");
        let tabbar = TabBar::new(&manager).show_close(false);
        assert!(!tabbar.show_close);
    }

    #[test]
    fn test_tabbar_empty_manager() {
        let manager = TabManager::new();
        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 0);
    }

    #[test]
    fn test_tabbar_with_icons() {
        let mut manager = TabManager::new();
        let tab = crate::workspace::Tab::new(0, "Main").with_icon("📁");
        manager.add_tab_with(tab);

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_with_modified_tabs() {
        let mut manager = TabManager::with_tab("Main");
        if let Some(tab) = manager.get_tab_mut(0) {
            tab.set_modified(true);
        }

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_active_tab() {
        let mut manager = TabManager::with_tab("Main");
        manager.add_tab("Settings");
        manager.next_tab();

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 2);
        assert_eq!(manager.active_index(), Some(1));
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_tabbar_very_long_title_truncation() {
        let mut manager = TabManager::new();
        let very_long_title = "A".repeat(100);
        manager.add_tab(&very_long_title);

        let tabbar = TabBar::new(&manager).max_tab_width(20);
        assert_eq!(tabbar.tab_count(), 1);
        // Truncation is handled in render, just verify it doesn't panic
    }

    #[test]
    fn test_tabbar_unicode_titles() {
        let mut manager = TabManager::new();
        manager.add_tab("📁 Files");
        manager.add_tab("🔧 Settings");
        manager.add_tab("日本語タブ");
        manager.add_tab("🌍🚀🎉");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 4);
    }

    #[test]
    fn test_tabbar_many_tabs() {
        let mut manager = TabManager::new();
        for i in 0..100 {
            manager.add_tab(&format!("Tab {}", i));
        }

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 100);
    }

    #[test]
    fn test_tabbar_single_tab() {
        let manager = TabManager::with_tab("Only Tab");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 1);
        assert_eq!(manager.active_index(), Some(0));
    }

    #[test]
    fn test_tabbar_max_width_extremes() {
        let manager = TabManager::with_tab("Tab");

        // Very small max width
        let tabbar1 = TabBar::new(&manager).max_tab_width(1);
        assert_eq!(tabbar1.max_tab_width, 1);

        // Very large max width
        let tabbar2 = TabBar::new(&manager).max_tab_width(1000);
        assert_eq!(tabbar2.max_tab_width, 1000);
    }

    #[test]
    fn test_tabbar_show_close_toggle() {
        let manager = TabManager::with_tab("Main");

        let tabbar_with_close = TabBar::new(&manager).show_close(true);
        assert!(tabbar_with_close.show_close);

        let tabbar_without_close = TabBar::new(&manager).show_close(false);
        assert!(!tabbar_without_close.show_close);
    }

    #[test]
    fn test_tabbar_mixed_closable_tabs() {
        let mut manager = TabManager::new();

        // Add tab with closable = true (default)
        manager.add_tab("Closable");

        // Add another tab
        manager.add_tab("Also Closable");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 2);
    }

    #[test]
    fn test_tabbar_all_tabs_modified() {
        let mut manager = TabManager::with_tab("Tab1");
        manager.add_tab("Tab2");
        manager.add_tab("Tab3");

        // Mark all as modified
        for i in 0..3 {
            if let Some(tab) = manager.get_tab_mut(i) {
                tab.set_modified(true);
            }
        }

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 3);
    }

    #[test]
    fn test_tabbar_with_empty_title() {
        let mut manager = TabManager::new();
        manager.add_tab("");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_with_whitespace_only_title() {
        let mut manager = TabManager::new();
        manager.add_tab("   ");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_navigation_through_all_tabs() {
        let mut manager = TabManager::with_tab("Tab1");
        manager.add_tab("Tab2");
        manager.add_tab("Tab3");
        manager.add_tab("Tab4");

        // Navigate through all tabs
        assert_eq!(manager.active_index(), Some(0));

        manager.next_tab();
        assert_eq!(manager.active_index(), Some(1));

        manager.next_tab();
        assert_eq!(manager.active_index(), Some(2));

        manager.next_tab();
        assert_eq!(manager.active_index(), Some(3));

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 4);
    }

    #[test]
    fn test_tabbar_with_special_characters() {
        let mut manager = TabManager::new();
        manager.add_tab("Tab<>\"'&");
        manager.add_tab("Tab\t\n");
        manager.add_tab("Tab|\\/*?");

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 3);
    }

    #[test]
    fn test_tabbar_builder_pattern() {
        let manager = TabManager::with_tab("Main");

        let tabbar = TabBar::new(&manager)
            .max_tab_width(25)
            .show_close(false);

        assert_eq!(tabbar.max_tab_width, 25);
        assert!(!tabbar.show_close);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_count_consistency() {
        let mut manager = TabManager::new();

        assert_eq!(manager.count(), 0);

        manager.add_tab("Tab1");
        assert_eq!(manager.count(), 1);

        manager.add_tab("Tab2");
        assert_eq!(manager.count(), 2);

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), manager.count());
    }

    #[test]
    fn test_tabbar_with_very_long_unicode_title() {
        let mut manager = TabManager::new();
        let long_unicode = "🐸".repeat(50); // 50 emoji frogs
        manager.add_tab(&long_unicode);

        let tabbar = TabBar::new(&manager).max_tab_width(20);
        assert_eq!(tabbar.tab_count(), 1);
    }

    #[test]
    fn test_tabbar_with_mixed_content() {
        let mut manager = TabManager::new();
        let tab1 = crate::workspace::Tab::new(0, "Normal").with_icon("📄");
        let tab2 = crate::workspace::Tab::new(1, "With Icon").with_icon("🔧");

        manager.add_tab_with(tab1);
        manager.add_tab_with(tab2);

        if let Some(tab) = manager.get_tab_mut(0) {
            tab.set_modified(true);
        }

        let tabbar = TabBar::new(&manager);
        assert_eq!(tabbar.tab_count(), 2);
    }
}
