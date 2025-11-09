use super::*;
use ratatui::style::Color;


    #[test]
    fn test_model_info_creation() {
        let model = ModelInfo::new("test-model", "Test Model", "TestProvider")
            .with_context_window(100_000)
            .with_cost(1.5);

        assert_eq!(model.id, "test-model");
        assert_eq!(model.context_window, 100_000);
        assert_eq!(model.cost, 1.5);
        assert_eq!(model.formatted_context(), "100K");
    }

    #[test]
    fn test_model_selector() {
        let mut selector = ModelSelector::new();
        assert!(selector.selected_model().is_some());

        selector.next();
        assert_eq!(selector.selected, 1);

        selector.previous();
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_model_selection_by_id() {
        let mut selector = ModelSelector::new();
        assert!(selector.select_by_id("claude-opus-4"));
        assert_eq!(selector.selected_id(), Some("claude-opus-4"));
    }

    #[test]
    fn test_cost_indicator() {
        let model = ModelInfo::new("test", "Test", "Provider").with_cost(2.5);
        assert!(!model.cost_indicator().is_empty());
    }

    #[test]
    fn test_speed_indicator() {
        let model = ModelInfo::new("test", "Test", "Provider").with_speed(2.0);
        assert!(!model.speed_indicator().is_empty());
    }

    // ============================================================================
    // COMPREHENSIVE EDGE CASE TESTS (ADVANCED TIER - Advanced Input)
    // ============================================================================

    // ============ Stress Tests ============

    #[test]
    fn test_selector_1000_models() {
        let mut models = Vec::new();
        for i in 0..1000 {
            models.push(
                ModelInfo::new(
                    format!("model-{}", i),
                    format!("Model {}", i),
                    format!("Provider{}", i % 10),
                )
                .with_cost((i as f64) / 100.0),
            );
        }

        let selector = ModelSelector::new().with_models(models);
        assert_eq!(selector.models.len(), 1000);
    }

    #[test]
    fn test_selector_rapid_navigation_1000() {
        let mut selector = ModelSelector::new();
        for _ in 0..1000 {
            selector.next();
        }
        // Default has 6 models, 1000 % 6 = 4
        assert_eq!(selector.selected, 4);
    }

    #[test]
    fn test_selector_alternating_next_previous_1000() {
        let mut selector = ModelSelector::new();
        for _ in 0..1000 {
            selector.next();
            selector.previous();
        }
        // Should end up at starting position
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_model_many_capabilities() {
        let mut model = ModelInfo::new("test", "Test", "Provider");
        for i in 0..1000 {
            model = model.with_capability(format!("capability-{}", i));
        }
        assert_eq!(model.capabilities.len(), 1000);
    }

    #[test]
    fn test_selector_rapid_filter_changes() {
        let mut selector = ModelSelector::new();
        for i in 0..1000 {
            selector.set_filter(Some(format!("filter-{}", i)));
        }
        assert!(selector.filter.is_some());
    }

    // ============ Unicode Edge Cases ============

    #[test]
    fn test_model_unicode_name() {
        let model = ModelInfo::new("test", "日本語モデル 🚀", "Provider");
        assert_eq!(model.name, "日本語モデル 🚀");
    }

    #[test]
    fn test_model_rtl_name() {
        let model = ModelInfo::new("test", "مرحبا بك Model", "Provider");
        assert!(model.name.contains("مرحبا"));
    }

    #[test]
    fn test_model_mixed_scripts_name() {
        let model = ModelInfo::new(
            "test",
            "Hello世界Привет안녕하세요",
            "Provider",
        );
        assert!(model.name.contains("世界"));
    }

    #[test]
    fn test_model_emoji_provider() {
        let model = ModelInfo::new("test", "Test", "🐸 Anthropic 🚀");
        assert!(model.provider.contains('🐸'));
    }

    #[test]
    fn test_model_unicode_capability() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_capability("日本語処理")
            .with_capability("🔧 coding")
            .with_capability("مرحبا");

        assert!(model.capabilities.contains(&"日本語処理".to_string()));
        assert!(model.capabilities.contains(&"🔧 coding".to_string()));
    }

    #[test]
    fn test_model_very_long_unicode_name() {
        let long_name = "日本語 ".repeat(1000);
        let model = ModelInfo::new("test", long_name.clone(), "Provider");
        assert_eq!(model.name, long_name);
    }

    // ============ Extreme Values ============

    #[test]
    fn test_model_context_window_max() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(usize::MAX);
        assert_eq!(model.context_window, usize::MAX);
    }

    #[test]
    fn test_model_context_window_zero() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(0);
        assert_eq!(model.context_window, 0);
    }

    #[test]
    fn test_model_max_output_extreme() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_max_output(usize::MAX);
        assert_eq!(model.max_output, usize::MAX);
    }

    #[test]
    fn test_model_cost_zero() {
        let model = ModelInfo::new("test", "Test", "Provider").with_cost(0.0);
        assert_eq!(model.cost, 0.0);
        assert_eq!(model.cost_indicator(), "$"); // Clamps to at least 1
    }

    #[test]
    fn test_model_cost_very_high() {
        let model = ModelInfo::new("test", "Test", "Provider").with_cost(100.0);
        assert_eq!(model.cost_indicator(), "$$$$$"); // Clamps to max 5
    }

    #[test]
    fn test_model_speed_zero() {
        let model = ModelInfo::new("test", "Test", "Provider").with_speed(0.0);
        assert_eq!(model.speed, 0.0);
        assert_eq!(model.speed_indicator(), "⚡"); // Clamps to at least 1
    }

    #[test]
    fn test_model_speed_very_high() {
        let model = ModelInfo::new("test", "Test", "Provider").with_speed(100.0);
        assert_eq!(model.speed_indicator(), "⚡⚡⚡"); // Clamps to max 3
    }

    #[test]
    fn test_formatted_context_millions() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(5_000_000);
        assert_eq!(model.formatted_context(), "5M");
    }

    #[test]
    fn test_formatted_context_thousands() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(128_000);
        assert_eq!(model.formatted_context(), "128K");
    }

    #[test]
    fn test_formatted_context_hundreds() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(500);
        assert_eq!(model.formatted_context(), "500");
    }

    // ============ Navigation Edge Cases ============

    #[test]
    fn test_selector_navigation_wrap_forward() {
        let mut selector = ModelSelector::new();
        let model_count = selector.models.len();

        // Navigate to last model
        for _ in 0..model_count - 1 {
            selector.next();
        }
        assert_eq!(selector.selected, model_count - 1);

        // Next should wrap to 0
        selector.next();
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_selector_navigation_wrap_backward() {
        let mut selector = ModelSelector::new();
        let model_count = selector.models.len();

        // At start (0), previous should wrap to last
        selector.previous();
        assert_eq!(selector.selected, model_count - 1);
    }

    #[test]
    fn test_selector_select_out_of_bounds() {
        let mut selector = ModelSelector::new();
        let original = selector.selected;

        selector.select(9999);
        // Should remain unchanged
        assert_eq!(selector.selected, original);
    }

    #[test]
    fn test_selector_select_by_invalid_id() {
        let mut selector = ModelSelector::new();
        let result = selector.select_by_id("nonexistent-model");
        assert!(!result);
    }

    #[test]
    fn test_selector_empty_models() {
        let selector = ModelSelector::new().with_models(vec![]);
        assert!(selector.selected_model().is_none());
        assert!(selector.selected_id().is_none());
    }

    #[test]
    fn test_selector_navigation_empty_models() {
        let mut selector = ModelSelector::new().with_models(vec![]);

        selector.next();
        selector.previous();

        // Should handle gracefully without panicking
        assert!(selector.selected_model().is_none());
    }

    // ============ Filtering Edge Cases ============

    #[test]
    fn test_filter_by_coding_capability() {
        let selector = ModelSelector::new();
        let coding_models = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"coding".to_string()))
            .count();

        // All default models have coding capability
        assert!(coding_models > 0);
    }

    #[test]
    fn test_filter_nonexistent_capability() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("nonexistent-capability".to_string()));

        let filtered = selector
            .models
            .iter()
            .filter(|m| {
                if let Some(ref f) = selector.filter {
                    m.capabilities.contains(f)
                } else {
                    true
                }
            })
            .count();

        assert_eq!(filtered, 0);
    }

    #[test]
    fn test_filter_clear() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("coding".to_string()));
        assert!(selector.filter.is_some());

        selector.set_filter(None);
        assert!(selector.filter.is_none());
    }

    // ============ Builder Pattern Edge Cases ============

    #[test]
    fn test_model_info_chained_builders() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_context_window(100_000)
            .with_max_output(4096)
            .with_cost(1.5)
            .with_speed(2.0)
            .with_capability("coding")
            .with_capability("reasoning")
            .with_available(false);

        assert_eq!(model.context_window, 100_000);
        assert_eq!(model.max_output, 4096);
        assert_eq!(model.cost, 1.5);
        assert_eq!(model.speed, 2.0);
        assert_eq!(model.capabilities.len(), 2);
        assert!(!model.available);
    }

    #[test]
    fn test_model_info_override_values() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_cost(1.0)
            .with_cost(2.0)
            .with_cost(3.0);

        assert_eq!(model.cost, 3.0); // Last value wins
    }

    // ============ Clone/Debug/Serialize Traits ============

    #[test]
    fn test_model_info_clone() {
        let model = ModelInfo::new("test", "Test Model", "Provider")
            .with_cost(1.5)
            .with_capability("coding");

        let cloned = model.clone();
        assert_eq!(model.id, cloned.id);
        assert_eq!(model.name, cloned.name);
        assert_eq!(model.cost, cloned.cost);
        assert_eq!(model.capabilities, cloned.capabilities);
    }

    #[test]
    fn test_model_info_debug() {
        let model = ModelInfo::new("test", "Test", "Provider");
        let debug_str = format!("{:?}", model);
        assert!(debug_str.contains("ModelInfo"));
    }

    #[test]
    fn test_model_info_serialize_deserialize() {
        let model = ModelInfo::new("test-id", "Test Model", "TestProvider")
            .with_context_window(100_000)
            .with_cost(1.5)
            .with_capability("coding");

        let json = serde_json::to_string(&model).unwrap();
        let deserialized: ModelInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(model.id, deserialized.id);
        assert_eq!(model.name, deserialized.name);
        assert_eq!(model.cost, deserialized.cost);
        assert_eq!(model.capabilities, deserialized.capabilities);
    }

    // ============ Complex Workflow Tests ============

    #[test]
    fn test_selector_add_remove_navigate() {
        let mut selector = ModelSelector::new();
        let original_count = selector.models.len();

        // Add a new model
        selector.add_model(
            ModelInfo::new("new-model", "New Model", "Provider")
                .with_capability("test"),
        );
        assert_eq!(selector.models.len(), original_count + 1);

        // Navigate to the new model
        selector.select(original_count);
        assert_eq!(selector.selected_id(), Some("new-model"));
    }

    #[test]
    fn test_selector_toggle_details() {
        let mut selector = ModelSelector::new();
        let initial_state = selector.show_details;

        selector.toggle_details();
        assert_eq!(selector.show_details, !initial_state);

        selector.toggle_details();
        assert_eq!(selector.show_details, initial_state);
    }

    #[test]
    fn test_selector_workflow_navigation_selection() {
        let mut selector = ModelSelector::new();

        // Navigate and select by ID
        selector.next();
        selector.next();
        let id_at_2 = selector.selected_id().unwrap().to_string();

        selector.select(0);
        assert_eq!(selector.selected, 0);

        selector.select_by_id(&id_at_2);
        assert_eq!(selector.selected, 2);
    }

    #[test]
    fn test_selector_with_unavailable_models() {
        let models = vec![
            ModelInfo::new("m1", "Model 1", "P1").with_available(true),
            ModelInfo::new("m2", "Model 2", "P2").with_available(false),
            ModelInfo::new("m3", "Model 3", "P3").with_available(true),
        ];

        let mut selector = ModelSelector::new().with_models(models);

        // Can still select unavailable models
        selector.select(1);
        assert!(selector.selected_model().is_some());
        assert!(!selector.selected_model().unwrap().available);
    }

    // ============ Comprehensive Stress Test ============

    #[test]
    fn test_comprehensive_model_selector_stress() {
        let mut selector = ModelSelector::new();

        // Phase 1: Add many models with varied configurations
        for i in 0..100 {
            let name = match i % 4 {
                0 => format!("ASCII Model {}", i),
                1 => format!("🚀 Emoji Model {}", i),
                2 => format!("日本語 Model {}", i),
                _ => format!("مرحبا Model {}", i),
            };

            let mut model = ModelInfo::new(
                format!("model-{}", i),
                name,
                format!("Provider{}", i % 5),
            )
            .with_context_window(50_000 + (i * 1000))
            .with_max_output(2048 + (i * 10))
            .with_cost((i as f64) / 20.0)
            .with_speed((i as f64) / 30.0);

            // Add capabilities
            for cap_idx in 0..(i % 5) {
                model = model.with_capability(format!("cap-{}", cap_idx));
            }

            // Some unavailable
            if i % 10 == 0 {
                model = model.with_available(false);
            }

            selector.add_model(model);
        }

        let total_models = selector.models.len(); // Default + 100
        assert!(total_models >= 100);

        // Phase 2: Navigation stress
        for _ in 0..200 {
            selector.next();
        }
        assert!(selector.selected_model().is_some());

        for _ in 0..100 {
            selector.previous();
        }
        assert!(selector.selected_model().is_some());

        // Phase 3: Direct selection
        selector.select(0);
        assert_eq!(selector.selected, 0);

        selector.select(total_models - 1);
        assert_eq!(selector.selected, total_models - 1);

        // Phase 4: Select by ID
        assert!(selector.select_by_id("model-50"));
        assert_eq!(selector.selected_id(), Some("model-50"));

        // Phase 5: Toggle details
        selector.toggle_details();
        selector.toggle_details();

        // Phase 6: Filtering
        selector.set_filter(Some("cap-1".to_string()));
        let filtered_count = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"cap-1".to_string()))
            .count();
        assert!(filtered_count > 0);

        // Phase 7: Clear filter
        selector.set_filter(None);
        assert!(selector.filter.is_none());

        // Final verification
        assert!(selector.selected < total_models);
        assert!(selector.selected_model().is_some());
    }

    // ============ Default Trait Test ============

    #[test]
    fn test_selector_default() {
        let selector = ModelSelector::default();
        assert!(!selector.models.is_empty());
        assert_eq!(selector.selected, 0);
        assert!(selector.show_details);
    }

    // ============ Empty Content Edge Cases ============

    #[test]
    fn test_model_empty_id() {
        let model = ModelInfo::new("", "Name", "Provider");
        assert_eq!(model.id, "");
    }

    #[test]
    fn test_model_empty_name() {
        let model = ModelInfo::new("id", "", "Provider");
        assert_eq!(model.name, "");
    }

    #[test]
    fn test_model_empty_provider() {
        let model = ModelInfo::new("id", "Name", "");
        assert_eq!(model.provider, "");
    }

    #[test]
    fn test_model_no_capabilities() {
        let model = ModelInfo::new("id", "Name", "Provider");
        assert!(model.capabilities.is_empty());
    }

    // ============ Extreme Stress Tests (10k operations) ============

    #[test]
    fn test_selector_10k_next_navigation() {
        let mut selector = ModelSelector::new();
        for _ in 0..10000 {
            selector.next();
        }
        // Should still be functional
        assert!(selector.selected_model().is_some());
    }

    #[test]
    fn test_selector_10k_previous_navigation() {
        let mut selector = ModelSelector::new();
        for _ in 0..10000 {
            selector.previous();
        }
        assert!(selector.selected_model().is_some());
    }

    #[test]
    fn test_selector_10k_mixed_operations() {
        let mut selector = ModelSelector::new();
        for i in 0..10000 {
            match i % 4 {
                0 => selector.next(),
                1 => selector.previous(),
                2 => selector.toggle_details(),
                _ => {
                    selector.select(i % selector.models.len());
                }
            }
        }
        assert!(selector.selected_model().is_some());
    }

    #[test]
    fn test_model_10k_capability_additions() {
        let mut model = ModelInfo::new("test", "Test", "Provider");
        for i in 0..10000 {
            model = model.with_capability(format!("cap{}", i));
        }
        assert_eq!(model.capabilities.len(), 10000);
    }

    // ============ Selection State Preservation ============

    #[test]
    fn test_selector_state_after_model_replacement() {
        let mut selector = ModelSelector::new();
        selector.select(2);
        assert_eq!(selector.selected, 2);

        // Replace with fewer models
        let new_models = vec![
            ModelInfo::new("m1", "Model 1", "P1"),
        ];
        selector = selector.with_models(new_models);

        // Should adjust selection to valid index
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_selector_state_after_model_expansion() {
        let mut selector = ModelSelector::new();
        selector.select(1);

        // Add more models
        for i in 0..50 {
            selector.add_model(ModelInfo::new(format!("m{}", i), format!("Model {}", i), "P"));
        }

        // Selection should remain at 1
        assert_eq!(selector.selected, 1);
    }

    // ============ Capability Filtering Edge Cases ============

    #[test]
    fn test_filter_with_empty_string() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("".to_string()));
        assert_eq!(selector.filter, Some("".to_string()));
    }

    #[test]
    fn test_filter_with_unicode_capability() {
        let mut selector = ModelSelector::new();
        selector.add_model(
            ModelInfo::new("test", "Test", "Provider")
                .with_capability("日本語処理")
        );

        selector.set_filter(Some("日本語処理".to_string()));

        let filtered_count = selector
            .models
            .iter()
            .filter(|m| {
                if let Some(ref f) = selector.filter {
                    m.capabilities.contains(f)
                } else {
                    true
                }
            })
            .count();

        assert_eq!(filtered_count, 1);
    }

    #[test]
    fn test_filter_case_sensitivity() {
        let selector = ModelSelector::new();
        // Filter is case-sensitive by default
        let filtered = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"CODING".to_string()))
            .count();

        // Should be 0 because capabilities use lowercase "coding"
        assert_eq!(filtered, 0);
    }

    // ============ Indicator Edge Cases ============

    #[test]
    fn test_cost_indicator_boundary_values() {
        // Test exact boundary values for cost indicator
        let model1 = ModelInfo::new("t", "T", "P").with_cost(0.25); // Should be $
        let model2 = ModelInfo::new("t", "T", "P").with_cost(0.5);  // Should be $$
        let model3 = ModelInfo::new("t", "T", "P").with_cost(1.0);  // Should be $$$$
        let model4 = ModelInfo::new("t", "T", "P").with_cost(1.25); // Should be $$$$$

        assert_eq!(model1.cost_indicator(), "$");
        assert_eq!(model2.cost_indicator(), "$$");
        assert_eq!(model3.cost_indicator(), "$$$$");
        assert_eq!(model4.cost_indicator(), "$$$$$");
    }

    #[test]
    fn test_speed_indicator_boundary_values() {
        let model1 = ModelInfo::new("t", "T", "P").with_speed(0.33); // ceil(0.33*3) = ceil(0.99) = 1 → ⚡
        let model2 = ModelInfo::new("t", "T", "P").with_speed(0.66); // ceil(0.66*3) = ceil(1.98) = 2 → ⚡⚡
        let model3 = ModelInfo::new("t", "T", "P").with_speed(1.0);  // ceil(1.0*3) = ceil(3.0) = 3 → ⚡⚡⚡

        assert_eq!(model1.speed_indicator(), "⚡");
        assert_eq!(model2.speed_indicator(), "⚡⚡");
        assert_eq!(model3.speed_indicator(), "⚡⚡⚡");
    }

    // ============ Multi-Phase Comprehensive Workflow (10 phases) ============

    #[test]
    fn test_selector_10_phase_comprehensive_workflow() {
        let mut selector = ModelSelector::new();

        // Phase 1: Initial state verification
        assert!(selector.selected_model().is_some());
        assert_eq!(selector.selected, 0);
        assert!(selector.show_details);

        // Phase 2: Add custom models with unicode
        for i in 0..10 {
            selector.add_model(
                ModelInfo::new(format!("custom-{}", i), format!("日本語 Model {}", i), "🚀 Provider")
                    .with_capability("coding")
                    .with_capability(format!("cap-{}", i))
            );
        }
        let total_models = selector.models.len();
        assert!(total_models >= 10);

        // Phase 3: Navigation stress
        for _ in 0..100 {
            selector.next();
        }
        assert!(selector.selected < total_models);

        // Phase 4: Navigate to specific index
        selector.select(total_models / 2);
        assert_eq!(selector.selected, total_models / 2);

        // Phase 5: Toggle details multiple times
        selector.toggle_details();
        assert!(!selector.show_details);
        selector.toggle_details();
        assert!(selector.show_details);

        // Phase 6: Select by ID
        assert!(selector.select_by_id("custom-5"));
        assert!(selector.selected_id().unwrap().contains("custom-5"));

        // Phase 7: Apply filter
        selector.set_filter(Some("cap-3".to_string()));
        let filtered = selector
            .models
            .iter()
            .filter(|m| m.capabilities.contains(&"cap-3".to_string()))
            .count();
        assert_eq!(filtered, 1);

        // Phase 8: Clear filter
        selector.set_filter(None);
        assert!(selector.filter.is_none());

        // Phase 9: Backward navigation
        for _ in 0..50 {
            selector.previous();
        }
        assert!(selector.selected < total_models);

        // Phase 10: Final state verification
        assert!(selector.selected_model().is_some());
        selector.select(0);
        assert_eq!(selector.selected, 0);
    }

    // ============ Context Window Formatting Edge Cases ============

    #[test]
    fn test_formatted_context_exactly_1m() {
        let model = ModelInfo::new("t", "T", "P").with_context_window(1_000_000);
        assert_eq!(model.formatted_context(), "1M");
    }

    #[test]
    fn test_formatted_context_exactly_1k() {
        let model = ModelInfo::new("t", "T", "P").with_context_window(1_000);
        assert_eq!(model.formatted_context(), "1K");
    }

    #[test]
    fn test_formatted_context_999() {
        let model = ModelInfo::new("t", "T", "P").with_context_window(999);
        assert_eq!(model.formatted_context(), "999");
    }

    // ============ Clone Independence ============

    #[test]
    fn test_model_info_clone_independence() {
        let mut model1 = ModelInfo::new("test", "Test", "Provider")
            .with_capability("coding");

        let mut model2 = model1.clone();

        // Modify model2
        model2 = model2.with_capability("reasoning");

        // model1 should be unchanged (capabilities should be different)
        assert_eq!(model1.capabilities.len(), 1);
        assert_eq!(model2.capabilities.len(), 2);
    }

    // ============ ADDITIONAL FUNCTIONAL TESTS FROM INTEGRATION ============

    #[test]
    fn test_model_info_with_max_output_builder() {
        let model = ModelInfo::new("test", "Test", "Provider").with_max_output(8192);
        assert_eq!(model.max_output, 8192);
    }

    #[test]
    fn test_model_info_with_capability_builder() {
        let model = ModelInfo::new("test", "Test", "Provider")
            .with_capability("coding")
            .with_capability("reasoning");
        assert_eq!(model.capabilities.len(), 2);
        assert!(model.capabilities.contains(&"coding".to_string()));
        assert!(model.capabilities.contains(&"reasoning".to_string()));
    }

    #[test]
    fn test_model_info_with_available_builder() {
        let available = ModelInfo::new("test", "Test", "Provider").with_available(true);
        assert!(available.available);
        let unavailable = ModelInfo::new("test", "Test", "Provider").with_available(false);
        assert!(!unavailable.available);
    }

    #[test]
    fn test_formatted_context_small_values() {
        let model = ModelInfo::new("test", "Test", "Provider").with_context_window(512);
        assert_eq!(model.formatted_context(), "512");
    }

    #[test]
    fn test_cost_indicator_values_range() {
        let cheap = ModelInfo::new("test", "Test", "Provider").with_cost(0.5);
        assert_eq!(cheap.cost_indicator(), "$$");
        let expensive = ModelInfo::new("test", "Test", "Provider").with_cost(3.0);
        assert_eq!(expensive.cost_indicator(), "$$$$$");
    }

    #[test]
    fn test_speed_indicator_values_range() {
        let slow = ModelInfo::new("test", "Test", "Provider").with_speed(0.5);
        assert_eq!(slow.speed_indicator(), "⚡⚡");
        let fast = ModelInfo::new("test", "Test", "Provider").with_speed(2.5);
        assert_eq!(fast.speed_indicator(), "⚡⚡⚡");
    }

    #[test]
    fn test_model_info_chaining_complete() {
        let model = ModelInfo::new("gpt-4", "GPT-4", "OpenAI")
            .with_context_window(128_000)
            .with_max_output(4096)
            .with_cost(2.0)
            .with_speed(1.5)
            .with_capability("coding")
            .with_capability("analysis")
            .with_available(true);
        assert_eq!(model.id, "gpt-4");
        assert_eq!(model.context_window, 128_000);
        assert_eq!(model.max_output, 4096);
        assert_eq!(model.cost, 2.0);
        assert_eq!(model.speed, 1.5);
        assert_eq!(model.capabilities.len(), 2);
        assert!(model.available);
    }

    #[test]
    fn test_selector_with_models_builder() {
        let models = vec![
            ModelInfo::new("model1", "Model 1", "Provider"),
            ModelInfo::new("model2", "Model 2", "Provider"),
        ];
        let selector = ModelSelector::new().with_models(models);
        assert_eq!(selector.models.len(), 2);
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_selector_add_model_method() {
        let mut selector = ModelSelector::new();
        let initial_count = selector.models.len();
        selector.add_model(ModelInfo::new("new-model", "New Model", "Provider"));
        assert_eq!(selector.models.len(), initial_count + 1);
    }

    #[test]
    fn test_selector_select_by_index_method() {
        let mut selector = ModelSelector::new();
        selector.select(2);
        assert_eq!(selector.selected, 2);
        assert_eq!(selector.list_state.selected(), Some(2));
    }

    #[test]
    fn test_selector_select_invalid_index_ignored() {
        let mut selector = ModelSelector::new();
        let initial_selected = selector.selected;
        selector.select(999);
        assert_eq!(selector.selected, initial_selected);
    }

    #[test]
    fn test_selector_set_filter_method() {
        let mut selector = ModelSelector::new();
        selector.set_filter(Some("coding".to_string()));
        assert_eq!(selector.filter, Some("coding".to_string()));
        selector.set_filter(None);
        assert_eq!(selector.filter, None);
    }

    #[test]
    fn test_selector_next_wraps_around_boundary() {
        let mut selector = ModelSelector::new();
        let count = selector.models.len();
        for _ in 0..count - 1 {
            selector.next();
        }
        assert_eq!(selector.selected, count - 1);
        selector.next();
        assert_eq!(selector.selected, 0);
    }

    #[test]
    fn test_selector_previous_wraps_around_boundary() {
        let mut selector = ModelSelector::new();
        assert_eq!(selector.selected, 0);
        selector.previous();
        assert_eq!(selector.selected, selector.models.len() - 1);
    }

    #[test]
    fn test_selector_select_by_id_invalid_returns_false() {
        let mut selector = ModelSelector::new();
        let result = selector.select_by_id("nonexistent-model");
        assert!(!result);
    }

    #[test]
    fn test_selector_with_models_adjusts_selection_boundary() {
        let mut selector = ModelSelector::new();
        selector.select(5);
        let models = vec![ModelInfo::new("model1", "Model 1", "Provider")];
        selector = selector.with_models(models);
        assert_eq!(selector.selected, 0);
    }
