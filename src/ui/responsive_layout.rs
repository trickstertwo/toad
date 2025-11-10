//! Responsive layout management for adaptive TUI layouts
//!
//! Automatically adjusts layouts based on terminal dimensions,
//! providing breakpoints and adaptive column/row configurations.
//!
//! # Examples
//!
//! ```
//! use toad::ui::responsive_layout::{ResponsiveLayout, ScreenSize};
//!
//! let layout = ResponsiveLayout::new(80, 24);
//! assert_eq!(layout.screen_size(), ScreenSize::Medium);
//! ```

use ratatui::layout::{Constraint, Layout, Rect};

/// Screen size categories based on terminal dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScreenSize {
    /// Tiny: < 40 columns or < 10 rows
    Tiny,
    /// Small: 40-79 columns or 10-19 rows
    Small,
    /// Medium: 80-119 columns, 20-39 rows (standard)
    Medium,
    /// Large: 120-159 columns, 40-59 rows
    Large,
    /// ExtraLarge: >= 160 columns, >= 60 rows
    ExtraLarge,
}

impl ScreenSize {
    /// Determine screen size from dimensions
    ///
    /// Uses the *smaller* dimension to ensure content fits
    pub fn from_dimensions(width: u16, height: u16) -> Self {
        // Use the limiting factor (smallest dimension relative to thresholds)
        if width < 40 || height < 10 {
            ScreenSize::Tiny
        } else if width < 80 || height < 20 {
            ScreenSize::Small
        } else if width < 120 || height < 40 {
            ScreenSize::Medium
        } else if width < 160 || height < 60 {
            ScreenSize::Large
        } else {
            ScreenSize::ExtraLarge
        }
    }

    /// Check if screen can fit multiple columns
    pub fn supports_multi_column(&self) -> bool {
        matches!(
            self,
            ScreenSize::Medium | ScreenSize::Large | ScreenSize::ExtraLarge
        )
    }

    /// Check if screen can fit three columns
    pub fn supports_three_column(&self) -> bool {
        matches!(self, ScreenSize::Large | ScreenSize::ExtraLarge)
    }

    /// Get recommended column count
    pub fn recommended_columns(&self) -> usize {
        match self {
            ScreenSize::Tiny | ScreenSize::Small => 1,
            ScreenSize::Medium => 2,
            ScreenSize::Large => 3,
            ScreenSize::ExtraLarge => 4,
        }
    }

    /// Get recommended sidebar width percentage
    pub fn sidebar_width_percent(&self) -> u16 {
        match self {
            ScreenSize::Tiny => 100, // No sidebar, full width
            ScreenSize::Small => 40,
            ScreenSize::Medium => 30,
            ScreenSize::Large => 25,
            ScreenSize::ExtraLarge => 20,
        }
    }

    /// Check if sidebar should be shown
    pub fn show_sidebar(&self) -> bool {
        !matches!(self, ScreenSize::Tiny)
    }

    /// Get compact mode recommendation
    pub fn should_use_compact(&self) -> bool {
        matches!(self, ScreenSize::Tiny | ScreenSize::Small)
    }
}

/// Responsive layout manager
pub struct ResponsiveLayout {
    /// Current terminal width
    width: u16,
    /// Current terminal height
    height: u16,
    /// Calculated screen size
    screen_size: ScreenSize,
    /// Whether to force compact mode
    force_compact: bool,
}

impl ResponsiveLayout {
    /// Create a new responsive layout
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::responsive_layout::ResponsiveLayout;
    ///
    /// let layout = ResponsiveLayout::new(100, 30);
    /// ```
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            screen_size: ScreenSize::from_dimensions(width, height),
            force_compact: false,
        }
    }

    /// Update dimensions
    pub fn update_dimensions(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.screen_size = ScreenSize::from_dimensions(width, height);
    }

    /// Get current screen size
    pub fn screen_size(&self) -> ScreenSize {
        self.screen_size
    }

    /// Get width
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Force compact mode
    pub fn set_force_compact(&mut self, compact: bool) {
        self.force_compact = compact;
    }

    /// Check if should use compact mode
    pub fn is_compact(&self) -> bool {
        self.force_compact || self.screen_size.should_use_compact()
    }

    /// Create adaptive split (vertical for wide, horizontal for narrow)
    pub fn adaptive_split(&self, area: Rect, ratio: (u16, u16)) -> Vec<Rect> {
        let (first, second) = ratio;

        let total = first + second;

        if self.width >= 80 {
            // Wide enough for side-by-side
            Layout::horizontal([
                Constraint::Ratio(first as u32, total as u32),
                Constraint::Ratio(second as u32, total as u32),
            ])
            .split(area)
            .to_vec()
        } else {
            // Stack vertically for narrow
            Layout::vertical([
                Constraint::Ratio(first as u32, total as u32),
                Constraint::Ratio(second as u32, total as u32),
            ])
            .split(area)
            .to_vec()
        }
    }

    /// Create responsive sidebar layout
    pub fn sidebar_layout(&self, area: Rect) -> (Rect, Rect) {
        if !self.screen_size.show_sidebar() {
            // No sidebar on tiny screens
            return (area, Rect::default());
        }

        let sidebar_percent = self.screen_size.sidebar_width_percent();
        let chunks = Layout::horizontal([
            Constraint::Percentage(sidebar_percent),
            Constraint::Percentage(100 - sidebar_percent),
        ])
        .split(area);

        (chunks[0], chunks[1])
    }

    /// Create responsive column layout
    pub fn column_layout(&self, area: Rect) -> Vec<Rect> {
        let cols = if self.is_compact() {
            1
        } else {
            self.screen_size.recommended_columns()
        };

        match cols {
            1 => vec![area],
            2 => Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area)
                .to_vec(),
            3 => Layout::horizontal([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area)
            .to_vec(),
            4 => Layout::horizontal([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area)
            .to_vec(),
            _ => vec![area],
        }
    }

    /// Create responsive three-pane layout (sidebar, main, preview)
    pub fn three_pane_layout(&self, area: Rect) -> (Rect, Rect, Option<Rect>) {
        if self.is_compact() {
            // Compact: main only
            return (Rect::default(), area, None);
        }

        if !self.screen_size.supports_three_column() {
            // Two pane: sidebar + main
            let (sidebar, main) = self.sidebar_layout(area);
            return (sidebar, main, None);
        }

        // Three pane: sidebar + main + preview
        let chunks = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ])
        .split(area);

        (chunks[0], chunks[1], Some(chunks[2]))
    }

    /// Get adaptive padding
    pub fn padding(&self) -> u16 {
        match self.screen_size {
            ScreenSize::Tiny => 0,
            ScreenSize::Small => 1,
            _ => 2,
        }
    }

    /// Get adaptive margin
    pub fn margin(&self) -> u16 {
        if self.is_compact() {
            0
        } else {
            1
        }
    }

    /// Check if should show help footer
    pub fn show_help_footer(&self) -> bool {
        self.height >= 15
    }

    /// Check if should show status bar
    pub fn show_status_bar(&self) -> bool {
        self.height >= 10
    }

    /// Get maximum visible items for lists
    pub fn max_list_items(&self) -> usize {
        let available_height = self.height.saturating_sub(5); // Reserve for borders/header
        available_height.max(5) as usize
    }

    /// Get recommended truncation length for text
    pub fn truncation_length(&self) -> usize {
        match self.screen_size {
            ScreenSize::Tiny => 20,
            ScreenSize::Small => 40,
            ScreenSize::Medium => 60,
            ScreenSize::Large => 80,
            ScreenSize::ExtraLarge => 100,
        }
    }
}

impl Default for ResponsiveLayout {
    fn default() -> Self {
        Self::new(80, 24) // Standard terminal size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_size_detection() {
        assert_eq!(ScreenSize::from_dimensions(30, 10), ScreenSize::Tiny);
        assert_eq!(ScreenSize::from_dimensions(60, 15), ScreenSize::Small);
        assert_eq!(ScreenSize::from_dimensions(100, 30), ScreenSize::Medium);
        assert_eq!(ScreenSize::from_dimensions(140, 50), ScreenSize::Large);
        assert_eq!(ScreenSize::from_dimensions(180, 70), ScreenSize::ExtraLarge);
    }

    #[test]
    fn test_screen_size_limiting_factor() {
        // Width is limiting
        assert_eq!(ScreenSize::from_dimensions(30, 100), ScreenSize::Tiny);
        // Height is limiting
        assert_eq!(ScreenSize::from_dimensions(200, 15), ScreenSize::Small);
    }

    #[test]
    fn test_recommended_columns() {
        assert_eq!(ScreenSize::Tiny.recommended_columns(), 1);
        assert_eq!(ScreenSize::Small.recommended_columns(), 1);
        assert_eq!(ScreenSize::Medium.recommended_columns(), 2);
        assert_eq!(ScreenSize::Large.recommended_columns(), 3);
        assert_eq!(ScreenSize::ExtraLarge.recommended_columns(), 4);
    }

    #[test]
    fn test_sidebar_support() {
        assert!(!ScreenSize::Tiny.show_sidebar());
        assert!(ScreenSize::Small.show_sidebar());
        assert!(ScreenSize::Medium.show_sidebar());
    }

    #[test]
    fn test_responsive_layout_new() {
        let layout = ResponsiveLayout::new(100, 30);
        assert_eq!(layout.screen_size(), ScreenSize::Medium);
        assert_eq!(layout.width(), 100);
        assert_eq!(layout.height(), 30);
    }

    #[test]
    fn test_update_dimensions() {
        let mut layout = ResponsiveLayout::new(80, 24);
        assert_eq!(layout.screen_size(), ScreenSize::Medium);

        layout.update_dimensions(40, 15);
        assert_eq!(layout.screen_size(), ScreenSize::Small);
    }

    #[test]
    fn test_compact_mode() {
        let mut layout = ResponsiveLayout::new(100, 30);
        assert!(!layout.is_compact());

        layout.set_force_compact(true);
        assert!(layout.is_compact());
    }

    #[test]
    fn test_adaptive_split() {
        let layout = ResponsiveLayout::new(100, 30);
        let area = Rect::new(0, 0, 100, 30);
        let chunks = layout.adaptive_split(area, (1, 1));

        // Wide enough for horizontal split
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].width > 0);
        assert!(chunks[1].width > 0);
    }

    #[test]
    fn test_column_layout() {
        let layout = ResponsiveLayout::new(140, 50);
        let area = Rect::new(0, 0, 140, 50);
        let columns = layout.column_layout(area);

        // Large screen gets 3 columns
        assert_eq!(columns.len(), 3);
    }

    #[test]
    fn test_sidebar_layout() {
        let layout = ResponsiveLayout::new(100, 30);
        let area = Rect::new(0, 0, 100, 30);
        let (sidebar, main) = layout.sidebar_layout(area);

        assert!(sidebar.width > 0);
        assert!(main.width > 0);
        assert_eq!(sidebar.width + main.width, area.width);
    }

    #[test]
    fn test_three_pane_layout() {
        let layout = ResponsiveLayout::new(140, 50);
        let area = Rect::new(0, 0, 140, 50);
        let (sidebar, main, preview) = layout.three_pane_layout(area);

        // Large screen gets three panes
        assert!(sidebar.width > 0);
        assert!(main.width > 0);
        assert!(preview.is_some());
    }

    #[test]
    fn test_compact_three_pane() {
        let mut layout = ResponsiveLayout::new(140, 50);
        layout.set_force_compact(true);

        let area = Rect::new(0, 0, 140, 50);
        let (sidebar, main, preview) = layout.three_pane_layout(area);

        // Compact mode: main only
        assert_eq!(sidebar.width, 0);
        assert_eq!(main.width, area.width);
        assert!(preview.is_none());
    }

    #[test]
    fn test_show_components() {
        let tiny = ResponsiveLayout::new(30, 8);
        assert!(!tiny.show_help_footer());
        assert!(!tiny.show_status_bar());

        let medium = ResponsiveLayout::new(100, 30);
        assert!(medium.show_help_footer());
        assert!(medium.show_status_bar());
    }

    #[test]
    fn test_truncation_length() {
        assert_eq!(ResponsiveLayout::new(30, 10).truncation_length(), 20);
        assert_eq!(ResponsiveLayout::new(60, 15).truncation_length(), 40);
        assert_eq!(ResponsiveLayout::new(100, 30).truncation_length(), 60);
    }

    #[test]
    fn test_max_list_items() {
        let layout = ResponsiveLayout::new(80, 24);
        let max_items = layout.max_list_items();

        assert!(max_items >= 5);
        assert!(max_items <= 24);
    }
}
