//! Context menu widget for action selection
//!
//! Provides right-click or keybind-activated menus showing available actions.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::ContextMenu;
//!
//! let menu = ContextMenu::new()
//!     .add_item("Copy", "copy", Some("Ctrl+C"))
//!     .add_item("Paste", "paste", Some("Ctrl+V"))
//!     .add_separator()
//!     .add_item("Delete", "delete", Some("Del"));
//!
//! assert_eq!(menu.item_count(), 4);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Menu item in a context menu
///
/// Represents an actionable item or separator in a menu.
///
/// # Examples
///
/// ```
/// use toad::widgets::MenuItem;
///
/// let item = MenuItem::new("Copy", "copy")
///     .with_keybind("Ctrl+C");
///
/// assert_eq!(item.label(), "Copy");
/// assert_eq!(item.action(), "copy");
/// assert_eq!(item.keybind(), Some("Ctrl+C"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItem {
    /// Display label
    label: String,
    /// Action identifier
    action: String,
    /// Optional keybind hint
    keybind: Option<String>,
    /// Whether item is enabled
    enabled: bool,
    /// Whether this is a separator
    separator: bool,
}

impl MenuItem {
    /// Create a new menu item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let item = MenuItem::new("Copy", "copy");
    /// assert_eq!(item.label(), "Copy");
    /// assert_eq!(item.action(), "copy");
    /// ```
    pub fn new(label: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: action.into(),
            keybind: None,
            enabled: true,
            separator: false,
        }
    }

    /// Create a separator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let sep = MenuItem::separator();
    /// assert!(sep.is_separator());
    /// ```
    pub fn separator() -> Self {
        Self {
            label: String::new(),
            action: String::new(),
            keybind: None,
            enabled: false,
            separator: true,
        }
    }

    /// Add keybind hint
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let item = MenuItem::new("Copy", "copy")
    ///     .with_keybind("Ctrl+C");
    /// assert_eq!(item.keybind(), Some("Ctrl+C"));
    /// ```
    pub fn with_keybind(mut self, keybind: impl Into<String>) -> Self {
        self.keybind = Some(keybind.into());
        self
    }

    /// Set enabled state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let item = MenuItem::new("Paste", "paste")
    ///     .with_enabled(false);
    /// assert!(!item.is_enabled());
    /// ```
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get action ID
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Get keybind hint
    pub fn keybind(&self) -> Option<&str> {
        self.keybind.as_deref()
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if separator
    pub fn is_separator(&self) -> bool {
        self.separator
    }

    /// Set enabled state (mutable)
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Context menu widget
///
/// Displays a popup menu with selectable items and keyboard navigation.
///
/// # Examples
///
/// ```
/// use toad::widgets::ContextMenu;
///
/// let mut menu = ContextMenu::new()
///     .add_item("Open", "open", Some("Enter"))
///     .add_item("Delete", "delete", Some("Del"))
///     .at_position(10, 5);
///
/// menu.show();
/// assert!(menu.is_visible());
///
/// menu.next();
/// assert_eq!(menu.selected_action(), Some("delete"));
/// ```
#[derive(Debug, Clone)]
pub struct ContextMenu {
    /// Menu items
    items: Vec<MenuItem>,
    /// Menu position (x, y)
    position: (u16, u16),
    /// Selected item index
    selected: usize,
    /// Whether menu is visible
    visible: bool,
    /// Maximum menu width
    max_width: u16,
    /// Title
    title: Option<String>,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMenu {
    /// Create a new context menu
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new();
    /// assert_eq!(menu.item_count(), 0);
    /// assert!(!menu.is_visible());
    /// ```
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            position: (0, 0),
            selected: 0,
            visible: false,
            max_width: 40,
            title: None,
        }
    }

    /// Add a menu item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new()
    ///     .add_item("Copy", "copy", Some("Ctrl+C"));
    ///
    /// assert_eq!(menu.item_count(), 1);
    /// ```
    pub fn add_item(
        mut self,
        label: impl Into<String>,
        action: impl Into<String>,
        keybind: Option<impl Into<String>>,
    ) -> Self {
        let mut item = MenuItem::new(label, action);
        if let Some(kb) = keybind {
            item = item.with_keybind(kb);
        }
        self.items.push(item);
        self
    }

    /// Add a separator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new()
    ///     .add_item("Copy", "copy", None::<&str>)
    ///     .add_separator()
    ///     .add_item("Delete", "delete", None::<&str>);
    ///
    /// assert_eq!(menu.item_count(), 3);
    /// ```
    pub fn add_separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }

    /// Set menu position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new()
    ///     .at_position(10, 5);
    /// ```
    pub fn at_position(mut self, x: u16, y: u16) -> Self {
        self.position = (x, y);
        self
    }

    /// Set maximum width
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new()
    ///     .with_max_width(30);
    /// ```
    pub fn with_max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new()
    ///     .with_title("File Actions");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Show the menu
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let mut menu = ContextMenu::new();
    /// menu.show();
    /// assert!(menu.is_visible());
    /// ```
    pub fn show(&mut self) {
        self.visible = true;
        self.selected = self.find_next_enabled(0).unwrap_or(0);
    }

    /// Hide the menu
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let mut menu = ContextMenu::new();
    /// menu.show();
    /// menu.hide();
    /// assert!(!menu.is_visible());
    /// ```
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        if self.visible {
            self.hide();
        } else {
            self.show();
        }
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Move selection down
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let mut menu = ContextMenu::new()
    ///     .add_item("First", "first", None::<&str>)
    ///     .add_item("Second", "second", None::<&str>);
    ///
    /// menu.next();
    /// assert_eq!(menu.selected_index(), 1);
    /// ```
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let start = (self.selected + 1) % self.items.len();
        self.selected = self.find_next_enabled(start).unwrap_or(self.selected);
    }

    /// Move selection up
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let mut menu = ContextMenu::new()
    ///     .add_item("First", "first", None::<&str>)
    ///     .add_item("Second", "second", None::<&str>);
    ///
    /// menu.next();
    /// menu.previous();
    /// assert_eq!(menu.selected_index(), 0);
    /// ```
    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let start = if self.selected == 0 {
            self.items.len() - 1
        } else {
            self.selected - 1
        };

        self.selected = self.find_previous_enabled(start).unwrap_or(self.selected);
    }

    /// Get selected item index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get selected item
    pub fn selected_item(&self) -> Option<&MenuItem> {
        self.items.get(self.selected)
    }

    /// Get selected action ID
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new()
    ///     .add_item("Copy", "copy", None::<&str>);
    ///
    /// assert_eq!(menu.selected_action(), Some("copy"));
    /// ```
    pub fn selected_action(&self) -> Option<&str> {
        self.selected_item()
            .filter(|item| !item.is_separator())
            .map(|item| item.action())
    }

    /// Get all items
    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    /// Add item (mutable)
    pub fn push_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = 0;
    }

    /// Set position
    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position = (x, y);
    }

    /// Get position
    pub fn position(&self) -> (u16, u16) {
        self.position
    }

    /// Find next enabled item starting from index
    fn find_next_enabled(&self, start: usize) -> Option<usize> {
        for i in 0..self.items.len() {
            let idx = (start + i) % self.items.len();
            if let Some(item) = self.items.get(idx)
                && item.is_enabled() && !item.is_separator()
            {
                return Some(idx);
            }
        }
        None
    }

    /// Find previous enabled item starting from index
    fn find_previous_enabled(&self, start: usize) -> Option<usize> {
        for i in 0..self.items.len() {
            let idx = if start >= i {
                start - i
            } else {
                self.items.len() + start - i
            };

            if let Some(item) = self.items.get(idx)
                && item.is_enabled() && !item.is_separator()
            {
                return Some(idx);
            }
        }
        None
    }

    /// Calculate menu dimensions
    fn calculate_size(&self) -> (u16, u16) {
        let height = self.items.len() as u16 + 2; // +2 for borders

        // Calculate max width from items
        let mut max_label_width = 0;
        let mut max_keybind_width = 0;

        for item in &self.items {
            if !item.is_separator() {
                max_label_width = max_label_width.max(item.label().len());
                if let Some(kb) = item.keybind() {
                    max_keybind_width = max_keybind_width.max(kb.len());
                }
            }
        }

        let width = if max_keybind_width > 0 {
            (max_label_width + max_keybind_width + 4) as u16 // 4 for spacing + borders
        } else {
            (max_label_width + 4) as u16
        };

        (width.min(self.max_width), height)
    }

    /// Render menu lines
    fn render_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            if item.is_separator() {
                lines.push(Line::from(vec![Span::raw("─".repeat(self.max_width as usize - 2))]));
            } else {
                let is_selected = i == self.selected;
                let mut spans = Vec::new();

                // Selection indicator
                if is_selected {
                    spans.push(Span::styled(
                        "> ".to_string(),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw("  ".to_string()));
                }

                // Label
                let label_style = if !item.is_enabled() {
                    Style::default().fg(Color::DarkGray)
                } else if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                spans.push(Span::styled(item.label().to_string(), label_style));

                // Keybind hint
                if let Some(keybind) = item.keybind() {
                    let padding = self.max_width as usize - item.label().len() - keybind.len() - 4;
                    spans.push(Span::raw(" ".repeat(padding)));
                    spans.push(Span::styled(
                        keybind.to_string(),
                        Style::default().fg(Color::DarkGray),
                    ));
                }

                lines.push(Line::from(spans));
            }
        }

        lines
    }
}

impl Widget for &ContextMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        let (width, height) = self.calculate_size();
        let (x, y) = self.position;

        // Calculate render area
        let menu_area = Rect {
            x: x.min(area.width.saturating_sub(width)),
            y: y.min(area.height.saturating_sub(height)),
            width: width.min(area.width),
            height: height.min(area.height),
        };

        let mut block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));

        if let Some(title) = &self.title {
            block = block.title(title.clone());
        }

        let inner = block.inner(menu_area);
        block.render(menu_area, buf);

        let lines = self.render_lines();
        for (i, line) in lines.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }
            let y = inner.y + i as u16;
            buf.set_line(inner.x, y, line, inner.width);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_new() {
        let item = MenuItem::new("Copy", "copy");
        assert_eq!(item.label(), "Copy");
        assert_eq!(item.action(), "copy");
        assert_eq!(item.keybind(), None);
        assert!(item.is_enabled());
        assert!(!item.is_separator());
    }

    #[test]
    fn test_menu_item_separator() {
        let sep = MenuItem::separator();
        assert!(sep.is_separator());
        assert!(!sep.is_enabled());
    }

    #[test]
    fn test_menu_item_with_keybind() {
        let item = MenuItem::new("Copy", "copy")
            .with_keybind("Ctrl+C");
        assert_eq!(item.keybind(), Some("Ctrl+C"));
    }

    #[test]
    fn test_menu_item_with_enabled() {
        let item = MenuItem::new("Paste", "paste")
            .with_enabled(false);
        assert!(!item.is_enabled());
    }

    #[test]
    fn test_menu_item_set_enabled() {
        let mut item = MenuItem::new("Copy", "copy");
        assert!(item.is_enabled());
        item.set_enabled(false);
        assert!(!item.is_enabled());
    }

    #[test]
    fn test_context_menu_new() {
        let menu = ContextMenu::new();
        assert_eq!(menu.item_count(), 0);
        assert!(!menu.is_visible());
        assert_eq!(menu.selected_index(), 0);
    }

    #[test]
    fn test_context_menu_default() {
        let menu = ContextMenu::default();
        assert_eq!(menu.item_count(), 0);
    }

    #[test]
    fn test_context_menu_add_item() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy", Some("Ctrl+C"))
            .add_item("Paste", "paste", None::<&str>);

        assert_eq!(menu.item_count(), 2);
        assert_eq!(menu.items()[0].label(), "Copy");
        assert_eq!(menu.items()[1].label(), "Paste");
    }

    #[test]
    fn test_context_menu_add_separator() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy", None::<&str>)
            .add_separator()
            .add_item("Delete", "delete", None::<&str>);

        assert_eq!(menu.item_count(), 3);
        assert!(menu.items()[1].is_separator());
    }

    #[test]
    fn test_context_menu_at_position() {
        let menu = ContextMenu::new()
            .at_position(10, 5);
        assert_eq!(menu.position(), (10, 5));
    }

    #[test]
    fn test_context_menu_with_max_width() {
        let menu = ContextMenu::new()
            .with_max_width(30);
        assert_eq!(menu.max_width, 30);
    }

    #[test]
    fn test_context_menu_with_title() {
        let menu = ContextMenu::new()
            .with_title("File Actions");
        assert_eq!(menu.title, Some("File Actions".to_string()));
    }

    #[test]
    fn test_context_menu_show_hide() {
        let mut menu = ContextMenu::new();
        assert!(!menu.is_visible());

        menu.show();
        assert!(menu.is_visible());

        menu.hide();
        assert!(!menu.is_visible());
    }

    #[test]
    fn test_context_menu_toggle() {
        let mut menu = ContextMenu::new();
        menu.toggle();
        assert!(menu.is_visible());
        menu.toggle();
        assert!(!menu.is_visible());
    }

    #[test]
    fn test_context_menu_navigation() {
        let mut menu = ContextMenu::new()
            .add_item("First", "first", None::<&str>)
            .add_item("Second", "second", None::<&str>)
            .add_item("Third", "third", None::<&str>);

        assert_eq!(menu.selected_index(), 0);

        menu.next();
        assert_eq!(menu.selected_index(), 1);

        menu.next();
        assert_eq!(menu.selected_index(), 2);

        menu.next();
        assert_eq!(menu.selected_index(), 0); // Wrap around

        menu.previous();
        assert_eq!(menu.selected_index(), 2); // Wrap around

        menu.previous();
        assert_eq!(menu.selected_index(), 1);
    }

    #[test]
    fn test_context_menu_skip_separator() {
        let mut menu = ContextMenu::new()
            .add_item("First", "first", None::<&str>)
            .add_separator()
            .add_item("Third", "third", None::<&str>);

        menu.show();
        assert_eq!(menu.selected_index(), 0);

        menu.next();
        assert_eq!(menu.selected_index(), 2); // Skip separator
    }

    #[test]
    fn test_context_menu_skip_disabled() {
        let mut menu = ContextMenu::new()
            .add_item("First", "first", None::<&str>)
            .add_item("Second", "second", None::<&str>)
            .add_item("Third", "third", None::<&str>);

        menu.items[1].set_enabled(false);

        menu.show();
        assert_eq!(menu.selected_index(), 0);

        menu.next();
        assert_eq!(menu.selected_index(), 2); // Skip disabled
    }

    #[test]
    fn test_context_menu_selected_action() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy", None::<&str>)
            .add_item("Paste", "paste", None::<&str>);

        assert_eq!(menu.selected_action(), Some("copy"));
    }

    #[test]
    fn test_context_menu_selected_item() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy", None::<&str>);

        let item = menu.selected_item();
        assert!(item.is_some());
        assert_eq!(item.unwrap().label(), "Copy");
    }

    #[test]
    fn test_context_menu_push_item() {
        let mut menu = ContextMenu::new();
        assert_eq!(menu.item_count(), 0);

        menu.push_item(MenuItem::new("Test", "test"));
        assert_eq!(menu.item_count(), 1);
    }

    #[test]
    fn test_context_menu_clear() {
        let mut menu = ContextMenu::new()
            .add_item("First", "first", None::<&str>)
            .add_item("Second", "second", None::<&str>);

        assert_eq!(menu.item_count(), 2);
        menu.clear();
        assert_eq!(menu.item_count(), 0);
        assert_eq!(menu.selected_index(), 0);
    }

    #[test]
    fn test_context_menu_set_position() {
        let mut menu = ContextMenu::new();
        menu.set_position(20, 10);
        assert_eq!(menu.position(), (20, 10));
    }

    #[test]
    fn test_context_menu_calculate_size() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy", Some("Ctrl+C"))
            .add_item("Paste", "paste", Some("Ctrl+V"));

        let (width, height) = menu.calculate_size();
        assert!(width > 0);
        assert_eq!(height, 4); // 2 items + 2 borders
    }

    #[test]
    fn test_context_menu_render_lines() {
        let menu = ContextMenu::new()
            .add_item("Copy", "copy", None::<&str>)
            .add_separator()
            .add_item("Delete", "delete", None::<&str>);

        let lines = menu.render_lines();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_context_menu_builder_pattern() {
        let menu = ContextMenu::new()
            .add_item("Open", "open", Some("Enter"))
            .add_item("Delete", "delete", Some("Del"))
            .at_position(10, 5)
            .with_max_width(30)
            .with_title("Actions");

        assert_eq!(menu.item_count(), 2);
        assert_eq!(menu.position(), (10, 5));
        assert_eq!(menu.max_width, 30);
        assert_eq!(menu.title, Some("Actions".to_string()));
    }
}
