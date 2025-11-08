/// Mouse event handling for the TUI
///
/// Provides mouse support for clicking, scrolling, and dragging
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use serde::{Deserialize, Serialize};

/// Mouse click action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClickAction {
    /// Single left click
    LeftClick,
    /// Double left click
    DoubleClick,
    /// Right click (context menu)
    RightClick,
    /// Middle click
    MiddleClick,
}

/// Mouse scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollDirection {
    /// Scroll up
    Up,
    /// Scroll down
    Down,
}

/// Processed mouse action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    /// Click at position with action type
    Click {
        /// X coordinate
        x: u16,
        /// Y coordinate
        y: u16,
        /// Click action type
        action: ClickAction,
    },
    /// Scroll action
    Scroll {
        /// X coordinate
        x: u16,
        /// Y coordinate
        y: u16,
        /// Scroll direction
        direction: ScrollDirection,
    },
    /// Drag action
    Drag {
        /// Start X
        from_x: u16,
        /// Start Y
        from_y: u16,
        /// End X
        to_x: u16,
        /// End Y
        to_y: u16,
    },
}

/// Mouse state tracker
#[derive(Debug, Default)]
pub struct MouseState {
    /// Last click position
    last_click: Option<(u16, u16, std::time::Instant)>,
    /// Drag start position
    drag_start: Option<(u16, u16)>,
}

impl MouseState {
    /// Create a new mouse state tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Process a mouse event and return an action
    pub fn process_event(&mut self, event: MouseEvent) -> Option<MouseAction> {
        match event.kind {
            MouseEventKind::Down(button) => {
                match button {
                    MouseButton::Left => {
                        // Check for double-click
                        let now = std::time::Instant::now();
                        let is_double_click =
                            if let Some((last_x, last_y, last_time)) = self.last_click {
                                let time_diff = now.duration_since(last_time);
                                let pos_match = (event.column == last_x) && (event.row == last_y);
                                pos_match && time_diff.as_millis() < 500 // 500ms for double-click
                            } else {
                                false
                            };

                        self.last_click = Some((event.column, event.row, now));

                        if is_double_click {
                            Some(MouseAction::Click {
                                x: event.column,
                                y: event.row,
                                action: ClickAction::DoubleClick,
                            })
                        } else {
                            // Start potential drag
                            self.drag_start = Some((event.column, event.row));

                            Some(MouseAction::Click {
                                x: event.column,
                                y: event.row,
                                action: ClickAction::LeftClick,
                            })
                        }
                    }
                    MouseButton::Right => Some(MouseAction::Click {
                        x: event.column,
                        y: event.row,
                        action: ClickAction::RightClick,
                    }),
                    MouseButton::Middle => Some(MouseAction::Click {
                        x: event.column,
                        y: event.row,
                        action: ClickAction::MiddleClick,
                    }),
                }
            }
            MouseEventKind::Up(_) => {
                // End drag
                self.drag_start = None;
                None
            }
            MouseEventKind::Drag(_) => {
                if let Some((start_x, start_y)) = self.drag_start {
                    Some(MouseAction::Drag {
                        from_x: start_x,
                        from_y: start_y,
                        to_x: event.column,
                        to_y: event.row,
                    })
                } else {
                    None
                }
            }
            MouseEventKind::ScrollUp => Some(MouseAction::Scroll {
                x: event.column,
                y: event.row,
                direction: ScrollDirection::Up,
            }),
            MouseEventKind::ScrollDown => Some(MouseAction::Scroll {
                x: event.column,
                y: event.row,
                direction: ScrollDirection::Down,
            }),
            _ => None,
        }
    }

    /// Check if a point is within a rectangle
    pub fn is_in_rect(x: u16, y: u16, rect: Rect) -> bool {
        x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_state_creation() {
        let state = MouseState::new();
        assert!(state.last_click.is_none());
        assert!(state.drag_start.is_none());
    }

    #[test]
    fn test_is_in_rect() {
        let rect = Rect {
            x: 10,
            y: 10,
            width: 20,
            height: 10,
        };

        // Inside
        assert!(MouseState::is_in_rect(15, 15, rect));
        assert!(MouseState::is_in_rect(10, 10, rect)); // Top-left corner

        // Outside
        assert!(!MouseState::is_in_rect(5, 15, rect)); // Left
        assert!(!MouseState::is_in_rect(35, 15, rect)); // Right
        assert!(!MouseState::is_in_rect(15, 5, rect)); // Above
        assert!(!MouseState::is_in_rect(15, 25, rect)); // Below
    }

    #[test]
    fn test_click_action() {
        let mut state = MouseState::new();

        let event = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 5,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };

        let action = state.process_event(event);
        assert!(matches!(
            action,
            Some(MouseAction::Click {
                x: 10,
                y: 5,
                action: ClickAction::LeftClick
            })
        ));
    }

    #[test]
    fn test_scroll_action() {
        let mut state = MouseState::new();

        let event = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 10,
            row: 5,
            modifiers: crossterm::event::KeyModifiers::empty(),
        };

        let action = state.process_event(event);
        assert!(matches!(
            action,
            Some(MouseAction::Scroll {
                x: 10,
                y: 5,
                direction: ScrollDirection::Up
            })
        ));
    }
}
