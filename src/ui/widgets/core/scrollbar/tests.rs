//! Scrollbar widget tests

    use super::*;

    #[test]
fn test_scrollbar_state_creation() {
    let state = ScrollbarState::new(100, 10, 20);
    assert_eq!(state.total, 100);
    assert_eq!(state.position, 10);
    assert_eq!(state.viewport_size, 20);
}

#[test]
fn test_scrollbar_state_should_show() {
    let state1 = ScrollbarState::new(100, 0, 20);
    assert!(state1.should_show()); // More items than viewport

    let state2 = ScrollbarState::new(20, 0, 20);
    assert!(!state2.should_show()); // Exact fit

    let state3 = ScrollbarState::new(10, 0, 20);
    assert!(!state3.should_show()); // Fewer items than viewport
}

#[test]
fn test_scrollbar_state_scroll_percentage() {
    let state1 = ScrollbarState::new(100, 0, 20);
    assert_eq!(state1.scroll_percentage(), 0.0); // At top

    let state2 = ScrollbarState::new(100, 80, 20);
    assert_eq!(state2.scroll_percentage(), 1.0); // At bottom

    let state3 = ScrollbarState::new(100, 40, 20);
    assert_eq!(state3.scroll_percentage(), 0.5); // Middle
}

#[test]
fn test_scrollbar_state_thumb_size_percentage() {
    let state1 = ScrollbarState::new(100, 0, 20);
    assert_eq!(state1.thumb_size_percentage(), 0.2); // 20/100

    let state2 = ScrollbarState::new(100, 0, 50);
    assert_eq!(state2.thumb_size_percentage(), 0.5); // 50/100

    let state3 = ScrollbarState::new(50, 0, 50);
    assert_eq!(state3.thumb_size_percentage(), 1.0); // 50/50 = 1.0 (max)
}

#[test]
fn test_scrollbar_vertical_creation() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::vertical(state);

    assert_eq!(scrollbar.orientation, ScrollbarOrientation::Vertical);
    assert_eq!(scrollbar.track_char, '│');
    assert_eq!(scrollbar.thumb_char, '█');
}

#[test]
fn test_scrollbar_horizontal_creation() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::horizontal(state);

    assert_eq!(scrollbar.orientation, ScrollbarOrientation::Horizontal);
    assert_eq!(scrollbar.track_char, '─');
    assert_eq!(scrollbar.thumb_char, '█');
}

#[test]
fn test_scrollbar_customization() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::vertical(state)
        .show_track(false)
        .track_char('|')
        .thumb_char('■');

    assert!(!scrollbar.show_track);
    assert_eq!(scrollbar.track_char, '|');
    assert_eq!(scrollbar.thumb_char, '■');
}

#[test]
fn test_scrollbar_set_state() {
    let state1 = ScrollbarState::new(100, 0, 20);
    let mut scrollbar = Scrollbar::vertical(state1);

    assert_eq!(scrollbar.state().position, 0);

    let state2 = ScrollbarState::new(100, 50, 20);
    scrollbar.set_state(state2);

    assert_eq!(scrollbar.state().position, 50);
}

#[test]
fn test_scroll_percentage_edge_cases() {
    // Empty list
    let state = ScrollbarState::new(0, 0, 0);
    assert_eq!(state.scroll_percentage(), 0.0);

    // Single item
    let state = ScrollbarState::new(1, 0, 1);
    assert_eq!(state.scroll_percentage(), 0.0);

    // Position beyond max
    let state = ScrollbarState::new(100, 200, 20);
    assert_eq!(state.scroll_percentage(), 1.0); // Should clamp to 1.0
}

#[test]
fn test_thumb_size_percentage_edge_cases() {
    // Empty list
    let state = ScrollbarState::new(0, 0, 0);
    assert_eq!(state.thumb_size_percentage(), 1.0);

    // Viewport larger than total
    let state = ScrollbarState::new(10, 0, 20);
    assert_eq!(state.thumb_size_percentage(), 1.0); // Clamped to max
}

// === COMPREHENSIVE EDGE CASE TESTS (MEDIUM tier) ===

// --- Extreme Values ---

#[test]
fn test_scrollbar_state_very_large_total() {
    let state = ScrollbarState::new(1_000_000, 500_000, 1000);
    assert_eq!(state.total, 1_000_000);
    assert!(state.should_show());
    // Approximately halfway through (500,000 / 999,000 ≈ 0.5005)
    assert!((state.scroll_percentage() - 0.5).abs() < 0.01);
}

#[test]
fn test_scrollbar_state_very_small_viewport() {
    let state = ScrollbarState::new(10_000, 0, 1);
    assert_eq!(state.viewport_size, 1);
    assert!(state.should_show());
    assert_eq!(state.thumb_size_percentage(), 0.0001); // 1/10000
}

#[test]
fn test_scrollbar_state_very_large_position() {
    let state = ScrollbarState::new(100, 10_000, 20);
    // Position beyond max should clamp
    assert_eq!(state.scroll_percentage(), 1.0);
}

#[test]
fn test_scrollbar_state_max_values() {
    let state = ScrollbarState::new(usize::MAX, usize::MAX / 2, 1000);
    // Should handle extreme values without overflow
    assert!(state.scroll_percentage() >= 0.0 && state.scroll_percentage() <= 1.0);
}

// --- Boundary Conditions ---

#[test]
fn test_scrollbar_state_position_at_zero() {
    let state = ScrollbarState::new(100, 0, 20);
    assert_eq!(state.position, 0);
    assert_eq!(state.scroll_percentage(), 0.0);
}

#[test]
fn test_scrollbar_state_position_at_max() {
    let state = ScrollbarState::new(100, 80, 20);
    assert_eq!(state.scroll_percentage(), 1.0);
}

#[test]
fn test_scrollbar_state_viewport_equals_one() {
    let state = ScrollbarState::new(100, 50, 1);
    assert_eq!(state.viewport_size, 1);
    assert!(state.should_show());
    assert_eq!(state.thumb_size_percentage(), 0.01); // 1/100
}

#[test]
fn test_scrollbar_state_viewport_equals_total() {
    let state = ScrollbarState::new(50, 0, 50);
    assert_eq!(state.viewport_size, state.total);
    assert!(!state.should_show()); // Should not show when equal
    assert_eq!(state.scroll_percentage(), 0.0);
}

#[test]
fn test_scrollbar_state_viewport_greater_than_total() {
    let state = ScrollbarState::new(30, 0, 50);
    assert!(state.viewport_size > state.total);
    assert!(!state.should_show());
    assert_eq!(state.thumb_size_percentage(), 1.0); // Clamped
}

#[test]
fn test_scrollbar_state_total_equals_one() {
    let state = ScrollbarState::new(1, 0, 1);
    assert_eq!(state.total, 1);
    assert!(!state.should_show());
}

#[test]
fn test_scrollbar_state_all_zeros() {
    let state = ScrollbarState::new(0, 0, 0);
    assert_eq!(state.scroll_percentage(), 0.0);
    assert_eq!(state.thumb_size_percentage(), 1.0);
    assert!(!state.should_show());
}

// --- Percentage Calculation Precision ---

#[test]
fn test_scroll_percentage_fractional_positions() {
    // Test various fractional positions
    let state1 = ScrollbarState::new(100, 25, 20);
    assert!((state1.scroll_percentage() - 0.3125).abs() < 0.001); // 25/80

    let state2 = ScrollbarState::new(100, 75, 20);
    assert!((state2.scroll_percentage() - 0.9375).abs() < 0.001); // 75/80

    let state3 = ScrollbarState::new(100, 10, 20);
    assert!((state3.scroll_percentage() - 0.125).abs() < 0.001); // 10/80
}

#[test]
fn test_thumb_size_percentage_fractional_viewports() {
    let state1 = ScrollbarState::new(100, 0, 33);
    assert!((state1.thumb_size_percentage() - 0.33).abs() < 0.01); // 33/100

    let state2 = ScrollbarState::new(1000, 0, 123);
    assert!((state2.thumb_size_percentage() - 0.123).abs() < 0.001); // 123/1000
}

// --- Unicode Characters ---

#[test]
fn test_scrollbar_unicode_track_char() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::vertical(state).track_char('░');

    assert_eq!(scrollbar.track_char, '░');
}

#[test]
fn test_scrollbar_unicode_thumb_char() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::vertical(state).thumb_char('▓');

    assert_eq!(scrollbar.thumb_char, '▓');
}

#[test]
fn test_scrollbar_emoji_characters() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::vertical(state)
        .track_char('🌫')
        .thumb_char('🐸');

    assert_eq!(scrollbar.track_char, '🌫');
    assert_eq!(scrollbar.thumb_char, '🐸');
}

// --- Builder Pattern Chaining ---

#[test]
fn test_scrollbar_builder_chaining() {
    let state = ScrollbarState::new(100, 50, 20);
    let scrollbar = Scrollbar::horizontal(state)
        .show_track(false)
        .track_char('.')
        .thumb_char('#');

    assert_eq!(scrollbar.orientation, ScrollbarOrientation::Horizontal);
    assert!(!scrollbar.show_track);
    assert_eq!(scrollbar.track_char, '.');
    assert_eq!(scrollbar.thumb_char, '#');
}

#[test]
fn test_scrollbar_builder_partial_chaining() {
    let state = ScrollbarState::new(100, 0, 20);
    let scrollbar = Scrollbar::vertical(state).show_track(false);

    assert!(!scrollbar.show_track);
    assert_eq!(scrollbar.track_char, '│'); // Default
    assert_eq!(scrollbar.thumb_char, '█'); // Default
}

// --- State Transitions ---

#[test]
fn test_scrollbar_multiple_state_updates() {
    let state1 = ScrollbarState::new(100, 0, 20);
    let mut scrollbar = Scrollbar::vertical(state1);

    assert_eq!(scrollbar.state().position, 0);

    let state2 = ScrollbarState::new(100, 25, 20);
    scrollbar.set_state(state2);
    assert_eq!(scrollbar.state().position, 25);

    let state3 = ScrollbarState::new(100, 50, 20);
    scrollbar.set_state(state3);
    assert_eq!(scrollbar.state().position, 50);

    let state4 = ScrollbarState::new(100, 80, 20);
    scrollbar.set_state(state4);
    assert_eq!(scrollbar.state().position, 80);
}

#[test]
fn test_scrollbar_state_updates_with_different_totals() {
    let state1 = ScrollbarState::new(100, 0, 20);
    let mut scrollbar = Scrollbar::vertical(state1);

    let state2 = ScrollbarState::new(200, 50, 20);
    scrollbar.set_state(state2);
    assert_eq!(scrollbar.state().total, 200);
    assert_eq!(scrollbar.state().position, 50);

    let state3 = ScrollbarState::new(50, 10, 20);
    scrollbar.set_state(state3);
    assert_eq!(scrollbar.state().total, 50);
    assert_eq!(scrollbar.state().position, 10);
}

// --- Trait Implementations ---

#[test]
fn test_scrollbar_orientation_clone() {
    let ori1 = ScrollbarOrientation::Vertical;
    let ori2 = ori1;
    assert_eq!(ori1, ori2); // Copy trait should work
}

#[test]
fn test_scrollbar_orientation_debug() {
    let ori = ScrollbarOrientation::Horizontal;
    let debug_str = format!("{:?}", ori);
    assert!(debug_str.contains("Horizontal"));
}

#[test]
fn test_scrollbar_orientation_partial_eq() {
    assert_eq!(ScrollbarOrientation::Vertical, ScrollbarOrientation::Vertical);
    assert_eq!(ScrollbarOrientation::Horizontal, ScrollbarOrientation::Horizontal);
    assert_ne!(ScrollbarOrientation::Vertical, ScrollbarOrientation::Horizontal);
}

#[test]
fn test_scrollbar_state_clone() {
    let state1 = ScrollbarState::new(100, 50, 20);
    let state2 = state1;

    assert_eq!(state1.total, state2.total);
    assert_eq!(state1.position, state2.position);
    assert_eq!(state1.viewport_size, state2.viewport_size);
}

#[test]
fn test_scrollbar_state_debug() {
    let state = ScrollbarState::new(100, 50, 20);
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("100"));
    assert!(debug_str.contains("50"));
    assert!(debug_str.contains("20"));
}

#[test]
fn test_scrollbar_state_partial_eq() {
    let state1 = ScrollbarState::new(100, 50, 20);
    let state2 = ScrollbarState::new(100, 50, 20);
    let state3 = ScrollbarState::new(100, 60, 20);

    assert_eq!(state1, state2);
    assert_ne!(state1, state3);
}

#[test]
fn test_scrollbar_clone() {
    let state = ScrollbarState::new(100, 50, 20);
    let scrollbar1 = Scrollbar::vertical(state).track_char('|');
    let scrollbar2 = scrollbar1.clone();

    assert_eq!(scrollbar1.orientation, scrollbar2.orientation);
    assert_eq!(scrollbar1.track_char, scrollbar2.track_char);
}

#[test]
fn test_scrollbar_debug() {
    let state = ScrollbarState::new(100, 50, 20);
    let scrollbar = Scrollbar::vertical(state);
    let debug_str = format!("{:?}", scrollbar);
    assert!(debug_str.contains("Vertical"));
}

// --- Complex Scenarios ---

#[test]
fn test_scrollbar_scrolling_through_large_list() {
    // Simulate scrolling through a list of 1000 items with viewport of 10
    let positions = vec![0, 100, 250, 500, 750, 900, 990];

    for pos in positions {
        let state = ScrollbarState::new(1000, pos, 10);
        assert!(state.should_show());

        let scroll_pct = state.scroll_percentage();
        assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);

        let thumb_pct = state.thumb_size_percentage();
        assert_eq!(thumb_pct, 0.01); // 10/1000
    }
}

#[test]
fn test_scrollbar_orientation_switching() {
    let state = ScrollbarState::new(100, 50, 20);

    let vertical = Scrollbar::vertical(state);
    assert_eq!(vertical.orientation, ScrollbarOrientation::Vertical);
    assert_eq!(vertical.track_char, '│');

    let horizontal = Scrollbar::horizontal(state);
    assert_eq!(horizontal.orientation, ScrollbarOrientation::Horizontal);
    assert_eq!(horizontal.track_char, '─');
}

#[test]
fn test_scrollbar_edge_case_combinations() {
    // Test various edge case combinations
    let test_cases = vec![
        (0, 0, 0),     // All zeros
        (1, 0, 1),     // Single item
        (10, 0, 10),   // Exact fit
        (10, 0, 20),   // Viewport larger
        (100, 0, 1),   // Minimal viewport
        (100, 99, 1),  // Position near end
    ];

    for (total, position, viewport) in test_cases {
        let state = ScrollbarState::new(total, position, viewport);

        // Verify calculations don't panic
        let _should_show = state.should_show();
        let scroll_pct = state.scroll_percentage();
        let thumb_pct = state.thumb_size_percentage();

        // Verify percentages are in valid range
        assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);
        assert!(thumb_pct >= 0.0 && thumb_pct <= 1.0);
    }
}

// ============================================================================
// ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
// ============================================================================

// ============ Stress Tests ============

#[test]
fn test_scrollbar_state_10000_positions() {
    // Stress test with 10,000 different positions
    for i in 0..10000 {
        let state = ScrollbarState::new(10000, i, 100);
        let scroll_pct = state.scroll_percentage();
        assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);
    }
}

#[test]
fn test_scrollbar_rapid_state_changes() {
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(1000, 0, 10));

    for i in 0..5000 {
        let state = ScrollbarState::new(1000, i % 1000, 10);
        scrollbar.set_state(state);
        assert_eq!(scrollbar.state().position, i % 1000);
    }
}

#[test]
fn test_scrollbar_rapid_creation_vertical_horizontal() {
    for i in 0..2000 {
        let state = ScrollbarState::new(100, i % 100, 10);
        if i % 2 == 0 {
            let _scrollbar = Scrollbar::vertical(state);
        } else {
            let _scrollbar = Scrollbar::horizontal(state);
        }
    }
}

#[test]
fn test_scrollbar_1000_builder_chains() {
    for i in 0..1000 {
        let state = ScrollbarState::new(100, i % 100, 10);
        let _scrollbar = Scrollbar::vertical(state)
            .show_track(i % 2 == 0)
            .track_char('│')
            .thumb_char('█');
    }
}

#[test]
fn test_scrollbar_alternating_orientation_stress() {
    for i in 0..3000 {
        let state = ScrollbarState::new(1000, i % 1000, 50);
        match i % 3 {
            0 => {
                let _scrollbar = Scrollbar::vertical(state);
            }
            1 => {
                let _scrollbar = Scrollbar::horizontal(state);
            }
            _ => {
                let mut scrollbar = Scrollbar::vertical(state);
                scrollbar.set_state(ScrollbarState::new(2000, i % 2000, 100));
            }
        }
    }
}

// ============ Unicode Edge Cases ============

#[test]
fn test_scrollbar_emoji_track_chars() {
    let state = ScrollbarState::new(100, 50, 10);
    let emojis = ['🐸', '🌟', '💚', '🎯', '🚀', '🔥', '✨', '🌈'];

    for emoji in emojis {
        let scrollbar = Scrollbar::vertical(state).track_char(emoji);
        assert_eq!(scrollbar.track_char, emoji);
    }
}

#[test]
fn test_scrollbar_emoji_thumb_chars() {
    let state = ScrollbarState::new(100, 50, 10);
    let emojis = ['🟢', '🟩', '💎', '🔷', '🔹', '⬜', '◼', '◾'];

    for emoji in emojis {
        let scrollbar = Scrollbar::vertical(state).thumb_char(emoji);
        assert_eq!(scrollbar.thumb_char, emoji);
    }
}

#[test]
fn test_scrollbar_combining_characters() {
    let state = ScrollbarState::new(100, 50, 10);
    // Combining diacritical marks
    let scrollbar = Scrollbar::vertical(state)
        .track_char('a') // Would combine with following marks
        .thumb_char('e'); // Would combine with following marks

    assert_eq!(scrollbar.track_char, 'a');
    assert_eq!(scrollbar.thumb_char, 'e');
}

#[test]
fn test_scrollbar_zero_width_characters() {
    let state = ScrollbarState::new(100, 50, 10);
    // Zero-width joiner (U+200D)
    let scrollbar = Scrollbar::vertical(state)
        .track_char('\u{200D}')
        .thumb_char('\u{200B}'); // Zero-width space

    assert_eq!(scrollbar.track_char, '\u{200D}');
    assert_eq!(scrollbar.thumb_char, '\u{200B}');
}

#[test]
fn test_scrollbar_rtl_characters() {
    let state = ScrollbarState::new(100, 50, 10);
    // Arabic and Hebrew characters
    let scrollbar = Scrollbar::vertical(state)
        .track_char('ع') // Arabic
        .thumb_char('ש'); // Hebrew

    assert_eq!(scrollbar.track_char, 'ع');
    assert_eq!(scrollbar.thumb_char, 'ש');
}

#[test]
fn test_scrollbar_box_drawing_characters() {
    let state = ScrollbarState::new(100, 50, 10);
    let box_chars = ['│', '─', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼'];

    for ch in box_chars {
        let scrollbar = Scrollbar::vertical(state).track_char(ch).thumb_char(ch);
        assert_eq!(scrollbar.track_char, ch);
        assert_eq!(scrollbar.thumb_char, ch);
    }
}

#[test]
fn test_scrollbar_emoji_sequences() {
    let state = ScrollbarState::new(100, 50, 10);
    // Single emoji from sequences (note: chars can't hold multi-char sequences)
    let scrollbar = Scrollbar::vertical(state)
        .track_char('👨') // Part of family emoji
        .thumb_char('🏴'); // Part of flag sequences

    assert_eq!(scrollbar.track_char, '👨');
    assert_eq!(scrollbar.thumb_char, '🏴');
}

// ============ Extreme Scroll Operations ============

#[test]
fn test_scrollbar_max_usize_total() {
    let state = ScrollbarState::new(usize::MAX, 0, 1000);
    assert!(state.should_show());
    // Should not panic or overflow
    let _scroll_pct = state.scroll_percentage();
    let _thumb_pct = state.thumb_size_percentage();
}

#[test]
fn test_scrollbar_max_usize_position() {
    let state = ScrollbarState::new(1000, usize::MAX, 100);
    // Position clamped to max_scroll
    assert_eq!(state.scroll_percentage(), 1.0);
}

#[test]
fn test_scrollbar_max_usize_viewport() {
    let state = ScrollbarState::new(100, 0, usize::MAX);
    assert!(!state.should_show()); // Viewport larger than total
    assert_eq!(state.thumb_size_percentage(), 1.0); // Clamped to 1.0
}

#[test]
fn test_scrollbar_rapid_scroll_to_extremes() {
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(1000, 0, 10));

    for _ in 0..100 {
        // Jump to beginning
        scrollbar.set_state(ScrollbarState::new(1000, 0, 10));
        assert_eq!(scrollbar.state().scroll_percentage(), 0.0);

        // Jump to end
        scrollbar.set_state(ScrollbarState::new(1000, 990, 10));
        assert_eq!(scrollbar.state().scroll_percentage(), 1.0);

        // Jump to middle
        scrollbar.set_state(ScrollbarState::new(1000, 495, 10));
        assert!((scrollbar.state().scroll_percentage() - 0.5).abs() < 0.01);
    }
}

#[test]
fn test_scrollbar_single_pixel_increments() {
    // Test very fine-grained scrolling
    let total = 100000;
    let viewport = 100;

    for i in 0..1000 {
        let state = ScrollbarState::new(total, i, viewport);
        let scroll_pct = state.scroll_percentage();
        assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);
    }
}

// ============ Percentage Calculation Precision ============

#[test]
fn test_scrollbar_percentage_precision_very_small() {
    // Test very small percentages
    let state = ScrollbarState::new(1000000, 1, 1000);
    let scroll_pct = state.scroll_percentage();
    // 1 / 999,000 ≈ 0.000001
    assert!(scroll_pct > 0.0 && scroll_pct < 0.0001);
}

#[test]
fn test_scrollbar_percentage_precision_very_close_to_one() {
    let state = ScrollbarState::new(1000000, 999999, 1000);
    let scroll_pct = state.scroll_percentage();
    // Should be very close to 1.0 but may be slightly less due to floating point
    assert!(scroll_pct > 0.9999 && scroll_pct <= 1.0);
}

#[test]
fn test_scrollbar_thumb_size_very_small_viewport() {
    let state = ScrollbarState::new(1000000, 0, 1);
    let thumb_pct = state.thumb_size_percentage();
    // 1 / 1,000,000 = 0.000001
    assert_eq!(thumb_pct, 0.000001);
}

#[test]
fn test_scrollbar_thumb_size_very_large_viewport() {
    let state = ScrollbarState::new(100, 0, 99);
    let thumb_pct = state.thumb_size_percentage();
    // 99 / 100 = 0.99
    assert_eq!(thumb_pct, 0.99);
}

#[test]
fn test_scrollbar_percentage_odd_numbers() {
    // Test with prime numbers to catch rounding issues
    let state1 = ScrollbarState::new(101, 53, 17);
    let scroll_pct1 = state1.scroll_percentage();
    assert!(scroll_pct1 >= 0.0 && scroll_pct1 <= 1.0);

    let state2 = ScrollbarState::new(997, 499, 103);
    let scroll_pct2 = state2.scroll_percentage();
    assert!(scroll_pct2 >= 0.0 && scroll_pct2 <= 1.0);
}

// ============ Complex State Transition Sequences ============

#[test]
fn test_scrollbar_growing_shrinking_total() {
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(100, 0, 10));

    // Grow total
    for i in 1..=10 {
        scrollbar.set_state(ScrollbarState::new(100 * i, 0, 10));
        assert_eq!(scrollbar.state().total, 100 * i);
    }

    // Shrink total
    for i in (1..=10).rev() {
        scrollbar.set_state(ScrollbarState::new(100 * i, 0, 10));
        assert_eq!(scrollbar.state().total, 100 * i);
    }
}

#[test]
fn test_scrollbar_changing_viewport_sizes() {
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(1000, 500, 10));

    let viewport_sizes = [1, 5, 10, 50, 100, 500, 1000, 2000];

    for viewport in viewport_sizes {
        scrollbar.set_state(ScrollbarState::new(1000, 500, viewport));
        assert_eq!(scrollbar.state().viewport_size, viewport);

        let thumb_pct = scrollbar.state().thumb_size_percentage();
        assert!(thumb_pct >= 0.0 && thumb_pct <= 1.0);
    }
}

#[test]
fn test_scrollbar_wave_scrolling_pattern() {
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(1000, 0, 10));

    // Simulate wave pattern: 0 -> 500 -> 0 -> 500 -> ...
    for i in 0..100 {
        let position = if i % 2 == 0 {
            (i / 2) * 10
        } else {
            500 - ((i / 2) * 10)
        };

        scrollbar.set_state(ScrollbarState::new(1000, position.min(990), 10));
        let scroll_pct = scrollbar.state().scroll_percentage();
        assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);
    }
}

#[test]
fn test_scrollbar_random_state_transitions() {
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(1000, 0, 10));

    // Pseudo-random but deterministic sequence
    let mut val = 123456789u64;
    for _ in 0..500 {
        val = val.wrapping_mul(1103515245).wrapping_add(12345);
        let position = (val % 1000) as usize;

        scrollbar.set_state(ScrollbarState::new(1000, position, 10));
        assert_eq!(scrollbar.state().position, position);
    }
}

// ============ Builder Pattern Edge Cases ============

#[test]
fn test_scrollbar_builder_show_track_toggle() {
    let state = ScrollbarState::new(100, 50, 10);

    for i in 0..100 {
        let scrollbar = Scrollbar::vertical(state).show_track(i % 2 == 0);
        assert_eq!(scrollbar.show_track, i % 2 == 0);
    }
}

#[test]
fn test_scrollbar_builder_all_combinations() {
    let state = ScrollbarState::new(100, 50, 10);
    let track_chars = ['│', '|', '.', '░'];
    let thumb_chars = ['█', '#', '▓', '■'];

    for track in track_chars {
        for thumb in thumb_chars {
            for show_track in [true, false] {
                let scrollbar = Scrollbar::vertical(state)
                    .track_char(track)
                    .thumb_char(thumb)
                    .show_track(show_track);

                assert_eq!(scrollbar.track_char, track);
                assert_eq!(scrollbar.thumb_char, thumb);
                assert_eq!(scrollbar.show_track, show_track);
            }
        }
    }
}

#[test]
fn test_scrollbar_builder_same_char_track_thumb() {
    let state = ScrollbarState::new(100, 50, 10);

    // When track and thumb use same character
    let scrollbar = Scrollbar::vertical(state)
        .track_char('█')
        .thumb_char('█');

    assert_eq!(scrollbar.track_char, scrollbar.thumb_char);
}

// ============ Trait Implementation Coverage ============

#[test]
fn test_scrollbar_orientation_eq_reflexive() {
    let ori = ScrollbarOrientation::Vertical;
    assert_eq!(ori, ori);
}

#[test]
fn test_scrollbar_orientation_eq_transitive() {
    let ori1 = ScrollbarOrientation::Horizontal;
    let ori2 = ScrollbarOrientation::Horizontal;
    let ori3 = ScrollbarOrientation::Horizontal;

    assert_eq!(ori1, ori2);
    assert_eq!(ori2, ori3);
    assert_eq!(ori1, ori3);
}

#[test]
fn test_scrollbar_state_eq_reflexive() {
    let state = ScrollbarState::new(100, 50, 10);
    assert_eq!(state, state);
}

#[test]
fn test_scrollbar_state_eq_transitive() {
    let state1 = ScrollbarState::new(100, 50, 10);
    let state2 = ScrollbarState::new(100, 50, 10);
    let state3 = ScrollbarState::new(100, 50, 10);

    assert_eq!(state1, state2);
    assert_eq!(state2, state3);
    assert_eq!(state1, state3);
}

#[test]
fn test_scrollbar_debug_output() {
    let state = ScrollbarState::new(12345, 6789, 100);
    let scrollbar = Scrollbar::vertical(state);
    let debug_str = format!("{:?}", scrollbar);

    assert!(debug_str.contains("Scrollbar"));
    assert!(debug_str.contains("Vertical"));
}

#[test]
fn test_scrollbar_state_debug_output() {
    let state = ScrollbarState::new(99999, 12345, 500);
    let debug_str = format!("{:?}", state);

    assert!(debug_str.contains("ScrollbarState"));
    assert!(debug_str.contains("99999"));
    assert!(debug_str.contains("12345"));
    assert!(debug_str.contains("500"));
}

#[test]
fn test_scrollbar_orientation_debug_output() {
    let vertical_str = format!("{:?}", ScrollbarOrientation::Vertical);
    let horizontal_str = format!("{:?}", ScrollbarOrientation::Horizontal);

    assert!(vertical_str.contains("Vertical"));
    assert!(horizontal_str.contains("Horizontal"));
}

// ============ Comprehensive Stress Test ============

#[test]
fn test_comprehensive_scrollbar_stress() {
    // Create initial scrollbar
    let mut scrollbar = Scrollbar::vertical(ScrollbarState::new(10000, 0, 100))
        .track_char('░')
        .thumb_char('█')
        .show_track(true);

    // Test rapid orientation switching with state updates
    for i in 0..100 {
        let position = i * 100;
        let state = ScrollbarState::new(10000, position, 100);

        if i % 2 == 0 {
            scrollbar = Scrollbar::vertical(state)
                .track_char('│')
                .thumb_char('█');
        } else {
            scrollbar = Scrollbar::horizontal(state)
                .track_char('─')
                .thumb_char('█');
        }

        assert_eq!(scrollbar.state().position, position);
    }

    // Test with emoji characters
    scrollbar = Scrollbar::vertical(ScrollbarState::new(10000, 5000, 100))
        .track_char('🌫')
        .thumb_char('🐸');

    assert_eq!(scrollbar.track_char, '🌫');
    assert_eq!(scrollbar.thumb_char, '🐸');

    // Test percentage calculations at various positions
    let positions = [0, 100, 500, 1000, 5000, 9000, 9900];
    for pos in positions {
        scrollbar.set_state(ScrollbarState::new(10000, pos, 100));
        let scroll_pct = scrollbar.state().scroll_percentage();
        let thumb_pct = scrollbar.state().thumb_size_percentage();

        assert!(scroll_pct >= 0.0 && scroll_pct <= 1.0);
        assert!(thumb_pct >= 0.0 && thumb_pct <= 1.0);
    }

    // Test builder pattern chaining
    let final_scrollbar = Scrollbar::horizontal(ScrollbarState::new(10000, 9900, 100))
        .show_track(false)
        .track_char('.')
        .thumb_char('#');

    assert_eq!(final_scrollbar.orientation, ScrollbarOrientation::Horizontal);
    assert!(!final_scrollbar.show_track);
    assert_eq!(final_scrollbar.track_char, '.');
    assert_eq!(final_scrollbar.thumb_char, '#');
    assert_eq!(final_scrollbar.state().position, 9900);
}
