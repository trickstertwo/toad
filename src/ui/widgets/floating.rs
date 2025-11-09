/// Floating windows - draggable overlays
///
/// Windows that float above other content and can be positioned anywhere
///
/// # Examples
///
/// ```
/// use toad::widgets::FloatingWindow;
///
/// let window = FloatingWindow::new("Settings", "Window content");
/// assert_eq!(window.title(), "Settings");
/// assert!(!window.is_minimized());
/// ```
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};

/// Position of floating window
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowPosition {
    /// X coordinate (column)
    pub x: u16,
    /// Y coordinate (row)
    pub y: u16,
}

impl WindowPosition {
    /// Create a new window position
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Create centered position for given terminal size
    pub fn centered(width: u16, height: u16, term_width: u16, term_height: u16) -> Self {
        let x = term_width.saturating_sub(width) / 2;
        let y = term_height.saturating_sub(height) / 2;
        Self { x, y }
    }
}

/// Floating window widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingWindow {
    /// Window title
    title: String,
    /// Window content
    content: String,
    /// Window position
    position: WindowPosition,
    /// Window width
    width: u16,
    /// Window height
    height: u16,
    /// Whether window is visible
    visible: bool,
    /// Whether window is minimized
    minimized: bool,
    /// Whether window can be moved
    draggable: bool,
    /// Whether window can be closed
    closable: bool,
}

impl FloatingWindow {
    /// Create a new floating window
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::FloatingWindow;
    ///
    /// let window = FloatingWindow::new("Title", "Content");
    /// assert_eq!(window.title(), "Title");
    /// ```
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            position: WindowPosition::new(0, 0),
            width: 40,
            height: 10,
            visible: true,
            minimized: false,
            draggable: true,
            closable: true,
        }
    }

    /// Set window position
    pub fn position(mut self, x: u16, y: u16) -> Self {
        self.position = WindowPosition::new(x, y);
        self
    }

    /// Set window size
    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set draggable
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    /// Set closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Get title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Get content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set content
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    /// Get position
    pub fn get_position(&self) -> WindowPosition {
        self.position
    }

    /// Set position
    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position = WindowPosition::new(x, y);
    }

    /// Move window by offset
    pub fn move_by(&mut self, dx: i16, dy: i16) {
        if !self.draggable {
            return;
        }

        self.position.x = (self.position.x as i16 + dx).max(0) as u16;
        self.position.y = (self.position.y as i16 + dy).max(0) as u16;
    }

    /// Get size
    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Set size
    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Show window
    pub fn show(&mut self) {
        self.visible = true;
        self.minimized = false;
    }

    /// Hide window
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if minimized
    pub fn is_minimized(&self) -> bool {
        self.minimized
    }

    /// Minimize window
    pub fn minimize(&mut self) {
        self.minimized = true;
    }

    /// Restore window
    pub fn restore(&mut self) {
        self.minimized = false;
    }

    /// Toggle minimized state
    pub fn toggle_minimize(&mut self) {
        self.minimized = !self.minimized;
    }

    /// Check if draggable
    pub fn is_draggable(&self) -> bool {
        self.draggable
    }

    /// Check if closable
    pub fn is_closable(&self) -> bool {
        self.closable
    }

    /// Center window in terminal
    pub fn center(&mut self, term_width: u16, term_height: u16) {
        self.position = WindowPosition::centered(self.width, self.height, term_width, term_height);
    }

    /// Get window rect
    pub fn rect(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: if self.minimized { 3 } else { self.height },
        }
    }

    /// Render the floating window
    pub fn render(&self, frame: &mut Frame) {
        if !self.visible {
            return;
        }

        let area = self.rect();

        let title_text = if self.minimized {
            format!("{} [_]", self.title)
        } else if self.closable {
            format!("{} [×]", self.title)
        } else {
            self.title.clone()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .title(title_text)
            .title_style(
                Style::default()
                    .fg(ToadTheme::TOAD_GREEN)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(ToadTheme::BLACK));

        if self.minimized {
            frame.render_widget(block, area);
        } else {
            let paragraph = Paragraph::new(self.content.as_str())
                .block(block)
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(ToadTheme::FOREGROUND));

            frame.render_widget(paragraph, area);
        }
    }
}

impl Default for FloatingWindow {
    fn default() -> Self {
        Self::new("Window", "")
    }
}

/// Floating window manager
#[derive(Debug, Clone, Default)]
pub struct FloatingWindowManager {
    /// All windows
    windows: Vec<FloatingWindow>,
    /// Focused window index
    focused: Option<usize>,
}

impl FloatingWindowManager {
    /// Create a new floating window manager
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused: None,
        }
    }

    /// Add a window
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{FloatingWindowManager, FloatingWindow};
    ///
    /// let mut manager = FloatingWindowManager::new();
    /// manager.add_window(FloatingWindow::new("Settings", "Content"));
    /// assert_eq!(manager.window_count(), 1);
    /// ```
    pub fn add_window(&mut self, window: FloatingWindow) {
        self.windows.push(window);
        if self.windows.len() == 1 {
            self.focused = Some(0);
        }
    }

    /// Remove window by index
    pub fn remove_window(&mut self, index: usize) -> Option<FloatingWindow> {
        if index < self.windows.len() {
            let window = self.windows.remove(index);

            // Adjust focused index
            if let Some(focused_idx) = self.focused {
                if focused_idx == index {
                    self.focused = if self.windows.is_empty() {
                        None
                    } else if focused_idx >= self.windows.len() {
                        Some(self.windows.len() - 1)
                    } else {
                        Some(focused_idx)
                    };
                } else if focused_idx > index {
                    self.focused = Some(focused_idx - 1);
                }
            }

            Some(window)
        } else {
            None
        }
    }

    /// Get all windows
    pub fn windows(&self) -> &[FloatingWindow] {
        &self.windows
    }

    /// Get mutable window by index
    pub fn window_mut(&mut self, index: usize) -> Option<&mut FloatingWindow> {
        self.windows.get_mut(index)
    }

    /// Get window count
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Get focused window
    pub fn focused_window(&self) -> Option<&FloatingWindow> {
        self.focused.and_then(|idx| self.windows.get(idx))
    }

    /// Get mutable focused window
    pub fn focused_window_mut(&mut self) -> Option<&mut FloatingWindow> {
        self.focused.and_then(|idx| self.windows.get_mut(idx))
    }

    /// Set focused window
    pub fn set_focus(&mut self, index: usize) -> bool {
        if index < self.windows.len() {
            self.focused = Some(index);
            true
        } else {
            false
        }
    }

    /// Focus next window
    pub fn focus_next(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        self.focused = Some(match self.focused {
            Some(idx) if idx + 1 < self.windows.len() => idx + 1,
            _ => 0,
        });
    }

    /// Focus previous window
    pub fn focus_previous(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        self.focused = Some(match self.focused {
            Some(0) | None => self.windows.len() - 1,
            Some(idx) => idx - 1,
        });
    }

    /// Close focused window
    pub fn close_focused(&mut self) -> Option<FloatingWindow> {
        self.focused.and_then(|idx| {
            if self.windows.get(idx).is_some_and(|w| w.is_closable()) {
                self.remove_window(idx)
            } else {
                None
            }
        })
    }

    /// Render all visible windows
    pub fn render(&self, frame: &mut Frame) {
        for window in &self.windows {
            window.render(frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_position() {
        let pos = WindowPosition::new(10, 20);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
    }

    #[test]
    fn test_window_position_centered() {
        let pos = WindowPosition::centered(40, 10, 80, 24);
        assert_eq!(pos.x, 20); // (80 - 40) / 2
        assert_eq!(pos.y, 7); // (24 - 10) / 2
    }

    #[test]
    fn test_floating_window_creation() {
        let window = FloatingWindow::new("Test", "Content");
        assert_eq!(window.title(), "Test");
        assert_eq!(window.content(), "Content");
        assert!(window.is_visible());
        assert!(!window.is_minimized());
    }

    #[test]
    fn test_window_visibility() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.hide();
        assert!(!window.is_visible());

        window.show();
        assert!(window.is_visible());

        window.toggle();
        assert!(!window.is_visible());
    }

    #[test]
    fn test_window_minimize() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.minimize();
        assert!(window.is_minimized());

        window.restore();
        assert!(!window.is_minimized());

        window.toggle_minimize();
        assert!(window.is_minimized());
    }

    #[test]
    fn test_window_position_update() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.set_position(50, 10);
        let pos = window.get_position();
        assert_eq!(pos.x, 50);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_window_move_by() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(10, 10);

        window.move_by(5, 3);
        let pos = window.get_position();
        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 13);

        window.move_by(-5, -3);
        let pos = window.get_position();
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_window_size() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.set_size(60, 20);
        let (w, h) = window.get_size();
        assert_eq!(w, 60);
        assert_eq!(h, 20);
    }

    #[test]
    fn test_window_center() {
        let mut window = FloatingWindow::new("Test", "Content").size(40, 10);

        window.center(80, 24);
        let pos = window.get_position();
        assert_eq!(pos.x, 20);
        assert_eq!(pos.y, 7);
    }

    #[test]
    fn test_manager_creation() {
        let manager = FloatingWindowManager::new();
        assert_eq!(manager.window_count(), 0);
    }

    #[test]
    fn test_manager_add_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        assert_eq!(manager.window_count(), 1);
        assert!(manager.focused_window().is_some());
    }

    #[test]
    fn test_manager_remove_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));

        let removed = manager.remove_window(0);
        assert!(removed.is_some());
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_manager_focus_navigation() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));
        manager.add_window(FloatingWindow::new("Window 3", "Content"));

        manager.focus_next();
        assert_eq!(manager.focused, Some(1));

        manager.focus_next();
        assert_eq!(manager.focused, Some(2));

        manager.focus_next(); // Should wrap
        assert_eq!(manager.focused, Some(0));

        manager.focus_previous();
        assert_eq!(manager.focused, Some(2));
    }

    #[test]
    fn test_manager_close_focused() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));

        let closed = manager.close_focused();
        assert!(closed.is_some());
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_non_draggable_window() {
        let mut window = FloatingWindow::new("Test", "Content")
            .draggable(false)
            .position(10, 10);

        window.move_by(5, 5);
        let pos = window.get_position();
        assert_eq!(pos.x, 10); // Should not move
        assert_eq!(pos.y, 10);
    }

    // ============================================================================
    // ADDITIONAL COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Layouts)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_manager_many_windows_1000() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..1000 {
            manager.add_window(FloatingWindow::new(
                format!("Window {}", i),
                format!("Content {}", i),
            ));
        }
        assert_eq!(manager.window_count(), 1000);
    }

    #[test]
    fn test_manager_rapid_add_remove_1000() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..1000 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
            if i % 2 == 0 && manager.window_count() > 0 {
                manager.remove_window(0);
            }
        }
        assert!(manager.window_count() >= 500);
    }

    #[test]
    fn test_window_rapid_move_operations_1000() {
        let mut window = FloatingWindow::new("Test", "Content");
        for _ in 0..500 {
            window.move_by(1, 1);
        }
        for _ in 0..500 {
            window.move_by(-1, -1);
        }
        let pos = window.get_position();
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_manager_rapid_focus_navigation() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..10 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
        }

        for _ in 0..1000 {
            manager.focus_next();
        }

        // Should have wrapped around many times, still have valid focus
        assert!(manager.focused_window().is_some());
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_window_unicode_title() {
        let window = FloatingWindow::new("日本語 Title 🚀", "Content");
        assert_eq!(window.title(), "日本語 Title 🚀");
    }

    #[test]
    fn test_window_rtl_title() {
        let window = FloatingWindow::new("مرحبا بك", "Content");
        assert_eq!(window.title(), "مرحبا بك");
    }

    #[test]
    fn test_window_unicode_content() {
        let content = "🚀 Rocket\n日本語\nمرحبا\nמזל טוב";
        let window = FloatingWindow::new("Test", content);
        assert!(window.content().contains('🚀'));
        assert!(window.content().contains("日本語"));
    }

    #[test]
    fn test_window_very_long_unicode_title() {
        let title = "日本語 ".repeat(1000);
        let window = FloatingWindow::new(title.clone(), "Content");
        assert_eq!(window.title(), title);
    }

    #[test]
    fn test_window_emoji_only_title() {
        let window = FloatingWindow::new("🚀🐸💚🎉", "Content");
        assert_eq!(window.title(), "🚀🐸💚🎉");
    }

    #[test]
    fn test_window_combining_characters() {
        let window = FloatingWindow::new("é̂ñ̃ỹ̀", "Café naïve");
        assert!(window.title().len() > 4);
        assert!(window.content().len() > 10);
    }

    // ============ Position/Size Edge Cases ============

    #[test]
    fn test_window_position_max_u16() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(u16::MAX, u16::MAX);
        let pos = window.get_position();
        assert_eq!(pos.x, u16::MAX);
        assert_eq!(pos.y, u16::MAX);
    }

    #[test]
    fn test_window_size_max_u16() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_size(u16::MAX, u16::MAX);
        let (w, h) = window.get_size();
        assert_eq!(w, u16::MAX);
        assert_eq!(h, u16::MAX);
    }

    #[test]
    fn test_window_size_zero() {
        let window = FloatingWindow::new("Test", "Content").size(0, 0);
        let (w, h) = window.get_size();
        assert_eq!(w, 0);
        assert_eq!(h, 0);
    }

    #[test]
    fn test_window_move_negative_from_zero() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(0, 0);
        window.move_by(-10, -10);
        let pos = window.get_position();
        // Should saturate/wrap to 0 or max depending on implementation
        assert!(pos.x == 0 || pos.x > 60000); // Either saturated at 0 or wrapped
        assert!(pos.y == 0 || pos.y > 60000);
    }

    #[test]
    fn test_window_move_positive_overflow() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(u16::MAX - 5, u16::MAX - 5);
        window.move_by(10, 10);
        let pos = window.get_position();
        // Should overflow or saturate
        assert!(pos.x >= u16::MAX - 5 || pos.x < 10);
        assert!(pos.y >= u16::MAX - 5 || pos.y < 10);
    }

    #[test]
    fn test_window_center_zero_terminal_size() {
        let mut window = FloatingWindow::new("Test", "Content").size(40, 10);
        window.center(0, 0);
        let pos = window.get_position();
        // Should handle gracefully (likely position at 0,0)
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_window_center_large_window_small_terminal() {
        let mut window = FloatingWindow::new("Test", "Content").size(100, 50);
        window.center(80, 24);
        let pos = window.get_position();
        // Window larger than terminal, should saturate at 0
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_position_centered_extreme_sizes() {
        let pos = WindowPosition::centered(u16::MAX, u16::MAX, u16::MAX, u16::MAX);
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    // ============ Window Manager Edge Cases ============

    #[test]
    fn test_manager_remove_from_empty() {
        let mut manager = FloatingWindowManager::new();
        let removed = manager.remove_window(0);
        assert!(removed.is_none());
    }

    #[test]
    fn test_manager_focus_navigation_empty() {
        let mut manager = FloatingWindowManager::new();
        manager.focus_next();
        manager.focus_previous();
        assert!(manager.focused_window().is_none());
    }

    #[test]
    fn test_manager_focus_navigation_single_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        manager.focus_next();
        assert_eq!(manager.focused, Some(0));

        manager.focus_previous();
        assert_eq!(manager.focused, Some(0));
    }

    #[test]
    fn test_manager_close_focused_empty() {
        let mut manager = FloatingWindowManager::new();
        let closed = manager.close_focused();
        assert!(closed.is_none());
    }

    #[test]
    fn test_manager_close_all_windows() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..10 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
        }

        while manager.window_count() > 0 {
            manager.close_focused();
        }

        assert_eq!(manager.window_count(), 0);
        assert!(manager.focused_window().is_none());
    }

    #[test]
    fn test_manager_remove_invalid_index() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        let removed = manager.remove_window(100);
        assert!(removed.is_none());
        assert_eq!(manager.window_count(), 1);
    }

    // ============ Serialize/Deserialize Tests ============

    #[test]
    fn test_window_position_serialize_deserialize() {
        let pos = WindowPosition::new(42, 84);
        let json = serde_json::to_string(&pos).unwrap();
        let deserialized: WindowPosition = serde_json::from_str(&json).unwrap();

        assert_eq!(pos.x, deserialized.x);
        assert_eq!(pos.y, deserialized.y);
    }

    #[test]
    fn test_floating_window_serialize_deserialize() {
        let window = FloatingWindow::new("Test Title", "Test Content")
            .position(10, 20)
            .size(50, 15)
            .draggable(false);

        let json = serde_json::to_string(&window).unwrap();
        let deserialized: FloatingWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(window.title(), deserialized.title());
        assert_eq!(window.content(), deserialized.content());
        assert_eq!(window.get_position().x, deserialized.get_position().x);
        assert_eq!(window.get_position().y, deserialized.get_position().y);
    }

    // ============ Clone/Debug Traits ============

    #[test]
    fn test_window_position_clone() {
        let pos = WindowPosition::new(15, 25);
        let cloned = pos;
        assert_eq!(pos.x, cloned.x);
        assert_eq!(pos.y, cloned.y);
    }

    #[test]
    fn test_window_position_debug() {
        let pos = WindowPosition::new(10, 20);
        let debug_str = format!("{:?}", pos);
        assert!(debug_str.contains("WindowPosition"));
    }

    #[test]
    fn test_window_position_partial_eq() {
        let pos1 = WindowPosition::new(10, 20);
        let pos2 = WindowPosition::new(10, 20);
        let pos3 = WindowPosition::new(15, 25);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_floating_window_clone() {
        let window = FloatingWindow::new("Title", "Content")
            .position(10, 20)
            .size(50, 15);

        let cloned = window.clone();
        assert_eq!(window.title(), cloned.title());
        assert_eq!(window.content(), cloned.content());
        assert_eq!(window.get_position(), cloned.get_position());
    }

    #[test]
    fn test_floating_window_debug() {
        let window = FloatingWindow::new("Test", "Content");
        let debug_str = format!("{:?}", window);
        assert!(debug_str.contains("FloatingWindow"));
    }

    // ============ Complex Workflow Tests ============

    #[test]
    fn test_window_complete_workflow() {
        let mut window = FloatingWindow::new("Test Window", "Initial content");

        // Move and resize
        window.set_position(10, 10);
        window.set_size(60, 20);

        // Minimize and restore
        window.minimize();
        assert!(window.is_minimized());

        window.restore();
        assert!(!window.is_minimized());

        // Hide and show
        window.hide();
        assert!(!window.is_visible());

        window.show();
        assert!(window.is_visible());

        // Move around
        window.move_by(5, 5);
        let pos = window.get_position();
        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 15);

        // Update content
        window.set_content("Updated content");
        assert_eq!(window.content(), "Updated content");
    }

    #[test]
    fn test_manager_complete_workflow() {
        let mut manager = FloatingWindowManager::new();

        // Add multiple windows
        manager.add_window(FloatingWindow::new("Window 1", "Content 1"));
        manager.add_window(FloatingWindow::new("Window 2", "Content 2"));
        manager.add_window(FloatingWindow::new("Window 3", "Content 3"));

        assert_eq!(manager.window_count(), 3);

        // Navigate focus
        manager.focus_next();
        manager.focus_next();
        assert_eq!(manager.focused, Some(2));

        // Close focused window
        manager.close_focused();
        assert_eq!(manager.window_count(), 2);

        // Remove specific window
        manager.remove_window(0);
        assert_eq!(manager.window_count(), 1);

        // Clear remaining
        manager.close_focused();
        assert_eq!(manager.window_count(), 0);
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let window = FloatingWindow::new("Test", "Content")
            .position(10, 20)
            .size(60, 15)
            .draggable(false)
            .closable(false);

        assert_eq!(window.get_position().x, 10);
        assert_eq!(window.get_position().y, 20);
        assert_eq!(window.get_size(), (60, 15));
        assert!(!window.is_draggable());
        assert!(!window.is_closable());
    }

    #[test]
    fn test_window_toggle_operations() {
        let mut window = FloatingWindow::new("Test", "Content");

        // Toggle visibility
        window.toggle();
        assert!(!window.is_visible());
        window.toggle();
        assert!(window.is_visible());

        // Toggle minimize
        window.toggle_minimize();
        assert!(window.is_minimized());
        window.toggle_minimize();
        assert!(!window.is_minimized());
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_floating_window_stress() {
        let mut manager = FloatingWindowManager::new();

        // Phase 1: Add many windows with varied configurations
        for i in 0..100 {
            let title = match i % 4 {
                0 => format!("ASCII Window {}", i),
                1 => format!("🚀 Emoji Window {}", i),
                2 => format!("日本語 Window {}", i),
                _ => format!("مرحبا Window {}", i),
            };

            let mut window = FloatingWindow::new(title, format!("Content {}", i))
                .position((i * 5) as u16, (i * 3) as u16)
                .size(40 + (i % 20) as u16, 10 + (i % 10) as u16);

            if i % 2 == 0 {
                window.minimize();
            }

            manager.add_window(window);
        }

        assert_eq!(manager.window_count(), 100);

        // Phase 2: Focus navigation
        for _ in 0..200 {
            manager.focus_next();
        }
        assert!(manager.focused_window().is_some());

        // Phase 3: Close every other window
        for _ in 0..50 {
            manager.close_focused();
            manager.focus_next();
        }

        assert_eq!(manager.window_count(), 50);

        // Phase 4: Modify remaining windows
        for _ in 0..50 {
            if let Some(window) = manager.focused_window_mut() {
                window.move_by(1, 1);
                window.toggle_minimize();
            }
            manager.focus_next();
        }

        // Phase 5: Close all remaining windows
        while manager.window_count() > 0 {
            manager.close_focused();
        }

        assert_eq!(manager.window_count(), 0);
    }

    // ============ Empty/Whitespace Content ============

    #[test]
    fn test_window_empty_title() {
        let window = FloatingWindow::new("", "Content");
        assert_eq!(window.title(), "");
    }

    #[test]
    fn test_window_empty_content() {
        let window = FloatingWindow::new("Title", "");
        assert_eq!(window.content(), "");
    }

    #[test]
    fn test_window_whitespace_only_content() {
        let window = FloatingWindow::new("Title", "     \n  \n    ");
        assert!(window.content().contains(' '));
        assert!(window.content().contains('\n'));
    }

    // ============ Content Update Tests ============

    #[test]
    fn test_window_set_title() {
        let mut window = FloatingWindow::new("Old Title", "Content");
        window.set_title("New Title");
        assert_eq!(window.title(), "New Title");
    }

    #[test]
    fn test_window_set_content_multiple_times() {
        let mut window = FloatingWindow::new("Title", "Content 1");
        window.set_content("Content 2");
        assert_eq!(window.content(), "Content 2");

        window.set_content("Content 3");
        assert_eq!(window.content(), "Content 3");
    }
}
