//! Storage and serialization for evaluation results
//!
//! This module provides versioned JSON storage for `EvaluationRun` data with support
//! for incremental saves during long-running evaluations. Results are stored in a
//! structured directory format with atomic writes and crash recovery.
//!
//! # Architecture
//!
//! - **Completed runs**: `./results/{run_id}.json`
//! - **In-progress runs**: `./results/.tmp/{run_id}/` (incremental snapshots)
//! - **Format**: Versioned JSON with explicit `format_version` field
//! - **Atomic writes**: Write to .tmp file, then rename
//!
//! # Examples
//!
//! ```rust,ignore
//! use toad::ai::evaluation::storage::StorageManager;
//! use toad::ai::evaluation::models::EvaluationRun;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let storage = StorageManager::new("./results");
//!
//!     // Save completed evaluation
//!     let run = EvaluationRun { /* ... */ };
//!     storage.save_evaluation_run(&run).await?;
//!
//!     // Load evaluation by ID
//!     let loaded = storage.load_evaluation_run(&run.run_id).await?;
//!
//!     Ok(())
//! }
//! ```

use crate::ai::evaluation::models::EvaluationRun;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

/// Manages persistent storage of evaluation results
///
/// Provides versioned JSON storage with support for:
/// - Atomic file writes (write to .tmp, then rename)
/// - Incremental saves during long runs (crash recovery)
/// - Run discovery and listing
/// - Automatic cleanup of abandoned temp directories
///
/// # Directory Structure
///
/// ```text
/// ./results/
/// ├── run-{uuid}.json          # Completed evaluations
/// ├── run-{uuid}.json
/// └── .tmp/
///     └── run-{uuid}/          # In-progress runs
///         ├── snapshot-1.json  # Incremental snapshots
///         ├── snapshot-2.json
///         └── ...
/// ```
///
/// # Examples
///
/// ```
/// use toad::ai::evaluation::storage::StorageManager;
///
/// let storage = StorageManager::new("./results");
/// ```
pub struct StorageManager {
    /// Base directory for storing results (e.g., "./results")
    results_dir: PathBuf,

    /// Temporary directory for in-progress runs
    tmp_dir: PathBuf,
}

impl StorageManager {
    /// Create a new storage manager
    ///
    /// # Parameters
    ///
    /// - `results_dir`: Base directory for storing evaluation results
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::evaluation::storage::StorageManager;
    ///
    /// let storage = StorageManager::new("./results");
    /// ```
    pub fn new<P: AsRef<Path>>(results_dir: P) -> Self {
        let results_dir = results_dir.as_ref().to_path_buf();
        let tmp_dir = results_dir.join(".tmp");

        Self {
            results_dir,
            tmp_dir,
        }
    }

    /// Get the default results directory
    ///
    /// Returns `./results` in the current working directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::evaluation::storage::StorageManager;
    ///
    /// let storage = StorageManager::default();
    /// ```
    pub fn default_results_dir() -> PathBuf {
        PathBuf::from("./results")
    }

    /// Generate a unique run ID
    ///
    /// Returns a UUID v4 string prefixed with "run-".
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ai::evaluation::storage::StorageManager;
    ///
    /// let run_id = StorageManager::generate_run_id();
    /// assert!(run_id.starts_with("run-"));
    /// assert!(run_id.len() > 10);
    /// ```
    pub fn generate_run_id() -> String {
        format!("run-{}", Uuid::new_v4())
    }

    /// Ensure the results directory exists
    ///
    /// Creates both the main results directory and the .tmp subdirectory.
    ///
    /// # Errors
    ///
    /// Returns error if directory creation fails (permissions, disk full, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let storage = StorageManager::new("./results");
    /// storage.ensure_results_dir_exists().await?;
    /// ```
    pub async fn ensure_results_dir_exists(&self) -> Result<()> {
        fs::create_dir_all(&self.results_dir)
            .await
            .context("Failed to create results directory")?;

        fs::create_dir_all(&self.tmp_dir)
            .await
            .context("Failed to create temporary results directory")?;

        Ok(())
    }

    /// Save a completed evaluation run
    ///
    /// Writes the evaluation to disk atomically using a write-then-rename strategy:
    /// 1. Write to `{run_id}.json.tmp`
    /// 2. Rename to `{run_id}.json`
    ///
    /// This ensures partial writes are never visible to readers.
    ///
    /// # Parameters
    ///
    /// - `run`: The completed evaluation run to save
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Directory creation fails
    /// - Serialization fails (invalid data)
    /// - File write fails (permissions, disk full)
    /// - Atomic rename fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let run = EvaluationRun { /* ... */ };
    /// storage.save_evaluation_run(&run).await?;
    /// ```
    pub async fn save_evaluation_run(&self, run: &EvaluationRun) -> Result<()> {
        self.ensure_results_dir_exists().await?;

        let filename = format!("{}.json", run.run_id);
        let filepath = self.results_dir.join(&filename);
        let tmp_filepath = self.results_dir.join(format!("{}.tmp", filename));

        // Serialize to JSON
        let json = serde_json::to_string_pretty(run)
            .context("Failed to serialize evaluation run to JSON")?;

        // Write to temporary file
        fs::write(&tmp_filepath, json)
            .await
            .context("Failed to write evaluation run to temporary file")?;

        // Atomic rename
        fs::rename(&tmp_filepath, &filepath)
            .await
            .context("Failed to atomically rename evaluation file")?;

        tracing::info!("Saved evaluation run to {}", filepath.display());

        Ok(())
    }

    /// Load an evaluation run by ID
    ///
    /// Reads and deserializes an evaluation from disk.
    ///
    /// # Parameters
    ///
    /// - `run_id`: The unique run identifier (e.g., "run-{uuid}")
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - File doesn't exist
    /// - File cannot be read (permissions)
    /// - JSON is malformed or invalid
    /// - Version mismatch (unsupported format_version)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let run = storage.load_evaluation_run("run-abc123").await?;
    /// println!("Loaded run with {} benchmarks", run.benchmark_results.len());
    /// ```
    pub async fn load_evaluation_run(&self, run_id: &str) -> Result<EvaluationRun> {
        let filename = format!("{}.json", run_id);
        let filepath = self.results_dir.join(&filename);

        // Read file
        let json = fs::read_to_string(&filepath)
            .await
            .with_context(|| format!("Failed to read evaluation run from {}", filepath.display()))?;

        // Deserialize
        let run: EvaluationRun = serde_json::from_str(&json)
            .context("Failed to deserialize evaluation run from JSON")?;

        // Version check (for future migrations)
        if run.format_version != 1 {
            anyhow::bail!(
                "Unsupported format version: {}. This version of TOAD only supports format_version: 1",
                run.format_version
            );
        }

        Ok(run)
    }

    /// Save an incremental snapshot during evaluation
    ///
    /// Writes a snapshot of the in-progress evaluation to `.tmp/{run_id}/snapshot-{n}.json`.
    /// This allows crash recovery for long-running evaluations.
    ///
    /// # Parameters
    ///
    /// - `run`: Current state of the evaluation
    /// - `snapshot_number`: Sequential snapshot number (1, 2, 3, ...)
    ///
    /// # Errors
    ///
    /// Returns error if file write fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Save snapshot after each benchmark completes
    /// for i in 1..=5 {
    ///     // Run benchmark...
    ///     storage.incremental_save(&run, i).await?;
    /// }
    /// ```
    pub async fn incremental_save(&self, run: &EvaluationRun, snapshot_number: usize) -> Result<()> {
        let run_tmp_dir = self.tmp_dir.join(&run.run_id);
        fs::create_dir_all(&run_tmp_dir)
            .await
            .context("Failed to create temporary run directory")?;

        let snapshot_filename = format!("snapshot-{}.json", snapshot_number);
        let snapshot_path = run_tmp_dir.join(&snapshot_filename);

        let json = serde_json::to_string_pretty(run)
            .context("Failed to serialize incremental snapshot to JSON")?;

        fs::write(&snapshot_path, json)
            .await
            .with_context(|| {
                format!("Failed to write incremental snapshot to {}", snapshot_path.display())
            })?;

        tracing::debug!("Saved incremental snapshot: {}", snapshot_path.display());

        Ok(())
    }

    /// List all completed evaluation runs
    ///
    /// Scans the results directory for `*.json` files and returns run IDs.
    ///
    /// # Errors
    ///
    /// Returns error if directory cannot be read
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let runs = storage.list_runs().await?;
    /// for run_id in runs {
    ///     println!("Found run: {}", run_id);
    /// }
    /// ```
    pub async fn list_runs(&self) -> Result<Vec<String>> {
        if !self.results_dir.exists() {
            return Ok(Vec::new());
        }

        let mut runs = Vec::new();
        let mut entries = fs::read_dir(&self.results_dir)
            .await
            .context("Failed to read results directory")?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read directory entry")?
        {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    if let Some(stem) = path.file_stem() {
                        runs.push(stem.to_string_lossy().to_string());
                    }
                }
            }
        }

        Ok(runs)
    }

    /// Clean up abandoned temporary directories
    ///
    /// Removes `.tmp/{run_id}/` directories for runs that don't have a corresponding
    /// completed `.json` file. This happens when evaluations crash or are cancelled.
    ///
    /// # Errors
    ///
    /// Returns error if filesystem operations fail
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Clean up on startup
    /// storage.cleanup_tmp_dirs().await?;
    /// ```
    pub async fn cleanup_tmp_dirs(&self) -> Result<usize> {
        if !self.tmp_dir.exists() {
            return Ok(0);
        }

        let mut cleaned = 0;
        let mut entries = fs::read_dir(&self.tmp_dir)
            .await
            .context("Failed to read temporary directory")?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read temporary directory entry")?
        {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            // Extract run ID from directory name
            let dir_name = path.file_name().unwrap().to_string_lossy().to_string();

            // Check if corresponding completed run exists
            let completed_path = self.results_dir.join(format!("{}.json", dir_name));
            if !completed_path.exists() {
                // Abandoned run - remove temp directory
                fs::remove_dir_all(&path)
                    .await
                    .with_context(|| format!("Failed to remove abandoned temp dir: {}", path.display()))?;

                tracing::info!("Cleaned up abandoned temp directory: {}", path.display());
                cleaned += 1;
            }
        }

        Ok(cleaned)
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new(Self::default_results_dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::evaluation::models::{AggregateMetrics, BenchmarkResult, EvaluationRun};
    use crate::benchmarks::BenchmarkMetadata;
    use chrono::Utc;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_generate_run_id() {
        let id1 = StorageManager::generate_run_id();
        let id2 = StorageManager::generate_run_id();

        assert!(id1.starts_with("run-"));
        assert!(id2.starts_with("run-"));
        assert_ne!(id1, id2, "Run IDs should be unique");
    }

    #[tokio::test]
    async fn test_save_and_load_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(temp_dir.path());

        // Create a minimal evaluation run
        let run = EvaluationRun {
            run_id: "run-test-123".to_string(),
            timestamp: Utc::now(),
            benchmark_results: vec![],
            aggregate_metrics: AggregateMetrics {
                mean_accuracy: 0.75,
                median_latency_ms: 1500.0,
                total_cost_usd: 5.25,
                total_tasks: 10,
                tasks_solved: 8,
                behavioral_metrics: None,
            },
            config_snapshot: json!({"milestone": 1}),
            format_version: 1,
        };

        // Save
        storage.save_evaluation_run(&run).await.unwrap();

        // Load
        let loaded = storage.load_evaluation_run("run-test-123").await.unwrap();

        // Verify roundtrip
        assert_eq!(loaded.run_id, run.run_id);
        assert_eq!(loaded.format_version, 1);
        assert_eq!(loaded.aggregate_metrics.mean_accuracy, 0.75);
        assert_eq!(loaded.aggregate_metrics.total_tasks, 10);
    }

    #[tokio::test]
    async fn test_incremental_save() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(temp_dir.path());

        let run = EvaluationRun {
            run_id: "run-incremental-test".to_string(),
            timestamp: Utc::now(),
            benchmark_results: vec![],
            aggregate_metrics: AggregateMetrics {
                mean_accuracy: 0.5,
                median_latency_ms: 1000.0,
                total_cost_usd: 1.0,
                total_tasks: 5,
                tasks_solved: 3,
                behavioral_metrics: None,
            },
            config_snapshot: json!({}),
            format_version: 1,
        };

        // Save 3 snapshots
        storage.incremental_save(&run, 1).await.unwrap();
        storage.incremental_save(&run, 2).await.unwrap();
        storage.incremental_save(&run, 3).await.unwrap();

        // Verify snapshots exist
        let snapshot_dir = storage.tmp_dir.join(&run.run_id);
        assert!(snapshot_dir.exists());
        assert!(snapshot_dir.join("snapshot-1.json").exists());
        assert!(snapshot_dir.join("snapshot-2.json").exists());
        assert!(snapshot_dir.join("snapshot-3.json").exists());
    }

    #[tokio::test]
    async fn test_list_runs() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(temp_dir.path());

        // Save 3 runs
        for i in 1..=3 {
            let run = EvaluationRun {
                run_id: format!("run-list-test-{}", i),
                timestamp: Utc::now(),
                benchmark_results: vec![],
                aggregate_metrics: AggregateMetrics {
                    mean_accuracy: 0.0,
                    median_latency_ms: 0.0,
                    total_cost_usd: 0.0,
                    total_tasks: 0,
                    tasks_solved: 0,
                    behavioral_metrics: None,
                },
                config_snapshot: json!({}),
                format_version: 1,
            };
            storage.save_evaluation_run(&run).await.unwrap();
        }

        // List runs
        let runs = storage.list_runs().await.unwrap();
        assert_eq!(runs.len(), 3);
        assert!(runs.contains(&"run-list-test-1".to_string()));
        assert!(runs.contains(&"run-list-test-2".to_string()));
        assert!(runs.contains(&"run-list-test-3".to_string()));
    }

    #[tokio::test]
    async fn test_cleanup_abandoned_tmp_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageManager::new(temp_dir.path());

        // Create 2 runs: 1 completed, 1 abandoned
        let completed_run = EvaluationRun {
            run_id: "run-completed".to_string(),
            timestamp: Utc::now(),
            benchmark_results: vec![],
            aggregate_metrics: AggregateMetrics {
                mean_accuracy: 0.0,
                median_latency_ms: 0.0,
                total_cost_usd: 0.0,
                total_tasks: 0,
                tasks_solved: 0,
                behavioral_metrics: None,
            },
            config_snapshot: json!({}),
            format_version: 1,
        };

        // Save completed run + incremental snapshot
        storage.save_evaluation_run(&completed_run).await.unwrap();
        storage.incremental_save(&completed_run, 1).await.unwrap();

        // Create abandoned run (temp dir only, no completed file)
        let abandoned_run = EvaluationRun {
            run_id: "run-abandoned".to_string(),
            timestamp: Utc::now(),
            benchmark_results: vec![],
            aggregate_metrics: AggregateMetrics {
                mean_accuracy: 0.0,
                median_latency_ms: 0.0,
                total_cost_usd: 0.0,
                total_tasks: 0,
                tasks_solved: 0,
                behavioral_metrics: None,
            },
            config_snapshot: json!({}),
            format_version: 1,
        };
        storage.incremental_save(&abandoned_run, 1).await.unwrap();

        // Cleanup
        let cleaned = storage.cleanup_tmp_dirs().await.unwrap();

        // Verify: abandoned removed, completed preserved
        assert_eq!(cleaned, 1);
        assert!(storage.tmp_dir.join("run-completed").exists());
        assert!(!storage.tmp_dir.join("run-abandoned").exists());
    }
}
