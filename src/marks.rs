/// Vim-style marks for bookmarks
///
/// Set and jump to marked positions
///
/// # Examples
///
/// ```
/// use toad::marks::{MarksManager, Mark};
///
/// let mut marks = MarksManager::new();
/// marks.set_mark('a', "file.txt", 10, 5);
/// assert!(marks.get_mark('a').is_some());
/// ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single mark position
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mark {
    /// File path
    pub file: String,
    /// Line number (0-indexed)
    pub line: usize,
    /// Column number (0-indexed)
    pub col: usize,
}

impl Mark {
    /// Create a new mark
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::marks::Mark;
    ///
    /// let mark = Mark::new("file.txt", 10, 5);
    /// assert_eq!(mark.file, "file.txt");
    /// assert_eq!(mark.line, 10);
    /// assert_eq!(mark.col, 5);
    /// ```
    pub fn new(file: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            file: file.into(),
            line,
            col,
        }
    }

    /// Update the position
    pub fn set_position(&mut self, line: usize, col: usize) {
        self.line = line;
        self.col = col;
    }

    /// Check if this mark is in the given file
    pub fn is_in_file(&self, file: &str) -> bool {
        self.file == file
    }
}

/// Mark type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    /// Lowercase marks (a-z) - local to file
    Local,
    /// Uppercase marks (A-Z) - global across files
    Global,
    /// Number marks (0-9) - special marks
    Number,
    /// Special marks (', `, <, >, etc.)
    Special,
}

impl MarkType {
    /// Classify a mark character
    pub fn classify(ch: char) -> Option<Self> {
        match ch {
            'a'..='z' => Some(MarkType::Local),
            'A'..='Z' => Some(MarkType::Global),
            '0'..='9' => Some(MarkType::Number),
            '\'' | '`' | '<' | '>' | '[' | ']' | '^' | '.' => Some(MarkType::Special),
            _ => None,
        }
    }

    /// Check if a character is a valid mark
    pub fn is_valid_mark(ch: char) -> bool {
        Self::classify(ch).is_some()
    }
}

/// Marks manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarksManager {
    /// All marks (key is the mark character)
    marks: HashMap<char, Mark>,
    /// Current file (for local marks)
    current_file: Option<String>,
}

impl MarksManager {
    /// Create a new marks manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::marks::MarksManager;
    ///
    /// let marks = MarksManager::new();
    /// assert_eq!(marks.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            marks: HashMap::new(),
            current_file: None,
        }
    }

    /// Set the current file
    pub fn set_current_file(&mut self, file: impl Into<String>) {
        self.current_file = Some(file.into());
    }

    /// Get the current file
    pub fn current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }

    /// Set a mark at a position
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::marks::MarksManager;
    ///
    /// let mut marks = MarksManager::new();
    /// marks.set_mark('a', "file.txt", 10, 5);
    /// let mark = marks.get_mark('a').unwrap();
    /// assert_eq!(mark.line, 10);
    /// ```
    pub fn set_mark(&mut self, ch: char, file: impl Into<String>, line: usize, col: usize) -> bool {
        if !MarkType::is_valid_mark(ch) {
            return false;
        }

        let mark = Mark::new(file, line, col);
        self.marks.insert(ch, mark);
        true
    }

    /// Set a mark at current file position
    pub fn set_mark_here(&mut self, ch: char, line: usize, col: usize) -> bool {
        if let Some(file) = self.current_file.clone() {
            self.set_mark(ch, file, line, col)
        } else {
            false
        }
    }

    /// Get a mark
    pub fn get_mark(&self, ch: char) -> Option<&Mark> {
        self.marks.get(&ch)
    }

    /// Delete a mark
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::marks::MarksManager;
    ///
    /// let mut marks = MarksManager::new();
    /// marks.set_mark('a', "file.txt", 10, 5);
    /// assert!(marks.get_mark('a').is_some());
    ///
    /// marks.delete_mark('a');
    /// assert!(marks.get_mark('a').is_none());
    /// ```
    pub fn delete_mark(&mut self, ch: char) -> bool {
        self.marks.remove(&ch).is_some()
    }

    /// Delete all marks
    pub fn clear(&mut self) {
        self.marks.clear();
    }

    /// Delete all local marks (a-z) for a specific file
    pub fn clear_local_marks(&mut self, file: &str) {
        self.marks.retain(|&ch, mark| {
            if let Some(MarkType::Local) = MarkType::classify(ch) {
                !mark.is_in_file(file)
            } else {
                true
            }
        });
    }

    /// Get all marks
    pub fn all_marks(&self) -> &HashMap<char, Mark> {
        &self.marks
    }

    /// Get marks for a specific file
    pub fn marks_for_file(&self, file: &str) -> Vec<(char, &Mark)> {
        self.marks
            .iter()
            .filter(|(_, mark)| mark.is_in_file(file))
            .map(|(&ch, mark)| (ch, mark))
            .collect()
    }

    /// Get local marks (a-z)
    pub fn local_marks(&self) -> Vec<(char, &Mark)> {
        self.marks
            .iter()
            .filter(|&(&ch, _)| matches!(MarkType::classify(ch), Some(MarkType::Local)))
            .map(|(&ch, mark)| (ch, mark))
            .collect()
    }

    /// Get global marks (A-Z)
    pub fn global_marks(&self) -> Vec<(char, &Mark)> {
        self.marks
            .iter()
            .filter(|&(&ch, _)| matches!(MarkType::classify(ch), Some(MarkType::Global)))
            .map(|(&ch, mark)| (ch, mark))
            .collect()
    }

    /// Get the number of marks
    pub fn count(&self) -> usize {
        self.marks.len()
    }

    /// Check if a mark exists
    pub fn has_mark(&self, ch: char) -> bool {
        self.marks.contains_key(&ch)
    }

    /// Get all mark characters
    pub fn mark_chars(&self) -> Vec<char> {
        let mut chars: Vec<char> = self.marks.keys().copied().collect();
        chars.sort();
        chars
    }

    /// Save marks to file
    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load marks from file
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let marks = serde_json::from_str(&contents)?;
        Ok(marks)
    }
}

impl Default for MarksManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_creation() {
        let mark = Mark::new("file.txt", 10, 5);
        assert_eq!(mark.file, "file.txt");
        assert_eq!(mark.line, 10);
        assert_eq!(mark.col, 5);
    }

    #[test]
    fn test_mark_set_position() {
        let mut mark = Mark::new("file.txt", 10, 5);
        mark.set_position(20, 10);
        assert_eq!(mark.line, 20);
        assert_eq!(mark.col, 10);
    }

    #[test]
    fn test_mark_is_in_file() {
        let mark = Mark::new("file.txt", 10, 5);
        assert!(mark.is_in_file("file.txt"));
        assert!(!mark.is_in_file("other.txt"));
    }

    #[test]
    fn test_mark_type_classify() {
        assert_eq!(MarkType::classify('a'), Some(MarkType::Local));
        assert_eq!(MarkType::classify('z'), Some(MarkType::Local));
        assert_eq!(MarkType::classify('A'), Some(MarkType::Global));
        assert_eq!(MarkType::classify('Z'), Some(MarkType::Global));
        assert_eq!(MarkType::classify('0'), Some(MarkType::Number));
        assert_eq!(MarkType::classify('9'), Some(MarkType::Number));
        assert_eq!(MarkType::classify('\''), Some(MarkType::Special));
        assert_eq!(MarkType::classify('!'), None);
    }

    #[test]
    fn test_mark_type_is_valid() {
        assert!(MarkType::is_valid_mark('a'));
        assert!(MarkType::is_valid_mark('A'));
        assert!(MarkType::is_valid_mark('0'));
        assert!(MarkType::is_valid_mark('\''));
        assert!(!MarkType::is_valid_mark('!'));
        assert!(!MarkType::is_valid_mark('@'));
    }

    #[test]
    fn test_marks_manager_creation() {
        let marks = MarksManager::new();
        assert_eq!(marks.count(), 0);
        assert!(marks.current_file().is_none());
    }

    #[test]
    fn test_set_current_file() {
        let mut marks = MarksManager::new();
        marks.set_current_file("test.txt");
        assert_eq!(marks.current_file(), Some("test.txt"));
    }

    #[test]
    fn test_set_mark() {
        let mut marks = MarksManager::new();
        assert!(marks.set_mark('a', "file.txt", 10, 5));
        assert_eq!(marks.count(), 1);

        let mark = marks.get_mark('a').unwrap();
        assert_eq!(mark.file, "file.txt");
        assert_eq!(mark.line, 10);
        assert_eq!(mark.col, 5);
    }

    #[test]
    fn test_set_invalid_mark() {
        let mut marks = MarksManager::new();
        assert!(!marks.set_mark('!', "file.txt", 10, 5));
        assert_eq!(marks.count(), 0);
    }

    #[test]
    fn test_set_mark_here() {
        let mut marks = MarksManager::new();
        marks.set_current_file("test.txt");
        assert!(marks.set_mark_here('a', 10, 5));

        let mark = marks.get_mark('a').unwrap();
        assert_eq!(mark.file, "test.txt");
    }

    #[test]
    fn test_set_mark_here_no_file() {
        let mut marks = MarksManager::new();
        assert!(!marks.set_mark_here('a', 10, 5));
    }

    #[test]
    fn test_delete_mark() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5);
        assert!(marks.has_mark('a'));

        assert!(marks.delete_mark('a'));
        assert!(!marks.has_mark('a'));
        assert!(!marks.delete_mark('a'));
    }

    #[test]
    fn test_clear() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5);
        marks.set_mark('b', "file.txt", 20, 10);
        assert_eq!(marks.count(), 2);

        marks.clear();
        assert_eq!(marks.count(), 0);
    }

    #[test]
    fn test_clear_local_marks() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file1.txt", 10, 5);
        marks.set_mark('b', "file1.txt", 20, 10);
        marks.set_mark('c', "file2.txt", 30, 15);
        marks.set_mark('A', "file1.txt", 40, 20); // Global mark
        assert_eq!(marks.count(), 4);

        marks.clear_local_marks("file1.txt");
        assert_eq!(marks.count(), 2); // 'c' and 'A' remain
        assert!(!marks.has_mark('a'));
        assert!(!marks.has_mark('b'));
        assert!(marks.has_mark('c'));
        assert!(marks.has_mark('A'));
    }

    #[test]
    fn test_marks_for_file() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file1.txt", 10, 5);
        marks.set_mark('b', "file2.txt", 20, 10);
        marks.set_mark('c', "file1.txt", 30, 15);

        let file1_marks = marks.marks_for_file("file1.txt");
        assert_eq!(file1_marks.len(), 2);

        let file2_marks = marks.marks_for_file("file2.txt");
        assert_eq!(file2_marks.len(), 1);
    }

    #[test]
    fn test_local_marks() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5);
        marks.set_mark('b', "file.txt", 20, 10);
        marks.set_mark('A', "file.txt", 30, 15); // Global
        marks.set_mark('0', "file.txt", 40, 20); // Number

        let local = marks.local_marks();
        assert_eq!(local.len(), 2);
    }

    #[test]
    fn test_global_marks() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5); // Local
        marks.set_mark('A', "file.txt", 20, 10);
        marks.set_mark('B', "file.txt", 30, 15);

        let global = marks.global_marks();
        assert_eq!(global.len(), 2);
    }

    #[test]
    fn test_mark_chars() {
        let mut marks = MarksManager::new();
        marks.set_mark('c', "file.txt", 10, 5);
        marks.set_mark('a', "file.txt", 20, 10);
        marks.set_mark('b', "file.txt", 30, 15);

        let chars = marks.mark_chars();
        assert_eq!(chars, vec!['a', 'b', 'c']); // Sorted
    }

    #[test]
    fn test_has_mark() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5);

        assert!(marks.has_mark('a'));
        assert!(!marks.has_mark('b'));
    }

    #[test]
    fn test_all_marks() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5);
        marks.set_mark('b', "file.txt", 20, 10);

        let all = marks.all_marks();
        assert_eq!(all.len(), 2);
        assert!(all.contains_key(&'a'));
        assert!(all.contains_key(&'b'));
    }

    #[test]
    fn test_default() {
        let marks = MarksManager::default();
        assert_eq!(marks.count(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file.txt", 10, 5);
        marks.set_mark('b', "file.txt", 20, 10);

        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_marks.json");

        // Save
        marks.save_to_file(&path).unwrap();

        // Load
        let loaded = MarksManager::load_from_file(&path).unwrap();
        assert_eq!(loaded.count(), 2);
        assert!(loaded.has_mark('a'));
        assert!(loaded.has_mark('b'));

        // Cleanup
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_overwrite_mark() {
        let mut marks = MarksManager::new();
        marks.set_mark('a', "file1.txt", 10, 5);

        let mark = marks.get_mark('a').unwrap();
        assert_eq!(mark.file, "file1.txt");
        assert_eq!(mark.line, 10);

        // Overwrite
        marks.set_mark('a', "file2.txt", 20, 10);

        let mark = marks.get_mark('a').unwrap();
        assert_eq!(mark.file, "file2.txt");
        assert_eq!(mark.line, 20);
    }
}
