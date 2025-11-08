/// Panel widget with focus indication
///
/// Provides a container with borders that change appearance based on focus state

use crate::theme::ToadTheme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders},
    Frame,
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
}
