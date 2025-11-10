//! Keyboard shortcut recorder for recording and playing back key sequences
//!
//! Provides macro-like functionality for recording key sequences and playing them back,
//! similar to Vim's macro system.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::keyboard_recorder::KeyboardRecorder;
//!
//! let mut recorder = KeyboardRecorder::new();
//! recorder.start_recording('a');
//! // ... record keys ...
//! recorder.stop_recording();
//! recorder.play('a');
//! ```

use crossterm::event::KeyEvent;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Recorded key event with timing information
#[derive(Debug, Clone)]
pub struct RecordedKey {
    /// The key event
    pub event: KeyEvent,
    /// Time since previous key (for playback timing)
    pub delay: Duration,
}

/// Key sequence (macro)
#[derive(Debug, Clone)]
pub struct KeySequence {
    /// Name/ID of the sequence
    pub name: String,
    /// Recorded keys
    pub keys: Vec<RecordedKey>,
    /// Total duration
    pub duration: Duration,
    /// When it was created
    pub created_at: Instant,
}

impl KeySequence {
    /// Create a new key sequence
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            keys: Vec::new(),
            duration: Duration::ZERO,
            created_at: Instant::now(),
        }
    }

    /// Add a key to the sequence
    pub fn add_key(&mut self, event: KeyEvent, delay: Duration) {
        self.keys.push(RecordedKey { event, delay });
        self.duration += delay;
    }

    /// Get key count
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Get total duration
    pub fn total_duration(&self) -> Duration {
        self.duration
    }
}

/// Keyboard recorder state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecorderState {
    /// Not recording
    Idle,
    /// Currently recording
    Recording,
    /// Playing back
    Playing,
}

/// Keyboard recorder
///
/// Records and plays back keyboard shortcuts, similar to Vim macros.
#[derive(Debug)]
pub struct KeyboardRecorder {
    /// Current state
    state: RecorderState,
    /// Currently recording register
    current_register: Option<char>,
    /// Current recording
    current_sequence: Option<KeySequence>,
    /// Last key time (for calculating delays)
    last_key_time: Option<Instant>,
    /// Saved sequences by register
    sequences: HashMap<char, KeySequence>,
    /// Current playback position
    playback_index: usize,
    /// Whether to preserve timing on playback
    preserve_timing: bool,
}

impl KeyboardRecorder {
    /// Create a new keyboard recorder
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::keyboard_recorder::KeyboardRecorder;
    ///
    /// let recorder = KeyboardRecorder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            state: RecorderState::Idle,
            current_register: None,
            current_sequence: None,
            last_key_time: None,
            sequences: HashMap::new(),
            playback_index: 0,
            preserve_timing: false,
        }
    }

    /// Start recording to a register
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::infrastructure::keyboard_recorder::KeyboardRecorder;
    ///
    /// let mut recorder = KeyboardRecorder::new();
    /// recorder.start_recording('a');
    /// ```
    pub fn start_recording(&mut self, register: char) -> bool {
        if self.state != RecorderState::Idle {
            return false;
        }

        self.state = RecorderState::Recording;
        self.current_register = Some(register);
        self.current_sequence = Some(KeySequence::new(register.to_string()));
        self.last_key_time = Some(Instant::now());
        true
    }

    /// Record a key event
    pub fn record_key(&mut self, event: KeyEvent) {
        if self.state != RecorderState::Recording {
            return;
        }

        let now = Instant::now();
        let delay = self
            .last_key_time
            .map(|t| now.duration_since(t))
            .unwrap_or(Duration::ZERO);

        if let Some(ref mut sequence) = self.current_sequence {
            sequence.add_key(event, delay);
        }

        self.last_key_time = Some(now);
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Option<char> {
        if self.state != RecorderState::Recording {
            return None;
        }

        let register = self.current_register?;

        if let Some(sequence) = self.current_sequence.take() {
            self.sequences.insert(register, sequence);
        }

        self.state = RecorderState::Idle;
        self.current_register = None;
        self.last_key_time = None;

        Some(register)
    }

    /// Start playing back a sequence
    pub fn play(&mut self, register: char) -> bool {
        if self.state != RecorderState::Idle {
            return false;
        }

        if !self.sequences.contains_key(&register) {
            return false;
        }

        self.state = RecorderState::Playing;
        self.current_register = Some(register);
        self.playback_index = 0;
        true
    }

    /// Get next key in playback
    pub fn next_playback_key(&mut self) -> Option<RecordedKey> {
        if self.state != RecorderState::Playing {
            return None;
        }

        let register = self.current_register?;
        let sequence = self.sequences.get(&register)?;

        if self.playback_index >= sequence.keys.len() {
            // Playback complete
            self.state = RecorderState::Idle;
            self.current_register = None;
            self.playback_index = 0;
            return None;
        }

        let key = sequence.keys[self.playback_index].clone();
        self.playback_index += 1;

        Some(key)
    }

    /// Stop playback
    pub fn stop_playback(&mut self) {
        if self.state == RecorderState::Playing {
            self.state = RecorderState::Idle;
            self.current_register = None;
            self.playback_index = 0;
        }
    }

    /// Get current state
    pub fn state(&self) -> RecorderState {
        self.state
    }

    /// Check if recording
    pub fn is_recording(&self) -> bool {
        self.state == RecorderState::Recording
    }

    /// Check if playing
    pub fn is_playing(&self) -> bool {
        self.state == RecorderState::Playing
    }

    /// Get current register
    pub fn current_register(&self) -> Option<char> {
        self.current_register
    }

    /// Get sequence by register
    pub fn get_sequence(&self, register: char) -> Option<&KeySequence> {
        self.sequences.get(&register)
    }

    /// Delete sequence
    pub fn delete_sequence(&mut self, register: char) -> bool {
        self.sequences.remove(&register).is_some()
    }

    /// List all registers
    pub fn list_registers(&self) -> Vec<char> {
        let mut registers: Vec<char> = self.sequences.keys().copied().collect();
        registers.sort();
        registers
    }

    /// Clear all sequences
    pub fn clear_all(&mut self) {
        self.sequences.clear();
    }

    /// Set preserve timing
    pub fn set_preserve_timing(&mut self, preserve: bool) {
        self.preserve_timing = preserve;
    }

    /// Get preserve timing setting
    pub fn preserve_timing(&self) -> bool {
        self.preserve_timing
    }

    /// Get total sequences count
    pub fn sequence_count(&self) -> usize {
        self.sequences.len()
    }
}

impl Default for KeyboardRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn test_recorder_creation() {
        let recorder = KeyboardRecorder::new();
        assert_eq!(recorder.state(), RecorderState::Idle);
        assert!(!recorder.is_recording());
        assert!(!recorder.is_playing());
    }

    #[test]
    fn test_start_recording() {
        let mut recorder = KeyboardRecorder::new();
        assert!(recorder.start_recording('a'));
        assert_eq!(recorder.state(), RecorderState::Recording);
        assert_eq!(recorder.current_register(), Some('a'));
    }

    #[test]
    fn test_cannot_start_recording_while_recording() {
        let mut recorder = KeyboardRecorder::new();
        recorder.start_recording('a');
        assert!(!recorder.start_recording('b')); // Should fail
    }

    #[test]
    fn test_record_keys() {
        let mut recorder = KeyboardRecorder::new();
        recorder.start_recording('a');

        recorder.record_key(make_key_event(KeyCode::Char('h')));
        recorder.record_key(make_key_event(KeyCode::Char('e')));
        recorder.record_key(make_key_event(KeyCode::Char('l')));
        recorder.record_key(make_key_event(KeyCode::Char('l')));
        recorder.record_key(make_key_event(KeyCode::Char('o')));

        recorder.stop_recording();

        let sequence = recorder.get_sequence('a').unwrap();
        assert_eq!(sequence.len(), 5);
    }

    #[test]
    fn test_stop_recording() {
        let mut recorder = KeyboardRecorder::new();
        recorder.start_recording('a');
        recorder.record_key(make_key_event(KeyCode::Char('x')));

        let register = recorder.stop_recording();
        assert_eq!(register, Some('a'));
        assert_eq!(recorder.state(), RecorderState::Idle);
    }

    #[test]
    fn test_playback() {
        let mut recorder = KeyboardRecorder::new();
        recorder.start_recording('a');
        recorder.record_key(make_key_event(KeyCode::Char('h')));
        recorder.record_key(make_key_event(KeyCode::Char('i')));
        recorder.stop_recording();

        // Start playback
        assert!(recorder.play('a'));
        assert_eq!(recorder.state(), RecorderState::Playing);

        // Get keys
        let key1 = recorder.next_playback_key();
        assert!(key1.is_some());
        assert_eq!(key1.unwrap().event.code, KeyCode::Char('h'));

        let key2 = recorder.next_playback_key();
        assert!(key2.is_some());
        assert_eq!(key2.unwrap().event.code, KeyCode::Char('i'));

        // No more keys
        let key3 = recorder.next_playback_key();
        assert!(key3.is_none());
        assert_eq!(recorder.state(), RecorderState::Idle);
    }

    #[test]
    fn test_cannot_play_nonexistent_sequence() {
        let mut recorder = KeyboardRecorder::new();
        assert!(!recorder.play('z')); // Doesn't exist
    }

    #[test]
    fn test_delete_sequence() {
        let mut recorder = KeyboardRecorder::new();
        recorder.start_recording('a');
        recorder.record_key(make_key_event(KeyCode::Char('x')));
        recorder.stop_recording();

        assert!(recorder.get_sequence('a').is_some());
        assert!(recorder.delete_sequence('a'));
        assert!(recorder.get_sequence('a').is_none());
    }

    #[test]
    fn test_list_registers() {
        let mut recorder = KeyboardRecorder::new();

        recorder.start_recording('a');
        recorder.stop_recording();

        recorder.start_recording('c');
        recorder.stop_recording();

        recorder.start_recording('b');
        recorder.stop_recording();

        let registers = recorder.list_registers();
        assert_eq!(registers, vec!['a', 'b', 'c']); // Should be sorted
    }

    #[test]
    fn test_clear_all() {
        let mut recorder = KeyboardRecorder::new();

        recorder.start_recording('a');
        recorder.stop_recording();

        recorder.start_recording('b');
        recorder.stop_recording();

        assert_eq!(recorder.sequence_count(), 2);

        recorder.clear_all();
        assert_eq!(recorder.sequence_count(), 0);
    }

    #[test]
    fn test_sequence_count() {
        let mut recorder = KeyboardRecorder::new();
        assert_eq!(recorder.sequence_count(), 0);

        recorder.start_recording('a');
        recorder.stop_recording();
        assert_eq!(recorder.sequence_count(), 1);

        recorder.start_recording('b');
        recorder.stop_recording();
        assert_eq!(recorder.sequence_count(), 2);
    }

    #[test]
    fn test_preserve_timing() {
        let mut recorder = KeyboardRecorder::new();
        assert!(!recorder.preserve_timing());

        recorder.set_preserve_timing(true);
        assert!(recorder.preserve_timing());
    }

    #[test]
    fn test_stop_playback() {
        let mut recorder = KeyboardRecorder::new();
        recorder.start_recording('a');
        recorder.record_key(make_key_event(KeyCode::Char('x')));
        recorder.stop_recording();

        recorder.play('a');
        assert_eq!(recorder.state(), RecorderState::Playing);

        recorder.stop_playback();
        assert_eq!(recorder.state(), RecorderState::Idle);
    }

    #[test]
    fn test_key_sequence_creation() {
        let sequence = KeySequence::new("test");
        assert_eq!(sequence.name, "test");
        assert!(sequence.is_empty());
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_key_sequence_add_key() {
        let mut sequence = KeySequence::new("test");
        sequence.add_key(make_key_event(KeyCode::Char('a')), Duration::from_millis(100));

        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence.total_duration(), Duration::from_millis(100));
    }

    #[test]
    fn test_default() {
        let recorder = KeyboardRecorder::default();
        assert_eq!(recorder.state(), RecorderState::Idle);
    }
}
