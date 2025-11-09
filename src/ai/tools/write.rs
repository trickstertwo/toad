/// Write tool - writes content to a file
use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct WriteTool;

impl Default for WriteTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write"
    }

    fn description(&self) -> &str {
        "Write content to a file. Creates the file if it doesn't exist, overwrites if it does."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .context("Missing 'path' argument")?;

        let content = args
            .get("content")
            .and_then(|v| v.as_str())
            .context("Missing 'content' argument")?;

        let path_buf = PathBuf::from(path);

        // Create parent directories if they don't exist
        if let Some(parent) = path_buf.parent()
            && !parent.exists()
                && let Err(e) = tokio::fs::create_dir_all(parent).await {
                    return Ok(ToolResult::error(
                        self.name(),
                        format!("Failed to create parent directories: {}", e),
                    ));
                }

        match tokio::fs::write(&path_buf, content).await {
            Ok(_) => Ok(ToolResult::success(
                self.name(),
                format!("Wrote {} bytes to {}", content.len(), path),
            )),
            Err(e) => Ok(ToolResult::error(
                self.name(),
                format!("Failed to write file: {}", e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_write_tool_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let tool = WriteTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String("Hello, world!".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);

        // Verify file was written
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Hello, world!");
    }

    #[tokio::test]
    async fn test_write_tool_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("subdir").join("test.txt");

        let tool = WriteTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String("content".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(file_path.exists());
    }

    #[tokio::test]
    async fn test_write_tool_missing_args() {
        let tool = WriteTool::new();
        let args = HashMap::new();

        let result = tool.execute(args).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_write_tool_schema() {
        let tool = WriteTool::new();
        assert_eq!(tool.name(), "write");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["content"].is_object());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
