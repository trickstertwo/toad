//! Input prompt tests

    use super::*;

    #[test]
fn test_input_prompt_creation() {
    let prompt = InputPrompt::new("Test Title", "Enter something");
    assert_eq!(prompt.value(), "");
    assert!(prompt.is_focused);
}

#[test]
fn test_input_prompt_placeholder() {
    let prompt = InputPrompt::new("Title", "Message").with_placeholder("Custom placeholder");
    assert_eq!(prompt.placeholder, "Custom placeholder");
}

#[test]
fn test_input_prompt_insert_char() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.insert_char('H');
    prompt.insert_char('i');
    assert_eq!(prompt.value(), "Hi");
}

#[test]
fn test_input_prompt_delete_char() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Hello".to_string());
    prompt.delete_char();
    assert_eq!(prompt.value(), "Hell");
}

#[test]
fn test_input_prompt_cursor_movement() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Test".to_string());

    assert_eq!(prompt.cursor_position, 4);

    prompt.move_cursor_left();
    assert_eq!(prompt.cursor_position, 3);

    prompt.move_cursor_start();
    assert_eq!(prompt.cursor_position, 0);

    prompt.move_cursor_end();
    assert_eq!(prompt.cursor_position, 4);

    prompt.move_cursor_right();
    assert_eq!(prompt.cursor_position, 4); // Can't move past end
}

#[test]
fn test_input_prompt_clear() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Test".to_string());
    assert_eq!(prompt.value(), "Test");

    prompt.clear();
    assert_eq!(prompt.value(), "");
    assert_eq!(prompt.cursor_position, 0);
}

#[test]
fn test_input_prompt_set_focused() {
    let mut prompt = InputPrompt::new("Title", "Message");
    assert!(prompt.is_focused);

    prompt.set_focused(false);
    assert!(!prompt.is_focused);
}

// ============================================================================
// COMPREHENSIVE EDGE CASE TESTS (ADVANCED Tier Coverage)
// ============================================================================

// ------------------------------------------------------------------------
// Stress Tests - Large inputs and many operations
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_1000_characters() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let text = "a".repeat(1000);
    prompt.set_value(text.clone());
    assert_eq!(prompt.value(), text);
    assert_eq!(prompt.cursor_position(), 1000);
}

#[test]
fn test_input_prompt_10000_insertions() {
    let mut prompt = InputPrompt::new("Title", "Message");
    for _ in 0..10000 {
        prompt.insert_char('x');
    }
    assert_eq!(prompt.value().len(), 10000);
    assert_eq!(prompt.cursor_position(), 10000);
}

#[test]
fn test_input_prompt_10000_deletions() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let text = "x".repeat(10000);
    prompt.set_value(text);

    for _ in 0..10000 {
        prompt.delete_char();
    }

    assert_eq!(prompt.value(), "");
    assert_eq!(prompt.cursor_position(), 0);
}

#[test]
fn test_input_prompt_rapid_cursor_movement() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("test".to_string());

    // Move cursor 1000 times in various directions
    for _ in 0..250 {
        prompt.move_cursor_start();
        prompt.move_cursor_end();
        prompt.move_cursor_left();
        prompt.move_cursor_right();
    }

    // Should still be valid
    assert_eq!(prompt.value(), "test");
}

#[test]
fn test_input_prompt_alternating_insert_delete_1000_times() {
    let mut prompt = InputPrompt::new("Title", "Message");

    for i in 0..1000 {
        prompt.insert_char('a');
        if i % 2 == 0 {
            prompt.delete_char();
        }
    }

    // Should have 500 characters (1000 inserts - 500 deletes)
    assert_eq!(prompt.value().len(), 500);
}

// ------------------------------------------------------------------------
// Unicode Tests - RTL, emoji, Japanese, mixed scripts, combining chars
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_rtl_arabic() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let arabic = "مرحبا بك";
    prompt.set_value(arabic.to_string());
    assert_eq!(prompt.value(), arabic);
    assert_eq!(prompt.cursor_position(), arabic.len());
}

#[test]
fn test_input_prompt_rtl_hebrew() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let hebrew = "שלום עולם";
    prompt.set_value(hebrew.to_string());
    assert_eq!(prompt.value(), hebrew);
    assert_eq!(prompt.cursor_position(), hebrew.len());
}

#[test]
fn test_input_prompt_emoji() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.insert_char('😀');
    prompt.insert_char('🎉');
    prompt.insert_char('👍');
    assert_eq!(prompt.value(), "😀🎉👍");

    // Delete one emoji
    prompt.delete_char();
    assert_eq!(prompt.value(), "😀🎉");
}

#[test]
fn test_input_prompt_japanese() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let japanese = "こんにちは世界";
    prompt.set_value(japanese.to_string());
    assert_eq!(prompt.value(), japanese);

    // Move cursor and insert
    prompt.move_cursor_start();
    prompt.insert_char('新');
    assert!(prompt.value().starts_with('新'));
}

#[test]
fn test_input_prompt_mixed_scripts() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let mixed = "Hello مرحبا こんにちは 🎉";
    prompt.set_value(mixed.to_string());
    assert_eq!(prompt.value(), mixed);

    // Navigate through mixed scripts
    prompt.move_cursor_start();
    for _ in 0..5 {
        prompt.move_cursor_right();
    }
    prompt.insert_char('!');
    assert!(prompt.value().contains('!'));
}

#[test]
fn test_input_prompt_combining_characters() {
    let mut prompt = InputPrompt::new("Title", "Message");
    // "é" as 'e' + combining acute accent
    let combining = "e\u{0301}";
    prompt.set_value(combining.to_string());
    assert_eq!(prompt.value(), combining);
}

#[test]
fn test_input_prompt_zero_width_characters() {
    let mut prompt = InputPrompt::new("Title", "Message");
    // Zero-width joiner
    let zwj = "a\u{200D}b";
    prompt.set_value(zwj.to_string());
    assert_eq!(prompt.value(), zwj);
}

// ------------------------------------------------------------------------
// Extreme Values Tests - Very long strings, boundary conditions
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_100k_character_string() {
    let mut prompt = InputPrompt::new("Title", "Message");
    let huge_text = "x".repeat(100_000);
    prompt.set_value(huge_text.clone());
    assert_eq!(prompt.value().len(), 100_000);
    assert_eq!(prompt.cursor_position(), 100_000);
}

#[test]
fn test_input_prompt_delete_on_empty() {
    let mut prompt = InputPrompt::new("Title", "Message");
    // Delete on empty should not panic
    prompt.delete_char();
    assert_eq!(prompt.value(), "");
    assert_eq!(prompt.cursor_position(), 0);
}

#[test]
fn test_input_prompt_cursor_left_at_start() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("test".to_string());
    prompt.move_cursor_start();

    // Try to move left from position 0
    prompt.move_cursor_left();
    assert_eq!(prompt.cursor_position(), 0);
}

#[test]
fn test_input_prompt_cursor_right_at_end() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("test".to_string());

    // Already at end, try to move right
    prompt.move_cursor_right();
    assert_eq!(prompt.cursor_position(), 4);
}

#[test]
fn test_input_prompt_very_long_title_and_message() {
    let long_title = "T".repeat(1000);
    let long_message = "M".repeat(1000);
    let prompt = InputPrompt::new(long_title.clone(), long_message.clone());
    assert_eq!(prompt.title, long_title);
    assert_eq!(prompt.message, long_message);
}

// ------------------------------------------------------------------------
// Cursor Navigation and Editing Edge Cases
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_insert_at_middle() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Hello".to_string());
    prompt.move_cursor_start();
    prompt.move_cursor_right();
    prompt.move_cursor_right();

    // Cursor at position 2 (between 'e' and 'l')
    prompt.insert_char('X');
    assert_eq!(prompt.value(), "HeXllo");
}

#[test]
fn test_input_prompt_delete_at_middle() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Hello".to_string());
    prompt.move_cursor_start();
    prompt.move_cursor_right();
    prompt.move_cursor_right();
    prompt.move_cursor_right();

    // Cursor at position 3, delete 'l'
    prompt.delete_char();
    assert_eq!(prompt.value(), "Helo");
}

#[test]
fn test_input_prompt_multiple_cursor_movements() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("abcdefgh".to_string());

    // Complex navigation
    prompt.move_cursor_start();
    assert_eq!(prompt.cursor_position(), 0);

    for _ in 0..3 {
        prompt.move_cursor_right();
    }
    assert_eq!(prompt.cursor_position(), 3);

    for _ in 0..2 {
        prompt.move_cursor_left();
    }
    assert_eq!(prompt.cursor_position(), 1);

    prompt.move_cursor_end();
    assert_eq!(prompt.cursor_position(), 8);
}

#[test]
fn test_input_prompt_insert_unicode_at_different_positions() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Hello".to_string());

    // Insert emoji at start
    prompt.move_cursor_start();
    prompt.insert_char('😀');
    assert_eq!(prompt.value(), "😀Hello");

    // Insert emoji at end
    prompt.move_cursor_end();
    prompt.insert_char('🎉');
    assert_eq!(prompt.value(), "😀Hello🎉");
}

#[test]
fn test_input_prompt_navigation_wraparound_prevention() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("test".to_string());

    // Try to move right past end many times
    for _ in 0..100 {
        prompt.move_cursor_right();
    }
    assert_eq!(prompt.cursor_position(), 4);

    // Try to move left past start many times
    for _ in 0..100 {
        prompt.move_cursor_left();
    }
    assert_eq!(prompt.cursor_position(), 0);
}

// ------------------------------------------------------------------------
// Builder Pattern Tests
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_builder_chaining() {
    let prompt = InputPrompt::new("Title", "Message")
        .with_placeholder("Enter email...");

    assert_eq!(prompt.placeholder, "Enter email...");
    assert_eq!(prompt.title, "Title");
    assert_eq!(prompt.message, "Message");
}

#[test]
fn test_input_prompt_builder_empty_placeholder() {
    let prompt = InputPrompt::new("Title", "Message").with_placeholder("");
    assert_eq!(prompt.placeholder, "");
}

#[test]
fn test_input_prompt_builder_unicode_placeholder() {
    let prompt = InputPrompt::new("Title", "Message")
        .with_placeholder("输入文本... 😀");
    assert_eq!(prompt.placeholder, "输入文本... 😀");
}

// ------------------------------------------------------------------------
// Complex Workflows - Multi-phase editing scenarios
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_complex_editing_workflow() {
    let mut prompt = InputPrompt::new("Edit File", "Enter filename:");

    // Phase 1: Initial typing
    prompt.insert_char('f');
    prompt.insert_char('i');
    prompt.insert_char('l');
    prompt.insert_char('e');
    assert_eq!(prompt.value(), "file");

    // Phase 2: Add extension
    prompt.insert_char('.');
    prompt.insert_char('t');
    prompt.insert_char('x');
    prompt.insert_char('t');
    assert_eq!(prompt.value(), "file.txt");

    // Phase 3: Realize mistake, go back and fix
    prompt.move_cursor_start();
    for _ in 0..5 {
        prompt.move_cursor_right();
    }
    prompt.delete_char(); // Delete '.' (backspace deletes char BEFORE cursor)
    assert_eq!(prompt.value(), "filetxt");

    // Phase 4: Re-insert period at correct position
    prompt.insert_char('.');
    assert_eq!(prompt.value(), "file.txt");

    // Phase 5: Change extension
    prompt.move_cursor_end();
    for _ in 0..3 {
        prompt.delete_char();
    }
    assert_eq!(prompt.value(), "file.");

    // Phase 6: Add new extension
    prompt.insert_char('r');
    prompt.insert_char('s');
    assert_eq!(prompt.value(), "file.rs");

    // Phase 7: Add path prefix
    prompt.move_cursor_start();
    prompt.insert_char('s');
    prompt.insert_char('r');
    prompt.insert_char('c');
    prompt.insert_char('/');
    assert_eq!(prompt.value(), "src/file.rs");

    // Phase 8: Test focus toggle
    prompt.set_focused(false);
    assert!(!prompt.is_focused());
    prompt.set_focused(true);
    assert!(prompt.is_focused());

    // Phase 9: Clear and start over
    prompt.clear();
    assert_eq!(prompt.value(), "");
    assert_eq!(prompt.cursor_position(), 0);

    // Phase 10: Type new value
    prompt.set_value("main.rs".to_string());
    assert_eq!(prompt.value(), "main.rs");
    assert_eq!(prompt.cursor_position(), 7);
}

#[test]
fn test_input_prompt_unicode_editing_workflow() {
    let mut prompt = InputPrompt::new("Title", "Message");

    // Phase 1: Type English
    prompt.set_value("Hello".to_string());
    assert_eq!(prompt.value(), "Hello");

    // Phase 2: Add space and Japanese
    prompt.insert_char(' ');
    prompt.insert_char('世');
    prompt.insert_char('界');
    assert_eq!(prompt.value(), "Hello 世界");

    // Phase 3: Navigate and insert emoji
    prompt.move_cursor_start();
    for _ in 0..5 {
        prompt.move_cursor_right();
    }
    prompt.insert_char('😀');
    assert!(prompt.value().contains('😀'));

    // Phase 4: Add Arabic at end
    prompt.move_cursor_end();
    prompt.insert_char(' ');
    prompt.insert_char('م');
    prompt.insert_char('ر');
    prompt.insert_char('ح');
    prompt.insert_char('ب');
    prompt.insert_char('ا');
    assert!(prompt.value().contains("مرحبا"));

    // Phase 5: Clear and verify
    prompt.clear();
    assert_eq!(prompt.value(), "");
}

#[test]
fn test_input_prompt_repeated_clear_and_fill() {
    let mut prompt = InputPrompt::new("Title", "Message");

    for i in 0..100 {
        let text = format!("iteration_{}", i);
        prompt.set_value(text.clone());
        assert_eq!(prompt.value(), text);

        prompt.clear();
        assert_eq!(prompt.value(), "");
    }
}

// ------------------------------------------------------------------------
// Empty State Tests
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_operations_on_empty() {
    let mut prompt = InputPrompt::new("Title", "Message");

    // All cursor movements should be safe on empty
    prompt.move_cursor_left();
    prompt.move_cursor_right();
    prompt.move_cursor_start();
    prompt.move_cursor_end();
    assert_eq!(prompt.cursor_position(), 0);

    // Delete on empty should be safe
    prompt.delete_char();
    assert_eq!(prompt.value(), "");
}

#[test]
fn test_input_prompt_empty_title_and_message() {
    let prompt = InputPrompt::new("", "");
    assert_eq!(prompt.title, "");
    assert_eq!(prompt.message, "");
    assert_eq!(prompt.value(), "");
}

#[test]
fn test_input_prompt_set_value_to_empty() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("test".to_string());
    assert_eq!(prompt.value(), "test");

    prompt.set_value("".to_string());
    assert_eq!(prompt.value(), "");
    assert_eq!(prompt.cursor_position(), 0);
}

// ------------------------------------------------------------------------
// Trait Coverage Tests
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_debug_trait() {
    let prompt = InputPrompt::new("Title", "Message");
    let debug_str = format!("{:?}", prompt);
    assert!(debug_str.contains("InputPrompt"));
}

// ------------------------------------------------------------------------
// Focus State Tests
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_default_focused() {
    let prompt = InputPrompt::new("Title", "Message");
    assert!(prompt.is_focused());
}

#[test]
fn test_input_prompt_focus_toggle_multiple_times() {
    let mut prompt = InputPrompt::new("Title", "Message");

    for _ in 0..100 {
        prompt.set_focused(false);
        assert!(!prompt.is_focused());
        prompt.set_focused(true);
        assert!(prompt.is_focused());
    }
}

// ------------------------------------------------------------------------
// UTF-8 Cursor Position Tests
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_cursor_position_with_multibyte_chars() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.insert_char('a'); // 1 byte
    assert_eq!(prompt.cursor_position(), 1);

    prompt.insert_char('é'); // 2 bytes
    assert_eq!(prompt.cursor_position(), 3);

    prompt.insert_char('世'); // 3 bytes
    assert_eq!(prompt.cursor_position(), 6);

    prompt.insert_char('😀'); // 4 bytes
    assert_eq!(prompt.cursor_position(), 10);
}

#[test]
fn test_input_prompt_navigation_through_multibyte_chars() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("a世😀b".to_string());

    prompt.move_cursor_start();
    assert_eq!(prompt.cursor_position(), 0);

    prompt.move_cursor_right(); // Move past 'a' (1 byte)
    assert_eq!(prompt.cursor_position(), 1);

    prompt.move_cursor_right(); // Move past '世' (3 bytes)
    assert_eq!(prompt.cursor_position(), 4);

    prompt.move_cursor_right(); // Move past '😀' (4 bytes)
    assert_eq!(prompt.cursor_position(), 8);

    prompt.move_cursor_right(); // Move past 'b' (1 byte)
    assert_eq!(prompt.cursor_position(), 9);

    // Now navigate backwards
    prompt.move_cursor_left(); // Back over 'b'
    assert_eq!(prompt.cursor_position(), 8);

    prompt.move_cursor_left(); // Back over '😀'
    assert_eq!(prompt.cursor_position(), 4);

    prompt.move_cursor_left(); // Back over '世'
    assert_eq!(prompt.cursor_position(), 1);

    prompt.move_cursor_left(); // Back over 'a'
    assert_eq!(prompt.cursor_position(), 0);
}

#[test]
fn test_input_prompt_delete_multibyte_chars() {
    let mut prompt = InputPrompt::new("Title", "Message");
    prompt.set_value("Hello世界😀".to_string());

    // Delete emoji at end
    prompt.delete_char();
    assert_eq!(prompt.value(), "Hello世界");

    // Delete '界'
    prompt.delete_char();
    assert_eq!(prompt.value(), "Hello世");

    // Delete '世'
    prompt.delete_char();
    assert_eq!(prompt.value(), "Hello");
}

// ------------------------------------------------------------------------
// Comprehensive Stress Test
// ------------------------------------------------------------------------

#[test]
fn test_input_prompt_comprehensive_stress() {
    let mut prompt = InputPrompt::new("Stress Test", "Testing all features");

    // Phase 1: Insert 1000 characters
    for i in 0..1000 {
        prompt.insert_char(char::from_u32((i % 26) + 97).unwrap());
    }
    assert_eq!(prompt.value().len(), 1000);

    // Phase 2: Navigate to middle
    prompt.move_cursor_start();
    for _ in 0..500 {
        prompt.move_cursor_right();
    }

    // Phase 3: Insert unicode
    prompt.insert_char('世');
    prompt.insert_char('😀');
    assert_eq!(prompt.value().len(), 1007); // 1000 + 3 (世) + 4 (😀)

    // Phase 4: Delete some characters
    for _ in 0..100 {
        prompt.delete_char();
    }

    // Phase 5: Clear and rebuild
    prompt.clear();
    assert_eq!(prompt.value(), "");

    // Phase 6: Build new content
    prompt.set_value("Final test 最終テスト 😀".to_string());
    assert!(prompt.value().contains("Final"));
    assert!(prompt.value().contains("最終"));
    assert!(prompt.value().contains('😀'));

    // Phase 7: Focus toggle
    prompt.set_focused(false);
    assert!(!prompt.is_focused());
    prompt.set_focused(true);
    assert!(prompt.is_focused());

    // Phase 8: Placeholder test
    prompt.clear();
    let with_placeholder = InputPrompt::new("Title", "Message")
        .with_placeholder("Custom placeholder");
    assert_eq!(with_placeholder.placeholder, "Custom placeholder");
}
