/// Test file discovery for multiple languages and frameworks
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Test file discovery
pub struct TestDiscovery {
    /// Test file patterns to search for
    patterns: Vec<TestPattern>,
}

/// Test file pattern configuration
#[derive(Debug, Clone)]
struct TestPattern {
    /// Directory patterns (e.g., "tests/", "test/")
    directories: Vec<String>,

    /// File name patterns (e.g., "test_*.py", "*_test.rs")
    file_patterns: Vec<String>,

    /// File extensions to check (e.g., "py", "rs", "js")
    extensions: Vec<String>,
}

impl TestDiscovery {
    /// Create a new test discovery with default patterns
    pub fn new() -> Self {
        Self {
            patterns: Self::default_patterns(),
        }
    }

    /// Get default test patterns for common languages
    fn default_patterns() -> Vec<TestPattern> {
        vec![
            // Python patterns
            TestPattern {
                directories: vec![
                    "tests".to_string(),
                    "test".to_string(),
                ],
                file_patterns: vec![
                    "test_*.py".to_string(),
                    "*_test.py".to_string(),
                ],
                extensions: vec!["py".to_string()],
            },
            // Rust patterns
            TestPattern {
                directories: vec![
                    "tests".to_string(),
                ],
                file_patterns: vec![
                    "*.rs".to_string(), // Rust tests can be in any .rs file
                ],
                extensions: vec!["rs".to_string()],
            },
            // JavaScript/TypeScript patterns
            TestPattern {
                directories: vec![
                    "tests".to_string(),
                    "test".to_string(),
                    "__tests__".to_string(),
                ],
                file_patterns: vec![
                    "*.test.js".to_string(),
                    "*.test.ts".to_string(),
                    "*.spec.js".to_string(),
                    "*.spec.ts".to_string(),
                ],
                extensions: vec!["js".to_string(), "ts".to_string()],
            },
        ]
    }

    /// Discover test files in a workspace
    ///
    /// Walks the directory tree and identifies test files based on patterns.
    pub async fn discover_tests(&self, workspace_root: &Path) -> Result<Vec<PathBuf>> {
        let mut test_files = Vec::new();

        // Walk directory tree
        self.walk_directory(workspace_root, workspace_root, &mut test_files)
            .await?;

        // Sort for deterministic results
        test_files.sort();
        test_files.dedup();

        Ok(test_files)
    }

    /// Recursively walk directory and collect test files
    async fn walk_directory(
        &self,
        workspace_root: &Path,
        current_dir: &Path,
        test_files: &mut Vec<PathBuf>,
    ) -> Result<()> {
        // Skip hidden directories and common exclude patterns
        if self.should_skip_directory(current_dir) {
            return Ok(());
        }

        let mut entries = fs::read_dir(current_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                // Recurse into subdirectories
                Box::pin(self.walk_directory(workspace_root, &path, test_files)).await?;
            } else if path.is_file() {
                // Check if file matches test patterns
                if self.is_test_file(&path) {
                    test_files.push(path);
                }
            }
        }

        Ok(())
    }

    /// Check if a file is a test file
    fn is_test_file(&self, path: &Path) -> bool {
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => return false,
        };

        let extension = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext,
            None => return false,
        };

        // Check against all patterns
        for pattern in &self.patterns {
            // Check extension
            if !pattern.extensions.iter().any(|ext| ext == extension) {
                continue;
            }

            // Check if in test directory
            let in_test_dir = pattern.directories.iter().any(|dir| {
                path.components()
                    .any(|c| c.as_os_str().to_str() == Some(dir))
            });

            // Check file name pattern
            let matches_pattern = pattern.file_patterns.iter().any(|pat| {
                self.matches_glob_pattern(file_name, pat)
            });

            if in_test_dir || matches_pattern {
                return true;
            }
        }

        false
    }

    /// Simple glob pattern matching (supports * wildcard)
    fn matches_glob_pattern(&self, text: &str, pattern: &str) -> bool {
        // Simple implementation: split on * and check parts
        let parts: Vec<&str> = pattern.split('*').collect();

        if parts.len() == 1 {
            // No wildcards, exact match
            return text == pattern;
        }

        // Check prefix
        if !parts[0].is_empty() && !text.starts_with(parts[0]) {
            return false;
        }

        // Check suffix
        if !parts[parts.len() - 1].is_empty() && !text.ends_with(parts[parts.len() - 1]) {
            return false;
        }

        // Check middle parts (if any)
        let mut pos = parts[0].len();
        for part in &parts[1..parts.len() - 1] {
            if part.is_empty() {
                continue;
            }

            match text[pos..].find(part) {
                Some(idx) => pos += idx + part.len(),
                None => return false,
            }
        }

        true
    }

    /// Check if directory should be skipped
    fn should_skip_directory(&self, path: &Path) -> bool {
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => return true,
        };

        // Skip hidden directories
        if name.starts_with('.') {
            return true;
        }

        // Skip common build/cache directories
        matches!(
            name,
            "node_modules"
                | "target"
                | "build"
                | "dist"
                | "__pycache__"
                | ".git"
                | ".venv"
                | "venv"
        )
    }
}

impl Default for TestDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_matching() {
        let discovery = TestDiscovery::new();

        assert!(discovery.matches_glob_pattern("test_foo.py", "test_*.py"));
        assert!(discovery.matches_glob_pattern("foo_test.py", "*_test.py"));
        assert!(discovery.matches_glob_pattern("test.py", "*.py"));
        assert!(discovery.matches_glob_pattern("foo.test.js", "*.test.js"));

        assert!(!discovery.matches_glob_pattern("foo.py", "test_*.py"));
        assert!(!discovery.matches_glob_pattern("test.js", "*.py"));
    }

    #[test]
    fn test_is_test_file() {
        let discovery = TestDiscovery::new();

        // Python tests
        assert!(discovery.is_test_file(Path::new("tests/test_foo.py")));
        assert!(discovery.is_test_file(Path::new("src/foo_test.py")));

        // JavaScript tests
        assert!(discovery.is_test_file(Path::new("src/foo.test.js")));
        assert!(discovery.is_test_file(Path::new("__tests__/foo.spec.js")));

        // Rust tests
        assert!(discovery.is_test_file(Path::new("tests/integration_test.rs")));

        // Not test files
        assert!(!discovery.is_test_file(Path::new("src/main.py")));
        assert!(!discovery.is_test_file(Path::new("src/app.js")));
    }

    #[test]
    fn test_should_skip_directory() {
        let discovery = TestDiscovery::new();

        assert!(discovery.should_skip_directory(Path::new("node_modules")));
        assert!(discovery.should_skip_directory(Path::new("target")));
        assert!(discovery.should_skip_directory(Path::new(".git")));
        assert!(discovery.should_skip_directory(Path::new("__pycache__")));

        assert!(!discovery.should_skip_directory(Path::new("tests")));
        assert!(!discovery.should_skip_directory(Path::new("src")));
    }
}
