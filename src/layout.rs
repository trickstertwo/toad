/// Layout management system for split panes and panels
///
/// Provides a flexible system for creating and managing split panes
/// with support for resizing, focus management, and dynamic layouts.

use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};
use serde::{Deserialize, Serialize};

/// A panel ID for tracking focus and state
pub type PanelId = usize;

/// Direction of a split
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Horizontal split (side by side)
    Horizontal,
    /// Vertical split (top and bottom)
    Vertical,
}

/// A pane in the layout
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pane {
    /// A leaf pane containing content
    Leaf {
        /// Unique ID for this pane
        id: PanelId,
        /// Whether this pane is visible
        visible: bool,
    },
    /// A split containing two child panes
    Split {
        /// Direction of the split
        direction: SplitDirection,
        /// Split ratio (0.0 to 1.0)
        ratio: u16,
        /// First child pane
        first: Box<Pane>,
        /// Second child pane
        second: Box<Pane>,
    },
}

impl Pane {
    /// Create a new leaf pane
    pub fn leaf(id: PanelId) -> Self {
        Self::Leaf {
            id,
            visible: true,
        }
    }

    /// Create a horizontal split
    pub fn hsplit(ratio: u16, first: Pane, second: Pane) -> Self {
        Self::Split {
            direction: SplitDirection::Horizontal,
            ratio,
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Create a vertical split
    pub fn vsplit(ratio: u16, first: Pane, second: Pane) -> Self {
        Self::Split {
            direction: SplitDirection::Vertical,
            ratio,
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Get all leaf panel IDs
    pub fn leaf_ids(&self) -> Vec<PanelId> {
        let mut ids = Vec::new();
        self.collect_leaf_ids(&mut ids);
        ids
    }

    fn collect_leaf_ids(&self, ids: &mut Vec<PanelId>) {
        match self {
            Pane::Leaf { id, visible } if *visible => {
                ids.push(*id);
            }
            Pane::Split { first, second, .. } => {
                first.collect_leaf_ids(ids);
                second.collect_leaf_ids(ids);
            }
            _ => {}
        }
    }

    /// Toggle visibility of a pane
    pub fn toggle_visibility(&mut self, target_id: PanelId) -> bool {
        match self {
            Pane::Leaf { id, visible } if *id == target_id => {
                *visible = !*visible;
                true
            }
            Pane::Split { first, second, .. } => {
                first.toggle_visibility(target_id) || second.toggle_visibility(target_id)
            }
            _ => false,
        }
    }

    /// Set visibility of a pane
    pub fn set_visibility(&mut self, target_id: PanelId, is_visible: bool) -> bool {
        match self {
            Pane::Leaf { id, visible } if *id == target_id => {
                *visible = is_visible;
                true
            }
            Pane::Split { first, second, .. } => {
                first.set_visibility(target_id, is_visible)
                    || second.set_visibility(target_id, is_visible)
            }
            _ => false,
        }
    }

    /// Resize a split
    pub fn resize_split(&mut self, delta: i16) -> bool {
        match self {
            Pane::Split { ratio, .. } => {
                let new_ratio = (*ratio as i16 + delta).clamp(10, 90) as u16;
                *ratio = new_ratio;
                true
            }
            _ => false,
        }
    }

    /// Calculate layout areas for rendering
    pub fn calculate_areas(&self, area: Rect) -> Vec<(PanelId, Rect)> {
        let mut areas = Vec::new();
        self.collect_areas(area, &mut areas);
        areas
    }

    fn collect_areas(&self, area: Rect, areas: &mut Vec<(PanelId, Rect)>) {
        match self {
            Pane::Leaf { id, visible } if *visible => {
                areas.push((*id, area));
            }
            Pane::Split {
                direction,
                ratio,
                first,
                second,
            } => {
                let (dir, constraints) = match direction {
                    SplitDirection::Horizontal => (
                        Direction::Horizontal,
                        vec![
                            Constraint::Percentage(*ratio),
                            Constraint::Percentage(100 - *ratio),
                        ],
                    ),
                    SplitDirection::Vertical => (
                        Direction::Vertical,
                        vec![
                            Constraint::Percentage(*ratio),
                            Constraint::Percentage(100 - *ratio),
                        ],
                    ),
                };

                let chunks = RatatuiLayout::default()
                    .direction(dir)
                    .constraints(constraints)
                    .split(area);

                first.collect_areas(chunks[0], areas);
                second.collect_areas(chunks[1], areas);
            }
            _ => {}
        }
    }
}

/// Layout manager for panels
#[derive(Debug)]
pub struct LayoutManager {
    /// Root pane
    root: Pane,
    /// Currently focused panel
    focused: PanelId,
    /// Next panel ID to assign
    #[allow(dead_code)]
    next_id: PanelId,
}

impl LayoutManager {
    /// Create a new layout manager with a single panel
    pub fn new() -> Self {
        Self {
            root: Pane::leaf(0),
            focused: 0,
            next_id: 1,
        }
    }

    /// Create a layout with horizontal split (two panels side by side)
    pub fn with_hsplit() -> Self {
        let left = Pane::leaf(0);
        let right = Pane::leaf(1);
        Self {
            root: Pane::hsplit(50, left, right),
            focused: 0,
            next_id: 2,
        }
    }

    /// Create a layout with vertical split (two panels top and bottom)
    pub fn with_vsplit() -> Self {
        let top = Pane::leaf(0);
        let bottom = Pane::leaf(1);
        Self {
            root: Pane::vsplit(50, top, bottom),
            focused: 0,
            next_id: 2,
        }
    }

    /// Get the root pane
    pub fn root(&self) -> &Pane {
        &self.root
    }

    /// Get mutable root pane
    pub fn root_mut(&mut self) -> &mut Pane {
        &mut self.root
    }

    /// Get focused panel ID
    pub fn focused(&self) -> PanelId {
        self.focused
    }

    /// Set focused panel
    pub fn set_focused(&mut self, id: PanelId) {
        let leaf_ids = self.root.leaf_ids();
        if leaf_ids.contains(&id) {
            self.focused = id;
        }
    }

    /// Focus next panel
    pub fn focus_next(&mut self) {
        let leaf_ids = self.root.leaf_ids();
        if leaf_ids.is_empty() {
            return;
        }

        if let Some(current_idx) = leaf_ids.iter().position(|&id| id == self.focused) {
            let next_idx = (current_idx + 1) % leaf_ids.len();
            self.focused = leaf_ids[next_idx];
        } else {
            self.focused = leaf_ids[0];
        }
    }

    /// Focus previous panel
    pub fn focus_previous(&mut self) {
        let leaf_ids = self.root.leaf_ids();
        if leaf_ids.is_empty() {
            return;
        }

        if let Some(current_idx) = leaf_ids.iter().position(|&id| id == self.focused) {
            let prev_idx = if current_idx == 0 {
                leaf_ids.len() - 1
            } else {
                current_idx - 1
            };
            self.focused = leaf_ids[prev_idx];
        } else {
            self.focused = leaf_ids[0];
        }
    }

    /// Toggle visibility of a panel
    pub fn toggle_panel(&mut self, id: PanelId) {
        self.root.toggle_visibility(id);
    }

    /// Show a panel
    pub fn show_panel(&mut self, id: PanelId) {
        self.root.set_visibility(id, true);
    }

    /// Hide a panel
    pub fn hide_panel(&mut self, id: PanelId) {
        self.root.set_visibility(id, false);
    }

    /// Calculate areas for all visible panels
    pub fn calculate_areas(&self, area: Rect) -> Vec<(PanelId, Rect)> {
        self.root.calculate_areas(area)
    }

    /// Get panel area for a specific ID
    pub fn get_panel_area(&self, area: Rect, id: PanelId) -> Option<Rect> {
        self.calculate_areas(area)
            .into_iter()
            .find(|(panel_id, _)| *panel_id == id)
            .map(|(_, rect)| rect)
    }

    /// Check if a panel is focused
    pub fn is_focused(&self, id: PanelId) -> bool {
        self.focused == id
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pane_leaf_ids() {
        let pane = Pane::hsplit(50, Pane::leaf(0), Pane::leaf(1));
        let ids = pane.leaf_ids();
        assert_eq!(ids, vec![0, 1]);
    }

    #[test]
    fn test_pane_visibility() {
        let mut pane = Pane::leaf(0);
        assert_eq!(pane.leaf_ids(), vec![0]);

        pane.toggle_visibility(0);
        assert_eq!(pane.leaf_ids(), vec![]);

        pane.toggle_visibility(0);
        assert_eq!(pane.leaf_ids(), vec![0]);
    }

    #[test]
    fn test_layout_manager_focus() {
        let mut layout = LayoutManager::with_hsplit();
        assert_eq!(layout.focused(), 0);

        layout.focus_next();
        assert_eq!(layout.focused(), 1);

        layout.focus_next();
        assert_eq!(layout.focused(), 0);

        layout.focus_previous();
        assert_eq!(layout.focused(), 1);
    }

    #[test]
    fn test_layout_manager_visibility() {
        let mut layout = LayoutManager::with_hsplit();
        let ids = layout.root().leaf_ids();
        assert_eq!(ids.len(), 2);

        layout.hide_panel(0);
        let ids = layout.root().leaf_ids();
        assert_eq!(ids, vec![1]);

        layout.show_panel(0);
        let ids = layout.root().leaf_ids();
        assert_eq!(ids, vec![0, 1]);
    }
}
