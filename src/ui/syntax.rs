//! Syntax highlighting using Tree-sitter
//!
//! Provides AST-based syntax highlighting for multiple programming languages
//! with semantic colors and tree-sitter integration.
//!
//! # Examples
//!
//! ```no_run
//! use toad::ui::syntax::{SyntaxHighlighter, Language};
//!
//! let highlighter = SyntaxHighlighter::new();
//! let code = "fn main() { println!(\"Hello\"); }";
//! let highlighted = highlighter.highlight(code, Language::Rust);
//! ```

use ratatui::style::{Color, Style};
use std::collections::HashMap;
use tree_sitter::{Parser, Tree};
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Rust programming language
    Rust,
    /// JavaScript/TypeScript
    JavaScript,
    /// Python
    Python,
    /// Plain text (no highlighting)
    PlainText,
}

impl Language {
    /// Get the tree-sitter language grammar
    fn grammar(&self) -> Option<tree_sitter::Language> {
        match self {
            Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
            Language::JavaScript => Some(tree_sitter_javascript::LANGUAGE.into()),
            Language::Python => Some(tree_sitter_python::LANGUAGE.into()),
            Language::PlainText => None,
        }
    }

    /// Get highlight query for this language
    fn highlight_query(&self) -> Option<&'static str> {
        match self {
            Language::Rust => Some(tree_sitter_rust::HIGHLIGHTS_QUERY),
            Language::JavaScript => Some(tree_sitter_javascript::HIGHLIGHT_QUERY),
            Language::Python => Some(tree_sitter_python::HIGHLIGHTS_QUERY),
            Language::PlainText => None,
        }
    }

    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => Language::JavaScript,
            "py" | "pyw" | "pyi" => Language::Python,
            _ => Language::PlainText,
        }
    }

    /// Get file extensions for this language
    pub fn extensions(&self) -> &[&str] {
        match self {
            Language::Rust => &["rs"],
            Language::JavaScript => &["js", "jsx", "ts", "tsx", "mjs", "cjs"],
            Language::Python => &["py", "pyw", "pyi"],
            Language::PlainText => &["txt"],
        }
    }
}

/// Highlight capture names mapped to colors
#[derive(Debug, Clone)]
pub struct HighlightTheme {
    colors: HashMap<String, Color>,
}

impl Default for HighlightTheme {
    fn default() -> Self {
        Self::monokai()
    }
}

impl HighlightTheme {
    /// Create a new highlight theme
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
        }
    }

    /// Monokai-inspired theme
    pub fn monokai() -> Self {
        let mut colors = HashMap::new();

        // Keywords
        colors.insert("keyword".to_string(), Color::Rgb(249, 38, 114)); // Pink
        colors.insert("keyword.control".to_string(), Color::Rgb(249, 38, 114));
        colors.insert("keyword.function".to_string(), Color::Rgb(249, 38, 114));

        // Functions
        colors.insert("function".to_string(), Color::Rgb(166, 226, 46)); // Green
        colors.insert("function.call".to_string(), Color::Rgb(166, 226, 46));
        colors.insert("function.method".to_string(), Color::Rgb(166, 226, 46));

        // Types
        colors.insert("type".to_string(), Color::Rgb(102, 217, 239)); // Cyan
        colors.insert("type.builtin".to_string(), Color::Rgb(102, 217, 239));

        // Strings
        colors.insert("string".to_string(), Color::Rgb(230, 219, 116)); // Yellow
        colors.insert("string.escape".to_string(), Color::Rgb(174, 129, 255)); // Purple

        // Numbers
        colors.insert("number".to_string(), Color::Rgb(174, 129, 255)); // Purple
        colors.insert("constant".to_string(), Color::Rgb(174, 129, 255));

        // Comments
        colors.insert("comment".to_string(), Color::Rgb(117, 113, 94)); // Gray

        // Variables
        colors.insert("variable".to_string(), Color::Rgb(248, 248, 242)); // White
        colors.insert("variable.parameter".to_string(), Color::Rgb(253, 151, 31)); // Orange

        // Operators
        colors.insert("operator".to_string(), Color::Rgb(249, 38, 114)); // Pink
        colors.insert("punctuation".to_string(), Color::Rgb(248, 248, 242)); // White

        Self { colors }
    }

    /// Get color for a capture name
    pub fn color_for(&self, capture: &str) -> Option<Color> {
        self.colors.get(capture).copied()
    }

    /// Set color for a capture name
    pub fn set_color(&mut self, capture: impl Into<String>, color: Color) {
        self.colors.insert(capture.into(), color);
    }
}

/// Highlighted text span
#[derive(Debug, Clone)]
pub struct HighlightedSpan {
    /// Text content
    pub text: String,
    /// Style to apply
    pub style: Style,
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
}

/// Syntax highlighter using Tree-sitter
pub struct SyntaxHighlighter {
    /// Highlight configurations by language
    configs: HashMap<Language, HighlightConfiguration>,
    /// Color theme
    theme: HighlightTheme,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::syntax::SyntaxHighlighter;
    ///
    /// let highlighter = SyntaxHighlighter::new();
    /// ```
    pub fn new() -> Self {
        let mut configs = HashMap::new();

        // Initialize highlight configurations for supported languages
        for lang in [Language::Rust, Language::JavaScript, Language::Python] {
            if let Some(config) = Self::create_highlight_config(lang) {
                configs.insert(lang, config);
            }
        }

        Self {
            configs,
            theme: HighlightTheme::monokai(),
        }
    }

    /// Create highlight configuration for a language
    fn create_highlight_config(language: Language) -> Option<HighlightConfiguration> {
        let grammar = language.grammar()?;
        let query = language.highlight_query()?;

        let mut config = HighlightConfiguration::new(
            grammar, "source", // Name of the language
            query, "", // Injection query (empty for now)
            "", // Locals query (empty for now)
        )
        .ok()?;

        // Configure highlight names
        config.configure(&[
            "keyword",
            "keyword.control",
            "keyword.function",
            "function",
            "function.call",
            "function.method",
            "type",
            "type.builtin",
            "string",
            "string.escape",
            "number",
            "constant",
            "comment",
            "variable",
            "variable.parameter",
            "operator",
            "punctuation",
        ]);

        Some(config)
    }

    /// Set the color theme
    pub fn set_theme(&mut self, theme: HighlightTheme) {
        self.theme = theme;
    }

    /// Highlight source code
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::ui::syntax::{SyntaxHighlighter, Language};
    ///
    /// let highlighter = SyntaxHighlighter::new();
    /// let code = "fn main() {}";
    /// let spans = highlighter.highlight(code, Language::Rust);
    /// ```
    pub fn highlight(&self, source: &str, language: Language) -> Vec<HighlightedSpan> {
        if language == Language::PlainText {
            return vec![HighlightedSpan {
                text: source.to_string(),
                style: Style::default(),
                start: 0,
                end: source.len(),
            }];
        }

        let config = match self.configs.get(&language) {
            Some(c) => c,
            None => {
                // No config, return plain text
                return vec![HighlightedSpan {
                    text: source.to_string(),
                    style: Style::default(),
                    start: 0,
                    end: source.len(),
                }];
            }
        };

        let mut highlighter = Highlighter::new();
        let highlights = highlighter
            .highlight(config, source.as_bytes(), None, |_| None)
            .ok();

        let highlights = match highlights {
            Some(h) => h,
            None => {
                return vec![HighlightedSpan {
                    text: source.to_string(),
                    style: Style::default(),
                    start: 0,
                    end: source.len(),
                }];
            }
        };

        let mut spans = Vec::new();
        let mut current_style = Style::default();
        let mut last_end = 0;

        for event in highlights {
            match event {
                Ok(HighlightEvent::Source { start, end }) => {
                    if start > last_end {
                        // Add gap as default style
                        spans.push(HighlightedSpan {
                            text: source[last_end..start].to_string(),
                            style: Style::default(),
                            start: last_end,
                            end: start,
                        });
                    }

                    spans.push(HighlightedSpan {
                        text: source[start..end].to_string(),
                        style: current_style,
                        start,
                        end,
                    });
                    last_end = end;
                }
                Ok(HighlightEvent::HighlightStart(highlight)) => {
                    // Map highlight index to capture name
                    // For simplicity, use a default color based on index
                    let color = match highlight.0 {
                        0 => self.theme.color_for("keyword"),
                        1 | 2 => self.theme.color_for("keyword.control"),
                        3..=5 => self.theme.color_for("function"),
                        6 | 7 => self.theme.color_for("type"),
                        8 | 9 => self.theme.color_for("string"),
                        10 | 11 => self.theme.color_for("number"),
                        12 => self.theme.color_for("comment"),
                        13 | 14 => self.theme.color_for("variable"),
                        _ => None,
                    };

                    current_style = Style::default().fg(color.unwrap_or(Color::White));
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    current_style = Style::default();
                }
                Err(_) => {}
            }
        }

        // Add remaining text
        if last_end < source.len() {
            spans.push(HighlightedSpan {
                text: source[last_end..].to_string(),
                style: Style::default(),
                start: last_end,
                end: source.len(),
            });
        }

        spans
    }

    /// Parse source code and return syntax tree
    pub fn parse(&self, source: &str, language: Language) -> Option<Tree> {
        let grammar = language.grammar()?;
        let mut parser = Parser::new();
        parser.set_language(&grammar).ok()?;
        parser.parse(source, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("ts"), Language::JavaScript);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("txt"), Language::PlainText);
        assert_eq!(Language::from_extension("unknown"), Language::PlainText);
    }

    #[test]
    fn test_language_extensions() {
        assert!(Language::Rust.extensions().contains(&"rs"));
        assert!(Language::JavaScript.extensions().contains(&"js"));
        assert!(Language::JavaScript.extensions().contains(&"ts"));
        assert!(Language::Python.extensions().contains(&"py"));
    }

    #[test]
    fn test_syntax_highlighter_new() {
        let highlighter = SyntaxHighlighter::new();
        assert!(highlighter.configs.contains_key(&Language::Rust));
        assert!(highlighter.configs.contains_key(&Language::JavaScript));
        assert!(highlighter.configs.contains_key(&Language::Python));
    }

    #[test]
    fn test_highlight_plain_text() {
        let highlighter = SyntaxHighlighter::new();
        let source = "Hello, world!";
        let spans = highlighter.highlight(source, Language::PlainText);

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].text, "Hello, world!");
    }

    #[test]
    fn test_highlight_rust() {
        let highlighter = SyntaxHighlighter::new();
        let source = "fn main() {}";
        let spans = highlighter.highlight(source, Language::Rust);

        assert!(!spans.is_empty());
        // Should have highlighted "fn" keyword
        assert!(spans.iter().any(|s| s.text.contains("fn")));
    }

    #[test]
    fn test_highlight_javascript() {
        let highlighter = SyntaxHighlighter::new();
        let source = "function hello() { return 42; }";
        let spans = highlighter.highlight(source, Language::JavaScript);

        assert!(!spans.is_empty());
    }

    #[test]
    fn test_highlight_python() {
        let highlighter = SyntaxHighlighter::new();
        let source = "def hello():\n    return 42";
        let spans = highlighter.highlight(source, Language::Python);

        assert!(!spans.is_empty());
    }

    #[test]
    fn test_highlight_theme() {
        let theme = HighlightTheme::monokai();
        assert!(theme.color_for("keyword").is_some());
        assert!(theme.color_for("function").is_some());
        assert!(theme.color_for("string").is_some());
        assert!(theme.color_for("comment").is_some());
    }

    #[test]
    fn test_parse_rust() {
        let highlighter = SyntaxHighlighter::new();
        let source = "fn main() {}";
        let tree = highlighter.parse(source, Language::Rust);

        assert!(tree.is_some());
        let tree = tree.unwrap();
        assert_eq!(tree.root_node().kind(), "source_file");
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let highlighter = SyntaxHighlighter::new();
        let source = "fn main() { invalid";
        let tree = highlighter.parse(source, Language::Rust);

        // Should still return a tree even with errors
        assert!(tree.is_some());
    }
}
