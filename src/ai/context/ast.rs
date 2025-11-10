/// Core AST types and structures
///
/// Defines the domain model for AST-based context extraction.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Complete AST context for multiple files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstContext {
    /// File-level contexts indexed by path
    pub file_contexts: HashMap<PathBuf, FileContext>,

    /// Total number of symbols across all files
    pub total_symbols: usize,
}

impl AstContext {
    /// Create a new empty AST context
    pub fn new() -> Self {
        Self {
            file_contexts: HashMap::new(),
            total_symbols: 0,
        }
    }

    /// Add a file context
    pub fn add_file(&mut self, context: FileContext) {
        self.total_symbols += context.symbols.len();
        self.file_contexts.insert(context.path.clone(), context);
    }

    /// Get context for a specific file
    pub fn get_file(&self, path: &PathBuf) -> Option<&FileContext> {
        self.file_contexts.get(path)
    }

    /// Get all symbols across all files
    pub fn all_symbols(&self) -> Vec<&Symbol> {
        self.file_contexts
            .values()
            .flat_map(|fc| &fc.symbols)
            .collect()
    }
}

impl Default for AstContext {
    fn default() -> Self {
        Self::new()
    }
}

/// File-level AST context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContext {
    /// File path
    pub path: PathBuf,

    /// Programming language
    pub language: Language,

    /// Extracted symbols (functions, classes, etc.)
    pub symbols: Vec<Symbol>,

    /// Import/use statements
    pub imports: Vec<Import>,
}

impl FileContext {
    /// Create a new file context
    pub fn new(path: PathBuf, language: Language) -> Self {
        Self {
            path,
            language,
            symbols: Vec::new(),
            imports: Vec::new(),
        }
    }

    /// Add a symbol to this file
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    /// Add an import statement
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    /// Get symbols of a specific kind
    pub fn symbols_of_kind(&self, kind: SymbolKind) -> Vec<&Symbol> {
        self.symbols.iter().filter(|s| s.kind == kind).collect()
    }
}

/// A code symbol (function, class, method, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Symbol name
    pub name: String,

    /// Symbol kind (function, class, etc.)
    pub kind: SymbolKind,

    /// Line range in source file (start, end)
    pub line_range: (usize, usize),

    /// Function/method signature (optional)
    pub signature: Option<String>,

    /// Documentation string (docstring, JSDoc, etc.)
    pub docstring: Option<String>,
}

impl Symbol {
    /// Create a new symbol
    pub fn new(name: impl Into<String>, kind: SymbolKind, line_range: (usize, usize)) -> Self {
        Self {
            name: name.into(),
            kind,
            line_range,
            signature: None,
            docstring: None,
        }
    }

    /// Set the signature
    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }

    /// Set the docstring
    pub fn with_docstring(mut self, docstring: impl Into<String>) -> Self {
        self.docstring = Some(docstring.into());
        self
    }
}

/// Kind of symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    /// Function definition
    Function,

    /// Class definition
    Class,

    /// Method (function inside class)
    Method,

    /// Variable or constant
    Variable,

    /// Constant
    Constant,

    /// Interface (TypeScript, etc.)
    Interface,

    /// Type alias
    Type,

    /// Enum
    Enum,

    /// Struct (Rust, etc.)
    Struct,

    /// Trait (Rust)
    Trait,
}

/// Import/use statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    /// Module/package being imported
    pub module: String,

    /// Specific items imported (empty for wildcard)
    pub items: Vec<String>,

    /// Line number in source file
    pub line: usize,
}

impl Import {
    /// Create a new import
    pub fn new(module: impl Into<String>, line: usize) -> Self {
        Self {
            module: module.into(),
            items: Vec::new(),
            line,
        }
    }

    /// Add imported items
    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }
}

/// Programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
}

impl Language {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "py" | "pyw" => Some(Language::Python),
            "js" | "jsx" | "mjs" | "cjs" => Some(Language::JavaScript),
            "ts" | "tsx" => Some(Language::TypeScript),
            "rs" => Some(Language::Rust),
            _ => None,
        }
    }

    /// Get file extensions for this language
    pub fn extensions(&self) -> &[&str] {
        match self {
            Language::Python => &["py", "pyw"],
            Language::JavaScript => &["js", "jsx", "mjs", "cjs"],
            Language::TypeScript => &["ts", "tsx"],
            Language::Rust => &["rs"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_context_creation() {
        let context = AstContext::new();
        assert_eq!(context.total_symbols, 0);
        assert!(context.file_contexts.is_empty());
    }

    #[test]
    fn test_ast_context_add_file() {
        let mut context = AstContext::new();
        let mut file_ctx = FileContext::new(PathBuf::from("test.py"), Language::Python);
        file_ctx.add_symbol(Symbol::new("foo", SymbolKind::Function, (1, 10)));

        context.add_file(file_ctx);
        assert_eq!(context.total_symbols, 1);
        assert_eq!(context.file_contexts.len(), 1);
    }

    #[test]
    fn test_symbol_creation() {
        let symbol = Symbol::new("test_func", SymbolKind::Function, (10, 20))
            .with_signature("def test_func(x: int) -> str")
            .with_docstring("Test function");

        assert_eq!(symbol.name, "test_func");
        assert_eq!(symbol.kind, SymbolKind::Function);
        assert_eq!(symbol.line_range, (10, 20));
        assert!(symbol.signature.is_some());
        assert!(symbol.docstring.is_some());
    }

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("py"), Some(Language::Python));
        assert_eq!(Language::from_extension("js"), Some(Language::JavaScript));
        assert_eq!(Language::from_extension("ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
        assert_eq!(Language::from_extension("txt"), None);
    }

    #[test]
    fn test_language_extensions() {
        let py_exts = Language::Python.extensions();
        assert!(py_exts.contains(&"py"));
        assert!(py_exts.contains(&"pyw"));
    }

    #[test]
    fn test_import_creation() {
        let import =
            Import::new("os", 1).with_items(vec!["path".to_string(), "getcwd".to_string()]);

        assert_eq!(import.module, "os");
        assert_eq!(import.items.len(), 2);
        assert_eq!(import.line, 1);
    }

    #[test]
    fn test_file_context_symbols_of_kind() {
        let mut file_ctx = FileContext::new(PathBuf::from("test.py"), Language::Python);
        file_ctx.add_symbol(Symbol::new("func1", SymbolKind::Function, (1, 5)));
        file_ctx.add_symbol(Symbol::new("Class1", SymbolKind::Class, (7, 20)));
        file_ctx.add_symbol(Symbol::new("func2", SymbolKind::Function, (22, 30)));

        let functions = file_ctx.symbols_of_kind(SymbolKind::Function);
        assert_eq!(functions.len(), 2);

        let classes = file_ctx.symbols_of_kind(SymbolKind::Class);
        assert_eq!(classes.len(), 1);
    }
}
