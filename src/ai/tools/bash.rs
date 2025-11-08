/// Bash tool - executes shell commands
use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

pub struct BashTool;

impl BashTool {
    pub fn new() -> Self {
        Self
    }

    /// Default timeout for command execution (30 seconds)
    const DEFAULT_TIMEOUT_SECS: u64 = 30;
}

#[async_trait::async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute a shell command and return the output. Uses sh on Unix/Linux/macOS, cmd on Windows. Returns stdout, stderr, and exit code."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The bash command to execute"
                },
                "timeout_secs": {
                    "type": "number",
                    "description": "Timeout in seconds (default: 30)"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .context("Missing 'command' argument")?;

        let timeout_secs = args
            .get("timeout_secs")
            .and_then(|v| v.as_u64())
            .unwrap_or(Self::DEFAULT_TIMEOUT_SECS);

        // Execute command with timeout (cross-platform)
        let execution = async {
            #[cfg(target_os = "windows")]
            let mut cmd = {
                let mut c = Command::new("cmd");
                c.args(&["/C", command]);
                c
            };

            #[cfg(not(target_os = "windows"))]
            let mut cmd = {
                let mut c = Command::new("sh");
                c.args(&["-c", command]);
                c
            };

            cmd.stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
        };

        let output = match timeout(Duration::from_secs(timeout_secs), execution).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Ok(ToolResult::error(
                    self.name(),
                    format!("Command execution failed: {}", e),
                ));
            }
            Err(_) => {
                return Ok(ToolResult::error(
                    self.name(),
                    format!("Command timed out after {} seconds", timeout_secs),
                ));
            }
        };

        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // Format output
        let mut result_output = String::new();

        if !stdout.is_empty() {
            result_output.push_str(&format!("stdout:\n{}\n", stdout));
        }

        if !stderr.is_empty() {
            result_output.push_str(&format!("stderr:\n{}\n", stderr));
        }

        if result_output.is_empty() {
            result_output.push_str("(no output)");
        }

        result_output.push_str(&format!("exit_code: {}", exit_code));

        // Success if exit code is 0
        if exit_code == 0 {
            Ok(ToolResult {
                tool: self.name().to_string(),
                output: result_output,
                success: true,
                error: None,
                exit_code: Some(exit_code),
            })
        } else {
            Ok(ToolResult {
                tool: self.name().to_string(),
                output: result_output,
                success: false,
                error: Some(format!("Command exited with code {}", exit_code)),
                exit_code: Some(exit_code),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bash_tool_success() {
        let tool = BashTool::new();
        let mut args = HashMap::new();
        args.insert(
            "command".to_string(),
            serde_json::Value::String("echo 'Hello World'".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Hello World"));
        assert_eq!(result.exit_code, Some(0));
    }

    #[tokio::test]
    async fn test_bash_tool_failure() {
        let tool = BashTool::new();
        let mut args = HashMap::new();
        args.insert(
            "command".to_string(),
            serde_json::Value::String("exit 1".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert_eq!(result.exit_code, Some(1));
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_bash_tool_stderr() {
        let tool = BashTool::new();
        let mut args = HashMap::new();
        args.insert(
            "command".to_string(),
            serde_json::Value::String("echo 'error message' >&2".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success); // Exit code 0, so success
        assert!(result.output.contains("stderr"));
        assert!(result.output.contains("error message"));
    }

    #[tokio::test]
    async fn test_bash_tool_timeout() {
        let tool = BashTool::new();
        let mut args = HashMap::new();
        args.insert(
            "command".to_string(),
            serde_json::Value::String("sleep 5".to_string()),
        );
        args.insert(
            "timeout_secs".to_string(),
            serde_json::Value::Number(1.into()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("timed out"));
    }

    #[tokio::test]
    async fn test_bash_tool_missing_command() {
        let tool = BashTool::new();
        let args = HashMap::new();

        let result = tool.execute(args).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_bash_tool_schema() {
        let tool = BashTool::new();
        assert_eq!(tool.name(), "bash");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["command"].is_object());
        assert!(schema["properties"]["timeout_secs"].is_object());
        assert_eq!(schema["required"].as_array().unwrap().len(), 1);
    }
}
