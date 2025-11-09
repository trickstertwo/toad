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
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::ui::theme::ToadTheme;

/// Selection mode for multi-select
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum SelectionMode {
    /// Single selection only
    Single,
    /// Multiple selection allowed
    #[default]
    Multiple,
    /// Range selection (shift+click)
    Range,
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
            cursor: 0,
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
        let mut select = MultiSelect::new(vec!["a", "b", "c"]).with_mode(SelectionMode::Single);

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
        let mut select =
            MultiSelect::new(vec!["a", "b", "c", "d", "e"]).with_mode(SelectionMode::Range);

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
        let select = MultiSelect::new(vec!["a", "b", "c"]).with_checkboxes(false);

        assert!(!select.show_checkboxes);
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_multiselect_10000_items() {
        let items: Vec<String> = (0..10000).map(|i| format!("Item {}", i)).collect();
        let select = MultiSelect::new(items);
        assert_eq!(select.item_count(), 10000);
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_multiselect_select_all_10000() {
        let items: Vec<String> = (0..10000).map(|i| format!("Item {}", i)).collect();
        let mut select = MultiSelect::new(items);
        select.select_all();
        assert_eq!(select.selected_count(), 10000);
    }

    #[test]
    fn test_multiselect_rapid_navigation_1000() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d", "e"]);
        for _ in 0..1000 {
            select.next();
        }
        // Should stop at last item
        assert_eq!(select.cursor(), 4);
    }

    #[test]
    fn test_multiselect_rapid_toggle_1000() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        for _ in 0..1000 {
            select.toggle(0);
        }
        // 1000 toggles = even number, should be not selected
        assert!(!select.is_selected(0));
    }

    #[test]
    fn test_multiselect_rapid_select_deselect() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        for _ in 0..1000 {
            select.select(0);
            select.deselect(0);
        }
        assert!(!select.is_selected(0));
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_multiselect_emoji_items() {
        let mut select = MultiSelect::new(vec!["🚀", "🐸", "💚"]);
        select.select(0);
        select.select(2);
        let items = select.selected_items();
        assert_eq!(items, vec![&"🚀", &"💚"]);
    }

    #[test]
    fn test_multiselect_rtl_arabic() {
        let mut select = MultiSelect::new(vec!["مرحبا", "بك", "في", "العالم"]);
        select.select(1);
        select.select(3);
        assert_eq!(select.selected_count(), 2);
    }

    #[test]
    fn test_multiselect_rtl_hebrew() {
        let mut select = MultiSelect::new(vec!["שלום", "עולם"]);
        select.select_all();
        assert_eq!(select.selected_count(), 2);
    }

    #[test]
    fn test_multiselect_japanese() {
        let mut select = MultiSelect::new(vec!["こんにちは", "世界", "日本語"]);
        select.select(0);
        select.select(2);
        let indices = select.selected_indices();
        assert_eq!(indices, vec![0, 2]);
    }

    #[test]
    fn test_multiselect_mixed_scripts() {
        let mut select = MultiSelect::new(vec![
            "Hello",
            "مرحبا",
            "שלום",
            "こんにちは",
            "🚀",
        ]);
        select.select_all();
        assert_eq!(select.selected_count(), 5);
    }

    #[test]
    fn test_multiselect_combining_characters() {
        let mut select = MultiSelect::new(vec!["é̂", "ñ̃", "ỹ̀"]);
        select.select(1);
        assert!(select.is_selected(1));
    }

    #[test]
    fn test_multiselect_zero_width_characters() {
        let mut select = MultiSelect::new(vec!["Test\u{200B}Zero", "Width\u{200C}Chars"]);
        select.select(0);
        let items = select.selected_items();
        assert_eq!(items.len(), 1);
    }

    // ============ Extreme Values ============

    #[test]
    fn test_multiselect_single_item() {
        let mut select = MultiSelect::new(vec!["only"]);
        select.next(); // Should not move beyond last
        assert_eq!(select.cursor(), 0);
        select.select(0);
        assert_eq!(select.selected_count(), 1);
    }

    #[test]
    fn test_multiselect_select_out_of_bounds() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.select(999);
        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_multiselect_toggle_out_of_bounds() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.toggle(999);
        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_multiselect_deselect_out_of_bounds() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.select(0);
        select.deselect(999); // Should not panic
        assert_eq!(select.selected_count(), 1);
    }

    #[test]
    fn test_multiselect_very_long_item_text() {
        let long_text = "A".repeat(100000);
        let mut select = MultiSelect::new(vec![long_text.clone()]);
        select.select(0);
        let items = select.selected_items();
        assert_eq!(items[0].len(), 100000);
    }

    // ============ Selection Mode Edge Cases ============

    #[test]
    fn test_single_mode_select_all_does_nothing() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]).with_mode(SelectionMode::Single);
        select.select_all();
        assert_eq!(select.selected_count(), 0);
    }

    #[test]
    fn test_single_mode_invert_does_nothing() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]).with_mode(SelectionMode::Single);
        select.select(0);
        select.invert_selection();
        assert_eq!(select.selected_count(), 1);
        assert!(select.is_selected(0));
    }

    #[test]
    fn test_range_mode_first_selection() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]).with_mode(SelectionMode::Range);
        select.select(1);
        assert!(select.is_selected(1));
        assert_eq!(select.selected_count(), 1);
    }

    #[test]
    fn test_range_mode_reverse_range() {
        let mut select =
            MultiSelect::new(vec!["a", "b", "c", "d", "e"]).with_mode(SelectionMode::Range);
        select.select(3);
        select.select(1);
        // Should select 1, 2, 3
        assert!(select.is_selected(1));
        assert!(select.is_selected(2));
        assert!(select.is_selected(3));
        assert_eq!(select.selected_count(), 3);
    }

    #[test]
    fn test_range_mode_same_index_twice() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]).with_mode(SelectionMode::Range);
        select.select(1);
        select.select(1);
        assert!(select.is_selected(1));
        assert_eq!(select.selected_count(), 1);
    }

    #[test]
    fn test_range_mode_full_range() {
        let mut select =
            MultiSelect::new(vec!["a", "b", "c", "d", "e"]).with_mode(SelectionMode::Range);
        select.select(0);
        select.select(4);
        assert_eq!(select.selected_count(), 5);
        for i in 0..5 {
            assert!(select.is_selected(i));
        }
    }

    // ============ Navigation Edge Cases ============

    #[test]
    fn test_navigation_empty_list() {
        let mut select: MultiSelect<String> = MultiSelect::new(vec![]);
        select.next();
        select.previous();
        select.first();
        select.last();
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_previous_at_beginning() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.previous();
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_next_at_end() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.last();
        select.next();
        assert_eq!(select.cursor(), 2);
    }

    #[test]
    fn test_first_last_single_item() {
        let mut select = MultiSelect::new(vec!["only"]);
        select.first();
        assert_eq!(select.cursor(), 0);
        select.last();
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_navigation_wraparound() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        for _ in 0..10 {
            select.next();
        }
        assert_eq!(select.cursor(), 2);

        select.first();
        for _ in 0..10 {
            select.previous();
        }
        assert_eq!(select.cursor(), 0);
    }

    // ============ Selected Items Edge Cases ============

    #[test]
    fn test_selected_items_empty_selection() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);
        let items = select.selected_items();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn test_selected_indices_sorted() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d"]);
        select.select(3);
        select.select(1);
        select.select(2);
        let indices = select.selected_indices();
        assert_eq!(indices, vec![1, 2, 3]);
    }

    #[test]
    fn test_selected_items_after_clear() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.select(0);
        select.select(1);
        select.clear_selection();
        let items = select.selected_items();
        assert_eq!(items.len(), 0);
    }

    // ============ Trait Coverage ============

    #[test]
    fn test_selection_mode_clone() {
        let mode = SelectionMode::Multiple;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_selection_mode_equality() {
        assert_eq!(SelectionMode::Single, SelectionMode::Single);
        assert_ne!(SelectionMode::Single, SelectionMode::Multiple);
    }

    #[test]
    fn test_selection_mode_debug() {
        let mode = SelectionMode::Range;
        let debug_str = format!("{:?}", mode);
        assert!(debug_str.contains("Range"));
    }

    #[test]
    fn test_selection_mode_default() {
        let mode = SelectionMode::default();
        assert_eq!(mode, SelectionMode::Multiple);
    }

    #[test]
    fn test_multiselect_clone() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);
        select.select(0);
        select.next();

        let cloned = select.clone();
        assert_eq!(cloned.cursor(), 1);
        assert!(cloned.is_selected(0));
        assert_eq!(cloned.item_count(), 3);
    }

    #[test]
    fn test_multiselect_debug() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);
        let debug_str = format!("{:?}", select);
        assert!(debug_str.contains("MultiSelect"));
    }

    #[test]
    fn test_selection_mode_serialize() {
        let mode = SelectionMode::Multiple;
        let json = serde_json::to_string(&mode).unwrap();
        assert!(json.contains("Multiple"));
    }

    #[test]
    fn test_selection_mode_deserialize() {
        let json = "\"Single\"";
        let mode: SelectionMode = serde_json::from_str(json).unwrap();
        assert_eq!(mode, SelectionMode::Single);
    }

    // ============ Complex Workflows ============

    #[test]
    fn test_select_navigate_select_workflow() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d", "e"]);

        select.select(0);
        select.next();
        select.next();
        select.toggle_current();

        assert!(select.is_selected(0));
        assert!(select.is_selected(2));
        assert_eq!(select.cursor(), 2);
    }

    #[test]
    fn test_select_all_then_deselect_some() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d"]);

        select.select_all();
        select.deselect(1);
        select.deselect(3);

        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
        assert!(select.is_selected(2));
        assert!(!select.is_selected(3));
        assert_eq!(select.selected_count(), 2);
    }

    #[test]
    fn test_invert_twice_restores_original() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.select(0);
        select.select(2);

        select.invert_selection();
        select.invert_selection();

        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_set_items_resets_state() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]);

        select.select(0);
        select.select(1);
        select.next();
        select.next();

        select.set_items(vec!["x", "y"]);

        assert_eq!(select.item_count(), 2);
        assert_eq!(select.selected_count(), 0);
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_toggle_current_with_navigation() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d"]);

        for _ in 0..4 {
            select.toggle_current();
            select.next();
        }

        assert!(select.is_selected(0));
        assert!(select.is_selected(1));
        assert!(select.is_selected(2));
        assert!(select.is_selected(3));
    }

    #[test]
    fn test_range_selection_multiple_ranges() {
        let mut select = MultiSelect::new(vec!["a", "b", "c", "d", "e", "f"])
            .with_mode(SelectionMode::Range);

        select.select(1);
        select.select(3);
        // Now 1, 2, 3 are selected

        select.clear_selection();
        select.select(4);
        select.select(5);
        // Now 4, 5 are selected

        assert!(!select.is_selected(1));
        assert!(!select.is_selected(2));
        assert!(!select.is_selected(3));
        assert!(select.is_selected(4));
        assert!(select.is_selected(5));
    }

    // ============ Item Access Edge Cases ============

    #[test]
    fn test_item_access_valid_indices() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);
        assert_eq!(select.item(0), Some(&"a"));
        assert_eq!(select.item(1), Some(&"b"));
        assert_eq!(select.item(2), Some(&"c"));
    }

    #[test]
    fn test_item_access_invalid_index() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);
        assert_eq!(select.item(999), None);
    }

    #[test]
    fn test_items_returns_all() {
        let select = MultiSelect::new(vec!["a", "b", "c"]);
        let items = select.items();
        assert_eq!(items.len(), 3);
        assert_eq!(items, &["a", "b", "c"]);
    }

    // ============ Builder Pattern Edge Cases ============

    #[test]
    fn test_chained_builders() {
        let select = MultiSelect::new(vec!["a", "b", "c"])
            .with_mode(SelectionMode::Single)
            .with_checkboxes(false)
            .with_mode(SelectionMode::Range)
            .with_checkboxes(true);

        assert_eq!(select.mode, SelectionMode::Range);
        assert!(select.show_checkboxes);
    }

    #[test]
    fn test_with_mode_all_variants() {
        let s1 = MultiSelect::new(vec!["a"]).with_mode(SelectionMode::Single);
        assert_eq!(s1.mode, SelectionMode::Single);

        let s2 = MultiSelect::new(vec!["a"]).with_mode(SelectionMode::Multiple);
        assert_eq!(s2.mode, SelectionMode::Multiple);

        let s3 = MultiSelect::new(vec!["a"]).with_mode(SelectionMode::Range);
        assert_eq!(s3.mode, SelectionMode::Range);
    }

    #[test]
    fn test_with_checkboxes_toggle() {
        let s1 = MultiSelect::new(vec!["a"]).with_checkboxes(true);
        assert!(s1.show_checkboxes);

        let s2 = MultiSelect::new(vec!["a"]).with_checkboxes(false);
        assert!(!s2.show_checkboxes);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_multiselect_stress() {
        let items: Vec<String> = (0..100)
            .map(|i| match i % 4 {
                0 => format!("ASCII {}", i),
                1 => format!("🚀 Emoji {}", i),
                2 => format!("日本語 {}", i),
                _ => format!("مرحبا {}", i),
            })
            .collect();

        let mut select = MultiSelect::new(items)
            .with_mode(SelectionMode::Multiple)
            .with_checkboxes(true);

        // Phase 1: Navigate and select
        for i in 0..50 {
            if i % 3 == 0 {
                select.toggle_current();
            }
            select.next();
        }
        assert_eq!(select.cursor(), 50);

        // Phase 2: Select all
        select.select_all();
        assert_eq!(select.selected_count(), 100);

        // Phase 3: Deselect some
        for i in (0..100).step_by(2) {
            select.deselect(i);
        }
        assert_eq!(select.selected_count(), 50);

        // Phase 4: Invert selection
        select.invert_selection();
        assert_eq!(select.selected_count(), 50);

        // Phase 5: Clear and select specific indices
        select.clear_selection();
        assert_eq!(select.selected_count(), 0);

        select.select(10);
        select.select(20);
        select.select(30);
        assert_eq!(select.selected_count(), 3);

        let indices = select.selected_indices();
        assert_eq!(indices, vec![10, 20, 30]);

        // Phase 6: Get selected items
        let items = select.selected_items();
        assert_eq!(items.len(), 3);

        // Phase 7: Navigation edge cases
        select.first();
        assert_eq!(select.cursor(), 0);

        select.last();
        assert_eq!(select.cursor(), 99);

        // Phase 8: Switch to Single mode
        select = select.with_mode(SelectionMode::Single);
        select.select(50);
        select.select(60);
        assert_eq!(select.selected_count(), 1);
        assert!(select.is_selected(60));

        // Phase 9: Switch to Range mode
        select = select.with_mode(SelectionMode::Range);
        select.clear_selection();
        select.select(20);
        select.select(25);
        assert_eq!(select.selected_count(), 6); // 20, 21, 22, 23, 24, 25

        // Phase 10: Set new items
        let new_items: Vec<String> = vec!["Final".to_string(), "Test".to_string()];
        select.set_items(new_items);
        assert_eq!(select.item_count(), 2);
        assert_eq!(select.selected_count(), 0);
        assert_eq!(select.cursor(), 0);
    }

    // ============ Empty List Edge Cases ============

    #[test]
    fn test_empty_list_operations() {
        let mut select: MultiSelect<String> = MultiSelect::new(vec![]);

        select.select(0);
        select.toggle(0);
        select.select_all();
        select.invert_selection();
        select.clear_selection();

        assert_eq!(select.item_count(), 0);
        assert_eq!(select.selected_count(), 0);
        assert_eq!(select.cursor(), 0);
    }

    #[test]
    fn test_empty_list_selected_items() {
        let select: MultiSelect<String> = MultiSelect::new(vec![]);
        let items = select.selected_items();
        let indices = select.selected_indices();

        assert_eq!(items.len(), 0);
        assert_eq!(indices.len(), 0);
    }

    #[test]
    fn test_last_selected_tracking() {
        let mut select = MultiSelect::new(vec!["a", "b", "c"]).with_mode(SelectionMode::Range);

        assert_eq!(select.last_selected, None);

        select.select(0);
        assert_eq!(select.last_selected, Some(0));

        select.select(2);
        assert_eq!(select.last_selected, Some(2));

        select.clear_selection();
        assert_eq!(select.last_selected, None);
    }
}
