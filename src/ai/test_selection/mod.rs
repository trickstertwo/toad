/// Smart test selection using change analysis and dependency mapping
///
/// This module implements intelligent test selection to run only tests
/// affected by code changes, reducing test execution time while maintaining
/// confidence.
///
/// Evidence: AutoCodeRover proven, +3-5 points accuracy improvement
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub mod discovery;
pub mod executor;
pub mod mapper;

pub use discovery::TestDiscovery;
pub use executor::{TestExecutionResult, TestExecutor};
pub use mapper::DependencyMapper;

/// Test selection result
#[derive(Debug, Clone)]
pub struct TestSelection {
    /// Files that changed (from git diff)
    pub changed_files: Vec<PathBuf>,

    /// Tests to run based on changes
    pub selected_tests: Vec<PathBuf>,

    /// All available tests (for fallback)
    pub all_tests: Vec<PathBuf>,

    /// Whether to run all tests (fallback mode)
    pub run_all: bool,
}

impl TestSelection {
    /// Create a new test selection
    pub fn new(
        changed_files: Vec<PathBuf>,
        selected_tests: Vec<PathBuf>,
        all_tests: Vec<PathBuf>,
    ) -> Self {
        Self {
            changed_files,
            selected_tests,
            all_tests,
            run_all: false,
        }
    }

    /// Create a selection that runs all tests (fallback)
    pub fn run_all(all_tests: Vec<PathBuf>) -> Self {
        Self {
            changed_files: Vec::new(),
            selected_tests: all_tests.clone(),
            all_tests,
            run_all: true,
        }
    }

    /// Get tests to execute
    pub fn tests_to_run(&self) -> &[PathBuf] {
        &self.selected_tests
    }

    /// Check if any tests were selected
    pub fn has_tests(&self) -> bool {
        !self.selected_tests.is_empty()
    }

    /// Get the number of selected tests
    pub fn count(&self) -> usize {
        self.selected_tests.len()
    }

    /// Get the reduction percentage (tests skipped)
    pub fn reduction_percentage(&self) -> f64 {
        if self.all_tests.is_empty() {
            return 0.0;
        }

        let skipped = self.all_tests.len() - self.selected_tests.len();
        (skipped as f64 / self.all_tests.len() as f64) * 100.0
    }
}

/// Smart test selector
pub struct TestSelector {
    /// Test file discovery
    discovery: TestDiscovery,

    /// Dependency mapper
    mapper: DependencyMapper,
}

impl TestSelector {
    /// Create a new test selector
    pub fn new() -> Self {
        Self {
            discovery: TestDiscovery::new(),
            mapper: DependencyMapper::new(),
        }
    }

    /// Select tests based on changed files
    ///
    /// # Arguments
    /// * `workspace_root` - Root directory of the workspace
    /// * `changed_files` - Files that have changed (from git diff)
    ///
    /// # Returns
    /// Test selection with files to run
    pub async fn select_tests(
        &self,
        workspace_root: &Path,
        changed_files: &[PathBuf],
    ) -> Result<TestSelection> {
        // Discover all test files
        let all_tests = self
            .discovery
            .discover_tests(workspace_root)
            .await
            .context("Failed to discover tests")?;

        // If no changes or no tests, run all tests
        if changed_files.is_empty() || all_tests.is_empty() {
            return Ok(TestSelection::run_all(all_tests));
        }

        // Map changed files to affected tests
        let affected_tests = self
            .mapper
            .map_files_to_tests(workspace_root, changed_files, &all_tests)
            .await
            .context("Failed to map files to tests")?;

        // If no specific tests found, run all (safe fallback)
        if affected_tests.is_empty() {
            return Ok(TestSelection::run_all(all_tests));
        }

        Ok(TestSelection::new(
            changed_files.to_vec(),
            affected_tests,
            all_tests,
        ))
    }

    /// Select tests using git diff to detect changes
    ///
    /// # Arguments
    /// * `workspace_root` - Root directory of the workspace
    /// * `base_ref` - Git reference to compare against (e.g., "HEAD", "main")
    ///
    /// # Returns
    /// Test selection with files to run
    pub async fn select_tests_from_git(
        &self,
        workspace_root: &Path,
        base_ref: Option<&str>,
    ) -> Result<TestSelection> {
        // Get changed files from git
        let changed_files = self.get_changed_files_from_git(workspace_root, base_ref).await?;

        // Select tests based on changes
        self.select_tests(workspace_root, &changed_files).await
    }

    /// Get changed files from git diff
    async fn get_changed_files_from_git(
        &self,
        workspace_root: &Path,
        base_ref: Option<&str>,
    ) -> Result<Vec<PathBuf>> {
        use tokio::process::Command;

        let base = base_ref.unwrap_or("HEAD");

        // Run git diff to get changed files
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg(base)
            .current_dir(workspace_root)
            .output()
            .await
            .context("Failed to run git diff")?;

        if !output.status.success() {
            // If git diff fails, return empty (will run all tests)
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let files: Vec<PathBuf> = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| workspace_root.join(line))
            .collect();

        Ok(files)
    }
}

impl Default for TestSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_new() {
        let changed = vec![PathBuf::from("src/main.rs")];
        let selected = vec![PathBuf::from("tests/test_main.rs")];
        let all = vec![
            PathBuf::from("tests/test_main.rs"),
            PathBuf::from("tests/test_other.rs"),
        ];

        let selection = TestSelection::new(changed, selected, all);

        assert_eq!(selection.count(), 1);
        assert!(selection.has_tests());
        assert!(!selection.run_all);
        assert_eq!(selection.reduction_percentage(), 50.0);
    }

    #[test]
    fn test_selection_run_all() {
        let all = vec![
            PathBuf::from("tests/test_main.rs"),
            PathBuf::from("tests/test_other.rs"),
        ];

        let selection = TestSelection::run_all(all);

        assert_eq!(selection.count(), 2);
        assert!(selection.has_tests());
        assert!(selection.run_all);
        assert_eq!(selection.reduction_percentage(), 0.0);
    }

    #[tokio::test]
    async fn test_selector_creation() {
        let _selector = TestSelector::new();
        // Just verify it compiles and creates
    }
}
