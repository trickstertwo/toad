/// RunTests tool - Intelligently selects and runs relevant tests
///
/// Uses smart test selection (M2 feature) to:
/// - Discover test files in workspace
/// - Analyze git changes to find affected tests
/// - Run only relevant tests (faster, less token cost)
/// - Fall back to all tests if selection fails
///
/// Evidence: AutoCodeRover proven (+3-5 points with smart test selection)

use super::{Tool, ToolResult};
use crate::ai::test_selection::TestSelector;
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

pub struct RunTestsTool {
    /// Whether to use smart test selection
    pub smart_selection: bool,
}

impl RunTestsTool {
    /// Create new run tests tool with smart selection disabled
    pub fn new() -> Self {
        Self {
            smart_selection: false,
        }
    }

    /// Create run tests tool with smart selection enabled (M2)
    pub fn with_smart_selection(enabled: bool) -> Self {
        Self {
            smart_selection: enabled,
        }
    }

    /// Execute test command and capture output
    fn run_command(&self, command: &str, workspace: &PathBuf) -> Result<String> {
        tracing::info!("Running test command: {}", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .current_dir(workspace)
                .output()?
        } else {
            Command::new("sh")
                .args(["-c", command])
                .current_dir(workspace)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let combined = if !stderr.is_empty() {
            format!("{}\n\nSTDERR:\n{}", stdout, stderr)
        } else {
            stdout.to_string()
        };

        if output.status.success() {
            Ok(combined)
        } else {
            Ok(format!("Tests failed:\n{}", combined))
        }
    }
}

impl Default for RunTestsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Tool for RunTestsTool {
    fn name(&self) -> &str {
        "run_tests"
    }

    fn description(&self) -> &str {
        if self.smart_selection {
            "Run tests intelligently - automatically selects only relevant tests based on code changes (M2 smart test selection)"
        } else {
            "Run all tests in the workspace"
        }
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "workspace_path": {
                    "type": "string",
                    "description": "Path to the workspace/repository root"
                },
                "test_command": {
                    "type": "string",
                    "description": "Optional: Custom test command to run. If not provided, will auto-detect (pytest, cargo test, npm test, etc.)"
                },
                "base_ref": {
                    "type": "string",
                    "description": "Optional: Git ref to compare against for smart selection (default: main/master)"
                }
            },
            "required": ["workspace_path"]
        })
    }

    async fn execute(&self, args: HashMap<String, Value>) -> Result<ToolResult> {
        let workspace_str = args.get("workspace_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("workspace_path is required"))?;

        let workspace = PathBuf::from(workspace_str);

        if !workspace.exists() {
            return Ok(ToolResult::error(
                "run_tests",
                format!("Workspace not found: {}", workspace.display()),
            ));
        }

        // Get optional parameters
        let custom_command = args.get("test_command").and_then(|v| v.as_str());
        let base_ref = args.get("base_ref").and_then(|v| v.as_str());

        if self.smart_selection {
            // M2: Use smart test selection
            tracing::info!("Using M2 smart test selection");

            let selector = TestSelector::new();

            // Get test selection based on git changes
            let selection = selector
                .select_tests_from_git(&workspace, base_ref)
                .await?;

            if selection.run_all {
                tracing::warn!("Smart selection failed or no changes detected, running all tests");

                // Fall back to running all tests
                let command = custom_command.unwrap_or("pytest -v");
                let output = self.run_command(command, &workspace)?;

                Ok(ToolResult::success(
                    "run_tests",
                    format!("Ran all tests (smart selection unavailable)\n\n{}", output),
                ))
            } else {
                // Run only selected tests
                let reduction = selection.reduction_percentage();
                tracing::info!(
                    "Smart selection: Running {}/{} tests ({:.1}% reduction)",
                    selection.selected_tests.len(),
                    selection.all_tests.len(),
                    reduction
                );

                // Build command for selected tests
                let command = if let Some(cmd) = custom_command {
                    cmd.to_string()
                } else {
                    // Auto-detect and build command
                    use crate::ai::test_selection::TestExecutor;
                    let executor = TestExecutor::new();
                    executor.build_test_command(&workspace, &selection)?
                };

                let output = self.run_command(&command, &workspace)?;

                Ok(ToolResult::success(
                    "run_tests",
                    format!(
                        "Smart test selection: Ran {}/{} relevant tests ({:.1}% reduction)\n\n{}",
                        selection.selected_tests.len(),
                        selection.all_tests.len(),
                        reduction,
                        output
                    ),
                ))
            }
        } else {
            // M1: Run all tests (no smart selection)
            let command = custom_command.unwrap_or("pytest -v");
            let output = self.run_command(command, &workspace)?;

            Ok(ToolResult::success(
                "run_tests",
                format!("Test results:\n\n{}", output),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_name() {
        let tool = RunTestsTool::new();
        assert_eq!(tool.name(), "run_tests");
    }

    #[test]
    fn test_smart_selection_description() {
        let tool = RunTestsTool::with_smart_selection(true);
        assert!(tool.description().contains("intelligently"));
        assert!(tool.description().contains("M2"));
    }

    #[test]
    fn test_basic_description() {
        let tool = RunTestsTool::new();
        assert!(tool.description().contains("all tests"));
        assert!(!tool.description().contains("M2"));
    }

    #[test]
    fn test_parameters_schema() {
        let tool = RunTestsTool::new();
        let params = tool.parameters_schema();

        assert_eq!(params["type"], "object");
        assert!(params["properties"]["workspace_path"].is_object());
        assert!(params["required"].is_array());
    }

    #[tokio::test]
    async fn test_missing_workspace_path() {
        let tool = RunTestsTool::new();
        let args = HashMap::new();

        let result = tool.execute(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_nonexistent_workspace() {
        let tool = RunTestsTool::new();
        let mut args = HashMap::new();
        args.insert("workspace_path".to_string(), json!("/nonexistent/path"));

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("not found"));
    }
}
