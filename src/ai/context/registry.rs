/// Parser registry for automatic language detection and parser selection
use crate::ai::context::{
    AstParser, FileContext, Language,
    extractors::{JavaScriptParser, PythonParser, TypeScriptParser},
};
use anyhow::{Context, Result, anyhow};
use std::path::Path;
use std::sync::Arc;

/// Registry for managing AST parsers across multiple languages
///
/// Provides automatic parser selection based on file extension and
/// manages parser instances for reuse.
pub struct ExtractorRegistry {
    python_parser: Arc<PythonParser>,
    javascript_parser: Arc<JavaScriptParser>,
    typescript_parser: Arc<TypeScriptParser>,
}

impl ExtractorRegistry {
    /// Create a new extractor registry with all supported parsers
    pub fn new() -> Result<Self> {
        Ok(Self {
            python_parser: Arc::new(PythonParser::new()?),
            javascript_parser: Arc::new(JavaScriptParser::new()?),
            typescript_parser: Arc::new(TypeScriptParser::new()?),
        })
    }

    /// Get the appropriate parser for a file based on its extension
    ///
    /// Returns an error if the file extension is not supported.
    pub fn get_parser(&self, path: &Path) -> Result<Arc<dyn AstParser>> {
        let language = Language::from_extension(
            path.extension()
                .and_then(|ext| ext.to_str())
                .ok_or_else(|| anyhow!("File has no extension: {}", path.display()))?
        )
        .ok_or_else(|| anyhow!("Unsupported file extension: {}", path.display()))?;

        match language {
            Language::Python => Ok(self.python_parser.clone() as Arc<dyn AstParser>),
            Language::JavaScript => Ok(self.javascript_parser.clone() as Arc<dyn AstParser>),
            Language::TypeScript => Ok(self.typescript_parser.clone() as Arc<dyn AstParser>),
            Language::Rust => Err(anyhow!("Rust parser not yet implemented")),
        }
    }

    /// Parse a file automatically detecting the appropriate parser
    ///
    /// This is a convenience method that combines parser selection and parsing.
    pub async fn parse_file(&self, path: &Path) -> Result<FileContext> {
        let parser = self.get_parser(path)
            .with_context(|| format!("Failed to get parser for {}", path.display()))?;

        parser.parse_file(path)
            .await
            .with_context(|| format!("Failed to parse file {}", path.display()))
    }

    /// Parse multiple files concurrently
    ///
    /// Returns a Vec of Results to allow partial success - some files may fail
    /// while others succeed.
    pub async fn parse_files(&self, paths: &[impl AsRef<Path>]) -> Vec<Result<FileContext>> {
        let tasks: Vec<_> = paths
            .iter()
            .map(|path| {
                let path = path.as_ref().to_path_buf();
                let registry = self.clone();
                tokio::spawn(async move {
                    registry.parse_file(&path).await
                })
            })
            .collect();

        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(anyhow!("Task panicked: {}", e))),
            }
        }

        results
    }

    /// Get all supported file extensions
    pub fn supported_extensions(&self) -> Vec<&str> {
        vec![
            // Python
            "py", "pyw",
            // JavaScript
            "js", "jsx", "mjs", "cjs",
            // TypeScript
            "ts", "tsx",
        ]
    }

    /// Check if a file extension is supported
    pub fn is_supported(&self, extension: &str) -> bool {
        Language::from_extension(extension).is_some()
    }
}

impl Clone for ExtractorRegistry {
    fn clone(&self) -> Self {
        Self {
            python_parser: self.python_parser.clone(),
            javascript_parser: self.javascript_parser.clone(),
            typescript_parser: self.typescript_parser.clone(),
        }
    }
}

impl Default for ExtractorRegistry {
    fn default() -> Self {
        Self::new().expect("Failed to create ExtractorRegistry")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_registry_creation() {
        let registry = ExtractorRegistry::new();
        assert!(registry.is_ok());
    }

    #[test]
    fn test_get_parser_python() {
        let registry = ExtractorRegistry::new().unwrap();
        let path = Path::new("test.py");
        let parser = registry.get_parser(path);
        assert!(parser.is_ok());
        assert_eq!(parser.unwrap().language(), Language::Python);
    }

    #[test]
    fn test_get_parser_javascript() {
        let registry = ExtractorRegistry::new().unwrap();
        let path = Path::new("test.js");
        let parser = registry.get_parser(path);
        assert!(parser.is_ok());
        assert_eq!(parser.unwrap().language(), Language::JavaScript);
    }

    #[test]
    fn test_get_parser_typescript() {
        let registry = ExtractorRegistry::new().unwrap();
        let path = Path::new("test.ts");
        let parser = registry.get_parser(path);
        assert!(parser.is_ok());
        assert_eq!(parser.unwrap().language(), Language::TypeScript);
    }

    #[test]
    fn test_get_parser_unsupported() {
        let registry = ExtractorRegistry::new().unwrap();
        let path = Path::new("test.txt");
        let parser = registry.get_parser(path);
        assert!(parser.is_err());
    }

    #[test]
    fn test_supported_extensions() {
        let registry = ExtractorRegistry::new().unwrap();
        let extensions = registry.supported_extensions();

        assert!(extensions.contains(&"py"));
        assert!(extensions.contains(&"js"));
        assert!(extensions.contains(&"ts"));
        assert!(extensions.contains(&"tsx"));
    }

    #[test]
    fn test_is_supported() {
        let registry = ExtractorRegistry::new().unwrap();

        assert!(registry.is_supported("py"));
        assert!(registry.is_supported("js"));
        assert!(registry.is_supported("ts"));
        assert!(!registry.is_supported("txt"));
        assert!(!registry.is_supported("md"));
    }

    #[tokio::test]
    async fn test_parse_file_python() {
        let code = r#"
def hello():
    print("Hello")
"#;

        let mut temp_file = NamedTempFile::with_suffix(".py").unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let registry = ExtractorRegistry::new().unwrap();
        let context = registry.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.language, Language::Python);
        assert!(!context.symbols.is_empty());
    }

    #[tokio::test]
    async fn test_parse_file_javascript() {
        let code = r#"
function greet() {
    console.log("Hello");
}
"#;

        let mut temp_file = NamedTempFile::with_suffix(".js").unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let registry = ExtractorRegistry::new().unwrap();
        let context = registry.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.language, Language::JavaScript);
        assert!(!context.symbols.is_empty());
    }

    #[tokio::test]
    async fn test_parse_file_typescript() {
        let code = r#"
function add(a: number, b: number): number {
    return a + b;
}
"#;

        let mut temp_file = NamedTempFile::with_suffix(".ts").unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let registry = ExtractorRegistry::new().unwrap();
        let context = registry.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.language, Language::TypeScript);
        assert!(!context.symbols.is_empty());
    }

    #[tokio::test]
    async fn test_parse_multiple_files() {
        // Create temp files for different languages
        let py_code = "def test(): pass";
        let mut py_file = NamedTempFile::with_suffix(".py").unwrap();
        py_file.write_all(py_code.as_bytes()).unwrap();

        let js_code = "function test() {}";
        let mut js_file = NamedTempFile::with_suffix(".js").unwrap();
        js_file.write_all(js_code.as_bytes()).unwrap();

        let ts_code = "function test(): void {}";
        let mut ts_file = NamedTempFile::with_suffix(".ts").unwrap();
        ts_file.write_all(ts_code.as_bytes()).unwrap();

        let paths = vec![
            py_file.path().to_path_buf(),
            js_file.path().to_path_buf(),
            ts_file.path().to_path_buf(),
        ];

        let registry = ExtractorRegistry::new().unwrap();
        let results = registry.parse_files(&paths).await;

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));

        // Verify each language was parsed correctly
        let contexts: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
        assert_eq!(contexts.len(), 3);

        let languages: Vec<_> = contexts.iter().map(|c| c.language).collect();
        assert!(languages.contains(&Language::Python));
        assert!(languages.contains(&Language::JavaScript));
        assert!(languages.contains(&Language::TypeScript));
    }
}
