/// Multi-key command sequences (vim-style)
///
/// Provides support for chained key commands like `gg`, `gc`, `dd` etc.
///
/// # Examples
///
/// ```
/// use toad::key_sequences::{KeySequence, KeySequenceManager};
/// use crossterm::event::KeyCode;
///
/// let mut manager = KeySequenceManager::new();
/// manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
///
/// assert!(manager.len() > 0);
/// ```
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A sequence of keys that triggers an action
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeySequence {
    /// The keys in the sequence
    #[serde(with = "vec_keycode_serde")]
    pub keys: Vec<KeyCode>,
}

impl KeySequence {
    /// Create a new key sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequence;
    /// use crossterm::event::KeyCode;
    ///
    /// let seq = KeySequence::new(vec![KeyCode::Char('g'), KeyCode::Char('g')]);
    /// assert_eq!(seq.len(), 2);
    /// ```
    pub fn new(keys: Vec<KeyCode>) -> Self {
        Self { keys }
    }

    /// Create a two-key sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequence;
    /// use crossterm::event::KeyCode;
    ///
    /// let gg = KeySequence::two(KeyCode::Char('g'), KeyCode::Char('g'));
    /// ```
    pub fn two(key1: KeyCode, key2: KeyCode) -> Self {
        Self::new(vec![key1, key2])
    }

    /// Create a three-key sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequence;
    /// use crossterm::event::KeyCode;
    ///
    /// let gca = KeySequence::three(
    ///     KeyCode::Char('g'),
    ///     KeyCode::Char('c'),
    ///     KeyCode::Char('a')
    /// );
    /// ```
    pub fn three(key1: KeyCode, key2: KeyCode, key3: KeyCode) -> Self {
        Self::new(vec![key1, key2, key3])
    }

    /// Get the length of the sequence
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if sequence is empty
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Check if this sequence starts with another sequence (prefix match)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequence;
    /// use crossterm::event::KeyCode;
    ///
    /// let gcc = KeySequence::new(vec![
    ///     KeyCode::Char('g'),
    ///     KeyCode::Char('c'),
    ///     KeyCode::Char('c'),
    /// ]);
    /// let gc = KeySequence::two(KeyCode::Char('g'), KeyCode::Char('c'));
    ///
    /// assert!(gcc.starts_with(&gc));
    /// ```
    pub fn starts_with(&self, other: &KeySequence) -> bool {
        if other.len() > self.len() {
            return false;
        }

        self.keys.iter().zip(&other.keys).all(|(a, b)| a == b)
    }
}

/// Manager for key sequences and partial matches
#[derive(Debug, Clone)]
pub struct KeySequenceManager {
    /// Registered sequences mapped to actions
    sequences: HashMap<KeySequence, String>,
    /// Current partial sequence being typed
    current_sequence: Vec<KeyCode>,
    /// Last key press time (for timeout)
    last_key_time: Option<Instant>,
    /// Timeout for partial sequences
    timeout: Duration,
}

impl Default for KeySequenceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl KeySequenceManager {
    /// Create a new key sequence manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    ///
    /// let manager = KeySequenceManager::new();
    /// assert_eq!(manager.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            sequences: HashMap::new(),
            current_sequence: Vec::new(),
            last_key_time: None,
            timeout: Duration::from_millis(1000),
        }
    }

    /// Create a manager with vim-style default sequences
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    ///
    /// let manager = KeySequenceManager::vim_defaults();
    /// assert!(manager.len() > 0);
    /// ```
    pub fn vim_defaults() -> Self {
        let mut manager = Self::new();

        // Navigation
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
        manager.register(vec![KeyCode::Char('G')], "goto_bottom");
        manager.register(
            vec![KeyCode::Char('g'), KeyCode::Char('e')],
            "goto_end_word",
        );
        manager.register(
            vec![KeyCode::Char('g'), KeyCode::Char('0')],
            "goto_line_start",
        );
        manager.register(
            vec![KeyCode::Char('g'), KeyCode::Char('$')],
            "goto_line_end",
        );

        // Deletion
        manager.register(vec![KeyCode::Char('d'), KeyCode::Char('d')], "delete_line");
        manager.register(vec![KeyCode::Char('d'), KeyCode::Char('w')], "delete_word");
        manager.register(
            vec![KeyCode::Char('d'), KeyCode::Char('$')],
            "delete_to_end",
        );
        manager.register(
            vec![KeyCode::Char('d'), KeyCode::Char('0')],
            "delete_to_start",
        );

        // Change
        manager.register(vec![KeyCode::Char('c'), KeyCode::Char('c')], "change_line");
        manager.register(vec![KeyCode::Char('c'), KeyCode::Char('w')], "change_word");
        manager.register(
            vec![KeyCode::Char('c'), KeyCode::Char('$')],
            "change_to_end",
        );

        // Yank (copy)
        manager.register(vec![KeyCode::Char('y'), KeyCode::Char('y')], "yank_line");
        manager.register(vec![KeyCode::Char('y'), KeyCode::Char('w')], "yank_word");

        // Indentation
        manager.register(vec![KeyCode::Char('>'), KeyCode::Char('>')], "indent_line");
        manager.register(
            vec![KeyCode::Char('<'), KeyCode::Char('<')],
            "unindent_line",
        );

        // Marks
        manager.register(vec![KeyCode::Char('m'), KeyCode::Char('a')], "set_mark_a");
        manager.register(vec![KeyCode::Char('\''), KeyCode::Char('a')], "goto_mark_a");

        // Comments
        manager.register(
            vec![KeyCode::Char('g'), KeyCode::Char('c')],
            "toggle_comment",
        );
        manager.register(
            vec![KeyCode::Char('g'), KeyCode::Char('c'), KeyCode::Char('c')],
            "comment_line",
        );

        // Folding
        manager.register(vec![KeyCode::Char('z'), KeyCode::Char('a')], "toggle_fold");
        manager.register(vec![KeyCode::Char('z'), KeyCode::Char('c')], "close_fold");
        manager.register(vec![KeyCode::Char('z'), KeyCode::Char('o')], "open_fold");

        manager
    }

    /// Set the timeout for partial sequences
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    /// use std::time::Duration;
    ///
    /// let mut manager = KeySequenceManager::new();
    /// manager.set_timeout(Duration::from_millis(500));
    /// ```
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Register a key sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    /// use crossterm::event::KeyCode;
    ///
    /// let mut manager = KeySequenceManager::new();
    /// manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
    /// ```
    pub fn register(&mut self, keys: Vec<KeyCode>, action: impl Into<String>) {
        let sequence = KeySequence::new(keys);
        self.sequences.insert(sequence, action.into());
    }

    /// Unregister a sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    /// use crossterm::event::KeyCode;
    ///
    /// let mut manager = KeySequenceManager::new();
    /// manager.register(vec![KeyCode::Char('d'), KeyCode::Char('d')], "delete");
    /// manager.unregister(&vec![KeyCode::Char('d'), KeyCode::Char('d')]);
    /// ```
    pub fn unregister(&mut self, keys: &[KeyCode]) -> Option<String> {
        let sequence = KeySequence::new(keys.to_vec());
        self.sequences.remove(&sequence)
    }

    /// Process a key and return the action if sequence is complete
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    /// use crossterm::event::KeyCode;
    ///
    /// let mut manager = KeySequenceManager::new();
    /// manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
    ///
    /// // First key - partial match
    /// let result = manager.process_key(KeyCode::Char('g'));
    /// assert!(result.is_none());
    ///
    /// // Second key - complete match
    /// let result = manager.process_key(KeyCode::Char('g'));
    /// assert_eq!(result, Some("goto_top".to_string()));
    /// ```
    pub fn process_key(&mut self, key: KeyCode) -> Option<String> {
        // Check for timeout
        if let Some(last_time) = self.last_key_time {
            if last_time.elapsed() > self.timeout {
                self.reset();
            }
        }

        // Add key to current sequence
        self.current_sequence.push(key);
        self.last_key_time = Some(Instant::now());

        // Check for complete match
        let current = KeySequence::new(self.current_sequence.clone());

        if let Some(action) = self.sequences.get(&current) {
            let action = action.clone();
            self.reset();
            return Some(action);
        }

        // Check if there's any sequence that starts with current
        let has_prefix = self.sequences.keys().any(|seq| seq.starts_with(&current));

        if !has_prefix {
            // No possible matches, reset
            self.reset();
        }

        None
    }

    /// Reset the current partial sequence
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    /// use crossterm::event::KeyCode;
    ///
    /// let mut manager = KeySequenceManager::new();
    /// manager.process_key(KeyCode::Char('g'));
    /// manager.reset();
    /// assert!(manager.current_partial().is_none());
    /// ```
    pub fn reset(&mut self) {
        self.current_sequence.clear();
        self.last_key_time = None;
    }

    /// Get the current partial sequence if any
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::key_sequences::KeySequenceManager;
    /// use crossterm::event::KeyCode;
    ///
    /// let mut manager = KeySequenceManager::new();
    /// manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
    /// manager.process_key(KeyCode::Char('g'));
    ///
    /// assert!(manager.current_partial().is_some());
    /// ```
    pub fn current_partial(&self) -> Option<&[KeyCode]> {
        if self.current_sequence.is_empty() {
            None
        } else {
            Some(&self.current_sequence)
        }
    }

    /// Get number of registered sequences
    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    /// Check if no sequences are registered
    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }

    /// Get all registered sequences
    pub fn sequences(&self) -> impl Iterator<Item = (&KeySequence, &String)> {
        self.sequences.iter()
    }

    /// Clear all registered sequences
    pub fn clear(&mut self) {
        self.sequences.clear();
        self.reset();
    }
}

// Serde helpers for Vec<KeyCode>
mod vec_keycode_serde {
    use crossterm::event::KeyCode;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(keys: &[KeyCode], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let strs: Vec<String> = keys
            .iter()
            .map(|k| match k {
                KeyCode::Char(c) => format!("{}", c),
                KeyCode::Enter => "Enter".to_string(),
                KeyCode::Esc => "Esc".to_string(),
                KeyCode::Tab => "Tab".to_string(),
                KeyCode::Backspace => "Backspace".to_string(),
                KeyCode::F(n) => format!("F{}", n),
                _ => "?".to_string(),
            })
            .collect();
        strs.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<KeyCode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strs = Vec::<String>::deserialize(deserializer)?;
        let keys = strs
            .into_iter()
            .map(|s| {
                if s.len() == 1 {
                    KeyCode::Char(s.chars().next().unwrap())
                } else if let Some(num_str) = s.strip_prefix('F') {
                    let n: u8 = num_str.parse().unwrap_or(1);
                    KeyCode::F(n)
                } else {
                    match s.as_str() {
                        "Enter" => KeyCode::Enter,
                        "Esc" => KeyCode::Esc,
                        "Tab" => KeyCode::Tab,
                        "Backspace" => KeyCode::Backspace,
                        _ => KeyCode::Null,
                    }
                }
            })
            .collect();
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_sequence_creation() {
        let seq = KeySequence::new(vec![KeyCode::Char('g'), KeyCode::Char('g')]);
        assert_eq!(seq.len(), 2);
        assert!(!seq.is_empty());
    }

    #[test]
    fn test_key_sequence_convenience() {
        let two = KeySequence::two(KeyCode::Char('d'), KeyCode::Char('d'));
        assert_eq!(two.len(), 2);

        let three = KeySequence::three(KeyCode::Char('g'), KeyCode::Char('c'), KeyCode::Char('c'));
        assert_eq!(three.len(), 3);
    }

    #[test]
    fn test_key_sequence_starts_with() {
        let gcc = KeySequence::new(vec![
            KeyCode::Char('g'),
            KeyCode::Char('c'),
            KeyCode::Char('c'),
        ]);
        let gc = KeySequence::two(KeyCode::Char('g'), KeyCode::Char('c'));
        let g = KeySequence::new(vec![KeyCode::Char('g')]);

        assert!(gcc.starts_with(&gc));
        assert!(gcc.starts_with(&g));
        assert!(!gc.starts_with(&gcc));
    }

    #[test]
    fn test_manager_creation() {
        let manager = KeySequenceManager::new();
        assert_eq!(manager.len(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_register() {
        let mut manager = KeySequenceManager::new();
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_manager_unregister() {
        let mut manager = KeySequenceManager::new();
        manager.register(vec![KeyCode::Char('d'), KeyCode::Char('d')], "delete");
        manager.unregister(&[KeyCode::Char('d'), KeyCode::Char('d')]);
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_manager_process_complete_sequence() {
        let mut manager = KeySequenceManager::new();
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");

        let result1 = manager.process_key(KeyCode::Char('g'));
        assert!(result1.is_none());

        let result2 = manager.process_key(KeyCode::Char('g'));
        assert_eq!(result2, Some("goto_top".to_string()));
    }

    #[test]
    fn test_manager_process_no_match() {
        let mut manager = KeySequenceManager::new();
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");

        manager.process_key(KeyCode::Char('g'));
        let result = manager.process_key(KeyCode::Char('x'));
        assert!(result.is_none());
        // Should reset after no match
        assert!(manager.current_partial().is_none());
    }

    #[test]
    fn test_manager_partial_sequence() {
        let mut manager = KeySequenceManager::new();
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");

        manager.process_key(KeyCode::Char('g'));
        let partial = manager.current_partial();
        assert!(partial.is_some());
        assert_eq!(partial.unwrap().len(), 1);
    }

    #[test]
    fn test_manager_reset() {
        let mut manager = KeySequenceManager::new();
        manager.process_key(KeyCode::Char('g'));
        manager.reset();
        assert!(manager.current_partial().is_none());
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = KeySequenceManager::new();
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");
        manager.clear();
        assert_eq!(manager.len(), 0);
    }

    #[test]
    fn test_vim_defaults() {
        let manager = KeySequenceManager::vim_defaults();
        assert!(!manager.is_empty());
        assert!(manager.len() > 10);
    }

    #[test]
    fn test_vim_default_gg() {
        let mut manager = KeySequenceManager::vim_defaults();

        manager.process_key(KeyCode::Char('g'));
        let result = manager.process_key(KeyCode::Char('g'));
        assert_eq!(result, Some("goto_top".to_string()));
    }

    #[test]
    fn test_vim_default_dd() {
        let mut manager = KeySequenceManager::vim_defaults();

        manager.process_key(KeyCode::Char('d'));
        let result = manager.process_key(KeyCode::Char('d'));
        assert_eq!(result, Some("delete_line".to_string()));
    }

    #[test]
    fn test_sequence_serialization() {
        let seq = KeySequence::two(KeyCode::Char('g'), KeyCode::Char('g'));
        let json = serde_json::to_string(&seq).unwrap();
        let deserialized: KeySequence = serde_json::from_str(&json).unwrap();
        assert_eq!(seq, deserialized);
    }

    #[test]
    fn test_empty_sequence() {
        let seq = KeySequence::new(vec![]);
        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_manager_timeout() {
        let mut manager = KeySequenceManager::new();
        manager.set_timeout(Duration::from_millis(1));
        manager.register(vec![KeyCode::Char('g'), KeyCode::Char('g')], "goto_top");

        manager.process_key(KeyCode::Char('g'));
        std::thread::sleep(Duration::from_millis(5));

        // Should timeout and reset
        let result = manager.process_key(KeyCode::Char('g'));
        assert!(result.is_none());
    }
}
