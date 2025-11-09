/// Task difficulty classifier for routing decisions
///
/// Analyzes task characteristics to classify difficulty:
/// - Easy: Single file, < 50 lines, simple operations
/// - Medium: Multiple files, < 200 lines, moderate complexity
/// - Hard: Large changes, complex logic, architecture decisions

use crate::ai::evaluation::Task;
use anyhow::Result;

/// Task difficulty level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Simple tasks: Single file, < 50 lines
    /// Examples: Fix typo, add simple function, rename variable
    Easy,

    /// Medium tasks: Multiple files, < 200 lines
    /// Examples: Add feature, refactor module, fix bug
    Medium,

    /// Hard tasks: Large scope, complex logic
    /// Examples: Architecture changes, multi-module features, performance optimization
    Hard,
}

/// Classifies task difficulty based on heuristics
pub struct TaskClassifier {
    /// Enable debug logging
    pub debug: bool,
}

impl TaskClassifier {
    /// Create a new task classifier
    pub fn new() -> Self {
        Self { debug: false }
    }

    /// Create classifier with debug logging
    pub fn with_debug(debug: bool) -> Self {
        Self { debug }
    }

    /// Classify task difficulty
    ///
    /// Uses multiple heuristics:
    /// 1. Problem statement length (longer = harder)
    /// 2. Number of files mentioned
    /// 3. Keywords indicating complexity
    /// 4. Estimated lines of code to change
    pub fn classify(&self, task: &Task) -> Result<Difficulty> {
        let problem = &task.problem_statement;

        // Heuristic 1: Problem statement length
        let statement_length = problem.len();

        // Heuristic 2: File mentions (rough estimate)
        let file_mentions = problem.matches(".py")
            .chain(problem.matches(".rs"))
            .chain(problem.matches(".js"))
            .chain(problem.matches(".ts"))
            .count();

        // Heuristic 3: Complexity keywords
        let has_architecture_keywords = problem.to_lowercase().contains("architecture")
            || problem.to_lowercase().contains("refactor")
            || problem.to_lowercase().contains("redesign")
            || problem.to_lowercase().contains("performance");

        let has_simple_keywords = problem.to_lowercase().contains("fix typo")
            || problem.to_lowercase().contains("rename")
            || problem.to_lowercase().contains("update comment");

        if self.debug {
            tracing::debug!(
                "Classifying task {}: length={}, files={}, arch_keywords={}, simple_keywords={}",
                task.id,
                statement_length,
                file_mentions,
                has_architecture_keywords,
                has_simple_keywords
            );
        }

        // Classification logic
        let difficulty = if has_simple_keywords && file_mentions <= 1 && statement_length < 200 {
            Difficulty::Easy
        } else if has_architecture_keywords || file_mentions > 5 || statement_length > 1000 {
            Difficulty::Hard
        } else if file_mentions > 2 || statement_length > 500 {
            Difficulty::Medium
        } else {
            Difficulty::Easy
        };

        if self.debug {
            tracing::debug!("Task {} classified as: {:?}", task.id, difficulty);
        }

        Ok(difficulty)
    }
}

impl Default for TaskClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy_task_classification() {
        let classifier = TaskClassifier::new();
        let task = Task {
            id: "test-1".to_string(),
            problem_statement: "Fix typo in utils.py".to_string(),
            ..Task::example()
        };

        let difficulty = classifier.classify(&task).unwrap();
        assert_eq!(difficulty, Difficulty::Easy);
    }

    #[test]
    fn test_medium_task_classification() {
        let classifier = TaskClassifier::new();
        let task = Task {
            id: "test-2".to_string(),
            problem_statement: "Add new authentication feature. Modify auth.py, user.py, and tests.py to implement JWT token validation.".to_string(),
            ..Task::example()
        };

        let difficulty = classifier.classify(&task).unwrap();
        assert_eq!(difficulty, Difficulty::Medium);
    }

    #[test]
    fn test_hard_task_classification() {
        let classifier = TaskClassifier::new();
        let task = Task {
            id: "test-3".to_string(),
            problem_statement: "Refactor the entire authentication architecture to improve performance. This requires changes across auth.py, middleware.py, database.py, cache.py, config.py, and all related test files.".to_string(),
            ..Task::example()
        };

        let difficulty = classifier.classify(&task).unwrap();
        assert_eq!(difficulty, Difficulty::Hard);
    }
}
