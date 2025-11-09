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
}
