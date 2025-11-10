/// Git-style diff highlighting
///
/// Parses and highlights unified diff format
///
/// # Examples
///
/// ```
/// use toad::diff::{DiffParser, DiffLineType};
///
/// let diff_text = "--- a/file.txt\n+++ b/file.txt\n@@ -1,3 +1,3 @@\n context\n-removed\n+added";
/// let parser = DiffParser::new();
/// let hunks = parser.parse(diff_text);
/// assert_eq!(hunks.len(), 1);
/// ```
use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of diff line
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineType {
    /// File header (---)
    FileHeader,
    /// File header (+++)
    FileHeaderNew,
    /// Chunk header (@@ -x,y +a,b @@)
    ChunkHeader,
    /// Added line (+)
    Added,
    /// Removed line (-)
    Removed,
    /// Context line (space)
    Context,
    /// No newline at end of file
    NoNewline,
}

impl DiffLineType {
    /// Get the color for this line type
    pub fn color(&self) -> &'static str {
        match self {
            DiffLineType::FileHeader | DiffLineType::FileHeaderNew => "cyan",
            DiffLineType::ChunkHeader => "blue",
            DiffLineType::Added => "green",
            DiffLineType::Removed => "red",
            DiffLineType::Context => "white",
            DiffLineType::NoNewline => "yellow",
        }
    }

    /// Get the prefix character for this line type
    pub fn prefix(&self) -> Option<char> {
        match self {
            DiffLineType::Added => Some('+'),
            DiffLineType::Removed => Some('-'),
            DiffLineType::Context => Some(' '),
            _ => None,
        }
    }
}

impl fmt::Display for DiffLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffLineType::FileHeader => write!(f, "FileHeader"),
            DiffLineType::FileHeaderNew => write!(f, "FileHeaderNew"),
            DiffLineType::ChunkHeader => write!(f, "ChunkHeader"),
            DiffLineType::Added => write!(f, "Added"),
            DiffLineType::Removed => write!(f, "Removed"),
            DiffLineType::Context => write!(f, "Context"),
            DiffLineType::NoNewline => write!(f, "NoNewline"),
        }
    }
}

/// A single line in a diff
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffLine {
    /// Line type
    pub line_type: DiffLineType,
    /// Line content (without prefix character)
    pub content: String,
    /// Original line number (for removed/context lines)
    pub old_line: Option<usize>,
    /// New line number (for added/context lines)
    pub new_line: Option<usize>,
}

impl DiffLine {
    /// Create a new diff line
    pub fn new(line_type: DiffLineType, content: impl Into<String>) -> Self {
        Self {
            line_type,
            content: content.into(),
            old_line: None,
            new_line: None,
        }
    }

    /// Create a diff line with line numbers
    pub fn with_line_numbers(
        line_type: DiffLineType,
        content: impl Into<String>,
        old_line: Option<usize>,
        new_line: Option<usize>,
    ) -> Self {
        Self {
            line_type,
            content: content.into(),
            old_line,
            new_line,
        }
    }

    /// Get the full line with prefix
    pub fn full_line(&self) -> String {
        if let Some(prefix) = self.line_type.prefix() {
            format!("{}{}", prefix, self.content)
        } else {
            self.content.clone()
        }
    }

    /// Check if this is an added line
    pub fn is_added(&self) -> bool {
        self.line_type == DiffLineType::Added
    }

    /// Check if this is a removed line
    pub fn is_removed(&self) -> bool {
        self.line_type == DiffLineType::Removed
    }

    /// Check if this is a context line
    pub fn is_context(&self) -> bool {
        self.line_type == DiffLineType::Context
    }

    /// Check if this is a header line
    pub fn is_header(&self) -> bool {
        matches!(
            self.line_type,
            DiffLineType::FileHeader | DiffLineType::FileHeaderNew | DiffLineType::ChunkHeader
        )
    }
}

/// Chunk header information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkHeader {
    /// Old file start line
    pub old_start: usize,
    /// Old file line count
    pub old_count: usize,
    /// New file start line
    pub new_start: usize,
    /// New file line count
    pub new_count: usize,
    /// Optional section header
    pub section: Option<String>,
}

impl ChunkHeader {
    /// Parse a chunk header from a string like "@@ -1,3 +1,4 @@ section"
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if !line.starts_with("@@") {
            return None;
        }

        // Find the second @@
        let parts: Vec<&str> = line.splitn(3, "@@").collect();
        if parts.len() < 2 {
            return None;
        }

        let ranges = parts[1].trim();
        let section = if parts.len() > 2 {
            let s = parts[2].trim();
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        } else {
            None
        };

        // Parse ranges: "-old_start,old_count +new_start,new_count"
        let range_parts: Vec<&str> = ranges.split_whitespace().collect();
        if range_parts.len() != 2 {
            return None;
        }

        let old_range = range_parts[0].strip_prefix('-')?;
        let new_range = range_parts[1].strip_prefix('+')?;

        let (old_start, old_count) = Self::parse_range(old_range)?;
        let (new_start, new_count) = Self::parse_range(new_range)?;

        Some(ChunkHeader {
            old_start,
            old_count,
            new_start,
            new_count,
            section,
        })
    }

    fn parse_range(range: &str) -> Option<(usize, usize)> {
        if let Some((start, count)) = range.split_once(',') {
            let start = start.parse().ok()?;
            let count = count.parse().ok()?;
            Some((start, count))
        } else {
            // Single line: "1" means "1,1"
            let start = range.parse().ok()?;
            Some((start, 1))
        }
    }

    /// Format as a chunk header line
    pub fn format(&self) -> String {
        let header = format!(
            "@@ -{},{} +{},{} @@",
            self.old_start, self.old_count, self.new_start, self.new_count
        );

        if let Some(ref section) = self.section {
            format!("{} {}", header, section)
        } else {
            header
        }
    }
}

/// A hunk (chunk) of diff
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffHunk {
    /// Chunk header
    pub header: ChunkHeader,
    /// Lines in this hunk
    pub lines: Vec<DiffLine>,
}

impl DiffHunk {
    /// Create a new diff hunk
    pub fn new(header: ChunkHeader) -> Self {
        Self {
            header,
            lines: Vec::new(),
        }
    }

    /// Add a line to the hunk
    pub fn add_line(&mut self, line: DiffLine) {
        self.lines.push(line);
    }

    /// Get the number of added lines
    pub fn added_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_added()).count()
    }

    /// Get the number of removed lines
    pub fn removed_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_removed()).count()
    }

    /// Get the number of context lines
    pub fn context_count(&self) -> usize {
        self.lines.iter().filter(|l| l.is_context()).count()
    }
}

/// File diff information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileDiff {
    /// Old file path
    pub old_path: String,
    /// New file path
    pub new_path: String,
    /// All hunks for this file
    pub hunks: Vec<DiffHunk>,
}

impl FileDiff {
    /// Create a new file diff
    pub fn new(old_path: impl Into<String>, new_path: impl Into<String>) -> Self {
        Self {
            old_path: old_path.into(),
            new_path: new_path.into(),
            hunks: Vec::new(),
        }
    }

    /// Add a hunk to this file diff
    pub fn add_hunk(&mut self, hunk: DiffHunk) {
        self.hunks.push(hunk);
    }

    /// Get total added lines across all hunks
    pub fn total_added(&self) -> usize {
        self.hunks.iter().map(|h| h.added_count()).sum()
    }

    /// Get total removed lines across all hunks
    pub fn total_removed(&self) -> usize {
        self.hunks.iter().map(|h| h.removed_count()).sum()
    }

    /// Check if this is a new file
    pub fn is_new_file(&self) -> bool {
        self.old_path == "/dev/null" || self.old_path.is_empty()
    }

    /// Check if this is a deleted file
    pub fn is_deleted_file(&self) -> bool {
        self.new_path == "/dev/null" || self.new_path.is_empty()
    }
}

/// Diff parser for unified diff format
#[derive(Debug, Clone, Default)]
pub struct DiffParser {
    /// Whether to track line numbers
    track_line_numbers: bool,
}

impl DiffParser {
    /// Create a new diff parser
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::diff::DiffParser;
    ///
    /// let parser = DiffParser::new();
    /// ```
    pub fn new() -> Self {
        Self {
            track_line_numbers: true,
        }
    }

    /// Set whether to track line numbers
    pub fn track_line_numbers(mut self, track: bool) -> Self {
        self.track_line_numbers = track;
        self
    }

    /// Parse a unified diff into file diffs
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::diff::DiffParser;
    ///
    /// let diff = "--- a/file.txt\n+++ b/file.txt\n@@ -1 +1 @@\n-old\n+new";
    /// let parser = DiffParser::new();
    /// let files = parser.parse_files(diff);
    /// assert_eq!(files.len(), 1);
    /// ```
    pub fn parse_files(&self, diff: &str) -> Vec<FileDiff> {
        let mut files = Vec::new();
        let mut current_file: Option<FileDiff> = None;
        let mut current_hunk: Option<DiffHunk> = None;
        let mut old_line = 0;
        let mut new_line = 0;

        for line in diff.lines() {
            if line.starts_with("--- ") {
                // Save previous file if any
                if let Some(mut file) = current_file.take() {
                    if let Some(hunk) = current_hunk.take() {
                        file.add_hunk(hunk);
                    }
                    files.push(file);
                }

                // Start new file
                let old_path = line.strip_prefix("--- ").unwrap_or("").to_string();
                current_file = Some(FileDiff::new(old_path, ""));
            } else if line.starts_with("+++ ") {
                if let Some(file) = current_file.as_mut() {
                    file.new_path = line.strip_prefix("+++ ").unwrap_or("").to_string();
                }
            } else if line.starts_with("@@ ") {
                // Save previous hunk if any
                if let Some(hunk) = current_hunk.take()
                    && let Some(file) = current_file.as_mut()
                {
                    file.add_hunk(hunk);
                }

                // Create default file if we don't have one (for hunks without file headers)
                if current_file.is_none() {
                    current_file = Some(FileDiff::new("", ""));
                }

                // Parse chunk header
                if let Some(header) = ChunkHeader::parse(line) {
                    old_line = header.old_start;
                    new_line = header.new_start;
                    current_hunk = Some(DiffHunk::new(header));
                }
            } else if line.starts_with('+') && !line.starts_with("+++") {
                // Added line
                let content = line.strip_prefix('+').unwrap_or("");
                let diff_line = if self.track_line_numbers {
                    let line = DiffLine::with_line_numbers(
                        DiffLineType::Added,
                        content,
                        None,
                        Some(new_line),
                    );
                    new_line += 1;
                    line
                } else {
                    DiffLine::new(DiffLineType::Added, content)
                };

                if let Some(hunk) = current_hunk.as_mut() {
                    hunk.add_line(diff_line);
                }
            } else if line.starts_with('-') && !line.starts_with("---") {
                // Removed line
                let content = line.strip_prefix('-').unwrap_or("");
                let diff_line = if self.track_line_numbers {
                    let line = DiffLine::with_line_numbers(
                        DiffLineType::Removed,
                        content,
                        Some(old_line),
                        None,
                    );
                    old_line += 1;
                    line
                } else {
                    DiffLine::new(DiffLineType::Removed, content)
                };

                if let Some(hunk) = current_hunk.as_mut() {
                    hunk.add_line(diff_line);
                }
            } else if line.starts_with(' ') || (!line.starts_with("\\") && current_hunk.is_some()) {
                // Context line
                let content = line.strip_prefix(' ').unwrap_or(line);
                let diff_line = if self.track_line_numbers {
                    let line = DiffLine::with_line_numbers(
                        DiffLineType::Context,
                        content,
                        Some(old_line),
                        Some(new_line),
                    );
                    old_line += 1;
                    new_line += 1;
                    line
                } else {
                    DiffLine::new(DiffLineType::Context, content)
                };

                if let Some(hunk) = current_hunk.as_mut() {
                    hunk.add_line(diff_line);
                }
            } else if line.starts_with("\\ No newline at end of file") {
                // No newline indicator
                let diff_line = DiffLine::new(DiffLineType::NoNewline, line);
                if let Some(hunk) = current_hunk.as_mut() {
                    hunk.add_line(diff_line);
                }
            }
        }

        // Save final file
        if let Some(mut file) = current_file {
            if let Some(hunk) = current_hunk {
                file.add_hunk(hunk);
            }
            files.push(file);
        }

        files
    }

    /// Parse a diff into hunks (legacy method for backward compatibility)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::diff::DiffParser;
    ///
    /// let diff = "@@ -1 +1 @@\n-old\n+new";
    /// let parser = DiffParser::new();
    /// let hunks = parser.parse(diff);
    /// assert_eq!(hunks.len(), 1);
    /// ```
    pub fn parse(&self, diff: &str) -> Vec<DiffHunk> {
        let files = self.parse_files(diff);
        files.into_iter().flat_map(|f| f.hunks).collect()
    }
}

/// Statistics about a diff
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffStats {
    /// Number of files changed
    pub files_changed: usize,
    /// Total lines added
    pub lines_added: usize,
    /// Total lines removed
    pub lines_removed: usize,
}

impl DiffStats {
    /// Calculate statistics from file diffs
    pub fn from_files(files: &[FileDiff]) -> Self {
        Self {
            files_changed: files.len(),
            lines_added: files.iter().map(|f| f.total_added()).sum(),
            lines_removed: files.iter().map(|f| f.total_removed()).sum(),
        }
    }

    /// Get total lines changed
    pub fn total_changed(&self) -> usize {
        self.lines_added + self.lines_removed
    }

    /// Format as a git-style summary
    pub fn format(&self) -> String {
        format!(
            "{} file{} changed, {} insertion{}(+), {} deletion{}(-)",
            self.files_changed,
            if self.files_changed == 1 { "" } else { "s" },
            self.lines_added,
            if self.lines_added == 1 { "" } else { "s" },
            self.lines_removed,
            if self.lines_removed == 1 { "" } else { "s" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_line_type_color() {
        assert_eq!(DiffLineType::Added.color(), "green");
        assert_eq!(DiffLineType::Removed.color(), "red");
        assert_eq!(DiffLineType::Context.color(), "white");
        assert_eq!(DiffLineType::ChunkHeader.color(), "blue");
    }

    #[test]
    fn test_diff_line_type_prefix() {
        assert_eq!(DiffLineType::Added.prefix(), Some('+'));
        assert_eq!(DiffLineType::Removed.prefix(), Some('-'));
        assert_eq!(DiffLineType::Context.prefix(), Some(' '));
        assert_eq!(DiffLineType::FileHeader.prefix(), None);
    }

    #[test]
    fn test_diff_line_creation() {
        let line = DiffLine::new(DiffLineType::Added, "new content");
        assert_eq!(line.line_type, DiffLineType::Added);
        assert_eq!(line.content, "new content");
        assert!(line.is_added());
        assert!(!line.is_removed());
    }

    #[test]
    fn test_diff_line_full_line() {
        let line = DiffLine::new(DiffLineType::Added, "content");
        assert_eq!(line.full_line(), "+content");

        let line = DiffLine::new(DiffLineType::Removed, "content");
        assert_eq!(line.full_line(), "-content");

        let line = DiffLine::new(DiffLineType::FileHeader, "--- a/file");
        assert_eq!(line.full_line(), "--- a/file");
    }

    #[test]
    fn test_chunk_header_parse() {
        let header = ChunkHeader::parse("@@ -1,3 +1,4 @@ section").unwrap();
        assert_eq!(header.old_start, 1);
        assert_eq!(header.old_count, 3);
        assert_eq!(header.new_start, 1);
        assert_eq!(header.new_count, 4);
        assert_eq!(header.section, Some("section".to_string()));
    }

    #[test]
    fn test_chunk_header_parse_no_section() {
        let header = ChunkHeader::parse("@@ -10,5 +12,6 @@").unwrap();
        assert_eq!(header.old_start, 10);
        assert_eq!(header.old_count, 5);
        assert_eq!(header.new_start, 12);
        assert_eq!(header.new_count, 6);
        assert_eq!(header.section, None);
    }

    #[test]
    fn test_chunk_header_parse_single_line() {
        let header = ChunkHeader::parse("@@ -1 +1 @@").unwrap();
        assert_eq!(header.old_start, 1);
        assert_eq!(header.old_count, 1);
        assert_eq!(header.new_start, 1);
        assert_eq!(header.new_count, 1);
    }

    #[test]
    fn test_chunk_header_format() {
        let header = ChunkHeader {
            old_start: 1,
            old_count: 3,
            new_start: 1,
            new_count: 4,
            section: Some("function".to_string()),
        };
        assert_eq!(header.format(), "@@ -1,3 +1,4 @@ function");
    }

    #[test]
    fn test_diff_hunk() {
        let header = ChunkHeader {
            old_start: 1,
            old_count: 2,
            new_start: 1,
            new_count: 2,
            section: None,
        };
        let mut hunk = DiffHunk::new(header);

        hunk.add_line(DiffLine::new(DiffLineType::Context, "context"));
        hunk.add_line(DiffLine::new(DiffLineType::Removed, "old"));
        hunk.add_line(DiffLine::new(DiffLineType::Added, "new"));

        assert_eq!(hunk.lines.len(), 3);
        assert_eq!(hunk.context_count(), 1);
        assert_eq!(hunk.removed_count(), 1);
        assert_eq!(hunk.added_count(), 1);
    }

    #[test]
    fn test_file_diff() {
        let mut file = FileDiff::new("a/file.txt", "b/file.txt");

        let header = ChunkHeader {
            old_start: 1,
            old_count: 1,
            new_start: 1,
            new_count: 1,
            section: None,
        };
        let mut hunk = DiffHunk::new(header);
        hunk.add_line(DiffLine::new(DiffLineType::Removed, "old"));
        hunk.add_line(DiffLine::new(DiffLineType::Added, "new"));

        file.add_hunk(hunk);

        assert_eq!(file.total_removed(), 1);
        assert_eq!(file.total_added(), 1);
        assert!(!file.is_new_file());
        assert!(!file.is_deleted_file());
    }

    #[test]
    fn test_file_diff_new_file() {
        let file = FileDiff::new("/dev/null", "b/new.txt");
        assert!(file.is_new_file());
        assert!(!file.is_deleted_file());
    }

    #[test]
    fn test_file_diff_deleted_file() {
        let file = FileDiff::new("a/old.txt", "/dev/null");
        assert!(!file.is_new_file());
        assert!(file.is_deleted_file());
    }

    #[test]
    fn test_parser_simple_diff() {
        let diff = "@@ -1 +1 @@\n-old line\n+new line";
        let parser = DiffParser::new();
        let hunks = parser.parse(diff);

        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].lines.len(), 2);
        assert_eq!(hunks[0].removed_count(), 1);
        assert_eq!(hunks[0].added_count(), 1);
    }

    #[test]
    fn test_parser_with_context() {
        let diff = "@@ -1,3 +1,3 @@\n context\n-removed\n+added";
        let parser = DiffParser::new();
        let hunks = parser.parse(diff);

        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].lines.len(), 3);
        assert_eq!(hunks[0].context_count(), 1);
        assert_eq!(hunks[0].removed_count(), 1);
        assert_eq!(hunks[0].added_count(), 1);
    }

    #[test]
    fn test_parser_multiple_hunks() {
        let diff = "@@ -1 +1 @@\n-old1\n+new1\n@@ -10 +10 @@\n-old2\n+new2";
        let parser = DiffParser::new();
        let hunks = parser.parse(diff);

        assert_eq!(hunks.len(), 2);
        assert_eq!(hunks[0].lines.len(), 2);
        assert_eq!(hunks[1].lines.len(), 2);
    }

    #[test]
    fn test_parser_full_diff() {
        let diff = "--- a/file.txt\n+++ b/file.txt\n@@ -1,2 +1,2 @@\n context\n-old\n+new";
        let parser = DiffParser::new();
        let files = parser.parse_files(diff);

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].old_path, "a/file.txt");
        assert_eq!(files[0].new_path, "b/file.txt");
        assert_eq!(files[0].hunks.len(), 1);
        assert_eq!(files[0].hunks[0].lines.len(), 3);
    }

    #[test]
    fn test_parser_multiple_files() {
        let diff = "--- a/file1.txt\n+++ b/file1.txt\n@@ -1 +1 @@\n-old1\n+new1\n--- a/file2.txt\n+++ b/file2.txt\n@@ -1 +1 @@\n-old2\n+new2";
        let parser = DiffParser::new();
        let files = parser.parse_files(diff);

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].old_path, "a/file1.txt");
        assert_eq!(files[1].old_path, "a/file2.txt");
    }

    #[test]
    fn test_parser_with_line_numbers() {
        let diff = "@@ -5,2 +5,2 @@\n context\n-removed\n+added";
        let parser = DiffParser::new().track_line_numbers(true);
        let hunks = parser.parse(diff);

        assert_eq!(hunks.len(), 1);
        let lines = &hunks[0].lines;
        assert_eq!(lines[0].old_line, Some(5));
        assert_eq!(lines[0].new_line, Some(5));
        assert_eq!(lines[1].old_line, Some(6));
        assert_eq!(lines[1].new_line, None);
        assert_eq!(lines[2].old_line, None);
        assert_eq!(lines[2].new_line, Some(6));
    }

    #[test]
    fn test_parser_no_newline() {
        let diff = "@@ -1 +1 @@\n-old\n+new\n\\ No newline at end of file";
        let parser = DiffParser::new();
        let hunks = parser.parse(diff);

        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].lines.len(), 3);
        assert_eq!(hunks[0].lines[2].line_type, DiffLineType::NoNewline);
    }

    #[test]
    fn test_diff_stats() {
        let mut file1 = FileDiff::new("a/file1.txt", "b/file1.txt");
        let mut hunk1 = DiffHunk::new(ChunkHeader {
            old_start: 1,
            old_count: 1,
            new_start: 1,
            new_count: 2,
            section: None,
        });
        hunk1.add_line(DiffLine::new(DiffLineType::Removed, "old"));
        hunk1.add_line(DiffLine::new(DiffLineType::Added, "new1"));
        hunk1.add_line(DiffLine::new(DiffLineType::Added, "new2"));
        file1.add_hunk(hunk1);

        let mut file2 = FileDiff::new("a/file2.txt", "b/file2.txt");
        let mut hunk2 = DiffHunk::new(ChunkHeader {
            old_start: 1,
            old_count: 2,
            new_start: 1,
            new_count: 1,
            section: None,
        });
        hunk2.add_line(DiffLine::new(DiffLineType::Removed, "old1"));
        hunk2.add_line(DiffLine::new(DiffLineType::Removed, "old2"));
        hunk2.add_line(DiffLine::new(DiffLineType::Added, "new"));
        file2.add_hunk(hunk2);

        let files = vec![file1, file2];
        let stats = DiffStats::from_files(&files);

        assert_eq!(stats.files_changed, 2);
        assert_eq!(stats.lines_added, 3);
        assert_eq!(stats.lines_removed, 3);
        assert_eq!(stats.total_changed(), 6);
    }

    #[test]
    fn test_diff_stats_format() {
        let stats = DiffStats {
            files_changed: 2,
            lines_added: 5,
            lines_removed: 3,
        };
        assert_eq!(
            stats.format(),
            "2 files changed, 5 insertions(+), 3 deletions(-)"
        );

        let stats = DiffStats {
            files_changed: 1,
            lines_added: 1,
            lines_removed: 1,
        };
        assert_eq!(
            stats.format(),
            "1 file changed, 1 insertion(+), 1 deletion(-)"
        );
    }

    #[test]
    fn test_chunk_header_parse_invalid() {
        assert!(ChunkHeader::parse("not a header").is_none());
        assert!(ChunkHeader::parse("@@ invalid @@").is_none());
        assert!(ChunkHeader::parse("@@").is_none());
    }

    #[test]
    fn test_diff_line_is_header() {
        let file_header = DiffLine::new(DiffLineType::FileHeader, "--- a/file");
        assert!(file_header.is_header());

        let chunk_header = DiffLine::new(DiffLineType::ChunkHeader, "@@ -1 +1 @@");
        assert!(chunk_header.is_header());

        let added = DiffLine::new(DiffLineType::Added, "content");
        assert!(!added.is_header());
    }
}
