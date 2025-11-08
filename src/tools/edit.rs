/// Edit tool - performs search/replace operations on files

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct EditTool;

impl EditTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit"
    }

    fn description(&self) -> &str {
        "Edit a file by replacing a search string with replacement text. Returns the number of replacements made."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to edit"
                },
                "search": {
                    "type": "string",
                    "description": "Text to search for"
                },
                "replace": {
                    "type": "string",
                    "description": "Text to replace with"
                },
                "all": {
                    "type": "boolean",
                    "description": "Replace all occurrences (default: true)"
                }
            },
            "required": ["path", "search", "replace"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .context("Missing 'path' argument")?;

        let search = args
            .get("search")
            .and_then(|v| v.as_str())
            .context("Missing 'search' argument")?;

        let replace = args
            .get("replace")
            .and_then(|v| v.as_str())
            .context("Missing 'replace' argument")?;

        let replace_all = args
            .get("all")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            return Ok(ToolResult::error(
                self.name(),
                format!("File does not exist: {}", path),
            ));
        }

        // Read the file
        let content = match tokio::fs::read_to_string(&path_buf).await {
            Ok(c) => c,
            Err(e) => {
                return Ok(ToolResult::error(
                    self.name(),
                    format!("Failed to read file: {}", e),
                ))
            }
        };

        // Perform replacement
        let (new_content, count) = if replace_all {
            let count = content.matches(search).count();
            (content.replace(search, replace), count)
        } else {
            // Replace only first occurrence
            if let Some(pos) = content.find(search) {
                let mut new_content = String::new();
                new_content.push_str(&content[..pos]);
                new_content.push_str(replace);
                new_content.push_str(&content[pos + search.len()..]);
                (new_content, 1)
            } else {
                (content.clone(), 0)
            }
        };

        if count == 0 {
            return Ok(ToolResult::success(
                self.name(),
                format!("Search string not found in {}", path),
            ));
        }

        // Write the modified content back
        match tokio::fs::write(&path_buf, new_content).await {
            Ok(_) => Ok(ToolResult::success(
                self.name(),
                format!("Made {} replacement(s) in {}", count, path),
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
    async fn test_edit_tool_single_replacement() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Hello world\nHello again")
            .await
            .unwrap();

        let tool = EditTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "search".to_string(),
            serde_json::Value::String("Hello".to_string()),
        );
        args.insert(
            "replace".to_string(),
            serde_json::Value::String("Hi".to_string()),
        );
        args.insert("all".to_string(), serde_json::Value::Bool(false));

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("1 replacement"));

        // Verify file was modified correctly (only first occurrence)
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Hi world\nHello again");
    }

    #[tokio::test]
    async fn test_edit_tool_all_replacements() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Hello world\nHello again\nHello!")
            .await
            .unwrap();

        let tool = EditTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "search".to_string(),
            serde_json::Value::String("Hello".to_string()),
        );
        args.insert(
            "replace".to_string(),
            serde_json::Value::String("Hi".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("3 replacement"));

        // Verify all occurrences were replaced
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Hi world\nHi again\nHi!");
    }

    #[tokio::test]
    async fn test_edit_tool_search_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Hello world").await.unwrap();

        let tool = EditTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "search".to_string(),
            serde_json::Value::String("Goodbye".to_string()),
        );
        args.insert(
            "replace".to_string(),
            serde_json::Value::String("Hi".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("not found"));

        // Verify file was not modified
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Hello world");
    }

    #[tokio::test]
    async fn test_edit_tool_missing_file() {
        let tool = EditTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String("/nonexistent/file.txt".to_string()),
        );
        args.insert(
            "search".to_string(),
            serde_json::Value::String("test".to_string()),
        );
        args.insert(
            "replace".to_string(),
            serde_json::Value::String("replacement".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("does not exist"));
    }

    #[tokio::test]
    async fn test_edit_tool_missing_args() {
        let tool = EditTool::new();
        let args = HashMap::new();

        let result = tool.execute(args).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_edit_tool_schema() {
        let tool = EditTool::new();
        assert_eq!(tool.name(), "edit");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["search"].is_object());
        assert!(schema["properties"]["replace"].is_object());
        assert!(schema["properties"]["all"].is_object());
        assert_eq!(schema["required"].as_array().unwrap().len(), 3);
    }
}
