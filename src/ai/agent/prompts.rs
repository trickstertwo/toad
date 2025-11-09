/// Prompt building for M1 baseline agent
///
/// Constructs effective prompts for coding tasks based on:
/// - Aider's successful prompting strategies
/// - Claude's tool use best practices
/// - SWE-bench task format
use crate::ai::context::AstContext;
use crate::ai::evaluation::Task;

pub struct PromptBuilder {
    task: Option<Task>,
    system_prompt: Option<String>,
    ast_context: Option<AstContext>,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            task: None,
            system_prompt: None,
            ast_context: None,
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

    pub fn with_ast_context(mut self, context: AstContext) -> Self {
        self.ast_context = Some(context);
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

        // Add AST context if provided
        if let Some(context) = self.ast_context {
            prompt.push_str(&Self::format_ast_context(&context));
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

            if let Some(hints) = task.hints
                && !hints.is_empty() {
                    prompt.push_str(&format!("**Hints:**\n{}\n\n", hints));
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

    /// Format AST context for inclusion in the prompt
    fn format_ast_context(context: &AstContext) -> String {
        use crate::ai::context::SymbolKind;

        let mut output = String::from("# Codebase Context\n\n");
        output.push_str(&format!(
            "The codebase contains {} files with {} total symbols.\n\n",
            context.file_contexts.len(),
            context.total_symbols
        ));

        // Sort files by path for consistent output
        let mut files: Vec<_> = context.file_contexts.values().collect();
        files.sort_by_key(|f| &f.path);

        // Limit to top 10 most relevant files to avoid overwhelming the prompt
        let max_files = 10;
        if files.len() > max_files {
            output.push_str(&format!(
                "Showing {} most relevant files:\n\n",
                max_files
            ));
        }

        for file_context in files.iter().take(max_files) {
            output.push_str(&format!("## {}\n\n", file_context.path.display()));
            output.push_str(&format!("**Language:** {:?}\n\n", file_context.language));

            // Group symbols by kind
            let functions = file_context.symbols_of_kind(SymbolKind::Function);
            let classes = file_context.symbols_of_kind(SymbolKind::Class);
            let interfaces = file_context.symbols_of_kind(SymbolKind::Interface);
            let types = file_context.symbols_of_kind(SymbolKind::Type);

            if !classes.is_empty() {
                output.push_str("**Classes:**\n");
                for class in classes {
                    output.push_str(&format!("- `{}` (lines {}-{})\n",
                        class.name,
                        class.line_range.0,
                        class.line_range.1
                    ));
                    if let Some(doc) = &class.docstring {
                        output.push_str(&format!("  {}\n", doc.lines().next().unwrap_or("")));
                    }
                }
                output.push('\n');
            }

            if !functions.is_empty() {
                output.push_str("**Functions:**\n");
                for func in functions.iter().take(5) {  // Limit to 5 functions per file
                    if let Some(sig) = &func.signature {
                        output.push_str(&format!("- `{}` (lines {}-{})\n",
                            sig,
                            func.line_range.0,
                            func.line_range.1
                        ));
                    } else {
                        output.push_str(&format!("- `{}` (lines {}-{})\n",
                            func.name,
                            func.line_range.0,
                            func.line_range.1
                        ));
                    }
                    if let Some(doc) = &func.docstring {
                        output.push_str(&format!("  {}\n", doc.lines().next().unwrap_or("")));
                    }
                }
                if functions.len() > 5 {
                    output.push_str(&format!("  ... and {} more functions\n", functions.len() - 5));
                }
                output.push('\n');
            }

            if !interfaces.is_empty() {
                output.push_str("**Interfaces:**\n");
                for interface in interfaces {
                    output.push_str(&format!("- `{}` (lines {}-{})\n",
                        interface.name,
                        interface.line_range.0,
                        interface.line_range.1
                    ));
                }
                output.push('\n');
            }

            if !types.is_empty() {
                output.push_str("**Type Aliases:**\n");
                for type_alias in types {
                    output.push_str(&format!("- `{}` (lines {}-{})\n",
                        type_alias.name,
                        type_alias.line_range.0,
                        type_alias.line_range.1
                    ));
                }
                output.push('\n');
            }

            // Show imports if not too many
            if !file_context.imports.is_empty() && file_context.imports.len() <= 5 {
                output.push_str("**Imports:**\n");
                for import in &file_context.imports {
                    output.push_str(&format!("- `{}` (line {})\n", import.module, import.line));
                }
                output.push('\n');
            }
        }

        output
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

    #[test]
    fn test_prompt_with_ast_context() {
        use crate::ai::context::{AstContext, FileContext, Language, Symbol, SymbolKind};
        use std::path::PathBuf;

        // Create a simple AST context
        let mut context = AstContext::new();
        let mut file_ctx = FileContext::new(PathBuf::from("test.py"), Language::Python);

        file_ctx.add_symbol(
            Symbol::new("test_function", SymbolKind::Function, (10, 20))
                .with_signature("def test_function(x, y)")
                .with_docstring("A test function")
        );

        file_ctx.add_symbol(
            Symbol::new("TestClass", SymbolKind::Class, (30, 50))
                .with_docstring("A test class")
        );

        context.add_file(file_ctx);

        let builder = PromptBuilder::new().with_ast_context(context);
        let prompt = builder.build();

        // Verify AST context is included
        assert!(prompt.contains("# Codebase Context"));
        assert!(prompt.contains("test.py"));
        assert!(prompt.contains("test_function"));
        assert!(prompt.contains("TestClass"));
        assert!(prompt.contains("A test function"));
        assert!(prompt.contains("A test class"));
        assert!(prompt.contains("**Functions:**"));
        assert!(prompt.contains("**Classes:**"));
    }
}
