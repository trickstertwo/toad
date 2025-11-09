/// Grep tool - searches for patterns in files
use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct GrepTool;

impl Default for GrepTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GrepTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for a pattern in a file. Returns matching lines with line numbers."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to search"
                },
                "pattern": {
                    "type": "string",
                    "description": "Pattern to search for (literal string, not regex)"
                },
                "context_lines": {
                    "type": "number",
                    "description": "Number of context lines before and after matches (default: 0)"
                },
                "case_insensitive": {
                    "type": "boolean",
                    "description": "Perform case-insensitive search (default: false)"
                }
            },
            "required": ["path", "pattern"]
        })
    }

    async fn execute(&self, args: HashMap<String, serde_json::Value>) -> Result<ToolResult> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .context("Missing 'path' argument")?;

        let pattern = args
            .get("pattern")
            .and_then(|v| v.as_str())
            .context("Missing 'pattern' argument")?;

        let context_lines = args
            .get("context_lines")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize;

        let case_insensitive = args
            .get("case_insensitive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

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

        let lines: Vec<&str> = content.lines().collect();
        let mut matches: Vec<(usize, String)> = Vec::new();

        // Search for pattern (case sensitive or insensitive)
        let search_pattern = if case_insensitive {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        for (i, line) in lines.iter().enumerate() {
            let search_line = if case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if search_line.contains(&search_pattern) {
                matches.push((i, line.to_string()));
            }
        }

        if matches.is_empty() {
            return Ok(ToolResult::success(
                self.name(),
                format!("No matches found for '{}' in {}", pattern, path),
            ));
        }

        // Format output with context
        let mut output = String::new();
        output.push_str(&format!(
            "Found {} match(es) for '{}' in {}:\n\n",
            matches.len(),
            pattern,
            path
        ));

        for (line_num, _) in &matches {
            let start = if *line_num > context_lines {
                line_num - context_lines
            } else {
                0
            };
            let end = std::cmp::min(line_num + context_lines + 1, lines.len());

            for (idx, line) in lines.iter().enumerate().take(end).skip(start) {
                let prefix = if idx == *line_num { ">" } else { " " };
                output.push_str(&format!("{} {:4} | {}\n", prefix, idx + 1, line));
            }

            if matches.len() > 1 {
                output.push_str("--\n");
            }
        }

        Ok(ToolResult::success(self.name(), output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_grep_tool_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "line 1\nHello World\nline 3")
            .await
            .unwrap();

        let tool = GrepTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "pattern".to_string(),
            serde_json::Value::String("Hello".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("1 match"));
        assert!(result.output.contains("Hello World"));
        assert!(result.output.contains("2 |")); // Line number
    }

    #[tokio::test]
    async fn test_grep_tool_multiple_matches() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "test 1\ntest 2\nother\ntest 3")
            .await
            .unwrap();

        let tool = GrepTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "pattern".to_string(),
            serde_json::Value::String("test".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("3 match"));
        assert!(result.output.contains("test 1"));
        assert!(result.output.contains("test 2"));
        assert!(result.output.contains("test 3"));
    }

    #[tokio::test]
    async fn test_grep_tool_with_context() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "line 1\nline 2\ntarget\nline 4\nline 5")
            .await
            .unwrap();

        let tool = GrepTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "pattern".to_string(),
            serde_json::Value::String("target".to_string()),
        );
        args.insert(
            "context_lines".to_string(),
            serde_json::Value::Number(1.into()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("line 2")); // Context before
        assert!(result.output.contains("target"));
        assert!(result.output.contains("line 4")); // Context after
    }

    #[tokio::test]
    async fn test_grep_tool_case_insensitive() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Hello World\nhello again")
            .await
            .unwrap();

        let tool = GrepTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "pattern".to_string(),
            serde_json::Value::String("HELLO".to_string()),
        );
        args.insert(
            "case_insensitive".to_string(),
            serde_json::Value::Bool(true),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("2 match"));
        assert!(result.output.contains("Hello World"));
        assert!(result.output.contains("hello again"));
    }

    #[tokio::test]
    async fn test_grep_tool_no_matches() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "line 1\nline 2")
            .await
            .unwrap();

        let tool = GrepTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "pattern".to_string(),
            serde_json::Value::String("notfound".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("No matches found"));
    }

    #[tokio::test]
    async fn test_grep_tool_missing_file() {
        let tool = GrepTool::new();
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String("/nonexistent/file.txt".to_string()),
        );
        args.insert(
            "pattern".to_string(),
            serde_json::Value::String("test".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("does not exist"));
    }

    #[test]
    fn test_grep_tool_schema() {
        let tool = GrepTool::new();
        assert_eq!(tool.name(), "grep");
        assert!(!tool.description().is_empty());

        let schema = tool.parameters_schema();
        assert!(schema["properties"]["path"].is_object());
        assert!(schema["properties"]["pattern"].is_object());
        assert!(schema["properties"]["context_lines"].is_object());
        assert!(schema["properties"]["case_insensitive"].is_object());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
