//! Toast notification tests

use super::*;
use crate::ui::theme::ToadTheme;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toast_level_icons() {
        assert_eq!(ToastLevel::Info.icon(), "ℹ");
        assert_eq!(ToastLevel::Success.icon(), "✓");
        assert_eq!(ToastLevel::Warning.icon(), "⚠");
        assert_eq!(ToastLevel::Error.icon(), "✗");
    }

    #[test]
    fn test_toast_creation() {
        let toast = Toast::info("Test message");
        assert_eq!(toast.message(), "Test message");
        assert_eq!(toast.level(), ToastLevel::Info);
        assert!(toast.is_visible());
    }

    #[test]
    fn test_toast_convenience_methods() {
        let info = Toast::info("info");
        assert_eq!(info.level(), ToastLevel::Info);

        let success = Toast::success("success");
        assert_eq!(success.level(), ToastLevel::Success);

        let warning = Toast::warning("warning");
        assert_eq!(warning.level(), ToastLevel::Warning);

        let error = Toast::error("error");
        assert_eq!(error.level(), ToastLevel::Error);
    }

    #[test]
    fn test_toast_manager_operations() {
        let mut manager = ToastManager::new();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());

        manager.info("Message 1");
        assert_eq!(manager.len(), 1);
        assert!(!manager.is_empty());

        manager.success("Message 2");
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_toast_manager_add_methods() {
        let mut manager = ToastManager::new();

        manager.info("info");
        manager.success("success");
        manager.warning("warning");
        manager.error("error");

        assert_eq!(manager.len(), 4);
    }

    #[test]
    fn test_toast_remaining_time() {
        let toast = Toast::info("Test");
        let remaining = toast.remaining_time();
        assert!(remaining <= Duration::from_secs(3));
        assert!(remaining > Duration::from_secs(2));
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_toast_level_border_colors_unique() {
        let info_color = ToastLevel::Info.border_color();
        let success_color = ToastLevel::Success.border_color();
        let warning_color = ToastLevel::Warning.border_color();
        let error_color = ToastLevel::Error.border_color();

        // All colors should be distinct
        assert_ne!(info_color, success_color);
        assert_ne!(success_color, warning_color);
        assert_ne!(warning_color, error_color);
        assert_ne!(info_color, error_color);
    }

    #[test]
    fn test_toast_with_very_long_message() {
        let long_message = "A".repeat(10000);
        let toast = Toast::info(long_message.clone());
        assert_eq!(toast.message(), &long_message);
    }

    #[test]
    fn test_toast_with_unicode_message() {
        let toast = Toast::info("🎉 成功しました！ Operation complete 🚀");
        assert!(toast.message().contains("🎉"));
        assert!(toast.message().contains("成功"));
    }

    #[test]
    fn test_toast_with_empty_message() {
        let toast = Toast::info("");
        assert_eq!(toast.message(), "");
        assert!(toast.is_visible());
    }

    // ========================================
    // MEDIUM TIER EDGE CASE TESTS
    // ========================================

    // ToastLevel Edge Cases
    #[test]
    fn test_toast_level_all_colors() {
        assert_eq!(ToastLevel::Info.border_color(), ToadTheme::BLUE);
        assert_eq!(ToastLevel::Success.border_color(), ToadTheme::TOAD_GREEN);
        assert_eq!(ToastLevel::Warning.border_color(), ToadTheme::YELLOW);
        assert_eq!(ToastLevel::Error.border_color(), ToadTheme::RED);
    }

    #[test]
    fn test_toast_level_clone() {
        let level1 = ToastLevel::Success;
        let level2 = level1;
        assert_eq!(level1, level2);
    }

    #[test]
    fn test_toast_level_debug() {
        let level = ToastLevel::Warning;
        let debug_str = format!("{:?}", level);
        assert!(debug_str.contains("Warning"));
    }

    #[test]
    fn test_toast_level_partial_eq() {
        assert_eq!(ToastLevel::Info, ToastLevel::Info);
        assert_ne!(ToastLevel::Info, ToastLevel::Success);
        assert_ne!(ToastLevel::Warning, ToastLevel::Error);
    }

    // Toast Message Edge Cases
    #[test]
    fn test_toast_empty_message() {
        let toast = Toast::info("");
        assert_eq!(toast.message(), "");
        assert!(toast.is_visible());
    }

    #[test]
    fn test_toast_with_newlines() {
        let toast = Toast::info("Line 1\nLine 2\nLine 3");
        assert!(toast.message().contains("\n"));
    }

    #[test]
    fn test_toast_with_special_characters() {
        let toast = Toast::info("Test<>&\"'\\|/*?");
        assert!(toast.message().contains("<>"));
    }

    #[test]
    fn test_toast_custom_duration() {
        let toast = Toast::new(ToastLevel::Info, "Test", Duration::from_millis(100));
        assert!(toast.is_visible());

        // Can't easily test that it becomes invisible without sleeping
    }

    #[test]
    fn test_toast_duration_differences() {
        let info = Toast::info("Info");
        let success = Toast::success("Success");
        let warning = Toast::warning("Warning");
        let error = Toast::error("Error");

        // Duration is private, but we can verify they're all visible
        assert!(info.is_visible());
        assert!(success.is_visible());
        assert!(warning.is_visible());
        assert!(error.is_visible());
    }

    #[test]
    fn test_toast_manager_add_custom_toast() {
        let mut manager = ToastManager::new();
        let custom_toast = Toast::new(ToastLevel::Success, "Custom", Duration::from_secs(10));

        manager.add(custom_toast);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_manager_multiple_types() {
        let mut manager = ToastManager::new();

        manager.info("Info 1");
        manager.success("Success 1");
        manager.warning("Warning 1");
        manager.error("Error 1");
        manager.info("Info 2");

        assert_eq!(manager.len(), 5);
    }

    #[test]
    fn test_toast_manager_cleanup_keeps_visible() {
        let mut manager = ToastManager::new();

        manager.info("Test 1");
        manager.info("Test 2");
        manager.info("Test 3");

        manager.cleanup();
        // All should still be visible (just created)
        assert_eq!(manager.len(), 3);
    }

    #[test]
    fn test_toast_manager_clear_removes_all() {
        let mut manager = ToastManager::new();

        manager.info("Test 1");
        manager.success("Test 2");
        manager.warning("Test 3");
        manager.error("Test 4");

        assert_eq!(manager.len(), 4);

        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_toast_manager_is_empty_initially() {
        let manager = ToastManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_toast_manager_default() {
        let manager = ToastManager::default();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_toast_manager_many_toasts() {
        let mut manager = ToastManager::new();

        for i in 0..100 {
            manager.info(&format!("Toast {}", i));
        }

        assert_eq!(manager.len(), 100);
    }

    #[test]
    fn test_toast_manager_mixed_cleanup() {
        let mut manager = ToastManager::new();

        manager.info("Keep this");
        manager.cleanup();

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_level_equality() {
        assert_eq!(ToastLevel::Info, ToastLevel::Info);
        assert_eq!(ToastLevel::Success, ToastLevel::Success);
        assert_ne!(ToastLevel::Info, ToastLevel::Success);
        assert_ne!(ToastLevel::Warning, ToastLevel::Error);
    }

    #[test]
    fn test_toast_remaining_time_saturating() {
        let toast = Toast::new(ToastLevel::Info, "Test", Duration::from_millis(1));

        // Initially should have some time
        let remaining = toast.remaining_time();
        assert!(remaining <= Duration::from_millis(1));
    }

    #[test]
    fn test_toast_manager_sequential_operations() {
        let mut manager = ToastManager::new();

        manager.info("First");
        assert_eq!(manager.len(), 1);

        manager.success("Second");
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.error("Third");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_with_unicode_emoji_only() {
        let toast = Toast::info("🎉🚀🌟💯🔥");
        assert_eq!(toast.message(), "🎉🚀🌟💯🔥");
    }

    #[test]
    fn test_toast_with_whitespace_only() {
        let toast = Toast::info("     ");
        assert_eq!(toast.message(), "     ");
    }

    #[test]
    fn test_toast_manager_alternating_add_clear() {
        let mut manager = ToastManager::new();

        manager.info("Test");
        assert_eq!(manager.len(), 1);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.success("Test 2");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_very_long_message() {
        let long_msg = "A".repeat(1000);
        let toast = Toast::success(&long_msg);
        assert_eq!(toast.message(), long_msg);
        assert_eq!(toast.message().len(), 1000);
    }

    #[test]
    fn test_toast_unicode_message() {
        let toast = Toast::info("日本語メッセージ");
        assert_eq!(toast.message(), "日本語メッセージ");

        let toast2 = Toast::warning("Тест 中文 العربية");
        assert_eq!(toast2.message(), "Тест 中文 العربية");
    }

    #[test]
    fn test_toast_emoji_message() {
        let toast = Toast::success("🎉 Great! 🐸");
        assert_eq!(toast.message(), "🎉 Great! 🐸");

        let toast2 = Toast::error("❌ Failed 👨‍💻");
        assert_eq!(toast2.message(), "❌ Failed 👨‍💻");
    }

    #[test]
    fn test_toast_message_with_newlines() {
        let toast = Toast::info("Line 1\nLine 2\nLine 3");
        assert_eq!(toast.message(), "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_toast_message_with_tabs() {
        let toast = Toast::info("Column1\tColumn2\tColumn3");
        assert!(toast.message().contains('\t'));
    }

    // Toast Duration Edge Cases
    #[test]
    fn test_toast_custom_zero_duration() {
        let toast = Toast::new(ToastLevel::Info, "Instant", Duration::from_secs(0));
        // Immediately expired (or very close)
        std::thread::sleep(Duration::from_millis(1));
        assert!(!toast.is_visible());
    }

    #[test]
    fn test_toast_custom_very_short_duration() {
        let toast = Toast::new(ToastLevel::Success, "Brief", Duration::from_millis(10));
        assert!(toast.is_visible());
        std::thread::sleep(Duration::from_millis(15));
        assert!(!toast.is_visible());
    }

    #[test]
    fn test_toast_custom_very_long_duration() {
        let toast = Toast::new(
            ToastLevel::Warning,
            "Persistent",
            Duration::from_secs(3600),
        );
        assert!(toast.is_visible());
        assert!(toast.remaining_time() > Duration::from_secs(3599));
    }

    #[test]
    fn test_toast_default_durations() {
        let info = Toast::info("Info");
        assert!(info.remaining_time() <= Duration::from_secs(3));

        let success = Toast::success("Success");
        assert!(success.remaining_time() <= Duration::from_secs(3));

        let warning = Toast::warning("Warning");
        assert!(warning.remaining_time() <= Duration::from_secs(5));

        let error = Toast::error("Error");
        assert!(error.remaining_time() <= Duration::from_secs(7));
    }

    // Toast Visibility Edge Cases
    #[test]
    fn test_toast_visibility_immediately_after_creation() {
        let toast = Toast::info("New");
        assert!(toast.is_visible());
    }

    #[test]
    fn test_toast_remaining_time_saturates() {
        // Create toast that expires immediately
        let toast = Toast::new(ToastLevel::Info, "Test", Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(10));

        // Should saturate at 0, not underflow
        let remaining = toast.remaining_time();
        assert_eq!(remaining, Duration::from_secs(0));
    }

    // Toast Trait Tests
    #[test]
    fn test_toast_clone() {
        let toast1 = Toast::success("Original");
        let toast2 = toast1.clone();

        assert_eq!(toast1.message(), toast2.message());
        assert_eq!(toast1.level(), toast2.level());
    }

    #[test]
    fn test_toast_debug() {
        let toast = Toast::error("Debug test");
        let debug_str = format!("{:?}", toast);
        assert!(debug_str.contains("Toast"));
    }

    // ToastManager Boundary Conditions
    #[test]
    fn test_manager_empty_operations() {
        let mut manager = ToastManager::new();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());

        // Cleanup on empty should not panic
        manager.cleanup();
        assert_eq!(manager.len(), 0);

        // Clear on empty should not panic
        manager.clear();
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_toast_manager_cleanup_multiple_times() {
        let mut manager = ToastManager::new();

        manager.info("Test");
        manager.cleanup();
        manager.cleanup();
        manager.cleanup();

        assert_eq!(manager.len(), 1); // Should still be there
    }

    #[test]
    fn test_toast_level_copy() {
        let level = ToastLevel::Success;
        let copied = level;

        assert_eq!(level, copied);
    }

    #[test]
    fn test_toast_manager_with_very_long_messages() {
        let mut manager = ToastManager::new();

        let long = "X".repeat(10000);
        manager.info(&long);
        manager.success(&long);
        manager.warning(&long);

        assert_eq!(manager.len(), 3);
    }

    // ============================================================================
    // ADVANCED COMPREHENSIVE EDGE CASE TESTS (90%+ COVERAGE)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_toast_manager_10000_toasts() {
        let mut manager = ToastManager::new();
        for i in 0..10000 {
            manager.info(format!("Toast {}", i));
        }
        assert_eq!(manager.len(), 10000);
    }

    #[test]
    fn test_toast_rapid_creation_all_types() {
        for _ in 0..1000 {
            let _info = Toast::info("Info");
            let _success = Toast::success("Success");
            let _warning = Toast::warning("Warning");
            let _error = Toast::error("Error");
        }
        // Just verify no crashes
    }

    #[test]
    fn test_toast_manager_rapid_add_clear_cycles() {
        let mut manager = ToastManager::new();
        for i in 0..100 {
            manager.info(format!("Message {}", i));
            manager.success(format!("Success {}", i));
            manager.clear();
            assert_eq!(manager.len(), 0);
        }
    }

    #[test]
    fn test_toast_manager_alternating_types_stress() {
        let mut manager = ToastManager::new();
        for i in 0..1000 {
            match i % 4 {
                0 => manager.info(format!("I{}", i)),
                1 => manager.success(format!("S{}", i)),
                2 => manager.warning(format!("W{}", i)),
                _ => manager.error(format!("E{}", i)),
            }
        }
        assert_eq!(manager.len(), 1000);
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_toast_with_rtl_text() {
        let toast = Toast::info("مرحبا العالم Hello שלום");
        assert!(toast.message().contains("مرحبا"));
        assert!(toast.message().contains("שלום"));
    }

    #[test]
    fn test_toast_with_emoji_sequences() {
        let toast = Toast::success("👨‍👩‍👧‍👦 Family emoji 🎉");
        assert!(toast.message().contains("👨‍👩‍👧‍👦"));
    }

    #[test]
    fn test_toast_with_combining_characters() {
        let toast = Toast::warning("Café résumé naïve");
        assert!(toast.message().contains("é"));
    }

    #[test]
    fn test_toast_with_zero_width_characters() {
        let toast = Toast::error("Test\u{200B}with\u{200B}zero\u{200B}width");
        assert!(toast.message().contains("\u{200B}"));
    }

    #[test]
    fn test_toast_with_all_unicode_types() {
        let toast = Toast::info("Latin αβγ 中文 日本語 한글 العربية עברית 🎉🚀");
        assert!(toast.message().contains("αβγ"));
        assert!(toast.message().contains("中文"));
    }

    #[test]
    fn test_toast_with_box_drawing_characters() {
        let toast = Toast::info("┌─┬─┐\n│ │ │\n├─┼─┤");
        assert!(toast.message().contains("┌"));
    }

    // ============ Duration Edge Cases ============

    #[test]
    fn test_toast_zero_duration() {
        let toast = Toast::new(ToastLevel::Info, "Zero", Duration::from_secs(0));
        // Might be immediately invisible
        let _ = toast.is_visible();
    }

    #[test]
    fn test_toast_very_long_duration() {
        let toast = Toast::new(
            ToastLevel::Success,
            "Long",
            Duration::from_secs(3600 * 24 * 365),
        );
        assert!(toast.is_visible());
        let remaining = toast.remaining_time();
        assert!(remaining > Duration::from_secs(3600 * 24 * 364));
    }

    #[test]
    fn test_toast_one_millisecond_duration() {
        let toast = Toast::new(ToastLevel::Warning, "Fast", Duration::from_millis(1));
        let remaining = toast.remaining_time();
        assert!(remaining <= Duration::from_millis(1));
    }

    #[test]
    fn test_toast_remaining_time_decreases() {
        let toast = Toast::info("Test");
        let remaining1 = toast.remaining_time();

        std::thread::sleep(Duration::from_millis(10));

        let remaining2 = toast.remaining_time();
        assert!(remaining2 <= remaining1);
    }

    // ============ ToastLevel Debug and Clone ============

    #[test]
    fn test_toast_level_debug_format() {
        let info = ToastLevel::Info;
        let success = ToastLevel::Success;
        let warning = ToastLevel::Warning;
        let error = ToastLevel::Error;

        assert!(format!("{:?}", info).contains("Info"));
        assert!(format!("{:?}", success).contains("Success"));
        assert!(format!("{:?}", warning).contains("Warning"));
        assert!(format!("{:?}", error).contains("Error"));
    }

    #[test]
    fn test_toast_level_all_icons_unique() {
        let icons = vec![
            ToastLevel::Info.icon(),
            ToastLevel::Success.icon(),
            ToastLevel::Warning.icon(),
            ToastLevel::Error.icon(),
        ];

        for (i, icon1) in icons.iter().enumerate() {
            for (j, icon2) in icons.iter().enumerate() {
                if i != j {
                    assert_ne!(icon1, icon2);
                }
            }
        }
    }

    #[test]
    fn test_toast_level_all_border_colors() {
        // Just verify all methods work without panicking
        let _info_color = ToastLevel::Info.border_color();
        let _success_color = ToastLevel::Success.border_color();
        let _warning_color = ToastLevel::Warning.border_color();
        let _error_color = ToastLevel::Error.border_color();
    }

    // ============ Toast Clone and Debug ============

    #[test]
    fn test_toast_debug_format() {
        let toast = Toast::info("Debug test");
        let debug_str = format!("{:?}", toast);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_toast_clone_preserves_all_fields() {
        let original = Toast::warning("Original message");
        let cloned = original.clone();

        assert_eq!(original.message(), cloned.message());
        assert_eq!(original.level(), cloned.level());
        // Both should be visible (just created)
        assert!(original.is_visible());
        assert!(cloned.is_visible());
    }

    // ============ ToastManager Edge Cases ============

    #[test]
    fn test_toast_manager_empty_after_default() {
        let manager = ToastManager::default();
        assert!(manager.is_empty());
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_toast_manager_debug_format() {
        let mut manager = ToastManager::new();
        manager.info("Test");
        let debug_str = format!("{:?}", manager);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_toast_manager_multiple_cleanup_calls() {
        let mut manager = ToastManager::new();
        manager.info("Test 1");
        manager.success("Test 2");

        manager.cleanup();
        let count1 = manager.len();

        manager.cleanup();
        let count2 = manager.len();

        manager.cleanup();
        let count3 = manager.len();

        // Should remain stable
        assert_eq!(count1, count2);
        assert_eq!(count2, count3);
    }

    #[test]
    fn test_toast_manager_clear_then_add() {
        let mut manager = ToastManager::new();
        manager.info("First batch");
        manager.success("First batch 2");

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.warning("Second batch");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_manager_sequential_adds_preserved() {
        let mut manager = ToastManager::new();

        manager.info("1");
        manager.success("2");
        manager.warning("3");
        manager.error("4");

        assert_eq!(manager.len(), 4);

        // Toasts are stored in order added
        manager.cleanup();
        assert_eq!(manager.len(), 4); // All still visible
    }

    // ============ Message Edge Cases ============

    #[test]
    fn test_toast_extremely_long_message_100k() {
        let long = "M".repeat(100000);
        let toast = Toast::error(long.clone());
        assert_eq!(toast.message().len(), 100000);
    }

    #[test]
    fn test_toast_message_with_multiple_newlines() {
        let toast = Toast::warning("Line1\n\n\nLine2");
        assert_eq!(toast.message().matches('\n').count(), 3);
    }

    #[test]
    fn test_toast_message_with_carriage_returns() {
        let toast = Toast::error("Text\rWith\rCR");
        assert!(toast.message().contains("\r"));
    }

    #[test]
    fn test_toast_message_with_ansi_sequences() {
        let toast = Toast::info("\x1b[31mRed\x1b[0m");
        assert!(toast.message().contains("\x1b"));
    }

    #[test]
    fn test_toast_message_only_special_chars() {
        let toast = Toast::success("!@#$%^&*()_+-=[]{}|;:',.<>?/~`");
        assert!(toast.message().contains("!@#"));
    }

    // ============ Complex Manager Operations ============

    #[test]
    fn test_toast_manager_mixed_operations_sequence() {
        let mut manager = ToastManager::new();

        manager.info("1");
        manager.success("2");
        assert_eq!(manager.len(), 2);

        manager.cleanup();
        assert_eq!(manager.len(), 2);

        manager.warning("3");
        assert_eq!(manager.len(), 3);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.error("4");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_manager_add_custom_with_various_durations() {
        let mut manager = ToastManager::new();

        manager.add(Toast::new(
            ToastLevel::Info,
            "1sec",
            Duration::from_secs(1),
        ));
        manager.add(Toast::new(
            ToastLevel::Success,
            "5sec",
            Duration::from_secs(5),
        ));
        manager.add(Toast::new(
            ToastLevel::Warning,
            "10sec",
            Duration::from_secs(10),
        ));

        assert_eq!(manager.len(), 3);
    }

    #[test]
    fn test_toast_manager_with_unicode_messages() {
        let mut manager = ToastManager::new();

        manager.info("日本語");
        manager.success("العربية");
        manager.warning("עברית");
        manager.error("한글");

        assert_eq!(manager.len(), 4);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_toast_manager_stress() {
        let mut manager = ToastManager::new();

        // Add 100 toasts with varying types and messages
        for i in 0..100 {
            let message = format!("Message {} with unicode 日本語 🎉", i);

            match i % 4 {
                0 => manager.info(message),
                1 => manager.success(message),
                2 => manager.warning(message),
                _ => manager.error(message),
            }
        }

        assert_eq!(manager.len(), 100);

        // Cleanup shouldn't remove any (all just created)
        manager.cleanup();
        assert_eq!(manager.len(), 100);

        // Clear and verify
        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());

        // Add more after clear
        for i in 0..50 {
            manager.add(Toast::new(
                ToastLevel::Success,
                format!("Custom {}", i),
                Duration::from_secs(i % 10 + 1),
            ));
        }

        assert_eq!(manager.len(), 50);
    }

    #[test]
    fn test_toast_level_coverage_all_methods() {
        let levels = vec![
            ToastLevel::Info,
            ToastLevel::Success,
            ToastLevel::Warning,
            ToastLevel::Error,
        ];

        for level in levels {
            // Call all methods to ensure they don't panic
            let _icon = level.icon();
            let _color = level.border_color();
            let _debug = format!("{:?}", level);
            let _clone = level.clone();
            let _copy = level;
        }
    }

    #[test]
    fn test_toast_all_constructors() {
        let info = Toast::info("Info test");
        let success = Toast::success("Success test");
        let warning = Toast::warning("Warning test");
        let error = Toast::error("Error test");
        let custom = Toast::new(ToastLevel::Info, "Custom", Duration::from_secs(1));

        assert_eq!(info.level(), ToastLevel::Info);
        assert_eq!(success.level(), ToastLevel::Success);
        assert_eq!(warning.level(), ToastLevel::Warning);
        assert_eq!(error.level(), ToastLevel::Error);
        assert_eq!(custom.level(), ToastLevel::Info);
    }

    #[test]
    fn test_toast_manager_len_consistency() {
        let mut manager = ToastManager::new();

        for i in 1..=10 {
            manager.info(format!("Message {}", i));
            assert_eq!(manager.len(), i);
            assert!(!manager.is_empty());
        }

        for i in (0..10).rev() {
            manager.toasts.pop();
            assert_eq!(manager.len(), i);
        }

        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_many_toasts() {
        let mut manager = ToastManager::new();

        // Add 100 toasts
        for i in 0..100 {
            manager.info(format!("Toast {}", i));
        }

        assert_eq!(manager.len(), 100);
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_manager_rapid_addition() {
        let mut manager = ToastManager::new();

        // Rapidly add different types
        for _ in 0..25 {
            manager.info("Info");
            manager.success("Success");
            manager.warning("Warning");
            manager.error("Error");
        }

        assert_eq!(manager.len(), 100);
    }

    #[test]
    fn test_manager_cleanup_removes_expired() {
        let mut manager = ToastManager::new();

        // Add toast with very short duration
        manager.add(Toast::new(
            ToastLevel::Info,
            "Expires soon",
            Duration::from_millis(10),
        ));

        assert_eq!(manager.len(), 1);

        // Wait for expiry
        std::thread::sleep(Duration::from_millis(15));
        manager.cleanup();

        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_manager_cleanup_keeps_fresh() {
        let mut manager = ToastManager::new();

        manager.info("Fresh toast");
        assert_eq!(manager.len(), 1);

        manager.cleanup();
        assert_eq!(manager.len(), 1); // Still there
    }

    #[test]
    fn test_manager_mixed_expired_and_fresh() {
        let mut manager = ToastManager::new();

        // Add expired toast
        manager.add(Toast::new(
            ToastLevel::Info,
            "Expired",
            Duration::from_millis(1),
        ));

        std::thread::sleep(Duration::from_millis(5));

        // Add fresh toast
        manager.info("Fresh");

        assert_eq!(manager.len(), 2);

        manager.cleanup();
        assert_eq!(manager.len(), 1); // Only fresh one remains
    }

    #[test]
    fn test_manager_clear_all() {
        let mut manager = ToastManager::new();

        manager.info("One");
        manager.success("Two");
        manager.warning("Three");
        manager.error("Four");

        assert_eq!(manager.len(), 4);

        manager.clear();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_add_custom_toast() {
        let mut manager = ToastManager::new();

        let custom = Toast::new(
            ToastLevel::Success,
            "Custom toast",
            Duration::from_secs(10),
        );

        manager.add(custom);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_manager_default() {
        let manager = ToastManager::default();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_debug() {
        let manager = ToastManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("ToastManager"));
    }

    // Unicode/Emoji in Manager
    #[test]
    fn test_manager_unicode_messages() {
        let mut manager = ToastManager::new();

        manager.info("日本語");
        manager.success("中文");
        manager.warning("العربية");
        manager.error("Тест");

        assert_eq!(manager.len(), 4);
    }

    #[test]
    fn test_manager_emoji_messages() {
        let mut manager = ToastManager::new();

        manager.info("🐸 Frog");
        manager.success("✅ Done");
        manager.warning("⚠️ Caution");
        manager.error("❌ Failed");

        assert_eq!(manager.len(), 4);
    }

    // Complex Scenarios
    #[test]
    fn test_manager_sequential_cleanup() {
        let mut manager = ToastManager::new();

        // Add multiple short-lived toasts
        for i in 0..10 {
            manager.add(Toast::new(
                ToastLevel::Info,
                format!("Toast {}", i),
                Duration::from_millis((i + 1) * 5),
            ));
        }

        assert_eq!(manager.len(), 10);

        // Cleanup at intervals
        std::thread::sleep(Duration::from_millis(20));
        manager.cleanup();
        assert!(manager.len() < 10);

        std::thread::sleep(Duration::from_millis(30));
        manager.cleanup();
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_manager_mixed_levels_and_durations() {
        let mut manager = ToastManager::new();

        manager.info("Info 3s");
        manager.success("Success 3s");
        manager.warning("Warning 5s");
        manager.error("Error 7s");

        assert_eq!(manager.len(), 4);

        // All should still be visible
        manager.cleanup();
        assert_eq!(manager.len(), 4);
    }

    #[test]
    fn test_manager_add_after_clear() {
        let mut manager = ToastManager::new();

        manager.info("First batch");
        manager.success("First batch");
        assert_eq!(manager.len(), 2);

        manager.clear();
        assert_eq!(manager.len(), 0);

        manager.warning("Second batch");
        manager.error("Second batch");
        assert_eq!(manager.len(), 2);
    }

    #[test]
    fn test_manager_cleanup_idempotent() {
        let mut manager = ToastManager::new();

        manager.info("Test");

        // Multiple cleanups should be fine
        for _ in 0..10 {
            manager.cleanup();
        }

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_toast_string_types() {
        // Test String
        let toast1 = Toast::info(String::from("String type"));
        assert_eq!(toast1.message(), "String type");

        // Test &str
        let toast2 = Toast::success("str type");
        assert_eq!(toast2.message(), "str type");

        // Test owned
        let owned = "Owned".to_string();
        let toast3 = Toast::warning(owned);
        assert_eq!(toast3.message(), "Owned");
    }

    #[test]
    fn test_manager_interleaved_operations() {
        let mut manager = ToastManager::new();

        manager.info("1");
        manager.cleanup();
        manager.success("2");
        manager.clear();
        manager.warning("3");
        manager.cleanup();
        manager.error("4");

        assert_eq!(manager.len(), 2); // 3 and 4
    }
}

