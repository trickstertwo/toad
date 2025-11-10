//! Preview widget tests

    use super::*;

    #[test]
fn test_preview_pane_creation() {
    let preview = PreviewPane::new("Test content");
    assert_eq!(preview.content(), "Test content");
    assert_eq!(preview.get_scroll_offset(), 0);
    assert!(!preview.show_line_numbers);
    assert!(preview.wrap_lines);
}

#[test]
fn test_preview_pane_with_title() {
    let preview = PreviewPane::new("Content").title("My Preview");
    assert_eq!(preview.title.as_deref(), Some("My Preview"));
}

#[test]
fn test_set_content() {
    let mut preview = PreviewPane::new("Old");
    preview.set_content("New");
    assert_eq!(preview.content(), "New");
    assert_eq!(preview.get_scroll_offset(), 0); // Should reset scroll
}

#[test]
fn test_scroll_down() {
    let mut preview = PreviewPane::new("Content");
    preview.scroll_down(5);
    assert_eq!(preview.get_scroll_offset(), 5);
}

#[test]
fn test_scroll_up() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(10);

    preview.scroll_up(3);
    assert_eq!(preview.get_scroll_offset(), 7);

    preview.scroll_up(20); // Should clamp at 0
    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_scroll_to_top() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(50);

    preview.scroll_to_top();
    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_line_count() {
    let preview = PreviewPane::new("Line 1\nLine 2\nLine 3");
    assert_eq!(preview.line_count(), 3);
}

#[test]
fn test_show_line_numbers() {
    let preview = PreviewPane::new("Content").show_line_numbers(true);
    assert!(preview.show_line_numbers);
}

#[test]
fn test_wrap_lines() {
    let preview = PreviewPane::new("Content").wrap_lines(false);
    assert!(!preview.wrap_lines);
}

#[test]
fn test_scroll_offset_builder() {
    let preview = PreviewPane::new("Content").scroll_offset(10);
    assert_eq!(preview.get_scroll_offset(), 10);
}

// ============================================================================
// ADDITIONAL COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Fuzzy Finding)
// ============================================================================

// ============ Stress Tests ============

#[test]
fn test_preview_very_large_content_100k_lines() {
    let mut lines = Vec::new();
    for i in 0..100000 {
        lines.push(format!("Line {}", i));
    }
    let content = lines.join("\n");

    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 100000);
}

#[test]
fn test_preview_very_long_single_line_100k_chars() {
    let content = "x".repeat(100000);
    let preview = PreviewPane::new(content);
    assert_eq!(preview.content().len(), 100000);
    assert_eq!(preview.line_count(), 1);
}

#[test]
fn test_preview_rapid_scrolling_1000_ops() {
    let mut preview = PreviewPane::new("Content");

    for _ in 0..500 {
        preview.scroll_down(1);
    }
    assert_eq!(preview.get_scroll_offset(), 500);

    for _ in 0..500 {
        preview.scroll_up(1);
    }
    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_preview_rapid_content_changes_1000() {
    let mut preview = PreviewPane::new("Initial");

    for i in 0..1000 {
        preview.set_content(format!("Content {}", i));
        assert_eq!(preview.get_scroll_offset(), 0); // Should reset each time
    }

    assert_eq!(preview.content(), "Content 999");
}

#[test]
fn test_preview_alternating_scroll_operations() {
    let mut preview = PreviewPane::new("Content");

    for _ in 0..1000 {
        preview.scroll_down(5);
        preview.scroll_up(5);
    }

    assert_eq!(preview.get_scroll_offset(), 0);
}

// ============ Unicode Edge Cases ============

#[test]
fn test_preview_content_with_emoji() {
    let content = "Line 1 🚀\nLine 2 🐸\nLine 3 💚";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert!(preview.content().contains('🚀'));
}

#[test]
fn test_preview_content_with_rtl_text() {
    let content = "مرحبا\nשלום\nHello";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert!(preview.content().contains("مرحبا"));
}

#[test]
fn test_preview_content_with_japanese() {
    let content = "日本語\nテスト\n内容";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert_eq!(preview.content(), "日本語\nテスト\n内容");
}

#[test]
fn test_preview_content_with_combining_chars() {
    let content = "é̂ñ̃\nCafé\nnaïve";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert!(preview.content().len() > 10);
}

#[test]
fn test_preview_content_with_zero_width() {
    let content = "Test\u{200B}Zero\u{200C}Width\u{200D}";
    let preview = PreviewPane::new(content);
    assert!(preview.content().contains("Test"));
    assert!(preview.content().contains("Zero"));
}

#[test]
fn test_preview_content_with_mixed_scripts() {
    let content = "Hello日本مرحبا🚀\nMixed\nScripts";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert!(preview.content().contains("Hello"));
    assert!(preview.content().contains("日本"));
}

#[test]
fn test_preview_line_count_with_unicode_lines() {
    let content = "🚀\n日本語\nمرحبا\nHello";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 4);
}

// ============ Scroll Edge Cases ============

#[test]
fn test_preview_scroll_beyond_max_u16() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(u16::MAX);
    assert_eq!(preview.get_scroll_offset(), u16::MAX);
}

#[test]
fn test_preview_scroll_up_from_zero() {
    let mut preview = PreviewPane::new("Content");
    preview.scroll_up(100); // Should saturate at 0
    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_preview_scroll_up_saturating() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(10);
    preview.scroll_up(20); // Should saturate at 0
    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_preview_scroll_with_empty_content() {
    let mut preview = PreviewPane::new("");
    preview.scroll_down(10);
    preview.scroll_up(5);
    assert_eq!(preview.get_scroll_offset(), 5);
}

#[test]
fn test_preview_scroll_to_top_from_max() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(u16::MAX);
    preview.scroll_to_top();
    assert_eq!(preview.get_scroll_offset(), 0);
}

// ============ Content Edge Cases ============

#[test]
fn test_preview_empty_content() {
    let preview = PreviewPane::new("");
    assert_eq!(preview.content(), "");
    assert_eq!(preview.line_count(), 0);
}

#[test]
fn test_preview_single_char_content() {
    let preview = PreviewPane::new("x");
    assert_eq!(preview.content(), "x");
    assert_eq!(preview.line_count(), 1);
}

#[test]
fn test_preview_many_newlines() {
    let content = "\n\n\n\n\n\n\n\n\n\n";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 10);
}

#[test]
fn test_preview_only_newlines_empty_lines() {
    let content = "\n\n\n";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
}

#[test]
fn test_preview_single_line_no_newline() {
    let content = "This is a single long line without any newlines";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 1);
}

#[test]
fn test_preview_whitespace_only_content() {
    let content = "   \n  \n    \n";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
}

#[test]
fn test_preview_tabs_and_spaces() {
    let content = "\t\tTabbed\n    Spaced\n\t  Mixed";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
}

// ============ Clone and Debug Traits ============

#[test]
fn test_preview_clone() {
    let preview = PreviewPane::new("Test content")
        .title("Test Title")
        .scroll_offset(10)
        .show_line_numbers(true)
        .wrap_lines(false);

    let cloned = preview.clone();
    assert_eq!(preview.content(), cloned.content());
    assert_eq!(preview.get_scroll_offset(), cloned.get_scroll_offset());
    assert_eq!(preview.show_line_numbers, cloned.show_line_numbers);
    assert_eq!(preview.wrap_lines, cloned.wrap_lines);
}

#[test]
fn test_preview_debug() {
    let preview = PreviewPane::new("Content");
    let debug_str = format!("{:?}", preview);
    assert!(debug_str.contains("PreviewPane"));
}

// ============ Complex Workflow Tests ============

#[test]
fn test_preview_workflow_set_scroll_reset() {
    let mut preview = PreviewPane::new("Initial content");
    preview.scroll_down(50);
    assert_eq!(preview.get_scroll_offset(), 50);

    preview.set_content("New content");
    assert_eq!(preview.get_scroll_offset(), 0); // Should reset

    preview.scroll_down(10);
    assert_eq!(preview.get_scroll_offset(), 10);
}

#[test]
fn test_preview_builder_pattern_chaining() {
    let preview = PreviewPane::new("Content")
        .title("My Preview")
        .scroll_offset(5)
        .show_line_numbers(true)
        .wrap_lines(false);

    assert_eq!(preview.content(), "Content");
    assert_eq!(preview.title.as_deref(), Some("My Preview"));
    assert_eq!(preview.get_scroll_offset(), 5);
    assert!(preview.show_line_numbers);
    assert!(!preview.wrap_lines);
}

#[test]
fn test_preview_rapid_operations_mixed() {
    let mut preview = PreviewPane::new("Initial");

    for i in 0..100 {
        preview.scroll_down(1);
        if i % 10 == 0 {
            preview.set_content(format!("Content {}", i));
        }
        preview.scroll_up(1);
    }

    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_preview_workflow_unicode_operations() {
    let mut preview = PreviewPane::new("日本語");
    preview.scroll_down(5);
    assert_eq!(preview.get_scroll_offset(), 5);

    preview.set_content("مرحبا\n🚀\nTest");
    assert_eq!(preview.line_count(), 3);
    assert_eq!(preview.get_scroll_offset(), 0);
}

// ============ Comprehensive Stress Test ============

#[test]
fn test_comprehensive_preview_stress() {
    let mut preview = PreviewPane::new("Initial content");

    // Phase 1: Large content with many lines
    let mut lines = Vec::new();
    for i in 0..1000 {
        lines.push(format!("Line {} with some content", i));
    }
    preview.set_content(lines.join("\n"));
    assert_eq!(preview.line_count(), 1000);
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 2: Scroll operations
    preview.scroll_down(100);
    assert_eq!(preview.get_scroll_offset(), 100);

    preview.scroll_up(50);
    assert_eq!(preview.get_scroll_offset(), 50);

    preview.scroll_to_top();
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 3: Builder pattern modifications
    let preview2 = preview
        .clone()
        .title("Stress Test")
        .show_line_numbers(true)
        .wrap_lines(false)
        .scroll_offset(25);

    assert_eq!(preview2.title.as_deref(), Some("Stress Test"));
    assert!(preview2.show_line_numbers);
    assert!(!preview2.wrap_lines);
    assert_eq!(preview2.get_scroll_offset(), 25);

    // Phase 4: Unicode content
    preview.set_content("🚀 日本語 مرحبا\n".repeat(100));
    assert_eq!(preview.line_count(), 100);

    // Phase 5: Extreme scrolling
    preview.scroll_down(u16::MAX / 2);
    assert!(preview.get_scroll_offset() > 0);

    preview.scroll_to_top();
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 6: Empty content
    preview.set_content("");
    assert_eq!(preview.line_count(), 0);
    assert_eq!(preview.get_scroll_offset(), 0);
}

// ============ Line Count Edge Cases ============

#[test]
fn test_preview_line_count_trailing_newline() {
    let content = "Line 1\nLine 2\n";
    let preview = PreviewPane::new(content);
    // lines() doesn't count trailing empty line after newline
    assert_eq!(preview.line_count(), 2);
}

#[test]
fn test_preview_line_count_no_trailing_newline() {
    let content = "Line 1\nLine 2";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 2);
}

#[test]
fn test_preview_line_count_windows_newlines() {
    let content = "Line 1\r\nLine 2\r\nLine 3";
    let preview = PreviewPane::new(content);
    // lines() handles both \n and \r\n
    assert_eq!(preview.line_count(), 3);
}

// ============ Title Edge Cases ============

#[test]
fn test_preview_title_empty() {
    let preview = PreviewPane::new("Content").title("");
    assert_eq!(preview.title.as_deref(), Some(""));
}

#[test]
fn test_preview_title_unicode() {
    let preview = PreviewPane::new("Content").title("日本語 Title 🚀");
    assert_eq!(preview.title.as_deref(), Some("日本語 Title 🚀"));
}

#[test]
fn test_preview_title_very_long() {
    let long_title = "x".repeat(1000);
    let preview = PreviewPane::new("Content").title(long_title.clone());
    assert_eq!(preview.title.as_deref(), Some(long_title.as_str()));
}

// ============ Default Trait Test ============

#[test]
fn test_preview_default() {
    let preview = PreviewPane::default();
    assert_eq!(preview.content(), "");
    assert_eq!(preview.get_scroll_offset(), 0);
    assert!(!preview.show_line_numbers);
    assert!(preview.wrap_lines);
    assert_eq!(preview.title, None);
}

// ============ Extreme Stress Tests (10k operations) ============

#[test]
fn test_preview_extreme_scroll_down_10k() {
    let mut preview = PreviewPane::new("Content");

    for _ in 0..10000 {
        preview.scroll_down(1);
    }

    assert_eq!(preview.get_scroll_offset(), 10000);
}

#[test]
fn test_preview_extreme_content_changes_10k() {
    let mut preview = PreviewPane::new("Initial");

    for i in 0..10000 {
        preview.set_content(format!("Content iteration {}", i));
        assert_eq!(preview.get_scroll_offset(), 0);
    }

    assert_eq!(preview.content(), "Content iteration 9999");
}

#[test]
fn test_preview_extreme_mixed_operations_10k() {
    let mut preview = PreviewPane::new("Initial");

    for i in 0..10000 {
        match i % 4 {
            0 => preview.scroll_down(1),
            1 => preview.scroll_up(1),
            2 => preview.scroll_to_top(),
            _ => preview.set_scroll_offset((i % 100) as u16),
        }
    }

    // Final state depends on last operation (i=9999, 9999%4=3)
    assert_eq!(preview.get_scroll_offset(), (9999 % 100) as u16);
}

#[test]
fn test_preview_extreme_builder_chaining() {
    let mut preview = PreviewPane::new("Initial");

    for i in 0..1000 {
        preview = preview
            .title(format!("Title {}", i))
            .scroll_offset((i % 100) as u16)
            .show_line_numbers(i % 2 == 0)
            .wrap_lines(i % 3 == 0);
    }

    assert_eq!(preview.title.as_deref(), Some("Title 999"));
    assert_eq!(preview.get_scroll_offset(), 99);
    // 999 % 2 == 1, so show_line_numbers = false
    assert!(!preview.show_line_numbers);
    // 999 % 3 == 0, so wrap_lines = true
    assert!(preview.wrap_lines);
}

// ============ Scroll Boundary Edge Cases ============

#[test]
fn test_preview_scroll_operations_at_max() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(u16::MAX);

    preview.scroll_up(1);
    assert_eq!(preview.get_scroll_offset(), u16::MAX - 1);

    preview.scroll_up(u16::MAX - 1);
    assert_eq!(preview.get_scroll_offset(), 0);

    preview.set_scroll_offset(u16::MAX);
    preview.scroll_to_top();
    assert_eq!(preview.get_scroll_offset(), 0);
}

#[test]
fn test_preview_scroll_near_max_boundary() {
    let mut preview = PreviewPane::new("Content");
    preview.set_scroll_offset(u16::MAX - 100);

    preview.scroll_down(50);
    assert_eq!(preview.get_scroll_offset(), u16::MAX - 50);

    preview.scroll_down(50);
    assert_eq!(preview.get_scroll_offset(), u16::MAX);
}

#[test]
fn test_preview_scroll_large_increments() {
    let mut preview = PreviewPane::new("Content");

    preview.scroll_down(10000);
    assert_eq!(preview.get_scroll_offset(), 10000);

    preview.scroll_down(20000);
    assert_eq!(preview.get_scroll_offset(), 30000);

    preview.scroll_up(15000);
    assert_eq!(preview.get_scroll_offset(), 15000);
}

// ============ Special Characters ============

#[test]
fn test_preview_content_with_null_bytes() {
    let content = "Line 1\0Line 2\0Line 3";
    let preview = PreviewPane::new(content);
    assert!(preview.content().contains('\0'));
    assert_eq!(preview.line_count(), 1); // Null bytes don't create new lines
}

#[test]
fn test_preview_content_with_control_chars() {
    let content = "Line 1\x01\x02\x03\nLine 2\x1B[31m\nLine 3";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert!(preview.content().contains('\x01'));
}

#[test]
fn test_preview_content_with_mixed_special_chars() {
    let content = "Tab\there\nNull\0byte\nControl\x1Bchar\nEmoji🚀end";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 4);
}

#[test]
fn test_preview_content_with_backspace_and_formfeed() {
    let content = "Line\x08\x0C1\nLine\x08\x0C2";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 2);
    assert!(preview.content().contains('\x08'));
}

// ============ State Transition Tests ============

#[test]
fn test_preview_wrap_lines_state_transitions() {
    let mut preview = PreviewPane::new("Content").wrap_lines(true);
    assert!(preview.wrap_lines);

    preview = preview.wrap_lines(false);
    assert!(!preview.wrap_lines);

    preview = preview.wrap_lines(true);
    assert!(preview.wrap_lines);
}

#[test]
fn test_preview_show_line_numbers_state_transitions() {
    let mut preview = PreviewPane::new("Content").show_line_numbers(false);
    assert!(!preview.show_line_numbers);

    preview = preview.show_line_numbers(true);
    assert!(preview.show_line_numbers);

    preview = preview.show_line_numbers(false);
    assert!(!preview.show_line_numbers);
}

#[test]
fn test_preview_combined_state_transitions() {
    let mut preview = PreviewPane::new("Content");

    for i in 0..100 {
        preview = preview
            .show_line_numbers(i % 2 == 0)
            .wrap_lines(i % 3 == 0);
    }

    // i=99: 99%2==1 (false), 99%3==0 (true)
    assert!(!preview.show_line_numbers);
    assert!(preview.wrap_lines);
}

// ============ Combined Edge Cases ============

#[test]
fn test_preview_large_title_large_content_rapid_scroll() {
    let long_title = "x".repeat(10000);
    let mut lines = Vec::new();
    for i in 0..10000 {
        lines.push(format!("Line {}", i));
    }
    let content = lines.join("\n");

    let mut preview = PreviewPane::new(content).title(long_title.clone());

    for _ in 0..100 {
        preview.scroll_down(10);
    }

    assert_eq!(preview.get_scroll_offset(), 1000);
    assert_eq!(preview.title.as_deref(), Some(long_title.as_str()));
    assert_eq!(preview.line_count(), 10000);
}

#[test]
fn test_preview_unicode_title_unicode_content_scroll() {
    let title = "🚀 日本語 مرحبا";
    let content = "Line 1: 日本語\nLine 2: مرحبا\nLine 3: 🚀";

    let mut preview = PreviewPane::new(content).title(title);
    preview.scroll_down(1);

    assert_eq!(preview.get_scroll_offset(), 1);
    assert_eq!(preview.title.as_deref(), Some(title));
    assert_eq!(preview.line_count(), 3);
}

#[test]
fn test_preview_all_flags_enabled_large_content() {
    let mut lines = Vec::new();
    for i in 0..1000 {
        lines.push(format!("Line {} content", i));
    }

    let preview = PreviewPane::new(lines.join("\n"))
        .title("All Flags Test")
        .show_line_numbers(true)
        .wrap_lines(true)
        .scroll_offset(50);

    assert!(preview.show_line_numbers);
    assert!(preview.wrap_lines);
    assert_eq!(preview.get_scroll_offset(), 50);
    assert_eq!(preview.line_count(), 1000);
}

// ============ More Unicode Extremes ============

#[test]
fn test_preview_100k_unicode_chars() {
    let content = "日本語🚀".repeat(20000); // Each repeat is 5 chars
    let preview = PreviewPane::new(content);
    assert!(preview.content().len() > 100000); // Multi-byte chars
}

#[test]
fn test_preview_mixed_line_endings() {
    let content = "Line 1\nLine 2\r\nLine 3";
    let preview = PreviewPane::new(content);
    // lines() treats \n and \r\n as line separators (but not standalone \r)
    assert_eq!(preview.line_count(), 3);
}

#[test]
fn test_preview_unicode_in_every_position() {
    let content = "🚀Start\nMiddle日本語Text\nEnd مرحبا";
    let preview = PreviewPane::new(content);
    assert_eq!(preview.line_count(), 3);
    assert!(preview.content().contains('🚀'));
    assert!(preview.content().contains("日本語"));
    assert!(preview.content().contains("مرحبا"));
}

// ============ Multi-Phase Comprehensive Workflow (10 phases) ============

#[test]
fn test_preview_10_phase_comprehensive_workflow() {
    // Phase 1: Initial setup
    let mut preview = PreviewPane::new("Initial content");
    assert_eq!(preview.content(), "Initial content");
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 2: Set large content
    let mut lines = Vec::new();
    for i in 0..5000 {
        lines.push(format!("Line {}", i));
    }
    preview.set_content(lines.join("\n"));
    assert_eq!(preview.line_count(), 5000);
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 3: Scroll operations
    preview.scroll_down(100);
    assert_eq!(preview.get_scroll_offset(), 100);
    preview.scroll_up(30);
    assert_eq!(preview.get_scroll_offset(), 70);

    // Phase 4: Builder pattern modifications
    preview = preview
        .title("Phase 4 Title")
        .show_line_numbers(true)
        .wrap_lines(false);
    assert_eq!(preview.title.as_deref(), Some("Phase 4 Title"));
    assert!(preview.show_line_numbers);
    assert!(!preview.wrap_lines);

    // Phase 5: Unicode content
    preview.set_content("🚀 日本語 مرحبا\n".repeat(100));
    assert_eq!(preview.line_count(), 100);
    assert_eq!(preview.get_scroll_offset(), 0); // Reset on set_content

    // Phase 6: Extreme scrolling
    preview.scroll_down(1000);
    assert_eq!(preview.get_scroll_offset(), 1000);
    preview.scroll_to_top();
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 7: State transitions
    preview = preview.show_line_numbers(false).wrap_lines(true);
    assert!(!preview.show_line_numbers);
    assert!(preview.wrap_lines);

    // Phase 8: Empty content
    preview.set_content("");
    assert_eq!(preview.line_count(), 0);
    assert_eq!(preview.get_scroll_offset(), 0);

    // Phase 9: Single character content
    preview.set_content("x");
    assert_eq!(preview.line_count(), 1);
    preview.scroll_down(5);
    assert_eq!(preview.get_scroll_offset(), 5);

    // Phase 10: Clone and verify
    let cloned = preview.clone();
    assert_eq!(cloned.content(), "x");
    assert_eq!(cloned.get_scroll_offset(), 5);
    assert!(!cloned.show_line_numbers);
    assert!(cloned.wrap_lines);
    assert_eq!(cloned.title.as_deref(), Some("Phase 4 Title"));
}

// ============ Boundary Conditions ============

#[test]
fn test_preview_content_exactly_at_boundaries() {
    // Test content at various size boundaries
    let content_255 = "x".repeat(255);
    let preview = PreviewPane::new(content_255);
    assert_eq!(preview.content().len(), 255);

    let content_256 = "x".repeat(256);
    let preview = PreviewPane::new(content_256);
    assert_eq!(preview.content().len(), 256);

    let content_65535 = "x".repeat(65535);
    let preview = PreviewPane::new(content_65535);
    assert_eq!(preview.content().len(), 65535);
}

#[test]
fn test_preview_line_count_at_boundaries() {
    // Test line counts at u16 boundaries
    let lines_255 = (0..255).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let preview = PreviewPane::new(lines_255);
    assert_eq!(preview.line_count(), 255);

    let lines_256 = (0..256).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let preview = PreviewPane::new(lines_256);
    assert_eq!(preview.line_count(), 256);
}

#[test]
fn test_preview_scroll_at_boundaries() {
    let mut preview = PreviewPane::new("Content");

    preview.set_scroll_offset(255);
    assert_eq!(preview.get_scroll_offset(), 255);

    preview.set_scroll_offset(256);
    assert_eq!(preview.get_scroll_offset(), 256);

    preview.set_scroll_offset(65535);
    assert_eq!(preview.get_scroll_offset(), 65535);

    preview.set_scroll_offset(u16::MAX);
    assert_eq!(preview.get_scroll_offset(), u16::MAX);
}

// ============ Rapid State Changes ============

#[test]
fn test_preview_rapid_title_changes() {
    let mut preview = PreviewPane::new("Content");

    for i in 0..1000 {
        preview = preview.title(format!("Title {}", i));
    }

    assert_eq!(preview.title.as_deref(), Some("Title 999"));
}

#[test]
fn test_preview_alternating_flags() {
    let mut preview = PreviewPane::new("Content");

    for i in 0..1000 {
        preview = preview
            .show_line_numbers(i % 2 == 0)
            .wrap_lines(i % 2 == 1);
    }

    // i=999: odd, so show_line_numbers=false, wrap_lines=true
    assert!(!preview.show_line_numbers);
    assert!(preview.wrap_lines);
}

// ============ Content Preservation Tests ============

#[test]
fn test_preview_content_preserved_through_scroll() {
    let content = "Original content with special chars: 🚀 日本語";
    let mut preview = PreviewPane::new(content);

    preview.scroll_down(100);
    preview.scroll_up(50);
    preview.scroll_to_top();

    assert_eq!(preview.content(), content);
}

#[test]
fn test_preview_content_preserved_through_flag_changes() {
    let content = "Preserved content";
    let mut preview = PreviewPane::new(content);

    preview = preview
        .show_line_numbers(true)
        .wrap_lines(false)
        .show_line_numbers(false)
        .wrap_lines(true);

    assert_eq!(preview.content(), content);
}

#[test]
fn test_preview_title_preserved_through_scroll() {
    let title = "Preserved Title 🚀";
    let mut preview = PreviewPane::new("Content").title(title);

    preview.scroll_down(50);
    preview.scroll_up(25);
    preview.scroll_to_top();

    assert_eq!(preview.title.as_deref(), Some(title));
}

// ============ Edge Case Combinations ============

#[test]
fn test_preview_empty_title_empty_content() {
    let preview = PreviewPane::new("").title("");
    assert_eq!(preview.content(), "");
    assert_eq!(preview.title.as_deref(), Some(""));
    assert_eq!(preview.line_count(), 0);
}

#[test]
fn test_preview_max_scroll_empty_content() {
    let mut preview = PreviewPane::new("");
    preview.set_scroll_offset(u16::MAX);

    assert_eq!(preview.get_scroll_offset(), u16::MAX);
    assert_eq!(preview.line_count(), 0);
}

#[test]
fn test_preview_all_features_with_empty_content() {
    let preview = PreviewPane::new("")
        .title("Empty Content Preview")
        .show_line_numbers(true)
        .wrap_lines(false)
        .scroll_offset(100);

    assert_eq!(preview.content(), "");
    assert_eq!(preview.line_count(), 0);
    assert_eq!(preview.get_scroll_offset(), 100);
    assert!(preview.show_line_numbers);
    assert!(!preview.wrap_lines);
}

// ============ Clone Preservation Tests ============

#[test]
fn test_preview_clone_after_many_operations() {
    let mut preview = PreviewPane::new("Initial");

    for i in 0..100 {
        preview.scroll_down(1);
        preview = preview.title(format!("T{}", i));
    }

    let cloned = preview.clone();
    assert_eq!(cloned.get_scroll_offset(), preview.get_scroll_offset());
    assert_eq!(cloned.title, preview.title);
    assert_eq!(cloned.content(), preview.content());
}

#[test]
fn test_preview_clone_independence() {
    let mut original = PreviewPane::new("Original");
    let mut cloned = original.clone();

    original.set_content("Modified");
    cloned.scroll_down(50);

    assert_eq!(original.content(), "Modified");
    assert_eq!(cloned.content(), "Original");
    assert_eq!(original.get_scroll_offset(), 0);
    assert_eq!(cloned.get_scroll_offset(), 50);
}
