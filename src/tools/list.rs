/// List tool - lists files in a directory

use super::{Tool, ToolResult};
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ListTool;

impl ListTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for ListTool {
    fn name(&self) -> &str {
        "list"
    }

    fn description(&self) -> &str {
        "List files and directories in a given path. Returns a list of entries with metadata."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the directory to list (defaults to current directory)"
                },
                "show_hidden": {
                    "type": "boolean",
                    "description": "Include hidden files (starting with .)"
                }
            }
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        let show_hidden = args
            .get("show_hidden")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            return Ok(ToolResult::error(
                self.name(),
                format!("Path does not exist: {}", path),
            ));
        }

        if !path_buf.is_dir() {
            return Ok(ToolResult::error(
                self.name(),
                format!("Path is not a directory: {}", path),
            ));
        }

        match tokio::fs::read_dir(&path_buf).await {
            Ok(mut entries) => {
                let mut items = Vec::new();

                while let Ok(Some(entry)) = entries.next_entry().await {
                    let file_name = entry.file_name();
                    let name = file_name.to_string_lossy().to_string();

                    // Skip hidden files if not requested
                    if !show_hidden && name.starts_with('.') {
                        continue;
                    }

                    let metadata = entry.metadata().await;
                    let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                    let size = metadata
                        .as_ref()
                        .map(|m| m.len())
                        .unwrap_or(0);

                    let entry_type = if is_dir { "dir" } else { "file" };
                    items.push(format!("{:8} {:>10} {}", entry_type, size, name));
                }

                items.sort();

                if items.is_empty() {
                    Ok(ToolResult::success(
                        self.name(),
                        format!("Directory is empty: {}", path),
                    ))
                } else {
                    let output = format!(
                        "Listing {} ({} entries):\n{}",
                        path,
                        items.len(),
                        items.join("\n")
                    );
                    Ok(ToolResult::success(self.name(), output))
                }
            }
            Err(e) => Ok(ToolResult::error(
                self.name(),
                format!("Failed to read directory: {}", e),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_list_tool_success() {
        let temp_dir = TempDir::new().unwrap();
        let test_file1 = temp_dir.path().join("file1.txt");
        let test_file2 = temp_dir.path().join("file2.txt");
        tokio::fs::write(&test_file1, "content1").await.unwrap();
        tokio::fs::write(&test_file2, "content2").await.unwrap();

        let tool = ListTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("file1.txt"));
        assert!(result.output.contains("file2.txt"));
        assert!(result.output.contains("2 entries"));
    }

    #[tokio::test]
    async fn test_list_tool_nonexistent_path() {
        let tool = ListTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String("/nonexistent/path".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("does not exist"));
    }

    #[tokio::test]
    async fn test_list_tool_empty_directory() {
        let temp_dir = TempDir::new().unwrap();

        let tool = ListTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("empty"));
    }

    #[tokio::test]
    async fn test_list_tool_hidden_files() {
        let temp_dir = TempDir::new().unwrap();
        let visible_file = temp_dir.path().join("visible.txt");
        let hidden_file = temp_dir.path().join(".hidden");
        tokio::fs::write(&visible_file, "visible").await.unwrap();
        tokio::fs::write(&hidden_file, "hidden").await.unwrap();

        let tool = ListTool::new();

        // Test without show_hidden
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(temp_dir.path().to_string_lossy().to_string()),
        );

        let result = tool.execute(args.clone()).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("visible.txt"));
        assert!(!result.output.contains(".hidden"));

        // Test with show_hidden
        args.insert(
            "show_hidden".to_string(),
            serde_json::Value::Bool(true),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("visible.txt"));
        assert!(result.output.contains(".hidden"));
    }

    #[test]
    fn test_list_tool_schema() {
        let tool = ListTool::new();
        assert_eq!(tool.name(), "list");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["show_hidden"].is_object());
    }
}
