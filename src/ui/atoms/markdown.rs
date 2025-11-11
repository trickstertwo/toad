//! Markdown rendering atom
//!
//! Converts markdown text to styled ratatui components for display in the TUI.
//!
//! # Features
//!
//! - **Bold** (`**text**`)
//! - *Italic* (`*text*`)
//! - `Inline code` (`` `code` ``)
//! - Block quotes (`> quote`)
//! - Code blocks (` ```language `)
//! - Headings (`# H1`, `## H2`, etc.)
//! - Lists (unordered and ordered)
//!
//! # Examples
//!
//! ```
//! use toad::ui::atoms::markdown::MarkdownRenderer;
//!
//! let md = "**Bold** and *italic* text with `code`";
//! let renderer = MarkdownRenderer::new();
//! let lines = renderer.render(md);
//! ```

use crate::ui::theme::ToadTheme;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

/// Markdown renderer that converts markdown to styled ratatui Lines
#[derive(Debug, Clone)]
pub struct MarkdownRenderer {
    /// Parser options
    options: Options,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownRenderer {
    /// Create a new markdown renderer with default options
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::markdown::MarkdownRenderer;
    ///
    /// let renderer = MarkdownRenderer::new();
    /// ```
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        Self { options }
    }

    /// Render markdown text to ratatui Lines
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::atoms::markdown::MarkdownRenderer;
    ///
    /// let renderer = MarkdownRenderer::new();
    /// let lines = renderer.render("**Bold** text");
    /// assert!(!lines.is_empty());
    /// ```
    pub fn render(&self, markdown: &str) -> Vec<Line<'static>> {
        let parser = Parser::new_ext(markdown, self.options);
        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let mut style_stack = Vec::new();

        for event in parser {
            match event {
                Event::Start(tag) => {
                    style_stack.push(tag);
                }
                Event::End(tag_end) => {
                    if let Some(last_tag) = style_stack.pop() {
                        // Verify matching tags
                        if !tags_match(&last_tag, &tag_end) {
                            // Tag mismatch, push back and continue
                            style_stack.push(last_tag);
                        }
                    }
                }
                Event::Text(text) => {
                    let style = Self::compute_style(&style_stack);
                    current_line.push(Span::styled(text.to_string(), style));
                }
                Event::Code(code) => {
                    let style = Style::default()
                        .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                        .bg(ToadTheme::DARK_GRAY)
                        .add_modifier(Modifier::BOLD);
                    current_line.push(Span::styled(code.to_string(), style));
                }
                Event::SoftBreak => {
                    current_line.push(Span::raw(" "));
                }
                Event::HardBreak => {
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                }
                Event::Rule => {
                    // Horizontal rule
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                    let rule = Span::styled(
                        "─".repeat(80),
                        Style::default().fg(ToadTheme::DARK_GRAY),
                    );
                    lines.push(Line::from(vec![rule]));
                }
                Event::Html(_) | Event::InlineHtml(_) => {
                    // Skip HTML for security and simplicity
                }
                Event::FootnoteReference(_) => {
                    // Skip footnote references for now
                }
                Event::TaskListMarker(checked) => {
                    let marker = if checked { "[✓] " } else { "[ ] " };
                    let style = Style::default().fg(ToadTheme::TOAD_GREEN);
                    current_line.push(Span::styled(marker.to_string(), style));
                }
                Event::InlineMath(_) | Event::DisplayMath(_) => {
                    // Skip math rendering for now (would need MathML or LaTeX parser)
                }
            }
        }

        // Push any remaining content
        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        // Ensure at least one empty line if no content
        if lines.is_empty() {
            lines.push(Line::from(""));
        }

        lines
    }

    /// Compute the combined style from the current tag stack
    fn compute_style(tags: &[Tag]) -> Style {
        let mut style = Style::default();

        for tag in tags {
            match tag {
                Tag::Strong => {
                    style = style.add_modifier(Modifier::BOLD);
                }
                Tag::Emphasis => {
                    style = style.add_modifier(Modifier::ITALIC);
                }
                Tag::Strikethrough => {
                    style = style.add_modifier(Modifier::CROSSED_OUT);
                }
                Tag::Heading { level, .. } => {
                    style = style
                        .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                        .add_modifier(Modifier::BOLD);

                    // Make H1 and H2 even brighter
                    if matches!(level, HeadingLevel::H1 | HeadingLevel::H2) {
                        style = style.add_modifier(Modifier::UNDERLINED);
                    }
                }
                Tag::BlockQuote(_) => {
                    style = style.fg(ToadTheme::DARK_GRAY).add_modifier(Modifier::ITALIC);
                }
                Tag::CodeBlock(_) => {
                    style = style
                        .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                        .bg(ToadTheme::DARK_GRAY);
                }
                Tag::Link { .. } => {
                    style = style
                        .fg(ToadTheme::TOAD_GREEN_BRIGHT)
                        .add_modifier(Modifier::UNDERLINED);
                }
                _ => {}
            }
        }

        style
    }
}

/// Check if opening tag matches closing tag
fn tags_match(open: &Tag, close: &TagEnd) -> bool {
    match (open, close) {
        (Tag::Paragraph, TagEnd::Paragraph) => true,
        (Tag::Heading { .. }, TagEnd::Heading(_)) => true,
        (Tag::BlockQuote(_), TagEnd::BlockQuote(_)) => true,
        (Tag::CodeBlock(_), TagEnd::CodeBlock) => true,
        (Tag::HtmlBlock, TagEnd::HtmlBlock) => true,
        (Tag::List(_), TagEnd::List(_)) => true,
        (Tag::Item, TagEnd::Item) => true,
        (Tag::FootnoteDefinition(_), TagEnd::FootnoteDefinition) => true,
        (Tag::Table(_), TagEnd::Table) => true,
        (Tag::TableHead, TagEnd::TableHead) => true,
        (Tag::TableRow, TagEnd::TableRow) => true,
        (Tag::TableCell, TagEnd::TableCell) => true,
        (Tag::Emphasis, TagEnd::Emphasis) => true,
        (Tag::Strong, TagEnd::Strong) => true,
        (Tag::Strikethrough, TagEnd::Strikethrough) => true,
        (Tag::Link { .. }, TagEnd::Link) => true,
        (Tag::Image { .. }, TagEnd::Image) => true,
        (Tag::MetadataBlock(_), TagEnd::MetadataBlock(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = MarkdownRenderer::new();
        assert!(renderer.options.contains(Options::ENABLE_STRIKETHROUGH));
    }

    #[test]
    fn test_empty_markdown() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("");
        assert_eq!(lines.len(), 1); // Should have at least one empty line
    }

    #[test]
    fn test_plain_text() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("Hello, world!");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_bold_text() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("**Bold text**");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_italic_text() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("*Italic text*");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_inline_code() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("`inline code`");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_mixed_formatting() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("**Bold** and *italic* with `code`");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_heading() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("# Heading 1");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_blockquote() {
        let renderer = MarkdownRenderer::new();
        let lines = renderer.render("> This is a quote");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_code_block() {
        let renderer = MarkdownRenderer::new();
        let markdown = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let lines = renderer.render(markdown);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_multiline_text() {
        let renderer = MarkdownRenderer::new();
        let markdown = "Line 1\n\nLine 2\n\nLine 3";
        let lines = renderer.render(markdown);
        assert!(lines.len() >= 3);
    }

    #[test]
    fn test_task_list() {
        let renderer = MarkdownRenderer::new();
        let markdown = "- [x] Completed task\n- [ ] Incomplete task";
        let lines = renderer.render(markdown);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_horizontal_rule() {
        let renderer = MarkdownRenderer::new();
        let markdown = "Before\n\n---\n\nAfter";
        let lines = renderer.render(markdown);
        assert!(lines.len() >= 3);
    }

    #[test]
    fn test_link() {
        let renderer = MarkdownRenderer::new();
        let markdown = "[Link text](https://example.com)";
        let lines = renderer.render(markdown);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_nested_formatting() {
        let renderer = MarkdownRenderer::new();
        let markdown = "**Bold with *italic* inside**";
        let lines = renderer.render(markdown);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_default_trait() {
        let renderer = MarkdownRenderer::default();
        let lines = renderer.render("Test");
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_unicode_content() {
        let renderer = MarkdownRenderer::new();
        let markdown = "🐸 **TOAD** with 日本語";
        let lines = renderer.render(markdown);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_escaped_characters() {
        let renderer = MarkdownRenderer::new();
        let markdown = r"Escaped \*asterisks\* and \`backticks\`";
        let lines = renderer.render(markdown);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_multiple_paragraphs() {
        let renderer = MarkdownRenderer::new();
        let markdown = "Paragraph 1\n\nParagraph 2\n\nParagraph 3";
        let lines = renderer.render(markdown);
        assert!(lines.len() >= 3);
    }
}
