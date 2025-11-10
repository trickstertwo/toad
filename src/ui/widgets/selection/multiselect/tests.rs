//! Multiselect widget tests

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
    let mut select = MultiSelect::new(vec!["Hello", "مرحبا", "שלום", "こんにちは", "🚀"]);
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
    let mut select =
        MultiSelect::new(vec!["a", "b", "c", "d", "e", "f"]).with_mode(SelectionMode::Range);

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
