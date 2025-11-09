/// Experiment tracking and management
///
/// This module provides infrastructure for managing A/B experiments,
/// tracking results, and making decisions based on statistical evidence.
use super::EvaluationResults;
use crate::config::FeatureFlags;
use crate::stats::{ComparisonResult, Recommendation};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// An experimental hypothesis to test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    /// Unique experiment ID
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Hypothesis being tested
    pub hypothesis: String,

    /// Baseline configuration
    pub baseline: FeatureFlags,

    /// Treatment configuration
    pub treatment: FeatureFlags,

    /// Expected improvement
    pub expected_improvement: f64,

    /// Status
    pub status: ExperimentStatus,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Results (if completed)
    pub results: Option<ExperimentResults>,
}

/// Status of an experiment
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExperimentStatus {
    /// Planned but not started
    Planned,
    /// Currently running
    Running,
    /// Completed successfully
    Completed,
    /// Failed to complete
    Failed,
    /// Cancelled
    Cancelled,
}

/// Results of an experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResults {
    /// Baseline evaluation results
    pub baseline_results: EvaluationResults,

    /// Treatment evaluation results
    pub treatment_results: EvaluationResults,

    /// Statistical comparison
    pub comparison: ComparisonResult,

    /// Decision
    pub decision: Recommendation,

    /// Notes/observations
    pub notes: String,

    /// Completed timestamp
    pub completed_at: DateTime<Utc>,
}

/// Manages experiments across the project
pub struct ExperimentManager {
    /// Directory to store experiments
    experiments_dir: PathBuf,

    /// Loaded experiments
    experiments: HashMap<String, Experiment>,
}

impl ExperimentManager {
    /// Create a new experiment manager
    pub fn new(experiments_dir: PathBuf) -> Self {
        Self {
            experiments_dir,
            experiments: HashMap::new(),
        }
    }

    /// Get the default experiments directory
    pub fn default_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".toad").join("experiments")
    }

    /// Initialize the experiments directory
    pub fn init(&self) -> Result<()> {
        std::fs::create_dir_all(&self.experiments_dir)?;
        Ok(())
    }

    /// Create a new experiment
    pub fn create_experiment(
        &mut self,
        name: String,
        hypothesis: String,
        baseline: FeatureFlags,
        treatment: FeatureFlags,
        expected_improvement: f64,
    ) -> Result<String> {
        let id = format!(
            "exp_{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );

        let experiment = Experiment {
            id: id.clone(),
            name,
            hypothesis,
            baseline,
            treatment,
            expected_improvement,
            status: ExperimentStatus::Planned,
            created_at: Utc::now(),
            results: None,
        };

        self.experiments.insert(id.clone(), experiment.clone());
        self.save_experiment(&experiment)?;

        Ok(id)
    }

    /// Get an experiment by ID
    pub fn get(&self, id: &str) -> Option<&Experiment> {
        self.experiments.get(id)
    }

    /// Update experiment status
    pub fn update_status(&mut self, id: &str, status: ExperimentStatus) -> Result<()> {
        if let Some(exp) = self.experiments.get_mut(id) {
            exp.status = status;
        }

        if let Some(exp) = self.experiments.get(id) {
            self.save_experiment(exp)?;
        }
        Ok(())
    }

    /// Record experiment results
    pub fn record_results(
        &mut self,
        id: &str,
        baseline_results: EvaluationResults,
        treatment_results: EvaluationResults,
        comparison: ComparisonResult,
        notes: String,
    ) -> Result<()> {
        if let Some(exp) = self.experiments.get_mut(id) {
            exp.results = Some(ExperimentResults {
                baseline_results,
                treatment_results,
                comparison: comparison.clone(),
                decision: comparison.recommendation,
                notes,
                completed_at: Utc::now(),
            });
            exp.status = ExperimentStatus::Completed;
        }

        if let Some(exp) = self.experiments.get(id) {
            self.save_experiment(exp)?;
        }
        Ok(())
    }

    /// List all experiments
    pub fn list(&self) -> Vec<&Experiment> {
        self.experiments.values().collect()
    }

    /// List experiments by status
    pub fn list_by_status(&self, status: ExperimentStatus) -> Vec<&Experiment> {
        self.experiments
            .values()
            .filter(|e| e.status == status)
            .collect()
    }

    /// Save an experiment to disk
    fn save_experiment(&self, experiment: &Experiment) -> Result<()> {
        self.init()?;
        let path = self.experiments_dir.join(format!("{}.json", experiment.id));
        let json = serde_json::to_string_pretty(experiment)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load an experiment from disk
    pub fn load_experiment(&mut self, id: &str) -> Result<()> {
        let path = self.experiments_dir.join(format!("{}.json", id));
        let json = std::fs::read_to_string(path)?;
        let experiment: Experiment = serde_json::from_str(&json)?;
        self.experiments.insert(id.to_string(), experiment);
        Ok(())
    }

    /// Load all experiments from disk
    pub fn load_all(&mut self) -> Result<()> {
        self.init()?;

        for entry in std::fs::read_dir(&self.experiments_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = std::fs::read_to_string(&path) {
                    if let Ok(exp) = serde_json::from_str::<Experiment>(&json) {
                        self.experiments.insert(exp.id.clone(), exp);
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate experiment report
    pub fn generate_report(&self) -> String {
        let mut report = String::from("# Experiment Report\n\n");

        let planned = self.list_by_status(ExperimentStatus::Planned);
        let running = self.list_by_status(ExperimentStatus::Running);
        let completed = self.list_by_status(ExperimentStatus::Completed);
        let failed = self.list_by_status(ExperimentStatus::Failed);

        report.push_str("## Summary\n");
        report.push_str(&format!("- Planned: {}\n", planned.len()));
        report.push_str(&format!("- Running: {}\n", running.len()));
        report.push_str(&format!("- Completed: {}\n", completed.len()));
        report.push_str(&format!("- Failed: {}\n\n", failed.len()));

        if !completed.is_empty() {
            report.push_str("## Completed Experiments\n\n");
            for exp in completed {
                report.push_str(&format!("### {} ({})\n", exp.name, exp.id));
                report.push_str(&format!("**Hypothesis:** {}\n\n", exp.hypothesis));

                if let Some(results) = &exp.results {
                    report.push_str(&format!(
                        "**Baseline:** {:.2}%\n",
                        results.baseline_results.accuracy
                    ));
                    report.push_str(&format!(
                        "**Treatment:** {:.2}%\n",
                        results.treatment_results.accuracy
                    ));
                    report.push_str(&format!(
                        "**Delta:** {:+.2}%\n",
                        results.comparison.delta.accuracy
                    ));
                    report.push_str(&format!("**Decision:** {:?}\n\n", results.decision));
                }
            }
        }

        report
    }
}

impl Default for ExperimentManager {
    fn default() -> Self {
        Self::new(Self::default_dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_experiment() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = ExperimentManager::new(temp_dir.path().to_path_buf());

        let baseline = FeatureFlags::milestone_1();
        let treatment = FeatureFlags::milestone_2();

        let id = manager
            .create_experiment(
                "Test AST context".to_string(),
                "AST improves accuracy by 3%".to_string(),
                baseline,
                treatment,
                3.0,
            )
            .unwrap();

        assert!(manager.get(&id).is_some());
        assert_eq!(manager.get(&id).unwrap().status, ExperimentStatus::Planned);
    }

    #[test]
    fn test_update_status() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = ExperimentManager::new(temp_dir.path().to_path_buf());

        let id = manager
            .create_experiment(
                "Test".to_string(),
                "Hypothesis".to_string(),
                FeatureFlags::default(),
                FeatureFlags::default(),
                1.0,
            )
            .unwrap();

        manager
            .update_status(&id, ExperimentStatus::Running)
            .unwrap();
        assert_eq!(manager.get(&id).unwrap().status, ExperimentStatus::Running);
    }

    #[test]
    fn test_list_by_status() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = ExperimentManager::new(temp_dir.path().to_path_buf());

        manager
            .create_experiment(
                "Exp 1".to_string(),
                "H1".to_string(),
                FeatureFlags::default(),
                FeatureFlags::default(),
                1.0,
            )
            .unwrap();

        let id2 = manager
            .create_experiment(
                "Exp 2".to_string(),
                "H2".to_string(),
                FeatureFlags::default(),
                FeatureFlags::default(),
                2.0,
            )
            .unwrap();

        manager
            .update_status(&id2, ExperimentStatus::Running)
            .unwrap();

        let planned = manager.list_by_status(ExperimentStatus::Planned);
        let running = manager.list_by_status(ExperimentStatus::Running);

        assert_eq!(planned.len(), 1);
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = ExperimentManager::new(temp_dir.path().to_path_buf());

        let id = manager
            .create_experiment(
                "Test".to_string(),
                "Hypothesis".to_string(),
                FeatureFlags::default(),
                FeatureFlags::default(),
                1.0,
            )
            .unwrap();

        // Create new manager and load
        let mut manager2 = ExperimentManager::new(temp_dir.path().to_path_buf());
        manager2.load_experiment(&id).unwrap();

        assert!(manager2.get(&id).is_some());
        assert_eq!(manager2.get(&id).unwrap().name, "Test");
    }

    #[test]
    fn test_generate_report() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = ExperimentManager::new(temp_dir.path().to_path_buf());

        manager
            .create_experiment(
                "Exp 1".to_string(),
                "H1".to_string(),
                FeatureFlags::default(),
                FeatureFlags::default(),
                1.0,
            )
            .unwrap();

        let report = manager.generate_report();
        assert!(report.contains("Experiment Report"));
        assert!(report.contains("Planned: 1"));
    }
}
