//! Smart text truncation with intelligent ellipsis placement
//!
//! Provides context-aware text truncation for display in constrained spaces,
//! with expand-on-demand functionality and semantic awareness.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::SmartTruncate;
//!
//! let text = "This is a very long string that needs truncation";
//! let truncated = SmartTruncate::truncate(text, 30, true);
//! ```

/// Truncation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruncationStrategy {
    /// Truncate at end with ellipsis: "Hello worl..."
    End,
    /// Truncate at start with ellipsis: "...orld!"
    Start,
    /// Truncate in middle with ellipsis: "Hello...rld!"
    Middle,
    /// Preserve file extension: "very_long_filen...txt"
    FileName,
    /// Preserve path parts: "/some/very/.../path"
    Path,
    /// Word-aware truncation: "Hello world ..."
    WordBoundary,
}

/// Smart text truncation utility
pub struct SmartTruncate;

impl SmartTruncate {
    /// Truncate text intelligently
    ///
    /// # Arguments
    ///
    /// * `text` - The text to truncate
    /// * `max_width` - Maximum width in graphemes
    /// * `word_aware` - Whether to break at word boundaries
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::SmartTruncate;
    ///
    /// let result = SmartTruncate::truncate("Hello world!", 8, false);
    /// assert_eq!(result, "Hello...");
    /// ```
    pub fn truncate(text: &str, max_width: usize, word_aware: bool) -> String {
        if max_width < 3 {
            return "...".to_string();
        }

        let chars: Vec<char> = text.chars().collect();

        if chars.len() <= max_width {
            return text.to_string();
        }

        if word_aware {
            Self::truncate_word_boundary(text, max_width)
        } else {
            Self::truncate_end(text, max_width)
        }
    }

    /// Truncate at end with ellipsis
    pub fn truncate_end(text: &str, max_width: usize) -> String {
        if max_width < 3 {
            return "...".to_string();
        }

        let chars: Vec<char> = text.chars().collect();

        if chars.len() <= max_width {
            return text.to_string();
        }

        let keep = max_width - 3;
        let mut result: String = chars[..keep].iter().collect();
        result.push_str("...");
        result
    }

    /// Truncate at start with ellipsis
    pub fn truncate_start(text: &str, max_width: usize) -> String {
        if max_width < 3 {
            return "...".to_string();
        }

        let chars: Vec<char> = text.chars().collect();

        if chars.len() <= max_width {
            return text.to_string();
        }

        let keep = max_width - 3;
        let start_pos = chars.len() - keep;
        let mut result = String::from("...");
        result.push_str(&chars[start_pos..].iter().collect::<String>());
        result
    }

    /// Truncate in middle with ellipsis
    pub fn truncate_middle(text: &str, max_width: usize) -> String {
        if max_width < 5 {
            return "...".to_string();
        }

        let chars: Vec<char> = text.chars().collect();

        if chars.len() <= max_width {
            return text.to_string();
        }

        let available = max_width - 3;
        let left_part = available / 2;
        let right_part = available - left_part;

        let mut result: String = chars[..left_part].iter().collect();
        result.push_str("...");
        result.push_str(&chars[chars.len() - right_part..].iter().collect::<String>());
        result
    }

    /// Truncate preserving file extension
    pub fn truncate_filename(text: &str, max_width: usize) -> String {
        if max_width < 7 {
            // Not enough space for "...ext"
            return Self::truncate_end(text, max_width);
        }

        // Find extension
        if let Some(dot_pos) = text.rfind('.') {
            let (name, ext) = text.split_at(dot_pos);

            if ext.len() > 5 {
                // Extension too long, treat as regular text
                return Self::truncate_end(text, max_width);
            }

            let chars: Vec<char> = text.chars().collect();

            if chars.len() <= max_width {
                return text.to_string();
            }

            // Reserve space for "..." + extension
            let ext_len = ext.chars().count();
            let available = max_width.saturating_sub(3 + ext_len);

            if available < 1 {
                return Self::truncate_end(text, max_width);
            }

            let name_chars: Vec<char> = name.chars().collect();
            let mut result: String = name_chars[..available.min(name_chars.len())]
                .iter()
                .collect();
            result.push_str("...");
            result.push_str(ext);
            result
        } else {
            Self::truncate_end(text, max_width)
        }
    }

    /// Truncate file path intelligently
    pub fn truncate_path(text: &str, max_width: usize) -> String {
        if max_width < 10 {
            return Self::truncate_end(text, max_width);
        }

        let chars: Vec<char> = text.chars().collect();

        if chars.len() <= max_width {
            return text.to_string();
        }

        // Try to preserve first and last parts
        let parts: Vec<&str> = text.split('/').collect();

        if parts.len() <= 2 {
            return Self::truncate_middle(text, max_width);
        }

        // Keep first part (directory root) and last part (filename)
        let first = parts[0];
        let last = parts[parts.len() - 1];

        let prefix = if first.is_empty() { "/" } else { first };
        let combined_len = prefix.len() + 4 + last.len(); // prefix + "/.../" + last

        if combined_len <= max_width {
            format!("{}/.../{}", prefix, last)
        } else {
            // Not enough space, truncate middle
            Self::truncate_middle(text, max_width)
        }
    }

    /// Truncate at word boundary
    pub fn truncate_word_boundary(text: &str, max_width: usize) -> String {
        if max_width < 5 {
            return "...".to_string();
        }

        let chars: Vec<char> = text.chars().collect();

        if chars.len() <= max_width {
            return text.to_string();
        }

        // Find last word boundary before max_width - 4
        let target = max_width - 4; // Reserve for " ..."
        let mut last_space = 0;

        for (i, &ch) in chars.iter().enumerate() {
            if i > target {
                break;
            }
            if ch == ' ' {
                last_space = i;
            }
        }

        if last_space == 0 {
            // No spaces found, truncate at position
            return Self::truncate_end(text, max_width);
        }

        let mut result: String = chars[..last_space].iter().collect();
        result.push_str(" ...");
        result
    }

    /// Truncate with strategy
    pub fn truncate_with_strategy(
        text: &str,
        max_width: usize,
        strategy: TruncationStrategy,
    ) -> String {
        match strategy {
            TruncationStrategy::End => Self::truncate_end(text, max_width),
            TruncationStrategy::Start => Self::truncate_start(text, max_width),
            TruncationStrategy::Middle => Self::truncate_middle(text, max_width),
            TruncationStrategy::FileName => Self::truncate_filename(text, max_width),
            TruncationStrategy::Path => Self::truncate_path(text, max_width),
            TruncationStrategy::WordBoundary => Self::truncate_word_boundary(text, max_width),
        }
    }

    /// Check if text needs truncation
    pub fn needs_truncation(text: &str, max_width: usize) -> bool {
        text.chars().count() > max_width
    }

    /// Get character count
    pub fn char_count(text: &str) -> usize {
        text.chars().count()
    }

    /// Truncate to fit width, auto-detecting strategy
    pub fn auto_truncate(text: &str, max_width: usize) -> String {
        if !Self::needs_truncation(text, max_width) {
            return text.to_string();
        }

        // Auto-detect best strategy
        if text.contains('/') && text.matches('/').count() >= 2 {
            Self::truncate_path(text, max_width)
        } else if text.contains('.') && text.rfind('.').map(|i| i > text.len() / 2).unwrap_or(false)
        {
            Self::truncate_filename(text, max_width)
        } else if text.contains(' ') {
            Self::truncate_word_boundary(text, max_width)
        } else {
            Self::truncate_end(text, max_width)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_end() {
        let text = "Hello world!";
        let result = SmartTruncate::truncate_end(text, 8);
        assert_eq!(result, "Hello...");
    }

    #[test]
    fn test_truncate_no_truncation_needed() {
        let text = "Short";
        let result = SmartTruncate::truncate_end(text, 10);
        assert_eq!(result, "Short");
    }

    #[test]
    fn test_truncate_start() {
        let text = "Hello world!";
        let result = SmartTruncate::truncate_start(text, 8);
        assert_eq!(result, "...orld!");
    }

    #[test]
    fn test_truncate_middle() {
        let text = "Hello world!";
        let result = SmartTruncate::truncate_middle(text, 9);
        assert_eq!(result, "Hel...ld!");
    }

    #[test]
    fn test_truncate_filename() {
        let text = "very_long_filename_here.txt";
        let result = SmartTruncate::truncate_filename(text, 20);
        assert_eq!(result, "very_long_fil....txt");
    }

    #[test]
    fn test_truncate_filename_no_extension() {
        let text = "very_long_filename_here";
        let result = SmartTruncate::truncate_filename(text, 15);
        assert_eq!(result, "very_long_f...");
    }

    #[test]
    fn test_truncate_path() {
        let text = "/home/user/very/long/path/to/file.txt";
        let result = SmartTruncate::truncate_path(text, 25);
        assert_eq!(result, "/.../file.txt");
    }

    #[test]
    fn test_truncate_word_boundary() {
        let text = "The quick brown fox jumps";
        let result = SmartTruncate::truncate_word_boundary(text, 18);
        assert_eq!(result, "The quick brown ...");
    }

    #[test]
    fn test_truncate_word_boundary_no_spaces() {
        let text = "Verylongwordwithoutspaces";
        let result = SmartTruncate::truncate_word_boundary(text, 15);
        assert_eq!(result, "Verylong...");
    }

    #[test]
    fn test_truncate_with_strategy_end() {
        let text = "Hello world!";
        let result = SmartTruncate::truncate_with_strategy(text, 8, TruncationStrategy::End);
        assert_eq!(result, "Hello...");
    }

    #[test]
    fn test_truncate_with_strategy_middle() {
        let text = "Hello world!";
        let result = SmartTruncate::truncate_with_strategy(text, 9, TruncationStrategy::Middle);
        assert_eq!(result, "Hel...ld!");
    }

    #[test]
    fn test_needs_truncation() {
        assert!(SmartTruncate::needs_truncation("Hello world!", 8));
        assert!(!SmartTruncate::needs_truncation("Hello", 10));
    }

    #[test]
    fn test_char_count() {
        assert_eq!(SmartTruncate::char_count("Hello"), 5);
        assert_eq!(SmartTruncate::char_count("Hello world!"), 12);
        // Unicode characters
        let emoji = "👋🌍";
        assert!(SmartTruncate::char_count(emoji) > 0);
    }

    #[test]
    fn test_auto_truncate_path() {
        let text = "/home/user/docs/file.txt";
        let result = SmartTruncate::auto_truncate(text, 18);
        assert_eq!(result, "/.../file.txt");
    }

    #[test]
    fn test_auto_truncate_filename() {
        let text = "document.txt";
        let result = SmartTruncate::auto_truncate(text, 10);
        assert_eq!(result, "docum...txt");
    }

    #[test]
    fn test_auto_truncate_text_with_spaces() {
        let text = "The quick brown fox";
        let result = SmartTruncate::auto_truncate(text, 14);
        assert_eq!(result, "The quick ...");
    }

    #[test]
    fn test_truncate_tiny_width() {
        let text = "Hello";
        let result = SmartTruncate::truncate_end(text, 2);
        assert_eq!(result, "...");
    }

    #[test]
    fn test_truncate_unicode() {
        let text = "Hello 👋 World 🌍!";
        let result = SmartTruncate::truncate_end(text, 12);
        assert!(result.ends_with("..."));
        assert!(result.len() < text.len());
    }
}
