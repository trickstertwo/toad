use crate::ui::{atoms::block::Block as AtomBlock, theme::ToadTheme};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::BorderType,
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
            let block1 = AtomBlock::new()
                .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
                .to_ratatui();
            let inner1 = block1.inner(pane1);
            frame.render_widget(block1, pane1);
            left(frame, inner1);
            right(frame, pane2);
        } else {
            let block2 = AtomBlock::new()
                .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
                .to_ratatui();
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
