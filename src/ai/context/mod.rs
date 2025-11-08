/// AST-based context extraction for code understanding
///
/// This module provides AST (Abstract Syntax Tree) parsing and context extraction
/// for multiple programming languages using tree-sitter. It extracts functions,
/// classes, imports, and other code structures to enhance LLM context.
///
/// # Architecture
///
/// - `ast.rs` - Core domain types (Symbol, FileContext, AstContext)
/// - `parser.rs` - Parser trait and language detection
/// - `cache.rs` - LRU cache for parsed ASTs
/// - `extractors/` - Language-specific extractors (Python, JS/TS, Rust)
/// - `builder.rs` - Context building API
///
/// # Example
///
/// ```no_run
/// use toad::ai::context::{ContextBuilder, Language};
///
/// # async fn example() -> anyhow::Result<()> {
/// let context = ContextBuilder::new()
///     .add_file("src/main.py")?
///     .add_file("src/utils.js")?
///     .build().await?;
///
/// println!("Extracted {} symbols from {} files",
///     context.total_symbols,
///     context.file_contexts.len());
/// # Ok(())
/// # }
/// ```

pub mod ast;
pub mod cache;
pub mod extractors;
pub mod parser;

pub use ast::{AstContext, FileContext, Import, Language, Symbol, SymbolKind};
pub use cache::AstCache;
pub use parser::AstParser;
