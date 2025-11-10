/// Python AST extractor using tree-sitter
use crate::ai::context::{AstParser, FileContext, Import, Language, Symbol, SymbolKind};
use anyhow::{Context as _, Result};
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Parser, Query, QueryCursor};

/// Python AST parser
pub struct PythonParser {
    parser: Parser,
    function_query: Query,
    class_query: Query,
    import_query: Query,
}

impl PythonParser {
    /// Create a new Python parser
    pub fn new() -> Result<Self> {
        let language = tree_sitter_python::LANGUAGE;
        let mut parser = Parser::new();
        parser
            .set_language(&language.into())
            .context("Failed to set Python language")?;

        // Tree-sitter query for function definitions
        let function_query = Query::new(
            &language.into(),
            r#"
            (function_definition
                name: (identifier) @name
                parameters: (parameters) @params
                body: (block
                    (expression_statement
                        (string) @docstring)?))
            "#,
        )
        .context("Failed to create function query")?;

        // Tree-sitter query for class definitions
        let class_query = Query::new(
            &language.into(),
            r#"
            (class_definition
                name: (identifier) @name
                body: (block
                    (expression_statement
                        (string) @docstring)?))
            "#,
        )
        .context("Failed to create class query")?;

        // Tree-sitter query for import statements
        let import_query = Query::new(
            &language.into(),
            r#"
            [
                (import_statement
                    name: (dotted_name) @module)
                (import_from_statement
                    module_name: (dotted_name) @module)
            ]
            "#,
        )
        .context("Failed to create import query")?;

        Ok(Self {
            parser,
            function_query,
            class_query,
            import_query,
        })
    }

    /// Extract function symbols from AST
    fn extract_functions(&self, source: &str, root: Node) -> Vec<Symbol> {
        let mut cursor = QueryCursor::new();
        let mut symbols = Vec::new();
        let mut matches = cursor.matches(&self.function_query, root, source.as_bytes());

        while let Some(match_) = matches.next() {
            let mut name = None;
            let mut params = None;
            let mut docstring = None;

            for capture in match_.captures {
                let capture_name = self.function_query.capture_names()[capture.index as usize];
                let text = &source[capture.node.byte_range()];

                match capture_name {
                    "name" => name = Some(text.to_string()),
                    "params" => params = Some(text.to_string()),
                    "docstring" => {
                        // Remove quotes from docstring
                        let doc = text.trim().trim_matches(|c| c == '"' || c == '\'');
                        docstring = Some(doc.to_string());
                    }
                    _ => {}
                }
            }

            if let Some(func_name) = name {
                let start_line = match_.captures[0].node.start_position().row + 1;
                let end_line = match_.captures[0].node.end_position().row + 1;

                let signature = params.map(|p| format!("def {}{})", func_name, p));

                let mut symbol =
                    Symbol::new(func_name, SymbolKind::Function, (start_line, end_line));
                if let Some(sig) = signature {
                    symbol = symbol.with_signature(sig);
                }
                if let Some(doc) = docstring {
                    symbol = symbol.with_docstring(doc);
                }

                symbols.push(symbol);
            }
        }

        symbols
    }

    /// Extract class symbols from AST
    fn extract_classes(&self, source: &str, root: Node) -> Vec<Symbol> {
        let mut cursor = QueryCursor::new();
        let mut symbols = Vec::new();
        let mut matches = cursor.matches(&self.class_query, root, source.as_bytes());

        while let Some(match_) = matches.next() {
            let mut name = None;
            let mut docstring = None;

            for capture in match_.captures {
                let capture_name = self.class_query.capture_names()[capture.index as usize];
                let text = &source[capture.node.byte_range()];

                match capture_name {
                    "name" => name = Some(text.to_string()),
                    "docstring" => {
                        let doc = text.trim().trim_matches(|c| c == '"' || c == '\'');
                        docstring = Some(doc.to_string());
                    }
                    _ => {}
                }
            }

            if let Some(class_name) = name {
                let start_line = match_.captures[0].node.start_position().row + 1;
                let end_line = match_.captures[0].node.end_position().row + 1;

                let mut symbol = Symbol::new(class_name, SymbolKind::Class, (start_line, end_line));
                if let Some(doc) = docstring {
                    symbol = symbol.with_docstring(doc);
                }

                symbols.push(symbol);
            }
        }

        symbols
    }

    /// Extract import statements from AST
    fn extract_imports(&self, source: &str, root: Node) -> Vec<Import> {
        let mut cursor = QueryCursor::new();
        let mut imports = Vec::new();
        let mut matches = cursor.matches(&self.import_query, root, source.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let capture_name = self.import_query.capture_names()[capture.index as usize];

                if capture_name == "module" {
                    let text = &source[capture.node.byte_range()];
                    let line = capture.node.start_position().row + 1;
                    imports.push(Import::new(text, line));
                }
            }
        }

        imports
    }
}

impl Default for PythonParser {
    fn default() -> Self {
        Self::new().expect("Failed to create Python parser")
    }
}

#[async_trait::async_trait]
impl AstParser for PythonParser {
    fn language(&self) -> Language {
        Language::Python
    }

    async fn parse_file(&self, path: &Path) -> Result<FileContext> {
        // Read file contents
        let source = tokio::fs::read_to_string(path)
            .await
            .context("Failed to read file")?;

        // Parse in blocking thread (tree-sitter is CPU-bound)
        let path_clone = path.to_path_buf();
        let source_clone = source.clone();

        let context = tokio::task::spawn_blocking(move || {
            let mut parser = Parser::new();
            parser
                .set_language(&tree_sitter_python::LANGUAGE.into())
                .context("Failed to set language")?;

            let tree = parser
                .parse(&source_clone, None)
                .context("Failed to parse Python file")?;

            let mut file_context = FileContext::new(path_clone, Language::Python);

            // Extract symbols
            let parser_inst = PythonParser::new()?;
            let root = tree.root_node();

            for func in parser_inst.extract_functions(&source_clone, root) {
                file_context.add_symbol(func);
            }

            for class in parser_inst.extract_classes(&source_clone, root) {
                file_context.add_symbol(class);
            }

            for import in parser_inst.extract_imports(&source_clone, root) {
                file_context.add_import(import);
            }

            Ok::<FileContext, anyhow::Error>(file_context)
        })
        .await??;

        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_parse_simple_function() {
        let code = r#"
def hello_world():
    """Say hello"""
    print("Hello, World!")
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = PythonParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.symbols.len(), 1);
        assert_eq!(context.symbols[0].name, "hello_world");
        assert_eq!(context.symbols[0].kind, SymbolKind::Function);
        assert!(
            context.symbols[0]
                .docstring
                .as_ref()
                .unwrap()
                .contains("Say hello")
        );
    }

    #[tokio::test]
    async fn test_parse_class() {
        let code = r#"
class MyClass:
    """A test class"""
    pass
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = PythonParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.symbols.len(), 1);
        assert_eq!(context.symbols[0].name, "MyClass");
        assert_eq!(context.symbols[0].kind, SymbolKind::Class);
    }

    #[tokio::test]
    async fn test_parse_imports() {
        let code = r#"
import os
import sys
from typing import List, Dict
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = PythonParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.imports.len(), 3);
        assert!(context.imports.iter().any(|i| i.module == "os"));
        assert!(context.imports.iter().any(|i| i.module == "sys"));
        assert!(context.imports.iter().any(|i| i.module == "typing"));
    }

    #[tokio::test]
    async fn test_parse_function_with_params() {
        let code = r#"
def add(x, y):
    return x + y
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = PythonParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.symbols.len(), 1);
        assert_eq!(context.symbols[0].name, "add");
        assert!(context.symbols[0].signature.is_some());
    }

    #[tokio::test]
    async fn test_parse_multiple_functions_and_classes() {
        let code = r#"
import os

class MyClass:
    def __init__(self):
        pass

    def method(self):
        pass

def standalone_function():
    pass
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = PythonParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        // Should find 1 class and 3 functions (__init__, method, standalone_function)
        let classes = context.symbols_of_kind(SymbolKind::Class);
        let functions = context.symbols_of_kind(SymbolKind::Function);

        assert_eq!(classes.len(), 1);
        assert!(functions.len() >= 1); // At least standalone_function
        assert_eq!(context.imports.len(), 1);
    }
}
