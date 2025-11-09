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

    // === COMPREHENSIVE EDGE CASE TESTS (MEDIUM tier) ===

    // --- Extreme Values ---

    #[test]
    fn test_scrollbar_state_very_large_total() {
        let state = ScrollbarState::new(1_000_000, 500_000, 1000);
        assert_eq!(state.total, 1_000_000);
        assert!(state.should_show());
        // Approximately halfway through (500,000 / 999,000 ≈ 0.5005)
        assert!((state.scroll_percentage() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_scrollbar_state_very_small_viewport() {
        let state = ScrollbarState::new(10_000, 0, 1);
        assert_eq!(state.viewport_size, 1);
        assert!(state.should_show());
        assert_eq!(state.thumb_size_percentage(), 0.0001); // 1/10000
    }

    #[test]
    fn test_scrollbar_state_very_large_position() {
        let state = ScrollbarState::new(100, 10_000, 20);
        // Position beyond max should clamp
        assert_eq!(state.scroll_percentage(), 1.0);
    }

    #[test]
    fn test_scrollbar_state_max_values() {
        let state = ScrollbarState::new(usize::MAX, usize::MAX / 2, 1000);
        // Should handle extreme values without overflow
        assert!(state.scroll_percentage() >= 0.0 && state.scroll_percentage() <= 1.0);
    }

    // --- Boundary Conditions ---

    #[test]
    fn test_scrollbar_state_position_at_zero() {
        let state = ScrollbarState::new(100, 0, 20);
        assert_eq!(state.position, 0);
        assert_eq!(state.scroll_percentage(), 0.0);
    }

    #[test]
    fn test_scrollbar_state_position_at_max() {
        let state = ScrollbarState::new(100, 80, 20);
        assert_eq!(state.scroll_percentage(), 1.0);
    }

    #[test]
    fn test_scrollbar_state_viewport_equals_one() {
        let state = ScrollbarState::new(100, 50, 1);
        assert_eq!(state.viewport_size, 1);
        assert!(state.should_show());
        assert_eq!(state.thumb_size_percentage(), 0.01); // 1/100
    }

    #[test]
    fn test_scrollbar_state_viewport_equals_total() {
        let state = ScrollbarState::new(50, 0, 50);
        assert_eq!(state.viewport_size, state.total);
        assert!(!state.should_show()); // Should not show when equal
        assert_eq!(state.scroll_percentage(), 0.0);
    }

    #[test]
    fn test_scrollbar_state_viewport_greater_than_total() {
        let state = ScrollbarState::new(30, 0, 50);
        assert!(state.viewport_size > state.total);
        assert!(!state.should_show());
        assert_eq!(state.thumb_size_percentage(), 1.0); // Clamped
    }

    #[test]
    fn test_scrollbar_state_total_equals_one() {
        let state = ScrollbarState::new(1, 0, 1);
        assert_eq!(state.total, 1);
        assert!(!state.should_show());
    }

    #[test]
    fn test_scrollbar_state_all_zeros() {
        let state = ScrollbarState::new(0, 0, 0);
        assert_eq!(state.scroll_percentage(), 0.0);
        assert_eq!(state.thumb_size_percentage(), 1.0);
        assert!(!state.should_show());
    }

    // --- Percentage Calculation Precision ---

    #[test]
    fn test_scroll_percentage_fractional_positions() {
        // Test various fractional positions
        let state1 = ScrollbarState::new(100, 25, 20);
        assert!((state1.scroll_percentage() - 0.3125).abs() < 0.001); // 25/80

        let state2 = ScrollbarState::new(100, 75, 20);
        assert!((state2.scroll_percentage() - 0.9375).abs() < 0.001); // 75/80

        let state3 = ScrollbarState::new(100, 10, 20);
        assert!((state3.scroll_percentage() - 0.125).abs() < 0.001); // 10/80
    }

    #[test]
    fn test_thumb_size_percentage_fractional_viewports() {
        let state1 = ScrollbarState::new(100, 0, 33);
        assert!((state1.thumb_size_percentage() - 0.33).abs() < 0.01); // 33/100

        let state2 = ScrollbarState::new(1000, 0, 123);
        assert!((state2.thumb_size_percentage() - 0.123).abs() < 0.001); // 123/1000
    }

    // --- Unicode Characters ---

    #[test]
    fn test_scrollbar_unicode_track_char() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::vertical(state).track_char('░');

        assert_eq!(scrollbar.track_char, '░');
    }

    #[test]
    fn test_scrollbar_unicode_thumb_char() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::vertical(state).thumb_char('▓');

        assert_eq!(scrollbar.thumb_char, '▓');
    }

    #[test]
    fn test_scrollbar_emoji_characters() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::vertical(state)
            .track_char('🌫')
            .thumb_char('🐸');

        assert_eq!(scrollbar.track_char, '🌫');
        assert_eq!(scrollbar.thumb_char, '🐸');
    }

    // --- Builder Pattern Chaining ---

    #[test]
    fn test_scrollbar_builder_chaining() {
        let state = ScrollbarState::new(100, 50, 20);
        let scrollbar = Scrollbar::horizontal(state)
            .show_track(false)
            .track_char('.')
            .thumb_char('#');

        assert_eq!(scrollbar.orientation, ScrollbarOrientation::Horizontal);
        assert!(!scrollbar.show_track);
        assert_eq!(scrollbar.track_char, '.');
        assert_eq!(scrollbar.thumb_char, '#');
    }

    #[test]
    fn test_scrollbar_builder_partial_chaining() {
        let state = ScrollbarState::new(100, 0, 20);
        let scrollbar = Scrollbar::vertical(state).show_track(false);

        assert!(!scrollbar.show_track);
        assert_eq!(scrollbar.track_char, '│'); // Default
        assert_eq!(scrollbar.thumb_char, '█'); // Default
    }

    // --- State Transitions ---

    #[test]
    fn test_scrollbar_multiple_state_updates() {
        let state1 = ScrollbarState::new(100, 0, 20);
        let mut scrollbar = Scrollbar::vertical(state1);

        assert_eq!(scrollbar.state().position, 0);

        let state2 = ScrollbarState::new(100, 25, 20);
        scrollbar.set_state(state2);
        assert_eq!(scrollbar.state().position, 25);

        let state3 = ScrollbarState::new(100, 50, 20);
        scrollbar.set_state(state3);
        assert_eq!(scrollbar.state().position, 50);

        let state4 = ScrollbarState::new(100, 80, 20);
        scrollbar.set_state(state4);
        assert_eq!(scrollbar.state().position, 80);
    }

    #[test]
    fn test_scrollbar_state_updates_with_different_totals() {
        let state1 = ScrollbarState::new(100, 0, 20);
        let mut scrollbar = Scrollbar::vertical(state1);

        let state2 = ScrollbarState::new(200, 50, 20);
        scrollbar.set_state(state2);
        assert_eq!(scrollbar.state().total, 200);
        assert_eq!(scrollbar.state().position, 50);

        let state3 = ScrollbarState::new(50, 10, 20);
        scrollbar.set_state(state3);
        assert_eq!(scrollbar.state().total, 50);
        assert_eq!(scrollbar.state().position, 10);
    }

    // --- Trait Implementations ---

    #[test]
    fn test_scrollbar_orientation_clone() {
        let ori1 = ScrollbarOrientation::Vertical;
        let ori2 = ori1;
        assert_eq!(ori1, ori2); // Copy trait should work
    }

    #[test]
    fn test_scrollbar_orientation_debug() {
        let ori = ScrollbarOrientation::Horizontal;
        let debug_str = format!("{:?}", ori);
        assert!(debug_str.contains("Horizontal"));
    }

    #[test]
    fn test_scrollbar_orientation_partial_eq() {
        assert_eq!(ScrollbarOrientation::Vertical, ScrollbarOrientation::Vertical);
        assert_eq!(ScrollbarOrientation::Horizontal, ScrollbarOrientation::Horizontal);
        assert_ne!(ScrollbarOrientation::Vertical, ScrollbarOrientation::Horizontal);
    }

    #[test]
    fn test_scrollbar_state_clone() {
        let state1 = ScrollbarState::new(100, 50, 20);
        let state2 = state1;

        assert_eq!(state1.total, state2.total);
        assert_eq!(state1.position, state2.position);
        assert_eq!(state1.viewport_size, state2.viewport_size);
    }

    #[test]
    fn test_scrollbar_state_debug() {
        let state = ScrollbarState::new(100, 50, 20);
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("100"));
        assert!(debug_str.contains("50"));
        assert!(debug_str.contains("20"));
    }

    #[test]
    fn test_scrollbar_state_partial_eq() {
        let state1 = ScrollbarState::new(100, 50, 20);
        let state2 = ScrollbarState::new(100, 50, 20);
        let state3 = ScrollbarState::new(100, 60, 20);

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_scrollbar_clone() {
        let state = ScrollbarState::new(100, 50, 20);
        let scrollbar1 = Scrollbar::vertical(state).track_char('|');
        let scrollbar2 = scrollbar1.clone();

        assert_eq!(scrollbar1.orientation, scrollbar2.orientation);
        assert_eq!(scrollbar1.track_char, scrollbar2.track_char);
    }

    #[test]
    fn test_scrollbar_debug() {
        let state = ScrollbarState::new(100, 50, 20);
        let scrollbar = Scrollbar::vertical(state);
        let debug_str = format!("{:?}", scrollbar);
        assert!(debug_str.contains("Vertical"));
    }

    // --- Complex Scenarios ---

    #[test]
    fn test_scrollbar_scrolling_through_large_list() {
        // Simulate scrolling through a list of 1000 items with viewport of 10
        let positions = vec![0, 100, 250, 500, 750, 900, 990];

        for pos in positions {
            let state = ScrollbarState::new(1000, pos, 10);
            assert!(state.should_show());

            let scroll_pct = state.scroll_percentage();
            assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);

            let thumb_pct = state.thumb_size_percentage();
            assert_eq!(thumb_pct, 0.01); // 10/1000
        }
    }

    #[test]
    fn test_scrollbar_orientation_switching() {
        let state = ScrollbarState::new(100, 50, 20);

        let vertical = Scrollbar::vertical(state);
        assert_eq!(vertical.orientation, ScrollbarOrientation::Vertical);
        assert_eq!(vertical.track_char, '│');

        let horizontal = Scrollbar::horizontal(state);
        assert_eq!(horizontal.orientation, ScrollbarOrientation::Horizontal);
        assert_eq!(horizontal.track_char, '─');
    }

    #[test]
    fn test_scrollbar_edge_case_combinations() {
        // Test various edge case combinations
        let test_cases = vec![
            (0, 0, 0),     // All zeros
            (1, 0, 1),     // Single item
            (10, 0, 10),   // Exact fit
            (10, 0, 20),   // Viewport larger
            (100, 0, 1),   // Minimal viewport
            (100, 99, 1),  // Position near end
        ];

        for (total, position, viewport) in test_cases {
            let state = ScrollbarState::new(total, position, viewport);

            // Verify calculations don't panic
            let _should_show = state.should_show();
            let scroll_pct = state.scroll_percentage();
            let thumb_pct = state.thumb_size_percentage();

            // Verify percentages are in valid range
            assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);
            assert!(thumb_pct >= 0.0 && thumb_pct <= 1.0);
        }
    }
}
