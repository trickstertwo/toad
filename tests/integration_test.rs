use std::path::PathBuf;
/// Integration tests for TOAD evaluation framework
use toad::config::{FeatureFlags, ToadConfig};
use toad::{Complexity, EvaluationHarness, task_loader};
use toad::Metrics;
use toad::ComparisonResult;

#[tokio::test]
async fn test_basic_evaluation() {
    // Create test tasks
    let tasks = task_loader::create_test_tasks(5);
    assert_eq!(tasks.len(), 5);

    // Create a minimal config
    let config = ToadConfig::minimal();

    // Run evaluation
    let harness = EvaluationHarness::new(tasks, PathBuf::from("/tmp/toad-test"));
    let results = harness.evaluate(&config).await.unwrap();

    // Check results
    assert_eq!(results.total_tasks, 5);
    assert!(results.accuracy >= 0.0 && results.accuracy <= 100.0);
}

#[tokio::test]
async fn test_ab_comparison() {
    // Create test tasks
    let tasks = task_loader::create_test_tasks(10);

    // Create two configurations
    let config_a = ToadConfig::for_milestone(1);
    let config_b = ToadConfig::for_milestone(2);

    // Run comparison
    let harness = EvaluationHarness::new(tasks, PathBuf::from("/tmp/toad-test"));
    let (results_a, results_b) = harness.compare(&config_a, &config_b).await.unwrap();

    // Analyze
    let comparison = ComparisonResult::compare(&results_a, &results_b);

    // Check that comparison was computed
    assert!(comparison.delta.accuracy.abs() <= 100.0);
}

#[test]
fn test_milestone_progression() {
    let m1 = ToadConfig::for_milestone(1);
    let m2 = ToadConfig::for_milestone(2);
    let m3 = ToadConfig::for_milestone(3);

    // M2 should have more features than M1
    assert!(m2.features.enabled_count() > m1.features.enabled_count());

    // M3 should have more features than M2
    assert!(m3.features.enabled_count() > m2.features.enabled_count());

    // M3 should include multi-model
    assert!(m3.features.routing_multi_model);
}

#[test]
fn test_feature_flag_defaults() {
    let flags = FeatureFlags::default();

    // Proven features should be enabled by default
    assert!(flags.context_ast);
    assert!(flags.prompt_caching);
    assert!(flags.tree_sitter_validation);

    // Experimental features should be disabled by default
    assert!(!flags.routing_multi_model);
    assert!(!flags.failure_memory);
    assert!(!flags.opportunistic_planning);
}

#[test]
fn test_task_complexity_estimation() {
    let tasks = task_loader::create_test_tasks(9);

    // Check complexity distribution
    let simple_count = tasks
        .iter()
        .filter(|t| t.complexity == Complexity::Simple)
        .count();
    let medium_count = tasks
        .iter()
        .filter(|t| t.complexity == Complexity::Medium)
        .count();
    let hard_count = tasks
        .iter()
        .filter(|t| t.complexity == Complexity::Hard)
        .count();

    assert_eq!(simple_count, 3); // 0, 3, 6
    assert_eq!(medium_count, 3); // 1, 4, 7
    assert_eq!(hard_count, 3); // 2, 5, 8
}

#[test]
fn test_metrics_aggregation() {
    use toad::AggregateMetrics;

    let metrics = vec![
        Metrics {
            solved: true,
            cost_usd: 0.01,
            duration_ms: 1000,
            ..Default::default()
        },
        Metrics {
            solved: false,
            cost_usd: 0.02,
            duration_ms: 2000,
            ..Default::default()
        },
        Metrics {
            solved: true,
            cost_usd: 0.015,
            duration_ms: 1500,
            ..Default::default()
        },
    ];

    let agg = AggregateMetrics::from_metrics(&metrics);

    assert_eq!(agg.count, 3);
    assert!((agg.accuracy - 66.67).abs() < 0.1);
    assert!((agg.mean_cost_usd - 0.015).abs() < 0.001);
}

#[tokio::test]
async fn test_save_and_load_results() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let tasks = task_loader::create_test_tasks(3);

    let config = ToadConfig::minimal();
    let harness = EvaluationHarness::new(tasks, temp_dir.path().to_path_buf());

    let results = harness.evaluate(&config).await.unwrap();
    harness.save_results(&results).unwrap();

    // Check that file was created
    let entries: Vec<_> = std::fs::read_dir(temp_dir.path()).unwrap().collect();

    assert!(entries.len() > 0);
}

#[test]
fn test_config_serialization() {
    let config = ToadConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ToadConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.provider.model, deserialized.provider.model);
    assert_eq!(config.max_context_tokens, deserialized.max_context_tokens);
}
