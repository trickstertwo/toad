/// Scrollbar widget for visual scroll indicators
///
/// Displays a vertical or horizontal scrollbar with track and thumb
use crate::ui::theme::ToadTheme;
use ratatui::{Frame, layout::Rect, style::Style, widgets::Block};

/// Scrollbar orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarOrientation {
    /// Vertical scrollbar
    Vertical,
    /// Horizontal scrollbar
    Horizontal,
}

/// Scrollbar state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollbarState {
    /// Total number of items
    pub total: usize,
    /// Current position (0-based)
    pub position: usize,
    /// Viewport size (how many items visible)
    pub viewport_size: usize,
}

impl ScrollbarState {
    /// Create a new scrollbar state
    pub fn new(total: usize, position: usize, viewport_size: usize) -> Self {
        Self {
            total,
            position,
            viewport_size,
        }
    }

    /// Check if scrollbar should be visible
    pub fn should_show(&self) -> bool {
        self.total > self.viewport_size
    }

    /// Get scroll percentage (0.0 to 1.0)
    pub fn scroll_percentage(&self) -> f64 {
        if self.total == 0 || self.viewport_size >= self.total {
            return 0.0;
        }

        let max_scroll = self.total.saturating_sub(self.viewport_size);
        if max_scroll == 0 {
            0.0
        } else {
            (self.position.min(max_scroll) as f64) / (max_scroll as f64)
        }
    }

    /// Get thumb size as percentage of track (0.0 to 1.0)
    pub fn thumb_size_percentage(&self) -> f64 {
        if self.total == 0 {
            return 1.0;
        }

        (self.viewport_size as f64 / self.total as f64).min(1.0)
    }
}

/// Scrollbar widget
#[derive(Debug, Clone)]
pub struct Scrollbar {
    /// Orientation
    orientation: ScrollbarOrientation,
    /// Scrollbar state
    state: ScrollbarState,
    /// Show track
    show_track: bool,
    /// Track character
    track_char: char,
    /// Thumb character
    thumb_char: char,
}

impl Scrollbar {
    /// Create a new vertical scrollbar
    pub fn vertical(state: ScrollbarState) -> Self {
        Self {
            orientation: ScrollbarOrientation::Vertical,
            state,
            show_track: true,
            track_char: '│',
            thumb_char: '█',
        }
    }

    /// Create a new horizontal scrollbar
    pub fn horizontal(state: ScrollbarState) -> Self {
        Self {
            orientation: ScrollbarOrientation::Horizontal,
            state,
            show_track: true,
            track_char: '─',
            thumb_char: '█',
        }
    }

    /// Set whether to show track
    pub fn show_track(mut self, show: bool) -> Self {
        self.show_track = show;
        self
    }

    /// Set track character
    pub fn track_char(mut self, ch: char) -> Self {
        self.track_char = ch;
        self
    }

    /// Set thumb character
    pub fn thumb_char(mut self, ch: char) -> Self {
        self.thumb_char = ch;
        self
    }

    /// Update state
    pub fn set_state(&mut self, state: ScrollbarState) {
        self.state = state;
    }

    /// Get state
    pub fn state(&self) -> &ScrollbarState {
        &self.state
    }

    /// Render the scrollbar
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Don't render if not needed
        if !self.state.should_show() {
            return;
        }

        match self.orientation {
            ScrollbarOrientation::Vertical => self.render_vertical(frame, area),
            ScrollbarOrientation::Horizontal => self.render_horizontal(frame, area),
        }
    }

    /// Render vertical scrollbar
    fn render_vertical(&self, frame: &mut Frame, area: Rect) {
        let height = area.height as usize;
        if height == 0 {
            return;
        }

        // Calculate thumb position and size
        let thumb_size = (self.state.thumb_size_percentage() * height as f64).max(1.0) as usize;
        let thumb_size = thumb_size.min(height);

        let scroll_pct = self.state.scroll_percentage();
        let max_thumb_pos = height.saturating_sub(thumb_size);
        let thumb_pos = (scroll_pct * max_thumb_pos as f64) as usize;

        // Render character by character
        for y in 0..height {
            let _ch = if y >= thumb_pos && y < thumb_pos + thumb_size {
                self.thumb_char
            } else if self.show_track {
                self.track_char
            } else {
                ' '
            };

            let style = if y >= thumb_pos && y < thumb_pos + thumb_size {
                Style::default().fg(ToadTheme::TOAD_GREEN)
            } else {
                Style::default().fg(ToadTheme::DARK_GRAY)
            };

            // Render using a Block widget (simplified approach)
            let block = Block::default().style(style);
            let rect = Rect::new(area.x, area.y + y as u16, area.width, 1);

            // Draw the character (using ratatui's low-level buffer access would be better,
            // but for simplicity we'll use a styled block)
            frame.render_widget(block, rect);
        }
    }

    /// Render horizontal scrollbar
    fn render_horizontal(&self, frame: &mut Frame, area: Rect) {
        let width = area.width as usize;
        if width == 0 {
            return;
        }

        // Calculate thumb position and size
        let thumb_size = (self.state.thumb_size_percentage() * width as f64).max(1.0) as usize;
        let thumb_size = thumb_size.min(width);

        let scroll_pct = self.state.scroll_percentage();
        let max_thumb_pos = width.saturating_sub(thumb_size);
        let thumb_pos = (scroll_pct * max_thumb_pos as f64) as usize;

        // Render character by character
        for x in 0..width {
            let _ch = if x >= thumb_pos && x < thumb_pos + thumb_size {
                self.thumb_char
            } else if self.show_track {
                self.track_char
            } else {
                ' '
            };

            let style = if x >= thumb_pos && x < thumb_pos + thumb_size {
                Style::default().fg(ToadTheme::TOAD_GREEN)
            } else {
                Style::default().fg(ToadTheme::DARK_GRAY)
            };

            let block = Block::default().style(style);
            let rect = Rect::new(area.x + x as u16, area.y, 1, area.height);

            frame.render_widget(block, rect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrollbar_state_creation() {
        let state = ScrollbarState::new(100, 10, 20);
        assert_eq!(state.total, 100);
        assert_eq!(state.position, 10);
        assert_eq!(state.viewport_size, 20);
    }

    #[test]
    fn test_scrollbar_state_should_show() {
        let state1 = ScrollbarState::new(100, 0, 20);
        assert!(state1.should_show()); // More items than viewport

        let state2 = ScrollbarState::new(20, 0, 20);
        assert!(!state2.should_show()); // Exact fit

        let state3 = ScrollbarState::new(10, 0, 20);
        assert!(!state3.should_show()); // Fewer items than viewport
    }

    #[test]
    fn test_scrollbar_state_scroll_percentage() {
        let state1 = ScrollbarState::new(100, 0, 20);
        assert_eq!(state1.scroll_percentage(), 0.0); // At top

        let state2 = ScrollbarState::new(100, 80, 20);
        assert_eq!(state2.scroll_percentage(), 1.0); // At bottom

        let state3 = ScrollbarState::new(100, 40, 20);
        assert_eq!(state3.scroll_percentage(), 0.5); // Middle
    }

    #[test]
    fn test_scrollbar_state_thumb_size_percentage() {
        let state1 = ScrollbarState::new(100, 0, 20);
        assert_eq!(state1.thumb_size_percentage(), 0.2); // 20/100

        let state2 = ScrollbarState::new(100, 0, 50);
        assert_eq!(state2.thumb_size_percentage(), 0.5); // 50/100

        let state3 = ScrollbarState::new(50, 0, 50);
        assert_eq!(state3.thumb_size_percentage(), 1.0); // 50/50 = 1.0 (max)
    }

    #[test]
    fn test_scrollbar_vertical_creation() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::vertical(state);

        assert_eq!(scrollbar.orientation, ScrollbarOrientation::Vertical);
        assert_eq!(scrollbar.track_char, '│');
        assert_eq!(scrollbar.thumb_char, '█');
    }

    #[test]
    fn test_scrollbar_horizontal_creation() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::horizontal(state);

        assert_eq!(scrollbar.orientation, ScrollbarOrientation::Horizontal);
        assert_eq!(scrollbar.track_char, '─');
        assert_eq!(scrollbar.thumb_char, '█');
    }

    #[test]
    fn test_scrollbar_customization() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::vertical(state)
            .show_track(false)
            .track_char('|')
            .thumb_char('■');

        assert!(!scrollbar.show_track);
        assert_eq!(scrollbar.track_char, '|');
        assert_eq!(scrollbar.thumb_char, '■');
    }

    #[test]
    fn test_scrollbar_set_state() {
        let state1 = ScrollbarState::new(100, 0, 20);
        let mut scrollbar = Scrollbar::vertical(state1);

        assert_eq!(scrollbar.state().position, 0);

        let state2 = ScrollbarState::new(100, 50, 20);
        scrollbar.set_state(state2);

        assert_eq!(scrollbar.state().position, 50);
    }

    #[test]
    fn test_scroll_percentage_edge_cases() {
        // Empty list
        let state = ScrollbarState::new(0, 0, 0);
        assert_eq!(state.scroll_percentage(), 0.0);

        // Single item
        let state = ScrollbarState::new(1, 0, 1);
        assert_eq!(state.scroll_percentage(), 0.0);

        // Position beyond max
        let state = ScrollbarState::new(100, 200, 20);
        assert_eq!(state.scroll_percentage(), 1.0); // Should clamp to 1.0
    }

    #[test]
    fn test_thumb_size_percentage_edge_cases() {
        // Empty list
        let state = ScrollbarState::new(0, 0, 0);
        assert_eq!(state.thumb_size_percentage(), 1.0);

        // Viewport larger than total
        let state = ScrollbarState::new(10, 0, 20);
        assert_eq!(state.thumb_size_percentage(), 1.0); // Clamped to max
    }
}
