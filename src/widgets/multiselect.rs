/// Multi-select widget for bulk operations
///
/// Allows selecting multiple items from a list with keyboard shortcuts
///
/// # Examples
///
/// ```
/// use toad::widgets::MultiSelect;
///
/// let items = vec!["item1", "item2", "item3"];
/// let mut select = MultiSelect::new(items);
/// select.select(0);
/// assert_eq!(select.selected_count(), 1);
/// ```

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::theme::ToadTheme;

/// Selection mode for multi-select
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
    /// Single selection only
    Single,
    /// Multiple selection allowed
    Multiple,
    /// Range selection (shift+click)
    Range,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Multiple
    }
}

/// Multi-select widget state
#[derive(Debug, Clone)]
pub struct MultiSelect<T> {
    /// All items
    items: Vec<T>,
    /// Selected item indices
    selected: HashSet<usize>,
    /// Current cursor position
    cursor: usize,
    /// Selection mode
    mode: SelectionMode,
    /// Last selected index (for range selection)
    last_selected: Option<usize>,
    /// Show selection checkboxes
    show_checkboxes: bool,
}

impl<T> MultiSelect<T> {
    /// Create a new multi-select widget
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let items = vec!["item1", "item2", "item3"];
    /// let select = MultiSelect::new(items);
    /// assert_eq!(select.item_count(), 3);
    /// ```
    pub fn new(items: Vec<T>) -> Self {
        Self {
            cursor: if items.is_empty() { 0 } else { 0 },
            items,
            selected: HashSet::new(),
            mode: SelectionMode::Multiple,
            last_selected: None,
            show_checkboxes: true,
        }
    }

    /// Set selection mode
    pub fn with_mode(mut self, mode: SelectionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set whether to show checkboxes
    pub fn with_checkboxes(mut self, show: bool) -> Self {
        self.show_checkboxes = show;
        self
    }

    /// Get number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get number of selected items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(0);
    /// select.select(1);
    /// assert_eq!(select.selected_count(), 2);
    /// ```
    pub fn selected_count(&self) -> usize {
        self.selected.len()
    }

    /// Get current cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Move cursor to next item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.next();
    /// assert_eq!(select.cursor(), 1);
    /// ```
    pub fn next(&mut self) {
        if !self.items.is_empty() && self.cursor < self.items.len() - 1 {
            self.cursor += 1;
        }
    }

    /// Move cursor to previous item
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.next();
    /// select.previous();
    /// assert_eq!(select.cursor(), 0);
    /// ```
    pub fn previous(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor to first item
    pub fn first(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to last item
    pub fn last(&mut self) {
        if !self.items.is_empty() {
            self.cursor = self.items.len() - 1;
        }
    }

    /// Select item at index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(1);
    /// assert!(select.is_selected(1));
    /// ```
    pub fn select(&mut self, index: usize) {
        if index >= self.items.len() {
            return;
        }

        match self.mode {
            SelectionMode::Single => {
                self.selected.clear();
                self.selected.insert(index);
            }
            SelectionMode::Multiple => {
                self.selected.insert(index);
            }
            SelectionMode::Range => {
                if let Some(last) = self.last_selected {
                    // Select range from last to current
                    let start = last.min(index);
                    let end = last.max(index);
                    for i in start..=end {
                        self.selected.insert(i);
                    }
                } else {
                    self.selected.insert(index);
                }
            }
        }

        self.last_selected = Some(index);
    }

    /// Deselect item at index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(1);
    /// select.deselect(1);
    /// assert!(!select.is_selected(1));
    /// ```
    pub fn deselect(&mut self, index: usize) {
        self.selected.remove(&index);
    }

    /// Toggle selection at index
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.toggle(0);
    /// assert!(select.is_selected(0));
    /// select.toggle(0);
    /// assert!(!select.is_selected(0));
    /// ```
    pub fn toggle(&mut self, index: usize) {
        if index >= self.items.len() {
            return;
        }

        if self.is_selected(index) {
            self.deselect(index);
        } else {
            self.select(index);
        }
    }

    /// Toggle selection at cursor position
    pub fn toggle_current(&mut self) {
        self.toggle(self.cursor);
    }

    /// Check if item at index is selected
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(1);
    /// assert!(select.is_selected(1));
    /// assert!(!select.is_selected(0));
    /// ```
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    /// Select all items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select_all();
    /// assert_eq!(select.selected_count(), 3);
    /// ```
    pub fn select_all(&mut self) {
        if self.mode == SelectionMode::Single {
            return;
        }

        for i in 0..self.items.len() {
            self.selected.insert(i);
        }
    }

    /// Clear all selections
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(0);
    /// select.select(1);
    /// select.clear_selection();
    /// assert_eq!(select.selected_count(), 0);
    /// ```
    pub fn clear_selection(&mut self) {
        self.selected.clear();
        self.last_selected = None;
    }

    /// Get selected indices
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(0);
    /// select.select(2);
    ///
    /// let selected = select.selected_indices();
    /// assert_eq!(selected.len(), 2);
    /// ```
    pub fn selected_indices(&self) -> Vec<usize> {
        let mut indices: Vec<_> = self.selected.iter().copied().collect();
        indices.sort();
        indices
    }

    /// Get selected items
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(0);
    /// select.select(2);
    ///
    /// let items = select.selected_items();
    /// assert_eq!(items.len(), 2);
    /// ```
    pub fn selected_items(&self) -> Vec<&T> {
        let mut items = Vec::new();
        let mut indices: Vec<_> = self.selected.iter().copied().collect();
        indices.sort();

        for idx in indices {
            if let Some(item) = self.items.get(idx) {
                items.push(item);
            }
        }

        items
    }

    /// Get all items
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Get item at index
    pub fn item(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Set items
    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.cursor = 0;
        self.selected.clear();
        self.last_selected = None;
    }

    /// Invert selection
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::MultiSelect;
    ///
    /// let mut select = MultiSelect::new(vec!["a", "b", "c"]);
    /// select.select(0);
    /// select.invert_selection();
    /// assert!(!select.is_selected(0));
    /// assert!(select.is_selected(1));
    /// assert!(select.is_selected(2));
    /// ```
    pub fn invert_selection(&mut self) {
        if self.mode == SelectionMode::Single {
            return;
        }

        let mut new_selected = HashSet::new();
        for i in 0..self.items.len() {
            if !self.selected.contains(&i) {
                new_selected.insert(i);
            }
        }
        self.selected = new_selected;
    }
}

impl<T: std::fmt::Display> MultiSelect<T> {
    /// Render the multi-select widget
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let checkbox = if self.show_checkboxes {
                    if self.is_selected(idx) {
                        "[x] "
                    } else {
                        "[ ] "
                    }
                } else {
                    ""
                };

                let text = format!("{}{}", checkbox, item);

                let style = if self.is_selected(idx) {
                    Style::default()
                        .fg(ToadTheme::TOAD_GREEN)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(text).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(ToadTheme::TOAD_GREEN_DARK)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let mut state = ListState::default();
        state.select(Some(self.cursor));

        frame.render_stateful_widget(list, area, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiselect_creation() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);
        assert_eq!(select.item_count(), 3);
        assert_eq!(select.selected_count(), 0);
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_multiselect_empty() {
        let select: MultiSelect<String> = MultiSelect::new(vec![]);
        assert_eq!(select.item_count(), 0);
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_multiselect_navigation() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.next();
        assert_eq!(select.cursor(), 1);

        select.next();
        assert_eq!(select.cursor(), 2);

        select.previous();
        assert_eq!(select.cursor(), 1);

        select.first();
        assert_eq!(select.cursor(), 0);

        select.last();
        assert_eq!(select.cursor(), 2);
    }

    #[test]
    fn test_multiselect_selection() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.select(0);
        assert!(select.is_selected(0));
        assert_eq!(select.selected_count(), 1);

        select.select(2);
        assert!(select.is_selected(2));
        assert_eq!(select.selected_count(), 2);

        select.deselect(0);
        assert!(!select.is_selected(0));
        assert_eq!(select.selected_count(), 1);
    }

    #[test]
    fn test_multiselect_toggle() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.toggle(0);
        assert!(select.is_selected(0));

        select.toggle(0);
        assert!(!select.is_selected(0));

        select.toggle_current();
        assert!(select.is_selected(0));
    }

    #[test]
    fn test_multiselect_single_mode() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"])
            .with_mode(SelectionMode::Single);

        select.select(0);
        select.select(1);

        assert!(!select.is_selected(0));
        assert!(select.is_selected(1));
        assert_eq!(select.selected_count(), 1);
    }

    #[test]
    fn test_multiselect_select_all() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.select_all();
        assert_eq!(select.selected_count(), 3);
        assert!(select.is_selected(0));
        assert!(select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_multiselect_clear_selection() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.select(0);
        select.select(1);
        select.clear_selection();

        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_multiselect_selected_indices() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d"]);

        select.select(2);
        select.select(0);
        select.select(3);

        let indices = select.selected_indices();
        assert_eq!(indices, vec![0, 2, 3]);
    }

    #[test]
    fn test_multiselect_selected_items() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d"]);

        select.select(1);
        select.select(3);

        let items = select.selected_items();
        assert_eq!(items, vec![&"b", &"d"]);
    }

    #[test]
    fn test_multiselect_invert_selection() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.select(0);
        select.invert_selection();

        assert!(!select.is_selected(0));
        assert!(select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_multiselect_set_items() {
        let mut select = MultiSelect::new(vec!["a", "b"]);
        select.select(0);

        select.set_items(vec!["x", "y", "z"]);

        assert_eq!(select.item_count(), 3);
        assert_eq!(select.selected_count(), 0);
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_multiselect_range_mode() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d", "e"])
            .with_mode(SelectionMode::Range);

        select.select(1);
        select.select(3);

        assert!(select.is_selected(1));
        assert!(select.is_selected(2));
        assert!(select.is_selected(3));
        assert_eq!(select.selected_count(), 3);
    }

    #[test]
    fn test_multiselect_item_access() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);

        assert_eq!(select.item(0), Some(&"a"));
        assert_eq!(select.item(1), Some(&"b"));
        assert_eq!(select.item(3), None);

        assert_eq!(select.items().len(), 3);
    }

    #[test]
    fn test_multiselect_with_checkboxes() {
        let select = MultiSelect::new(vec!["a", "b", "c"])
            .with_checkboxes(false);

        assert!(!select.show_checkboxes);
    }
}
