//! Vim-style navigation support
//!
//! Provides Vim-inspired navigation keybindings (h/j/k/l, g/G, etc.)
//! for consistent keyboard-driven UI navigation.
//!
//! # Examples
//!
//! ## Basic Navigation
//!
//! ```
//! use toad::navigation::{VimNavigation, NavigationAction};
//! use crossterm::event::{KeyCode, KeyModifiers};
//!
//! let mut nav = VimNavigation::new();
//!
//! // h = left
//! let action = nav.handle_key(KeyCode::Char('h'), KeyModifiers::NONE);
//! assert_eq!(action, Some(NavigationAction::Left));
//!
//! // j = down
//! let action = nav.handle_key(KeyCode::Char('j'), KeyModifiers::NONE);
//! assert_eq!(action, Some(NavigationAction::Down));
//! ```
//!
//! ## Jump Navigation
//!
//! ```
//! use toad::navigation::{VimNavigation, NavigationAction};
//! use crossterm::event::{KeyCode, KeyModifiers};
//!
//! let mut nav = VimNavigation::new();
//!
//! // gg = jump to top
//! let action = nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE);
//! // Returns None, waiting for second 'g'
//! assert_eq!(action, None);
//! ```

use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize};

/// Navigation actions
///
/// # Examples
///
/// ```
/// use toad::navigation::NavigationAction;
///
/// let action = NavigationAction::Down;
/// assert!(matches!(action, NavigationAction::Down));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NavigationAction {
    /// Move left (h, Left arrow)
    Left,
    /// Move down (j, Down arrow)
    Down,
    /// Move up (k, Up arrow)
    Up,
    /// Move right (l, Right arrow)
    Right,
    /// Jump to top (gg, Ctrl+Home)
    JumpTop,
    /// Jump to bottom (G, Ctrl+End)
    JumpBottom,
    /// Page up (Ctrl+u, PageUp)
    PageUp,
    /// Page down (Ctrl+d, PageDown)
    PageDown,
    /// Half page up (Ctrl+u)
    HalfPageUp,
    /// Half page down (Ctrl+d)
    HalfPageDown,
    /// Jump to line start (0, Home)
    LineStart,
    /// Jump to line end ($, End)
    LineEnd,
    /// Word forward (w)
    WordForward,
    /// Word backward (b)
    WordBackward,
    /// Find character (f)
    FindChar(char),
    /// Find character backward (F)
    FindCharBackward(char),
}

/// Vim navigation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VimState {
    /// Normal state, no pending keys
    Normal,
    /// Waiting for second 'g' (for gg)
    PendingGG,
    /// Waiting for character to find (after f)
    PendingFind,
    /// Waiting for character to find backward (after F)
    PendingFindBackward,
}

/// Vim-style navigation handler
///
/// Handles Vim-inspired navigation keybindings with support for:
/// - Basic movement (h/j/k/l)
/// - Jump commands (gg, G)
/// - Page navigation (Ctrl+u, Ctrl+d)
/// - Word movement (w, b)
/// - Character finding (f, F)
///
/// # Examples
///
/// ```
/// use toad::navigation::VimNavigation;
/// use crossterm::event::{KeyCode, KeyModifiers};
///
/// let mut nav = VimNavigation::new();
///
/// // Enable Vim mode
/// nav.set_enabled(true);
///
/// // Handle 'j' key (down)
/// let action = nav.handle_key(KeyCode::Char('j'), KeyModifiers::NONE);
/// assert!(action.is_some());
/// ```
#[derive(Debug, Clone)]
pub struct VimNavigation {
    enabled: bool,
    state: VimState,
    case_sensitive: bool,
}

impl VimNavigation {
    /// Create a new Vim navigation handler
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::navigation::VimNavigation;
    ///
    /// let nav = VimNavigation::new();
    /// assert!(nav.is_enabled()); // Enabled by default
    /// ```
    pub fn new() -> Self {
        Self {
            enabled: true,
            state: VimState::Normal,
            case_sensitive: false,
        }
    }

    /// Enable or disable Vim navigation
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::navigation::VimNavigation;
    ///
    /// let mut nav = VimNavigation::new();
    /// nav.set_enabled(false);
    /// assert!(!nav.is_enabled());
    /// ```
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.state = VimState::Normal; // Reset state when disabled
        }
    }

    /// Check if Vim navigation is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set case sensitivity for character finding
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::navigation::VimNavigation;
    ///
    /// let mut nav = VimNavigation::new();
    /// nav.set_case_sensitive(true);
    /// assert!(nav.is_case_sensitive());
    /// ```
    pub fn set_case_sensitive(&mut self, sensitive: bool) {
        self.case_sensitive = sensitive;
    }

    /// Check if case sensitive mode is enabled
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }

    /// Reset navigation state
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::navigation::VimNavigation;
    /// use crossterm::event::{KeyCode, KeyModifiers};
    ///
    /// let mut nav = VimNavigation::new();
    /// nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE); // Pending gg
    /// nav.reset();
    /// // State is now reset to Normal
    /// ```
    pub fn reset(&mut self) {
        self.state = VimState::Normal;
    }

    /// Handle a key press and return navigation action
    ///
    /// Returns `Some(NavigationAction)` if the key maps to a navigation action,
    /// or `None` if waiting for more input or key is not mapped.
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::navigation::{VimNavigation, NavigationAction};
    /// use crossterm::event::{KeyCode, KeyModifiers};
    ///
    /// let mut nav = VimNavigation::new();
    ///
    /// // Basic movement
    /// assert_eq!(
    ///     nav.handle_key(KeyCode::Char('h'), KeyModifiers::NONE),
    ///     Some(NavigationAction::Left)
    /// );
    /// assert_eq!(
    ///     nav.handle_key(KeyCode::Char('j'), KeyModifiers::NONE),
    ///     Some(NavigationAction::Down)
    /// );
    /// ```
    pub fn handle_key(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Option<NavigationAction> {
        if !self.enabled {
            return None;
        }

        match self.state {
            VimState::Normal => self.handle_normal(code, modifiers),
            VimState::PendingGG => self.handle_pending_gg(code),
            VimState::PendingFind => self.handle_pending_find(code),
            VimState::PendingFindBackward => self.handle_pending_find_backward(code),
        }
    }

    fn handle_normal(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Option<NavigationAction> {
        match (code, modifiers) {
            // Basic movement (h/j/k/l)
            (KeyCode::Char('h'), KeyModifiers::NONE) => Some(NavigationAction::Left),
            (KeyCode::Char('j'), KeyModifiers::NONE) => Some(NavigationAction::Down),
            (KeyCode::Char('k'), KeyModifiers::NONE) => Some(NavigationAction::Up),
            (KeyCode::Char('l'), KeyModifiers::NONE) => Some(NavigationAction::Right),

            // Arrow keys (always work)
            (KeyCode::Left, _) => Some(NavigationAction::Left),
            (KeyCode::Down, _) => Some(NavigationAction::Down),
            (KeyCode::Up, _) => Some(NavigationAction::Up),
            (KeyCode::Right, _) => Some(NavigationAction::Right),

            // Jump to top (gg)
            (KeyCode::Char('g'), KeyModifiers::NONE) => {
                self.state = VimState::PendingGG;
                None // Wait for second 'g'
            }

            // Jump to bottom (G)
            (KeyCode::Char('G'), KeyModifiers::SHIFT) => Some(NavigationAction::JumpBottom),

            // Page navigation
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => Some(NavigationAction::HalfPageUp),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(NavigationAction::HalfPageDown),
            (KeyCode::PageUp, _) => Some(NavigationAction::PageUp),
            (KeyCode::PageDown, _) => Some(NavigationAction::PageDown),

            // Line navigation
            (KeyCode::Char('0'), KeyModifiers::NONE) => Some(NavigationAction::LineStart),
            (KeyCode::Char('$'), KeyModifiers::SHIFT) => Some(NavigationAction::LineEnd),
            (KeyCode::Home, _) => Some(NavigationAction::LineStart),
            (KeyCode::End, _) => Some(NavigationAction::LineEnd),

            // Word movement
            (KeyCode::Char('w'), KeyModifiers::NONE) => Some(NavigationAction::WordForward),
            (KeyCode::Char('b'), KeyModifiers::NONE) => Some(NavigationAction::WordBackward),

            // Character finding
            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                self.state = VimState::PendingFind;
                None // Wait for character
            }
            (KeyCode::Char('F'), KeyModifiers::SHIFT) => {
                self.state = VimState::PendingFindBackward;
                None // Wait for character
            }

            _ => None,
        }
    }

    fn handle_pending_gg(&mut self, code: KeyCode) -> Option<NavigationAction> {
        self.state = VimState::Normal;
        match code {
            KeyCode::Char('g') => Some(NavigationAction::JumpTop),
            _ => None, // Invalid sequence, reset
        }
    }

    fn handle_pending_find(&mut self, code: KeyCode) -> Option<NavigationAction> {
        self.state = VimState::Normal;
        match code {
            KeyCode::Char(c) => Some(NavigationAction::FindChar(c)),
            _ => None,
        }
    }

    fn handle_pending_find_backward(&mut self, code: KeyCode) -> Option<NavigationAction> {
        self.state = VimState::Normal;
        match code {
            KeyCode::Char(c) => Some(NavigationAction::FindCharBackward(c)),
            _ => None,
        }
    }
}

impl Default for VimNavigation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vim_navigation_creation() {
        let nav = VimNavigation::new();
        assert!(nav.is_enabled());
        assert!(!nav.is_case_sensitive());
    }

    #[test]
    fn test_vim_navigation_enable_disable() {
        let mut nav = VimNavigation::new();
        nav.set_enabled(false);
        assert!(!nav.is_enabled());

        // Disabled navigation returns None
        let action = nav.handle_key(KeyCode::Char('h'), KeyModifiers::NONE);
        assert_eq!(action, None);

        nav.set_enabled(true);
        let action = nav.handle_key(KeyCode::Char('h'), KeyModifiers::NONE);
        assert_eq!(action, Some(NavigationAction::Left));
    }

    #[test]
    fn test_vim_basic_movement() {
        let mut nav = VimNavigation::new();

        assert_eq!(
            nav.handle_key(KeyCode::Char('h'), KeyModifiers::NONE),
            Some(NavigationAction::Left)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Char('j'), KeyModifiers::NONE),
            Some(NavigationAction::Down)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Char('k'), KeyModifiers::NONE),
            Some(NavigationAction::Up)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Char('l'), KeyModifiers::NONE),
            Some(NavigationAction::Right)
        );
    }

    #[test]
    fn test_vim_arrow_keys() {
        let mut nav = VimNavigation::new();

        assert_eq!(
            nav.handle_key(KeyCode::Left, KeyModifiers::NONE),
            Some(NavigationAction::Left)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Down, KeyModifiers::NONE),
            Some(NavigationAction::Down)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Up, KeyModifiers::NONE),
            Some(NavigationAction::Up)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Right, KeyModifiers::NONE),
            Some(NavigationAction::Right)
        );
    }

    #[test]
    fn test_vim_jump_top_gg() {
        let mut nav = VimNavigation::new();

        // First 'g' - should return None (waiting)
        let action = nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE);
        assert_eq!(action, None);

        // Second 'g' - should jump to top
        let action = nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE);
        assert_eq!(action, Some(NavigationAction::JumpTop));
    }

    #[test]
    fn test_vim_jump_bottom() {
        let mut nav = VimNavigation::new();

        let action = nav.handle_key(KeyCode::Char('G'), KeyModifiers::SHIFT);
        assert_eq!(action, Some(NavigationAction::JumpBottom));
    }

    #[test]
    fn test_vim_page_navigation() {
        let mut nav = VimNavigation::new();

        assert_eq!(
            nav.handle_key(KeyCode::Char('u'), KeyModifiers::CONTROL),
            Some(NavigationAction::HalfPageUp)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Char('d'), KeyModifiers::CONTROL),
            Some(NavigationAction::HalfPageDown)
        );
        assert_eq!(
            nav.handle_key(KeyCode::PageUp, KeyModifiers::NONE),
            Some(NavigationAction::PageUp)
        );
        assert_eq!(
            nav.handle_key(KeyCode::PageDown, KeyModifiers::NONE),
            Some(NavigationAction::PageDown)
        );
    }

    #[test]
    fn test_vim_line_navigation() {
        let mut nav = VimNavigation::new();

        assert_eq!(
            nav.handle_key(KeyCode::Char('0'), KeyModifiers::NONE),
            Some(NavigationAction::LineStart)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Char('$'), KeyModifiers::SHIFT),
            Some(NavigationAction::LineEnd)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Home, KeyModifiers::NONE),
            Some(NavigationAction::LineStart)
        );
        assert_eq!(
            nav.handle_key(KeyCode::End, KeyModifiers::NONE),
            Some(NavigationAction::LineEnd)
        );
    }

    #[test]
    fn test_vim_word_movement() {
        let mut nav = VimNavigation::new();

        assert_eq!(
            nav.handle_key(KeyCode::Char('w'), KeyModifiers::NONE),
            Some(NavigationAction::WordForward)
        );
        assert_eq!(
            nav.handle_key(KeyCode::Char('b'), KeyModifiers::NONE),
            Some(NavigationAction::WordBackward)
        );
    }

    #[test]
    fn test_vim_find_character() {
        let mut nav = VimNavigation::new();

        // f - find forward
        let action = nav.handle_key(KeyCode::Char('f'), KeyModifiers::NONE);
        assert_eq!(action, None); // Waiting for character

        let action = nav.handle_key(KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(action, Some(NavigationAction::FindChar('x')));
    }

    #[test]
    fn test_vim_find_character_backward() {
        let mut nav = VimNavigation::new();

        // F - find backward
        let action = nav.handle_key(KeyCode::Char('F'), KeyModifiers::SHIFT);
        assert_eq!(action, None); // Waiting for character

        let action = nav.handle_key(KeyCode::Char('y'), KeyModifiers::NONE);
        assert_eq!(action, Some(NavigationAction::FindCharBackward('y')));
    }

    #[test]
    fn test_vim_reset() {
        let mut nav = VimNavigation::new();

        // Start gg sequence
        nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE);

        // Reset before completing
        nav.reset();

        // Should be back to normal state
        let action = nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE);
        assert_eq!(action, None); // Waiting for second 'g' again
    }

    #[test]
    fn test_vim_case_sensitive() {
        let mut nav = VimNavigation::new();
        assert!(!nav.is_case_sensitive());

        nav.set_case_sensitive(true);
        assert!(nav.is_case_sensitive());
    }

    #[test]
    fn test_vim_invalid_sequence() {
        let mut nav = VimNavigation::new();

        // Start gg sequence
        let action = nav.handle_key(KeyCode::Char('g'), KeyModifiers::NONE);
        assert_eq!(action, None);

        // Press invalid key
        let action = nav.handle_key(KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(action, None); // Should reset and return None
    }

    #[test]
    fn test_default() {
        let nav = VimNavigation::default();
        assert!(nav.is_enabled());
    }
}
