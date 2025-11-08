//! Keyboard shortcut and keybinding system
//!
//! Provides a flexible system for defining and managing keyboard shortcuts
//! with support for modifiers, chords, and context-specific bindings.
//!
//! # Examples
//!
//! ```
//! use toad::keybinds::{KeyBinding, KeyBindings};
//! use crossterm::event::{KeyCode, KeyModifiers};
//!
//! let mut bindings = KeyBindings::new();
//! bindings.bind(
//!     KeyBinding::new(KeyCode::Char('q'), KeyModifiers::CONTROL),
//!     "quit"
//! );
//!
//! // Check if key combo matches
//! let key = KeyBinding::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
//! assert_eq!(bindings.get(&key), Some(&"quit".to_string()));
//! ```

use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A keyboard binding (key + modifiers)
///
/// # Examples
///
/// ```
/// use toad::keybinds::KeyBinding;
/// use crossterm::event::{KeyCode, KeyModifiers};
///
/// // Ctrl+S
/// let save = KeyBinding::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
///
/// // Ctrl+Shift+P
/// let palette = KeyBinding::new(
///     KeyCode::Char('p'),
///     KeyModifiers::CONTROL | KeyModifiers::SHIFT
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    /// The key code
    #[serde(with = "keycode_serde")]
    pub code: KeyCode,
    /// Modifier keys (Ctrl, Alt, Shift)
    #[serde(with = "keymodifiers_serde")]
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    /// Create a new key binding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::{KeyCode, KeyModifiers};
    ///
    /// let binding = KeyBinding::new(KeyCode::Esc, KeyModifiers::NONE);
    /// assert_eq!(binding.code, KeyCode::Esc);
    /// ```
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    /// Create a key binding without modifiers
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let enter = KeyBinding::plain(KeyCode::Enter);
    /// assert!(enter.modifiers.is_empty());
    /// ```
    pub fn plain(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::NONE)
    }

    /// Create a Ctrl+key binding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let ctrl_c = KeyBinding::ctrl(KeyCode::Char('c'));
    /// ```
    pub fn ctrl(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::CONTROL)
    }

    /// Create an Alt+key binding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let alt_f = KeyBinding::alt(KeyCode::Char('f'));
    /// ```
    pub fn alt(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::ALT)
    }

    /// Create a Shift+key binding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let shift_tab = KeyBinding::shift(KeyCode::Tab);
    /// ```
    pub fn shift(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::SHIFT)
    }

    /// Check if this binding has Ctrl modifier
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let ctrl_s = KeyBinding::ctrl(KeyCode::Char('s'));
    /// assert!(ctrl_s.has_ctrl());
    /// ```
    pub fn has_ctrl(&self) -> bool {
        self.modifiers.contains(KeyModifiers::CONTROL)
    }

    /// Check if this binding has Alt modifier
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let alt_f = KeyBinding::alt(KeyCode::Char('f'));
    /// assert!(alt_f.has_alt());
    /// ```
    pub fn has_alt(&self) -> bool {
        self.modifiers.contains(KeyModifiers::ALT)
    }

    /// Check if this binding has Shift modifier
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBinding;
    /// use crossterm::event::KeyCode;
    ///
    /// let shift_tab = KeyBinding::shift(KeyCode::Tab);
    /// assert!(shift_tab.has_shift());
    /// ```
    pub fn has_shift(&self) -> bool {
        self.modifiers.contains(KeyModifiers::SHIFT)
    }
}

impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.has_ctrl() {
            parts.push("Ctrl");
        }
        if self.has_alt() {
            parts.push("Alt");
        }
        if self.has_shift() {
            parts.push("Shift");
        }

        let key_str = match self.code {
            KeyCode::Char(c) => c.to_uppercase().to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            _ => "?".to_string(),
        };

        parts.push(&key_str);
        write!(f, "{}", parts.join("+"))
    }
}

/// Collection of keybindings mapped to actions
///
/// # Examples
///
/// ```
/// use toad::keybinds::{KeyBinding, KeyBindings};
/// use crossterm::event::KeyCode;
///
/// let mut bindings = KeyBindings::new();
/// bindings.bind(KeyBinding::ctrl(KeyCode::Char('q')), "quit");
/// bindings.bind(KeyBinding::plain(KeyCode::Esc), "cancel");
///
/// assert_eq!(bindings.len(), 2);
/// ```
#[derive(Debug, Clone, Default)]
pub struct KeyBindings {
    bindings: HashMap<KeyBinding, String>,
}

impl KeyBindings {
    /// Create a new empty keybindings collection
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBindings;
    ///
    /// let bindings = KeyBindings::new();
    /// assert_eq!(bindings.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Create default keybindings for Toad
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBindings;
    ///
    /// let bindings = KeyBindings::default_bindings();
    /// assert!(bindings.len() > 0);
    /// ```
    pub fn default_bindings() -> Self {
        let mut bindings = Self::new();

        // Application controls
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('c')), "quit");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('q')), "quit");
        bindings.bind(KeyBinding::plain(KeyCode::Esc), "cancel");

        // Help
        bindings.bind(KeyBinding::plain(KeyCode::Char('?')), "help");
        bindings.bind(KeyBinding::plain(KeyCode::F(1)), "help");

        // Command palette
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('p')), "palette");
        bindings.bind(KeyBinding::plain(KeyCode::Char('/')), "commands");

        // Navigation
        bindings.bind(KeyBinding::plain(KeyCode::Tab), "next");
        bindings.bind(KeyBinding::shift(KeyCode::Tab), "previous");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('n')), "next");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('p')), "previous");

        // Editing
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('s')), "save");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('z')), "undo");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('y')), "redo");

        // Clipboard
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('c')), "copy");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('x')), "cut");
        bindings.bind(KeyBinding::ctrl(KeyCode::Char('v')), "paste");

        bindings
    }

    /// Bind a key to an action
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::{KeyBinding, KeyBindings};
    /// use crossterm::event::KeyCode;
    ///
    /// let mut bindings = KeyBindings::new();
    /// bindings.bind(KeyBinding::ctrl(KeyCode::Char('s')), "save");
    /// ```
    pub fn bind(&mut self, key: KeyBinding, action: impl Into<String>) {
        self.bindings.insert(key, action.into());
    }

    /// Get the action for a key binding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::{KeyBinding, KeyBindings};
    /// use crossterm::event::KeyCode;
    ///
    /// let mut bindings = KeyBindings::new();
    /// bindings.bind(KeyBinding::ctrl(KeyCode::Char('q')), "quit");
    ///
    /// let key = KeyBinding::ctrl(KeyCode::Char('q'));
    /// assert_eq!(bindings.get(&key), Some(&"quit".to_string()));
    /// ```
    pub fn get(&self, key: &KeyBinding) -> Option<&String> {
        self.bindings.get(key)
    }

    /// Remove a key binding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::{KeyBinding, KeyBindings};
    /// use crossterm::event::KeyCode;
    ///
    /// let mut bindings = KeyBindings::new();
    /// bindings.bind(KeyBinding::ctrl(KeyCode::Char('q')), "quit");
    /// bindings.unbind(&KeyBinding::ctrl(KeyCode::Char('q')));
    ///
    /// assert_eq!(bindings.len(), 0);
    /// ```
    pub fn unbind(&mut self, key: &KeyBinding) -> Option<String> {
        self.bindings.remove(key)
    }

    /// Get number of bindings
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::{KeyBinding, KeyBindings};
    /// use crossterm::event::KeyCode;
    ///
    /// let mut bindings = KeyBindings::new();
    /// bindings.bind(KeyBinding::plain(KeyCode::Esc), "cancel");
    /// assert_eq!(bindings.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Check if there are no bindings
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::keybinds::KeyBindings;
    ///
    /// let bindings = KeyBindings::new();
    /// assert!(bindings.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Get all bindings as iterator
    pub fn iter(&self) -> impl Iterator<Item = (&KeyBinding, &String)> {
        self.bindings.iter()
    }
}

// Serde helpers for KeyCode and KeyModifiers
mod keycode_serde {
    use crossterm::event::KeyCode;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(code: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match code {
            KeyCode::Char(c) => format!("Char({})", c),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::F(n) => format!("F{}", n),
            _ => "Unknown".to_string(),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("Char(") && s.ends_with(')') {
            let c = s[5..s.len() - 1].chars().next().unwrap_or(' ');
            Ok(KeyCode::Char(c))
        } else if let Some(num_str) = s.strip_prefix("F") {
            let n: u8 = num_str.parse().unwrap_or(1);
            Ok(KeyCode::F(n))
        } else {
            Ok(match s.as_str() {
                "Enter" => KeyCode::Enter,
                "Esc" => KeyCode::Esc,
                "Tab" => KeyCode::Tab,
                "Backspace" => KeyCode::Backspace,
                "Left" => KeyCode::Left,
                "Right" => KeyCode::Right,
                "Up" => KeyCode::Up,
                "Down" => KeyCode::Down,
                "Home" => KeyCode::Home,
                "End" => KeyCode::End,
                _ => KeyCode::Null,
            })
        }
    }
}

mod keymodifiers_serde {
    use crossterm::event::KeyModifiers;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(mods: &KeyModifiers, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(mods.bits())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyModifiers, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u8::deserialize(deserializer)?;
        Ok(KeyModifiers::from_bits_truncate(bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keybinding_creation() {
        let binding = KeyBinding::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        assert_eq!(binding.code, KeyCode::Char('a'));
        assert!(binding.has_ctrl());
    }

    #[test]
    fn test_keybinding_convenience_methods() {
        let ctrl = KeyBinding::ctrl(KeyCode::Char('c'));
        assert!(ctrl.has_ctrl());

        let alt = KeyBinding::alt(KeyCode::Char('f'));
        assert!(alt.has_alt());

        let shift = KeyBinding::shift(KeyCode::Tab);
        assert!(shift.has_shift());

        let plain = KeyBinding::plain(KeyCode::Enter);
        assert!(plain.modifiers.is_empty());
    }

    #[test]
    fn test_keybinding_display() {
        let ctrl_s = KeyBinding::ctrl(KeyCode::Char('s'));
        assert_eq!(ctrl_s.to_string(), "Ctrl+S");

        let ctrl_shift_p = KeyBinding::new(
            KeyCode::Char('p'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        );
        assert_eq!(ctrl_shift_p.to_string(), "Ctrl+Shift+P");

        let esc = KeyBinding::plain(KeyCode::Esc);
        assert_eq!(esc.to_string(), "Esc");
    }

    #[test]
    fn test_keybindings_operations() {
        let mut bindings = KeyBindings::new();
        assert_eq!(bindings.len(), 0);
        assert!(bindings.is_empty());

        bindings.bind(KeyBinding::ctrl(KeyCode::Char('q')), "quit");
        assert_eq!(bindings.len(), 1);

        let key = KeyBinding::ctrl(KeyCode::Char('q'));
        assert_eq!(bindings.get(&key), Some(&"quit".to_string()));

        bindings.unbind(&key);
        assert_eq!(bindings.len(), 0);
    }

    #[test]
    fn test_default_bindings() {
        let bindings = KeyBindings::default_bindings();
        assert!(!bindings.is_empty());

        let quit_key = KeyBinding::ctrl(KeyCode::Char('q'));
        assert!(bindings.get(&quit_key).is_some());
    }

    #[test]
    fn test_keybinding_serialization() {
        let binding = KeyBinding::ctrl(KeyCode::Char('s'));
        let json = serde_json::to_string(&binding).unwrap();
        let deserialized: KeyBinding = serde_json::from_str(&json).unwrap();
        assert_eq!(binding, deserialized);
    }
}
