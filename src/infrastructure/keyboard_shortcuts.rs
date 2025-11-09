//! Application-level keyboard shortcuts system
//!
//! Provides a comprehensive registry of keyboard shortcuts for efficient
//! power user navigation and actions across the application.
//!
//! # Examples
//!
//! ```
//! use toad::infrastructure::keyboard_shortcuts::{ShortcutRegistry, ShortcutAction, ShortcutCategory};
//!
//! let registry = ShortcutRegistry::with_defaults();
//! let shortcuts = registry.get_by_category(ShortcutCategory::Navigation);
//! ```

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shortcut action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutAction {
    // Navigation
    /// Move up (h/k/↑)
    MoveUp,
    /// Move down (j/↓)
    MoveDown,
    /// Move left (h/←)
    MoveLeft,
    /// Move right (l/→)
    MoveRight,
    /// Jump to top (g/Home)
    JumpToTop,
    /// Jump to bottom (G/End)
    JumpToBottom,
    /// Page up (Ctrl+u/PgUp)
    PageUp,
    /// Page down (Ctrl+d/PgDn)
    PageDown,
    /// Go back (Backspace)
    GoBack,
    /// Go forward (Space)
    GoForward,

    // File operations
    /// Create/new file (c/Ctrl+n)
    Create,
    /// Edit/open (e/Enter)
    Edit,
    /// Delete (d/Del)
    Delete,
    /// Rename (r/F2)
    Rename,
    /// Copy (y/Ctrl+c)
    Copy,
    /// Paste (p/Ctrl+v)
    Paste,
    /// Cut (x/Ctrl+x)
    Cut,
    /// Save (s/Ctrl+s)
    Save,

    // Search & filter
    /// Search (//:)
    Search,
    /// Filter (f/Ctrl+f)
    Filter,
    /// Next result (n)
    NextResult,
    /// Previous result (N/Shift+n)
    PreviousResult,
    /// Clear search (Esc)
    ClearSearch,

    // View & display
    /// Toggle view (v)
    ToggleView,
    /// Fullscreen (F11)
    Fullscreen,
    /// Toggle sidebar (Ctrl+b)
    ToggleSidebar,
    /// Toggle panel (Ctrl+\)
    TogglePanel,
    /// Zoom in (Ctrl++)
    ZoomIn,
    /// Zoom out (Ctrl+-)
    ZoomOut,
    /// Reset zoom (Ctrl+0)
    ResetZoom,

    // Workspace
    /// New tab (t/Ctrl+t)
    NewTab,
    /// Close tab (Ctrl+w)
    CloseTab,
    /// Next tab (Tab/gt)
    NextTab,
    /// Previous tab (Shift+Tab/gT)
    PreviousTab,
    /// Switch workspace (Ctrl+1-9)
    SwitchWorkspace(u8),

    // Window management
    /// New window (Ctrl+Shift+n)
    NewWindow,
    /// Close window (Ctrl+Shift+w)
    CloseWindow,
    /// Next window (Ctrl+Tab)
    NextWindow,
    /// Previous window (Ctrl+Shift+Tab)
    PreviousWindow,
    /// Minimize (Ctrl+m)
    Minimize,

    // Git
    /// Stage (s)
    Stage,
    /// Unstage (u)
    Unstage,
    /// Commit (c)
    Commit,
    /// Push (p)
    Push,
    /// Pull (P/Shift+p)
    Pull,
    /// Diff (d)
    Diff,
    /// Log (l)
    Log,

    // AI specific
    /// Send message (Enter/Ctrl+Enter)
    Send,
    /// Stop generation (Esc/Ctrl+c)
    Stop,
    /// Accept (a/Ctrl+a)
    Accept,
    /// Reject (r/Ctrl+r)
    Reject,
    /// Toggle context (Ctrl+k)
    ToggleContext,

    // General
    /// Command palette (Ctrl+p/Ctrl+Shift+p)
    CommandPalette,
    /// Help (?/F1)
    Help,
    /// Quit (q/Ctrl+q)
    Quit,
    /// Cancel/Escape (Esc)
    Cancel,
    /// Confirm (Enter/y)
    Confirm,
    /// Undo (u/Ctrl+z)
    Undo,
    /// Redo (Ctrl+r/Ctrl+y)
    Redo,
    /// Refresh (r/F5)
    Refresh,
}

impl ShortcutAction {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            ShortcutAction::MoveUp => "Move Up",
            ShortcutAction::MoveDown => "Move Down",
            ShortcutAction::MoveLeft => "Move Left",
            ShortcutAction::MoveRight => "Move Right",
            ShortcutAction::JumpToTop => "Jump to Top",
            ShortcutAction::JumpToBottom => "Jump to Bottom",
            ShortcutAction::PageUp => "Page Up",
            ShortcutAction::PageDown => "Page Down",
            ShortcutAction::GoBack => "Go Back",
            ShortcutAction::GoForward => "Go Forward",
            ShortcutAction::Create => "Create",
            ShortcutAction::Edit => "Edit",
            ShortcutAction::Delete => "Delete",
            ShortcutAction::Rename => "Rename",
            ShortcutAction::Copy => "Copy",
            ShortcutAction::Paste => "Paste",
            ShortcutAction::Cut => "Cut",
            ShortcutAction::Save => "Save",
            ShortcutAction::Search => "Search",
            ShortcutAction::Filter => "Filter",
            ShortcutAction::NextResult => "Next Result",
            ShortcutAction::PreviousResult => "Previous Result",
            ShortcutAction::ClearSearch => "Clear Search",
            ShortcutAction::ToggleView => "Toggle View",
            ShortcutAction::Fullscreen => "Fullscreen",
            ShortcutAction::ToggleSidebar => "Toggle Sidebar",
            ShortcutAction::TogglePanel => "Toggle Panel",
            ShortcutAction::ZoomIn => "Zoom In",
            ShortcutAction::ZoomOut => "Zoom Out",
            ShortcutAction::ResetZoom => "Reset Zoom",
            ShortcutAction::NewTab => "New Tab",
            ShortcutAction::CloseTab => "Close Tab",
            ShortcutAction::NextTab => "Next Tab",
            ShortcutAction::PreviousTab => "Previous Tab",
            ShortcutAction::SwitchWorkspace(_) => "Switch Workspace",
            ShortcutAction::NewWindow => "New Window",
            ShortcutAction::CloseWindow => "Close Window",
            ShortcutAction::NextWindow => "Next Window",
            ShortcutAction::PreviousWindow => "Previous Window",
            ShortcutAction::Minimize => "Minimize",
            ShortcutAction::Stage => "Stage",
            ShortcutAction::Unstage => "Unstage",
            ShortcutAction::Commit => "Commit",
            ShortcutAction::Push => "Push",
            ShortcutAction::Pull => "Pull",
            ShortcutAction::Diff => "Diff",
            ShortcutAction::Log => "Log",
            ShortcutAction::Send => "Send",
            ShortcutAction::Stop => "Stop",
            ShortcutAction::Accept => "Accept",
            ShortcutAction::Reject => "Reject",
            ShortcutAction::ToggleContext => "Toggle Context",
            ShortcutAction::CommandPalette => "Command Palette",
            ShortcutAction::Help => "Help",
            ShortcutAction::Quit => "Quit",
            ShortcutAction::Cancel => "Cancel",
            ShortcutAction::Confirm => "Confirm",
            ShortcutAction::Undo => "Undo",
            ShortcutAction::Redo => "Redo",
            ShortcutAction::Refresh => "Refresh",
        }
    }

    /// Get category
    pub fn category(&self) -> ShortcutCategory {
        match self {
            ShortcutAction::MoveUp | ShortcutAction::MoveDown | ShortcutAction::MoveLeft |
            ShortcutAction::MoveRight | ShortcutAction::JumpToTop | ShortcutAction::JumpToBottom |
            ShortcutAction::PageUp | ShortcutAction::PageDown | ShortcutAction::GoBack |
            ShortcutAction::GoForward => ShortcutCategory::Navigation,

            ShortcutAction::Create | ShortcutAction::Edit | ShortcutAction::Delete |
            ShortcutAction::Rename | ShortcutAction::Copy | ShortcutAction::Paste |
            ShortcutAction::Cut | ShortcutAction::Save => ShortcutCategory::FileOps,

            ShortcutAction::Search | ShortcutAction::Filter | ShortcutAction::NextResult |
            ShortcutAction::PreviousResult | ShortcutAction::ClearSearch => ShortcutCategory::Search,

            ShortcutAction::ToggleView | ShortcutAction::Fullscreen | ShortcutAction::ToggleSidebar |
            ShortcutAction::TogglePanel | ShortcutAction::ZoomIn | ShortcutAction::ZoomOut |
            ShortcutAction::ResetZoom => ShortcutCategory::View,

            ShortcutAction::NewTab | ShortcutAction::CloseTab | ShortcutAction::NextTab |
            ShortcutAction::PreviousTab | ShortcutAction::SwitchWorkspace(_) => ShortcutCategory::Workspace,

            ShortcutAction::NewWindow | ShortcutAction::CloseWindow | ShortcutAction::NextWindow |
            ShortcutAction::PreviousWindow | ShortcutAction::Minimize => ShortcutCategory::Window,

            ShortcutAction::Stage | ShortcutAction::Unstage | ShortcutAction::Commit |
            ShortcutAction::Push | ShortcutAction::Pull | ShortcutAction::Diff |
            ShortcutAction::Log => ShortcutCategory::Git,

            ShortcutAction::Send | ShortcutAction::Stop | ShortcutAction::Accept |
            ShortcutAction::Reject | ShortcutAction::ToggleContext => ShortcutCategory::AI,

            ShortcutAction::CommandPalette | ShortcutAction::Help | ShortcutAction::Quit |
            ShortcutAction::Cancel | ShortcutAction::Confirm | ShortcutAction::Undo |
            ShortcutAction::Redo | ShortcutAction::Refresh => ShortcutCategory::General,
        }
    }
}

/// Shortcut category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutCategory {
    /// Navigation shortcuts
    Navigation,
    /// File operations
    FileOps,
    /// Search and filtering
    Search,
    /// View and display
    View,
    /// Workspace management
    Workspace,
    /// Window management
    Window,
    /// Git operations
    Git,
    /// AI-specific actions
    AI,
    /// General actions
    General,
}

impl ShortcutCategory {
    /// Get all categories
    pub fn all() -> &'static [ShortcutCategory] {
        &[
            ShortcutCategory::Navigation,
            ShortcutCategory::FileOps,
            ShortcutCategory::Search,
            ShortcutCategory::View,
            ShortcutCategory::Workspace,
            ShortcutCategory::Window,
            ShortcutCategory::Git,
            ShortcutCategory::AI,
            ShortcutCategory::General,
        ]
    }

    /// Get category name
    pub fn name(&self) -> &'static str {
        match self {
            ShortcutCategory::Navigation => "Navigation",
            ShortcutCategory::FileOps => "File Operations",
            ShortcutCategory::Search => "Search & Filter",
            ShortcutCategory::View => "View & Display",
            ShortcutCategory::Workspace => "Workspace",
            ShortcutCategory::Window => "Window Management",
            ShortcutCategory::Git => "Git",
            ShortcutCategory::AI => "AI Actions",
            ShortcutCategory::General => "General",
        }
    }
}

/// Keyboard shortcut definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    /// Action triggered by this shortcut
    pub action: ShortcutAction,
    /// Key binding
    pub key: KeyEvent,
    /// Description
    pub description: String,
    /// Whether this is the primary binding for the action
    pub primary: bool,
}

impl Shortcut {
    /// Create a new shortcut
    pub fn new(action: ShortcutAction, key: KeyEvent, description: impl Into<String>) -> Self {
        Self {
            action,
            key,
            description: description.into(),
            primary: true,
        }
    }

    /// Mark as alternate binding
    pub fn alternate(mut self) -> Self {
        self.primary = false;
        self
    }

    /// Format key binding for display
    pub fn format_key(&self) -> String {
        let mut parts = Vec::new();

        if self.key.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl");
        }
        if self.key.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt");
        }
        if self.key.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift");
        }

        let key_str = match self.key.code {
            KeyCode::Char(c) => c.to_string(),
            KeyCode::F(n) => format!("F{}", n),
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PgUp".to_string(),
            KeyCode::PageDown => "PgDn".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            _ => "?".to_string(),
        };

        parts.push(&key_str);
        parts.join("+")
    }
}

/// Shortcut registry
///
/// Manages application-wide keyboard shortcuts with support for
/// multiple bindings per action, categorization, and searching.
#[derive(Debug, Clone)]
pub struct ShortcutRegistry {
    /// All registered shortcuts
    shortcuts: Vec<Shortcut>,
    /// Lookup by key event
    by_key: HashMap<KeyEvent, ShortcutAction>,
    /// Lookup by action
    by_action: HashMap<ShortcutAction, Vec<KeyEvent>>,
}

impl ShortcutRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
            by_key: HashMap::new(),
            by_action: HashMap::new(),
        }
    }

    /// Create registry with default shortcuts
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_default_shortcuts();
        registry
    }

    /// Add a shortcut
    pub fn add_shortcut(&mut self, shortcut: Shortcut) {
        self.by_key.insert(shortcut.key, shortcut.action);
        self.by_action
            .entry(shortcut.action)
            .or_insert_with(Vec::new)
            .push(shortcut.key);
        self.shortcuts.push(shortcut);
    }

    /// Get action for key event
    pub fn get_action(&self, key: &KeyEvent) -> Option<ShortcutAction> {
        self.by_key.get(key).copied()
    }

    /// Get all shortcuts for an action
    pub fn get_shortcuts(&self, action: ShortcutAction) -> Vec<&Shortcut> {
        self.shortcuts
            .iter()
            .filter(|s| s.action == action)
            .collect()
    }

    /// Get all shortcuts in a category
    pub fn get_by_category(&self, category: ShortcutCategory) -> Vec<&Shortcut> {
        self.shortcuts
            .iter()
            .filter(|s| s.action.category() == category)
            .collect()
    }

    /// Get primary shortcut for an action
    pub fn get_primary(&self, action: ShortcutAction) -> Option<&Shortcut> {
        self.shortcuts
            .iter()
            .find(|s| s.action == action && s.primary)
    }

    /// Get all shortcuts
    pub fn all_shortcuts(&self) -> &[Shortcut] {
        &self.shortcuts
    }

    /// Remove shortcut by key
    pub fn remove_shortcut(&mut self, key: &KeyEvent) -> bool {
        if let Some(action) = self.by_key.remove(key) {
            if let Some(keys) = self.by_action.get_mut(&action) {
                keys.retain(|k| k != key);
                if keys.is_empty() {
                    self.by_action.remove(&action);
                }
            }
            self.shortcuts.retain(|s| s.key != *key);
            true
        } else {
            false
        }
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
        self.by_key.clear();
        self.by_action.clear();
    }

    /// Add default shortcuts
    fn add_default_shortcuts(&mut self) {
        use KeyCode::*;
        use KeyModifiers as M;

        // Helper to create key event
        let key = |code: KeyCode, mods: KeyModifiers| KeyEvent::new(code, mods);

        // Navigation
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveUp, key(Char('k'), M::NONE), "Move up"));
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveUp, key(Up, M::NONE), "Move up").alternate());
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveDown, key(Char('j'), M::NONE), "Move down"));
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveDown, key(Down, M::NONE), "Move down").alternate());
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveLeft, key(Char('h'), M::NONE), "Move left"));
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveLeft, key(Left, M::NONE), "Move left").alternate());
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveRight, key(Char('l'), M::NONE), "Move right"));
        self.add_shortcut(Shortcut::new(ShortcutAction::MoveRight, key(Right, M::NONE), "Move right").alternate());
        self.add_shortcut(Shortcut::new(ShortcutAction::JumpToTop, key(Char('g'), M::NONE), "Jump to top"));
        self.add_shortcut(Shortcut::new(ShortcutAction::JumpToBottom, key(Char('G'), M::SHIFT), "Jump to bottom"));
        self.add_shortcut(Shortcut::new(ShortcutAction::PageUp, key(Char('u'), M::CONTROL), "Page up"));
        self.add_shortcut(Shortcut::new(ShortcutAction::PageDown, key(Char('d'), M::CONTROL), "Page down"));

        // File operations
        self.add_shortcut(Shortcut::new(ShortcutAction::Create, key(Char('c'), M::NONE), "Create"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Edit, key(Char('e'), M::NONE), "Edit"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Delete, key(Char('d'), M::NONE), "Delete"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Rename, key(Char('r'), M::NONE), "Rename"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Save, key(Char('s'), M::CONTROL), "Save"));

        // Search
        self.add_shortcut(Shortcut::new(ShortcutAction::Search, key(Char('/'), M::NONE), "Search"));
        self.add_shortcut(Shortcut::new(ShortcutAction::NextResult, key(Char('n'), M::NONE), "Next result"));
        self.add_shortcut(Shortcut::new(ShortcutAction::PreviousResult, key(Char('N'), M::SHIFT), "Previous result"));

        // View
        self.add_shortcut(Shortcut::new(ShortcutAction::ToggleSidebar, key(Char('b'), M::CONTROL), "Toggle sidebar"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Fullscreen, key(F(11), M::NONE), "Toggle fullscreen"));

        // Tabs
        self.add_shortcut(Shortcut::new(ShortcutAction::NewTab, key(Char('t'), M::CONTROL), "New tab"));
        self.add_shortcut(Shortcut::new(ShortcutAction::CloseTab, key(Char('w'), M::CONTROL), "Close tab"));
        self.add_shortcut(Shortcut::new(ShortcutAction::NextTab, key(Tab, M::NONE), "Next tab"));
        self.add_shortcut(Shortcut::new(ShortcutAction::PreviousTab, key(Tab, M::SHIFT), "Previous tab"));

        // General
        self.add_shortcut(Shortcut::new(ShortcutAction::CommandPalette, key(Char('p'), M::CONTROL), "Command palette"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Help, key(Char('?'), M::SHIFT), "Show help"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Quit, key(Char('q'), M::NONE), "Quit"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Cancel, key(Esc, M::NONE), "Cancel"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Undo, key(Char('z'), M::CONTROL), "Undo"));
        self.add_shortcut(Shortcut::new(ShortcutAction::Redo, key(Char('y'), M::CONTROL), "Redo"));
    }
}

impl Default for ShortcutRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, mods)
    }

    #[test]
    fn test_shortcut_action_name() {
        assert_eq!(ShortcutAction::MoveUp.name(), "Move Up");
        assert_eq!(ShortcutAction::Save.name(), "Save");
        assert_eq!(ShortcutAction::CommandPalette.name(), "Command Palette");
    }

    #[test]
    fn test_shortcut_action_category() {
        assert_eq!(ShortcutAction::MoveUp.category(), ShortcutCategory::Navigation);
        assert_eq!(ShortcutAction::Save.category(), ShortcutCategory::FileOps);
        assert_eq!(ShortcutAction::Search.category(), ShortcutCategory::Search);
        assert_eq!(ShortcutAction::Stage.category(), ShortcutCategory::Git);
        assert_eq!(ShortcutAction::Send.category(), ShortcutCategory::AI);
    }

    #[test]
    fn test_shortcut_category_all() {
        let categories = ShortcutCategory::all();
        assert_eq!(categories.len(), 9);
        assert!(categories.contains(&ShortcutCategory::Navigation));
        assert!(categories.contains(&ShortcutCategory::Git));
    }

    #[test]
    fn test_shortcut_category_name() {
        assert_eq!(ShortcutCategory::Navigation.name(), "Navigation");
        assert_eq!(ShortcutCategory::FileOps.name(), "File Operations");
        assert_eq!(ShortcutCategory::AI.name(), "AI Actions");
    }

    #[test]
    fn test_shortcut_creation() {
        let key = make_key(KeyCode::Char('k'), KeyModifiers::NONE);
        let shortcut = Shortcut::new(ShortcutAction::MoveUp, key, "Move up");

        assert_eq!(shortcut.action, ShortcutAction::MoveUp);
        assert_eq!(shortcut.key, key);
        assert_eq!(shortcut.description, "Move up");
        assert!(shortcut.primary);
    }

    #[test]
    fn test_shortcut_alternate() {
        let key = make_key(KeyCode::Up, KeyModifiers::NONE);
        let shortcut = Shortcut::new(ShortcutAction::MoveUp, key, "Move up").alternate();

        assert!(!shortcut.primary);
    }

    #[test]
    fn test_shortcut_format_key() {
        let shortcut1 = Shortcut::new(
            ShortcutAction::MoveUp,
            make_key(KeyCode::Char('k'), KeyModifiers::NONE),
            "Move up"
        );
        assert_eq!(shortcut1.format_key(), "k");

        let shortcut2 = Shortcut::new(
            ShortcutAction::Save,
            make_key(KeyCode::Char('s'), KeyModifiers::CONTROL),
            "Save"
        );
        assert_eq!(shortcut2.format_key(), "Ctrl+s");

        let shortcut3 = Shortcut::new(
            ShortcutAction::MoveUp,
            make_key(KeyCode::Up, KeyModifiers::NONE),
            "Move up"
        );
        assert_eq!(shortcut3.format_key(), "↑");
    }

    #[test]
    fn test_registry_creation() {
        let registry = ShortcutRegistry::new();
        assert_eq!(registry.all_shortcuts().len(), 0);
    }

    #[test]
    fn test_registry_with_defaults() {
        let registry = ShortcutRegistry::with_defaults();
        assert!(registry.all_shortcuts().len() > 0);
    }

    #[test]
    fn test_registry_add_shortcut() {
        let mut registry = ShortcutRegistry::new();
        let key = make_key(KeyCode::Char('k'), KeyModifiers::NONE);
        let shortcut = Shortcut::new(ShortcutAction::MoveUp, key, "Move up");

        registry.add_shortcut(shortcut);
        assert_eq!(registry.all_shortcuts().len(), 1);
    }

    #[test]
    fn test_registry_get_action() {
        let registry = ShortcutRegistry::with_defaults();
        let key = make_key(KeyCode::Char('k'), KeyModifiers::NONE);

        assert_eq!(registry.get_action(&key), Some(ShortcutAction::MoveUp));
    }

    #[test]
    fn test_registry_get_shortcuts() {
        let registry = ShortcutRegistry::with_defaults();
        let shortcuts = registry.get_shortcuts(ShortcutAction::MoveUp);

        assert!(shortcuts.len() >= 1);
        assert!(shortcuts.iter().any(|s| s.action == ShortcutAction::MoveUp));
    }

    #[test]
    fn test_registry_get_by_category() {
        let registry = ShortcutRegistry::with_defaults();
        let nav_shortcuts = registry.get_by_category(ShortcutCategory::Navigation);

        assert!(nav_shortcuts.len() > 0);
        assert!(nav_shortcuts.iter().all(|s| s.action.category() == ShortcutCategory::Navigation));
    }

    #[test]
    fn test_registry_get_primary() {
        let registry = ShortcutRegistry::with_defaults();
        let primary = registry.get_primary(ShortcutAction::MoveUp);

        assert!(primary.is_some());
        assert!(primary.unwrap().primary);
    }

    #[test]
    fn test_registry_remove_shortcut() {
        let mut registry = ShortcutRegistry::with_defaults();
        let key = make_key(KeyCode::Char('k'), KeyModifiers::NONE);

        assert!(registry.get_action(&key).is_some());
        assert!(registry.remove_shortcut(&key));
        assert!(registry.get_action(&key).is_none());
    }

    #[test]
    fn test_registry_clear() {
        let mut registry = ShortcutRegistry::with_defaults();
        assert!(registry.all_shortcuts().len() > 0);

        registry.clear();
        assert_eq!(registry.all_shortcuts().len(), 0);
    }

    #[test]
    fn test_default_registry() {
        let registry = ShortcutRegistry::default();
        assert!(registry.all_shortcuts().len() > 0);
    }

    #[test]
    fn test_multiple_bindings_per_action() {
        let registry = ShortcutRegistry::with_defaults();

        // MoveUp should have both 'k' and arrow up
        let up_shortcuts = registry.get_shortcuts(ShortcutAction::MoveUp);
        assert!(up_shortcuts.len() >= 2);
    }

    #[test]
    fn test_primary_vs_alternate() {
        let registry = ShortcutRegistry::with_defaults();

        let primary = registry.get_primary(ShortcutAction::MoveUp);
        assert!(primary.is_some());

        let all = registry.get_shortcuts(ShortcutAction::MoveUp);
        let primary_count = all.iter().filter(|s| s.primary).count();
        assert_eq!(primary_count, 1);
    }
}
