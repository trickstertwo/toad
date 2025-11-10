/// Dependency mapper - maps source files to test files
use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Maps source files to their related test files
pub struct DependencyMapper {
    /// Minimum similarity threshold for file matching (0.0-1.0)
    similarity_threshold: f64,
}

impl DependencyMapper {
    /// Create a new dependency mapper with default settings
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.3,
        }
    }

    /// Map changed source files to affected test files
    ///
    /// Uses multiple strategies:
    /// 1. Direct name matching (foo.py -> test_foo.py)
    /// 2. Directory-based matching (src/foo.py -> tests/foo_test.py)
    /// 3. Fallback: if test directory changed, run all tests
    pub async fn map_files_to_tests(
        &self,
        _workspace_root: &Path,
        changed_files: &[PathBuf],
        all_tests: &[PathBuf],
    ) -> Result<Vec<PathBuf>> {
        let mut affected_tests = HashSet::new();

        for changed_file in changed_files {
            // Strategy 1: Check if the changed file itself is a test
            if all_tests.contains(changed_file) {
                affected_tests.insert(changed_file.clone());
                continue;
            }

            // Strategy 2: Find tests by name similarity
            let related_tests = self.find_related_tests_by_name(changed_file, all_tests);
            affected_tests.extend(related_tests);

            // Strategy 3: Find tests in related directories
            let dir_tests = self.find_related_tests_by_directory(changed_file, all_tests);
            affected_tests.extend(dir_tests);
        }

        // Strategy 4: If test files themselves changed, include them
        for test_file in all_tests {
            if changed_files.contains(test_file) {
                affected_tests.insert(test_file.clone());
            }
        }

        // Convert to sorted vector for deterministic results
        let mut result: Vec<PathBuf> = affected_tests.into_iter().collect();
        result.sort();

        Ok(result)
    }

    /// Find tests related to a source file by name matching
    fn find_related_tests_by_name(
        &self,
        source_file: &Path,
        all_tests: &[PathBuf],
    ) -> Vec<PathBuf> {
        let source_stem = match source_file.file_stem().and_then(|s| s.to_str()) {
            Some(stem) => stem,
            None => return Vec::new(),
        };

        let mut related = Vec::new();

        for test_file in all_tests {
            let test_stem = match test_file.file_stem().and_then(|s| s.to_str()) {
                Some(stem) => stem,
                None => continue,
            };

            // Check various naming conventions
            let is_related =
                // test_foo.py matches foo.py
                test_stem.trim_start_matches("test_") == source_stem
                // foo_test.py matches foo.py
                || test_stem.trim_end_matches("_test") == source_stem
                // test_foo matches foo
                || test_stem == format!("test_{}", source_stem)
                // foo_test matches foo
                || test_stem == format!("{}_test", source_stem)
                // foo.test.js matches foo.js (remove .test)
                || test_stem.replace(".test", "") == source_stem
                // foo.spec.js matches foo.js (remove .spec)
                || test_stem.replace(".spec", "") == source_stem;

            if is_related {
                related.push(test_file.clone());
            }
        }

        related
    }

    /// Find tests related to a source file by directory structure
    fn find_related_tests_by_directory(
        &self,
        source_file: &Path,
        all_tests: &[PathBuf],
    ) -> Vec<PathBuf> {
        let source_components: Vec<_> = source_file.components().collect();
        let mut related = Vec::new();

        for test_file in all_tests {
            let test_components: Vec<_> = test_file.components().collect();

            // Calculate directory similarity
            let similarity = self.calculate_path_similarity(&source_components, &test_components);

            if similarity >= self.similarity_threshold {
                related.push(test_file.clone());
            }
        }

        related
    }

    /// Calculate similarity between two paths (0.0-1.0)
    ///
    /// Based on common directory components
    fn calculate_path_similarity(
        &self,
        path1: &[std::path::Component],
        path2: &[std::path::Component],
    ) -> f64 {
        if path1.is_empty() || path2.is_empty() {
            return 0.0;
        }

        // Count common leading components (excluding file name)
        let mut common = 0;
        let max_len = std::cmp::min(path1.len() - 1, path2.len() - 1);

        for i in 0..max_len {
            if path1[i] == path2[i] {
                common += 1;
            } else {
                break;
            }
        }

        // Similarity is ratio of common components
        common as f64 / max_len.max(1) as f64
    }
}

impl Default for DependencyMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_map_test_file_to_itself() {
        let mapper = DependencyMapper::new();
        let changed = vec![PathBuf::from("tests/test_foo.py")];
        let all_tests = vec![
            PathBuf::from("tests/test_foo.py"),
            PathBuf::from("tests/test_bar.py"),
        ];

        let result = mapper
            .map_files_to_tests(Path::new("."), &changed, &all_tests)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert!(result.contains(&PathBuf::from("tests/test_foo.py")));
    }

    #[tokio::test]
    async fn test_map_source_to_test_by_name() {
        let mapper = DependencyMapper::new();
        let changed = vec![PathBuf::from("src/foo.py")];
        let all_tests = vec![
            PathBuf::from("tests/test_foo.py"),
            PathBuf::from("tests/test_bar.py"),
        ];

        let result = mapper
            .map_files_to_tests(Path::new("."), &changed, &all_tests)
            .await
            .unwrap();

        assert!(result.contains(&PathBuf::from("tests/test_foo.py")));
        assert!(!result.contains(&PathBuf::from("tests/test_bar.py")));
    }

    #[test]
    fn test_find_related_by_name_python() {
        let mapper = DependencyMapper::new();
        let source = PathBuf::from("src/foo.py");
        let tests = vec![
            PathBuf::from("tests/test_foo.py"),
            PathBuf::from("tests/foo_test.py"),
            PathBuf::from("tests/test_bar.py"),
        ];

        let related = mapper.find_related_tests_by_name(&source, &tests);

        assert_eq!(related.len(), 2);
        assert!(related.contains(&PathBuf::from("tests/test_foo.py")));
        assert!(related.contains(&PathBuf::from("tests/foo_test.py")));
    }

    #[test]
    fn test_find_related_by_name_javascript() {
        let mapper = DependencyMapper::new();
        let source = PathBuf::from("src/foo.js");
        let tests = vec![
            PathBuf::from("src/foo.test.js"),
            PathBuf::from("src/foo.spec.js"),
            PathBuf::from("src/bar.test.js"),
        ];

        let related = mapper.find_related_tests_by_name(&source, &tests);

        assert_eq!(related.len(), 2);
        assert!(related.contains(&PathBuf::from("src/foo.test.js")));
        assert!(related.contains(&PathBuf::from("src/foo.spec.js")));
    }

    #[test]
    fn test_path_similarity() {
        let mapper = DependencyMapper::new();

        let path1 = PathBuf::from("src/ai/tools/write.rs");
        let path2 = PathBuf::from("src/ai/tools/test_write.rs");

        let path1_components: Vec<_> = path1.components().collect();
        let path2_components: Vec<_> = path2.components().collect();

        let similarity = mapper.calculate_path_similarity(&path1_components, &path2_components);

        // Should have high similarity (same directory)
        assert!(similarity >= 0.5);
    }
}
