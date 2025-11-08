/// Context menu widget
///
/// Right-click or keybind menu for showing contextual actions
///
/// # Examples
///
/// ```
/// use toad::widgets::{ContextMenu, MenuItem};
///
/// let mut menu = ContextMenu::new();
/// menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
/// assert_eq!(menu.item_count(), 1);
/// ```

use crate::theme::ToadTheme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use serde::{Deserialize, Serialize};

/// Menu item type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuItem {
    /// Action item with label and optional shortcut
    Action {
        label: String,
        shortcut: Option<String>,
        icon: Option<String>,
        enabled: bool,
    },
    /// Separator line
    Separator,
}

impl MenuItem {
    /// Create a new action item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let item = MenuItem::action("Copy", "Ctrl+C");
    /// assert!(item.is_action());
    /// assert!(item.is_enabled());
    /// ```
    pub fn action(label: impl Into<String>, shortcut: impl Into<String>) -> Self {
        Self::Action {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            icon: None,
            enabled: true,
        }
    }

    /// Create an action without shortcut
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let item = MenuItem::simple("Delete");
    /// assert!(item.is_action());
    /// ```
    pub fn simple(label: impl Into<String>) -> Self {
        Self::Action {
            label: label.into(),
            shortcut: None,
            icon: None,
            enabled: true,
        }
    }

    /// Create a separator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MenuItem;
    ///
    /// let item = MenuItem::separator();
    /// assert!(item.is_separator());
    /// ```
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Set icon
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        if let Self::Action { icon: icon_field, .. } = &mut self {
            *icon_field = Some(icon.into());
        }
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        if let Self::Action { enabled: enabled_field, .. } = &mut self {
            *enabled_field = enabled;
        }
        self
    }

    /// Check if this is an action item
    pub fn is_action(&self) -> bool {
        matches!(self, Self::Action { .. })
    }

    /// Check if this is a separator
    pub fn is_separator(&self) -> bool {
        matches!(self, Self::Separator)
    }

    /// Check if item is enabled
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Action { enabled, .. } => *enabled,
            Self::Separator => false,
        }
    }

    /// Get label (if action)
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Action { label, .. } => Some(label),
            Self::Separator => None,
        }
    }

    /// Get shortcut (if action)
    pub fn shortcut(&self) -> Option<&str> {
        match self {
            Self::Action { shortcut, .. } => shortcut.as_deref(),
            Self::Separator => None,
        }
    }
}

/// Context menu widget
#[derive(Debug, Clone)]
pub struct ContextMenu {
    /// Menu items
    items: Vec<MenuItem>,
    /// Selected item index
    selected: Option<usize>,
    /// Menu title
    title: Option<String>,
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
    /// ```
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selected: None,
            title: None,
        }
    }

    /// Set menu title
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::ContextMenu;
    ///
    /// let menu = ContextMenu::new().title("Actions");
    /// ```
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a menu item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ContextMenu, MenuItem};
    ///
    /// let mut menu = ContextMenu::new();
    /// menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
    /// assert_eq!(menu.item_count(), 1);
    /// ```
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
        if self.selected.is_none() && !self.items.is_empty() {
            self.select_first_enabled();
        }
    }

    /// Add multiple items
    pub fn add_items(&mut self, items: Vec<MenuItem>) {
        for item in items {
            self.add_item(item);
        }
    }

    /// Get all items
    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    /// Get number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Check if menu is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get selected item index
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Get selected item
    pub fn selected_item(&self) -> Option<&MenuItem> {
        self.selected.and_then(|idx| self.items.get(idx))
    }

    /// Select next item (skip separators and disabled items)
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let start = self.selected.map(|i| i + 1).unwrap_or(0);
        for offset in 0..self.items.len() {
            let idx = (start + offset) % self.items.len();
            if self.items[idx].is_enabled() {
                self.selected = Some(idx);
                return;
            }
        }
    }

    /// Select previous item (skip separators and disabled items)
    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let start = self.selected.unwrap_or(0);
        for offset in 0..self.items.len() {
            let idx = (start + self.items.len() - offset - 1) % self.items.len();
            if self.items[idx].is_enabled() {
                self.selected = Some(idx);
                return;
            }
        }
    }

    /// Select first enabled item
    fn select_first_enabled(&mut self) {
        for (idx, item) in self.items.iter().enumerate() {
            if item.is_enabled() {
                self.selected = Some(idx);
                return;
            }
        }
    }

    /// Set selected index
    pub fn set_selected(&mut self, index: usize) -> bool {
        if index < self.items.len() && self.items[index].is_enabled() {
            self.selected = Some(index);
            true
        } else {
            false
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// Render the context menu
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::DARK_GRAY))
            .title(self.title.as_deref().unwrap_or("Menu"))
            .title_style(Style::default().fg(ToadTheme::FOREGROUND));

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let is_selected = Some(idx) == self.selected;

                match item {
                    MenuItem::Separator => {
                        let line = Line::from(Span::styled(
                            "─".repeat(area.width.saturating_sub(4) as usize),
                            Style::default().fg(ToadTheme::DARK_GRAY),
                        ));
                        ListItem::new(line)
                    }
                    MenuItem::Action {
                        label,
                        shortcut,
                        icon,
                        enabled,
                    } => {
                        let style = if !enabled {
                            Style::default()
                                .fg(ToadTheme::DARK_GRAY)
                                .add_modifier(Modifier::DIM)
                        } else if is_selected {
                            Style::default()
                                .fg(ToadTheme::TOAD_GREEN)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(ToadTheme::FOREGROUND)
                        };

                        let mut spans = Vec::new();

                        // Selection indicator
                        if is_selected {
                            spans.push(Span::styled("> ", style));
                        } else {
                            spans.push(Span::raw("  "));
                        }

                        // Icon if present
                        if let Some(icon_str) = icon {
                            spans.push(Span::styled(format!("{} ", icon_str), style));
                        }

                        // Label
                        spans.push(Span::styled(label, style));

                        // Shortcut (right-aligned)
                        if let Some(shortcut_str) = shortcut {
                            spans.push(Span::raw(" "));
                            spans.push(Span::styled(
                                shortcut_str,
                                Style::default()
                                    .fg(ToadTheme::DARK_GRAY)
                                    .add_modifier(Modifier::DIM),
                            ));
                        }

                        ListItem::new(Line::from(spans))
                    }
                }
            })
            .collect();

        let list = List::new(items).block(block);
        frame.render_widget(list, area);
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_action() {
        let item = MenuItem::action("Copy", "Ctrl+C");
        assert!(item.is_action());
        assert!(!item.is_separator());
        assert!(item.is_enabled());
        assert_eq!(item.label(), Some("Copy"));
        assert_eq!(item.shortcut(), Some("Ctrl+C"));
    }

    #[test]
    fn test_menu_item_simple() {
        let item = MenuItem::simple("Delete");
        assert!(item.is_action());
        assert_eq!(item.label(), Some("Delete"));
        assert_eq!(item.shortcut(), None);
    }

    #[test]
    fn test_menu_item_separator() {
        let item = MenuItem::separator();
        assert!(item.is_separator());
        assert!(!item.is_action());
        assert!(!item.is_enabled());
    }

    #[test]
    fn test_menu_item_with_icon() {
        let item = MenuItem::simple("Open").with_icon("📁");
        assert!(item.is_action());
    }

    #[test]
    fn test_menu_item_disabled() {
        let item = MenuItem::simple("Paste").with_enabled(false);
        assert!(!item.is_enabled());
    }

    #[test]
    fn test_context_menu_creation() {
        let menu = ContextMenu::new();
        assert_eq!(menu.item_count(), 0);
        assert!(menu.is_empty());
        assert_eq!(menu.selected_index(), None);
    }

    #[test]
    fn test_context_menu_add_item() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
        menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

        assert_eq!(menu.item_count(), 2);
        assert!(!menu.is_empty());
    }

    #[test]
    fn test_context_menu_navigation() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
        menu.add_item(MenuItem::action("Paste", "Ctrl+V"));
        menu.add_item(MenuItem::action("Cut", "Ctrl+X"));

        assert_eq!(menu.selected_index(), Some(0));

        menu.select_next();
        assert_eq!(menu.selected_index(), Some(1));

        menu.select_next();
        assert_eq!(menu.selected_index(), Some(2));

        menu.select_next();
        assert_eq!(menu.selected_index(), Some(0)); // Wrap around

        menu.select_previous();
        assert_eq!(menu.selected_index(), Some(2)); // Wrap around
    }

    #[test]
    fn test_context_menu_skip_separators() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
        menu.add_item(MenuItem::separator());
        menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

        assert_eq!(menu.selected_index(), Some(0));

        menu.select_next();
        assert_eq!(menu.selected_index(), Some(2)); // Skip separator
    }

    #[test]
    fn test_context_menu_skip_disabled() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
        menu.add_item(MenuItem::simple("Paste").with_enabled(false));
        menu.add_item(MenuItem::action("Cut", "Ctrl+X"));

        assert_eq!(menu.selected_index(), Some(0));

        menu.select_next();
        assert_eq!(menu.selected_index(), Some(2)); // Skip disabled item
    }

    #[test]
    fn test_context_menu_set_selected() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
        menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

        assert!(menu.set_selected(1));
        assert_eq!(menu.selected_index(), Some(1));

        // Can't select separator
        menu.add_item(MenuItem::separator());
        assert!(!menu.set_selected(2));
    }

    #[test]
    fn test_context_menu_selected_item() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
        menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

        let selected = menu.selected_item();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().label(), Some("Copy"));
    }

    #[test]
    fn test_context_menu_clear_selection() {
        let mut menu = ContextMenu::new();
        menu.add_item(MenuItem::action("Copy", "Ctrl+C"));

        assert!(menu.selected_index().is_some());

        menu.clear_selection();
        assert_eq!(menu.selected_index(), None);
    }

    #[test]
    fn test_context_menu_with_title() {
        let menu = ContextMenu::new().title("File Actions");
        assert_eq!(menu.item_count(), 0);
    }
}
