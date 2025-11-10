//! Integration tests for Modal Editing and Smart Features
//!
//! Tests PLATINUM tier features for visual selection, macros, marks,
//! context menus, quick actions, and smart suggestions.

use toad::commands::{ActionCategory, QuickAction, QuickActionManager};
use toad::editor::{Macro, MacroAction, MacroManager, Mark, MarkType, MarksManager};
use toad::editor::{Position, SelectionMode, VisualSelection};
use toad::ui::widgets::{ContextMenu, MenuItem};

// ============================================================================
// VisualSelection Integration Tests
// ============================================================================

#[test]
fn test_visual_selection_creation() {
    let selection = VisualSelection::new();
    assert!(!selection.is_active());
    assert_eq!(selection.mode(), SelectionMode::Character);
}

#[test]
fn test_visual_selection_start() {
    let mut selection = VisualSelection::new();
    selection.start(SelectionMode::Character, Position::new(0, 0));
    assert!(selection.is_active());
    assert_eq!(selection.mode(), SelectionMode::Character);
    assert_eq!(selection.start_pos(), Some(Position::new(0, 0)));
}

#[test]
fn test_visual_selection_update_end() {
    let mut selection = VisualSelection::new();
    selection.start(SelectionMode::Character, Position::new(0, 0));
    selection.update_end(Position::new(0, 10));
    assert_eq!(selection.end_pos(), Some(Position::new(0, 10)));
}

#[test]
fn test_visual_selection_modes() {
    let mut selection = VisualSelection::new();

    // Character mode
    selection.start(SelectionMode::Character, Position::new(0, 0));
    assert_eq!(selection.mode(), SelectionMode::Character);

    // Change to line mode
    selection.change_mode(SelectionMode::Line);
    assert_eq!(selection.mode(), SelectionMode::Line);

    // Change to block mode
    selection.change_mode(SelectionMode::Block);
    assert_eq!(selection.mode(), SelectionMode::Block);
}

#[test]
fn test_visual_selection_get_range() {
    let mut selection = VisualSelection::new();
    selection.start(SelectionMode::Character, Position::new(0, 5));
    selection.update_end(Position::new(0, 15));

    let range = selection.get_range();
    assert!(range.is_some());
    let range = range.unwrap();
    assert_eq!(range.mode, SelectionMode::Character);
    assert_eq!(range.start, Position::new(0, 5));
    assert_eq!(range.end, Position::new(0, 15));
}

#[test]
fn test_visual_selection_is_selected() {
    let mut selection = VisualSelection::new();
    selection.start(SelectionMode::Character, Position::new(0, 5));
    selection.update_end(Position::new(0, 15));

    assert!(selection.is_selected(Position::new(0, 8)));
    assert!(selection.is_selected(Position::new(0, 5)));
    assert!(selection.is_selected(Position::new(0, 15)));
    assert!(!selection.is_selected(Position::new(0, 20)));
}

#[test]
fn test_visual_selection_end() {
    let mut selection = VisualSelection::new();
    selection.start(SelectionMode::Character, Position::new(0, 0));
    selection.update_end(Position::new(0, 10));

    let range = selection.end();
    assert!(range.is_some());
    assert!(!selection.is_active());
}

#[test]
fn test_visual_selection_cancel() {
    let mut selection = VisualSelection::new();
    selection.start(SelectionMode::Line, Position::new(0, 0));
    assert!(selection.is_active());

    selection.cancel();
    assert!(!selection.is_active());
    assert_eq!(selection.start_pos(), None);
    assert_eq!(selection.end_pos(), None);
}

#[test]
fn test_position_comparison() {
    let pos1 = Position::new(0, 5);
    let pos2 = Position::new(0, 10);
    let pos3 = Position::new(1, 0);

    assert!(pos1.is_before(&pos2));
    assert!(pos2.is_after(&pos1));
    assert!(pos1.is_before(&pos3));
    assert!(pos3.is_after(&pos1));
}

#[test]
fn test_selection_mode_names() {
    assert_eq!(SelectionMode::Character.name(), "VISUAL");
    assert_eq!(SelectionMode::Line.name(), "VISUAL LINE");
    assert_eq!(SelectionMode::Block.name(), "VISUAL BLOCK");

    assert_eq!(SelectionMode::Character.short_name(), "v");
    assert_eq!(SelectionMode::Line.short_name(), "V");
    assert_eq!(SelectionMode::Block.short_name(), "^V");
}

// ============================================================================
// Macro Integration Tests
// ============================================================================

#[test]
fn test_macro_creation() {
    let macro_ = Macro::new('a');
    assert_eq!(macro_.register, 'a');
    assert_eq!(macro_.len(), 0);
    assert!(macro_.is_empty());
}

#[test]
fn test_macro_add_action() {
    let mut macro_ = Macro::new('a');
    macro_.add_action(MacroAction::InsertText("hello".to_string()));
    assert_eq!(macro_.len(), 1);
    assert!(!macro_.is_empty());
}

#[test]
fn test_macro_actions() {
    let mut macro_ = Macro::new('a');
    macro_.add_action(MacroAction::InsertText("hello".to_string()));
    macro_.add_action(MacroAction::DeleteText(5));
    macro_.add_action(MacroAction::MoveCursor { line: 1, col: 0 });

    assert_eq!(macro_.len(), 3);
    assert_eq!(macro_.actions().len(), 3);
}

#[test]
fn test_macro_clear() {
    let mut macro_ = Macro::new('a');
    macro_.add_action(MacroAction::InsertText("test".to_string()));
    assert_eq!(macro_.len(), 1);

    macro_.clear();
    assert_eq!(macro_.len(), 0);
    assert!(macro_.is_empty());
}

#[test]
fn test_macro_action_descriptions() {
    let actions = vec![
        MacroAction::InsertText("hello".to_string()),
        MacroAction::DeleteText(5),
        MacroAction::MoveCursor { line: 1, col: 2 },
        MacroAction::EnterMode("insert".to_string()),
        MacroAction::Command("save".to_string()),
        MacroAction::Custom {
            name: "test".to_string(),
            data: "data".to_string(),
        },
    ];

    for action in actions {
        let desc = action.description();
        assert!(!desc.is_empty());
    }
}

#[test]
fn test_macro_manager_creation() {
    let manager = MacroManager::new();
    assert!(!manager.is_recording());
}

#[test]
fn test_macro_manager_start_recording() {
    let mut manager = MacroManager::new();
    assert!(manager.start_recording('a'));
    assert!(manager.is_recording());
}

#[test]
fn test_macro_manager_record_action() {
    let mut manager = MacroManager::new();
    manager.start_recording('a');

    assert!(manager.record_action(MacroAction::InsertText("hello".to_string())));
    assert!(manager.record_action(MacroAction::DeleteText(1)));
}

#[test]
fn test_macro_manager_stop_recording() {
    let mut manager = MacroManager::new();
    manager.start_recording('a');
    manager.record_action(MacroAction::InsertText("test".to_string()));

    assert!(manager.stop_recording());
    assert!(!manager.is_recording());
    assert!(manager.has_macro('a'));
}

#[test]
fn test_macro_manager_get_macro() {
    let mut manager = MacroManager::new();
    manager.start_recording('a');
    manager.record_action(MacroAction::InsertText("hello".to_string()));
    manager.stop_recording();

    let macro_ = manager.get_macro('a');
    assert!(macro_.is_some());
    assert_eq!(macro_.unwrap().len(), 1);
}

#[test]
fn test_macro_manager_invalid_register() {
    let mut manager = MacroManager::new();
    assert!(!manager.start_recording('!'));
    assert!(!manager.start_recording('@'));
}

#[test]
fn test_macro_manager_replay() {
    let mut manager = MacroManager::new();
    manager.start_recording('a');
    manager.record_action(MacroAction::InsertText("test".to_string()));
    manager.stop_recording();

    let macro_ = manager.get_macro('a');
    assert!(macro_.is_some());
    assert_eq!(macro_.unwrap().len(), 1);
}

// ============================================================================
// Marks Integration Tests
// ============================================================================

#[test]
fn test_mark_creation() {
    let mark = Mark::new("file.txt", 10, 5);
    assert_eq!(mark.file, "file.txt");
    assert_eq!(mark.line, 10);
    assert_eq!(mark.col, 5);
}

#[test]
fn test_mark_set_position() {
    let mut mark = Mark::new("file.txt", 10, 5);
    mark.set_position(20, 15);
    assert_eq!(mark.line, 20);
    assert_eq!(mark.col, 15);
}

#[test]
fn test_mark_is_in_file() {
    let mark = Mark::new("file.txt", 10, 5);
    assert!(mark.is_in_file("file.txt"));
    assert!(!mark.is_in_file("other.txt"));
}

#[test]
fn test_mark_type_classify() {
    assert_eq!(MarkType::classify('a'), Some(MarkType::Local));
    assert_eq!(MarkType::classify('z'), Some(MarkType::Local));
    assert_eq!(MarkType::classify('A'), Some(MarkType::Global));
    assert_eq!(MarkType::classify('Z'), Some(MarkType::Global));
    assert_eq!(MarkType::classify('0'), Some(MarkType::Number));
    assert_eq!(MarkType::classify('9'), Some(MarkType::Number));
    assert_eq!(MarkType::classify('\''), Some(MarkType::Special));
    assert_eq!(MarkType::classify('`'), Some(MarkType::Special));
    assert_eq!(MarkType::classify('!'), None);
}

#[test]
fn test_mark_type_is_valid() {
    assert!(MarkType::is_valid_mark('a'));
    assert!(MarkType::is_valid_mark('A'));
    assert!(MarkType::is_valid_mark('0'));
    assert!(MarkType::is_valid_mark('\''));
    assert!(!MarkType::is_valid_mark('!'));
    assert!(!MarkType::is_valid_mark('@'));
}

#[test]
fn test_marks_manager_creation() {
    let manager = MarksManager::new();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_marks_manager_set_mark() {
    let mut manager = MarksManager::new();
    assert!(manager.set_mark('a', "file.txt", 10, 5));
    assert_eq!(manager.count(), 1);
}

#[test]
fn test_marks_manager_get_mark() {
    let mut manager = MarksManager::new();
    manager.set_mark('a', "file.txt", 10, 5);

    let mark = manager.get_mark('a');
    assert!(mark.is_some());
    let mark = mark.unwrap();
    assert_eq!(mark.line, 10);
    assert_eq!(mark.col, 5);
}

#[test]
fn test_marks_manager_delete_mark() {
    let mut manager = MarksManager::new();
    manager.set_mark('a', "file.txt", 10, 5);
    assert!(manager.get_mark('a').is_some());

    assert!(manager.delete_mark('a'));
    assert!(manager.get_mark('a').is_none());
}

#[test]
fn test_marks_manager_current_file() {
    let mut manager = MarksManager::new();
    assert!(manager.current_file().is_none());

    manager.set_current_file("file.txt");
    assert_eq!(manager.current_file(), Some("file.txt"));
}

#[test]
fn test_marks_manager_set_mark_here() {
    let mut manager = MarksManager::new();
    manager.set_current_file("file.txt");

    assert!(manager.set_mark_here('a', 10, 5));
    let mark = manager.get_mark('a').unwrap();
    assert_eq!(mark.file, "file.txt");
}

#[test]
fn test_marks_manager_clear() {
    let mut manager = MarksManager::new();
    manager.set_mark('a', "file.txt", 10, 5);
    manager.set_mark('b', "file.txt", 20, 10);
    assert_eq!(manager.count(), 2);

    manager.clear();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_marks_manager_clear_local_marks() {
    let mut manager = MarksManager::new();
    manager.set_mark('a', "file1.txt", 10, 5); // local
    manager.set_mark('b', "file1.txt", 20, 10); // local
    manager.set_mark('A', "file2.txt", 30, 15); // global

    manager.clear_local_marks("file1.txt");

    assert!(manager.get_mark('a').is_none());
    assert!(manager.get_mark('b').is_none());
    assert!(manager.get_mark('A').is_some()); // global should remain
}

#[test]
fn test_marks_manager_list_marks() {
    let mut manager = MarksManager::new();
    manager.set_mark('a', "file.txt", 10, 5);
    manager.set_mark('b', "file.txt", 20, 10);

    let marks = manager.all_marks();
    assert_eq!(marks.len(), 2);
}

// ============================================================================
// ContextMenu Integration Tests
// ============================================================================

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
    let item = MenuItem::simple("Save").with_icon("");
    assert!(item.is_action());
}

#[test]
fn test_menu_item_with_enabled() {
    let item = MenuItem::simple("Action").with_enabled(false);
    assert!(!item.is_enabled());

    let item = MenuItem::simple("Action").with_enabled(true);
    assert!(item.is_enabled());
}

#[test]
fn test_context_menu_creation() {
    let menu = ContextMenu::new();
    assert_eq!(menu.item_count(), 0);
}

#[test]
fn test_context_menu_add_item() {
    let mut menu = ContextMenu::new();
    menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
    assert_eq!(menu.item_count(), 1);

    menu.add_item(MenuItem::separator());
    assert_eq!(menu.item_count(), 2);
}

#[test]
fn test_context_menu_with_title() {
    let menu = ContextMenu::new().title("Actions");
    assert_eq!(menu.item_count(), 0);
}

#[test]
fn test_context_menu_select() {
    let mut menu = ContextMenu::new();
    menu.add_item(MenuItem::action("Copy", "Ctrl+C"));
    menu.add_item(MenuItem::action("Paste", "Ctrl+V"));

    menu.select_next();
    menu.select_next();
    menu.select_previous();
}

#[test]
fn test_context_menu_builder_pattern() {
    let mut menu = ContextMenu::new().title("File Actions");
    menu.add_item(MenuItem::action("Save", "Ctrl+S").with_icon(""));
    menu.add_item(MenuItem::separator());
    menu.add_item(MenuItem::action("Close", "Ctrl+W"));

    assert_eq!(menu.item_count(), 3);
}

// ============================================================================
// QuickAction Integration Tests
// ============================================================================

#[test]
fn test_quick_action_creation() {
    let action = QuickAction::new("save", "Save file", "Ctrl+S");
    assert_eq!(action.id, "save");
    assert_eq!(action.label, "Save file");
    assert_eq!(action.shortcut, Some("Ctrl+S".to_string()));
    assert!(action.enabled);
}

#[test]
fn test_quick_action_with_category() {
    let action = QuickAction::new("save", "Save", "Ctrl+S").with_category(ActionCategory::File);
    assert_eq!(action.category, ActionCategory::File);
}

#[test]
fn test_quick_action_with_icon() {
    let action = QuickAction::new("save", "Save", "Ctrl+S").with_icon("");
    assert_eq!(action.icon, Some("".to_string()));
}

#[test]
fn test_quick_action_with_enabled() {
    let action = QuickAction::new("save", "Save", "Ctrl+S").with_enabled(false);
    assert!(!action.enabled);
}

#[test]
fn test_quick_action_record_usage() {
    let mut action = QuickAction::new("save", "Save", "Ctrl+S");
    assert_eq!(action.usage_count, 0);

    action.record_usage();
    assert_eq!(action.usage_count, 1);

    action.record_usage();
    assert_eq!(action.usage_count, 2);
}

#[test]
fn test_quick_action_scores() {
    let mut action = QuickAction::new("save", "Save", "Ctrl+S");
    action.record_usage();

    let recency = action.recency_score();
    assert!(recency > 0.0);

    let frequency = action.frequency_score(10);
    assert!(frequency > 0.0);

    let priority = action.priority_score(10);
    assert!(priority > 0.0);
}

#[test]
fn test_quick_action_manager_creation() {
    let manager = QuickActionManager::new();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_quick_action_manager_add_action() {
    let mut manager = QuickActionManager::new();
    manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));
    assert_eq!(manager.count(), 1);
}

#[test]
fn test_quick_action_manager_with_defaults() {
    let manager = QuickActionManager::with_defaults();
    assert!(manager.count() > 0);
}

#[test]
fn test_quick_action_manager_get_action() {
    let mut manager = QuickActionManager::new();
    manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));

    let action = manager.get_action("save");
    assert!(action.is_some());
    assert_eq!(action.unwrap().label, "Save");
}

#[test]
fn test_quick_action_manager_record_usage() {
    let mut manager = QuickActionManager::new();
    manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));

    manager.record_usage("save");
    let action = manager.get_action("save").unwrap();
    assert_eq!(action.usage_count, 1);
}

#[test]
fn test_quick_action_manager_remove_action() {
    let mut manager = QuickActionManager::new();
    manager.add_action(QuickAction::new("save", "Save", "Ctrl+S"));
    assert_eq!(manager.count(), 1);

    manager.remove_action("save");
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_quick_action_manager_get_by_category() {
    let mut manager = QuickActionManager::new();
    manager
        .add_action(QuickAction::new("save", "Save", "Ctrl+S").with_category(ActionCategory::File));
    manager.add_action(
        QuickAction::new("search", "Search", "Ctrl+F").with_category(ActionCategory::Search),
    );

    let file_actions = manager.by_category(ActionCategory::File);
    assert_eq!(file_actions.len(), 1);
}

#[test]
fn test_quick_action_manager_sorted() {
    let mut manager = QuickActionManager::new();
    manager.add_action(QuickAction::new("a", "Action A", ""));
    manager.add_action(QuickAction::new("b", "Action B", ""));
    manager.add_action(QuickAction::new("c", "Action C", ""));

    // Record different usage counts
    manager.record_usage("b");
    manager.record_usage("b");
    manager.record_usage("c");

    let sorted = manager.most_frequent(3);
    assert_eq!(sorted.len(), 3);
    // Most used should be first
    assert_eq!(sorted[0].id, "b");
}

// ============================================================================
// Cross-Feature Integration Tests
// ============================================================================

#[test]
fn test_visual_selection_with_macros() {
    let mut selection = VisualSelection::new();
    let mut macro_mgr = MacroManager::new();

    // Start recording
    macro_mgr.start_recording('a');

    // Start selection
    selection.start(SelectionMode::Character, Position::new(0, 0));
    macro_mgr.record_action(MacroAction::Custom {
        name: "start_selection".to_string(),
        data: "character".to_string(),
    });

    // Update selection
    selection.update_end(Position::new(0, 10));
    macro_mgr.record_action(MacroAction::Custom {
        name: "update_selection".to_string(),
        data: "0,10".to_string(),
    });

    // End selection
    selection.end();
    macro_mgr.record_action(MacroAction::Custom {
        name: "end_selection".to_string(),
        data: "".to_string(),
    });

    macro_mgr.stop_recording();

    let macro_ = macro_mgr.get_macro('a').unwrap();
    assert_eq!(macro_.len(), 3);
}

#[test]
fn test_marks_with_context_menu() {
    let mut marks = MarksManager::new();
    let mut menu = ContextMenu::new().title("Marks");

    // Set some marks
    marks.set_mark('a', "file.txt", 10, 5);
    marks.set_mark('b', "file.txt", 20, 10);

    // Create menu items for marks
    menu.add_item(MenuItem::action("Jump to mark 'a'", "'a"));
    menu.add_item(MenuItem::action("Jump to mark 'b'", "'b"));

    assert_eq!(menu.item_count(), 2);
    assert_eq!(marks.count(), 2);
}

#[test]
fn test_quick_actions_with_macros() {
    let mut actions = QuickActionManager::new();
    let mut macros = MacroManager::new();

    // Add quick action for recording macro
    actions.add_action(QuickAction::new("record_macro", "Record Macro", "q"));

    // Start recording
    macros.start_recording('a');
    actions.record_usage("record_macro");

    // Record some actions
    macros.record_action(MacroAction::InsertText("hello".to_string()));

    macros.stop_recording();

    assert!(macros.has_macro('a'));
    assert_eq!(actions.get_action("record_macro").unwrap().usage_count, 1);
}

#[test]
fn test_complete_modal_editing_workflow() {
    // Setup all systems
    let mut selection = VisualSelection::new();
    let mut macros = MacroManager::new();
    let mut marks = MarksManager::new();

    // Set a mark
    marks.set_mark('a', "file.txt", 0, 0);

    // Start recording macro
    macros.start_recording('q');

    // Start visual selection
    selection.start(SelectionMode::Line, Position::new(5, 0));
    macros.record_action(MacroAction::Custom {
        name: "visual_line".to_string(),
        data: "5,0".to_string(),
    });

    // Expand selection
    selection.update_end(Position::new(10, 0));
    macros.record_action(MacroAction::Custom {
        name: "expand".to_string(),
        data: "10,0".to_string(),
    });

    // Delete selection
    selection.end();
    macros.record_action(MacroAction::DeleteText(5));

    // Set mark at new position
    marks.set_mark('b', "file.txt", 5, 0);
    macros.record_action(MacroAction::Custom {
        name: "set_mark".to_string(),
        data: "b".to_string(),
    });

    // Stop recording
    macros.stop_recording();

    // Verify workflow
    assert!(macros.has_macro('q'));
    assert_eq!(marks.count(), 2);
    let macro_ = macros.get_macro('q').unwrap();
    assert_eq!(macro_.len(), 4);
}
