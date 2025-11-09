/// Git tools - git diff and git status
use super::{Tool, ToolResult};
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

/// Git diff tool - shows changes in the repository
pub struct GitDiffTool;

impl Default for GitDiffTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GitDiffTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for GitDiffTool {
    fn name(&self) -> &str {
        "git_diff"
    }

    fn description(&self) -> &str {
        "Show changes in the git repository. Returns git diff output."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to git repository (defaults to current directory)"
                },
                "staged": {
                    "type": "boolean",
                    "description": "Show staged changes only (default: false)"
                },
                "file": {
                    "type": "string",
                    "description": "Show diff for specific file only"
                }
            }
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");

        let staged = args
            .get("staged")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let file = args.get("file").and_then(|v| v.as_str());

        // Build git diff command
        let mut cmd = Command::new("git");
        cmd.current_dir(path);
        cmd.arg("diff");

        if staged {
            cmd.arg("--cached");
        }

        if let Some(f) = file {
            cmd.arg(f);
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = match cmd.output().await {
            Ok(output) => output,
            Err(e) => {
                return Ok(ToolResult::error(
                    self.name(),
                    format!("Failed to run git diff: {}", e),
                ))
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Ok(ToolResult::error(
                self.name(),
                format!("git diff failed: {}", stderr),
            ));
        }

        if stdout.trim().is_empty() {
            Ok(ToolResult::success(
                self.name(),
                "No changes found".to_string(),
            ))
        } else {
            Ok(ToolResult::success(self.name(), stdout))
        }
    }
}

/// Git status tool - shows repository status
pub struct GitStatusTool;

impl Default for GitStatusTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GitStatusTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for GitStatusTool {
    fn name(&self) -> &str {
        "git_status"
    }

    fn description(&self) -> &str {
        "Show git repository status. Returns git status output."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to git repository (defaults to current directory)"
                },
                "short": {
                    "type": "boolean",
                    "description": "Use short format (default: false)"
                }
            }
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");

        let short = args.get("short").and_then(|v| v.as_bool()).unwrap_or(false);

        // Build git status command
        let mut cmd = Command::new("git");
        cmd.current_dir(path);
        cmd.arg("status");

        if short {
            cmd.arg("--short");
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        let output = match cmd.output().await {
            Ok(output) => output,
            Err(e) => {
                return Ok(ToolResult::error(
                    self.name(),
                    format!("Failed to run git status: {}", e),
                ))
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Ok(ToolResult::error(
                self.name(),
                format!("git status failed: {}", stderr),
            ));
        }

        Ok(ToolResult::success(self.name(), stdout))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::process::Command;

    async fn init_git_repo(path: &std::path::Path) {
        // Initialize git repo
        Command::new("git")
            .current_dir(path)
            .args(["init"])
            .output()
            .await
            .unwrap();

        // Configure git
        Command::new("git")
            .current_dir(path)
            .args(["config", "user.email", "test@example.com"])
            .output()
            .await
            .unwrap();

        Command::new("git")
            .current_dir(path)
            .args(["config", "user.name", "Test User"])
            .output()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_git_status_tool_success() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create a file
        let test_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&test_file, "content").await.unwrap();

        let tool = GitStatusTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("test.txt"));
    }

    #[tokio::test]
    async fn test_git_status_tool_short() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let test_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&test_file, "content").await.unwrap();

        let tool = GitStatusTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );
        args.insert("short".to_string(), serde_json::Value::Bool(true));

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("??")); // Untracked file marker
    }

    #[tokio::test]
    async fn test_git_diff_tool_no_changes() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        let tool = GitDiffTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("No changes"));
    }

    #[tokio::test]
    async fn test_git_diff_tool_with_changes() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create and commit a file
        let test_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&test_file, "original content")
            .await
            .unwrap();

        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "test.txt"])
            .output()
            .await
            .unwrap();

        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["commit", "-m", "initial"])
            .output()
            .await
            .unwrap();

        // Modify the file
        tokio::fs::write(&test_file, "modified content")
            .await
            .unwrap();

        let tool = GitDiffTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("test.txt"));
        assert!(result.output.contains("modified content") || result.output.contains("diff"));
    }

    #[tokio::test]
    async fn test_git_diff_tool_staged() {
        let temp_dir = TempDir::new().unwrap();
        init_git_repo(temp_dir.path()).await;

        // Create file
        let test_file = temp_dir.path().join("test.txt");
        tokio::fs::write(&test_file, "content").await.unwrap();

        // Stage the file
        Command::new("git")
            .current_dir(temp_dir.path())
            .args(["add", "test.txt"])
            .output()
            .await
            .unwrap();

        let tool = GitDiffTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );
        args.insert("staged".to_string(), serde_json::Value::Bool(true));

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        // Should show diff of staged file
        assert!(result.output.contains("test.txt") || result.output.contains("content"));
    }

    #[test]
    fn test_git_diff_tool_schema() {
        let tool = GitDiffTool::new();
        assert_eq!(tool.name(), "git_diff");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["staged"].is_object());
        assert!(schema["properties"]["file"].is_object());
    }

    #[test]
    fn test_git_status_tool_schema() {
        let tool = GitStatusTool::new();
        assert_eq!(tool.name(), "git_status");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["short"].is_object());
    }
}
