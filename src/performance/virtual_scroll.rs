/// Virtual scrolling for handling massive lists (1M+ items)
///
/// Only renders visible elements, dramatically improving performance for large datasets
///
/// # Examples
///
/// ```
/// use toad::virtual_scroll::VirtualScrollState;
///
/// let mut state = VirtualScrollState::new(1_000_000);
/// state.set_viewport_height(20);
///
/// let visible = state.visible_range();
/// assert!(visible.end - visible.start <= 20);
/// ```
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Virtual scroll state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualScrollState {
    /// Total number of items
    total_items: usize,
    /// Current scroll offset (index of first visible item)
    offset: usize,
    /// Height of viewport (number of visible items)
    viewport_height: usize,
    /// Optional fixed item height
    item_height: Option<usize>,
}

impl VirtualScrollState {
    /// Create a new virtual scroll state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let state = VirtualScrollState::new(1_000_000);
    /// assert_eq!(state.total_items(), 1_000_000);
    /// assert_eq!(state.offset(), 0);
    /// ```
    pub fn new(total_items: usize) -> Self {
        Self {
            total_items,
            offset: 0,
            viewport_height: 0,
            item_height: Some(1),
        }
    }

    /// Set viewport height
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(100);
    /// state.set_viewport_height(20);
    /// assert_eq!(state.viewport_height(), 20);
    /// ```
    pub fn set_viewport_height(&mut self, height: usize) {
        self.viewport_height = height;
    }

    /// Set item height
    pub fn set_item_height(&mut self, height: usize) {
        self.item_height = Some(height);
    }

    /// Get total items
    pub fn total_items(&self) -> usize {
        self.total_items
    }

    /// Set total items
    pub fn set_total_items(&mut self, total: usize) {
        self.total_items = total;
        self.clamp_offset();
    }

    /// Get current offset
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get viewport height
    pub fn viewport_height(&self) -> usize {
        self.viewport_height
    }

    /// Get visible range
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(1000);
    /// state.set_viewport_height(20);
    ///
    /// let range = state.visible_range();
    /// assert_eq!(range.start, 0);
    /// assert_eq!(range.end, 20);
    /// ```
    pub fn visible_range(&self) -> Range<usize> {
        let start = self.offset;
        let end = (self.offset + self.viewport_height).min(self.total_items);
        start..end
    }

    /// Get number of visible items
    pub fn visible_count(&self) -> usize {
        let range = self.visible_range();
        range.end - range.start
    }

    /// Scroll down by n items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(100);
    /// state.set_viewport_height(20);
    ///
    /// state.scroll_down(5);
    /// assert_eq!(state.offset(), 5);
    /// ```
    pub fn scroll_down(&mut self, n: usize) {
        self.offset = (self.offset + n).min(self.max_offset());
    }

    /// Scroll up by n items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(100);
    /// state.set_viewport_height(20);
    /// state.scroll_down(10);
    ///
    /// state.scroll_up(5);
    /// assert_eq!(state.offset(), 5);
    /// ```
    pub fn scroll_up(&mut self, n: usize) {
        self.offset = self.offset.saturating_sub(n);
    }

    /// Scroll to specific offset
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(100);
    /// state.set_viewport_height(20);
    ///
    /// state.scroll_to(50);
    /// assert_eq!(state.offset(), 50);
    /// ```
    pub fn scroll_to(&mut self, offset: usize) {
        self.offset = offset.min(self.max_offset());
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.offset = 0;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.offset = self.max_offset();
    }

    /// Scroll to make item visible
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(100);
    /// state.set_viewport_height(20);
    ///
    /// state.scroll_to_item(50);
    /// let range = state.visible_range();
    /// assert!(range.contains(&50));
    /// ```
    pub fn scroll_to_item(&mut self, item_index: usize) {
        if item_index < self.total_items {
            if item_index < self.offset {
                // Scroll up to show item at top
                self.offset = item_index;
            } else if item_index >= self.offset + self.viewport_height {
                // Scroll down to show item at bottom
                self.offset = (item_index + 1).saturating_sub(self.viewport_height);
            }
            self.clamp_offset();
        }
    }

    /// Get maximum valid offset
    fn max_offset(&self) -> usize {
        self.total_items.saturating_sub(self.viewport_height)
    }

    /// Clamp offset to valid range
    fn clamp_offset(&mut self) {
        self.offset = self.offset.min(self.max_offset());
    }

    /// Check if item is visible
    pub fn is_visible(&self, item_index: usize) -> bool {
        self.visible_range().contains(&item_index)
    }

    /// Get scroll percentage (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::virtual_scroll::VirtualScrollState;
    ///
    /// let mut state = VirtualScrollState::new(100);
    /// state.set_viewport_height(20);
    ///
    /// assert_eq!(state.scroll_percentage(), 0.0);
    ///
    /// state.scroll_to_bottom();
    /// assert_eq!(state.scroll_percentage(), 1.0);
    /// ```
    pub fn scroll_percentage(&self) -> f64 {
        if self.total_items <= self.viewport_height {
            0.0
        } else {
            let max_offset = self.max_offset() as f64;
            if max_offset == 0.0 {
                0.0
            } else {
                self.offset as f64 / max_offset
            }
        }
    }

    /// Check if at top
    pub fn is_at_top(&self) -> bool {
        self.offset == 0
    }

    /// Check if at bottom
    pub fn is_at_bottom(&self) -> bool {
        self.offset >= self.max_offset()
    }

    /// Page down (scroll by viewport height)
    pub fn page_down(&mut self) {
        self.scroll_down(self.viewport_height);
    }

    /// Page up (scroll by viewport height)
    pub fn page_up(&mut self) {
        self.scroll_up(self.viewport_height);
    }
}

impl Default for VirtualScrollState {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_scroll_creation() {
        let state = VirtualScrollState::new(1000);
        assert_eq!(state.total_items(), 1000);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.viewport_height(), 0);
    }

    #[test]
    fn test_set_viewport_height() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);
        assert_eq!(state.viewport_height(), 20);
    }

    #[test]
    fn test_visible_range() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        let range = state.visible_range();
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 20);

        state.scroll_down(10);
        let range = state.visible_range();
        assert_eq!(range.start, 10);
        assert_eq!(range.end, 30);
    }

    #[test]
    fn test_visible_count() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);
        assert_eq!(state.visible_count(), 20);

        // At bottom, might show fewer items
        state.scroll_to_bottom();
        assert_eq!(state.visible_count(), 20);
    }

    #[test]
    fn test_scroll_down() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        state.scroll_down(10);
        assert_eq!(state.offset(), 10);

        state.scroll_down(5);
        assert_eq!(state.offset(), 15);
    }

    #[test]
    fn test_scroll_up() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);
        state.scroll_down(20);

        state.scroll_up(5);
        assert_eq!(state.offset(), 15);

        state.scroll_up(20); // Should clamp at 0
        assert_eq!(state.offset(), 0);
    }

    #[test]
    fn test_scroll_to() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        state.scroll_to(50);
        assert_eq!(state.offset(), 50);

        state.scroll_to(200); // Should clamp to max
        assert_eq!(state.offset(), 80); // 100 - 20
    }

    #[test]
    fn test_scroll_to_top() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);
        state.scroll_down(50);

        state.scroll_to_top();
        assert_eq!(state.offset(), 0);
        assert!(state.is_at_top());
    }

    #[test]
    fn test_scroll_to_bottom() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        state.scroll_to_bottom();
        assert_eq!(state.offset(), 80); // 100 - 20
        assert!(state.is_at_bottom());
    }

    #[test]
    fn test_scroll_to_item() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        // Scroll to item below viewport
        state.scroll_to_item(50);
        assert!(state.visible_range().contains(&50));

        // Scroll to item above viewport
        state.scroll_to_item(5);
        assert!(state.visible_range().contains(&5));
    }

    #[test]
    fn test_is_visible() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        assert!(state.is_visible(0));
        assert!(state.is_visible(19));
        assert!(!state.is_visible(20));
        assert!(!state.is_visible(50));
    }

    #[test]
    fn test_scroll_percentage() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        assert_eq!(state.scroll_percentage(), 0.0);

        state.scroll_to(40);
        assert_eq!(state.scroll_percentage(), 0.5); // 40 / 80

        state.scroll_to_bottom();
        assert_eq!(state.scroll_percentage(), 1.0);
    }

    #[test]
    fn test_page_down_up() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);

        state.page_down();
        assert_eq!(state.offset(), 20);

        state.page_down();
        assert_eq!(state.offset(), 40);

        state.page_up();
        assert_eq!(state.offset(), 20);
    }

    #[test]
    fn test_large_dataset() {
        let mut state = VirtualScrollState::new(1_000_000);
        state.set_viewport_height(50);

        // Should handle million items efficiently
        state.scroll_to(500_000);
        assert_eq!(state.offset(), 500_000);

        let range = state.visible_range();
        assert_eq!(range.end - range.start, 50);
    }

    #[test]
    fn test_set_total_items() {
        let mut state = VirtualScrollState::new(100);
        state.set_viewport_height(20);
        state.scroll_to_bottom();

        // Reduce total items
        state.set_total_items(50);
        assert!(state.offset() <= state.max_offset());

        // Increase total items
        state.set_total_items(200);
        assert_eq!(state.total_items(), 200);
    }
}
