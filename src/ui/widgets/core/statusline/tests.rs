//! Statusline widget tests

use crate::ui::theme::ToadTheme;
    use super::*;

    #[test]
fn test_status_level_colors() {
    assert_eq!(StatusLevel::Normal.color(), ToadTheme::FOREGROUND);
    assert_eq!(StatusLevel::Info.color(), ToadTheme::BLUE);
    assert_eq!(StatusLevel::Warning.color(), ToadTheme::YELLOW);
    assert_eq!(StatusLevel::Error.color(), ToadTheme::RED);
    assert_eq!(StatusLevel::Success.color(), ToadTheme::TOAD_GREEN);
}

#[test]
fn test_status_section_creation() {
    let section = StatusSection::new("Test")
        .with_level(StatusLevel::Info)
        .with_highlight(true);

    assert_eq!(section.text, "Test");
    assert_eq!(section.level, StatusLevel::Info);
    assert!(section.highlight);
}

#[test]
fn test_statusline_creation() {
    let statusline = Statusline::new();
    assert!(statusline.left.is_empty());
    assert!(statusline.center.is_empty());
    assert!(statusline.right.is_empty());
}

#[test]
fn test_statusline_add_sections() {
    let mut statusline = Statusline::new();

    statusline.add_left(StatusSection::new("Left"));
    statusline.add_center(StatusSection::new("Center"));
    statusline.add_right(StatusSection::new("Right"));

    assert_eq!(statusline.left.len(), 1);
    assert_eq!(statusline.center.len(), 1);
    assert_eq!(statusline.right.len(), 1);
}

#[test]
fn test_statusline_set_sections() {
    let mut statusline = Statusline::new();

    let sections = vec![
        StatusSection::new("A"),
        StatusSection::new("B"),
        StatusSection::new("C"),
    ];

    statusline.set_sections(SectionAlignment::Left, sections.clone());
    assert_eq!(statusline.sections(SectionAlignment::Left).len(), 3);

    statusline.set_sections(SectionAlignment::Center, sections.clone());
    assert_eq!(statusline.sections(SectionAlignment::Center).len(), 3);

    statusline.set_sections(SectionAlignment::Right, sections.clone());
    assert_eq!(statusline.sections(SectionAlignment::Right).len(), 3);
}

#[test]
fn test_statusline_clear() {
    let mut statusline = Statusline::new();

    statusline.add_left(StatusSection::new("L"));
    statusline.add_center(StatusSection::new("C"));
    statusline.add_right(StatusSection::new("R"));

    statusline.clear();

    assert!(statusline.left.is_empty());
    assert!(statusline.center.is_empty());
    assert!(statusline.right.is_empty());
}

#[test]
fn test_statusline_clear_alignment() {
    let mut statusline = Statusline::new();

    statusline.add_left(StatusSection::new("L"));
    statusline.add_center(StatusSection::new("C"));
    statusline.add_right(StatusSection::new("R"));

    statusline.clear_alignment(SectionAlignment::Left);

    assert!(statusline.left.is_empty());
    assert_eq!(statusline.center.len(), 1);
    assert_eq!(statusline.right.len(), 1);
}

#[test]
fn test_statusline_separator() {
    let mut statusline = Statusline::new();
    assert_eq!(statusline.separator, " │ ");

    statusline.set_separator(" | ");
    assert_eq!(statusline.separator, " | ");
}

#[test]
fn test_build_line() {
    let statusline = Statusline::new();

    let sections = vec![
        StatusSection::new("A").with_level(StatusLevel::Info),
        StatusSection::new("B").with_level(StatusLevel::Warning),
    ];

    let spans = statusline.build_line(&sections, true);

    // Should have: "A", separator, "B"
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].content, "A");
    assert_eq!(spans[1].content, " │ ");
    assert_eq!(spans[2].content, "B");
}

#[test]
fn test_build_line_no_separator() {
    let statusline = Statusline::new();

    let sections = vec![StatusSection::new("A"), StatusSection::new("B")];

    let spans = statusline.build_line(&sections, false);

    // Should have: "A", "B" (no separator)
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].content, "A");
    assert_eq!(spans[1].content, "B");
}

// ========================================================================
// EDGE CASE TESTS (Added for comprehensive coverage)
// ========================================================================

#[test]
fn test_status_section_very_long_text() {
    let long_text = "x".repeat(1000);
    let section = StatusSection::new(long_text.clone());
    assert_eq!(section.text.len(), 1000);
    assert_eq!(section.text, long_text);
}

#[test]
fn test_status_section_with_unicode() {
    let section = StatusSection::new("🐸 日本語 Status");
    assert!(section.text.contains('🐸'));
    assert!(section.text.contains("日本語"));
}

#[test]
fn test_status_section_empty_text() {
    let section = StatusSection::new("");
    assert_eq!(section.text, "");
    assert_eq!(section.level, StatusLevel::Normal);
}

#[test]
fn test_status_section_with_newlines() {
    let section = StatusSection::new("Line1\nLine2\nLine3");
    assert!(section.text.contains('\n'));
    // Note: statusline typically doesn't handle newlines specially
}

#[test]
fn test_statusline_many_sections() {
    let mut statusline = Statusline::new();

    // Add 50 sections to left
    for i in 0..50 {
        statusline.add_left(StatusSection::new(format!("Section {}", i)));
    }

    assert_eq!(statusline.left.len(), 50);
}

#[test]
fn test_statusline_default_implementation() {
    let default_statusline = Statusline::default();
    let new_statusline = Statusline::new();

    assert_eq!(default_statusline.left.len(), new_statusline.left.len());
    assert_eq!(default_statusline.separator, new_statusline.separator);
}

#[test]
fn test_statusline_separator_with_unicode() {
    let mut statusline = Statusline::new();
    statusline.set_separator(" 🐸 ");
    assert_eq!(statusline.separator, " 🐸 ");
}

#[test]
fn test_statusline_separator_empty() {
    let mut statusline = Statusline::new();
    statusline.set_separator("");
    assert_eq!(statusline.separator, "");
}

#[test]
fn test_status_section_highlight_default() {
    let section = StatusSection::new("Test");
    assert!(!section.highlight, "Highlight should be false by default");
}

#[test]
fn test_status_section_level_default() {
    let section = StatusSection::new("Test");
    assert_eq!(section.level, StatusLevel::Normal, "Level should be Normal by default");
}

#[test]
fn test_status_level_all_variants() {
    // Ensure all status level variants work
    let levels = vec![
        StatusLevel::Normal,
        StatusLevel::Info,
        StatusLevel::Warning,
        StatusLevel::Error,
        StatusLevel::Success,
    ];

    for level in levels {
        // Should not panic
        let _color = level.color();
    }
}

#[test]
fn test_section_alignment_all_variants() {
    // Ensure all alignment variants work
    let alignments = vec![
        SectionAlignment::Left,
        SectionAlignment::Center,
        SectionAlignment::Right,
    ];

    let mut statusline = Statusline::new();
    for alignment in alignments {
        statusline.clear_alignment(alignment);
        assert_eq!(statusline.sections(alignment).len(), 0);
    }
}

#[test]
fn test_statusline_mixed_levels() {
    let mut statusline = Statusline::new();

    statusline.add_left(StatusSection::new("Normal").with_level(StatusLevel::Normal));
    statusline.add_left(StatusSection::new("Info").with_level(StatusLevel::Info));
    statusline.add_left(StatusSection::new("Warning").with_level(StatusLevel::Warning));
    statusline.add_left(StatusSection::new("Error").with_level(StatusLevel::Error));
    statusline.add_left(StatusSection::new("Success").with_level(StatusLevel::Success));

    assert_eq!(statusline.left.len(), 5);
    assert_eq!(statusline.left[0].level, StatusLevel::Normal);
    assert_eq!(statusline.left[1].level, StatusLevel::Info);
    assert_eq!(statusline.left[2].level, StatusLevel::Warning);
    assert_eq!(statusline.left[3].level, StatusLevel::Error);
    assert_eq!(statusline.left[4].level, StatusLevel::Success);
}

#[test]
fn test_build_line_single_section() {
    let statusline = Statusline::new();
    let sections = vec![StatusSection::new("Single")];

    let spans = statusline.build_line(&sections, true);

    // Single section = no separator needed
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].content, "Single");
}

#[test]
fn test_build_line_empty_sections() {
    let statusline = Statusline::new();
    let sections = vec![];

    let spans = statusline.build_line(&sections, true);

    assert_eq!(spans.len(), 0, "Empty sections should produce empty spans");
}

#[test]
fn test_statusline_clear_preserves_separator() {
    let mut statusline = Statusline::new();
    statusline.set_separator(" | ");

    statusline.add_left(StatusSection::new("Test"));
    statusline.clear();

    assert_eq!(statusline.separator, " | ", "Clear should preserve separator");
}

// ============================================================================
// ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
// ============================================================================

// ============ Stress Tests ============

#[test]
fn test_statusline_10000_sections_left() {
    let mut statusline = Statusline::new();

    for i in 0..10000 {
        statusline.add_left(StatusSection::new(format!("L{}", i)));
    }

    assert_eq!(statusline.left.len(), 10000);
    assert_eq!(statusline.left[0].text, "L0");
    assert_eq!(statusline.left[9999].text, "L9999");
}

#[test]
fn test_statusline_rapid_clear_refill_cycles() {
    let mut statusline = Statusline::new();

    for cycle in 0..1000 {
        statusline.add_left(StatusSection::new(format!("Cycle{}", cycle)));
        statusline.add_center(StatusSection::new("Center"));
        statusline.add_right(StatusSection::new("Right"));

        assert_eq!(statusline.left.len(), 1); // Always 1 after clear + add
        assert_eq!(statusline.center.len(), 1);
        assert_eq!(statusline.right.len(), 1);

        statusline.clear();

        assert!(statusline.left.is_empty());
        assert!(statusline.center.is_empty());
        assert!(statusline.right.is_empty());
    }
}

#[test]
fn test_statusline_alternating_add_clear_alignment() {
    let mut statusline = Statusline::new();

    for i in 0..1000 {
        match i % 3 {
            0 => {
                statusline.add_left(StatusSection::new("L"));
                statusline.clear_alignment(SectionAlignment::Left);
            }
            1 => {
                statusline.add_center(StatusSection::new("C"));
                statusline.clear_alignment(SectionAlignment::Center);
            }
            _ => {
                statusline.add_right(StatusSection::new("R"));
                statusline.clear_alignment(SectionAlignment::Right);
            }
        }
    }

    assert!(statusline.left.is_empty());
    assert!(statusline.center.is_empty());
    assert!(statusline.right.is_empty());
}

#[test]
fn test_statusline_stress_all_alignments() {
    let mut statusline = Statusline::new();

    for i in 0..5000 {
        statusline.add_left(StatusSection::new(format!("L{}", i)));
        statusline.add_center(StatusSection::new(format!("C{}", i)));
        statusline.add_right(StatusSection::new(format!("R{}", i)));
    }

    assert_eq!(statusline.left.len(), 5000);
    assert_eq!(statusline.center.len(), 5000);
    assert_eq!(statusline.right.len(), 5000);
}

#[test]
fn test_statusline_rapid_separator_changes() {
    let mut statusline = Statusline::new();

    for i in 0..3000 {
        statusline.set_separator(format!(" S{} ", i));
        assert_eq!(statusline.separator, format!(" S{} ", i));
    }
}

// ============ Unicode Edge Cases ============

#[test]
fn test_status_section_rtl_text() {
    let section = StatusSection::new("مرحبا بك في TOAD שלום");
    assert!(section.text.contains("مرحبا"));
    assert!(section.text.contains("שלום"));
    assert!(section.text.contains("TOAD"));
}

#[test]
fn test_status_section_mixed_scripts() {
    let section = StatusSection::new("Hello世界Привет안녕하세요");
    assert!(section.text.contains("Hello"));
    assert!(section.text.contains("世界"));
    assert!(section.text.contains("Привет"));
    assert!(section.text.contains("안녕하세요"));
}

#[test]
fn test_status_section_combining_characters() {
    let section = StatusSection::new("é̂ñ̃ỹ̀");
    assert!(section.text.len() > 5); // Multi-byte combining chars
}

#[test]
fn test_status_section_zero_width_characters() {
    let section = StatusSection::new("Test\u{200B}\u{200C}\u{200D}");
    assert!(section.text.contains("Test"));
    assert!(section.text.len() > 4); // Zero-width chars add bytes
}

#[test]
fn test_status_section_emoji_variations() {
    let section = StatusSection::new("👍🏻👍🏿🐸🚀");
    assert!(section.text.contains('🐸'));
    assert!(section.text.contains("👍"));
}

#[test]
fn test_separator_with_rtl() {
    let mut statusline = Statusline::new();
    statusline.set_separator(" → مرحبا ← ");
    assert_eq!(statusline.separator, " → مرحبا ← ");
}

// ============ Extreme Sizes ============

#[test]
fn test_status_section_100k_characters() {
    let huge_text = "X".repeat(100000);
    let section = StatusSection::new(huge_text.clone());
    assert_eq!(section.text.len(), 100000);
    assert_eq!(section.text, huge_text);
}

#[test]
fn test_statusline_1000_emoji_sections() {
    let mut statusline = Statusline::new();

    for i in 0..1000 {
        let emoji = match i % 5 {
            0 => "🐸",
            1 => "🚀",
            2 => "💚",
            3 => "🎯",
            _ => "⚡",
        };
        statusline.add_left(StatusSection::new(format!("{} Status", emoji)));
    }

    assert_eq!(statusline.left.len(), 1000);
}

#[test]
fn test_separator_very_long() {
    let mut statusline = Statusline::new();
    let long_separator = " X ".repeat(1000);
    statusline.set_separator(long_separator.clone());
    assert_eq!(statusline.separator, long_separator);
}

// ============ Complex Workflows ============

#[test]
fn test_statusline_set_then_clear_then_add() {
    let mut statusline = Statusline::new();

    let sections = vec![
        StatusSection::new("A"),
        StatusSection::new("B"),
        StatusSection::new("C"),
    ];

    statusline.set_sections(SectionAlignment::Left, sections);
    assert_eq!(statusline.left.len(), 3);

    statusline.clear_alignment(SectionAlignment::Left);
    assert!(statusline.left.is_empty());

    statusline.add_left(StatusSection::new("New"));
    assert_eq!(statusline.left.len(), 1);
}

#[test]
fn test_statusline_mixed_alignment_operations() {
    let mut statusline = Statusline::new();

    for i in 0..100 {
        match i % 3 {
            0 => {
                statusline.add_left(StatusSection::new(format!("L{}", i)));
                let left = statusline.sections(SectionAlignment::Left);
                assert!(!left.is_empty());
            }
            1 => {
                statusline.add_center(StatusSection::new(format!("C{}", i)));
                let center = statusline.sections(SectionAlignment::Center);
                assert!(!center.is_empty());
            }
            _ => {
                statusline.add_right(StatusSection::new(format!("R{}", i)));
                let right = statusline.sections(SectionAlignment::Right);
                assert!(!right.is_empty());
            }
        }
    }

    assert!(statusline.left.len() >= 33);
    assert!(statusline.center.len() >= 33);
    assert!(statusline.right.len() >= 33);
}

#[test]
fn test_statusline_separator_changes_between_builds() {
    let statusline = Statusline::new();

    let sections = vec![StatusSection::new("A"), StatusSection::new("B")];

    let spans1 = statusline.build_line(&sections, true);
    assert_eq!(spans1[1].content, " │ ");

    let mut statusline2 = Statusline::new();
    statusline2.set_separator(" => ");

    let spans2 = statusline2.build_line(&sections, true);
    assert_eq!(spans2[1].content, " => ");
}

// ============ Builder Pattern Edge Cases ============

#[test]
fn test_status_section_chained_builders() {
    let section = StatusSection::new("Test")
        .with_level(StatusLevel::Warning)
        .with_highlight(true)
        .with_level(StatusLevel::Error) // Override previous level
        .with_highlight(false); // Override previous highlight

    assert_eq!(section.level, StatusLevel::Error);
    assert!(!section.highlight);
}

#[test]
fn test_status_section_builder_all_levels() {
    let levels = vec![
        StatusLevel::Normal,
        StatusLevel::Info,
        StatusLevel::Warning,
        StatusLevel::Error,
        StatusLevel::Success,
    ];

    for level in levels {
        let section = StatusSection::new("Test").with_level(level);
        assert_eq!(section.level, level);
    }
}

#[test]
fn test_status_section_builder_highlight_toggle() {
    let s1 = StatusSection::new("Test").with_highlight(true);
    assert!(s1.highlight);

    let s2 = StatusSection::new("Test").with_highlight(false);
    assert!(!s2.highlight);

    let s3 = StatusSection::new("Test")
        .with_highlight(true)
        .with_highlight(false);
    assert!(!s3.highlight);
}

// ============ Clone/PartialEq/Debug Trait Coverage ============

#[test]
fn test_status_section_clone() {
    let section = StatusSection::new("Original")
        .with_level(StatusLevel::Info)
        .with_highlight(true);

    let cloned = section.clone();

    assert_eq!(section.text, cloned.text);
    assert_eq!(section.level, cloned.level);
    assert_eq!(section.highlight, cloned.highlight);
}

#[test]
fn test_status_section_partial_eq() {
    let s1 = StatusSection::new("Test").with_level(StatusLevel::Warning);
    let s2 = StatusSection::new("Test").with_level(StatusLevel::Warning);
    let s3 = StatusSection::new("Different");

    assert_eq!(s1, s2);
    assert_ne!(s1, s3);
}

#[test]
fn test_status_level_debug() {
    let level = StatusLevel::Error;
    let debug_str = format!("{:?}", level);
    assert!(debug_str.contains("Error"));
}

#[test]
fn test_status_level_clone() {
    let level = StatusLevel::Success;
    let cloned = level;
    assert_eq!(level, cloned);
}

#[test]
fn test_status_level_partial_eq() {
    assert_eq!(StatusLevel::Normal, StatusLevel::Normal);
    assert_ne!(StatusLevel::Normal, StatusLevel::Error);
}

#[test]
fn test_status_section_debug() {
    let section = StatusSection::new("Debug Test");
    let debug_str = format!("{:?}", section);
    assert!(debug_str.contains("StatusSection"));
}

#[test]
fn test_section_alignment_debug() {
    let alignment = SectionAlignment::Center;
    let debug_str = format!("{:?}", alignment);
    assert!(debug_str.contains("Center"));
}

#[test]
fn test_section_alignment_clone() {
    let alignment = SectionAlignment::Right;
    let cloned = alignment;
    assert_eq!(alignment, cloned);
}

#[test]
fn test_section_alignment_partial_eq() {
    assert_eq!(SectionAlignment::Left, SectionAlignment::Left);
    assert_ne!(SectionAlignment::Left, SectionAlignment::Right);
}

#[test]
fn test_statusline_clone() {
    let mut statusline = Statusline::new();
    statusline.add_left(StatusSection::new("Left"));
    statusline.set_separator(" | ");

    let cloned = statusline.clone();

    assert_eq!(statusline.left.len(), cloned.left.len());
    assert_eq!(statusline.separator, cloned.separator);
}

#[test]
fn test_statusline_debug() {
    let statusline = Statusline::new();
    let debug_str = format!("{:?}", statusline);
    assert!(debug_str.contains("Statusline"));
}

// ============ Section Alignment Edge Cases ============

#[test]
fn test_sections_returns_slices() {
    let mut statusline = Statusline::new();

    statusline.add_left(StatusSection::new("L1"));
    statusline.add_left(StatusSection::new("L2"));

    let left_sections = statusline.sections(SectionAlignment::Left);
    assert_eq!(left_sections.len(), 2);
    assert_eq!(left_sections[0].text, "L1");
    assert_eq!(left_sections[1].text, "L2");
}

#[test]
fn test_set_sections_empty_vector() {
    let mut statusline = Statusline::new();
    statusline.add_left(StatusSection::new("Test"));

    statusline.set_sections(SectionAlignment::Left, vec![]);
    assert!(statusline.left.is_empty());
}

#[test]
fn test_set_sections_replaces_existing() {
    let mut statusline = Statusline::new();
    statusline.add_left(StatusSection::new("Old1"));
    statusline.add_left(StatusSection::new("Old2"));

    let new_sections = vec![
        StatusSection::new("New1"),
        StatusSection::new("New2"),
        StatusSection::new("New3"),
    ];

    statusline.set_sections(SectionAlignment::Left, new_sections);

    assert_eq!(statusline.left.len(), 3);
    assert_eq!(statusline.left[0].text, "New1");
    assert_eq!(statusline.left[2].text, "New3");
}

// ============ Highlight Edge Cases ============

#[test]
fn test_all_sections_highlighted() {
    let mut statusline = Statusline::new();

    for i in 0..100 {
        statusline.add_left(
            StatusSection::new(format!("H{}", i)).with_highlight(true),
        );
    }

    for section in &statusline.left {
        assert!(section.highlight);
    }
}

#[test]
fn test_mixed_highlight_states() {
    let mut statusline = Statusline::new();

    for i in 0..100 {
        let highlight = i % 2 == 0;
        statusline.add_left(
            StatusSection::new(format!("S{}", i)).with_highlight(highlight),
        );
    }

    assert!(statusline.left[0].highlight);
    assert!(!statusline.left[1].highlight);
    assert!(statusline.left[2].highlight);
}

#[test]
fn test_highlight_with_all_status_levels() {
    let levels = vec![
        StatusLevel::Normal,
        StatusLevel::Info,
        StatusLevel::Warning,
        StatusLevel::Error,
        StatusLevel::Success,
    ];

    for level in levels {
        let section = StatusSection::new("Test")
            .with_level(level)
            .with_highlight(true);

        assert_eq!(section.level, level);
        assert!(section.highlight);
    }
}

// ============ Build Line Edge Cases ============

#[test]
fn test_build_line_many_sections() {
    let statusline = Statusline::new();

    let sections: Vec<_> = (0..1000)
        .map(|i| StatusSection::new(format!("S{}", i)))
        .collect();

    let spans = statusline.build_line(&sections, true);

    // 1000 sections + 999 separators = 1999 spans
    assert_eq!(spans.len(), 1999);
}

#[test]
fn test_build_line_with_highlights() {
    let statusline = Statusline::new();

    let sections = vec![
        StatusSection::new("Normal"),
        StatusSection::new("Highlighted").with_highlight(true),
        StatusSection::new("Normal2"),
    ];

    let spans = statusline.build_line(&sections, true);

    // Should have 5 spans: section, sep, section, sep, section
    assert_eq!(spans.len(), 5);
}

#[test]
fn test_build_line_all_status_levels() {
    let statusline = Statusline::new();

    let sections = vec![
        StatusSection::new("N").with_level(StatusLevel::Normal),
        StatusSection::new("I").with_level(StatusLevel::Info),
        StatusSection::new("W").with_level(StatusLevel::Warning),
        StatusSection::new("E").with_level(StatusLevel::Error),
        StatusSection::new("S").with_level(StatusLevel::Success),
    ];

    let spans = statusline.build_line(&sections, true);

    // 5 sections + 4 separators = 9 spans
    assert_eq!(spans.len(), 9);
    assert_eq!(spans[0].content, "N");
    assert_eq!(spans[2].content, "I");
    assert_eq!(spans[4].content, "W");
    assert_eq!(spans[6].content, "E");
    assert_eq!(spans[8].content, "S");
}

// ============ Comprehensive Stress Test ============

#[test]
fn test_comprehensive_statusline_stress() {
    let mut statusline = Statusline::new();

    // Set custom separator
    statusline.set_separator(" 🐸 ");

    // Add sections with various configurations
    for i in 0..100 {
        let level = match i % 5 {
            0 => StatusLevel::Normal,
            1 => StatusLevel::Info,
            2 => StatusLevel::Warning,
            3 => StatusLevel::Error,
            _ => StatusLevel::Success,
        };

        let highlight = i % 3 == 0;

        let text = match i % 4 {
            0 => format!("ASCII {}", i),
            1 => format!("🚀 Emoji {}", i),
            2 => format!("日本語 {}", i),
            _ => format!("مرحبا {}", i),
        };

        match i % 3 {
            0 => statusline.add_left(
                StatusSection::new(text)
                    .with_level(level)
                    .with_highlight(highlight),
            ),
            1 => statusline.add_center(
                StatusSection::new(text)
                    .with_level(level)
                    .with_highlight(highlight),
            ),
            _ => statusline.add_right(
                StatusSection::new(text)
                    .with_level(level)
                    .with_highlight(highlight),
            ),
        }
    }

    // Verify counts
    assert!(statusline.left.len() >= 33);
    assert!(statusline.center.len() >= 33);
    assert!(statusline.right.len() >= 33);

    // Get sections and verify
    let left = statusline.sections(SectionAlignment::Left);
    let center = statusline.sections(SectionAlignment::Center);
    let right = statusline.sections(SectionAlignment::Right);

    assert!(!left.is_empty());
    assert!(!center.is_empty());
    assert!(!right.is_empty());

    // Build lines
    let left_spans = statusline.build_line(left, true);
    let center_spans = statusline.build_line(center, true);
    let right_spans = statusline.build_line(right, true);

    assert!(!left_spans.is_empty());
    assert!(!center_spans.is_empty());
    assert!(!right_spans.is_empty());

    // Clear specific alignment
    statusline.clear_alignment(SectionAlignment::Center);
    assert!(statusline.center.is_empty());
    assert!(!statusline.left.is_empty());
    assert!(!statusline.right.is_empty());

    // Clear all
    statusline.clear();
    assert!(statusline.left.is_empty());
    assert!(statusline.center.is_empty());
    assert!(statusline.right.is_empty());

    // Verify separator preserved
    assert_eq!(statusline.separator, " 🐸 ");

    // Refill
    statusline.add_left(StatusSection::new("Final 💚"));
    assert_eq!(statusline.left.len(), 1);
    assert!(statusline.left[0].text.contains("Final"));
    assert!(statusline.left[0].text.contains("💚"));
}
