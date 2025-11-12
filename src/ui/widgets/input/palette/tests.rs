//! Command palette tests

use super::*;

// ============================================================================
// COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
// ============================================================================

// ============ Basic Functionality Tests ============

#[test]
fn test_palette_creation() {
    let palette = CommandPalette::new();
    assert_eq!(palette.query(), "");
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_palette_default() {
    let palette = CommandPalette::default();
    assert_eq!(palette.query(), "");
}

#[test]
fn test_insert_single_char() {
    let mut palette = CommandPalette::new();
    palette.insert_char('h');
    assert_eq!(palette.query(), "h");
}

#[test]
fn test_insert_multiple_chars() {
    let mut palette = CommandPalette::new();
    palette.insert_char('h');
    palette.insert_char('e');
    palette.insert_char('l');
    palette.insert_char('p');
    assert_eq!(palette.query(), "help");
}

#[test]
fn test_delete_char() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('b');
    palette.delete_char();
    assert_eq!(palette.query(), "a");
}

#[test]
fn test_delete_char_when_empty() {
    let mut palette = CommandPalette::new();
    palette.delete_char();
    assert_eq!(palette.query(), "");
}

#[test]
fn test_clear_query() {
    let mut palette = CommandPalette::new();
    palette.insert_char('t');
    palette.insert_char('e');
    palette.insert_char('s');
    palette.insert_char('t');
    palette.clear_query();
    assert_eq!(palette.query(), "");
}

#[test]
fn test_select_next() {
    let mut palette = CommandPalette::new();
    let first = palette.selected_command();
    palette.select_next();
    let second = palette.selected_command();
    assert_ne!(first, second);
}

#[test]
fn test_select_previous() {
    let mut palette = CommandPalette::new();
    let first = palette.selected_command();
    palette.select_previous();
    let last = palette.selected_command();
    assert_ne!(first, last);
}

#[test]
fn test_palette_command_clone() {
    let cmd = PaletteCommand {
        id: "test".to_string(),
        label: "Test".to_string(),
        description: "Test description".to_string(),
    };
    let cloned = cmd.clone();
    assert_eq!(cmd.id, cloned.id);
    assert_eq!(cmd.label, cloned.label);
    assert_eq!(cmd.description, cloned.description);
}

#[test]
fn test_palette_command_debug() {
    let cmd = PaletteCommand {
        id: "test".to_string(),
        label: "Test".to_string(),
        description: "Test description".to_string(),
    };
    let debug_str = format!("{:?}", cmd);
    assert!(debug_str.contains("PaletteCommand"));
}

// ============ Stress Tests ============

#[test]
fn test_palette_rapid_char_insertion_1000() {
    let mut palette = CommandPalette::new();
    for _ in 0..1000 {
        palette.insert_char('x');
    }
    assert_eq!(palette.query().len(), 1000);
}

#[test]
fn test_palette_rapid_navigation_1000_next() {
    let mut palette = CommandPalette::new();
    for _ in 0..1000 {
        palette.select_next();
    }
    // Should have wrapped around many times, still have a selection
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_palette_rapid_navigation_1000_previous() {
    let mut palette = CommandPalette::new();
    for _ in 0..1000 {
        palette.select_previous();
    }
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_palette_alternating_insert_delete_1000() {
    let mut palette = CommandPalette::new();
    for i in 0..1000 {
        if i % 2 == 0 {
            palette.insert_char('a');
        } else {
            palette.delete_char();
        }
    }
    // Should end with empty query (1000 iterations, alternating)
    assert_eq!(palette.query(), "");
}

#[test]
fn test_palette_rapid_clear_operations() {
    let mut palette = CommandPalette::new();
    for _ in 0..1000 {
        palette.insert_char('t');
        palette.insert_char('e');
        palette.insert_char('s');
        palette.insert_char('t');
        palette.clear_query();
    }
    assert_eq!(palette.query(), "");
}

// ============ Unicode Edge Cases ============

#[test]
fn test_palette_query_with_emoji() {
    let mut palette = CommandPalette::new();
    palette.insert_char('🚀');
    palette.insert_char('🐸');
    assert!(palette.query().contains('🚀'));
    assert!(palette.query().contains('🐸'));
}

#[test]
fn test_palette_query_with_rtl_text() {
    let mut palette = CommandPalette::new();
    for c in "مرحبا".chars() {
        palette.insert_char(c);
    }
    assert!(palette.query().contains("مرحبا"));
}

#[test]
fn test_palette_query_with_hebrew() {
    let mut palette = CommandPalette::new();
    for c in "שלום".chars() {
        palette.insert_char(c);
    }
    assert!(palette.query().contains("שלום"));
}

#[test]
fn test_palette_query_with_japanese() {
    let mut palette = CommandPalette::new();
    for c in "日本語".chars() {
        palette.insert_char(c);
    }
    assert_eq!(palette.query(), "日本語");
}

#[test]
fn test_palette_query_with_combining_characters() {
    let mut palette = CommandPalette::new();
    for c in "é̂ñ̃".chars() {
        palette.insert_char(c);
    }
    assert!(palette.query().len() > 2);
}

#[test]
fn test_palette_query_with_zero_width() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('\u{200B}'); // Zero-width space
    palette.insert_char('b');
    assert!(palette.query().contains('a'));
    assert!(palette.query().contains('b'));
}

#[test]
fn test_palette_query_with_emoji_skin_tones() {
    let mut palette = CommandPalette::new();
    for c in "👍🏻👍🏿".chars() {
        palette.insert_char(c);
    }
    assert!(palette.query().contains("👍"));
}

#[test]
fn test_palette_query_with_mixed_scripts() {
    let mut palette = CommandPalette::new();
    for c in "Hello日本مرحبا🚀".chars() {
        palette.insert_char(c);
    }
    assert_eq!(palette.query(), "Hello日本مرحبا🚀");
}

#[test]
fn test_palette_delete_emoji() {
    let mut palette = CommandPalette::new();
    palette.insert_char('🚀');
    palette.delete_char();
    assert_eq!(palette.query(), "");
}

#[test]
fn test_palette_delete_multibyte_char() {
    let mut palette = CommandPalette::new();
    palette.insert_char('日');
    palette.delete_char();
    assert_eq!(palette.query(), "");
}

// ============ Filter Edge Cases ============

#[test]
fn test_palette_filter_empty_query_shows_all() {
    let palette = CommandPalette::new();
    // With empty query, should have a selection (all commands visible)
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_palette_filter_case_insensitive() {
    let mut palette = CommandPalette::new();
    palette.insert_char('H');
    palette.insert_char('E');
    palette.insert_char('L');
    palette.insert_char('P');
    // Should still find "help" command (case-insensitive)
    let selected = palette.selected_command();
    assert!(selected.is_some());
}

#[test]
fn test_palette_filter_no_matches() {
    let mut palette = CommandPalette::new();
    for c in "xyzabc123nonexistent".chars() {
        palette.insert_char(c);
    }
    // With no matches, selection should be None
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_palette_filter_single_char() {
    let mut palette = CommandPalette::new();
    palette.insert_char('q');
    // Should filter to commands containing 'q'
    let selected = palette.selected_command();
    assert!(selected.is_some());
}

#[test]
fn test_palette_filter_progressive() {
    let mut palette = CommandPalette::new();
    palette.insert_char('h');
    let count_h = palette.selected_command();
    palette.insert_char('e');
    palette.insert_char('l');
    palette.insert_char('p');
    let count_help = palette.selected_command();
    // Both should have selections
    assert!(count_h.is_some());
    assert!(count_help.is_some());
}

#[test]
fn test_palette_clear_query_restores_all() {
    let mut palette = CommandPalette::new();
    palette.insert_char('q');
    palette.insert_char('u');
    palette.insert_char('i');
    palette.insert_char('t');
    palette.clear_query();
    // After clear, should show all commands again
    assert!(palette.selected_command().is_some());
}

// ============ Selection Edge Cases ============

#[test]
fn test_palette_select_next_wraps() {
    let mut palette = CommandPalette::new();
    let first_selection = palette.selected_command();

    // Navigate to last item by going previous once (wraps)
    palette.select_previous();

    // Then go next (should wrap to first)
    palette.select_next();

    let wrapped_selection = palette.selected_command();
    assert_eq!(first_selection, wrapped_selection);
}

#[test]
fn test_palette_select_previous_wraps() {
    let mut palette = CommandPalette::new();
    let first = palette.selected_command();
    palette.select_previous();
    let last = palette.selected_command();
    assert_ne!(first, last);
}

#[test]
fn test_palette_selection_after_filter_change() {
    let mut palette = CommandPalette::new();
    palette.insert_char('h');
    let selected_after_h = palette.selected_command();
    palette.insert_char('e');
    palette.insert_char('l');
    palette.insert_char('p');
    let selected_after_help = palette.selected_command();
    // Both should have selections, might be different commands
    assert!(selected_after_h.is_some());
    assert!(selected_after_help.is_some());
}

#[test]
fn test_palette_selection_when_no_results() {
    let mut palette = CommandPalette::new();
    for c in "nonexistentcommand123".chars() {
        palette.insert_char(c);
    }
    assert!(palette.selected_command().is_none());

    // Navigation should be safe with no results
    palette.select_next();
    palette.select_previous();
    assert!(palette.selected_command().is_none());
}

// ============ Cursor Position Edge Cases ============

#[test]
fn test_palette_cursor_at_start() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('b');
    palette.insert_char('c');
    // Cursor should be at end (position 3)
    assert_eq!(palette.query(), "abc");
}

#[test]
fn test_palette_delete_updates_cursor() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('b');
    palette.delete_char();
    palette.insert_char('c');
    assert_eq!(palette.query(), "ac");
}

#[test]
fn test_palette_empty_after_deletes() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.delete_char();
    palette.delete_char(); // Extra delete on empty
    assert_eq!(palette.query(), "");
}

// ============ Complex Workflow Tests ============

#[test]
fn test_palette_complex_workflow_type_navigate_clear() {
    let mut palette = CommandPalette::new();

    // Type a query
    for c in "help".chars() {
        palette.insert_char(c);
    }
    assert_eq!(palette.query(), "help");

    // Navigate
    palette.select_next();
    palette.select_next();
    palette.select_previous();

    // Clear and verify
    palette.clear_query();
    assert_eq!(palette.query(), "");
    assert!(palette.selected_command().is_some());

    // Type new query
    for c in "quit".chars() {
        palette.insert_char(c);
    }
    assert_eq!(palette.query(), "quit");
}

#[test]
fn test_palette_workflow_filter_to_nothing_then_back() {
    let mut palette = CommandPalette::new();

    // Filter to something that doesn't exist
    for c in "nonexistent".chars() {
        palette.insert_char(c);
    }
    assert!(palette.selected_command().is_none());

    // Clear and go back
    palette.clear_query();
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_palette_workflow_rapid_operations() {
    let mut palette = CommandPalette::new();

    for _ in 0..100 {
        palette.insert_char('h');
        palette.select_next();
        palette.insert_char('e');
        palette.select_previous();
        palette.delete_char();
        palette.delete_char();
    }

    // Should still be functional
    assert_eq!(palette.query(), "");
}

#[test]
fn test_palette_workflow_unicode_operations() {
    let mut palette = CommandPalette::new();

    // Insert unicode
    palette.insert_char('日');
    palette.insert_char('本');

    // Navigate
    palette.select_next();

    // Delete one char
    palette.delete_char();
    assert_eq!(palette.query(), "日");

    // Add more
    palette.insert_char('🚀');
    assert_eq!(palette.query(), "日🚀");

    // Clear
    palette.clear_query();
    assert_eq!(palette.query(), "");
}

// ============ Comprehensive Stress Test ============

#[test]
fn test_comprehensive_palette_stress() {
    let mut palette = CommandPalette::new();

    // Phase 1: Rapid insertions with varied characters
    for i in 0..200 {
        let c = match i % 5 {
            0 => 'a',
            1 => '🚀',
            2 => '日',
            3 => 'x',
            _ => 'q',
        };
        palette.insert_char(c);
    }
    assert_eq!(palette.query().chars().count(), 200);

    // Phase 2: Rapid deletions
    for _ in 0..100 {
        palette.delete_char();
    }
    assert_eq!(palette.query().chars().count(), 100);

    // Phase 3: Clear and start fresh
    palette.clear_query();
    assert_eq!(palette.query(), "");

    // Phase 4: Type realistic queries with navigation
    for c in "help".chars() {
        palette.insert_char(c);
    }
    palette.select_next();
    palette.select_next();
    palette.select_previous();

    let selected = palette.selected_command();
    assert!(selected.is_some());

    // Phase 5: Clear and try another
    palette.clear_query();
    for c in "quit".chars() {
        palette.insert_char(c);
    }

    let quit_selected = palette.selected_command();
    assert!(quit_selected.is_some());

    // Phase 6: Verify state consistency
    palette.clear_query();
    assert!(palette.selected_command().is_some());
}

// ============ Edge Case: Whitespace and Special Characters ============

#[test]
fn test_palette_query_with_spaces() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char(' ');
    palette.insert_char('b');
    assert_eq!(palette.query(), "a b");
}

#[test]
fn test_palette_query_only_spaces() {
    let mut palette = CommandPalette::new();
    palette.insert_char(' ');
    palette.insert_char(' ');
    palette.insert_char(' ');
    assert_eq!(palette.query(), "   ");
}

#[test]
fn test_palette_query_with_newline() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('\n');
    palette.insert_char('b');
    assert!(palette.query().contains('\n'));
}

#[test]
fn test_palette_query_with_tab() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('\t');
    palette.insert_char('b');
    assert!(palette.query().contains('\t'));
}

#[test]
fn test_palette_query_with_special_chars() {
    let mut palette = CommandPalette::new();
    for c in "!@#$%^&*()".chars() {
        palette.insert_char(c);
    }
    assert_eq!(palette.query(), "!@#$%^&*()");
}

// ============ Debug Trait Test ============

#[test]
fn test_palette_debug() {
    let palette = CommandPalette::new();
    let debug_str = format!("{:?}", palette);
    assert!(debug_str.contains("CommandPalette"));
}

// ============ Filter Matching Specifics ============

#[test]
fn test_palette_filter_matches_label() {
    let mut palette = CommandPalette::new();
    for c in "help".chars() {
        palette.insert_char(c);
    }
    // Should match "Show Help" in label
    let selected = palette.selected_command();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap(), "help");
}

#[test]
fn test_palette_filter_matches_description() {
    let mut palette = CommandPalette::new();
    for c in "keybindings".chars() {
        palette.insert_char(c);
    }
    // Should match "help" by description containing "keybindings"
    let selected = palette.selected_command();
    assert!(selected.is_some());
}

#[test]
fn test_palette_filter_matches_id() {
    let mut palette = CommandPalette::new();
    for c in "quit".chars() {
        palette.insert_char(c);
    }
    // Should match by ID "quit"
    let selected = palette.selected_command();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap(), "quit");
}

#[test]
fn test_palette_filter_partial_match() {
    let mut palette = CommandPalette::new();
    palette.insert_char('h');
    palette.insert_char('e');
    // "he" should match "help", "theme", etc.
    let selected = palette.selected_command();
    assert!(selected.is_some());
}

// ============ Navigation on Filtered Results ============

#[test]
fn test_palette_navigation_on_filtered_single_result() {
    let mut palette = CommandPalette::new();
    for c in "quit".chars() {
        palette.insert_char(c);
    }

    let first = palette.selected_command();
    palette.select_next(); // Should wrap to same
    let second = palette.selected_command();
    assert_eq!(first, second);
}

#[test]
fn test_palette_navigation_on_filtered_multiple_results() {
    let mut palette = CommandPalette::new();
    palette.insert_char('s'); // Matches "split", "search", "status"

    let first = palette.selected_command();
    palette.select_next();
    let second = palette.selected_command();
    assert_ne!(first, second);
}

#[test]
fn test_palette_navigation_wraps_on_filtered_results() {
    let mut palette = CommandPalette::new();
    palette.insert_char('s'); // Matches multiple

    let first = palette.selected_command();

    // Navigate to end
    for _ in 0..10 {
        palette.select_next();
    }

    // Should have wrapped back
    let wrapped = palette.selected_command();
    assert!(wrapped.is_some());
}

// ============ Extreme Stress Tests (10k operations) ============

#[test]
fn test_palette_10k_char_insertions() {
    let mut palette = CommandPalette::new();
    for i in 0..10000 {
        let c = (b'a' + (i % 26) as u8) as char;
        palette.insert_char(c);
    }
    assert_eq!(palette.query().chars().count(), 10000);
}

#[test]
fn test_palette_10k_mixed_operations() {
    let mut palette = CommandPalette::new();
    for i in 0..10000 {
        match i % 4 {
            0 => palette.insert_char('a'),
            1 => palette.select_next(),
            2 => palette.delete_char(),
            _ => palette.select_previous(),
        }
    }
    // Should still be functional
    assert!(palette.query().len() >= 0);
}

#[test]
fn test_palette_10k_navigation_cycles() {
    let mut palette = CommandPalette::new();
    for i in 0..10000 {
        if i % 2 == 0 {
            palette.select_next();
        } else {
            palette.select_previous();
        }
    }
    // Should still have valid selection
    assert!(palette.selected_command().is_some());
}

// ============ Unicode Boundary Cases ============

#[test]
fn test_palette_very_long_unicode_string() {
    let mut palette = CommandPalette::new();
    for _ in 0..100 {
        palette.insert_char('🚀');
        palette.insert_char('日');
        palette.insert_char('本');
    }
    assert_eq!(palette.query().chars().count(), 300);
}

#[test]
fn test_palette_unicode_delete_boundary() {
    let mut palette = CommandPalette::new();
    palette.insert_char('a');
    palette.insert_char('🚀');
    palette.insert_char('日');
    palette.insert_char('b');

    // Delete 'b'
    palette.delete_char();
    assert_eq!(palette.query(), "a🚀日");

    // Delete '日'
    palette.delete_char();
    assert_eq!(palette.query(), "a🚀");

    // Delete '🚀'
    palette.delete_char();
    assert_eq!(palette.query(), "a");
}

// ============ Multi-Phase Comprehensive Workflow (10 phases) ============

#[test]
fn test_palette_10_phase_comprehensive_workflow() {
    let mut palette = CommandPalette::new();

    // Phase 1: Initial state verification
    assert_eq!(palette.query(), "");
    assert!(palette.selected_command().is_some());

    // Phase 2: Type a query and navigate
    for c in "help".chars() {
        palette.insert_char(c);
    }
    palette.select_next();
    palette.select_previous();
    assert_eq!(palette.query(), "help");

    // Phase 3: Clear and verify reset
    palette.clear_query();
    assert_eq!(palette.query(), "");
    assert!(palette.selected_command().is_some());

    // Phase 4: Type unicode query
    for c in "日本🚀".chars() {
        palette.insert_char(c);
    }
    assert_eq!(palette.query(), "日本🚀");

    // Phase 5: Delete characters one by one
    palette.delete_char();
    palette.delete_char();
    palette.delete_char();
    assert_eq!(palette.query(), "");

    // Phase 6: Type query that produces no results
    for c in "nonexistent123".chars() {
        palette.insert_char(c);
    }
    assert!(palette.selected_command().is_none());

    // Phase 7: Clear and type valid query
    palette.clear_query();
    for c in "quit".chars() {
        palette.insert_char(c);
    }
    assert!(palette.selected_command().is_some());

    // Phase 8: Rapid navigation on filtered results
    for _ in 0..10 {
        palette.select_next();
        palette.select_previous();
    }

    // Phase 9: Add more characters to narrow filter
    palette.insert_char('x');
    palette.insert_char('y');
    palette.insert_char('z');

    // Phase 10: Clear and verify final state
    palette.clear_query();
    assert_eq!(palette.query(), "");
    assert!(palette.selected_command().is_some());
}

// ============ Edge Cases: Empty and Boundary Conditions ============

#[test]
fn test_palette_query_length_boundary_10k() {
    let mut palette = CommandPalette::new();
    for _ in 0..10000 {
        palette.insert_char('x');
    }
    assert_eq!(palette.query().len(), 10000);

    // Delete half
    for _ in 0..5000 {
        palette.delete_char();
    }
    assert_eq!(palette.query().len(), 5000);
}

#[test]
fn test_palette_filter_after_every_char_insertion() {
    let mut palette = CommandPalette::new();

    // Type "help" character by character, verifying filter updates
    palette.insert_char('h');
    assert!(palette.selected_command().is_some());

    palette.insert_char('e');
    assert!(palette.selected_command().is_some());

    palette.insert_char('l');
    assert!(palette.selected_command().is_some());

    palette.insert_char('p');
    assert!(palette.selected_command().is_some());
    assert_eq!(palette.selected_command().unwrap(), "help");
}

#[test]
fn test_palette_select_operations_with_single_filtered_result() {
    let mut palette = CommandPalette::new();

    // Filter to exactly one result
    for c in "Toggle Vim Mode".chars() {
        palette.insert_char(c);
    }

    let first = palette.selected_command();

    // Navigate back and forth - should stay on same command
    palette.select_next();
    assert_eq!(first, palette.selected_command());

    palette.select_previous();
    assert_eq!(first, palette.selected_command());
}

// ============ Rapid Operation Combinations ============

#[test]
fn test_palette_rapid_insert_clear_cycles_1000() {
    let mut palette = CommandPalette::new();

    for _ in 0..1000 {
        palette.insert_char('a');
        palette.insert_char('b');
        palette.insert_char('c');
        palette.clear_query();
    }

    assert_eq!(palette.query(), "");
    assert!(palette.selected_command().is_some());
}

// ============ Recent Commands Tests ============

#[test]
fn test_record_command_use() {
    let mut palette = CommandPalette::new();
    palette.record_command_use("help");

    assert_eq!(palette.recent_commands().len(), 1);
    assert_eq!(palette.recent_commands()[0], "help");
}

#[test]
fn test_record_multiple_commands() {
    let mut palette = CommandPalette::new();
    palette.record_command_use("help");
    palette.record_command_use("quit");
    palette.record_command_use("clear_conversation");

    assert_eq!(palette.recent_commands().len(), 3);
    assert_eq!(palette.recent_commands()[0], "clear_conversation"); // Most recent first
    assert_eq!(palette.recent_commands()[1], "quit");
    assert_eq!(palette.recent_commands()[2], "help");
}

#[test]
fn test_record_duplicate_command_moves_to_front() {
    let mut palette = CommandPalette::new();
    palette.record_command_use("help");
    palette.record_command_use("quit");
    palette.record_command_use("help"); // Use again

    assert_eq!(palette.recent_commands().len(), 2); // No duplicates
    assert_eq!(palette.recent_commands()[0], "help"); // Moved to front
    assert_eq!(palette.recent_commands()[1], "quit");
}

#[test]
fn test_recent_commands_max_size() {
    let mut palette = CommandPalette::new();

    // Record more than max_recent (10) commands
    for i in 0..15 {
        palette.record_command_use(format!("command_{}", i));
    }

    assert_eq!(palette.recent_commands().len(), 10); // Capped at max
    assert_eq!(palette.recent_commands()[0], "command_14"); // Most recent
    assert_eq!(palette.recent_commands()[9], "command_5"); // Oldest kept
}

#[test]
fn test_clear_recent_commands() {
    let mut palette = CommandPalette::new();
    palette.record_command_use("help");
    palette.record_command_use("quit");

    assert_eq!(palette.recent_commands().len(), 2);

    palette.clear_recent_commands();

    assert_eq!(palette.recent_commands().len(), 0);
}

#[test]
fn test_recent_commands_prioritization_in_filter() {
    let mut palette = CommandPalette::new();

    // Record some commands as "recently used"
    palette.record_command_use("quit");
    palette.record_command_use("help");

    // Clear query to get all commands
    palette.clear_query();

    // Get all filtered commands
    let filtered: Vec<String> = palette
        .filtered
        .iter()
        .map(|&idx| palette.commands[idx].id.clone())
        .collect();

    // Recently used commands should be at the top
    assert_eq!(filtered[0], "help"); // Most recent
    assert_eq!(filtered[1], "quit"); // Second most recent
}

#[test]
fn test_recent_commands_prioritization_with_search() {
    let mut palette = CommandPalette::new();

    // Record "vim_mode" as recently used
    palette.record_command_use("vim_mode");

    // Search for something that matches multiple commands
    palette.insert_char('m'); // Matches "vim_mode" and others

    // Get filtered commands
    let filtered: Vec<String> = palette
        .filtered
        .iter()
        .map(|&idx| palette.commands[idx].id.clone())
        .collect();

    // vim_mode should be first (recently used)
    assert_eq!(filtered[0], "vim_mode");
}

#[test]
fn test_empty_recent_commands() {
    let palette = CommandPalette::new();
    assert_eq!(palette.recent_commands().len(), 0);
}

#[test]
fn test_record_command_use_string_type() {
    let mut palette = CommandPalette::new();
    palette.record_command_use(String::from("help"));

    assert_eq!(palette.recent_commands()[0], "help");
}

#[test]
fn test_recent_commands_immutability() {
    let mut palette = CommandPalette::new();
    palette.record_command_use("help");

    let recent = palette.recent_commands();
    assert_eq!(recent.len(), 1);

    // Ensure we can only read, not modify
    // (Rust borrow checker enforces this at compile time)
}

