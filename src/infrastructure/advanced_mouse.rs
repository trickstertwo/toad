//! Advanced mouse support with gesture recognition
//!
//! Provides click/drag optimization for trackpad gestures,
//! including double-click, drag & drop, scroll gestures, and multi-touch.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::advanced_mouse::{AdvancedMouseHandler, MouseGesture};
//!
//! let mut handler = AdvancedMouseHandler::new();
//! let gesture = handler.process_click(10, 10);
//! ```

use std::time::{Duration, Instant};

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Mouse gesture type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseGesture {
    /// Single click
    Click,
    /// Double click
    DoubleClick,
    /// Triple click
    TripleClick,
    /// Drag operation
    Drag,
    /// Drop (end of drag)
    Drop,
    /// Scroll up
    ScrollUp,
    /// Scroll down
    ScrollDown,
    /// Right-click menu
    ContextMenu,
    /// Hover (mouse over)
    Hover,
    /// Long press
    LongPress,
}

/// Click record for multi-click detection
#[derive(Debug, Clone)]
struct ClickRecord {
    /// Position X
    x: u16,
    /// Position Y
    y: u16,
    /// Timestamp of click
    time: Instant,
    /// Button used
    button: MouseButton,
}

/// Drag state
#[derive(Debug, Clone)]
struct DragState {
    /// Start X position
    start_x: u16,
    /// Start Y position
    start_y: u16,
    /// Current X position
    current_x: u16,
    /// Current Y position
    current_y: u16,
    /// Drag started timestamp
    started: Instant,
    /// Whether drag is active
    is_active: bool,
}

/// Advanced mouse handler
///
/// Tracks mouse state and detects gestures including double-click,
/// drag & drop, scrolling, and hover.
#[derive(Debug)]
pub struct AdvancedMouseHandler {
    /// Recent click history (for multi-click detection)
    click_history: Vec<ClickRecord>,
    /// Current drag state
    drag_state: Option<DragState>,
    /// Current hover position
    hover_pos: Option<(u16, u16)>,
    /// Last hover update time
    last_hover_time: Option<Instant>,
    /// Double-click time threshold
    double_click_threshold: Duration,
    /// Double-click distance threshold
    double_click_distance: u16,
    /// Drag distance threshold
    drag_threshold: u16,
    /// Long press duration threshold
    long_press_threshold: Duration,
    /// Hover delay before triggering
    hover_delay: Duration,
}

impl AdvancedMouseHandler {
    /// Create a new advanced mouse handler
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::advanced_mouse::AdvancedMouseHandler;
    ///
    /// let handler = AdvancedMouseHandler::new();
    /// ```
    pub fn new() -> Self {
        Self {
            click_history: Vec::new(),
            drag_state: None,
            hover_pos: None,
            last_hover_time: None,
            double_click_threshold: Duration::from_millis(500),
            double_click_distance: 5,
            drag_threshold: 5,
            long_press_threshold: Duration::from_millis(500),
            hover_delay: Duration::from_millis(300),
        }
    }

    /// Process a mouse click
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::advanced_mouse::AdvancedMouseHandler;
    ///
    /// let mut handler = AdvancedMouseHandler::new();
    /// let gesture = handler.process_click(10, 10);
    /// ```
    pub fn process_click(&mut self, x: u16, y: u16) -> MouseGesture {
        self.process_click_with_button(x, y, MouseButton::Left)
    }

    /// Process a mouse click with specific button
    pub fn process_click_with_button(
        &mut self,
        x: u16,
        y: u16,
        button: MouseButton,
    ) -> MouseGesture {
        let now = Instant::now();

        // Check for multi-click
        let click_count = self.count_recent_clicks(x, y, button, now);

        // Record this click
        self.click_history.push(ClickRecord {
            x,
            y,
            time: now,
            button,
        });

        // Cleanup old click history
        self.click_history
            .retain(|c| now.duration_since(c.time) < self.double_click_threshold * 3);

        // Determine gesture based on click count
        match click_count {
            0 => MouseGesture::Click,
            1 => MouseGesture::DoubleClick,
            2 => MouseGesture::TripleClick,
            _ => MouseGesture::Click,
        }
    }

    /// Start a drag operation
    pub fn start_drag(&mut self, x: u16, y: u16) {
        self.drag_state = Some(DragState {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            started: Instant::now(),
            is_active: false,
        });
    }

    /// Update drag position
    pub fn update_drag(&mut self, x: u16, y: u16) -> Option<MouseGesture> {
        if let Some(ref mut drag) = self.drag_state {
            drag.current_x = x;
            drag.current_y = y;

            // Check if we've moved enough to consider it a drag
            if !drag.is_active {
                let dx = (x as i32 - drag.start_x as i32).abs();
                let dy = (y as i32 - drag.start_y as i32).abs();

                if dx > self.drag_threshold as i32 || dy > self.drag_threshold as i32 {
                    drag.is_active = true;
                    return Some(MouseGesture::Drag);
                }
            }

            if drag.is_active {
                Some(MouseGesture::Drag)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// End drag operation
    pub fn end_drag(&mut self) -> Option<MouseGesture> {
        if let Some(drag) = self.drag_state.take() {
            if drag.is_active {
                Some(MouseGesture::Drop)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.drag_state
            .as_ref()
            .map(|d| d.is_active)
            .unwrap_or(false)
    }

    /// Get drag delta (current - start)
    pub fn drag_delta(&self) -> Option<(i16, i16)> {
        self.drag_state.as_ref().map(|d| {
            let dx = d.current_x as i16 - d.start_x as i16;
            let dy = d.current_y as i16 - d.start_y as i16;
            (dx, dy)
        })
    }

    /// Get drag distance
    pub fn drag_distance(&self) -> Option<u16> {
        self.drag_delta()
            .map(|(dx, dy)| ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as u16)
    }

    /// Update hover position
    pub fn update_hover(&mut self, x: u16, y: u16) -> Option<MouseGesture> {
        let now = Instant::now();

        // Check if position changed significantly
        if let Some((old_x, old_y)) = self.hover_pos {
            let dx = (x as i32 - old_x as i32).abs();
            let dy = (y as i32 - old_y as i32).abs();

            if dx > 2 || dy > 2 {
                // Position changed, reset hover
                self.hover_pos = Some((x, y));
                self.last_hover_time = Some(now);
                return None;
            }
        } else {
            // First hover
            self.hover_pos = Some((x, y));
            self.last_hover_time = Some(now);
            return None;
        }

        // Check if we've hovered long enough
        if let Some(last_time) = self.last_hover_time
            && now.duration_since(last_time) >= self.hover_delay
        {
            return Some(MouseGesture::Hover);
        }

        None
    }

    /// Clear hover state
    pub fn clear_hover(&mut self) {
        self.hover_pos = None;
        self.last_hover_time = None;
    }

    /// Process scroll event
    pub fn process_scroll(&mut self, delta: i32) -> MouseGesture {
        if delta > 0 {
            MouseGesture::ScrollUp
        } else {
            MouseGesture::ScrollDown
        }
    }

    /// Process right-click
    pub fn process_right_click(&mut self, x: u16, y: u16) -> MouseGesture {
        self.process_click_with_button(x, y, MouseButton::Right);
        MouseGesture::ContextMenu
    }

    /// Check for long press
    pub fn check_long_press(&mut self) -> Option<MouseGesture> {
        if let Some(drag) = &self.drag_state
            && !drag.is_active
        {
            let now = Instant::now();
            if now.duration_since(drag.started) >= self.long_press_threshold {
                return Some(MouseGesture::LongPress);
            }
        }
        None
    }

    /// Set double-click threshold
    pub fn set_double_click_threshold(&mut self, threshold: Duration) {
        self.double_click_threshold = threshold;
    }

    /// Set drag threshold
    pub fn set_drag_threshold(&mut self, threshold: u16) {
        self.drag_threshold = threshold;
    }

    /// Set hover delay
    pub fn set_hover_delay(&mut self, delay: Duration) {
        self.hover_delay = delay;
    }

    /// Count recent clicks near the given position
    fn count_recent_clicks(&self, x: u16, y: u16, button: MouseButton, now: Instant) -> usize {
        self.click_history
            .iter()
            .rev()
            .take_while(|c| {
                let time_ok = now.duration_since(c.time) < self.double_click_threshold;
                let button_ok = c.button == button;
                let dx = (x as i32 - c.x as i32).abs();
                let dy = (y as i32 - c.y as i32).abs();
                let pos_ok = dx <= self.double_click_distance as i32
                    && dy <= self.double_click_distance as i32;

                time_ok && button_ok && pos_ok
            })
            .count()
    }
}

impl Default for AdvancedMouseHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_click() {
        let mut handler = AdvancedMouseHandler::new();
        let gesture = handler.process_click(10, 10);
        assert_eq!(gesture, MouseGesture::Click);
    }

    #[test]
    fn test_double_click() {
        let mut handler = AdvancedMouseHandler::new();
        handler.process_click(10, 10);
        let gesture = handler.process_click(10, 10);
        assert_eq!(gesture, MouseGesture::DoubleClick);
    }

    #[test]
    fn test_triple_click() {
        let mut handler = AdvancedMouseHandler::new();
        handler.process_click(10, 10);
        handler.process_click(10, 10);
        let gesture = handler.process_click(10, 10);
        assert_eq!(gesture, MouseGesture::TripleClick);
    }

    #[test]
    fn test_drag_operation() {
        let mut handler = AdvancedMouseHandler::new();
        handler.start_drag(10, 10);

        // Small movement shouldn't trigger drag
        assert_eq!(handler.update_drag(12, 12), None);
        assert!(!handler.is_dragging());

        // Larger movement should trigger drag
        assert_eq!(handler.update_drag(20, 20), Some(MouseGesture::Drag));
        assert!(handler.is_dragging());

        // End drag
        assert_eq!(handler.end_drag(), Some(MouseGesture::Drop));
        assert!(!handler.is_dragging());
    }

    #[test]
    fn test_drag_delta() {
        let mut handler = AdvancedMouseHandler::new();
        handler.start_drag(10, 10);
        handler.update_drag(20, 15);

        let delta = handler.drag_delta();
        assert_eq!(delta, Some((10, 5)));
    }

    #[test]
    fn test_drag_distance() {
        let mut handler = AdvancedMouseHandler::new();
        handler.start_drag(0, 0);
        handler.update_drag(3, 4);

        let distance = handler.drag_distance();
        assert_eq!(distance, Some(5)); // 3-4-5 triangle
    }

    #[test]
    fn test_hover() {
        let mut handler = AdvancedMouseHandler::new();
        handler.set_hover_delay(Duration::from_millis(10));

        // First update shouldn't trigger hover
        assert_eq!(handler.update_hover(10, 10), None);

        // Wait a bit
        std::thread::sleep(Duration::from_millis(15));

        // Second update should trigger hover
        assert_eq!(handler.update_hover(10, 10), Some(MouseGesture::Hover));
    }

    #[test]
    fn test_hover_reset_on_movement() {
        let mut handler = AdvancedMouseHandler::new();
        handler.set_hover_delay(Duration::from_millis(10));

        handler.update_hover(10, 10);

        // Move to different position
        let result = handler.update_hover(20, 20);
        assert_eq!(result, None); // Hover should reset
    }

    #[test]
    fn test_scroll_up() {
        let mut handler = AdvancedMouseHandler::new();
        let gesture = handler.process_scroll(1);
        assert_eq!(gesture, MouseGesture::ScrollUp);
    }

    #[test]
    fn test_scroll_down() {
        let mut handler = AdvancedMouseHandler::new();
        let gesture = handler.process_scroll(-1);
        assert_eq!(gesture, MouseGesture::ScrollDown);
    }

    #[test]
    fn test_context_menu() {
        let mut handler = AdvancedMouseHandler::new();
        let gesture = handler.process_right_click(10, 10);
        assert_eq!(gesture, MouseGesture::ContextMenu);
    }

    #[test]
    fn test_long_press() {
        let mut handler = AdvancedMouseHandler::new();
        handler.long_press_threshold = Duration::from_millis(10);

        handler.start_drag(10, 10);

        // Immediately, no long press
        assert_eq!(handler.check_long_press(), None);

        // Wait a bit
        std::thread::sleep(Duration::from_millis(15));

        // Now should detect long press
        assert_eq!(handler.check_long_press(), Some(MouseGesture::LongPress));
    }

    #[test]
    fn test_clear_hover() {
        let mut handler = AdvancedMouseHandler::new();
        handler.update_hover(10, 10);
        assert!(handler.hover_pos.is_some());

        handler.clear_hover();
        assert!(handler.hover_pos.is_none());
    }

    #[test]
    fn test_set_thresholds() {
        let mut handler = AdvancedMouseHandler::new();

        handler.set_double_click_threshold(Duration::from_millis(1000));
        assert_eq!(handler.double_click_threshold, Duration::from_millis(1000));

        handler.set_drag_threshold(10);
        assert_eq!(handler.drag_threshold, 10);

        handler.set_hover_delay(Duration::from_millis(500));
        assert_eq!(handler.hover_delay, Duration::from_millis(500));
    }

    #[test]
    fn test_default() {
        let handler = AdvancedMouseHandler::default();
        assert_eq!(handler.double_click_threshold, Duration::from_millis(500));
        assert_eq!(handler.drag_threshold, 5);
    }
}
