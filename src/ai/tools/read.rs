/// Read tool - reads file contents
use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ReadTool;

impl ReadTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read"
    }

    fn description(&self) -> &str {
        "Read the contents of a file. Returns the file contents as a string."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .context("Missing 'path' argument")?;

        let path_buf = PathBuf::from(path);

        match tokio::fs::read_to_string(&path_buf).await {
            Ok(contents) => Ok(ToolResult::success(self.name(), contents)),
            Err(e) => Ok(ToolResult::error(
                self.name(),
                format!("Failed to read file: {}", e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_tool_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "Hello, world!").unwrap();

        let tool = ReadTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_file.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert_eq!(result.output, "Hello, world!");
    }

    #[tokio::test]
    async fn test_read_tool_missing_file() {
        let tool = ReadTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String("/nonexistent/file.txt".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_read_tool_missing_path_arg() {
        let tool = ReadTool::new();
        let args = HashMap::new();

        let result = tool.execute(args).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_read_tool_schema() {
        let tool = ReadTool::new();
        assert_eq!(tool.name(), "read");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert_eq!(schema["required"][0], "path");
    }
}
