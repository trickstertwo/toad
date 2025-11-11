/// Test execution with smart selection
use super::{TestSelection, TestSelector};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Test execution result
#[derive(Debug, Clone)]
pub struct TestExecutionResult {
    /// Test command that was run
    pub command: String,

    /// Whether tests passed
    pub success: bool,

    /// Test output
    pub output: String,

    /// Number of tests selected
    pub tests_selected: usize,

    /// Number of tests skipped
    pub tests_skipped: usize,

    /// Reduction percentage
    pub reduction_percentage: f64,
}

/// Smart test executor
///
/// Selects and runs only relevant tests based on code changes
pub struct TestExecutor {
    #[allow(dead_code)]
    selector: TestSelector,
}

impl TestExecutor {
    /// Create a new test executor
    pub fn new() -> Self {
        Self {
            selector: TestSelector::new(),
        }
    }

    /// Build test command for selected tests
    ///
    /// Generates language-specific test commands based on file patterns
    pub fn build_test_command(
        &self,
        workspace_root: &Path,
        selection: &TestSelection,
    ) -> Result<String> {
        if selection.tests_to_run().is_empty() {
            return Ok(String::new());
        }

        // Detect language/framework from test files
        let test_files = selection.tests_to_run();

        // Check for Python tests
        if test_files.iter().any(|f| {
            f.extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "py")
                .unwrap_or(false)
        }) {
            return self.build_python_command(workspace_root, test_files);
        }

        // Check for Rust tests
        if test_files.iter().any(|f| {
            f.extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "rs")
                .unwrap_or(false)
        }) {
            return self.build_rust_command(workspace_root, test_files);
        }

        // Check for JavaScript/TypeScript tests
        if test_files.iter().any(|f| {
            f.extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "js" || e == "ts")
                .unwrap_or(false)
        }) {
            return self.build_javascript_command(workspace_root, test_files);
        }

        // Fallback: run generic test command
        Ok("npm test".to_string())
    }

    /// Build Python test command (pytest)
    fn build_python_command(
        &self,
        workspace_root: &Path,
        test_files: &[PathBuf],
    ) -> Result<String> {
        // Use pytest with specific test files
        let mut cmd = String::from("pytest");

        for test_file in test_files {
            // Get relative path from workspace root
            let relative = test_file.strip_prefix(workspace_root).unwrap_or(test_file);

            cmd.push(' ');
            cmd.push_str(&relative.to_string_lossy());
        }

        cmd.push_str(" -v");
        Ok(cmd)
    }

    /// Build Rust test command (cargo test)
    fn build_rust_command(&self, _workspace_root: &Path, test_files: &[PathBuf]) -> Result<String> {
        // For Rust, we can run specific test modules
        // Extract test names from file paths
        let mut test_names = Vec::new();

        for test_file in test_files {
            if let Some(stem) = test_file.file_stem().and_then(|s| s.to_str()) {
                test_names.push(stem.to_string());
            }
        }

        if test_names.is_empty() {
            return Ok("cargo test".to_string());
        }

        // Run specific tests
        let mut cmd = String::from("cargo test");
        for name in test_names {
            cmd.push(' ');
            cmd.push_str(&name);
        }

        Ok(cmd)
    }

    /// Build JavaScript/TypeScript test command (jest/vitest)
    fn build_javascript_command(
        &self,
        workspace_root: &Path,
        test_files: &[PathBuf],
    ) -> Result<String> {
        // Try to detect test runner (check for package.json)
        let package_json = workspace_root.join("package.json");

        let test_runner = if package_json.exists() {
            // TODO: Parse package.json to detect runner
            "npm test --"
        } else {
            "npm test --"
        };

        let mut cmd = String::from(test_runner);

        for test_file in test_files {
            let relative = test_file.strip_prefix(workspace_root).unwrap_or(test_file);

            cmd.push(' ');
            cmd.push_str(&relative.to_string_lossy());
        }

        Ok(cmd)
    }
}

impl Default for TestExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = TestExecutor::new();
        assert!(true); // Verify it compiles
    }

    #[test]
    fn test_build_python_command() {
        let executor = TestExecutor::new();
        let workspace = Path::new("/project");
        let tests = vec![
            PathBuf::from("/project/tests/test_foo.py"),
            PathBuf::from("/project/tests/test_bar.py"),
        ];

        let cmd = executor.build_python_command(workspace, &tests).unwrap();

        assert!(cmd.contains("pytest"));
        assert!(cmd.contains("tests/test_foo.py"));
        assert!(cmd.contains("tests/test_bar.py"));
    }

    #[test]
    fn test_build_rust_command() {
        let executor = TestExecutor::new();
        let workspace = Path::new("/project");
        let tests = vec![PathBuf::from("/project/tests/integration_test.rs")];

        let cmd = executor.build_rust_command(workspace, &tests).unwrap();

        assert!(cmd.contains("cargo test"));
        assert!(cmd.contains("integration_test"));
    }

    #[test]
    fn test_build_javascript_command() {
        let executor = TestExecutor::new();
        let workspace = Path::new("/project");
        let tests = vec![PathBuf::from("/project/src/foo.test.js")];

        let cmd = executor
            .build_javascript_command(workspace, &tests)
            .unwrap();

        assert!(cmd.contains("npm test"));
        assert!(cmd.contains("src/foo.test.js"));
    }
}
