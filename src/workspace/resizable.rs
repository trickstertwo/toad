/// Resizable panes for dynamic layout adjustment
///
/// Allows panes to be resized with drag borders or keybindings
///
/// # Examples
///
/// ```
/// use toad::resizable::{ResizablePane, ResizeDirection};
///
/// let mut pane = ResizablePane::new("main", 50);
/// pane.resize(ResizeDirection::Grow, 10);
/// assert_eq!(pane.size(), 60);
/// ```
use serde::{Deserialize, Serialize};

/// Resize direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeDirection {
    /// Grow the pane
    Grow,
    /// Shrink the pane
    Shrink,
}

/// Resizable pane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizablePane {
    /// Pane identifier
    id: String,
    /// Current size (percentage or absolute)
    size: u16,
    /// Minimum size
    min_size: u16,
    /// Maximum size
    max_size: u16,
    /// Whether pane can be resized
    resizable: bool,
}

impl ResizablePane {
    /// Create a new resizable pane
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::resizable::ResizablePane;
    ///
    /// let pane = ResizablePane::new("main", 50);
    /// assert_eq!(pane.id(), "main");
    /// assert_eq!(pane.size(), 50);
    /// ```
    pub fn new(id: impl Into<String>, size: u16) -> Self {
        Self {
            id: id.into(),
            size,
            min_size: 10,
            max_size: 90,
            resizable: true,
        }
    }

    /// Set minimum size
    pub fn min_size(mut self, min: u16) -> Self {
        self.min_size = min;
        self
    }

    /// Set maximum size
    pub fn max_size(mut self, max: u16) -> Self {
        self.max_size = max;
        self
    }

    /// Set resizable
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Get pane ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get current size
    pub fn size(&self) -> u16 {
        self.size
    }

    /// Set size
    pub fn set_size(&mut self, size: u16) {
        if self.resizable {
            self.size = size.clamp(self.min_size, self.max_size);
        }
    }

    /// Resize pane
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::resizable::{ResizablePane, ResizeDirection};
    ///
    /// let mut pane = ResizablePane::new("main", 50);
    /// pane.resize(ResizeDirection::Grow, 10);
    /// assert_eq!(pane.size(), 60);
    /// ```
    pub fn resize(&mut self, direction: ResizeDirection, amount: u16) {
        if !self.resizable {
            return;
        }

        let new_size = match direction {
            ResizeDirection::Grow => self.size.saturating_add(amount),
            ResizeDirection::Shrink => self.size.saturating_sub(amount),
        };

        self.set_size(new_size);
    }

    /// Check if pane can be resized
    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    /// Get minimum size
    pub fn get_min_size(&self) -> u16 {
        self.min_size
    }

    /// Get maximum size
    pub fn get_max_size(&self) -> u16 {
        self.max_size
    }

    /// Check if at minimum size
    pub fn is_at_min_size(&self) -> bool {
        self.size <= self.min_size
    }

    /// Check if at maximum size
    pub fn is_at_max_size(&self) -> bool {
        self.size >= self.max_size
    }
}

/// Resizable pane manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizablePaneManager {
    /// All panes
    panes: Vec<ResizablePane>,
    /// Currently focused pane index
    focused: Option<usize>,
}

impl ResizablePaneManager {
    /// Create a new resizable pane manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::resizable::ResizablePaneManager;
    ///
    /// let manager = ResizablePaneManager::new();
    /// assert_eq!(manager.pane_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            panes: Vec::new(),
            focused: None,
        }
    }

    /// Add a pane
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::resizable::{ResizablePaneManager, ResizablePane};
    ///
    /// let mut manager = ResizablePaneManager::new();
    /// manager.add_pane(ResizablePane::new("main", 50));
    /// assert_eq!(manager.pane_count(), 1);
    /// ```
    pub fn add_pane(&mut self, pane: ResizablePane) {
        self.panes.push(pane);
        if self.panes.len() == 1 {
            self.focused = Some(0);
        }
    }

    /// Get pane by ID
    pub fn get_pane(&self, id: &str) -> Option<&ResizablePane> {
        self.panes.iter().find(|p| p.id == id)
    }

    /// Get mutable pane by ID
    pub fn get_pane_mut(&mut self, id: &str) -> Option<&mut ResizablePane> {
        self.panes.iter_mut().find(|p| p.id == id)
    }

    /// Get all panes
    pub fn panes(&self) -> &[ResizablePane] {
        &self.panes
    }

    /// Get pane count
    pub fn pane_count(&self) -> usize {
        self.panes.len()
    }

    /// Get focused pane
    pub fn focused_pane(&self) -> Option<&ResizablePane> {
        self.focused.and_then(|idx| self.panes.get(idx))
    }

    /// Get mutable focused pane
    pub fn focused_pane_mut(&mut self) -> Option<&mut ResizablePane> {
        self.focused.and_then(|idx| self.panes.get_mut(idx))
    }

    /// Set focused pane by ID
    pub fn set_focus(&mut self, id: &str) -> bool {
        if let Some(idx) = self.panes.iter().position(|p| p.id == id) {
            self.focused = Some(idx);
            true
        } else {
            false
        }
    }

    /// Focus next pane
    pub fn focus_next(&mut self) {
        if self.panes.is_empty() {
            return;
        }

        self.focused = Some(match self.focused {
            Some(idx) if idx + 1 < self.panes.len() => idx + 1,
            _ => 0,
        });
    }

    /// Focus previous pane
    pub fn focus_previous(&mut self) {
        if self.panes.is_empty() {
            return;
        }

        self.focused = Some(match self.focused {
            Some(0) | None => self.panes.len() - 1,
            Some(idx) => idx - 1,
        });
    }

    /// Resize focused pane
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::resizable::{ResizablePaneManager, ResizablePane, ResizeDirection};
    ///
    /// let mut manager = ResizablePaneManager::new();
    /// manager.add_pane(ResizablePane::new("main", 50));
    ///
    /// manager.resize_focused(ResizeDirection::Grow, 10);
    /// assert_eq!(manager.focused_pane().map(|p| p.size()), Some(60));
    /// ```
    pub fn resize_focused(&mut self, direction: ResizeDirection, amount: u16) -> bool {
        if let Some(pane) = self.focused_pane_mut() {
            pane.resize(direction, amount);
            true
        } else {
            false
        }
    }

    /// Resize pane by ID
    pub fn resize_pane(&mut self, id: &str, direction: ResizeDirection, amount: u16) -> bool {
        if let Some(pane) = self.get_pane_mut(id) {
            pane.resize(direction, amount);
            true
        } else {
            false
        }
    }

    /// Reset all panes to equal size
    pub fn reset_sizes(&mut self) {
        if self.panes.is_empty() {
            return;
        }

        let equal_size = 100 / self.panes.len() as u16;
        for pane in &mut self.panes {
            pane.set_size(equal_size);
        }
    }
}

impl Default for ResizablePaneManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resizable_pane_creation() {
        let pane = ResizablePane::new("main", 50);
        assert_eq!(pane.id(), "main");
        assert_eq!(pane.size(), 50);
        assert!(pane.is_resizable());
    }

    #[test]
    fn test_pane_resize() {
        let mut pane = ResizablePane::new("main", 50);

        pane.resize(ResizeDirection::Grow, 10);
        assert_eq!(pane.size(), 60);

        pane.resize(ResizeDirection::Shrink, 20);
        assert_eq!(pane.size(), 40);
    }

    #[test]
    fn test_pane_size_limits() {
        let mut pane = ResizablePane::new("main", 50).min_size(20).max_size(80);

        pane.set_size(10); // Below min
        assert_eq!(pane.size(), 20);

        pane.set_size(100); // Above max
        assert_eq!(pane.size(), 80);
    }

    #[test]
    fn test_non_resizable_pane() {
        let mut pane = ResizablePane::new("main", 50).resizable(false);

        pane.resize(ResizeDirection::Grow, 10);
        assert_eq!(pane.size(), 50); // Should not change
    }

    #[test]
    fn test_pane_at_limits() {
        let mut pane = ResizablePane::new("main", 50).min_size(20).max_size(80);

        pane.set_size(20);
        assert!(pane.is_at_min_size());

        pane.set_size(80);
        assert!(pane.is_at_max_size());
    }

    #[test]
    fn test_manager_creation() {
        let manager = ResizablePaneManager::new();
        assert_eq!(manager.pane_count(), 0);
    }

    #[test]
    fn test_manager_add_pane() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("main", 50));

        assert_eq!(manager.pane_count(), 1);
        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("main"));
    }

    #[test]
    fn test_manager_get_pane() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("main", 50));

        assert!(manager.get_pane("main").is_some());
        assert!(manager.get_pane("nonexistent").is_none());
    }

    #[test]
    fn test_manager_focus_navigation() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("pane1", 33));
        manager.add_pane(ResizablePane::new("pane2", 33));
        manager.add_pane(ResizablePane::new("pane3", 34));

        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("pane1"));

        manager.focus_next();
        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("pane2"));

        manager.focus_next();
        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("pane3"));

        manager.focus_next(); // Should wrap
        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("pane1"));

        manager.focus_previous();
        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("pane3"));
    }

    #[test]
    fn test_manager_resize_focused() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("main", 50));

        assert!(manager.resize_focused(ResizeDirection::Grow, 10));
        assert_eq!(manager.focused_pane().map(|p| p.size()), Some(60));
    }

    #[test]
    fn test_manager_resize_by_id() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("pane1", 50));
        manager.add_pane(ResizablePane::new("pane2", 50));

        assert!(manager.resize_pane("pane2", ResizeDirection::Shrink, 10));
        assert_eq!(manager.get_pane("pane2").map(|p| p.size()), Some(40));
    }

    #[test]
    fn test_manager_reset_sizes() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("pane1", 60));
        manager.add_pane(ResizablePane::new("pane2", 40));

        manager.reset_sizes();

        assert_eq!(manager.get_pane("pane1").map(|p| p.size()), Some(50));
        assert_eq!(manager.get_pane("pane2").map(|p| p.size()), Some(50));
    }

    #[test]
    fn test_manager_set_focus() {
        let mut manager = ResizablePaneManager::new();
        manager.add_pane(ResizablePane::new("pane1", 50));
        manager.add_pane(ResizablePane::new("pane2", 50));

        assert!(manager.set_focus("pane2"));
        assert_eq!(manager.focused_pane().map(|p| p.id()), Some("pane2"));

        assert!(!manager.set_focus("nonexistent"));
    }
}
