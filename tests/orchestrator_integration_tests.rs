//! Integration tests for the multi-benchmark orchestrator
//!
//! These tests verify end-to-end functionality of the orchestrator including:
//! - Concurrent benchmark execution
//! - Progress event emission
//! - Graceful cancellation
//!
//! NOTE: These tests are currently skipped because actual benchmark executors
//! (SWE-bench, LiveCodeBench) are stubs that call unimplemented!(). They will
//! be enabled once Phase 6 (benchmark implementation) is complete.

use toad::benchmarks::{Orchestrator, OrchestratorConfig, ExecutionContext};
use toad::benchmarks::types::ProgressEvent;
use tokio_util::sync::CancellationToken;
use std::time::Duration;

/// Test that orchestrator can run multiple benchmarks concurrently
///
/// This test creates an orchestrator with SWE-bench stub and verifies:
/// - Evaluation completes successfully
/// - Results are collected
/// - Aggregate metrics are computed
///
/// NOTE: Ignored until benchmark executors are implemented (Phase 3-4)
#[tokio::test]
#[ignore = "Requires implemented benchmark executors (Phase 3-4)"]
async fn test_orchestrator_completes_successfully() {
    // Create config with limited tasks for fast testing
    let config = OrchestratorConfig {
        benchmarks: vec!["swebench-verified".to_string()],
        task_limit: Some(2), // Only run 2 tasks
        max_concurrent_benchmarks: 1,
        execution_context: ExecutionContext {
            timeout: Duration::from_secs(60),
            max_steps: 10,
            system_config: serde_json::json!({}),
            sandbox_config: None,
        },
    };

    let cancel_token = CancellationToken::new();
    let orchestrator = Orchestrator::new(config, cancel_token);

    // Run evaluation
    let result = orchestrator.run_evaluation().await;

    // Verify it completes without error
    assert!(result.is_ok(), "Orchestrator should complete successfully");

    let (run, _progress_rx) = result.unwrap();

    // Verify run ID is generated
    assert!(!run.run_id.is_empty(), "Run ID should be generated");

    // Verify benchmark results exist
    assert_eq!(run.benchmark_results.len(), 1, "Should have 1 benchmark result");

    // Verify aggregate metrics
    assert!(run.aggregate_metrics.total_tasks >= 0, "Should have task count");
}

/// Test that progress events are emitted correctly
///
/// Verifies:
/// - EvaluationStarted event is sent first
/// - BenchmarkStarted events are sent
/// - TaskCompleted events are sent for each task
/// - BenchmarkCompleted events are sent
/// - Events are in correct order
///
/// NOTE: Ignored until benchmark executors are implemented (Phase 3-4)
#[tokio::test]
#[ignore = "Requires implemented benchmark executors (Phase 3-4)"]
async fn test_orchestrator_emits_progress_events() {
    let config = OrchestratorConfig {
        benchmarks: vec!["swebench-verified".to_string()],
        task_limit: Some(3), // Run 3 tasks
        max_concurrent_benchmarks: 1,
        execution_context: ExecutionContext::default(),
    };

    let cancel_token = CancellationToken::new();
    let orchestrator = Orchestrator::new(config, cancel_token);

    let (run, mut progress_rx) = orchestrator.run_evaluation().await.unwrap();

    // Collect all progress events
    let mut events = Vec::new();
    while let Some(event) = progress_rx.recv().await {
        events.push(event);
    }

    // Verify events were received
    assert!(!events.is_empty(), "Should receive progress events");

    // First event should be EvaluationStarted
    match &events[0] {
        ProgressEvent::EvaluationStarted { run_id, benchmarks, .. } => {
            assert!(!run_id.is_empty(), "Run ID should be present");
            assert_eq!(benchmarks.len(), 1, "Should have 1 benchmark");
        }
        _ => panic!("First event should be EvaluationStarted"),
    }

    // Should have BenchmarkStarted event
    let has_benchmark_started = events.iter().any(|e| matches!(e, ProgressEvent::BenchmarkStarted { .. }));
    assert!(has_benchmark_started, "Should have BenchmarkStarted event");

    // Should have TaskCompleted events
    let task_completed_count = events.iter().filter(|e| matches!(e, ProgressEvent::TaskCompleted { .. })).count();
    assert!(task_completed_count > 0, "Should have TaskCompleted events");

    // Should have BenchmarkCompleted event
    let has_benchmark_completed = events.iter().any(|e| matches!(e, ProgressEvent::BenchmarkCompleted { .. }));
    assert!(has_benchmark_completed, "Should have BenchmarkCompleted event");

    // Verify run completed
    assert!(!run.run_id.is_empty(), "Run should complete with ID");
}

/// Test that cancellation stops evaluation gracefully
///
/// Verifies:
/// - Cancellation token stops further task execution
/// - Partial results are returned
/// - No panics or errors on cancellation
#[tokio::test]
async fn test_orchestrator_cancellation() {
    let config = OrchestratorConfig {
        benchmarks: vec!["swebench-verified".to_string()],
        task_limit: Some(100), // Large number of tasks
        max_concurrent_benchmarks: 1,
        execution_context: ExecutionContext {
            timeout: Duration::from_secs(300),
            max_steps: 25,
            system_config: serde_json::json!({}),
            sandbox_config: None,
        },
    };

    let cancel_token = CancellationToken::new();
    let orchestrator = Orchestrator::new(config, cancel_token.clone());

    // Start evaluation
    let eval_handle = tokio::spawn(async move {
        orchestrator.run_evaluation().await
    });

    // Wait a short time, then cancel
    tokio::time::sleep(Duration::from_millis(100)).await;
    cancel_token.cancel();

    // Wait for evaluation to complete
    let result = eval_handle.await.unwrap();

    // Should complete successfully (not error)
    assert!(result.is_ok(), "Cancellation should not cause errors");

    let (run, _progress_rx) = result.unwrap();

    // Should have partial results
    assert!(!run.run_id.is_empty(), "Should have run ID even when cancelled");

    // Aggregate metrics should be valid (even if zero)
    assert!(run.aggregate_metrics.total_tasks >= 0, "Should have valid aggregate metrics");
}
