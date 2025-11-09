/// Comprehensive end-to-end validation test for M0
///
/// This test validates the complete experimental workflow from
/// experiment creation through statistical comparison to decision.
#[cfg(test)]
mod m0_validation_tests {
    
    use tempfile::TempDir;
    use toad::config::{FeatureFlags, ToadConfig};
    use toad::evaluation::{task_loader, EvaluationHarness, ExperimentManager, ExperimentStatus};
    use toad::stats::ComparisonResult;

    /// Test the complete experimental workflow
    #[tokio::test]
    #[ignore] // Requires ANTHROPIC_API_KEY, run with `cargo test -- --ignored`
    async fn test_complete_experimental_workflow() {
        // 1. Create experiment manager
        let temp_dir = TempDir::new().unwrap();
        let mut exp_manager = ExperimentManager::new(temp_dir.path().join("experiments"));

        // 2. Define hypothesis
        let baseline = FeatureFlags::milestone_1();
        let treatment = FeatureFlags::milestone_2();

        let exp_id = exp_manager
            .create_experiment(
                "Test AST Context".to_string(),
                "AST-based context improves accuracy by 3-5 points".to_string(),
                baseline.clone(),
                treatment.clone(),
                3.0,
            )
            .unwrap();

        // Verify experiment created
        assert!(exp_manager.get(&exp_id).is_some());
        assert_eq!(
            exp_manager.get(&exp_id).unwrap().status,
            ExperimentStatus::Planned
        );

        // 3. Update status to running
        exp_manager
            .update_status(&exp_id, ExperimentStatus::Running)
            .unwrap();

        // 4. Generate test tasks
        let tasks = task_loader::create_test_tasks(20);
        assert_eq!(tasks.len(), 20);

        // 5. Run evaluation with baseline
        let config_a = ToadConfig {
            features: baseline,
            ..Default::default()
        };

        let harness = EvaluationHarness::new(tasks.clone(), temp_dir.path().join("results"));

        let results_a = harness.evaluate(&config_a).await.unwrap();
        assert_eq!(results_a.total_tasks, 20);
        assert!(results_a.accuracy >= 0.0 && results_a.accuracy <= 100.0);

        // 6. Run evaluation with treatment
        let config_b = ToadConfig {
            features: treatment,
            ..Default::default()
        };

        let results_b = harness.evaluate(&config_b).await.unwrap();
        assert_eq!(results_b.total_tasks, 20);

        // 7. Compare results
        let comparison = ComparisonResult::compare(&results_a, &results_b);

        // Verify comparison has all components
        assert!(comparison.significance.accuracy_p_value >= 0.0);
        assert!(comparison.significance.accuracy_p_value <= 1.0);
        assert!(comparison.delta.accuracy.abs() <= 100.0);

        // 8. Record results in experiment
        exp_manager
            .record_results(
                &exp_id,
                results_a,
                results_b,
                comparison,
                "End-to-end test completed successfully".to_string(),
            )
            .unwrap();

        // Verify experiment completed
        let exp = exp_manager.get(&exp_id).unwrap();
        assert_eq!(exp.status, ExperimentStatus::Completed);
        assert!(exp.results.is_some());

        // 9. Generate report
        let report = exp_manager.generate_report();
        assert!(report.contains("Experiment Report"));
        assert!(report.contains("Test AST Context"));
    }

    /// Test feature flag milestone progression
    #[test]
    fn test_feature_flag_progression() {
        let m1 = FeatureFlags::milestone_1();
        let m2 = FeatureFlags::milestone_2();
        let m3 = FeatureFlags::milestone_3();

        // M1: Minimal features
        assert_eq!(m1.enabled_count(), 2); // prompt_caching + tree_sitter
        assert!(!m1.context_ast);
        assert!(!m1.smart_test_selection);
        assert!(!m1.routing_multi_model);

        // M2: Add AST + smart tests
        assert_eq!(m2.enabled_count(), 4);
        assert!(m2.context_ast);
        assert!(m2.smart_test_selection);
        assert!(!m2.routing_multi_model);

        // M3: Add multi-model
        assert_eq!(m3.enabled_count(), 5);
        assert!(m3.context_ast);
        assert!(m3.smart_test_selection);
        assert!(m3.routing_multi_model);

        // Verify progression: M1 ⊂ M2 ⊂ M3
        assert!(m1.enabled_count() < m2.enabled_count());
        assert!(m2.enabled_count() < m3.enabled_count());
    }

    /// Test all 13 feature flags are accounted for
    #[test]
    fn test_all_13_features_present() {
        let flags = FeatureFlags::default();

        // Context (4)
        let _ = flags.context_ast;
        let _ = flags.context_embeddings;
        let _ = flags.context_graph;
        let _ = flags.context_reranking;

        // Routing (3)
        let _ = flags.routing_semantic;
        let _ = flags.routing_multi_model;
        let _ = flags.routing_speculative;

        // Intelligence (3)
        let _ = flags.smart_test_selection;
        let _ = flags.failure_memory;
        let _ = flags.opportunistic_planning;

        // Optimization (3)
        let _ = flags.prompt_caching;
        let _ = flags.semantic_caching;
        let _ = flags.tree_sitter_validation;

        // Verify total count
        let all_flags = FeatureFlags {
            context_ast: true,
            context_embeddings: true,
            context_graph: true,
            context_reranking: true,
            routing_semantic: true,
            routing_multi_model: true,
            routing_speculative: true,
            smart_test_selection: true,
            failure_memory: true,
            opportunistic_planning: true,
            prompt_caching: true,
            semantic_caching: true,
            tree_sitter_validation: true,
        };

        assert_eq!(all_flags.enabled_count(), 13);
    }

    /// Test dataset manager with different sources
    #[test]
    fn test_dataset_manager_sources() {
        use tempfile::TempDir;
        use toad::evaluation::{DatasetManager, DatasetSource};

        let temp_dir = TempDir::new().unwrap();
        let manager = DatasetManager::new(temp_dir.path().to_path_buf());
        manager.init().unwrap();

        // Test all source types
        let verified = DatasetSource::Verified;
        let lite = DatasetSource::Lite;
        let full = DatasetSource::Full;

        assert!(verified.huggingface_url().is_some());
        assert!(lite.huggingface_url().is_some());
        assert!(full.huggingface_url().is_some());

        assert!(verified.cache_filename().contains("verified"));
        assert!(lite.cache_filename().contains("lite"));
        assert!(full.cache_filename().contains("full"));

        // Test info retrieval
        let info = manager.dataset_info(&lite);
        assert!(!info.cached);
        assert_eq!(info.size, 0);
    }

    /// Test metrics collection completeness
    #[test]
    fn test_metrics_completeness() {
        use toad::metrics::{MetricsCollector, QualityMetrics};

        let mut collector = MetricsCollector::new();
        collector.start();

        // Record various events
        collector.record_api_call(1000, 500, 200, 0.05);
        collector.record_file_read();
        collector.record_file_write();
        collector.record_edit_attempt();
        collector.record_test_run();
        collector.record_agent_step();
        collector.record_first_response();

        collector.mark_solved(QualityMetrics {
            syntax_valid: 1.0,
            test_pass_rate: 1.0,
            code_coverage: 0.8,
            file_accuracy: 1.0,
        });

        let metrics = collector.finish();

        // Verify all metrics captured
        assert!(metrics.solved);
        assert_eq!(metrics.quality.syntax_valid, 1.0);
        assert_eq!(metrics.input_tokens, 1000);
        assert_eq!(metrics.output_tokens, 500);
        assert_eq!(metrics.cached_tokens, 200);
        assert_eq!(metrics.cost_usd, 0.05);
        assert_eq!(metrics.files_read, 1);
        assert_eq!(metrics.files_written, 1);
        assert_eq!(metrics.edit_attempts, 1);
        assert_eq!(metrics.test_runs, 1);
        assert_eq!(metrics.agent_steps, 1);
        // Duration fields are u64, so they're always set (can be 0 for fast tests)

        // Test calculations
        assert_eq!(metrics.total_tokens(), 1500);
        assert_eq!(metrics.effective_tokens(), 1300);
        assert!(metrics.cost_per_token() > 0.0);
        assert!(metrics.efficiency() > 0.0);
    }

    /// Test statistical decision criteria
    #[test]
    fn test_statistical_decision_criteria() {
        use toad::evaluation::{EvaluationResults, TaskResult};
        use toad::stats::{ComparisonResult, Recommendation};

        fn create_results(name: &str, solved: Vec<bool>, costs: Vec<f64>) -> EvaluationResults {
            let results: Vec<TaskResult> = solved
                .iter()
                .zip(costs.iter())
                .enumerate()
                .map(|(i, (s, c))| {
                    let mut result = TaskResult::new(format!("task-{}", i));
                    if *s {
                        result.mark_solved();
                    }
                    result.cost_usd = *c;
                    result.duration_ms = 1000;
                    result
                })
                .collect();

            EvaluationResults::from_results(name.to_string(), results)
        }

        // Test Case 1: Clear improvement (should adopt)
        let baseline = create_results(
            "baseline",
            vec![true; 10].into_iter().chain(vec![false; 10]).collect(),
            vec![0.01; 20],
        );
        let improved = create_results(
            "improved",
            vec![true; 15].into_iter().chain(vec![false; 5]).collect(),
            vec![0.012; 20], // 20% cost increase
        );

        let comparison = ComparisonResult::compare(&baseline, &improved);
        // 50% -> 75% = +25% accuracy, 20% cost increase
        assert!(comparison.delta.accuracy > 20.0);
        assert!(comparison.delta.cost_pct < 25.0);

        // Test Case 2: High cost, no benefit (should reject)
        let expensive = create_results(
            "expensive",
            vec![true; 10].into_iter().chain(vec![false; 10]).collect(),
            vec![0.05; 20], // 5x cost
        );

        let comparison = ComparisonResult::compare(&baseline, &expensive);
        // High cost with no benefit should not be adopted
        assert!(!matches!(comparison.recommendation, Recommendation::Adopt));
        assert!(comparison.delta.cost_pct > 100.0);
    }

    /// Test configuration serialization round-trip
    #[test]
    fn test_config_serialization_roundtrip() {
        use toad::config::ToadConfig;

        let original = ToadConfig::for_milestone(2);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ToadConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(original.model, deserialized.model);
        assert_eq!(original.max_context_tokens, deserialized.max_context_tokens);
        assert_eq!(
            original.features.context_ast,
            deserialized.features.context_ast
        );
        assert_eq!(
            original.features.enabled_count(),
            deserialized.features.enabled_count()
        );
    }

    /// Smoke test: Full pipeline with real file I/O
    #[tokio::test]
    #[ignore] // Requires ANTHROPIC_API_KEY, run with `cargo test -- --ignored`
    async fn test_smoke_full_pipeline() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();

        // 1. Generate test tasks
        let tasks = task_loader::create_test_tasks(5);
        assert_eq!(tasks.len(), 5);

        // 2. Run evaluation
        let config = ToadConfig::minimal();
        let harness = EvaluationHarness::new(tasks, temp_dir.path().join("results"));

        let results = harness.evaluate(&config).await.unwrap();
        assert_eq!(results.total_tasks, 5);

        // 3. Save results
        harness.save_results(&results).unwrap();

        // 4. Verify results file exists
        let result_files: Vec<_> = std::fs::read_dir(temp_dir.path().join("results"))
            .unwrap()
            .collect();

        assert!(!result_files.is_empty());

        // 5. Load results back
        for entry in result_files {
            let entry = entry.unwrap();
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let content = std::fs::read_to_string(entry.path()).unwrap();
                let _: toad::evaluation::EvaluationResults =
                    serde_json::from_str(&content).unwrap();
            }
        }
    }
}
