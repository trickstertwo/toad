/// Lazy rendering - only render visible elements
///
/// Optimization to skip rendering elements outside the viewport
///
/// # Examples
///
/// ```
/// use toad::lazy_render::LazyRenderState;
///
/// let state = LazyRenderState::new(100);
/// assert!(state.should_render(10));
/// assert!(!state.should_render(150));
/// ```
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Lazy render state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazyRenderState {
    /// Visible area top (y coordinate)
    viewport_top: usize,
    /// Visible area bottom (y coordinate)
    viewport_bottom: usize,
    /// Render buffer (render slightly outside viewport)
    buffer: usize,
}

impl LazyRenderState {
    /// Create a new lazy render state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::lazy_render::LazyRenderState;
    ///
    /// let state = LazyRenderState::new(100);
    /// assert!(state.should_render(50));
    /// ```
    pub fn new(viewport_height: usize) -> Self {
        Self {
            viewport_top: 0,
            viewport_bottom: viewport_height,
            buffer: 5, // Render 5 lines above/below viewport
        }
    }

    /// Set viewport range
    pub fn set_viewport(&mut self, top: usize, bottom: usize) {
        self.viewport_top = top;
        self.viewport_bottom = bottom;
    }

    /// Set buffer size
    pub fn set_buffer(&mut self, buffer: usize) {
        self.buffer = buffer;
    }

    /// Get effective render range (viewport + buffer)
    pub fn render_range(&self) -> Range<usize> {
        let start = self.viewport_top.saturating_sub(self.buffer);
        let end = self.viewport_bottom + self.buffer;
        start..end
    }

    /// Check if position should be rendered
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::lazy_render::LazyRenderState;
    ///
    /// let mut state = LazyRenderState::new(100);
    /// state.set_viewport(10, 110);
    ///
    /// assert!(state.should_render(50));
    /// assert!(!state.should_render(200));
    /// ```
    pub fn should_render(&self, position: usize) -> bool {
        self.render_range().contains(&position)
    }

    /// Check if range should be rendered
    pub fn should_render_range(&self, range: Range<usize>) -> bool {
        let render_range = self.render_range();
        range.start < render_range.end && range.end > render_range.start
    }

    /// Get viewport top
    pub fn viewport_top(&self) -> usize {
        self.viewport_top
    }

    /// Get viewport bottom
    pub fn viewport_bottom(&self) -> usize {
        self.viewport_bottom
    }

    /// Get buffer size
    pub fn buffer(&self) -> usize {
        self.buffer
    }
}

impl Default for LazyRenderState {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Renderable item trait
pub trait LazyRenderable {
    /// Get item position (y coordinate)
    fn position(&self) -> usize;

    /// Get item height
    fn height(&self) -> usize {
        1
    }

    /// Get item range
    fn range(&self) -> Range<usize> {
        let pos = self.position();
        pos..(pos + self.height())
    }
}

/// Lazy render manager
#[derive(Debug, Clone)]
pub struct LazyRenderManager {
    /// Render state
    state: LazyRenderState,
    /// Count of items skipped
    skipped_count: usize,
    /// Count of items rendered
    rendered_count: usize,
}

impl LazyRenderManager {
    /// Create a new lazy render manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::lazy_render::LazyRenderManager;
    ///
    /// let manager = LazyRenderManager::new(100);
    /// assert_eq!(manager.skipped_count(), 0);
    /// ```
    pub fn new(viewport_height: usize) -> Self {
        Self {
            state: LazyRenderState::new(viewport_height),
            skipped_count: 0,
            rendered_count: 0,
        }
    }

    /// Update viewport
    pub fn update_viewport(&mut self, top: usize, bottom: usize) {
        self.state.set_viewport(top, bottom);
    }

    /// Set buffer
    pub fn set_buffer(&mut self, buffer: usize) {
        self.state.set_buffer(buffer);
    }

    /// Reset counters
    pub fn reset_counters(&mut self) {
        self.skipped_count = 0;
        self.rendered_count = 0;
    }

    /// Check if item should be rendered and update counters
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::lazy_render::LazyRenderManager;
    ///
    /// let mut manager = LazyRenderManager::new(100);
    /// manager.update_viewport(0, 100);
    ///
    /// if manager.should_render(50) {
    ///     // Render item at position 50
    /// }
    /// assert_eq!(manager.rendered_count(), 1);
    /// ```
    pub fn should_render(&mut self, position: usize) -> bool {
        let should_render = self.state.should_render(position);
        if should_render {
            self.rendered_count += 1;
        } else {
            self.skipped_count += 1;
        }
        should_render
    }

    /// Check if range should be rendered
    pub fn should_render_range(&mut self, range: Range<usize>) -> bool {
        let should_render = self.state.should_render_range(range);
        if should_render {
            self.rendered_count += 1;
        } else {
            self.skipped_count += 1;
        }
        should_render
    }

    /// Get number of items skipped
    pub fn skipped_count(&self) -> usize {
        self.skipped_count
    }

    /// Get number of items rendered
    pub fn rendered_count(&self) -> usize {
        self.rendered_count
    }

    /// Get rendering efficiency (percentage of items rendered)
    pub fn efficiency(&self) -> f64 {
        let total = self.skipped_count + self.rendered_count;
        if total == 0 {
            1.0
        } else {
            self.rendered_count as f64 / total as f64
        }
    }

    /// Get state
    pub fn state(&self) -> &LazyRenderState {
        &self.state
    }
}

impl Default for LazyRenderManager {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_render_state_creation() {
        let state = LazyRenderState::new(100);
        assert_eq!(state.viewport_top(), 0);
        assert_eq!(state.viewport_bottom(), 100);
        assert_eq!(state.buffer(), 5);
    }

    #[test]
    fn test_should_render() {
        let mut state = LazyRenderState::new(100);
        state.set_viewport(10, 110);

        // Within viewport
        assert!(state.should_render(50));
        assert!(state.should_render(10));
        assert!(state.should_render(109));

        // Within buffer
        assert!(state.should_render(5)); // 10 - 5 = 5
        assert!(state.should_render(114)); // 110 + 5 = 115

        // Outside range
        assert!(!state.should_render(0));
        assert!(!state.should_render(200));
    }

    #[test]
    fn test_render_range() {
        let mut state = LazyRenderState::new(100);
        state.set_viewport(10, 110);

        let range = state.render_range();
        assert_eq!(range.start, 5); // 10 - 5
        assert_eq!(range.end, 115); // 110 + 5
    }

    #[test]
    fn test_should_render_range() {
        let mut state = LazyRenderState::new(100);
        state.set_viewport(10, 110);

        // Range fully within
        assert!(state.should_render_range(50..60));

        // Range partially overlapping
        assert!(state.should_render_range(0..20));
        assert!(state.should_render_range(100..120));

        // Range fully outside
        assert!(!state.should_render_range(200..210));
    }

    #[test]
    fn test_set_buffer() {
        let mut state = LazyRenderState::new(100);
        state.set_viewport(10, 110);
        state.set_buffer(10);

        let range = state.render_range();
        assert_eq!(range.start, 0); // 10 - 10 = 0
        assert_eq!(range.end, 120); // 110 + 10
    }

    #[test]
    fn test_manager_creation() {
        let manager = LazyRenderManager::new(100);
        assert_eq!(manager.skipped_count(), 0);
        assert_eq!(manager.rendered_count(), 0);
    }

    #[test]
    fn test_manager_should_render() {
        let mut manager = LazyRenderManager::new(100);
        manager.update_viewport(0, 100);

        assert!(manager.should_render(50));
        assert_eq!(manager.rendered_count(), 1);
        assert_eq!(manager.skipped_count(), 0);

        assert!(!manager.should_render(200));
        assert_eq!(manager.rendered_count(), 1);
        assert_eq!(manager.skipped_count(), 1);
    }

    #[test]
    fn test_manager_reset_counters() {
        let mut manager = LazyRenderManager::new(100);
        manager.update_viewport(0, 100);

        manager.should_render(50);
        manager.should_render(200);

        assert_eq!(manager.rendered_count(), 1);
        assert_eq!(manager.skipped_count(), 1);

        manager.reset_counters();
        assert_eq!(manager.rendered_count(), 0);
        assert_eq!(manager.skipped_count(), 0);
    }

    #[test]
    fn test_manager_efficiency() {
        let mut manager = LazyRenderManager::new(100);
        manager.update_viewport(0, 100);

        // Render 3, skip 2
        manager.should_render(10);
        manager.should_render(20);
        manager.should_render(30);
        manager.should_render(200);
        manager.should_render(300);

        assert_eq!(manager.efficiency(), 0.6); // 3/5
    }

    #[test]
    fn test_manager_efficiency_no_items() {
        let manager = LazyRenderManager::new(100);
        assert_eq!(manager.efficiency(), 1.0); // No items = 100% efficient
    }

    #[test]
    fn test_large_viewport() {
        let mut state = LazyRenderState::new(1000);
        state.set_viewport(500, 1500);

        assert!(state.should_render(1000));
        assert!(!state.should_render(2000));
    }

    #[test]
    fn test_zero_buffer() {
        let mut state = LazyRenderState::new(100);
        state.set_viewport(10, 110);
        state.set_buffer(0);

        let range = state.render_range();
        assert_eq!(range.start, 10);
        assert_eq!(range.end, 110);
    }

    #[test]
    fn test_viewport_at_zero() {
        let state = LazyRenderState::new(100);
        // Default viewport is 0..100

        assert!(state.should_render(0));
        assert!(state.should_render(50));
        assert!(state.should_render(99));
        assert!(state.should_render(104)); // Within buffer (100 + 5 - 1)
    }
}
