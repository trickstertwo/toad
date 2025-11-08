/// Vim-style motion commands
///
/// Word movement (w/b/e) and character jumping (f/t)
///
/// # Examples
///
/// ```
/// use toad::vim_motions::{VimMotions, Motion};
///
/// let text = "hello world test";
/// let motions = VimMotions::new(text);
///
/// // Find next word start from position 0
/// assert_eq!(motions.word_forward(0), Some(6));
/// ```

use serde::{Deserialize, Serialize};

/// Vim motion type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    /// w - word forward
    WordForward,
    /// b - word backward
    WordBackward,
    /// e - end of word
    EndOfWord,
    /// W - WORD forward (whitespace-delimited)
    WORDForward,
    /// B - WORD backward (whitespace-delimited)
    WORDBackward,
    /// E - end of WORD
    EndOfWORD,
    /// f{char} - find character forward
    FindChar(char),
    /// F{char} - find character backward
    FindCharBackward(char),
    /// t{char} - till character forward (before char)
    TillChar(char),
    /// T{char} - till character backward (after char)
    TillCharBackward(char),
}

/// Vim motions engine
#[derive(Debug, Clone)]
pub struct VimMotions {
    /// The text to operate on
    text: String,
}

impl VimMotions {
    /// Create a new vim motions engine
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world");
    /// assert_eq!(motions.text(), "hello world");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    /// Get the text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Check if character is a word character (alphanumeric or underscore)
    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    /// Check if character is whitespace
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    /// Move forward to the start of the next word (w motion)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world test");
    /// assert_eq!(motions.word_forward(0), Some(6));
    /// assert_eq!(motions.word_forward(6), Some(12));
    /// ```
    pub fn word_forward(&self, pos: usize) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos >= chars.len() {
            return None;
        }

        let mut i = pos;
        let current_is_word = Self::is_word_char(chars[i]);

        // Skip current word
        while i < chars.len() && Self::is_word_char(chars[i]) == current_is_word && !Self::is_whitespace(chars[i]) {
            i += 1;
        }

        // Skip whitespace
        while i < chars.len() && Self::is_whitespace(chars[i]) {
            i += 1;
        }

        if i < chars.len() && i != pos {
            Some(i)
        } else {
            None
        }
    }

    /// Move backward to the start of the previous word (b motion)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world test");
    /// assert_eq!(motions.word_backward(12), Some(6));
    /// assert_eq!(motions.word_backward(6), Some(0));
    /// ```
    pub fn word_backward(&self, pos: usize) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos == 0 || chars.is_empty() {
            return None;
        }

        let mut i = if pos >= chars.len() { chars.len() - 1 } else { pos };

        // If we're at the start of a word, move back one position first
        if i > 0 && i < chars.len() && !Self::is_whitespace(chars[i]) && (i == 0 || Self::is_whitespace(chars[i - 1])) {
            i -= 1;
        }

        // Skip whitespace going backward
        while i > 0 && Self::is_whitespace(chars[i]) {
            i -= 1;
        }

        if i == 0 {
            return if i != pos { Some(0) } else { None };
        }

        // Now we're on a non-whitespace character
        // Skip to start of this word
        while i > 0 {
            if Self::is_whitespace(chars[i - 1]) {
                break;
            }
            if Self::is_word_char(chars[i]) != Self::is_word_char(chars[i - 1]) {
                break;
            }
            i -= 1;
        }

        if i != pos {
            Some(i)
        } else {
            None
        }
    }

    /// Move to the end of the current/next word (e motion)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world test");
    /// assert_eq!(motions.end_of_word(0), Some(4));
    /// assert_eq!(motions.end_of_word(6), Some(10));
    /// ```
    pub fn end_of_word(&self, pos: usize) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos >= chars.len() {
            return None;
        }

        let mut i = pos;

        // If on whitespace, skip to next word
        while i < chars.len() && Self::is_whitespace(chars[i]) {
            i += 1;
        }

        if i >= chars.len() {
            return None;
        }

        // If at start of word, move to end
        let is_word = Self::is_word_char(chars[i]);
        while i < chars.len() - 1 {
            let next_char = chars[i + 1];
            if Self::is_whitespace(next_char) {
                break;
            }
            if Self::is_word_char(next_char) != is_word {
                break;
            }
            i += 1;
        }

        if i != pos {
            Some(i)
        } else {
            None
        }
    }

    /// Move forward to the start of the next WORD (W motion)
    /// WORD is whitespace-delimited
    pub fn word_forward_ws(&self, pos: usize) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos >= chars.len() {
            return None;
        }

        let mut i = pos;

        // Skip non-whitespace
        while i < chars.len() && !Self::is_whitespace(chars[i]) {
            i += 1;
        }

        // Skip whitespace
        while i < chars.len() && Self::is_whitespace(chars[i]) {
            i += 1;
        }

        if i < chars.len() && i != pos {
            Some(i)
        } else {
            None
        }
    }

    /// Move backward to the start of the previous WORD (B motion)
    pub fn word_backward_ws(&self, pos: usize) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos == 0 || chars.is_empty() {
            return None;
        }

        let mut i = if pos >= chars.len() { chars.len() - 1 } else { pos };

        // If we're at the start of a WORD, move back one position first
        if i > 0 && i < chars.len() && !Self::is_whitespace(chars[i]) && (i == 0 || Self::is_whitespace(chars[i - 1])) {
            i -= 1;
        }

        // Skip whitespace
        while i > 0 && Self::is_whitespace(chars[i]) {
            i -= 1;
        }

        if i == 0 {
            return if i != pos { Some(0) } else { None };
        }

        // Skip to start of WORD
        while i > 0 && !Self::is_whitespace(chars[i - 1]) {
            i -= 1;
        }

        if i != pos {
            Some(i)
        } else {
            None
        }
    }

    /// Move to the end of the current/next WORD (E motion)
    pub fn end_of_word_ws(&self, pos: usize) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos >= chars.len() {
            return None;
        }

        let mut i = pos;

        // Skip whitespace
        while i < chars.len() && Self::is_whitespace(chars[i]) {
            i += 1;
        }

        if i >= chars.len() {
            return None;
        }

        // Move to end of WORD
        while i < chars.len() - 1 && !Self::is_whitespace(chars[i + 1]) {
            i += 1;
        }

        if i != pos {
            Some(i)
        } else {
            None
        }
    }

    /// Find character forward (f motion)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world");
    /// assert_eq!(motions.find_char(0, 'w'), Some(6));
    /// assert_eq!(motions.find_char(0, 'o'), Some(4));
    /// assert_eq!(motions.find_char(0, 'z'), None);
    /// ```
    pub fn find_char(&self, pos: usize, ch: char) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos >= chars.len() {
            return None;
        }

        for i in (pos + 1)..chars.len() {
            if chars[i] == ch {
                return Some(i);
            }
        }

        None
    }

    /// Find character backward (F motion)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world");
    /// assert_eq!(motions.find_char_backward(10, 'h'), Some(0));
    /// assert_eq!(motions.find_char_backward(10, 'l'), Some(9));
    /// ```
    pub fn find_char_backward(&self, pos: usize, ch: char) -> Option<usize> {
        let chars: Vec<char> = self.text.chars().collect();
        if pos == 0 || chars.is_empty() {
            return None;
        }

        for i in (0..pos).rev() {
            if chars[i] == ch {
                return Some(i);
            }
        }

        None
    }

    /// Till character forward (t motion) - stops before the character
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world");
    /// assert_eq!(motions.till_char(0, 'w'), Some(5));
    /// ```
    pub fn till_char(&self, pos: usize, ch: char) -> Option<usize> {
        self.find_char(pos, ch).and_then(|i| {
            if i > 0 {
                Some(i - 1)
            } else {
                None
            }
        })
    }

    /// Till character backward (T motion) - stops after the character
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::vim_motions::VimMotions;
    ///
    /// let motions = VimMotions::new("hello world");
    /// assert_eq!(motions.till_char_backward(10, 'h'), Some(1));
    /// ```
    pub fn till_char_backward(&self, pos: usize, ch: char) -> Option<usize> {
        self.find_char_backward(pos, ch).map(|i| i + 1)
    }

    /// Execute a motion and return the new position
    pub fn execute(&self, pos: usize, motion: Motion) -> Option<usize> {
        match motion {
            Motion::WordForward => self.word_forward(pos),
            Motion::WordBackward => self.word_backward(pos),
            Motion::EndOfWord => self.end_of_word(pos),
            Motion::WORDForward => self.word_forward_ws(pos),
            Motion::WORDBackward => self.word_backward_ws(pos),
            Motion::EndOfWORD => self.end_of_word_ws(pos),
            Motion::FindChar(ch) => self.find_char(pos, ch),
            Motion::FindCharBackward(ch) => self.find_char_backward(pos, ch),
            Motion::TillChar(ch) => self.till_char(pos, ch),
            Motion::TillCharBackward(ch) => self.till_char_backward(pos, ch),
        }
    }

    /// Execute a motion with count (repeat n times)
    pub fn execute_with_count(&self, pos: usize, motion: Motion, count: usize) -> Option<usize> {
        let mut current_pos = pos;

        for _ in 0..count.max(1) {
            if let Some(new_pos) = self.execute(current_pos, motion) {
                current_pos = new_pos;
            } else {
                break;
            }
        }

        if current_pos != pos {
            Some(current_pos)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_forward() {
        let motions = VimMotions::new("hello world test");
        assert_eq!(motions.word_forward(0), Some(6));
        assert_eq!(motions.word_forward(6), Some(12));
        assert_eq!(motions.word_forward(12), None);
    }

    #[test]
    fn test_word_forward_with_punctuation() {
        let motions = VimMotions::new("hello, world!");
        assert_eq!(motions.word_forward(0), Some(5));
        assert_eq!(motions.word_forward(5), Some(7));
    }

    #[test]
    fn test_word_backward() {
        let motions = VimMotions::new("hello world test");
        assert_eq!(motions.word_backward(16), Some(12));
        assert_eq!(motions.word_backward(12), Some(6));
        assert_eq!(motions.word_backward(6), Some(0));
        assert_eq!(motions.word_backward(0), None);
    }

    #[test]
    fn test_end_of_word() {
        let motions = VimMotions::new("hello world test");
        assert_eq!(motions.end_of_word(0), Some(4));
        assert_eq!(motions.end_of_word(6), Some(10));
        assert_eq!(motions.end_of_word(12), Some(15));
    }

    #[test]
    fn test_word_forward_ws() {
        let motions = VimMotions::new("hello,world test");
        assert_eq!(motions.word_forward_ws(0), Some(12));
        assert_eq!(motions.word_forward_ws(12), None);
    }

    #[test]
    fn test_word_backward_ws() {
        let motions = VimMotions::new("hello,world test");
        assert_eq!(motions.word_backward_ws(16), Some(12));
        assert_eq!(motions.word_backward_ws(12), Some(0));
        assert_eq!(motions.word_backward_ws(0), None);
    }

    #[test]
    fn test_end_of_word_ws() {
        let motions = VimMotions::new("hello,world test");
        assert_eq!(motions.end_of_word_ws(0), Some(10));
        assert_eq!(motions.end_of_word_ws(12), Some(15));
    }

    #[test]
    fn test_find_char() {
        let motions = VimMotions::new("hello world");
        assert_eq!(motions.find_char(0, 'w'), Some(6));
        assert_eq!(motions.find_char(0, 'o'), Some(4));
        assert_eq!(motions.find_char(5, 'o'), Some(7));
        assert_eq!(motions.find_char(0, 'z'), None);
    }

    #[test]
    fn test_find_char_backward() {
        let motions = VimMotions::new("hello world");
        assert_eq!(motions.find_char_backward(10, 'h'), Some(0));
        assert_eq!(motions.find_char_backward(10, 'l'), Some(9));
        assert_eq!(motions.find_char_backward(10, 'o'), Some(7));
        assert_eq!(motions.find_char_backward(0, 'h'), None);
    }

    #[test]
    fn test_till_char() {
        let motions = VimMotions::new("hello world");
        assert_eq!(motions.till_char(0, 'w'), Some(5));
        assert_eq!(motions.till_char(0, 'o'), Some(3));
    }

    #[test]
    fn test_till_char_backward() {
        let motions = VimMotions::new("hello world");
        assert_eq!(motions.till_char_backward(10, 'h'), Some(1));
        assert_eq!(motions.till_char_backward(10, 'e'), Some(2));
    }

    #[test]
    fn test_execute() {
        let motions = VimMotions::new("hello world");
        assert_eq!(motions.execute(0, Motion::WordForward), Some(6));
        assert_eq!(motions.execute(0, Motion::FindChar('w')), Some(6));
        assert_eq!(motions.execute(0, Motion::EndOfWord), Some(4));
    }

    #[test]
    fn test_execute_with_count() {
        let motions = VimMotions::new("one two three four");
        // 2w - two words forward
        assert_eq!(motions.execute_with_count(0, Motion::WordForward, 2), Some(8));
        // 3w - three words forward
        assert_eq!(motions.execute_with_count(0, Motion::WordForward, 3), Some(14));
    }

    #[test]
    fn test_execute_with_count_zero() {
        let motions = VimMotions::new("one two three");
        // Count of 0 should be treated as 1
        assert_eq!(motions.execute_with_count(0, Motion::WordForward, 0), Some(4));
    }

    #[test]
    fn test_set_text() {
        let mut motions = VimMotions::new("hello");
        assert_eq!(motions.text(), "hello");

        motions.set_text("world");
        assert_eq!(motions.text(), "world");
    }

    #[test]
    fn test_empty_text() {
        let motions = VimMotions::new("");
        assert_eq!(motions.word_forward(0), None);
        assert_eq!(motions.find_char(0, 'a'), None);
    }

    #[test]
    fn test_word_with_numbers() {
        let motions = VimMotions::new("hello123 world456");
        assert_eq!(motions.word_forward(0), Some(9));
        assert_eq!(motions.word_backward(9), Some(0));
    }

    #[test]
    fn test_word_with_underscores() {
        let motions = VimMotions::new("hello_world test_case");
        assert_eq!(motions.word_forward(0), Some(12));
        assert_eq!(motions.end_of_word(0), Some(10));
    }

    #[test]
    fn test_multiple_spaces() {
        let motions = VimMotions::new("hello    world");
        assert_eq!(motions.word_forward(0), Some(9));
        assert_eq!(motions.word_backward(9), Some(0));
    }

    #[test]
    fn test_motion_enum() {
        let motion = Motion::WordForward;
        assert_eq!(motion, Motion::WordForward);

        let motion2 = Motion::FindChar('a');
        assert!(matches!(motion2, Motion::FindChar('a')));
    }
}
