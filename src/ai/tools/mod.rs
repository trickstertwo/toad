/// M1 Tool System
///
/// This module implements the basic tool execution framework for TOAD.
/// All tools follow a simple interface: take input, return result.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod bash;
pub mod edit;
pub mod git;
pub mod grep;
pub mod list;
pub mod read;
pub mod write;

pub use bash::BashTool;
pub use edit::EditTool;
pub use git::{GitDiffTool, GitStatusTool};
pub use grep::GrepTool;
pub use list::ListTool;
pub use read::ReadTool;
pub use write::WriteTool;

/// Result of tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Tool name that produced this result
    pub tool: String,

    /// Output from the tool
    pub output: String,

    /// Whether the tool succeeded
    pub success: bool,

    /// Error message if failed
    pub error: Option<String>,

    /// Exit code (for bash commands)
    pub exit_code: Option<i32>,
}

impl ToolResult {
    /// Create a success result
    pub fn success(tool: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            output: output.into(),
            success: true,
            error: None,
            exit_code: Some(0),
        }
    }

    /// Create an error result
    pub fn error(tool: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            output: String::new(),
            success: false,
            error: Some(error.into()),
            exit_code: Some(1),
        }
    }
}

/// A tool call request from the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool name to execute
    pub tool: String,

    /// Arguments for the tool
    pub arguments: HashMap<String, serde_json::Value>,
}

/// Trait that all tools must implement
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    /// Name of the tool
    fn name(&self) -> &str;

    /// Description of what the tool does
    fn description(&self) -> &str;

    /// JSON schema for the tool's parameters
    fn parameters_schema(&self) -> serde_json::Value;

    /// Execute the tool with given arguments
    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult>;
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }

    /// List all registered tools
    pub fn list(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    /// Get tool count
    pub fn count(&self) -> usize {
        self.tools.len()
    }

    /// Create registry with M1 baseline tools (no validation)
    pub fn m1_baseline() -> Self {
        let mut registry = Self::new();

        // Register all M1 baseline tools (8 total)
        registry.register(Box::new(ReadTool::new()));
        registry.register(Box::new(WriteTool::new()));
        registry.register(Box::new(ListTool::new()));
        registry.register(Box::new(EditTool::new()));
        registry.register(Box::new(BashTool::new()));
        registry.register(Box::new(GrepTool::new()));
        registry.register(Box::new(GitDiffTool::new()));
        registry.register(Box::new(GitStatusTool::new()));

        registry
    }

    /// Create registry with M1 baseline tools and feature flags
    pub fn m1_with_features(features: &crate::config::FeatureFlags) -> Self {
        let mut registry = Self::new();

        // Register all M1 baseline tools (8 total)
        // WriteTool uses tree-sitter validation if enabled
        registry.register(Box::new(ReadTool::new()));
        registry.register(Box::new(WriteTool::with_validation(
            features.tree_sitter_validation,
        )));
        registry.register(Box::new(ListTool::new()));
        registry.register(Box::new(EditTool::new()));
        registry.register(Box::new(BashTool::new()));
        registry.register(Box::new(GrepTool::new()));
        registry.register(Box::new(GitDiffTool::new()));
        registry.register(Box::new(GitStatusTool::new()));

        registry
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_result_success() {
        let result = ToolResult::success("test_tool", "output");
        assert!(result.success);
        assert_eq!(result.output, "output");
        assert_eq!(result.error, None);
    }

    #[test]
    fn test_tool_result_error() {
        let result = ToolResult::error("test_tool", "error message");
        assert!(!result.success);
        assert_eq!(result.error, Some("error message".to_string()));
    }

    #[test]
    fn test_registry_basic() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.count(), 0);
        assert!(registry.list().is_empty());
    }

    #[test]
    fn test_m1_baseline_tools() {
        let registry = ToolRegistry::m1_baseline();

        // M1 has all 8 baseline tools implemented
        assert_eq!(registry.count(), 8);

        // Verify all tools are registered
        assert!(registry.get("read").is_some());
        assert!(registry.get("write").is_some());
        assert!(registry.get("list").is_some());
        assert!(registry.get("edit").is_some());
        assert!(registry.get("bash").is_some());
        assert!(registry.get("grep").is_some());
        assert!(registry.get("git_diff").is_some());
        assert!(registry.get("git_status").is_some());
    }
}
