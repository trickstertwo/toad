/// User-defined command aliases
///
/// Allows users to create custom shortcuts for commands and command sequences
///
/// # Examples
///
/// ```
/// use toad::aliases::AliasManager;
///
/// let mut manager = AliasManager::new();
/// manager.add("gs", "git status");
/// manager.add("w", "write");
///
/// assert_eq!(manager.expand("gs"), Some("git status".to_string()));
/// ```
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Maximum recursion depth for alias expansion
const MAX_RECURSION_DEPTH: usize = 10;

/// An alias definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alias {
    /// The short name/trigger
    pub name: String,
    /// The expanded command(s)
    pub expansion: String,
    /// Optional description
    pub description: Option<String>,
}

impl Alias {
    /// Create a new alias
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::Alias;
    ///
    /// let alias = Alias::new("gs", "git status");
    /// assert_eq!(alias.name, "gs");
    /// assert_eq!(alias.expansion, "git status");
    /// ```
    pub fn new(name: impl Into<String>, expansion: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expansion: expansion.into(),
            description: None,
        }
    }

    /// Set description
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::Alias;
    ///
    /// let alias = Alias::new("gs", "git status")
    ///     .with_description("Show git status");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Expand the alias with arguments
    ///
    /// Replaces $1, $2, etc. with provided arguments
    /// $@ expands to all arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::Alias;
    ///
    /// let alias = Alias::new("gc", "git commit -m \"$1\"");
    /// let expanded = alias.expand(&["fix bug"]);
    /// assert_eq!(expanded, "git commit -m \"fix bug\"");
    /// ```
    pub fn expand(&self, args: &[&str]) -> String {
        let mut result = self.expansion.clone();

        // Replace $@ with all args
        if result.contains("$@") {
            result = result.replace("$@", &args.join(" "));
        }

        // Replace $1, $2, etc.
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("${}", i + 1);
            result = result.replace(&placeholder, arg);
        }

        result
    }
}

/// Manager for user-defined aliases
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AliasManager {
    /// Aliases by name
    aliases: HashMap<String, Alias>,
}

impl AliasManager {
    /// Create a new alias manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let manager = AliasManager::new();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    /// Create manager with common defaults
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let manager = AliasManager::with_defaults();
    /// assert!(manager.count() > 0);
    /// ```
    pub fn with_defaults() -> Self {
        let mut manager = Self::new();

        // Common shortcuts
        manager.add("q", "quit");
        manager.add("w", "write");
        manager.add("wq", "write; quit");
        manager.add("x", "write; quit");

        // Git shortcuts
        manager.add("gs", "git status");
        manager.add("ga", "git add");
        manager.add("gc", "git commit -m \"$1\"");
        manager.add("gp", "git push");
        manager.add("gl", "git log");
        manager.add("gd", "git diff");

        // File operations
        manager.add("e", "edit $1");
        manager.add("o", "open $1");

        manager
    }

    /// Add an alias
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add("gs", "git status");
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn add(&mut self, name: impl Into<String>, expansion: impl Into<String>) {
        let name = name.into();
        let alias = Alias::new(name.clone(), expansion);
        self.aliases.insert(name, alias);
    }

    /// Add an alias with description
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add_with_description("gs", "git status", "Show git status");
    /// ```
    pub fn add_with_description(
        &mut self,
        name: impl Into<String>,
        expansion: impl Into<String>,
        description: impl Into<String>,
    ) {
        let name = name.into();
        let alias = Alias::new(name.clone(), expansion).with_description(description);
        self.aliases.insert(name, alias);
    }

    /// Remove an alias
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add("gs", "git status");
    /// manager.remove("gs");
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn remove(&mut self, name: &str) -> Option<Alias> {
        self.aliases.remove(name)
    }

    /// Get an alias
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add("gs", "git status");
    /// assert!(manager.get("gs").is_some());
    /// ```
    pub fn get(&self, name: &str) -> Option<&Alias> {
        self.aliases.get(name)
    }

    /// Expand an alias (simple, no args)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add("gs", "git status");
    /// assert_eq!(manager.expand("gs"), Some("git status".to_string()));
    /// ```
    pub fn expand(&self, name: &str) -> Option<String> {
        self.expand_with_args(name, &[])
    }

    /// Expand an alias with arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add("gc", "git commit -m \"$1\"");
    /// let expanded = manager.expand_with_args("gc", &["fix bug"]).unwrap();
    /// assert_eq!(expanded, "git commit -m \"fix bug\"");
    /// ```
    pub fn expand_with_args(&self, name: &str, args: &[&str]) -> Option<String> {
        self.expand_recursive(name, args, 0)
    }

    /// Recursively expand aliases (prevents infinite loops)
    fn expand_recursive(&self, name: &str, args: &[&str], depth: usize) -> Option<String> {
        if depth >= MAX_RECURSION_DEPTH {
            return None; // Prevent infinite recursion
        }

        let alias = self.aliases.get(name)?;
        let expanded = alias.expand(args);

        // Check if the expanded form starts with another alias
        let first_word = expanded.split_whitespace().next()?;

        if let Some(_) = self.aliases.get(first_word) {
            // Recursively expand
            self.expand_recursive(first_word, &[], depth + 1)
        } else {
            Some(expanded)
        }
    }

    /// Get all aliases
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let manager = AliasManager::with_defaults();
    /// let all = manager.all();
    /// assert!(!all.is_empty());
    /// ```
    pub fn all(&self) -> Vec<&Alias> {
        self.aliases.values().collect()
    }

    /// Get count of aliases
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::new();
    /// manager.add("gs", "git status");
    /// assert_eq!(manager.count(), 1);
    /// ```
    pub fn count(&self) -> usize {
        self.aliases.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }

    /// Clear all aliases
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager = AliasManager::with_defaults();
    /// manager.clear();
    /// assert_eq!(manager.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.aliases.clear();
    }

    /// Search for aliases by pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let manager = AliasManager::with_defaults();
    /// let git_aliases = manager.search("git");
    /// assert!(!git_aliases.is_empty());
    /// ```
    pub fn search(&self, pattern: &str) -> Vec<&Alias> {
        let pattern_lower = pattern.to_lowercase();
        self.aliases
            .values()
            .filter(|a| {
                a.name.to_lowercase().contains(&pattern_lower)
                    || a.expansion.to_lowercase().contains(&pattern_lower)
                    || a.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&pattern_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    /// Load aliases from JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::aliases::AliasManager;
    ///
    /// let manager = AliasManager::load_from_file("aliases.json").unwrap();
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).context("Failed to read aliases file")?;
        let manager: Self =
            serde_json::from_str(&content).context("Failed to parse aliases JSON")?;
        Ok(manager)
    }

    /// Save aliases to JSON file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use toad::aliases::AliasManager;
    ///
    /// let manager = AliasManager::with_defaults();
    /// manager.save_to_file("aliases.json").unwrap();
    /// ```
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Failed to serialize aliases")?;
        fs::write(path.as_ref(), json).context("Failed to write aliases file")?;
        Ok(())
    }

    /// Merge another alias manager into this one
    ///
    /// Existing aliases with the same name are overwritten
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::aliases::AliasManager;
    ///
    /// let mut manager1 = AliasManager::new();
    /// manager1.add("gs", "git status");
    ///
    /// let mut manager2 = AliasManager::new();
    /// manager2.add("ga", "git add");
    ///
    /// manager1.merge(manager2);
    /// assert_eq!(manager1.count(), 2);
    /// ```
    pub fn merge(&mut self, other: AliasManager) {
        self.aliases.extend(other.aliases);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_creation() {
        let alias = Alias::new("gs", "git status");
        assert_eq!(alias.name, "gs");
        assert_eq!(alias.expansion, "git status");
        assert!(alias.description.is_none());
    }

    #[test]
    fn test_alias_with_description() {
        let alias = Alias::new("gs", "git status").with_description("Show git status");
        assert_eq!(alias.description, Some("Show git status".to_string()));
    }

    #[test]
    fn test_alias_expand_no_args() {
        let alias = Alias::new("gs", "git status");
        assert_eq!(alias.expand(&[]), "git status");
    }

    #[test]
    fn test_alias_expand_with_args() {
        let alias = Alias::new("gc", "git commit -m \"$1\"");
        assert_eq!(alias.expand(&["fix bug"]), "git commit -m \"fix bug\"");
    }

    #[test]
    fn test_alias_expand_multiple_args() {
        let alias = Alias::new("cmd", "command $1 $2");
        assert_eq!(alias.expand(&["arg1", "arg2"]), "command arg1 arg2");
    }

    #[test]
    fn test_alias_expand_all_args() {
        let alias = Alias::new("echo", "echo $@");
        assert_eq!(alias.expand(&["hello", "world"]), "echo hello world");
    }

    #[test]
    fn test_manager_creation() {
        let manager = AliasManager::new();
        assert_eq!(manager.count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_manager_with_defaults() {
        let manager = AliasManager::with_defaults();
        assert!(manager.count() > 0);
        assert!(manager.get("q").is_some());
        assert!(manager.get("w").is_some());
    }

    #[test]
    fn test_manager_add() {
        let mut manager = AliasManager::new();
        manager.add("gs", "git status");
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_manager_add_with_description() {
        let mut manager = AliasManager::new();
        manager.add_with_description("gs", "git status", "Show status");
        let alias = manager.get("gs").unwrap();
        assert_eq!(alias.description, Some("Show status".to_string()));
    }

    #[test]
    fn test_manager_remove() {
        let mut manager = AliasManager::new();
        manager.add("gs", "git status");
        manager.remove("gs");
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_get() {
        let mut manager = AliasManager::new();
        manager.add("gs", "git status");
        assert!(manager.get("gs").is_some());
        assert!(manager.get("nonexistent").is_none());
    }

    #[test]
    fn test_manager_expand() {
        let mut manager = AliasManager::new();
        manager.add("gs", "git status");
        assert_eq!(manager.expand("gs"), Some("git status".to_string()));
    }

    #[test]
    fn test_manager_expand_with_args() {
        let mut manager = AliasManager::new();
        manager.add("gc", "git commit -m \"$1\"");
        let expanded = manager.expand_with_args("gc", &["fix"]).unwrap();
        assert_eq!(expanded, "git commit -m \"fix\"");
    }

    #[test]
    fn test_manager_all() {
        let manager = AliasManager::with_defaults();
        let all = manager.all();
        assert!(!all.is_empty());
    }

    #[test]
    fn test_manager_clear() {
        let mut manager = AliasManager::with_defaults();
        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_manager_search() {
        let manager = AliasManager::with_defaults();
        let git_aliases = manager.search("git");
        assert!(!git_aliases.is_empty());
    }

    #[test]
    fn test_manager_merge() {
        let mut manager1 = AliasManager::new();
        manager1.add("gs", "git status");

        let mut manager2 = AliasManager::new();
        manager2.add("ga", "git add");

        manager1.merge(manager2);
        assert_eq!(manager1.count(), 2);
    }

    #[test]
    fn test_serialization() {
        let manager = AliasManager::with_defaults();
        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: AliasManager = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.count(), manager.count());
    }

    #[test]
    fn test_recursion_limit() {
        let mut manager = AliasManager::new();
        // Create circular aliases
        manager.add("a", "b");
        manager.add("b", "c");
        manager.add("c", "a"); // Circular!

        // Should not crash, returns None due to recursion limit
        assert!(manager.expand("a").is_none());
    }
}
