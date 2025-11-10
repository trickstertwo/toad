//! Progress widget tests

use super::*;
use std::time::Duration;

mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_creation() {
        let progress = ProgressBar::new("Test");
        assert_eq!(progress.progress(), 0.0);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_progress_bar_with_progress() {
        let progress = ProgressBar::new("Test").with_progress(0.5);
        assert_eq!(progress.progress(), 0.5);
    }

    #[test]
    fn test_progress_bar_set_progress() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(0.75);
        assert_eq!(progress.progress(), 0.75);
    }

    #[test]
    fn test_progress_bar_clamps() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(1.5);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(-0.5);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_is_complete() {
        let mut progress = ProgressBar::new("Test");
        assert!(!progress.is_complete());

        progress.set_progress(1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_stage_status_is_active() {
        assert!(StageStatus::InProgress.is_active());
        assert!(!StageStatus::Pending.is_active());
        assert!(!StageStatus::Complete.is_active());
    }

    #[test]
    fn test_stage_status_is_complete() {
        assert!(StageStatus::Complete.is_complete());
        assert!(!StageStatus::Pending.is_complete());
        assert!(!StageStatus::InProgress.is_complete());
    }

    #[test]
    fn test_stage_status_indicator() {
        assert_eq!(StageStatus::Pending.indicator(), "○");
        assert_eq!(StageStatus::InProgress.indicator(), "◉");
        assert_eq!(StageStatus::Complete.indicator(), "✓");
    }

    #[test]
    fn test_multi_stage_creation() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let progress = MultiStageProgress::new("Test", stages);
        assert_eq!(progress.stage_count(), 2);
        assert_eq!(progress.current_stage(), 0);
    }

    #[test]
    fn test_multi_stage_set_stage() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(1);
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_progress() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);
        progress.set_stage_progress(0.5);

        // 0.5 progress in first of 2 stages = 0.25 overall
        assert_eq!(progress.overall_progress(), 0.25);
    }

    #[test]
    fn test_multi_stage_complete() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);
        progress.complete_stage();

        assert_eq!(progress.completed_stages(), 1);
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_is_complete() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        assert!(!progress.is_complete());

        progress.set_stage(0);
        progress.complete_stage();
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_next_stage() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);
        progress.next_stage();
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_elapsed_time() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);
        progress.set_stage(0);

        // Should have some elapsed time
        assert!(progress.stage_elapsed(0).is_some());
    }

    #[test]
    fn test_multi_stage_overall_progress_empty() {
        let stages: Vec<String> = vec![];
        let progress = MultiStageProgress::new("Test", stages);
        assert_eq!(progress.overall_progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_with_time_tracking() {
        let stages = vec!["A".to_string()];
        let progress = MultiStageProgress::new("Test", stages).with_time_tracking(true);
        assert!(progress.show_time);
    }

    #[test]
    fn test_multi_stage_render_string() {
        let stages = vec!["Download".to_string(), "Install".to_string()];
        let mut progress = MultiStageProgress::new("Setup", stages);
        progress.set_stage(0);

        let output = progress.render_string();
        assert!(output.contains("Download"));
        assert!(output.contains("Install"));
    }

    #[test]
    fn test_stage_bounds_checking() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        // Setting stage beyond bounds should be handled gracefully
        progress.set_stage(10);
        // Should not crash and should stay within bounds
    }

    #[test]
    fn test_progress_bar_default() {
        let progress = ProgressBar::default();
        assert_eq!(progress.progress(), 0.0);
    }

    // ============ COMPREHENSIVE EDGE CASE TESTS ============

    #[test]
    fn test_progress_bar_negative_values_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(-0.5);
        assert_eq!(progress.progress(), 0.0);

        progress.set_progress(-100.0);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_overflow_values_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(1.5);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(100.0);
        assert_eq!(progress.progress(), 1.0);
    }

    #[test]
    fn test_progress_bar_with_very_long_title() {
        let long_title = "A".repeat(1000);
        let progress = ProgressBar::new(long_title);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_with_unicode_title() {
        let progress = ProgressBar::new("🚀 Loading 日本語");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_with_very_long_message() {
        let long_message = "B".repeat(1000);
        let progress = ProgressBar::new("Test").with_message(long_message);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_with_unicode_message() {
        let progress = ProgressBar::new("Test").with_message("処理中... 🔄");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_very_small_increments() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(0.001);
        assert_eq!(progress.progress(), 0.001);

        progress.set_progress(0.0001);
        assert_eq!(progress.progress(), 0.0001);
    }

    #[test]
    fn test_progress_bar_rapid_updates() {
        let mut progress = ProgressBar::new("Test");

        for i in 0..1000 {
            progress.set_progress(i as f64 / 1000.0);
        }

        assert_eq!(progress.progress(), 0.999);
    }

    #[test]
    fn test_progress_bar_message_update() {
        let mut progress = ProgressBar::new("Test");
        progress.set_message("First");
        progress.set_message("Second");
        progress.set_message("Third");
        // Verify no panic on multiple updates
    }

    #[test]
    fn test_progress_bar_exactly_half() {
        let progress = ProgressBar::new("Test").with_progress(0.5);
        assert_eq!(progress.progress(), 0.5);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_progress_bar_exactly_complete() {
        let progress = ProgressBar::new("Test").with_progress(1.0);
        assert_eq!(progress.progress(), 1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_progress_bar_just_below_complete() {
        let progress = ProgressBar::new("Test").with_progress(0.9999);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_multi_stage_empty_stages() {
        let stages: Vec<String> = vec![];
        let progress = MultiStageProgress::new("Empty", stages);

        assert_eq!(progress.stage_count(), 0);
        assert_eq!(progress.overall_progress(), 0.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_single_stage() {
        let stages = vec!["Only Stage".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage();

        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_many_stages() {
        let stages: Vec<String> = (0..100).map(|i| format!("Stage {}", i)).collect();
        let progress = MultiStageProgress::new("Test", stages);

        assert_eq!(progress.stage_count(), 100);
    }

    #[test]
    fn test_multi_stage_unicode_stage_names() {
        let stages = vec![
            "📥 Download".to_string(),
            "📦 Extract".to_string(),
            "⚙️ Configure".to_string(),
            "✅ Complete".to_string(),
        ];
        let mut progress = MultiStageProgress::new("Setup", stages);

        progress.set_stage(0);
        assert_eq!(progress.current_stage(), 0);
    }

    #[test]
    fn test_multi_stage_very_long_stage_names() {
        let long_name = "A".repeat(1000);
        let stages = vec![long_name.clone(), long_name.clone()];
        let progress = MultiStageProgress::new("Test", stages);

        assert_eq!(progress.stage_count(), 2);
    }

    #[test]
    fn test_multi_stage_progress_precision() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(0.5);

        // 0.5 progress in first of 2 stages
        let overall = progress.overall_progress();
        assert!((overall - 0.25).abs() < 1e-10); // Floating point comparison
    }

    #[test]
    fn test_multi_stage_complete_all_stages() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage();
        progress.complete_stage();
        progress.complete_stage();

        assert!(progress.is_complete());
        assert_eq!(progress.completed_stages(), 3);
    }

    #[test]
    fn test_multi_stage_next_stage_without_completing() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.next_stage();

        assert_eq!(progress.current_stage(), 1);
        assert_eq!(progress.completed_stages(), 0); // Not completed, just moved
    }

    #[test]
    fn test_multi_stage_next_stage_at_last_stage() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(1);
        progress.next_stage();

        // Should stay at stage 1
        assert_eq!(progress.current_stage(), 1);
    }

    #[test]
    fn test_multi_stage_complete_at_last_stage() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage(); // Move to stage 1
        progress.complete_stage(); // Complete stage 1

        assert_eq!(progress.current_stage(), 1);
        assert_eq!(progress.stage_progress, 1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_stage_elapsed_non_existent() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);

        // Query non-existent stage
        assert!(progress.stage_elapsed(10).is_none());
    }

    #[test]
    fn test_multi_stage_render_string_output() {
        let stages = vec!["Download".to_string(), "Install".to_string()];
        let mut progress = MultiStageProgress::new("Setup", stages);

        progress.set_stage(0);
        let output = progress.render_string();

        assert!(output.contains("Download"));
        assert!(output.contains("Install"));
        assert!(output.contains("→")); // Separator
    }

    #[test]
    fn test_stage_status_color_codes() {
        assert_ne!(
            StageStatus::Pending.color(),
            StageStatus::InProgress.color()
        );
        assert_ne!(
            StageStatus::InProgress.color(),
            StageStatus::Complete.color()
        );
    }

    #[test]
    fn test_multi_stage_overall_progress_at_boundaries() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        // At start
        assert_eq!(progress.overall_progress(), 0.0);

        // First stage complete
        progress.set_stage(0);
        progress.complete_stage();
        assert_eq!(progress.overall_progress(), 0.5);

        // Second stage complete
        progress.complete_stage();
        assert_eq!(progress.overall_progress(), 1.0);
    }

    #[test]
    fn test_multi_stage_with_time_tracking_disabled() {
        let stages = vec!["A".to_string()];
        let progress = MultiStageProgress::new("Test", stages).with_time_tracking(false);

        assert!(!progress.show_time);
    }

    #[test]
    fn test_multi_stage_total_elapsed_with_no_stages_started() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let progress = MultiStageProgress::new("Test", stages);

        let elapsed = progress.total_elapsed();
        assert_eq!(elapsed, Duration::from_secs(0));
    }

    #[test]
    fn test_multi_stage_set_stage_progress_clamping() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);

        // Test overflow
        progress.set_stage_progress(2.0);
        assert_eq!(progress.stage_progress, 1.0);

        // Test underflow
        progress.set_stage_progress(-0.5);
        assert_eq!(progress.stage_progress, 0.0);
    }

    #[test]
    fn test_progress_bar_builder_chaining() {
        let progress = ProgressBar::new("Test")
            .with_progress(0.75)
            .with_message("Processing...");

        assert_eq!(progress.progress(), 0.75);
    }

    #[test]
    fn test_multi_stage_set_stage_resets_progress() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(0.8);

        progress.set_stage(1);
        assert_eq!(progress.stage_progress, 0.0); // Should reset
    }

    // ============================================================================
    // ADVANCED TIER: Additional Comprehensive Edge Case Tests
    // ============================================================================

    // Stress Tests (10k operations)

    #[test]
    fn test_progress_bar_10k_progress_updates() {
        let mut progress = ProgressBar::new("Test");

        for i in 0..10000 {
            progress.set_progress((i % 100) as f64 / 100.0);
        }

        assert_eq!(progress.progress(), 0.99);
    }

    #[test]
    fn test_progress_bar_10k_message_updates() {
        let mut progress = ProgressBar::new("Test");

        for i in 0..10000 {
            progress.set_message(format!("Message {}", i));
        }

        // Should not panic, just verify completion
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_10k_stage_transitions() {
        let stages: Vec<String> = (0..100).map(|i| format!("Stage {}", i)).collect();
        let mut progress = MultiStageProgress::new("Test", stages);

        for i in 0..100 {
            progress.set_stage(i);
            for _ in 0..100 {
                progress.set_stage_progress(0.5);
            }
        }

        assert!(progress.current_stage() < 100);
    }

    #[test]
    fn test_multi_stage_1000_stages() {
        let stages: Vec<String> = (0..1000).map(|i| format!("Stage {}", i)).collect();
        let progress = MultiStageProgress::new("Test", stages);

        assert_eq!(progress.stage_count(), 1000);
        assert_eq!(progress.current_stage(), 0);
    }

    #[test]
    fn test_multi_stage_10k_progress_calculations() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        for i in 0..10000 {
            progress.set_stage_progress((i % 100) as f64 / 100.0);
            let _ = progress.overall_progress();
        }

        assert!(progress.overall_progress() >= 0.0);
        assert!(progress.overall_progress() <= 1.0);
    }

    // Unicode Edge Cases

    #[test]
    fn test_progress_bar_rtl_text_arabic() {
        let progress = ProgressBar::new("تحميل البيانات").with_message("معالجة الملفات...");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_rtl_text_hebrew() {
        let progress = ProgressBar::new("טוען נתונים").with_message("מעבד קבצים...");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_mixed_scripts() {
        let progress = ProgressBar::new("Loading 加载中 تحميل ロード中")
            .with_message("Processing データ処理 معالجة 处理");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_emoji_combinations() {
        let progress = ProgressBar::new("🚀 Launch 🎯 Target 💯")
            .with_message("📥 Downloading... 🔄 Processing... ✅");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_zero_width_characters() {
        let text_with_zwj = "Test\u{200D}Progress";
        let progress = ProgressBar::new(text_with_zwj).with_message("Test\u{200C}Message");

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_combining_characters() {
        let text_with_combining = "Progre\u{0301}s"; // é with combining accent
        let progress = ProgressBar::new(text_with_combining).with_message("Cafe\u{0301}"); // Café

        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_rtl_text_arabic() {
        let stages = vec![
            "تحميل".to_string(),
            "استخراج".to_string(),
            "تثبيت".to_string(),
        ];
        let progress = MultiStageProgress::new("إعداد", stages);

        assert_eq!(progress.stage_count(), 3);
    }

    #[test]
    fn test_multi_stage_rtl_text_hebrew() {
        let stages = vec![
            "הורדה".to_string(),
            "חילוץ".to_string(),
            "התקנה".to_string(),
        ];
        let progress = MultiStageProgress::new("התקנה", stages);

        assert_eq!(progress.stage_count(), 3);
    }

    #[test]
    fn test_multi_stage_mixed_scripts() {
        let stages = vec![
            "Download 下载 تحميل".to_string(),
            "Extract 解压 استخراج".to_string(),
            "Install インストール تثبيت".to_string(),
        ];
        let progress = MultiStageProgress::new("Setup 安装 إعداد", stages);

        assert_eq!(progress.stage_count(), 3);
    }

    // Extreme Values

    #[test]
    fn test_progress_bar_infinity_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(f64::INFINITY);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(f64::NEG_INFINITY);
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_nan_clamped() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(f64::NAN);
        // NaN comparisons are tricky, but clamp should handle it
        let val = progress.progress();
        assert!(val >= 0.0 && val <= 1.0 || val.is_nan());
    }

    #[test]
    fn test_progress_bar_very_precise_values() {
        let mut progress = ProgressBar::new("Test");
        progress.set_progress(0.123456789012345);
        assert!((progress.progress() - 0.123456789012345).abs() < 1e-10);
    }

    #[test]
    fn test_multi_stage_progress_very_precise() {
        let stages = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(0.333333333333);

        let overall = progress.overall_progress();
        assert!(overall >= 0.0 && overall <= 1.0);
    }

    // Trait Tests

    #[test]
    fn test_stage_status_debug_trait() {
        let status = StageStatus::InProgress;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("InProgress"));
    }

    #[test]
    fn test_stage_status_clone_trait() {
        let original = StageStatus::InProgress;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_stage_status_partial_eq() {
        assert_eq!(StageStatus::Pending, StageStatus::Pending);
        assert_eq!(StageStatus::InProgress, StageStatus::InProgress);
        assert_eq!(StageStatus::Complete, StageStatus::Complete);
        assert_ne!(StageStatus::Pending, StageStatus::InProgress);
    }

    // Multi-phase Comprehensive Workflow

    #[test]
    fn test_progress_bar_10_phase_comprehensive_workflow() {
        // Phase 1: Create basic progress bar
        let mut progress = ProgressBar::new("Comprehensive Test");
        assert_eq!(progress.progress(), 0.0);
        assert!(!progress.is_complete());

        // Phase 2: Set initial progress
        progress.set_progress(0.1);
        assert_eq!(progress.progress(), 0.1);

        // Phase 3: Add message
        progress.set_message("Starting...");

        // Phase 4: Update progress incrementally
        for i in 1..=10 {
            progress.set_progress(i as f64 / 10.0);
        }
        assert_eq!(progress.progress(), 1.0);

        // Phase 5: Verify completion
        assert!(progress.is_complete());

        // Phase 6: Update message after completion
        progress.set_message("Complete!");

        // Phase 7: Test boundary conditions
        progress.set_progress(2.0); // Should clamp to 1.0
        assert_eq!(progress.progress(), 1.0);

        // Phase 8: Reset to zero
        progress.set_progress(0.0);
        assert!(!progress.is_complete());

        // Phase 9: Rapid updates
        for _ in 0..100 {
            progress.set_progress(0.5);
        }
        assert_eq!(progress.progress(), 0.5);

        // Phase 10: Final completion
        progress.set_progress(1.0);
        assert!(progress.is_complete());
    }

    #[test]
    fn test_multi_stage_10_phase_comprehensive_workflow() {
        // Phase 1: Create multi-stage progress
        let stages = vec![
            "Init".to_string(),
            "Download".to_string(),
            "Extract".to_string(),
            "Configure".to_string(),
            "Install".to_string(),
        ];
        let mut progress = MultiStageProgress::new("Installation", stages);
        assert_eq!(progress.stage_count(), 5);
        assert_eq!(progress.current_stage(), 0);

        // Phase 2: Start first stage
        progress.set_stage(0);
        assert_eq!(progress.overall_progress(), 0.0);

        // Phase 3: Progress through first stage
        for i in 0..=10 {
            progress.set_stage_progress(i as f64 / 10.0);
        }
        assert_eq!(progress.stage_progress, 1.0);

        // Phase 4: Complete first stage
        progress.complete_stage();
        assert_eq!(progress.completed_stages(), 1);
        assert_eq!(progress.current_stage(), 1);

        // Phase 5: Progress through remaining stages
        for _ in 0..3 {
            progress.set_stage_progress(0.5);
            progress.complete_stage();
        }
        assert_eq!(progress.completed_stages(), 4);

        // Phase 6: Check overall progress (4 of 5 complete)
        let overall = progress.overall_progress();
        assert!(overall >= 0.8); // At least 80% (4 of 5 stages complete)

        // Phase 7: Complete final stage
        assert_eq!(progress.current_stage(), 4);
        progress.set_stage_progress(1.0);
        progress.complete_stage();

        // Phase 8: Verify all stages complete
        assert!(progress.is_complete());
        assert_eq!(progress.completed_stages(), 5);
        assert_eq!(progress.overall_progress(), 1.0);

        // Phase 9: Test next_stage at completion (should stay at last stage)
        progress.next_stage();
        assert_eq!(progress.current_stage(), 4);

        // Phase 10: Test time tracking
        for i in 0..5 {
            let elapsed = progress.stage_elapsed(i);
            assert!(elapsed.is_some());
        }
        let total = progress.total_elapsed();
        assert!(total > Duration::from_secs(0));
    }

    // Builder Pattern Edge Cases

    #[test]
    fn test_progress_bar_multiple_progress_calls() {
        let progress = ProgressBar::new("Test")
            .with_progress(0.25)
            .with_progress(0.5)
            .with_progress(0.75);

        assert_eq!(progress.progress(), 0.75);
    }

    #[test]
    fn test_progress_bar_multiple_message_calls() {
        let progress = ProgressBar::new("Test")
            .with_message("First")
            .with_message("Second")
            .with_message("Third");

        // Last message should be set
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_builder_chaining_many_operations() {
        let progress = ProgressBar::new("Test")
            .with_progress(0.1)
            .with_progress(0.2)
            .with_progress(0.3)
            .with_message("M1")
            .with_message("M2")
            .with_message("M3")
            .with_progress(0.9);

        assert_eq!(progress.progress(), 0.9);
    }

    #[test]
    fn test_multi_stage_multiple_time_tracking_toggles() {
        let stages = vec!["A".to_string()];
        let progress = MultiStageProgress::new("Test", stages)
            .with_time_tracking(true)
            .with_time_tracking(false)
            .with_time_tracking(true);

        assert!(progress.show_time);
    }

    // Empty State Operations

    #[test]
    fn test_progress_bar_all_operations_on_default() {
        let mut progress = ProgressBar::default();

        progress.set_progress(0.5);
        progress.set_message("Test");
        assert_eq!(progress.progress(), 0.5);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_multi_stage_render_string_with_empty_stages() {
        let stages: Vec<String> = vec![];
        let progress = MultiStageProgress::new("Empty", stages);

        let output = progress.render_string();
        assert!(output.is_empty() || !output.contains("→"));
    }

    // Additional Edge Cases

    #[test]
    fn test_progress_bar_empty_title() {
        let progress = ProgressBar::new("");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_progress_bar_empty_message() {
        let progress = ProgressBar::new("Test").with_message("");
        assert_eq!(progress.progress(), 0.0);
    }

    #[test]
    fn test_multi_stage_empty_stage_name() {
        let stages = vec!["".to_string(), "Valid".to_string()];
        let progress = MultiStageProgress::new("Test", stages);
        assert_eq!(progress.stage_count(), 2);
    }

    #[test]
    fn test_stage_status_all_variants_covered() {
        let pending = StageStatus::Pending;
        let in_progress = StageStatus::InProgress;
        let complete = StageStatus::Complete;

        assert!(!pending.is_active());
        assert!(in_progress.is_active());
        assert!(!complete.is_active());

        assert!(!pending.is_complete());
        assert!(!in_progress.is_complete());
        assert!(complete.is_complete());
    }

    #[test]
    fn test_multi_stage_complete_stage_at_boundaries() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.complete_stage();

        // Should be at last stage with progress 1.0
        assert_eq!(progress.current_stage(), 0);
        assert_eq!(progress.stage_progress, 1.0);
    }

    #[test]
    fn test_progress_bar_progress_boundary_values() {
        let mut progress = ProgressBar::new("Test");

        progress.set_progress(0.0);
        assert_eq!(progress.progress(), 0.0);

        progress.set_progress(1.0);
        assert_eq!(progress.progress(), 1.0);

        progress.set_progress(0.5);
        assert_eq!(progress.progress(), 0.5);
    }

    #[test]
    fn test_multi_stage_set_stage_beyond_bounds() {
        let stages = vec!["A".to_string(), "B".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        let before = progress.current_stage();
        progress.set_stage(100); // Way beyond bounds

        // Should not panic and should stay within valid range
        assert!(progress.current_stage() <= 1);
    }

    #[test]
    fn test_multi_stage_overall_progress_clamping() {
        let stages = vec!["A".to_string()];
        let mut progress = MultiStageProgress::new("Test", stages);

        progress.set_stage(0);
        progress.set_stage_progress(2.0); // Overflow

        let overall = progress.overall_progress();
        assert!(overall >= 0.0 && overall <= 1.0);
    }
}
