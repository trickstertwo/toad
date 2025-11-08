/// Task loader for SWE-bench datasets

use super::{Task, Complexity};
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Load tasks from SWE-bench dataset
pub struct TaskLoader {
    dataset_path: PathBuf,
}

impl TaskLoader {
    /// Create a new task loader
    pub fn new(dataset_path: PathBuf) -> Self {
        Self { dataset_path }
    }

    /// Load all tasks from the dataset
    pub fn load_all(&self) -> Result<Vec<Task>> {
        let content = std::fs::read_to_string(&self.dataset_path)
            .context("Failed to read dataset file")?;

        let tasks: Vec<Value> = serde_json::from_str(&content)
            .context("Failed to parse JSON")?;

        tasks.into_iter()
            .map(|v| self.parse_task(v))
            .collect()
    }

    /// Load a sample of N tasks
    pub fn load_sample(&self, n: usize) -> Result<Vec<Task>> {
        let all_tasks = self.load_all()?;
        Ok(all_tasks.into_iter().take(n).collect())
    }

    /// Load tasks stratified by complexity
    pub fn load_stratified(&self, simple: usize, medium: usize, hard: usize) -> Result<Vec<Task>> {
        let all_tasks = self.load_all()?;

        let mut simple_tasks = Vec::new();
        let mut medium_tasks = Vec::new();
        let mut hard_tasks = Vec::new();

        for task in all_tasks {
            match task.complexity {
                Complexity::Simple => simple_tasks.push(task),
                Complexity::Medium => medium_tasks.push(task),
                Complexity::Hard => hard_tasks.push(task),
            }
        }

        let mut result = Vec::new();
        result.extend(simple_tasks.into_iter().take(simple));
        result.extend(medium_tasks.into_iter().take(medium));
        result.extend(hard_tasks.into_iter().take(hard));

        Ok(result)
    }

    /// Parse a task from JSON
    fn parse_task(&self, value: Value) -> Result<Task> {
        let obj = value.as_object().context("Task is not an object")?;

        let id = obj.get("instance_id")
            .and_then(|v| v.as_str())
            .context("Missing instance_id")?
            .to_string();

        let repo = obj.get("repo")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown/unknown")
            .to_string();

        let base_commit = obj.get("base_commit")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let problem_statement = obj.get("problem_statement")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let hints = obj.get("hints_text")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let test_patch = obj.get("test_patch")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let files_to_modify = obj.get("patch")
            .and_then(|v| v.as_str())
            .map(|patch| self.extract_files_from_patch(patch))
            .unwrap_or_default();

        let solution_patch = obj.get("patch")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Estimate complexity based on problem statement length and files
        let complexity = self.estimate_complexity(&problem_statement, &files_to_modify);

        Ok(Task {
            id,
            repo,
            base_commit,
            problem_statement,
            hints,
            test_patch,
            files_to_modify,
            solution_patch,
            complexity,
            metadata: HashMap::new(),
        })
    }

    /// Extract file paths from a unified diff patch
    fn extract_files_from_patch(&self, patch: &str) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for line in patch.lines() {
            if line.starts_with("--- a/") {
                if let Some(path) = line.strip_prefix("--- a/") {
                    files.push(PathBuf::from(path));
                }
            }
        }

        files
    }

    /// Estimate task complexity
    fn estimate_complexity(&self, problem: &str, files: &[PathBuf]) -> Complexity {
        let word_count = problem.split_whitespace().count();
        let file_count = files.len();

        // Simple heuristic:
        // - Simple: Short problem (<200 words), 1-2 files
        // - Medium: Moderate (200-500 words), 2-4 files
        // - Hard: Long (>500 words) or many files (>4)
        if word_count < 200 && file_count <= 2 {
            Complexity::Simple
        } else if word_count < 500 && file_count <= 4 {
            Complexity::Medium
        } else {
            Complexity::Hard
        }
    }
}

/// Create example tasks for testing
pub fn create_test_tasks(count: usize) -> Vec<Task> {
    (0..count)
        .map(|i| {
            let complexity = match i % 3 {
                0 => Complexity::Simple,
                1 => Complexity::Medium,
                _ => Complexity::Hard,
            };

            Task {
                id: format!("test__task-{:03}", i),
                repo: "test/repo".to_string(),
                base_commit: format!("commit{}", i),
                problem_statement: format!("Fix issue #{}", i),
                hints: Some(format!("Hint for issue {}", i)),
                test_patch: format!("test patch {}", i),
                files_to_modify: vec![PathBuf::from(format!("file{}.rs", i))],
                solution_patch: Some(format!("solution {}", i)),
                complexity,
                metadata: HashMap::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_test_tasks() {
        let tasks = create_test_tasks(10);
        assert_eq!(tasks.len(), 10);
        assert_eq!(tasks[0].complexity, Complexity::Simple);
        assert_eq!(tasks[1].complexity, Complexity::Medium);
        assert_eq!(tasks[2].complexity, Complexity::Hard);
    }

    #[test]
    fn test_extract_files_from_patch() {
        let loader = TaskLoader::new(PathBuf::from("/tmp/test"));
        let patch = r#"
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,5 @@
-old line
+new line
--- a/src/lib.rs
+++ b/src/lib.rs
"#;

        let files = loader.extract_files_from_patch(patch);
        assert_eq!(files.len(), 2);
        assert_eq!(files[0], PathBuf::from("src/main.rs"));
        assert_eq!(files[1], PathBuf::from("src/lib.rs"));
    }

    #[test]
    fn test_estimate_complexity() {
        let loader = TaskLoader::new(PathBuf::from("/tmp/test"));

        // Simple: short + few files
        let complexity = loader.estimate_complexity(
            "Short problem",
            &[PathBuf::from("file.rs")],
        );
        assert_eq!(complexity, Complexity::Simple);

        // Hard: many files
        let complexity = loader.estimate_complexity(
            "Problem",
            &[
                PathBuf::from("f1.rs"),
                PathBuf::from("f2.rs"),
                PathBuf::from("f3.rs"),
                PathBuf::from("f4.rs"),
                PathBuf::from("f5.rs"),
            ],
        );
        assert_eq!(complexity, Complexity::Hard);
    }

    #[test]
    fn test_load_from_json() {
        let json = r#"[
            {
                "instance_id": "django__django-12345",
                "repo": "django/django",
                "base_commit": "abc123",
                "problem_statement": "Fix the bug in middleware",
                "test_patch": "test code here",
                "patch": "--- a/django/middleware.py\n+++ b/django/middleware.py"
            }
        ]"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json.as_bytes()).unwrap();

        let loader = TaskLoader::new(temp_file.path().to_path_buf());
        let tasks = loader.load_all().unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "django__django-12345");
        assert_eq!(tasks[0].repo, "django/django");
    }
}
