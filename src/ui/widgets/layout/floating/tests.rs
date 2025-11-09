//! Floating window tests

use super::*;
use ratatui::layout::Rect;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_position() {
        let pos = WindowPosition::new(10, 20);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
    }

    #[test]
    fn test_window_position_centered() {
        let pos = WindowPosition::centered(40, 10, 80, 24);
        assert_eq!(pos.x, 20); // (80 - 40) / 2
        assert_eq!(pos.y, 7); // (24 - 10) / 2
    }

    #[test]
    fn test_floating_window_creation() {
        let window = FloatingWindow::new("Test", "Content");
        assert_eq!(window.title(), "Test");
        assert_eq!(window.content(), "Content");
        assert!(window.is_visible());
        assert!(!window.is_minimized());
    }

    #[test]
    fn test_window_visibility() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.hide();
        assert!(!window.is_visible());

        window.show();
        assert!(window.is_visible());

        window.toggle();
        assert!(!window.is_visible());
    }

    #[test]
    fn test_window_minimize() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.minimize();
        assert!(window.is_minimized());

        window.restore();
        assert!(!window.is_minimized());

        window.toggle_minimize();
        assert!(window.is_minimized());
    }

    #[test]
    fn test_window_position_update() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.set_position(50, 10);
        let pos = window.get_position();
        assert_eq!(pos.x, 50);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_window_move_by() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(10, 10);

        window.move_by(5, 3);
        let pos = window.get_position();
        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 13);

        window.move_by(-5, -3);
        let pos = window.get_position();
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_window_size() {
        let mut window = FloatingWindow::new("Test", "Content");

        window.set_size(60, 20);
        let (w, h) = window.get_size();
        assert_eq!(w, 60);
        assert_eq!(h, 20);
    }

    #[test]
    fn test_window_center() {
        let mut window = FloatingWindow::new("Test", "Content").size(40, 10);

        window.center(80, 24);
        let pos = window.get_position();
        assert_eq!(pos.x, 20);
        assert_eq!(pos.y, 7);
    }

    #[test]
    fn test_manager_creation() {
        let manager = FloatingWindowManager::new();
        assert_eq!(manager.window_count(), 0);
    }

    #[test]
    fn test_manager_add_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        assert_eq!(manager.window_count(), 1);
        assert!(manager.focused_window().is_some());
    }

    #[test]
    fn test_manager_remove_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));

        let removed = manager.remove_window(0);
        assert!(removed.is_some());
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_manager_focus_navigation() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));
        manager.add_window(FloatingWindow::new("Window 3", "Content"));

        manager.focus_next();
        assert_eq!(manager.focused, Some(1));

        manager.focus_next();
        assert_eq!(manager.focused, Some(2));

        manager.focus_next(); // Should wrap
        assert_eq!(manager.focused, Some(0));

        manager.focus_previous();
        assert_eq!(manager.focused, Some(2));
    }

    #[test]
    fn test_manager_close_focused() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));

        let closed = manager.close_focused();
        assert!(closed.is_some());
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_non_draggable_window() {
        let mut window = FloatingWindow::new("Test", "Content")
            .draggable(false)
            .position(10, 10);

        window.move_by(5, 5);
        let pos = window.get_position();
        assert_eq!(pos.x, 10); // Should not move
        assert_eq!(pos.y, 10);
    }

    // ============================================================================
    // ADDITIONAL COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Layouts)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_manager_many_windows_1000() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..1000 {
            manager.add_window(FloatingWindow::new(
                format!("Window {}", i),
                format!("Content {}", i),
            ));
        }
        assert_eq!(manager.window_count(), 1000);
    }

    #[test]
    fn test_manager_rapid_add_remove_1000() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..1000 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
            if i % 2 == 0 && manager.window_count() > 0 {
                manager.remove_window(0);
            }
        }
        assert!(manager.window_count() >= 500);
    }

    #[test]
    fn test_window_rapid_move_operations_1000() {
        let mut window = FloatingWindow::new("Test", "Content");
        for _ in 0..500 {
            window.move_by(1, 1);
        }
        for _ in 0..500 {
            window.move_by(-1, -1);
        }
        let pos = window.get_position();
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_manager_rapid_focus_navigation() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..10 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
        }

        for _ in 0..1000 {
            manager.focus_next();
        }

        // Should have wrapped around many times, still have valid focus
        assert!(manager.focused_window().is_some());
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_window_unicode_title() {
        let window = FloatingWindow::new("日本語 Title 🚀", "Content");
        assert_eq!(window.title(), "日本語 Title 🚀");
    }

    #[test]
    fn test_window_rtl_title() {
        let window = FloatingWindow::new("مرحبا بك", "Content");
        assert_eq!(window.title(), "مرحبا بك");
    }

    #[test]
    fn test_window_unicode_content() {
        let content = "🚀 Rocket\n日本語\nمرحبا\nמזל טוב";
        let window = FloatingWindow::new("Test", content);
        assert!(window.content().contains('🚀'));
        assert!(window.content().contains("日本語"));
    }

    #[test]
    fn test_window_very_long_unicode_title() {
        let title = "日本語 ".repeat(1000);
        let window = FloatingWindow::new(title.clone(), "Content");
        assert_eq!(window.title(), title);
    }

    #[test]
    fn test_window_emoji_only_title() {
        let window = FloatingWindow::new("🚀🐸💚🎉", "Content");
        assert_eq!(window.title(), "🚀🐸💚🎉");
    }

    #[test]
    fn test_window_combining_characters() {
        let window = FloatingWindow::new("é̂ñ̃ỹ̀", "Café naïve");
        assert!(window.title().len() > 4);
        assert!(window.content().len() > 10);
    }

    // ============ Position/Size Edge Cases ============

    #[test]
    fn test_window_position_max_u16() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(u16::MAX, u16::MAX);
        let pos = window.get_position();
        assert_eq!(pos.x, u16::MAX);
        assert_eq!(pos.y, u16::MAX);
    }

    #[test]
    fn test_window_size_max_u16() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_size(u16::MAX, u16::MAX);
        let (w, h) = window.get_size();
        assert_eq!(w, u16::MAX);
        assert_eq!(h, u16::MAX);
    }

    #[test]
    fn test_window_size_zero() {
        let window = FloatingWindow::new("Test", "Content").size(0, 0);
        let (w, h) = window.get_size();
        assert_eq!(w, 0);
        assert_eq!(h, 0);
    }

    #[test]
    fn test_window_move_negative_from_zero() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(0, 0);
        window.move_by(-10, -10);
        let pos = window.get_position();
        // Should saturate/wrap to 0 or max depending on implementation
        assert!(pos.x == 0 || pos.x > 60000); // Either saturated at 0 or wrapped
        assert!(pos.y == 0 || pos.y > 60000);
    }

    #[test]
    fn test_window_move_positive_overflow() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(u16::MAX - 5, u16::MAX - 5);
        window.move_by(10, 10);
        let pos = window.get_position();
        // Should overflow or saturate
        assert!(pos.x >= u16::MAX - 5 || pos.x < 10);
        assert!(pos.y >= u16::MAX - 5 || pos.y < 10);
    }

    #[test]
    fn test_window_center_zero_terminal_size() {
        let mut window = FloatingWindow::new("Test", "Content").size(40, 10);
        window.center(0, 0);
        let pos = window.get_position();
        // Should handle gracefully (likely position at 0,0)
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_window_center_large_window_small_terminal() {
        let mut window = FloatingWindow::new("Test", "Content").size(100, 50);
        window.center(80, 24);
        let pos = window.get_position();
        // Window larger than terminal, should saturate at 0
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_position_centered_extreme_sizes() {
        let pos = WindowPosition::centered(u16::MAX, u16::MAX, u16::MAX, u16::MAX);
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    // ============ Window Manager Edge Cases ============

    #[test]
    fn test_manager_remove_from_empty() {
        let mut manager = FloatingWindowManager::new();
        let removed = manager.remove_window(0);
        assert!(removed.is_none());
    }

    #[test]
    fn test_manager_focus_navigation_empty() {
        let mut manager = FloatingWindowManager::new();
        manager.focus_next();
        manager.focus_previous();
        assert!(manager.focused_window().is_none());
    }

    #[test]
    fn test_manager_focus_navigation_single_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        manager.focus_next();
        assert_eq!(manager.focused, Some(0));

        manager.focus_previous();
        assert_eq!(manager.focused, Some(0));
    }

    #[test]
    fn test_manager_close_focused_empty() {
        let mut manager = FloatingWindowManager::new();
        let closed = manager.close_focused();
        assert!(closed.is_none());
    }

    #[test]
    fn test_manager_close_all_windows() {
        let mut manager = FloatingWindowManager::new();
        for i in 0..10 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
        }

        while manager.window_count() > 0 {
            manager.close_focused();
        }

        assert_eq!(manager.window_count(), 0);
        assert!(manager.focused_window().is_none());
    }

    #[test]
    fn test_manager_remove_invalid_index() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        let removed = manager.remove_window(100);
        assert!(removed.is_none());
        assert_eq!(manager.window_count(), 1);
    }

    // ============ Serialize/Deserialize Tests ============

    #[test]
    fn test_window_position_serialize_deserialize() {
        let pos = WindowPosition::new(42, 84);
        let json = serde_json::to_string(&pos).unwrap();
        let deserialized: WindowPosition = serde_json::from_str(&json).unwrap();

        assert_eq!(pos.x, deserialized.x);
        assert_eq!(pos.y, deserialized.y);
    }

    #[test]
    fn test_floating_window_serialize_deserialize() {
        let window = FloatingWindow::new("Test Title", "Test Content")
            .position(10, 20)
            .size(50, 15)
            .draggable(false);

        let json = serde_json::to_string(&window).unwrap();
        let deserialized: FloatingWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(window.title(), deserialized.title());
        assert_eq!(window.content(), deserialized.content());
        assert_eq!(window.get_position().x, deserialized.get_position().x);
        assert_eq!(window.get_position().y, deserialized.get_position().y);
    }

    // ============ Clone/Debug Traits ============

    #[test]
    fn test_window_position_clone() {
        let pos = WindowPosition::new(15, 25);
        let cloned = pos;
        assert_eq!(pos.x, cloned.x);
        assert_eq!(pos.y, cloned.y);
    }

    #[test]
    fn test_window_position_debug() {
        let pos = WindowPosition::new(10, 20);
        let debug_str = format!("{:?}", pos);
        assert!(debug_str.contains("WindowPosition"));
    }

    #[test]
    fn test_window_position_partial_eq() {
        let pos1 = WindowPosition::new(10, 20);
        let pos2 = WindowPosition::new(10, 20);
        let pos3 = WindowPosition::new(15, 25);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_floating_window_clone() {
        let window = FloatingWindow::new("Title", "Content")
            .position(10, 20)
            .size(50, 15);

        let cloned = window.clone();
        assert_eq!(window.title(), cloned.title());
        assert_eq!(window.content(), cloned.content());
        assert_eq!(window.get_position(), cloned.get_position());
    }

    #[test]
    fn test_floating_window_debug() {
        let window = FloatingWindow::new("Test", "Content");
        let debug_str = format!("{:?}", window);
        assert!(debug_str.contains("FloatingWindow"));
    }

    // ============ Complex Workflow Tests ============

    #[test]
    fn test_window_complete_workflow() {
        let mut window = FloatingWindow::new("Test Window", "Initial content");

        // Move and resize
        window.set_position(10, 10);
        window.set_size(60, 20);

        // Minimize and restore
        window.minimize();
        assert!(window.is_minimized());

        window.restore();
        assert!(!window.is_minimized());

        // Hide and show
        window.hide();
        assert!(!window.is_visible());

        window.show();
        assert!(window.is_visible());

        // Move around
        window.move_by(5, 5);
        let pos = window.get_position();
        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 15);

        // Update content
        window.set_content("Updated content");
        assert_eq!(window.content(), "Updated content");
    }

    #[test]
    fn test_manager_complete_workflow() {
        let mut manager = FloatingWindowManager::new();

        // Add multiple windows
        manager.add_window(FloatingWindow::new("Window 1", "Content 1"));
        manager.add_window(FloatingWindow::new("Window 2", "Content 2"));
        manager.add_window(FloatingWindow::new("Window 3", "Content 3"));

        assert_eq!(manager.window_count(), 3);

        // Navigate focus
        manager.focus_next();
        manager.focus_next();
        assert_eq!(manager.focused, Some(2));

        // Close focused window
        manager.close_focused();
        assert_eq!(manager.window_count(), 2);

        // Remove specific window
        manager.remove_window(0);
        assert_eq!(manager.window_count(), 1);

        // Clear remaining
        manager.close_focused();
        assert_eq!(manager.window_count(), 0);
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let window = FloatingWindow::new("Test", "Content")
            .position(10, 20)
            .size(60, 15)
            .draggable(false)
            .closable(false);

        assert_eq!(window.get_position().x, 10);
        assert_eq!(window.get_position().y, 20);
        assert_eq!(window.get_size(), (60, 15));
        assert!(!window.is_draggable());
        assert!(!window.is_closable());
    }

    #[test]
    fn test_window_toggle_operations() {
        let mut window = FloatingWindow::new("Test", "Content");

        // Toggle visibility
        window.toggle();
        assert!(!window.is_visible());
        window.toggle();
        assert!(window.is_visible());

        // Toggle minimize
        window.toggle_minimize();
        assert!(window.is_minimized());
        window.toggle_minimize();
        assert!(!window.is_minimized());
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_floating_window_stress() {
        let mut manager = FloatingWindowManager::new();

        // Phase 1: Add many windows with varied configurations
        for i in 0..100 {
            let title = match i % 4 {
                0 => format!("ASCII Window {}", i),
                1 => format!("🚀 Emoji Window {}", i),
                2 => format!("日本語 Window {}", i),
                _ => format!("مرحبا Window {}", i),
            };

            let mut window = FloatingWindow::new(title, format!("Content {}", i))
                .position((i * 5) as u16, (i * 3) as u16)
                .size(40 + (i % 20) as u16, 10 + (i % 10) as u16);

            if i % 2 == 0 {
                window.minimize();
            }

            manager.add_window(window);
        }

        assert_eq!(manager.window_count(), 100);

        // Phase 2: Focus navigation
        for _ in 0..200 {
            manager.focus_next();
        }
        assert!(manager.focused_window().is_some());

        // Phase 3: Close every other window
        for _ in 0..50 {
            manager.close_focused();
            manager.focus_next();
        }

        assert_eq!(manager.window_count(), 50);

        // Phase 4: Modify remaining windows
        for _ in 0..50 {
            if let Some(window) = manager.focused_window_mut() {
                window.move_by(1, 1);
                window.toggle_minimize();
            }
            manager.focus_next();
        }

        // Phase 5: Close all remaining windows
        while manager.window_count() > 0 {
            manager.close_focused();
        }

        assert_eq!(manager.window_count(), 0);
    }

    // ============ Empty/Whitespace Content ============

    #[test]
    fn test_window_empty_title() {
        let window = FloatingWindow::new("", "Content");
        assert_eq!(window.title(), "");
    }

    #[test]
    fn test_window_empty_content() {
        let window = FloatingWindow::new("Title", "");
        assert_eq!(window.content(), "");
    }

    #[test]
    fn test_window_whitespace_only_content() {
        let window = FloatingWindow::new("Title", "     \n  \n    ");
        assert!(window.content().contains(' '));
        assert!(window.content().contains('\n'));
    }

    // ============ Content Update Tests ============

    #[test]
    fn test_window_set_title() {
        let mut window = FloatingWindow::new("Old Title", "Content");
        window.set_title("New Title");
        assert_eq!(window.title(), "New Title");
    }

    #[test]
    fn test_window_set_content_multiple_times() {
        let mut window = FloatingWindow::new("Title", "Content 1");
        window.set_content("Content 2");
        assert_eq!(window.content(), "Content 2");

        window.set_content("Content 3");
        assert_eq!(window.content(), "Content 3");
    }

    // ============ Default Trait Test ============

    #[test]
    fn test_floating_window_default() {
        let window = FloatingWindow::default();
        assert_eq!(window.title(), "Window");
        assert_eq!(window.content(), "");
        assert!(window.is_visible());
        assert!(!window.is_minimized());
        assert!(window.is_draggable());
        assert!(window.is_closable());
    }

    #[test]
    fn test_manager_default() {
        let manager = FloatingWindowManager::default();
        assert_eq!(manager.window_count(), 0);
        assert!(manager.focused_window().is_none());
    }

    // ============ Extreme Stress Tests (10k operations) ============

    #[test]
    fn test_window_extreme_move_operations_10k() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(5000, 5000);

        for _ in 0..5000 {
            window.move_by(1, 1);
        }
        for _ in 0..5000 {
            window.move_by(-1, -1);
        }

        let pos = window.get_position();
        assert_eq!(pos.x, 5000);
        assert_eq!(pos.y, 5000);
    }

    #[test]
    fn test_window_rapid_toggle_visibility_10k() {
        let mut window = FloatingWindow::new("Test", "Content");

        for _ in 0..10000 {
            window.toggle();
        }

        // Even number of toggles, should be back to original state (visible)
        assert!(window.is_visible());
    }

    #[test]
    fn test_window_rapid_toggle_minimize_10k() {
        let mut window = FloatingWindow::new("Test", "Content");

        for _ in 0..10000 {
            window.toggle_minimize();
        }

        // Even number of toggles, should be back to original state (not minimized)
        assert!(!window.is_minimized());
    }

    #[test]
    fn test_window_rapid_size_changes_10k() {
        let mut window = FloatingWindow::new("Test", "Content");

        for i in 1..=10000 {
            window.set_size((i % 100) as u16 + 10, (i % 50) as u16 + 5);
        }

        // Final size should be from last iteration (i=10000)
        let (w, h) = window.get_size();
        assert_eq!(w, 10); // 10000 % 100 + 10 = 0 + 10
        assert_eq!(h, 5); // 10000 % 50 + 5 = 0 + 5
    }

    #[test]
    fn test_manager_extreme_add_remove_10k() {
        let mut manager = FloatingWindowManager::new();

        for i in 0..10000 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), "Content"));
            if i % 3 == 0 && manager.window_count() > 0 {
                manager.remove_window(0);
            }
        }

        assert!(manager.window_count() > 6600); // Approximately 2/3 remain
    }

    // ============ Non-closable Window Tests ============

    #[test]
    fn test_non_closable_window_cannot_close() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Non-closable", "Content").closable(false));

        let closed = manager.close_focused();
        assert!(closed.is_none());
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_mixed_closable_non_closable() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Closable", "Content"));
        manager.add_window(FloatingWindow::new("Non-closable", "Content").closable(false));
        manager.add_window(FloatingWindow::new("Closable 2", "Content"));

        manager.set_focus(1); // Focus non-closable
        let closed = manager.close_focused();
        assert!(closed.is_none());
        assert_eq!(manager.window_count(), 3);

        manager.set_focus(0); // Focus closable
        let closed = manager.close_focused();
        assert!(closed.is_some());
        assert_eq!(manager.window_count(), 2);
    }

    // ============ Manager Set Focus Edge Cases ============

    #[test]
    fn test_manager_set_focus_out_of_bounds() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        let result = manager.set_focus(10);
        assert!(!result);
        assert_eq!(manager.focused, Some(0));
    }

    #[test]
    fn test_manager_set_focus_valid_index() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));
        manager.add_window(FloatingWindow::new("Window 3", "Content"));

        let result = manager.set_focus(2);
        assert!(result);
        assert_eq!(manager.focused, Some(2));
    }

    #[test]
    fn test_manager_focused_window_mut() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content 1"));
        manager.add_window(FloatingWindow::new("Window 2", "Content 2"));

        manager.set_focus(1);
        if let Some(window) = manager.focused_window_mut() {
            window.set_title("Modified Title");
        }

        assert_eq!(manager.windows()[1].title(), "Modified Title");
    }

    #[test]
    fn test_manager_window_mut_out_of_bounds() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        let window = manager.window_mut(100);
        assert!(window.is_none());
    }

    #[test]
    fn test_manager_window_mut_valid_index() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));

        if let Some(window) = manager.window_mut(0) {
            window.set_content("New Content");
        }

        assert_eq!(manager.windows()[0].content(), "New Content");
    }

    // ============ Window rect() Method Tests ============

    #[test]
    fn test_window_rect_normal() {
        let window = FloatingWindow::new("Test", "Content")
            .position(10, 20)
            .size(50, 15);

        let rect = window.rect();
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 50);
        assert_eq!(rect.height, 15);
    }

    #[test]
    fn test_window_rect_minimized() {
        let mut window = FloatingWindow::new("Test", "Content")
            .position(10, 20)
            .size(50, 15);

        window.minimize();
        let rect = window.rect();
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 50);
        assert_eq!(rect.height, 3); // Minimized height is 3
    }

    #[test]
    fn test_window_rect_after_move() {
        let mut window = FloatingWindow::new("Test", "Content")
            .position(10, 10)
            .size(40, 10);

        window.move_by(5, 5);
        let rect = window.rect();
        assert_eq!(rect.x, 15);
        assert_eq!(rect.y, 15);
    }

    // ============ Show/Restore Interaction Tests ============

    #[test]
    fn test_show_restores_minimized() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.minimize();
        assert!(window.is_minimized());

        window.show();
        assert!(window.is_visible());
        assert!(!window.is_minimized());
    }

    #[test]
    fn test_restore_preserves_visibility() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.hide();
        window.minimize();

        window.restore();
        assert!(!window.is_visible());
        assert!(!window.is_minimized());
    }

    #[test]
    fn test_hide_preserves_minimized_state() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.minimize();
        assert!(window.is_minimized());

        window.hide();
        assert!(!window.is_visible());
        assert!(window.is_minimized());
    }

    // ============ Complex Multi-Phase Workflows ============

    #[test]
    fn test_window_10_phase_comprehensive_workflow() {
        let mut window = FloatingWindow::new("Test Window", "Initial content");

        // Phase 1: Setup
        window.set_position(0, 0);
        window.set_size(40, 10);
        assert_eq!(window.get_position(), WindowPosition::new(0, 0));

        // Phase 2: Move around terminal
        for _ in 0..10 {
            window.move_by(5, 2);
        }
        assert_eq!(window.get_position(), WindowPosition::new(50, 20));

        // Phase 3: Minimize and restore
        window.minimize();
        assert!(window.is_minimized());
        assert_eq!(window.rect().height, 3);

        window.restore();
        assert!(!window.is_minimized());
        assert_eq!(window.rect().height, 10);

        // Phase 4: Hide and show
        window.hide();
        assert!(!window.is_visible());

        window.show();
        assert!(window.is_visible());

        // Phase 5: Toggle operations
        window.toggle();
        assert!(!window.is_visible());

        window.toggle();
        assert!(window.is_visible());

        // Phase 6: Center in terminal
        window.center(80, 24);
        assert_eq!(window.get_position(), WindowPosition::new(20, 7));

        // Phase 7: Resize
        window.set_size(60, 20);
        assert_eq!(window.get_size(), (60, 20));

        // Phase 8: Update content and title
        window.set_title("Updated Title");
        window.set_content("Updated Content");
        assert_eq!(window.title(), "Updated Title");
        assert_eq!(window.content(), "Updated Content");

        // Phase 9: Test draggable
        let original_pos = window.get_position();
        window.move_by(10, 10);
        assert_ne!(window.get_position(), original_pos);

        // Phase 10: Clone and verify
        let cloned = window.clone();
        assert_eq!(cloned.title(), "Updated Title");
        assert_eq!(cloned.content(), "Updated Content");
        assert_eq!(cloned.get_position(), window.get_position());
    }

    #[test]
    fn test_manager_10_phase_comprehensive_workflow() {
        let mut manager = FloatingWindowManager::new();

        // Phase 1: Add initial windows
        for i in 0..5 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), format!("Content {}", i)));
        }
        assert_eq!(manager.window_count(), 5);

        // Phase 2: Navigate focus
        for _ in 0..10 {
            manager.focus_next();
        }
        assert_eq!(manager.focused, Some(0)); // Wrapped around

        // Phase 3: Modify focused windows
        for _ in 0..5 {
            if let Some(window) = manager.focused_window_mut() {
                window.toggle_minimize();
            }
            manager.focus_next();
        }

        // Phase 4: Add more windows
        for i in 5..10 {
            manager.add_window(FloatingWindow::new(format!("Window {}", i), format!("Content {}", i)));
        }
        assert_eq!(manager.window_count(), 10);

        // Phase 5: Close some windows
        for _ in 0..3 {
            manager.close_focused();
            manager.focus_next();
        }
        assert_eq!(manager.window_count(), 7);

        // Phase 6: Remove specific window
        manager.remove_window(0);
        assert_eq!(manager.window_count(), 6);

        // Phase 7: Focus navigation backwards
        for _ in 0..5 {
            manager.focus_previous();
        }

        // Phase 8: Modify via window_mut
        if let Some(window) = manager.window_mut(0) {
            window.set_title("Modified via window_mut");
        }

        // Phase 9: Set specific focus
        manager.set_focus(3);
        assert_eq!(manager.focused, Some(3));

        // Phase 10: Clean up all windows
        while manager.window_count() > 0 {
            manager.remove_window(0);
        }
        assert_eq!(manager.window_count(), 0);
        assert!(manager.focused_window().is_none());
    }

    // ============ Builder Pattern Edge Cases ============

    #[test]
    fn test_builder_all_methods() {
        let window = FloatingWindow::new("Test", "Content")
            .position(10, 20)
            .size(50, 15)
            .draggable(false)
            .closable(false);

        assert_eq!(window.get_position(), WindowPosition::new(10, 20));
        assert_eq!(window.get_size(), (50, 15));
        assert!(!window.is_draggable());
        assert!(!window.is_closable());
    }

    #[test]
    fn test_builder_chaining_order() {
        let window1 = FloatingWindow::new("Test", "Content")
            .position(10, 10)
            .size(40, 10);

        let window2 = FloatingWindow::new("Test", "Content")
            .size(40, 10)
            .position(10, 10);

        assert_eq!(window1.get_position(), window2.get_position());
        assert_eq!(window1.get_size(), window2.get_size());
    }

    #[test]
    fn test_builder_overwrite_values() {
        let window = FloatingWindow::new("Test", "Content")
            .position(10, 10)
            .position(20, 20)
            .size(40, 10)
            .size(60, 20);

        assert_eq!(window.get_position(), WindowPosition::new(20, 20));
        assert_eq!(window.get_size(), (60, 20));
    }

    // ============ Manager Remove Focus Adjustment Tests ============

    #[test]
    fn test_manager_remove_focused_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));
        manager.add_window(FloatingWindow::new("Window 3", "Content"));

        manager.set_focus(1);
        manager.remove_window(1);

        // Focus should remain at index 1 (which is now "Window 3")
        assert_eq!(manager.focused, Some(1));
        assert_eq!(manager.window_count(), 2);
    }

    #[test]
    fn test_manager_remove_last_focused_window() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));

        manager.set_focus(1);
        manager.remove_window(1);

        // Focus should move to last available window (index 0)
        assert_eq!(manager.focused, Some(0));
        assert_eq!(manager.window_count(), 1);
    }

    #[test]
    fn test_manager_remove_before_focused() {
        let mut manager = FloatingWindowManager::new();
        manager.add_window(FloatingWindow::new("Window 1", "Content"));
        manager.add_window(FloatingWindow::new("Window 2", "Content"));
        manager.add_window(FloatingWindow::new("Window 3", "Content"));

        manager.set_focus(2);
        manager.remove_window(0);

        // Focus should adjust to index 1 (since window before focused was removed)
        assert_eq!(manager.focused, Some(1));
        assert_eq!(manager.window_count(), 2);
    }

    // ============ Very Large Content Tests ============

    #[test]
    fn test_window_100k_content() {
        let large_content = "X".repeat(100000);
        let window = FloatingWindow::new("Test", large_content.clone());
        assert_eq!(window.content().len(), 100000);
    }

    #[test]
    fn test_window_multiline_content_1000_lines() {
        let multiline = "Line\n".repeat(1000);
        let window = FloatingWindow::new("Test", multiline.clone());
        assert_eq!(window.content().lines().count(), 1000);
    }

    // ============ Position Clamp Tests ============

    #[test]
    fn test_window_move_negative_clamps_to_zero() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(5, 5);
        window.move_by(-10, -10);

        let pos = window.get_position();
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_window_move_large_positive() {
        let mut window = FloatingWindow::new("Test", "Content");
        window.set_position(0, 0);
        window.move_by(1000, 1000);

        let pos = window.get_position();
        assert_eq!(pos.x, 1000);
        assert_eq!(pos.y, 1000);
    }
}
