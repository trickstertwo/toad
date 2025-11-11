//! Startup tips system
//!
//! Displays helpful tips on startup to guide users and introduce features.
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::StartupTips;
//!
//! let tips = StartupTips::new();
//! let tip = tips.get_random_tip();
//! assert!(tip.is_some());
//! ```

use crate::ui::atoms::{block::Block as AtomBlock, text::Text as AtomText};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Borders, Clear, Paragraph, Widget, Wrap},
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Startup tips widget
///
/// Shows helpful tips on startup with random selection and dismiss functionality.
pub struct StartupTips {
    /// Available tips
    tips: Vec<Tip>,
    /// Current tip index
    current: usize,
    /// Whether tips are visible
    visible: bool,
    /// User can disable startup tips
    show_on_startup: bool,
}

/// A single tip
#[derive(Debug, Clone)]
pub struct Tip {
    /// Tip title
    pub title: String,
    /// Tip description
    pub description: String,
    /// Optional keybinding reference
    pub keybinding: Option<String>,
}

impl Tip {
    /// Create a new tip
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            keybinding: None,
        }
    }

    /// Add keybinding reference
    pub fn with_keybinding(mut self, keybinding: impl Into<String>) -> Self {
        self.keybinding = Some(keybinding.into());
        self
    }
}

impl StartupTips {
    /// Create a new startup tips widget
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::StartupTips;
    ///
    /// let tips = StartupTips::new();
    /// assert!(!tips.is_visible());
    /// ```
    pub fn new() -> Self {
        Self {
            tips: Self::default_tips(),
            current: 0,
            visible: false,
            show_on_startup: true,
        }
    }

    /// Get default tips
    fn default_tips() -> Vec<Tip> {
        vec![
            Tip::new(
                "Command Palette",
                "Press Ctrl+P to access all commands with fuzzy search. It's the fastest way to find what you need!",
            ).with_keybinding("Ctrl+P"),

            Tip::new(
                "Vim Motions",
                "Use h/j/k/l for navigation. Once you get used to it, you'll never want to go back!",
            ).with_keybinding("hjkl"),

            Tip::new(
                "Search Everything",
                "Press / to search within files. Use Ctrl+F to search across your entire project.",
            ).with_keybinding("/ or Ctrl+F"),

            Tip::new(
                "Quick Help",
                "Lost? Press ? anytime to see context-specific help and keybindings.",
            ).with_keybinding("?"),

            Tip::new(
                "Split Panes",
                "Work on multiple files at once! Use Ctrl+\\ to split your workspace.",
            ).with_keybinding("Ctrl+\\"),

            Tip::new(
                "Git Integration",
                "Stage, commit, and push without leaving TOAD. Check out the Git panel!",
            ),

            Tip::new(
                "Undo/Redo",
                "Made a mistake? Press u to undo and Ctrl+R to redo. Your work is safe!",
            ).with_keybinding("u / Ctrl+R"),

            Tip::new(
                "Tabs",
                "Open multiple workspaces with tabs. Use Ctrl+T for a new tab, Alt+1-9 to switch.",
            ).with_keybinding("Ctrl+T"),

            Tip::new(
                "Cheat Sheet",
                "Forgot a keybinding? There's a built-in cheat sheet with all shortcuts!",
            ),

            Tip::new(
                "Theme Customization",
                "TOAD supports multiple themes including Catppuccin, Nord, and custom themes!",
            ),
        ]
    }

    /// Show tips
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide tips
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get random tip (uses time-based pseudo-randomness)
    pub fn get_random_tip(&mut self) -> Option<&Tip> {
        if self.tips.is_empty() {
            return None;
        }
        let idx = self.pseudo_random_index();
        self.tips.get(idx)
    }

    /// Set random tip as current (uses time-based pseudo-randomness)
    pub fn randomize(&mut self) {
        if !self.tips.is_empty() {
            self.current = self.pseudo_random_index();
        }
    }

    /// Generate a pseudo-random index based on current time
    fn pseudo_random_index(&self) -> usize {
        if self.tips.is_empty() {
            return 0;
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        (now % self.tips.len() as u128) as usize
    }

    /// Get next tip
    pub fn next_tip(&mut self) {
        self.current = (self.current + 1) % self.tips.len();
    }

    /// Get previous tip
    pub fn previous_tip(&mut self) {
        self.current = if self.current == 0 {
            self.tips.len() - 1
        } else {
            self.current - 1
        };
    }

    /// Get current tip
    pub fn current_tip(&self) -> Option<&Tip> {
        self.tips.get(self.current)
    }

    /// Set whether to show on startup
    pub fn set_show_on_startup(&mut self, show: bool) {
        self.show_on_startup = show;
    }

    /// Check if should show on startup
    pub fn should_show_on_startup(&self) -> bool {
        self.show_on_startup
    }

    /// Get tip count
    pub fn tip_count(&self) -> usize {
        self.tips.len()
    }
}

impl Default for StartupTips {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &StartupTips {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.visible {
            return;
        }

        // Calculate tip overlay area
        let width = (area.width as f32 * 0.5).min(60.0) as u16;
        let height = 12;

        let tip_area = Rect {
            x: (area.width.saturating_sub(width)) / 2 + area.x,
            y: (area.height.saturating_sub(height)) / 2 + area.y,
            width,
            height,
        };

        // Clear background
        Clear.render(tip_area, buf);

        if let Some(tip) = self.current_tip() {
            // Render tip content
            let mut lines = vec![
                Line::from(vec![
                    AtomText::new("💡 ")
                        .style(Style::default().fg(Color::Yellow))
                        .to_span(),
                    AtomText::new(&tip.title)
                        .style(
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )
                        .to_span(),
                ]),
                Line::from(""),
                Line::from(
                    AtomText::new(&tip.description)
                        .style(Style::default().fg(Color::White))
                        .to_span(),
                ),
            ];

            if let Some(keybinding) = &tip.keybinding {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    AtomText::new("Keybinding: ")
                        .style(Style::default().fg(Color::Gray))
                        .to_span(),
                    AtomText::new(keybinding)
                        .style(Style::default().fg(Color::Green))
                        .to_span(),
                ]));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(vec![AtomText::new(format!(
                "Tip {} of {}",
                self.current + 1,
                self.tips.len()
            ))
            .style(Style::default().fg(Color::DarkGray))
            .to_span()]));

            let para = Paragraph::new(lines)
                .block(
                    AtomBlock::new()
                        .borders(Borders::ALL)
                        .title("💡 Tip of the Day")
                        .border_style(Style::default().fg(Color::Yellow))
                        .to_ratatui(),
                )
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Left);

            para.render(tip_area, buf);

            // Footer
            let footer_area = Rect {
                x: tip_area.x,
                y: tip_area.y + tip_area.height - 1,
                width: tip_area.width,
                height: 1,
            };

            let footer = Paragraph::new("← → Navigate | Enter: Dismiss | D: Don't show again")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);

            Clear.render(footer_area, buf);
            footer.render(footer_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_tips_new() {
        let tips = StartupTips::new();
        assert!(!tips.is_visible());
        assert!(tips.should_show_on_startup());
        assert!(tips.tip_count() > 0);
    }

    #[test]
    fn test_startup_tips_visibility() {
        let mut tips = StartupTips::new();

        tips.show();
        assert!(tips.is_visible());

        tips.hide();
        assert!(!tips.is_visible());

        tips.toggle();
        assert!(tips.is_visible());
    }

    #[test]
    fn test_startup_tips_navigation() {
        let mut tips = StartupTips::new();
        let total = tips.tip_count();

        assert_eq!(tips.current, 0);

        tips.next_tip();
        assert_eq!(tips.current, 1);

        tips.previous_tip();
        assert_eq!(tips.current, 0);

        // Wrap around
        tips.previous_tip();
        assert_eq!(tips.current, total - 1);
    }

    #[test]
    fn test_startup_tips_randomize() {
        let mut tips = StartupTips::new();
        let initial = tips.current;

        tips.randomize();

        // Might be the same by chance, but at least it shouldn't panic
        assert!(tips.current < tips.tip_count());
    }

    #[test]
    fn test_startup_tips_get_random() {
        let mut tips = StartupTips::new();
        let tip = tips.get_random_tip();

        assert!(tip.is_some());
    }

    #[test]
    fn test_tip_creation() {
        let tip = Tip::new("Title", "Description").with_keybinding("Ctrl+K");

        assert_eq!(tip.title, "Title");
        assert_eq!(tip.description, "Description");
        assert_eq!(tip.keybinding.as_deref(), Some("Ctrl+K"));
    }

    #[test]
    fn test_startup_preference() {
        let mut tips = StartupTips::new();

        assert!(tips.should_show_on_startup());

        tips.set_show_on_startup(false);
        assert!(!tips.should_show_on_startup());
    }
}
