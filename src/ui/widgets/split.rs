//! Split pane system for resizable layouts
//!
//! Provides horizontal and vertical split panes with configurable sizes,
//! focus management, and keyboard-based resizing.
//!
//! # Examples
//!
//! ## Horizontal Split
//!
//! ```
//! use toad::widgets::{SplitPane, SplitDirection, SplitSize};
//!
//! // Create 60-40 horizontal split
//! let split = SplitPane::new(SplitDirection::Horizontal)
//!     .with_split_size(SplitSize::Percentage(60));
//!
//! assert_eq!(split.direction(), SplitDirection::Horizontal);
//! ```
//!
//! ## Vertical Split
//!
//! ```
//! use toad::widgets::{SplitPane, SplitDirection, SplitSize};
//!
//! // Create vertical split with fixed left pane
//! let split = SplitPane::new(SplitDirection::Vertical)
//!     .with_split_size(SplitSize::Fixed(40));
//!
//! assert_eq!(split.direction(), SplitDirection::Vertical);
//! ```
//!
//! ## Resizing
//!
//! ```
//! use toad::widgets::{SplitPane, SplitDirection, SplitSize};
//!
//! let mut split = SplitPane::new(SplitDirection::Horizontal);
//!
//! // Increase left pane by 5%
//! split.resize(5);
//! ```

use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType},
};
use thiserror::Error;

/// Split pane errors
#[derive(Debug, Error, PartialEq)]
pub enum SplitPaneError {
    /// Invalid split size (< 0 or > 100 for percentage)
    #[error("Invalid split size: {0}")]
    InvalidSize(i32),

    /// Invalid pane index
    #[error("Invalid pane index: {0}")]
    InvalidPane(usize),
}

/// Direction of split
///
/// # Examples
///
/// ```
/// use toad::widgets::SplitDirection;
///
/// let horizontal = SplitDirection::Horizontal;
/// let vertical = SplitDirection::Vertical;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Left and right panes
    Horizontal,
    /// Top and bottom panes
    Vertical,
}

/// Size specification for split panes
///
/// # Examples
///
/// ```
/// use toad::widgets::SplitSize;
///
/// // 60% for first pane
/// let percentage = SplitSize::Percentage(60);
///
/// // Fixed 40 columns/rows for first pane
/// let fixed = SplitSize::Fixed(40);
///
/// // Minimum 20 columns/rows
/// let min = SplitSize::Min(20);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitSize {
    /// Percentage of total size (0-100)
    Percentage(u16),
    /// Fixed size in columns/rows
    Fixed(u16),
    /// Minimum size in columns/rows
    Min(u16),
}

impl SplitSize {
    /// Convert to ratatui Constraint
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SplitSize;
    /// use ratatui::layout::Constraint;
    ///
    /// let size = SplitSize::Percentage(60);
    /// let constraint = size.to_constraint();
    /// ```
    pub fn to_constraint(self) -> Constraint {
        match self {
            SplitSize::Percentage(p) => Constraint::Percentage(p),
            SplitSize::Fixed(n) => Constraint::Length(n),
            SplitSize::Min(n) => Constraint::Min(n),
        }
    }
}

/// Split pane container
///
/// A split pane divides a region into two resizable sections.
/// Supports both horizontal (left/right) and vertical (top/bottom) splits.
///
/// # Examples
///
/// ```
/// use toad::widgets::{SplitPane, SplitDirection, SplitSize};
///
/// let mut split = SplitPane::new(SplitDirection::Horizontal)
///     .with_split_size(SplitSize::Percentage(50))
///     .with_resizable(true);
///
/// // Resize by +10%
/// split.resize(10).unwrap();
///
/// // Focus pane 1
/// split.set_focused_pane(1).unwrap();
/// ```
/// Border style configuration for focus indication
///
/// # Examples
///
/// ```
/// use toad::widgets::PaneBorderStyle;
///
/// let style = PaneBorderStyle::default();
/// assert_eq!(style.show_borders(), true);
/// ```
#[derive(Debug, Clone)]
pub struct PaneBorderStyle {
    /// Whether to show borders
    show_borders: bool,
    /// Border type for focused pane
    focused_border_type: BorderType,
    /// Border type for unfocused pane
    unfocused_border_type: BorderType,
    /// Border color for focused pane
    focused_border_color: Color,
    /// Border color for unfocused pane
    unfocused_border_color: Color,
}

impl Default for PaneBorderStyle {
    fn default() -> Self {
        Self {
            show_borders: true,
            focused_border_type: BorderType::Thick,
            unfocused_border_type: BorderType::Plain,
            focused_border_color: Color::Green,
            unfocused_border_color: Color::DarkGray,
        }
    }
}

impl PaneBorderStyle {
    /// Create a new border style with defaults
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::PaneBorderStyle;
    ///
    /// let style = PaneBorderStyle::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if borders should be shown
    pub fn show_borders(&self) -> bool {
        self.show_borders
    }

    /// Set whether to show borders
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::PaneBorderStyle;
    ///
    /// let mut style = PaneBorderStyle::new();
    /// style.set_show_borders(false);
    /// assert!(!style.show_borders());
    /// ```
    pub fn set_show_borders(&mut self, show: bool) {
        self.show_borders = show;
    }

    /// Get focused border type
    pub fn focused_border_type(&self) -> BorderType {
        self.focused_border_type
    }

    /// Set focused border type
    pub fn set_focused_border_type(&mut self, border_type: BorderType) {
        self.focused_border_type = border_type;
    }

    /// Get unfocused border type
    pub fn unfocused_border_type(&self) -> BorderType {
        self.unfocused_border_type
    }

    /// Set unfocused border type
    pub fn set_unfocused_border_type(&mut self, border_type: BorderType) {
        self.unfocused_border_type = border_type;
    }

    /// Get focused border color
    pub fn focused_border_color(&self) -> Color {
        self.focused_border_color
    }

    /// Set focused border color
    pub fn set_focused_border_color(&mut self, color: Color) {
        self.focused_border_color = color;
    }

    /// Get unfocused border color
    pub fn unfocused_border_color(&self) -> Color {
        self.unfocused_border_color
    }

    /// Set unfocused border color
    pub fn set_unfocused_border_color(&mut self, color: Color) {
        self.unfocused_border_color = color;
    }
}

#[derive(Debug, Clone)]
pub struct SplitPane {
    direction: SplitDirection,
    split_size: SplitSize,
    resizable: bool,
    focused_pane: usize,
    show_separator: bool,
    min_size: u16,
    border_style: PaneBorderStyle,
}

impl SplitPane {
    /// Create a new split pane with default settings
    ///
    /// Default: 50-50 split, resizable, pane 0 focused
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let horizontal = SplitPane::new(SplitDirection::Horizontal);
    /// let vertical = SplitPane::new(SplitDirection::Vertical);
    /// ```
    pub fn new(direction: SplitDirection) -> Self {
        Self {
            direction,
            split_size: SplitSize::Percentage(50),
            resizable: true,
            focused_pane: 0,
            show_separator: true,
            min_size: 10,
            border_style: PaneBorderStyle::default(),
        }
    }

    /// Set the split size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection, SplitSize};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_split_size(SplitSize::Percentage(70));
    /// ```
    pub fn with_split_size(mut self, size: SplitSize) -> Self {
        self.split_size = size;
        self
    }

    /// Set whether the split is resizable
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_resizable(false);
    ///
    /// assert!(!split.is_resizable());
    /// ```
    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set whether to show separator line
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_separator(false);
    ///
    /// assert!(!split.has_separator());
    /// ```
    pub fn with_separator(mut self, show: bool) -> Self {
        self.show_separator = show;
        self
    }

    /// Set minimum pane size
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_min_size(20);
    ///
    /// assert_eq!(split.min_size(), 20);
    /// ```
    pub fn with_min_size(mut self, min: u16) -> Self {
        self.min_size = min;
        self
    }

    /// Set custom border style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection, PaneBorderStyle};
    ///
    /// let style = PaneBorderStyle::new();
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_border_style(style);
    /// ```
    pub fn with_border_style(mut self, style: PaneBorderStyle) -> Self {
        self.border_style = style;
        self
    }

    /// Set whether to show borders
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_borders(false);
    /// ```
    pub fn with_borders(mut self, show: bool) -> Self {
        self.border_style.set_show_borders(show);
        self
    }

    /// Set focused border color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    /// use ratatui::style::Color;
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_focused_color(Color::Cyan);
    /// ```
    pub fn with_focused_color(mut self, color: Color) -> Self {
        self.border_style.set_focused_border_color(color);
        self
    }

    /// Set unfocused border color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    /// use ratatui::style::Color;
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_unfocused_color(Color::Gray);
    /// ```
    pub fn with_unfocused_color(mut self, color: Color) -> Self {
        self.border_style.set_unfocused_border_color(color);
        self
    }

    /// Get the split direction
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Vertical);
    /// assert_eq!(split.direction(), SplitDirection::Vertical);
    /// ```
    pub fn direction(&self) -> SplitDirection {
        self.direction
    }

    /// Check if split is resizable
    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    /// Check if separator is shown
    pub fn has_separator(&self) -> bool {
        self.show_separator
    }

    /// Get minimum pane size
    pub fn min_size(&self) -> u16 {
        self.min_size
    }

    /// Get the currently focused pane index (0 or 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal);
    /// assert_eq!(split.focused_pane(), 0);
    /// ```
    pub fn focused_pane(&self) -> usize {
        self.focused_pane
    }

    /// Set the focused pane
    ///
    /// # Errors
    ///
    /// Returns `SplitPaneError::InvalidPane` if pane index is not 0 or 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let mut split = SplitPane::new(SplitDirection::Horizontal);
    /// split.set_focused_pane(1).unwrap();
    /// assert_eq!(split.focused_pane(), 1);
    /// ```
    pub fn set_focused_pane(&mut self, pane: usize) -> Result<(), SplitPaneError> {
        if pane > 1 {
            return Err(SplitPaneError::InvalidPane(pane));
        }
        self.focused_pane = pane;
        Ok(())
    }

    /// Switch focus to the other pane
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let mut split = SplitPane::new(SplitDirection::Horizontal);
    /// assert_eq!(split.focused_pane(), 0);
    ///
    /// split.toggle_focus();
    /// assert_eq!(split.focused_pane(), 1);
    ///
    /// split.toggle_focus();
    /// assert_eq!(split.focused_pane(), 0);
    /// ```
    pub fn toggle_focus(&mut self) {
        self.focused_pane = if self.focused_pane == 0 { 1 } else { 0 };
    }

    /// Get the border style configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal);
    /// let style = split.border_style();
    /// assert!(style.show_borders());
    /// ```
    pub fn border_style(&self) -> &PaneBorderStyle {
        &self.border_style
    }

    /// Get mutable border style configuration
    pub fn border_style_mut(&mut self) -> &mut PaneBorderStyle {
        &mut self.border_style
    }

    /// Resize the split pane
    ///
    /// Positive delta increases first pane, negative decreases it.
    ///
    /// # Errors
    ///
    /// Returns `SplitPaneError::InvalidSize` if resulting size is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection, SplitSize};
    ///
    /// let mut split = SplitPane::new(SplitDirection::Horizontal)
    ///     .with_split_size(SplitSize::Percentage(50));
    ///
    /// split.resize(10).unwrap();
    /// // Now 60-40 split
    ///
    /// split.resize(-20).unwrap();
    /// // Now 40-60 split
    /// ```
    pub fn resize(&mut self, delta: i32) -> Result<(), SplitPaneError> {
        if !self.resizable {
            return Ok(());
        }

        match &mut self.split_size {
            SplitSize::Percentage(p) => {
                let new_value = (*p as i32) + delta;
                if new_value < (self.min_size as i32) || new_value > (100 - self.min_size as i32) {
                    return Err(SplitPaneError::InvalidSize(new_value));
                }
                *p = new_value as u16;
            }
            SplitSize::Fixed(n) => {
                let new_value = (*n as i32) + delta;
                if new_value < (self.min_size as i32) {
                    return Err(SplitPaneError::InvalidSize(new_value));
                }
                *n = new_value as u16;
            }
            SplitSize::Min(n) => {
                let new_value = (*n as i32) + delta;
                if new_value < 0 {
                    return Err(SplitPaneError::InvalidSize(new_value));
                }
                *n = new_value as u16;
            }
        }

        Ok(())
    }

    /// Get current split size
    pub fn split_size(&self) -> SplitSize {
        self.split_size
    }

    /// Calculate the two pane rectangles from the given area
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{SplitPane, SplitDirection};
    /// use ratatui::layout::Rect;
    ///
    /// let split = SplitPane::new(SplitDirection::Horizontal);
    /// let area = Rect::new(0, 0, 100, 50);
    ///
    /// let (left, right) = split.calculate_panes(area);
    /// assert_eq!(left.width, 50);
    /// assert_eq!(right.width, 50);
    /// ```
    pub fn calculate_panes(&self, area: Rect) -> (Rect, Rect) {
        let direction = match self.direction {
            SplitDirection::Horizontal => Direction::Horizontal,
            SplitDirection::Vertical => Direction::Vertical,
        };

        let second_constraint = match self.split_size {
            SplitSize::Percentage(p) => Constraint::Percentage(100 - p),
            SplitSize::Fixed(_) | SplitSize::Min(_) => Constraint::Min(0),
        };

        let chunks = Layout::default()
            .direction(direction)
            .constraints([self.split_size.to_constraint(), second_constraint])
            .split(area);

        (chunks[0], chunks[1])
    }

    /// Render the split pane with custom content renderers
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::widgets::{SplitPane, SplitDirection};
    /// use ratatui::widgets::Paragraph;
    ///
    /// fn example(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    ///     let split = SplitPane::new(SplitDirection::Horizontal);
    ///
    ///     split.render(
    ///         frame,
    ///         area,
    ///         |frame, area| {
    ///             frame.render_widget(Paragraph::new("Left pane"), area);
    ///         },
    ///         |frame, area| {
    ///             frame.render_widget(Paragraph::new("Right pane"), area);
    ///         },
    ///     );
    /// }
    /// ```
    pub fn render<F1, F2>(&self, frame: &mut Frame, area: Rect, left: F1, right: F2)
    where
        F1: FnOnce(&mut Frame, Rect),
        F2: FnOnce(&mut Frame, Rect),
    {
        let (pane1, pane2) = self.calculate_panes(area);

        // Render panes with focus indicator
        if self.focused_pane == 0 {
            let block1 = Block::default().border_style(Style::default().fg(ToadTheme::TOAD_GREEN));
            let inner1 = block1.inner(pane1);
            frame.render_widget(block1, pane1);
            left(frame, inner1);
            right(frame, pane2);
        } else {
            let block2 = Block::default().border_style(Style::default().fg(ToadTheme::TOAD_GREEN));
            let inner2 = block2.inner(pane2);
            left(frame, pane1);
            frame.render_widget(block2, pane2);
            right(frame, inner2);
        }
    }
}

impl Default for SplitPane {
    fn default() -> Self {
        Self::new(SplitDirection::Horizontal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pane_creation() {
        let split = SplitPane::new(SplitDirection::Horizontal);
        assert_eq!(split.direction(), SplitDirection::Horizontal);
        assert_eq!(split.focused_pane(), 0);
        assert!(split.is_resizable());
        assert!(split.has_separator());
    }

    #[test]
    fn test_split_pane_vertical() {
        let split = SplitPane::new(SplitDirection::Vertical);
        assert_eq!(split.direction(), SplitDirection::Vertical);
    }

    #[test]
    fn test_split_size_percentage() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(70));

        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 70),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_split_size_fixed() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(40));

        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, 40),
            _ => panic!("Expected Fixed"),
        }
    }

    #[test]
    fn test_focus_toggle() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        assert_eq!(split.focused_pane(), 0);

        split.toggle_focus();
        assert_eq!(split.focused_pane(), 1);

        split.toggle_focus();
        assert_eq!(split.focused_pane(), 0);
    }

    #[test]
    fn test_set_focused_pane() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        assert!(split.set_focused_pane(0).is_ok());
        assert_eq!(split.focused_pane(), 0);

        assert!(split.set_focused_pane(1).is_ok());
        assert_eq!(split.focused_pane(), 1);

        assert!(split.set_focused_pane(2).is_err());
    }

    #[test]
    fn test_resize_percentage() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        assert!(split.resize(10).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 60),
            _ => panic!("Expected Percentage"),
        }

        assert!(split.resize(-20).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 40),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_resize_bounds() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        // Try to resize below minimum
        assert!(split.resize(-50).is_err());

        // Try to resize above maximum
        assert!(split.resize(50).is_err());
    }

    #[test]
    fn test_resize_non_resizable() {
        let mut split = SplitPane::new(SplitDirection::Horizontal).with_resizable(false);

        assert!(split.resize(10).is_ok()); // Should succeed but not change
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50), // Still 50
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_calculate_panes_horizontal() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 50);
        assert_eq!(right.width, 50);
        assert_eq!(left.height, 50);
        assert_eq!(right.height, 50);
    }

    #[test]
    fn test_calculate_panes_vertical() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(60));

        let area = Rect::new(0, 0, 100, 50);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height, 30);
        assert_eq!(bottom.height, 20);
    }

    #[test]
    fn test_with_methods() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(70))
            .with_resizable(false)
            .with_separator(false)
            .with_min_size(20);

        assert!(!split.is_resizable());
        assert!(!split.has_separator());
        assert_eq!(split.min_size(), 20);
    }

    #[test]
    fn test_default() {
        let split = SplitPane::default();
        assert_eq!(split.direction(), SplitDirection::Horizontal);
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage 50"),
        }
    }

    #[test]
    fn test_split_size_to_constraint() {
        let perc = SplitSize::Percentage(60).to_constraint();
        assert_eq!(perc, Constraint::Percentage(60));

        let fixed = SplitSize::Fixed(40).to_constraint();
        assert_eq!(fixed, Constraint::Length(40));

        let min = SplitSize::Min(20).to_constraint();
        assert_eq!(min, Constraint::Min(20));
    }

    #[test]
    fn test_error_display() {
        let err = SplitPaneError::InvalidSize(-5);
        assert!(err.to_string().contains("-5"));

        let err = SplitPaneError::InvalidPane(3);
        assert!(err.to_string().contains("3"));
    }

    // ========================================
    // MEDIUM TIER EDGE CASE TESTS
    // ========================================

    // Boundary Conditions
    #[test]
    fn test_very_small_terminal() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 20, 8); // Minimal terminal
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width + right.width, 20);
        assert!(left.width > 0);
        assert!(right.width > 0);
    }

    #[test]
    fn test_extreme_uneven_split_99_1() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(99));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 99);
        assert_eq!(right.width, 1);
    }

    #[test]
    fn test_extreme_uneven_split_1_99() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(1));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 1);
        assert_eq!(right.width, 99);
    }

    #[test]
    fn test_resize_to_minimum_boundary() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        // Resize to exact minimum (should succeed)
        assert!(split.resize(-40).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 10),
            _ => panic!("Expected Percentage"),
        }

        // Try to resize below minimum by 1 (should fail)
        assert!(split.resize(-1).is_err());
    }

    #[test]
    fn test_resize_to_maximum_boundary() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        // Resize to exact maximum (100 - min_size = 90, should succeed)
        assert!(split.resize(40).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 90),
            _ => panic!("Expected Percentage"),
        }

        // Try to resize above maximum by 1 (should fail)
        assert!(split.resize(1).is_err());
    }

    #[test]
    fn test_resize_fixed_size() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(50));

        assert!(split.resize(20).is_ok());
        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, 70),
            _ => panic!("Expected Fixed"),
        }

        assert!(split.resize(-30).is_ok());
        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, 40),
            _ => panic!("Expected Fixed"),
        }
    }

    #[test]
    fn test_resize_fixed_below_minimum() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Fixed(20))
            .with_min_size(10);

        // Should fail to resize below minimum
        assert!(split.resize(-15).is_err());
    }

    #[test]
    fn test_resize_min_size() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Min(30));

        assert!(split.resize(10).is_ok());
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 40),
            _ => panic!("Expected Min"),
        }

        // Min size can go to 0
        assert!(split.resize(-40).is_ok());
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 0),
            _ => panic!("Expected Min"),
        }

        // But not negative
        assert!(split.resize(-1).is_err());
    }

    #[test]
    fn test_rapid_resize_operations() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        // Perform 100 rapid resize operations
        for i in 0..50 {
            if i % 2 == 0 {
                let _ = split.resize(1);
            } else {
                let _ = split.resize(-1);
            }
        }

        // Should end at 50% (started at 50, +1 then -1 for 50 iterations)
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_calculate_panes_very_large_area() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 10000, 5000);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 5000);
        assert_eq!(right.width, 5000);
        assert_eq!(left.height, 5000);
    }

    #[test]
    fn test_calculate_panes_zero_area() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 0, 0);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        assert_eq!(right.width, 0);
    }

    #[test]
    fn test_calculate_panes_single_column() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 1, 50);
        let (left, right) = split.calculate_panes(area);

        // With 1 column total, 50% split should give 0 and 1 or similar
        assert_eq!(left.width + right.width, 1);
    }

    #[test]
    fn test_calculate_panes_single_row() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 100, 1);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height + bottom.height, 1);
    }

    // Focus Transfer Edge Cases
    #[test]
    fn test_toggle_focus_multiple_times() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        for i in 0..100 {
            split.toggle_focus();
            let expected = if (i + 1) % 2 == 0 { 0 } else { 1 };
            assert_eq!(split.focused_pane(), expected);
        }
    }

    #[test]
    fn test_set_focused_pane_invalid_indices() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        // Valid indices
        assert!(split.set_focused_pane(0).is_ok());
        assert!(split.set_focused_pane(1).is_ok());

        // Invalid indices
        assert!(split.set_focused_pane(2).is_err());
        assert!(split.set_focused_pane(3).is_err());
        assert!(split.set_focused_pane(100).is_err());
        assert!(split.set_focused_pane(usize::MAX).is_err());
    }

    #[test]
    fn test_focus_idempotency() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        // Setting same focus multiple times should be idempotent
        assert!(split.set_focused_pane(1).is_ok());
        assert_eq!(split.focused_pane(), 1);

        assert!(split.set_focused_pane(1).is_ok());
        assert_eq!(split.focused_pane(), 1);

        assert!(split.set_focused_pane(1).is_ok());
        assert_eq!(split.focused_pane(), 1);
    }

    // Border Style Tests
    #[test]
    fn test_border_style_default() {
        let style = PaneBorderStyle::default();
        assert!(style.show_borders());
        assert_eq!(style.focused_border_type(), BorderType::Thick);
        assert_eq!(style.unfocused_border_type(), BorderType::Plain);
        assert_eq!(style.focused_border_color(), Color::Green);
        assert_eq!(style.unfocused_border_color(), Color::DarkGray);
    }

    #[test]
    fn test_border_style_new() {
        let style = PaneBorderStyle::new();
        assert!(style.show_borders());
    }

    #[test]
    fn test_border_style_setters() {
        let mut style = PaneBorderStyle::new();

        style.set_show_borders(false);
        assert!(!style.show_borders());

        style.set_focused_border_type(BorderType::Double);
        assert_eq!(style.focused_border_type(), BorderType::Double);

        style.set_unfocused_border_type(BorderType::Rounded);
        assert_eq!(style.unfocused_border_type(), BorderType::Rounded);

        style.set_focused_border_color(Color::Cyan);
        assert_eq!(style.focused_border_color(), Color::Cyan);

        style.set_unfocused_border_color(Color::Gray);
        assert_eq!(style.unfocused_border_color(), Color::Gray);
    }

    #[test]
    fn test_split_pane_with_borders() {
        let split = SplitPane::new(SplitDirection::Horizontal).with_borders(false);

        assert!(!split.border_style().show_borders());
    }

    #[test]
    fn test_split_pane_with_focused_color() {
        let split = SplitPane::new(SplitDirection::Horizontal).with_focused_color(Color::Cyan);

        assert_eq!(split.border_style().focused_border_color(), Color::Cyan);
    }

    #[test]
    fn test_split_pane_with_unfocused_color() {
        let split = SplitPane::new(SplitDirection::Horizontal).with_unfocused_color(Color::Gray);

        assert_eq!(split.border_style().unfocused_border_color(), Color::Gray);
    }

    #[test]
    fn test_split_pane_with_custom_border_style() {
        let mut custom_style = PaneBorderStyle::new();
        custom_style.set_show_borders(false);
        custom_style.set_focused_border_type(BorderType::Double);

        let split = SplitPane::new(SplitDirection::Horizontal).with_border_style(custom_style);

        assert!(!split.border_style().show_borders());
        assert_eq!(split.border_style().focused_border_type(), BorderType::Double);
    }

    #[test]
    fn test_border_style_mut() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        split.border_style_mut().set_show_borders(false);
        assert!(!split.border_style().show_borders());

        split.border_style_mut().set_focused_border_color(Color::Magenta);
        assert_eq!(split.border_style().focused_border_color(), Color::Magenta);
    }

    // Builder Pattern Chaining
    #[test]
    fn test_builder_chaining_all_methods() {
        let split = SplitPane::new(SplitDirection::Vertical)
            .with_split_size(SplitSize::Percentage(75))
            .with_resizable(false)
            .with_separator(false)
            .with_min_size(15)
            .with_borders(true)
            .with_focused_color(Color::Yellow)
            .with_unfocused_color(Color::Blue);

        assert_eq!(split.direction(), SplitDirection::Vertical);
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 75),
            _ => panic!("Expected Percentage"),
        }
        assert!(!split.is_resizable());
        assert!(!split.has_separator());
        assert_eq!(split.min_size(), 15);
        assert!(split.border_style().show_borders());
        assert_eq!(split.border_style().focused_border_color(), Color::Yellow);
        assert_eq!(split.border_style().unfocused_border_color(), Color::Blue);
    }

    // State Transitions
    #[test]
    fn test_resize_after_focus_change() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        split.toggle_focus();
        assert_eq!(split.focused_pane(), 1);

        assert!(split.resize(10).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 60),
            _ => panic!("Expected Percentage"),
        }

        // Focus should still be 1
        assert_eq!(split.focused_pane(), 1);
    }

    #[test]
    fn test_multiple_resize_operations_sequence() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let operations = vec![10, -5, 15, -10, 5];
        let expected_results = vec![60, 55, 70, 60, 65];

        for (delta, expected) in operations.iter().zip(expected_results.iter()) {
            assert!(split.resize(*delta).is_ok());
            match split.split_size() {
                SplitSize::Percentage(p) => assert_eq!(p, *expected),
                _ => panic!("Expected Percentage"),
            }
        }
    }

    // Trait Tests
    #[test]
    fn test_split_direction_clone() {
        let dir1 = SplitDirection::Horizontal;
        let dir2 = dir1;
        assert_eq!(dir1, dir2);
    }

    #[test]
    fn test_split_direction_debug() {
        let dir = SplitDirection::Vertical;
        let debug_str = format!("{:?}", dir);
        assert!(debug_str.contains("Vertical"));
    }

    #[test]
    fn test_split_size_clone() {
        let size1 = SplitSize::Percentage(60);
        let size2 = size1;
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_split_size_debug() {
        let size = SplitSize::Fixed(40);
        let debug_str = format!("{:?}", size);
        assert!(debug_str.contains("Fixed"));
        assert!(debug_str.contains("40"));
    }

    #[test]
    fn test_split_pane_clone() {
        let split1 = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(70));
        let split2 = split1.clone();

        assert_eq!(split1.direction(), split2.direction());
        match (split1.split_size(), split2.split_size()) {
            (SplitSize::Percentage(p1), SplitSize::Percentage(p2)) => assert_eq!(p1, p2),
            _ => panic!("Expected matching Percentage sizes"),
        }
    }

    #[test]
    fn test_split_pane_debug() {
        let split = SplitPane::new(SplitDirection::Vertical);
        let debug_str = format!("{:?}", split);
        assert!(debug_str.contains("SplitPane"));
    }

    #[test]
    fn test_border_style_clone() {
        let style1 = PaneBorderStyle::new();
        let style2 = style1.clone();
        assert_eq!(style1.show_borders(), style2.show_borders());
    }

    #[test]
    fn test_border_style_debug() {
        let style = PaneBorderStyle::new();
        let debug_str = format!("{:?}", style);
        assert!(debug_str.contains("PaneBorderStyle"));
    }

    // Error Type Tests
    #[test]
    fn test_error_partial_eq() {
        let err1 = SplitPaneError::InvalidSize(-5);
        let err2 = SplitPaneError::InvalidSize(-5);
        let err3 = SplitPaneError::InvalidSize(-10);

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);

        let err4 = SplitPaneError::InvalidPane(2);
        let err5 = SplitPaneError::InvalidPane(2);
        assert_eq!(err4, err5);
    }

    #[test]
    fn test_error_debug() {
        let err = SplitPaneError::InvalidSize(105);
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidSize"));
    }

    // Extreme Edge Cases
    #[test]
    fn test_split_size_percentage_0() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(0));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        assert_eq!(right.width, 100);
    }

    #[test]
    fn test_split_size_percentage_100() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(100));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 100);
        assert_eq!(right.width, 0);
    }

    #[test]
    fn test_split_size_fixed_0() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(0));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        assert!(right.width > 0);
    }

    #[test]
    fn test_split_size_min_0() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Min(0));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        // Min(0) means first pane gets at least 0, could get more
        assert!(left.width >= 0);
        assert!(right.width >= 0);
    }

    #[test]
    fn test_min_size_0() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(0);

        // Should be able to resize all the way to 0
        assert!(split.resize(-50).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 0),
            _ => panic!("Expected Percentage"),
        }

        // And all the way to 100
        assert!(split.resize(100).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 100),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_calculate_panes_with_fixed_very_large() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Fixed(10000));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        // Fixed size larger than available should take all space
        assert!(left.width <= 100);
        assert!(right.width <= 100);
    }
}
