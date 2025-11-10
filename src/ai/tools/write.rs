/// Write tool - writes content to a file with optional syntax validation
use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct WriteTool {
    /// Whether to validate syntax before writing
    validate_syntax: bool,
}

impl Default for WriteTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteTool {
    /// Create a new WriteTool without syntax validation (backwards compatible)
    pub fn new() -> Self {
        Self {
            validate_syntax: false,
        }
    }

    /// Create a WriteTool with optional syntax validation
    pub fn with_validation(validate_syntax: bool) -> Self {
        Self { validate_syntax }
    }

    /// Validate syntax using tree-sitter for supported languages
    fn validate_syntax(path: &Path, content: &str) -> Result<(), String> {
        use crate::ai::context::Language;
        use crate::ai::context::parser::detect_language;

        // Detect language from file extension
        let language = match detect_language(path) {
            Some(lang) => lang,
            None => {
                // If we don't recognize the language, skip validation
                return Ok(());
            }
        };

        // Use tree-sitter to validate syntax
        let mut parser = tree_sitter::Parser::new();

        // Set language based on file type
        let tree_sitter_lang = match language {
            Language::Python => {
                parser
                    .set_language(&tree_sitter_python::LANGUAGE.into())
                    .map_err(|e| format!("Failed to set Python language: {}", e))?;
                "Python"
            }
            Language::JavaScript => {
                parser
                    .set_language(&tree_sitter_javascript::LANGUAGE.into())
                    .map_err(|e| format!("Failed to set JavaScript language: {}", e))?;
                "JavaScript"
            }
            Language::TypeScript => {
                parser
                    .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
                    .map_err(|e| format!("Failed to set TypeScript language: {}", e))?;
                "TypeScript"
            }
            Language::Rust => {
                // Rust parser not implemented yet, skip validation
                return Ok(());
            }
        };

        // Try to parse the content
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| format!("Failed to parse {} file", tree_sitter_lang))?;

        // Check for syntax errors (missing nodes indicate parse errors)
        let root = tree.root_node();
        if root.has_error() {
            return Err(format!("Syntax error in {} file", tree_sitter_lang));
        }

        Ok(())
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

        // Validate syntax if enabled
        if self.validate_syntax
            && let Err(e) = Self::validate_syntax(&path_buf, content)
        {
            return Ok(ToolResult::error(
                self.name(),
                format!("Validation failed: {}", e),
            ));
        }

        // Create parent directories if they don't exist
        if let Some(parent) = path_buf.parent()
            && !parent.exists()
            && let Err(e) = tokio::fs::create_dir_all(parent).await
        {
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

    // === Tree-sitter Validation Tests ===

    #[tokio::test]
    async fn test_write_tool_validation_valid_python() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("valid.py");

        let tool = WriteTool::with_validation(true);
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String("def hello():\n    print('world')\n".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success, "Valid Python code should pass validation");
        assert!(file_path.exists());
    }

    #[tokio::test]
    async fn test_write_tool_validation_invalid_python() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.py");

        let tool = WriteTool::with_validation(true);
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String(
                "def hello(\n    print('missing closing paren'\n".to_string(),
            ),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(
            !result.success,
            "Invalid Python code should fail validation"
        );
        assert!(result.error.unwrap().contains("Syntax error"));
        assert!(
            !file_path.exists(),
            "File should not be written on validation failure"
        );
    }

    #[tokio::test]
    async fn test_write_tool_validation_valid_javascript() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("valid.js");

        let tool = WriteTool::with_validation(true);
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String(
                "function hello() {\n  console.log('world');\n}\n".to_string(),
            ),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(
            result.success,
            "Valid JavaScript code should pass validation"
        );
        assert!(file_path.exists());
    }

    #[tokio::test]
    async fn test_write_tool_validation_invalid_javascript() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.js");

        let tool = WriteTool::with_validation(true);
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String(
                "function hello() {\n  console.log('missing brace'\n".to_string(),
            ),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(
            !result.success,
            "Invalid JavaScript code should fail validation"
        );
        assert!(result.error.unwrap().contains("Syntax error"));
        assert!(
            !file_path.exists(),
            "File should not be written on validation failure"
        );
    }

    #[tokio::test]
    async fn test_write_tool_validation_unknown_extension() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("file.unknown");

        let tool = WriteTool::with_validation(true);
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String("any content here, no validation".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(result.success, "Unknown file types should skip validation");
        assert!(file_path.exists());
    }

    #[tokio::test]
    async fn test_write_tool_validation_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("invalid.py");

        // Validation disabled
        let tool = WriteTool::with_validation(false);
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            serde_json::Value::String(file_path.to_string_lossy().to_string()),
        );
        args.insert(
            "content".to_string(),
            serde_json::Value::String("def hello(\n    invalid python\n".to_string()),
        );

        let result = tool.execute(args).await.unwrap();
        assert!(
            result.success,
            "Invalid code should be written when validation is disabled"
        );
        assert!(file_path.exists());
    }
}
