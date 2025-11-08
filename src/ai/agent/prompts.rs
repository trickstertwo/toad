/// Prompt building for M1 baseline agent
///
/// Constructs effective prompts for coding tasks based on:
/// - Aider's successful prompting strategies
/// - Claude's tool use best practices
/// - SWE-bench task format
use crate::ai::evaluation::Task;

pub struct PromptBuilder {
    task: Option<Task>,
    system_prompt: Option<String>,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            task: None,
            system_prompt: None,
        }
    }

    pub fn with_task(mut self, task: &Task) -> Self {
        self.task = Some(task.clone());
        self
    }

    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = Some(prompt);
        self
    }

    pub fn build(self) -> String {
        let mut prompt = String::new();

        // Add system prompt or default
        if let Some(sys) = self.system_prompt {
            prompt.push_str(&sys);
            prompt.push_str("\n\n");
        } else {
            prompt.push_str(&Self::default_system_prompt());
            prompt.push_str("\n\n");
        }

        // Add task
        if let Some(task) = self.task {
            prompt.push_str("# Task\n\n");
            prompt.push_str(&format!("**Instance ID:** {}\n\n", task.id));
            prompt.push_str(&format!(
                "**Problem Statement:**\n{}\n\n",
                task.problem_statement
            ));

            if let Some(hints) = task.hints {
                if !hints.is_empty() {
                    prompt.push_str(&format!("**Hints:**\n{}\n\n", hints));
                }
            }
        }

        prompt
    }

    /// Default system prompt for M1 baseline agent
    /// Based on Aider's successful prompting and Claude's tool use guidelines
    fn default_system_prompt() -> String {
        r#"You are an expert software engineer tasked with solving a coding problem.

# Your Capabilities

You have access to tools that let you:
- Read and write files
- List directory contents
- Search for patterns in files (grep)
- Edit files with search/replace
- Run shell commands
- Check git status and diffs

# Your Task

Your goal is to solve the given problem by:
1. Understanding the problem statement
2. Exploring the codebase to find relevant files
3. Making necessary changes to fix the issue
4. Verifying your solution works

# Guidelines

- **Be methodical**: Start by exploring the codebase to understand the context
- **Make focused changes**: Edit only what's necessary to solve the problem
- **Verify your work**: Run tests when available to confirm your solution
- **Use git**: Check git status/diff to see what you've changed
- **Think step-by-step**: Break down complex problems into smaller steps
- **Be precise**: When editing files, ensure search patterns match exactly
- **Handle errors**: If a tool fails, understand why and try a different approach

# Response Format

When you complete the task:
- Explain what you did and why
- Confirm the solution addresses the problem
- Note any tests run and their results

Start by exploring the codebase to understand the problem context."#
            .to_string()
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_prompt() {
        let builder = PromptBuilder::new();
        let prompt = builder.build();

        assert!(prompt.contains("expert software engineer"));
        assert!(prompt.contains("# Your Capabilities"));
        assert!(prompt.contains("# Your Task"));
        assert!(prompt.contains("# Guidelines"));
    }

    #[test]
    fn test_prompt_with_task() {
        let task = Task::example();

        let builder = PromptBuilder::new().with_task(&task);
        let prompt = builder.build();

        assert!(prompt.contains(&task.id));
        assert!(prompt.contains(&task.problem_statement));
        assert!(prompt.contains("# Task"));
    }

    #[test]
    fn test_custom_system_prompt() {
        let custom = "Custom instructions".to_string();
        let builder = PromptBuilder::new().with_system_prompt(custom.clone());
        let prompt = builder.build();

        assert!(prompt.contains(&custom));
        assert!(!prompt.contains("expert software engineer"));
    }

    #[test]
    fn test_task_with_hints() {
        let mut task = Task::example();
        task.hints = Some("Check the utils module".to_string());

        let builder = PromptBuilder::new().with_task(&task);
        let prompt = builder.build();

        assert!(prompt.contains("Check the utils module"));
        assert!(prompt.contains("**Hints:**"));
    }
}
