/// JavaScript/JSX AST extractor using tree-sitter
use crate::ai::context::{AstParser, FileContext, Import, Language, Symbol, SymbolKind};
use anyhow::{Context as _, Result};
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Parser, Query, QueryCursor};

/// JavaScript AST parser (supports both JS and JSX)
pub struct JavaScriptParser {
    #[allow(dead_code)]
    parser: Parser,
    function_query: Query,
    arrow_function_query: Query,
    class_query: Query,
    import_query: Query,
}

impl JavaScriptParser {
    /// Create a new JavaScript parser
    pub fn new() -> Result<Self> {
        let language = tree_sitter_javascript::LANGUAGE;
        let mut parser = Parser::new();
        parser
            .set_language(&language.into())
            .context("Failed to set JavaScript language")?;

        // Tree-sitter query for function declarations
        let function_query = Query::new(
            &language.into(),
            r#"
            (function_declaration
                name: (identifier) @name
                parameters: (formal_parameters) @params
                body: (statement_block))
            "#,
        )
        .context("Failed to create function query")?;

        // Tree-sitter query for arrow functions (const fn = () => {})
        let arrow_function_query = Query::new(
            &language.into(),
            r#"
            (variable_declarator
                name: (identifier) @name
                value: (arrow_function
                    parameters: (_) @params))
            "#,
        )
        .context("Failed to create arrow function query")?;

        // Tree-sitter query for class definitions
        let class_query = Query::new(
            &language.into(),
            r#"
            (class_declaration
                name: (identifier) @name
                body: (class_body))
            "#,
        )
        .context("Failed to create class query")?;

        // Tree-sitter query for import statements
        let import_query = Query::new(
            &language.into(),
            r#"
            [
                (import_statement
                    source: (string) @module)
            ]
            "#,
        )
        .context("Failed to create import query")?;

        Ok(Self {
            parser,
            function_query,
            arrow_function_query,
            class_query,
            import_query,
        })
    }

    /// Extract function symbols from AST
    fn extract_functions(&self, source: &str, root: Node) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        // Extract regular function declarations
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&self.function_query, root, source.as_bytes());

        while let Some(match_) = matches.next() {
            let mut name = None;
            let mut params = None;

            for capture in match_.captures {
                let capture_name = self.function_query.capture_names()[capture.index as usize];
                let text = &source[capture.node.byte_range()];

                match capture_name {
                    "name" => name = Some(text.to_string()),
                    "params" => params = Some(text.to_string()),
                    _ => {}
                }
            }

            if let Some(func_name) = name {
                let start_line = match_.captures[0].node.start_position().row + 1;
                let end_line = match_.captures[0].node.end_position().row + 1;

                let signature = params.map(|p| format!("function {}{} {{ }}", func_name, p));

                let mut symbol =
                    Symbol::new(func_name, SymbolKind::Function, (start_line, end_line));
                if let Some(sig) = signature {
                    symbol = symbol.with_signature(sig);
                }

                symbols.push(symbol);
            }
        }

        // Extract arrow functions
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&self.arrow_function_query, root, source.as_bytes());

        while let Some(match_) = matches.next() {
            let mut name = None;
            let mut params = None;

            for capture in match_.captures {
                let capture_name =
                    self.arrow_function_query.capture_names()[capture.index as usize];
                let text = &source[capture.node.byte_range()];

                match capture_name {
                    "name" => name = Some(text.to_string()),
                    "params" => params = Some(text.to_string()),
                    _ => {}
                }
            }

            if let Some(func_name) = name {
                let start_line = match_.captures[0].node.start_position().row + 1;
                let end_line = match_.captures[0].node.end_position().row + 1;

                let signature = params.map(|p| format!("const {} = {} => {{ }}", func_name, p));

                let mut symbol =
                    Symbol::new(func_name, SymbolKind::Function, (start_line, end_line));
                if let Some(sig) = signature {
                    symbol = symbol.with_signature(sig);
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

            for capture in match_.captures {
                let capture_name = self.class_query.capture_names()[capture.index as usize];
                let text = &source[capture.node.byte_range()];

                if capture_name == "name" {
                    name = Some(text.to_string());
                }
            }

            if let Some(class_name) = name {
                let start_line = match_.captures[0].node.start_position().row + 1;
                let end_line = match_.captures[0].node.end_position().row + 1;

                let symbol = Symbol::new(class_name, SymbolKind::Class, (start_line, end_line));
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
                    // Remove quotes from module path
                    let module = text.trim().trim_matches(|c| c == '"' || c == '\'');
                    let line = capture.node.start_position().row + 1;
                    imports.push(Import::new(module, line));
                }
            }
        }

        imports
    }
}

impl Default for JavaScriptParser {
    fn default() -> Self {
        Self::new().expect("Failed to create JavaScript parser")
    }
}

#[async_trait::async_trait]
impl AstParser for JavaScriptParser {
    fn language(&self) -> Language {
        Language::JavaScript
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
                .set_language(&tree_sitter_javascript::LANGUAGE.into())
                .context("Failed to set language")?;

            let tree = parser
                .parse(&source_clone, None)
                .context("Failed to parse JavaScript file")?;

            let mut file_context = FileContext::new(path_clone, Language::JavaScript);

            // Extract symbols
            let parser_inst = JavaScriptParser::new()?;
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
    async fn test_parse_function_declaration() {
        let code = r#"
function hello(name) {
    console.log("Hello, " + name);
}
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = JavaScriptParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.symbols.len(), 1);
        assert_eq!(context.symbols[0].name, "hello");
        assert_eq!(context.symbols[0].kind, SymbolKind::Function);
    }

    #[tokio::test]
    async fn test_parse_arrow_function() {
        let code = r#"
const greet = (name) => {
    return `Hello, ${name}`;
};
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = JavaScriptParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.symbols.len(), 1);
        assert_eq!(context.symbols[0].name, "greet");
        assert_eq!(context.symbols[0].kind, SymbolKind::Function);
    }

    #[tokio::test]
    async fn test_parse_class() {
        let code = r#"
class MyComponent {
    constructor(props) {
        this.props = props;
    }

    render() {
        return null;
    }
}
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = JavaScriptParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        // Should find the class
        let classes = context.symbols_of_kind(SymbolKind::Class);
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "MyComponent");
    }

    #[tokio::test]
    async fn test_parse_imports() {
        let code = r#"
import React from 'react';
import { useState, useEffect } from 'react';
import * as utils from './utils';
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = JavaScriptParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        assert_eq!(context.imports.len(), 3);
        assert!(context.imports.iter().any(|i| i.module == "react"));
        assert!(context.imports.iter().any(|i| i.module == "./utils"));
    }

    #[tokio::test]
    async fn test_parse_mixed_code() {
        let code = r#"
import React from 'react';

class App extends React.Component {
    render() {
        return null;
    }
}

function helper() {
    return true;
}

const process = (data) => {
    return data.map(x => x * 2);
};
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(code.as_bytes()).unwrap();

        let parser = JavaScriptParser::new().unwrap();
        let context = parser.parse_file(temp_file.path()).await.unwrap();

        // Should find: 1 class, 2 functions (helper, process)
        let classes = context.symbols_of_kind(SymbolKind::Class);
        let functions = context.symbols_of_kind(SymbolKind::Function);

        assert_eq!(classes.len(), 1);
        assert!(functions.len() >= 2);
        assert_eq!(context.imports.len(), 1);
    }
}
