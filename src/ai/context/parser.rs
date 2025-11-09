/// Parser trait and language detection for AST extraction
use super::{FileContext, Language};
use anyhow::Result;
use std::path::Path;

/// Trait for AST parsers
///
/// Each language has its own parser implementation that extracts
/// symbols and imports from source code.
#[async_trait::async_trait]
pub trait AstParser: Send + Sync {
    /// Get the language this parser handles
    fn language(&self) -> Language;

    /// Parse a file and extract AST context
    ///
    /// # Arguments
    /// * `path` - Path to the source file
    ///
    /// # Returns
    /// * `FileContext` with extracted symbols and imports
    ///
    /// # Errors
    /// * File I/O errors
    /// * Parse errors (malformed syntax)
    async fn parse_file(&self, path: &Path) -> Result<FileContext>;
}

/// Detect programming language from file path
pub fn detect_language(path: &Path) -> Option<Language> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .and_then(Language::from_extension)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_detect_language_python() {
        assert_eq!(
            detect_language(&PathBuf::from("test.py")),
            Some(Language::Python)
        );
        assert_eq!(
            detect_language(&PathBuf::from("script.pyw")),
            Some(Language::Python)
        );
    }

    #[test]
    fn test_detect_language_javascript() {
        assert_eq!(
            detect_language(&PathBuf::from("app.js")),
            Some(Language::JavaScript)
        );
        assert_eq!(
            detect_language(&PathBuf::from("component.jsx")),
            Some(Language::JavaScript)
        );
    }

    #[test]
    fn test_detect_language_typescript() {
        assert_eq!(
            detect_language(&PathBuf::from("app.ts")),
            Some(Language::TypeScript)
        );
        assert_eq!(
            detect_language(&PathBuf::from("component.tsx")),
            Some(Language::TypeScript)
        );
    }

    #[test]
    fn test_detect_language_rust() {
        assert_eq!(
            detect_language(&PathBuf::from("main.rs")),
            Some(Language::Rust)
        );
    }

    #[test]
    fn test_detect_language_unknown() {
        assert_eq!(detect_language(&PathBuf::from("readme.txt")), None);
        assert_eq!(detect_language(&PathBuf::from("config.json")), None);
    }
}
