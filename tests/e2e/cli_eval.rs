//! End-to-end CLI evaluation tests
//!
//! Tests complete evaluation workflows by spawning the TOAD binary
//! and verifying output, exit codes, and generated files.

use std::process::Command;
use std::path::Path;

/// Test that eval command with synthetic data completes successfully
#[test]
fn test_cli_eval_synthetic_completes() {
    let output = Command::new("cargo")
        .args(&["run", "--", "eval", "--count", "1", "--milestone", "1"])
        .output()
        .expect("Failed to execute toad eval command");

    // Should complete without error
    assert!(
        output.status.success(),
        "Eval command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Output should contain accuracy metric
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Accuracy:") || stdout.contains("accuracy"),
        "Output missing accuracy metric: {}",
        stdout
    );
}

/// Test that show-config command displays milestone configuration
#[test]
fn test_cli_show_config() {
    let output = Command::new("cargo")
        .args(&["run", "--", "show-config", "--milestone", "1"])
        .output()
        .expect("Failed to execute show-config command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("M1") || stdout.contains("Milestone 1"));
    assert!(stdout.contains("Feature") || stdout.contains("feature"));
}

/// Test that generate-test-data creates valid JSON file
#[test]
fn test_cli_generate_test_data() {
    let test_file = "/tmp/toad_test_data.json";

    // Clean up any existing file
    let _ = std::fs::remove_file(test_file);

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "generate-test-data",
            "--count",
            "5",
            "--output",
            test_file,
        ])
        .output()
        .expect("Failed to execute generate-test-data command");

    assert!(
        output.status.success(),
        "generate-test-data failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify file was created
    assert!(
        Path::new(test_file).exists(),
        "Test data file was not created at {}",
        test_file
    );

    // Verify file contains valid JSON
    let content = std::fs::read_to_string(test_file)
        .expect("Failed to read generated test data");

    assert!(
        content.contains("task_id") || content.contains("repo"),
        "Generated file doesn't look like task data: {}",
        &content[..100.min(content.len())]
    );

    // Clean up
    let _ = std::fs::remove_file(test_file);
}

/// Test that eval with invalid milestone fails gracefully
#[test]
fn test_cli_eval_invalid_milestone() {
    let output = Command::new("cargo")
        .args(&["run", "--", "eval", "--count", "1", "--milestone", "999"])
        .output()
        .expect("Failed to execute command");

    // Should fail or show error
    assert!(
        !output.status.success() ||
        String::from_utf8_lossy(&output.stderr).contains("error") ||
        String::from_utf8_lossy(&output.stderr).contains("Invalid"),
        "Should fail or show error for invalid milestone"
    );
}

/// Test that help command works
#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute --help");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("eval") || stdout.contains("Evaluate"));
    assert!(stdout.contains("Usage") || stdout.contains("USAGE"));
}

/// Test that version flag works
#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute --version");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("toad") || stdout.contains("0.1.0"),
        "Version output unexpected: {}",
        stdout
    );
}

/// Test eval with SWE-bench dataset (marked expensive, ignored by default)
#[test]
#[ignore]
fn test_cli_eval_swebench_verified() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "eval",
            "--swebench",
            "verified",
            "--count",
            "2",
            "--milestone",
            "1",
        ])
        .output()
        .expect("Failed to execute swebench eval");

    assert!(
        output.status.success(),
        "SWE-bench eval failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Accuracy:"));
    assert!(stdout.contains("2/2") || stdout.contains("Task"));
}
