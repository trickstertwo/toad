//! Integration tests for PLATINUM Tier Smart Features & Developer Tools
//!
//! Tests for undo/redo, breadcrumbs, context menus, minimap, and multiselect.

use toad::ui::multiselect::SelectionMode;
use toad::ui::widgets::{
    Action, BreadcrumbSegment, Breadcrumbs, ContextMenu, MenuItem, Minimap, MinimapMode,
    MultiSelect, UndoRedoManager,
};

// ==================== UndoRedoManager Tests ====================

#[test]
fn test_undo_redo_basic_workflow() {
    let mut manager = UndoRedoManager::new();

    // Execute some actions
    let action1 = Action::new("Insert Text", "insert 'hello'");
    let action2 = Action::new("Delete Char", "delete 1 char");
    let action3 = Action::new("Insert Line", "insert new line");

    manager.execute(action1);
    manager.execute(action2);
    manager.execute(action3);

    assert_eq!(manager.history_size(), 3);
    assert!(manager.can_undo());
    assert!(!manager.can_redo());
}

#[test]
fn test_undo_redo_operations() {
    let mut manager = UndoRedoManager::new();

    let action1 = Action::new("Action 1", "data 1");
    let action2 = Action::new("Action 2", "data 2");

    manager.execute(action1);
    manager.execute(action2);

    // Undo
    let undone = manager.undo();
    assert!(undone.is_some());
    assert_eq!(undone.unwrap().name(), "Action 2");
    assert!(manager.can_redo());

    // Redo
    let redone = manager.redo();
    assert!(redone.is_some());
    assert_eq!(redone.unwrap().name(), "Action 2");
}

#[test]
fn test_undo_redo_branching() {
    let mut manager = UndoRedoManager::new();

    manager.execute(Action::new("A", "data"));
    manager.execute(Action::new("B", "data"));
    manager.execute(Action::new("C", "data"));

    // Undo twice
    manager.undo();
    manager.undo();

    // Execute new action (creates branch, clears redo)
    manager.execute(Action::new("D", "data"));

    assert!(!manager.can_redo());
    assert_eq!(manager.history_size(), 2); // A, D
}

#[test]
fn test_undo_redo_max_history() {
    let mut manager = UndoRedoManager::with_max_history(3);

    for i in 0..5 {
        manager.execute(Action::new(format!("Action {}", i), format!("data {}", i)));
    }

    // Should only keep last 3
    assert!(manager.history_size() <= 3);
}

#[test]
fn test_undo_redo_clear() {
    let mut manager = UndoRedoManager::new();

    manager.execute(Action::new("A", "data"));
    manager.execute(Action::new("B", "data"));

    assert_eq!(manager.history_size(), 2);

    manager.clear();

    assert_eq!(manager.history_size(), 0);
    assert!(!manager.can_undo());
    assert!(!manager.can_redo());
}

#[test]
fn test_undo_redo_position_tracking() {
    let mut manager = UndoRedoManager::new();

    manager.execute(Action::new("A", "data"));
    manager.execute(Action::new("B", "data"));
    manager.execute(Action::new("C", "data"));

    assert_eq!(manager.position(), 3);

    manager.undo();
    assert_eq!(manager.position(), 2);

    manager.undo();
    assert_eq!(manager.position(), 1);

    manager.redo();
    assert_eq!(manager.position(), 2);
}

// ==================== Breadcrumbs Tests ====================

#[test]
fn test_breadcrumbs_creation() {
    let breadcrumbs = Breadcrumbs::new();

    assert_eq!(breadcrumbs.segments().len(), 0);
}

#[test]
fn test_breadcrumbs_from_path() {
    let breadcrumbs = Breadcrumbs::from_path("/home/user/projects/toad");

    assert!(breadcrumbs.segments().len() > 0);
}

#[test]
fn test_breadcrumbs_push_pop() {
    let mut breadcrumbs = Breadcrumbs::new();

    breadcrumbs.push(BreadcrumbSegment::new("Home"));
    breadcrumbs.push(BreadcrumbSegment::new("Documents"));
    breadcrumbs.push(BreadcrumbSegment::new("Projects"));

    assert_eq!(breadcrumbs.segments().len(), 3);

    let popped = breadcrumbs.pop();
    assert!(popped.is_some());
    assert_eq!(breadcrumbs.segments().len(), 2);
}

#[test]
fn test_breadcrumbs_set_segments() {
    let mut breadcrumbs = Breadcrumbs::new();

    let segments = vec![
        BreadcrumbSegment::new("Root"),
        BreadcrumbSegment::new("Folder1"),
        BreadcrumbSegment::new("Folder2"),
    ];

    breadcrumbs.set_segments(segments);

    assert_eq!(breadcrumbs.segments().len(), 3);
}

#[test]
fn test_breadcrumbs_clear() {
    let mut breadcrumbs = Breadcrumbs::new();

    breadcrumbs.push(BreadcrumbSegment::new("A"));
    breadcrumbs.push(BreadcrumbSegment::new("B"));

    assert_eq!(breadcrumbs.segments().len(), 2);

    breadcrumbs.clear();

    assert_eq!(breadcrumbs.segments().len(), 0);
}

#[test]
fn test_breadcrumbs_hover_tracking() {
    let mut breadcrumbs = Breadcrumbs::new();

    breadcrumbs.push(BreadcrumbSegment::new("A"));
    breadcrumbs.push(BreadcrumbSegment::new("B"));
    breadcrumbs.push(BreadcrumbSegment::new("C"));

    assert_eq!(breadcrumbs.hovered(), None);

    breadcrumbs.set_hovered(Some(1));
    assert_eq!(breadcrumbs.hovered(), Some(1));

    breadcrumbs.set_hovered(None);
    assert_eq!(breadcrumbs.hovered(), None);
}

// ==================== ContextMenu Tests ====================

#[test]
fn test_context_menu_creation() {
    let menu = ContextMenu::new();

    assert_eq!(menu.item_count(), 0);
    assert!(menu.is_empty());
}

#[test]
fn test_context_menu_add_items() {
    let mut menu = ContextMenu::new().title("File Menu");

    menu.add_item(MenuItem::action("Open", "Ctrl+O"));
    menu.add_item(MenuItem::action("Save", "Ctrl+S"));
    menu.add_item(MenuItem::separator());
    menu.add_item(MenuItem::simple("Exit"));

    assert_eq!(menu.item_count(), 4);
    assert!(!menu.is_empty());
}

#[test]
fn test_context_menu_selection() {
    let mut menu = ContextMenu::new();

    menu.add_item(MenuItem::action("Cut", "Ctrl+X"));
    menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
    menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

    // First item auto-selected
    assert_eq!(menu.selected_index(), Some(0));

    menu.select_next();
    assert_eq!(menu.selected_index(), Some(1));

    menu.select_next();
    assert_eq!(menu.selected_index(), Some(2));

    menu.select_previous();
    assert_eq!(menu.selected_index(), Some(1));
}

#[test]
fn test_context_menu_disabled_items() {
    let mut menu = ContextMenu::new();

    menu.add_item(MenuItem::action("Enabled", ""));
    menu.add_item(MenuItem::action("Disabled", "").with_enabled(false));
    menu.add_item(MenuItem::action("Also Enabled", ""));

    assert_eq!(menu.item_count(), 3);

    // Navigation should work (implementation may skip disabled)
    menu.select_next();
    menu.select_next();
}

#[test]
fn test_context_menu_separators() {
    let mut menu = ContextMenu::new();

    menu.add_item(MenuItem::action("Action 1", ""));
    menu.add_item(MenuItem::separator());
    menu.add_item(MenuItem::action("Action 2", ""));

    let items = menu.items();
    assert!(items[1].is_separator());
    assert!(!items[0].is_separator());
    assert!(!items[2].is_separator());
}

// ==================== Minimap Tests ====================

#[test]
fn test_minimap_creation() {
    let lines = vec!["Line 1", "Line 2", "Line 3"];
    let minimap = Minimap::new(lines);

    assert_eq!(minimap.line_count(), 3);
}

#[test]
fn test_minimap_modes() {
    let lines = vec!["A", "B", "C"];
    let chars = Minimap::new(lines.clone()).with_mode(MinimapMode::Characters);
    let blocks = Minimap::new(lines.clone()).with_mode(MinimapMode::Blocks);
    let colors = Minimap::new(lines).with_mode(MinimapMode::Colors);

    // All modes can be created
    assert_eq!(chars.line_count(), 3);
    assert_eq!(blocks.line_count(), 3);
    assert_eq!(colors.line_count(), 3);
}

#[test]
fn test_minimap_viewport_tracking() {
    let lines: Vec<String> = (0..100).map(|i| format!("Line {}", i)).collect();
    let mut minimap = Minimap::new(lines);

    minimap.set_viewport(10, 50);
    let (start, end) = minimap.viewport();
    assert_eq!(start, 10);
    assert_eq!(end, 50);

    minimap.set_viewport(20, 60);
    let (start, end) = minimap.viewport();
    assert_eq!(start, 20);
    assert_eq!(end, 60);
}

#[test]
fn test_minimap_scroll() {
    let lines: Vec<String> = (0..100).map(|i| format!("Line {}", i)).collect();
    let mut minimap = Minimap::new(lines);

    // Scroll down
    minimap.scroll(10);
    minimap.scroll(5);

    // Scroll up
    minimap.scroll(-3);

    // Jump to specific line
    minimap.jump_to(50);
}

// ==================== MultiSelect Tests ====================

#[test]
fn test_multiselect_creation() {
    let items = vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ];
    let selector = MultiSelect::new(items);

    assert_eq!(selector.item_count(), 3);
    assert_eq!(selector.selected_count(), 0);
}

#[test]
fn test_multiselect_single_mode() {
    let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Single);

    selector.toggle(0);
    assert_eq!(selector.selected_count(), 1);
    assert!(selector.is_selected(0));

    // Selecting another deselects first in Single mode
    selector.toggle(1);
    assert_eq!(selector.selected_count(), 1);
    assert!(!selector.is_selected(0));
    assert!(selector.is_selected(1));
}

#[test]
fn test_multiselect_multiple_mode() {
    let items = vec!["X".to_string(), "Y".to_string(), "Z".to_string()];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Multiple);

    selector.toggle(0);
    selector.toggle(2);

    assert_eq!(selector.selected_count(), 2);
    assert!(selector.is_selected(0));
    assert!(!selector.is_selected(1));
    assert!(selector.is_selected(2));
}

#[test]
fn test_multiselect_toggle() {
    let items = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Multiple);

    // Select item
    selector.toggle(0);
    assert!(selector.is_selected(0));

    // Toggle again to deselect
    selector.toggle(0);
    assert!(!selector.is_selected(0));
}

#[test]
fn test_multiselect_select_all() {
    let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Multiple);

    selector.select_all();

    assert_eq!(selector.selected_count(), 3);
    assert!(selector.is_selected(0));
    assert!(selector.is_selected(1));
    assert!(selector.is_selected(2));
}

#[test]
fn test_multiselect_clear_selection() {
    let items = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Multiple);

    selector.select_all();
    assert_eq!(selector.selected_count(), 3);

    selector.clear_selection();
    assert_eq!(selector.selected_count(), 0);
}

#[test]
fn test_multiselect_invert_selection() {
    let items = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Multiple);

    selector.toggle(0);
    selector.toggle(2);

    assert_eq!(selector.selected_count(), 2);

    selector.invert_selection();

    assert_eq!(selector.selected_count(), 2);
    assert!(!selector.is_selected(0));
    assert!(selector.is_selected(1));
    assert!(!selector.is_selected(2));
    assert!(selector.is_selected(3));
}

#[test]
fn test_multiselect_range_mode() {
    let items = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
    ];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Range);

    // In range mode, manually select multiple items
    selector.select(1);
    selector.select(2);
    selector.select(3);

    assert!(selector.is_selected(1));
    assert!(selector.is_selected(2));
    assert!(selector.is_selected(3));
    assert!(!selector.is_selected(0));
    assert!(!selector.is_selected(4));
}

// ==================== Cross-Feature Integration Tests ====================

#[test]
fn test_undo_redo_with_breadcrumbs() {
    let mut manager = UndoRedoManager::new();
    let mut breadcrumbs = Breadcrumbs::new();

    // Navigate and track with undo
    breadcrumbs.push(BreadcrumbSegment::new("Home"));
    manager.execute(Action::new("Navigate", "push Home"));

    breadcrumbs.push(BreadcrumbSegment::new("Documents"));
    manager.execute(Action::new("Navigate", "push Documents"));

    breadcrumbs.push(BreadcrumbSegment::new("Projects"));
    manager.execute(Action::new("Navigate", "push Projects"));

    assert_eq!(breadcrumbs.segments().len(), 3);
    assert_eq!(manager.history_size(), 3);

    // Undo navigation
    manager.undo();
    breadcrumbs.pop();

    assert_eq!(breadcrumbs.segments().len(), 2);
    assert!(manager.can_redo());
}

#[test]
fn test_context_menu_with_undo_redo() {
    let mut menu = ContextMenu::new().title("Edit");
    let manager = UndoRedoManager::new();

    // Add undo/redo menu items
    let undo_enabled = manager.can_undo();
    let redo_enabled = manager.can_redo();

    menu.add_item(MenuItem::action("Undo", "Ctrl+Z").with_enabled(undo_enabled));
    menu.add_item(MenuItem::action("Redo", "Ctrl+Y").with_enabled(redo_enabled));

    assert_eq!(menu.item_count(), 2);
}

#[test]
fn test_multiselect_with_context_menu() {
    let items = vec![
        "File 1".to_string(),
        "File 2".to_string(),
        "File 3".to_string(),
    ];
    let mut selector = MultiSelect::new(items).with_mode(SelectionMode::Multiple);
    let mut menu = ContextMenu::new();

    selector.toggle(0);
    selector.toggle(2);

    // Context menu based on selection
    if selector.selected_count() > 1 {
        menu.add_item(MenuItem::action("Delete Selected", "Del"));
        menu.add_item(MenuItem::action("Move Selected", ""));
    }

    assert_eq!(menu.item_count(), 2);
    assert_eq!(selector.selected_count(), 2);
}

#[test]
fn test_breadcrumbs_with_context_menu() {
    let mut breadcrumbs = Breadcrumbs::new();
    let mut menu = ContextMenu::new();

    breadcrumbs.push(BreadcrumbSegment::new("src"));
    breadcrumbs.push(BreadcrumbSegment::new("ui"));
    breadcrumbs.push(BreadcrumbSegment::new("widgets"));

    // Right-click on breadcrumb shows menu
    breadcrumbs.set_hovered(Some(1));

    menu.add_item(MenuItem::action("Go to Parent", "Alt+Up"));
    menu.add_item(MenuItem::action("Copy Path", "Ctrl+Shift+C"));
    menu.add_item(MenuItem::separator());
    menu.add_item(MenuItem::simple("Open in Explorer"));

    assert_eq!(breadcrumbs.hovered(), Some(1));
    assert_eq!(menu.item_count(), 4);
}

#[test]
fn test_complete_editing_workflow() {
    let mut undo_manager = UndoRedoManager::new();
    let mut breadcrumbs = Breadcrumbs::new();
    let mut menu = ContextMenu::new();

    // Navigate to file
    breadcrumbs.push(BreadcrumbSegment::new("src"));
    breadcrumbs.push(BreadcrumbSegment::new("main.rs"));

    // Make edits
    undo_manager.execute(Action::new("Insert", "added text"));
    undo_manager.execute(Action::new("Format", "formatted code"));
    undo_manager.execute(Action::new("Save", "saved file"));

    // Show edit menu
    menu.add_item(MenuItem::action("Undo", "Ctrl+Z").with_enabled(undo_manager.can_undo()));
    menu.add_item(MenuItem::action("Redo", "Ctrl+Y").with_enabled(undo_manager.can_redo()));
    menu.add_item(MenuItem::separator());
    menu.add_item(MenuItem::action("Cut", "Ctrl+X"));
    menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
    menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

    // Verify state
    assert_eq!(breadcrumbs.segments().len(), 2);
    assert_eq!(undo_manager.history_size(), 3);
    assert_eq!(menu.item_count(), 6);
    assert!(undo_manager.can_undo());
}
