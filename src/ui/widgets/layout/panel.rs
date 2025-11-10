/// Panel widget with focus indication
///
/// Provides a container with borders that change appearance based on focus state
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders},
};

/// A panel widget that can be focused
#[derive(Debug, Clone)]
pub struct Panel {
    /// Panel title
    title: String,
    /// Whether this panel is focused
    is_focused: bool,
    /// Whether to show borders
    borders: Borders,
}

impl Panel {
    /// Create a new panel
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_focused: false,
            borders: Borders::ALL,
        }
    }

    /// Set whether this panel is focused
    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    /// Set border style
    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    /// Get the block widget for this panel
    pub fn block(&self) -> Block<'_> {
        let block = Block::default()
            .borders(self.borders)
            .title(self.title.clone());

        if self.is_focused {
            // Focused panel: bright green border with bold title
            block
                .border_style(Style::default().fg(ToadTheme::TOAD_GREEN_BRIGHT))
                .title_style(
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                        .add_modifier(Modifier::BOLD),
                )
        } else {
            // Unfocused panel: dimmer border
            block
                .border_style(Style::default().fg(ToadTheme::BORDER))
                .title_style(Style::default().fg(ToadTheme::GRAY))
        }
    }

    /// Render the panel (just the border, content should be rendered separately)
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = self.block();
        frame.render_widget(block, area);
    }

    /// Get the inner area (area minus borders)
    pub fn inner(&self, area: Rect) -> Rect {
        let block = self.block();
        block.inner(area)
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new("Panel")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = Panel::new("Test Panel");
        assert_eq!(panel.title, "Test Panel");
        assert!(!panel.is_focused);
    }

    #[test]
    fn test_panel_focused() {
        let panel = Panel::new("Test").focused(true);
        assert!(panel.is_focused);
    }

    #[test]
    fn test_panel_borders() {
        let panel = Panel::new("Test").borders(Borders::LEFT | Borders::RIGHT);
        assert_eq!(panel.borders, Borders::LEFT | Borders::RIGHT);
    }

    // ========================================
    // MEDIUM TIER EDGE CASE TESTS
    // ========================================

    // Title Edge Cases
    #[test]
    fn test_panel_empty_title() {
        let panel = Panel::new("");
        assert_eq!(panel.title, "");
        assert!(!panel.is_focused);
    }

    #[test]
    fn test_panel_very_long_title() {
        let long_title = "Panel ".repeat(200);
        let panel = Panel::new(&long_title);
        assert_eq!(panel.title, long_title);
    }

    #[test]
    fn test_panel_unicode_title() {
        let panel = Panel::new("日本語パネル");
        assert_eq!(panel.title, "日本語パネル");

        let panel2 = Panel::new("Тест 中文 العربية");
        assert_eq!(panel2.title, "Тест 中文 العربية");
    }

    #[test]
    fn test_panel_emoji_title() {
        let panel = Panel::new("🐸 Frog Panel 🎉");
        assert_eq!(panel.title, "🐸 Frog Panel 🎉");

        let panel2 = Panel::new("👨‍💻 Code 🔧");
        assert_eq!(panel2.title, "👨‍💻 Code 🔧");
    }

    // Border Combinations
    #[test]
    fn test_panel_borders_all() {
        let panel = Panel::new("Test").borders(Borders::ALL);
        assert_eq!(panel.borders, Borders::ALL);
    }

    #[test]
    fn test_panel_borders_none() {
        let panel = Panel::new("Test").borders(Borders::NONE);
        assert_eq!(panel.borders, Borders::NONE);
    }

    #[test]
    fn test_panel_borders_top() {
        let panel = Panel::new("Test").borders(Borders::TOP);
        assert_eq!(panel.borders, Borders::TOP);
    }

    #[test]
    fn test_panel_borders_bottom() {
        let panel = Panel::new("Test").borders(Borders::BOTTOM);
        assert_eq!(panel.borders, Borders::BOTTOM);
    }

    #[test]
    fn test_panel_borders_left() {
        let panel = Panel::new("Test").borders(Borders::LEFT);
        assert_eq!(panel.borders, Borders::LEFT);
    }

    #[test]
    fn test_panel_borders_right() {
        let panel = Panel::new("Test").borders(Borders::RIGHT);
        assert_eq!(panel.borders, Borders::RIGHT);
    }

    #[test]
    fn test_panel_borders_horizontal() {
        let panel = Panel::new("Test").borders(Borders::LEFT | Borders::RIGHT);
        assert_eq!(panel.borders, Borders::LEFT | Borders::RIGHT);
    }

    #[test]
    fn test_panel_borders_vertical() {
        let panel = Panel::new("Test").borders(Borders::TOP | Borders::BOTTOM);
        assert_eq!(panel.borders, Borders::TOP | Borders::BOTTOM);
    }

    #[test]
    fn test_panel_borders_three_sides() {
        let panel = Panel::new("Test").borders(Borders::TOP | Borders::LEFT | Borders::RIGHT);
        assert_eq!(panel.borders, Borders::TOP | Borders::LEFT | Borders::RIGHT);
    }

    // Focus State
    #[test]
    fn test_panel_focused_true() {
        let panel = Panel::new("Test").focused(true);
        assert!(panel.is_focused);
    }

    #[test]
    fn test_panel_focused_false() {
        let panel = Panel::new("Test").focused(false);
        assert!(!panel.is_focused);
    }

    #[test]
    fn test_panel_focus_toggle() {
        let panel1 = Panel::new("Test").focused(true);
        assert!(panel1.is_focused);

        let panel2 = panel1.focused(false);
        assert!(!panel2.is_focused);

        let panel3 = panel2.focused(true);
        assert!(panel3.is_focused);
    }

    // Builder Pattern
    #[test]
    fn test_panel_builder_chaining() {
        let panel = Panel::new("Complete")
            .focused(true)
            .borders(Borders::ALL);

        assert_eq!(panel.title, "Complete");
        assert!(panel.is_focused);
        assert_eq!(panel.borders, Borders::ALL);
    }

    #[test]
    fn test_panel_builder_all_options() {
        let panel = Panel::new("🎨 Design Panel 日本語")
            .focused(false)
            .borders(Borders::LEFT | Borders::RIGHT);

        assert_eq!(panel.title, "🎨 Design Panel 日本語");
        assert!(!panel.is_focused);
        assert_eq!(panel.borders, Borders::LEFT | Borders::RIGHT);
    }

    // Inner Area Calculation
    #[test]
    fn test_panel_inner_all_borders() {
        let panel = Panel::new("Test").borders(Borders::ALL);
        let area = Rect::new(0, 0, 100, 50);
        let inner = panel.inner(area);

        // With all borders, inner area should be smaller
        assert!(inner.width < area.width);
        assert!(inner.height < area.height);
    }

    #[test]
    fn test_panel_inner_no_borders() {
        let panel = Panel::new("Test").borders(Borders::NONE);
        let area = Rect::new(0, 0, 100, 50);
        let inner = panel.inner(area);

        // With no borders, width should equal outer area
        // Height is slightly less due to title rendering
        assert_eq!(inner.width, area.width);
        assert!(inner.height <= area.height);
        assert!(inner.height >= area.height - 1); // Title takes max 1 line
    }

    #[test]
    fn test_panel_inner_small_area() {
        let panel = Panel::new("Test").borders(Borders::ALL);
        let area = Rect::new(0, 0, 5, 3);
        let inner = panel.inner(area);

        // Should handle small areas without panic
        assert!(inner.width <= area.width);
        assert!(inner.height <= area.height);
    }

    #[test]
    fn test_panel_inner_zero_area() {
        let panel = Panel::new("Test").borders(Borders::ALL);
        let area = Rect::new(0, 0, 0, 0);
        let inner = panel.inner(area);

        assert_eq!(inner.width, 0);
        assert_eq!(inner.height, 0);
    }

    #[test]
    fn test_panel_inner_very_large_area() {
        let panel = Panel::new("Test").borders(Borders::ALL);
        let area = Rect::new(0, 0, 10000, 5000);
        let inner = panel.inner(area);

        assert!(inner.width > 0);
        assert!(inner.height > 0);
        assert!(inner.width < area.width);
        assert!(inner.height < area.height);
    }

    // Block Generation
    #[test]
    fn test_panel_block_unfocused() {
        let panel = Panel::new("Test").focused(false);
        let _block = panel.block();

        // Block should be created without panic
        // (Internal styling details can't be tested without exposing internals)
    }

    #[test]
    fn test_panel_block_focused() {
        let panel = Panel::new("Test").focused(true);
        let _block = panel.block();

        // Block should be created without panic
        // (Internal styling details can't be tested without exposing internals)
    }

    // Trait Tests
    #[test]
    fn test_panel_clone() {
        let panel1 = Panel::new("Original").focused(true);
        let panel2 = panel1.clone();

        assert_eq!(panel1.title, panel2.title);
        assert_eq!(panel1.is_focused, panel2.is_focused);
        assert_eq!(panel1.borders, panel2.borders);
    }

    #[test]
    fn test_panel_debug() {
        let panel = Panel::new("Debug Test");
        let debug_str = format!("{:?}", panel);

        assert!(debug_str.contains("Panel"));
        assert!(debug_str.contains("Debug Test"));
    }

    #[test]
    fn test_panel_default() {
        let panel = Panel::default();
        assert_eq!(panel.title, "Panel");
        assert!(!panel.is_focused);
        assert_eq!(panel.borders, Borders::ALL);
    }

    // State Transitions
    #[test]
    fn test_panel_multiple_focus_changes() {
        let mut panel = Panel::new("Test");

        for i in 0..100 {
            panel = panel.focused(i % 2 == 0);
            assert_eq!(panel.is_focused, i % 2 == 0);
        }
    }

    #[test]
    fn test_panel_border_changes() {
        let panel = Panel::new("Test")
            .borders(Borders::ALL)
            .borders(Borders::NONE)
            .borders(Borders::LEFT)
            .borders(Borders::RIGHT);

        assert_eq!(panel.borders, Borders::RIGHT);
    }

    // Complex Scenarios
    #[test]
    fn test_panel_with_everything() {
        let panel = Panel::new("🎨 Complex Panel 日本語 🔧")
            .focused(true)
            .borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT);

        assert_eq!(panel.title, "🎨 Complex Panel 日本語 🔧");
        assert!(panel.is_focused);
        assert_eq!(panel.borders, Borders::TOP | Borders::BOTTOM | Borders::LEFT);

        let area = Rect::new(10, 20, 200, 100);
        let inner = panel.inner(area);
        assert!(inner.width < area.width);
        assert!(inner.height < area.height);
    }

    #[test]
    fn test_panel_title_types() {
        // Test String
        let panel1 = Panel::new(String::from("String Type"));
        assert_eq!(panel1.title, "String Type");

        // Test &str
        let panel2 = Panel::new("str Type");
        assert_eq!(panel2.title, "str Type");

        // Test owned string
        let owned = "Owned".to_string();
        let panel3 = Panel::new(owned);
        assert_eq!(panel3.title, "Owned");
    }

    #[test]
    fn test_panel_immutability() {
        let panel1 = Panel::new("Original");
        let panel2 = panel1.focused(true);
        let panel3 = panel2.borders(Borders::NONE);

        // Each transformation creates a new panel
        assert_eq!(panel3.title, "Original");
        assert!(panel3.is_focused);
        assert_eq!(panel3.borders, Borders::NONE);
    }
}
