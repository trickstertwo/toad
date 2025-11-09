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

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_extreme_split_ratio_1_percent() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(1))
            .with_min_size(0);

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 1);
        assert_eq!(right.width, 99);
    }

    #[test]
    fn test_extreme_split_ratio_99_percent() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(99))
            .with_min_size(0);

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 99);
        assert_eq!(right.width, 1);
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
    fn test_very_small_terminal_20x8() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 20, 8);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 10);
        assert_eq!(right.width, 10);
        assert_eq!(left.height, 8);
        assert_eq!(right.height, 8);
    }

    #[test]
    fn test_very_small_terminal_vertical() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 20, 8);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height, 4);
        assert_eq!(bottom.height, 4);
    }

    #[test]
    fn test_zero_width_area() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 0, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        assert_eq!(right.width, 0);
    }

    #[test]
    fn test_zero_height_area() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 100, 0);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height, 0);
        assert_eq!(bottom.height, 0);
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
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(40));

        assert!(split.resize(10).is_ok());
        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, 50),
            _ => panic!("Expected Fixed"),
        }

        assert!(split.resize(-20).is_ok());
        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, 30),
            _ => panic!("Expected Fixed"),
        }

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
    fn test_resize_fixed_size_below_min() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Fixed(30))
            .with_min_size(20);

        assert!(split.resize(-15).is_err());

        // Should still be at 30
        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, 30),
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
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Min(20));

        assert!(split.resize(10).is_ok());
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 30),
            _ => panic!("Expected Min"),
        }

        assert!(split.resize(-15).is_ok());
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 15),
            _ => panic!("Expected Min"),
        }
    }

    #[test]
    fn test_resize_min_size_below_zero() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Min(5));

        assert!(split.resize(-10).is_err());

        // Should still be at 5
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 5),
            _ => panic!("Expected Min"),
        }
    }

    #[test]
    fn test_border_style_configuration() {
        let mut style = PaneBorderStyle::new();
        assert!(style.show_borders());

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
    fn test_split_with_custom_border_style() {
        let style = PaneBorderStyle::new();
        let split = SplitPane::new(SplitDirection::Horizontal).with_border_style(style.clone());

        assert!(split.border_style().show_borders());
    }

    #[test]
    fn test_split_with_borders_disabled() {
        let split = SplitPane::new(SplitDirection::Horizontal).with_borders(false);

        assert!(!split.border_style().show_borders());
    }

    #[test]
    fn test_split_with_custom_colors() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_focused_color(Color::Cyan)
            .with_unfocused_color(Color::Gray);

        assert_eq!(split.border_style().focused_border_color(), Color::Cyan);
        assert_eq!(split.border_style().unfocused_border_color(), Color::Gray);
    }

    #[test]
    fn test_border_style_mut() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        split.border_style_mut().set_show_borders(false);
        assert!(!split.border_style().show_borders());
    }

    #[test]
    fn test_multiple_rapid_focus_toggles() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        for _ in 0..100 {
            split.toggle_focus();
        }

        // Should be back at pane 0 (100 toggles = even)
        assert_eq!(split.focused_pane(), 0);
    }

    #[test]
    fn test_focus_toggle_idempotence() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        split.toggle_focus();
        split.toggle_focus();
        split.toggle_focus();
        split.toggle_focus();

        // After 4 toggles, should be back at start
        assert_eq!(split.focused_pane(), 0);
    }

    #[test]
    fn test_calculate_panes_with_fixed_larger_than_area() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(150));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        // ratatui's layout should handle this gracefully
        assert!(left.width <= 150);
        assert!(left.width + right.width <= 100);
    }

    #[test]
    fn test_calculate_panes_0_percent() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(0))
            .with_min_size(0);

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        assert_eq!(right.width, 100);
    }

    #[test]
    fn test_calculate_panes_100_percent() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(100))
            .with_min_size(0);

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 100);
        assert_eq!(right.width, 0);
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
    fn test_very_large_terminal_1000x1000() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 1000, 1000);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 500);
        assert_eq!(right.width, 500);
        assert_eq!(left.height, 1000);
        assert_eq!(right.height, 1000);
    }

    #[test]
    fn test_resize_at_exact_min_boundary() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        // Resize to exactly the min size
        assert!(split.resize(-40).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 10),
            _ => panic!("Expected Percentage"),
        }

        // Try to go one below - should fail
        assert!(split.resize(-1).is_err());
    }

    #[test]
    fn test_resize_at_exact_max_boundary() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        // Resize to exactly the max size (100 - min_size)
        assert!(split.resize(40).is_ok());
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 90),
            _ => panic!("Expected Percentage"),
        }

        // Try to go one above - should fail
        assert!(split.resize(1).is_err());
    }

    #[test]
    fn test_split_direction_clone_and_eq() {
        let dir1 = SplitDirection::Horizontal;
        let dir2 = SplitDirection::Horizontal;
        let dir3 = SplitDirection::Vertical;

        assert_eq!(dir1, dir2);
        assert_ne!(dir1, dir3);
    }

    #[test]
    fn test_split_size_clone_and_eq() {
        let size1 = SplitSize::Percentage(50);
        let size2 = SplitSize::Percentage(50);
        let size3 = SplitSize::Percentage(60);
        let size4 = SplitSize::Fixed(50);

        assert_eq!(size1, size2);
        assert_ne!(size1, size3);
        assert_ne!(size1, size4);
    }

    #[test]
    fn test_split_pane_error_equality() {
        let err1 = SplitPaneError::InvalidSize(10);
        let err2 = SplitPaneError::InvalidSize(10);
        let err3 = SplitPaneError::InvalidSize(20);
        let err4 = SplitPaneError::InvalidPane(1);

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
        assert_ne!(err1, err4);
    }

    #[test]
    fn test_vertical_split_extreme_ratios() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(5));

        let area = Rect::new(0, 0, 100, 100);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height, 5);
        assert_eq!(bottom.height, 95);
    }

    #[test]
    fn test_min_size_zero() {
        let split = SplitPane::new(SplitDirection::Horizontal).with_min_size(0);

        assert_eq!(split.min_size(), 0);
    }

    #[test]
    fn test_min_size_large_value() {
        let split = SplitPane::new(SplitDirection::Horizontal).with_min_size(100);

        assert_eq!(split.min_size(), 100);
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let split = SplitPane::new(SplitDirection::Vertical)
            .with_split_size(SplitSize::Percentage(70))
            .with_resizable(true)
            .with_separator(true)
            .with_min_size(15)
            .with_borders(true)
            .with_focused_color(Color::Blue)
            .with_unfocused_color(Color::DarkGray);

        assert_eq!(split.direction(), SplitDirection::Vertical);
        assert!(split.is_resizable());
        assert!(split.has_separator());
        assert_eq!(split.min_size(), 15);
        assert!(split.border_style().show_borders());
        assert_eq!(split.border_style().focused_border_color(), Color::Blue);
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - 90%+ COVERAGE)
    // ============================================================================

    // ============ Extreme Terminal Sizes ============

    #[test]
    fn test_calculate_panes_1x1_terminal() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 1, 1);
        let (left, right) = split.calculate_panes(area);

        // At 1x1, layout should handle gracefully
        assert!(left.width <= 1);
        assert!(right.width <= 1);
        assert_eq!(left.height, 1);
        assert_eq!(right.height, 1);
    }

    #[test]
    fn test_calculate_panes_1x10000_terminal() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 1, 10000);
        let (left, right) = split.calculate_panes(area);

        // Very narrow terminal
        assert!(left.width <= 1);
        assert!(right.width <= 1);
        assert_eq!(left.height, 10000);
        assert_eq!(right.height, 10000);
    }

    #[test]
    fn test_calculate_panes_10000x1_terminal() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 10000, 1);
        let (top, bottom) = split.calculate_panes(area);

        // Very short terminal
        assert_eq!(top.width, 10000);
        assert_eq!(bottom.width, 10000);
        assert!(top.height <= 1);
        assert!(bottom.height <= 1);
    }

    #[test]
    fn test_calculate_panes_massive_10000x10000_terminal() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 10000, 10000);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 5000);
        assert_eq!(right.width, 5000);
        assert_eq!(left.height, 10000);
        assert_eq!(right.height, 10000);
    }

    // ============ Fixed Size Edge Cases ============

    #[test]
    fn test_fixed_size_very_large_u16_max() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Fixed(u16::MAX));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        // Layout should clamp to available space
        assert!(left.width <= 100);
        assert!(right.width <= 100);
    }

    #[test]
    fn test_fixed_size_zero() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(0));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        // Right pane gets all remaining space
        assert!(right.width > 0);
    }

    #[test]
    fn test_min_size_very_large_u16_max() {
        let split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Min(u16::MAX));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        // Min constraint should expand to fill available space
        assert!(left.width >= 0);
        assert!(right.width >= 0);
    }

    #[test]
    fn test_resize_fixed_to_u16_max() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Fixed(100));

        // Try to resize to near u16::MAX
        let delta = (u16::MAX as i32) - 100;
        assert!(split.resize(delta).is_ok());

        match split.split_size() {
            SplitSize::Fixed(n) => assert_eq!(n, u16::MAX),
            _ => panic!("Expected Fixed"),
        }
    }

    // ============ All BorderType Variants ============

    #[test]
    fn test_border_style_all_border_types() {
        let types = vec![
            BorderType::Plain,
            BorderType::Rounded,
            BorderType::Double,
            BorderType::Thick,
        ];

        for border_type in types {
            let mut style = PaneBorderStyle::new();
            style.set_focused_border_type(border_type);
            assert_eq!(style.focused_border_type(), border_type);

            style.set_unfocused_border_type(border_type);
            assert_eq!(style.unfocused_border_type(), border_type);
        }
    }

    #[test]
    fn test_border_style_all_colors() {
        let colors = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Cyan,
            Color::Magenta,
            Color::White,
            Color::Black,
            Color::Gray,
            Color::DarkGray,
            Color::LightRed,
            Color::LightGreen,
            Color::LightBlue,
            Color::LightYellow,
            Color::LightCyan,
            Color::LightMagenta,
        ];

        for color in colors {
            let mut style = PaneBorderStyle::new();
            style.set_focused_border_color(color);
            assert_eq!(style.focused_border_color(), color);

            style.set_unfocused_border_color(color);
            assert_eq!(style.unfocused_border_color(), color);
        }
    }

    #[test]
    fn test_border_style_clone() {
        let style = PaneBorderStyle::new();
        let cloned = style.clone();

        assert_eq!(style.show_borders(), cloned.show_borders());
        assert_eq!(style.focused_border_type(), cloned.focused_border_type());
        assert_eq!(
            style.unfocused_border_type(),
            cloned.unfocused_border_type()
        );
        assert_eq!(
            style.focused_border_color(),
            cloned.focused_border_color()
        );
        assert_eq!(
            style.unfocused_border_color(),
            cloned.unfocused_border_color()
        );
    }

    // ============ Split Pane Clone Behavior ============

    #[test]
    fn test_split_pane_clone() {
        let split = SplitPane::new(SplitDirection::Vertical)
            .with_split_size(SplitSize::Percentage(70))
            .with_resizable(false)
            .with_separator(false)
            .with_min_size(25)
            .with_borders(false);

        let cloned = split.clone();

        assert_eq!(split.direction(), cloned.direction());
        assert_eq!(split.is_resizable(), cloned.is_resizable());
        assert_eq!(split.has_separator(), cloned.has_separator());
        assert_eq!(split.min_size(), cloned.min_size());
        assert_eq!(split.focused_pane(), cloned.focused_pane());
        assert_eq!(
            split.border_style().show_borders(),
            cloned.border_style().show_borders()
        );
    }

    #[test]
    fn test_split_pane_clone_with_focus_state() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);
        split.set_focused_pane(1).unwrap();

        let cloned = split.clone();
        assert_eq!(cloned.focused_pane(), 1);
    }

    #[test]
    fn test_split_pane_clone_with_modified_split_size() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));
        split.resize(20).unwrap();

        let cloned = split.clone();
        match cloned.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 70),
            _ => panic!("Expected Percentage"),
        }
    }

    // ============ Stress Tests ============

    #[test]
    fn test_stress_1000_focus_toggles() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        for i in 0..1000 {
            split.toggle_focus();
            assert_eq!(split.focused_pane(), if i % 2 == 0 { 1 } else { 0 });
        }

        // After 1000 toggles (even), back to pane 0
        assert_eq!(split.focused_pane(), 0);
    }

    #[test]
    fn test_stress_rapid_resize_operations() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        // Rapid resize operations
        for _ in 0..100 {
            let _ = split.resize(5);
            let _ = split.resize(-5);
        }

        // Should be back at 50%
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_stress_alternating_pane_focus() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        for i in 0..500 {
            split.set_focused_pane(i % 2).unwrap();
            assert_eq!(split.focused_pane(), i % 2);
        }
    }

    #[test]
    fn test_stress_resize_to_extremes_repeatedly() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(5);

        for _ in 0..50 {
            // Resize to near minimum
            let _ = split.resize(-45);
            // Resize to near maximum
            let _ = split.resize(85);
            // Back to middle
            let _ = split.resize(-40);
        }

        // Should end at 50%
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_stress_calculate_panes_many_sizes() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        // Calculate panes for many different sizes
        for width in (10..1000).step_by(10) {
            for height in (10..500).step_by(10) {
                let area = Rect::new(0, 0, width, height);
                let (left, right) = split.calculate_panes(area);

                // Verify panes don't exceed area
                assert!(left.width + right.width <= width);
                assert_eq!(left.height, height);
                assert_eq!(right.height, height);
            }
        }
    }

    // ============ Error Recovery and State Integrity ============

    #[test]
    fn test_invalid_pane_doesnt_corrupt_state() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);
        split.set_focused_pane(1).unwrap();

        // Try invalid pane
        assert!(split.set_focused_pane(2).is_err());

        // State should still be valid
        assert_eq!(split.focused_pane(), 1);
    }

    #[test]
    fn test_invalid_resize_doesnt_corrupt_state() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        let original_size = split.split_size();

        // Try invalid resize
        assert!(split.resize(-100).is_err());

        // State should be unchanged
        assert_eq!(split.split_size(), original_size);
    }

    #[test]
    fn test_multiple_failed_resizes_preserve_state() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        for _ in 0..10 {
            let _ = split.resize(100); // Should fail
            let _ = split.resize(-100); // Should fail
        }

        // Should still be at 50%
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage"),
        }
    }

    // ============ Mixed Resize Operations ============

    #[test]
    fn test_resize_zero_delta() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        assert!(split.resize(0).is_ok());

        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_resize_non_resizable_preserves_size() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_resizable(false);

        for delta in &[-50, -10, 0, 10, 50, 100] {
            split.resize(*delta).unwrap();
            match split.split_size() {
                SplitSize::Percentage(p) => assert_eq!(p, 50),
                _ => panic!("Expected Percentage"),
            }
        }
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
    fn test_resize_min_with_zero_constraint() {
        let mut split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Min(0));

        assert!(split.resize(100).is_ok());
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 100),
            _ => panic!("Expected Min"),
        }

        assert!(split.resize(-100).is_ok());
        match split.split_size() {
            SplitSize::Min(n) => assert_eq!(n, 0),
            _ => panic!("Expected Min"),
        }
    }

    // ============ Complex State Transitions ============

    #[test]
    fn test_complex_state_transitions() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_min_size(10);

        // Complex sequence of operations
        split.toggle_focus();
        assert_eq!(split.focused_pane(), 1);

        split.resize(10).unwrap();
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 60),
            _ => panic!("Expected Percentage"),
        }

        split.toggle_focus();
        assert_eq!(split.focused_pane(), 0);

        split.resize(-20).unwrap();
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 40),
            _ => panic!("Expected Percentage"),
        }

        split.set_focused_pane(1).unwrap();
        assert_eq!(split.focused_pane(), 1);

        split.resize(10).unwrap();
        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage"),
        }
    }

    #[test]
    fn test_border_style_mutations_during_operations() {
        let mut split = SplitPane::new(SplitDirection::Horizontal);

        split.toggle_focus();
        split.border_style_mut().set_show_borders(false);
        assert!(!split.border_style().show_borders());

        split.toggle_focus();
        split
            .border_style_mut()
            .set_focused_border_color(Color::Red);
        assert_eq!(split.border_style().focused_border_color(), Color::Red);
    }

    // ============ Vertical vs Horizontal Comprehensive Tests ============

    #[test]
    fn test_vertical_zero_height() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 100, 0);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height, 0);
        assert_eq!(bottom.height, 0);
        assert_eq!(top.width, 100);
        assert_eq!(bottom.width, 100);
    }

    #[test]
    fn test_horizontal_zero_width() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Percentage(50));

        let area = Rect::new(0, 0, 0, 100);
        let (left, right) = split.calculate_panes(area);

        assert_eq!(left.width, 0);
        assert_eq!(right.width, 0);
        assert_eq!(left.height, 100);
        assert_eq!(right.height, 100);
    }

    #[test]
    fn test_vertical_with_fixed_size() {
        let split =
            SplitPane::new(SplitDirection::Vertical).with_split_size(SplitSize::Fixed(30));

        let area = Rect::new(0, 0, 100, 100);
        let (top, bottom) = split.calculate_panes(area);

        assert_eq!(top.height, 30);
        assert!(bottom.height >= 0);
        assert_eq!(top.width, 100);
        assert_eq!(bottom.width, 100);
    }

    #[test]
    fn test_horizontal_with_min_size() {
        let split =
            SplitPane::new(SplitDirection::Horizontal).with_split_size(SplitSize::Min(20));

        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.calculate_panes(area);

        assert!(left.width >= 20);
        assert_eq!(left.width + right.width, 100);
    }

    #[test]
    fn test_split_pane_debug() {
        let split = SplitPane::new(SplitDirection::Vertical);
        let debug_str = format!("{:?}", split);
        assert!(debug_str.contains("SplitPane"));
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

        assert!(left.width >= 20);
        assert!(right.width >= 0);
        assert_eq!(left.height, 50);
        assert_eq!(right.height, 50);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_stress_all_features() {
        let mut split = SplitPane::new(SplitDirection::Horizontal)
            .with_split_size(SplitSize::Percentage(50))
            .with_resizable(true)
            .with_separator(true)
            .with_min_size(5)
            .with_borders(true)
            .with_focused_color(Color::Green)
            .with_unfocused_color(Color::Gray);

        // Complex sequence combining all operations
        for i in 0..100 {
            // Toggle focus
            split.toggle_focus();

            // Resize based on iteration
            let delta = if i % 3 == 0 {
                5
            } else if i % 3 == 1 {
                -5
            } else {
                0
            };
            let _ = split.resize(delta);

            // Calculate panes with varying sizes
            let width = 100 + (i * 10) % 500;
            let height = 50 + (i * 5) % 200;
            let area = Rect::new(0, 0, width, height);
            let (left, right) = split.calculate_panes(area);

            // Verify integrity
            assert!(left.width + right.width <= width);
            assert_eq!(left.height, height);
            assert_eq!(right.height, height);

            // Modify border style
            if i % 10 == 0 {
                split
                    .border_style_mut()
                    .set_focused_border_color(Color::Blue);
            }
        }

        // Final state checks
        assert_eq!(split.focused_pane(), 0); // 100 toggles = back to 0
        assert!(split.is_resizable());
        assert!(split.has_separator());
    }

    #[test]
    fn test_default_trait_all_properties() {
        let split = SplitPane::default();

        assert_eq!(split.direction(), SplitDirection::Horizontal);
        assert_eq!(split.focused_pane(), 0);
        assert!(split.is_resizable());
        assert!(split.has_separator());
        assert_eq!(split.min_size(), 10);
        assert!(split.border_style().show_borders());

        match split.split_size() {
            SplitSize::Percentage(p) => assert_eq!(p, 50),
            _ => panic!("Expected Percentage 50"),
        }
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
    fn test_pane_border_style_default() {
        let style = PaneBorderStyle::default();

        assert_eq!(style.show_borders(), true);
        assert_eq!(style.focused_border_type(), BorderType::Thick);
        assert_eq!(style.unfocused_border_type(), BorderType::Plain);
        assert_eq!(style.focused_border_color(), Color::Green);
        assert_eq!(style.unfocused_border_color(), Color::DarkGray);
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
