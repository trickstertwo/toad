/// Mode indicator widget for Vim-style modes
///
/// Visual display of current editing mode
///
/// # Examples
///
/// ```
/// use toad::widgets::ModeIndicator;
/// use toad::widgets::EditorMode;
///
/// let indicator = ModeIndicator::new(EditorMode::Normal);
/// assert_eq!(indicator.mode(), EditorMode::Normal);
/// ```
use crate::ui::theme::ToadTheme;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Editor mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorMode {
    /// Normal mode - navigation and commands
    Normal,
    /// Insert mode - text editing
    Insert,
    /// Visual mode - selection (character-wise)
    Visual,
    /// Visual Line mode - line-wise selection
    VisualLine,
    /// Visual Block mode - block selection
    VisualBlock,
    /// Command mode - ex-style commands
    Command,
    /// Replace mode - overwrite text
    Replace,
}

impl EditorMode {
    /// Get mode name
    pub fn name(&self) -> &'static str {
        match self {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
            EditorMode::Visual => "VISUAL",
            EditorMode::VisualLine => "VISUAL LINE",
            EditorMode::VisualBlock => "VISUAL BLOCK",
            EditorMode::Command => "COMMAND",
            EditorMode::Replace => "REPLACE",
        }
    }

    /// Get short name (for compact display)
    pub fn short_name(&self) -> &'static str {
        match self {
            EditorMode::Normal => "N",
            EditorMode::Insert => "I",
            EditorMode::Visual => "V",
            EditorMode::VisualLine => "VL",
            EditorMode::VisualBlock => "VB",
            EditorMode::Command => "C",
            EditorMode::Replace => "R",
        }
    }

    /// Get the color for this mode
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            EditorMode::Normal => ToadTheme::TOAD_GREEN,
            EditorMode::Insert => ToadTheme::BLUE,
            EditorMode::Visual => ToadTheme::YELLOW,
            EditorMode::VisualLine => ToadTheme::YELLOW,
            EditorMode::VisualBlock => ToadTheme::YELLOW,
            EditorMode::Command => ToadTheme::TOAD_GREEN_BRIGHT,
            EditorMode::Replace => ToadTheme::RED,
        }
    }

    /// All available modes
    pub fn all() -> &'static [EditorMode] {
        &[
            EditorMode::Normal,
            EditorMode::Insert,
            EditorMode::Visual,
            EditorMode::VisualLine,
            EditorMode::VisualBlock,
            EditorMode::Command,
            EditorMode::Replace,
        ]
    }
}

impl Default for EditorMode {
    fn default() -> Self {
        EditorMode::Normal
    }
}

impl fmt::Display for EditorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Mode indicator display style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndicatorStyle {
    /// Full mode name
    Full,
    /// Short mode name
    Short,
    /// Colored block with mode name
    Block,
}

impl Default for IndicatorStyle {
    fn default() -> Self {
        IndicatorStyle::Full
    }
}

/// Mode indicator widget
pub struct ModeIndicator {
    /// Current mode
    mode: EditorMode,
    /// Display style
    style: IndicatorStyle,
    /// Whether to show borders
    show_border: bool,
}

impl ModeIndicator {
    /// Create a new mode indicator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ModeIndicator, EditorMode};
    ///
    /// let indicator = ModeIndicator::new(EditorMode::Normal);
    /// assert_eq!(indicator.mode(), EditorMode::Normal);
    /// ```
    pub fn new(mode: EditorMode) -> Self {
        Self {
            mode,
            style: IndicatorStyle::Full,
            show_border: false,
        }
    }

    /// Set the mode
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    /// Get the current mode
    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    /// Set the display style
    pub fn with_style(mut self, style: IndicatorStyle) -> Self {
        self.style = style;
        self
    }

    /// Set whether to show borders
    pub fn with_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Get the mode text based on style
    fn mode_text(&self) -> &str {
        match self.style {
            IndicatorStyle::Full | IndicatorStyle::Block => self.mode.name(),
            IndicatorStyle::Short => self.mode.short_name(),
        }
    }

    /// Render the mode indicator
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mode_text = self.mode_text();
        let color = self.mode.color();

        let span = match self.style {
            IndicatorStyle::Full | IndicatorStyle::Short => Span::styled(
                format!(" {} ", mode_text),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            IndicatorStyle::Block => Span::styled(
                format!(" {} ", mode_text),
                Style::default()
                    .fg(ToadTheme::BLACK)
                    .bg(color)
                    .add_modifier(Modifier::BOLD),
            ),
        };

        let line = Line::from(vec![span]);
        let paragraph = if self.show_border {
            Paragraph::new(line).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color)),
            )
        } else {
            Paragraph::new(line)
        };

        frame.render_widget(paragraph, area);
    }

    /// Get a text representation (for testing/display)
    pub fn to_string(&self) -> String {
        self.mode_text().to_string()
    }
}

impl Default for ModeIndicator {
    fn default() -> Self {
        Self::new(EditorMode::Normal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_mode_name() {
        assert_eq!(EditorMode::Normal.name(), "NORMAL");
        assert_eq!(EditorMode::Insert.name(), "INSERT");
        assert_eq!(EditorMode::Visual.name(), "VISUAL");
        assert_eq!(EditorMode::VisualLine.name(), "VISUAL LINE");
        assert_eq!(EditorMode::VisualBlock.name(), "VISUAL BLOCK");
        assert_eq!(EditorMode::Command.name(), "COMMAND");
        assert_eq!(EditorMode::Replace.name(), "REPLACE");
    }

    #[test]
    fn test_editor_mode_short_name() {
        assert_eq!(EditorMode::Normal.short_name(), "N");
        assert_eq!(EditorMode::Insert.short_name(), "I");
        assert_eq!(EditorMode::Visual.short_name(), "V");
        assert_eq!(EditorMode::VisualLine.short_name(), "VL");
        assert_eq!(EditorMode::VisualBlock.short_name(), "VB");
        assert_eq!(EditorMode::Command.short_name(), "C");
        assert_eq!(EditorMode::Replace.short_name(), "R");
    }

    #[test]
    fn test_editor_mode_color() {
        assert_eq!(EditorMode::Normal.color(), ToadTheme::TOAD_GREEN);
        assert_eq!(EditorMode::Insert.color(), ToadTheme::BLUE);
        assert_eq!(EditorMode::Visual.color(), ToadTheme::YELLOW);
        assert_eq!(EditorMode::Command.color(), ToadTheme::TOAD_GREEN_BRIGHT);
        assert_eq!(EditorMode::Replace.color(), ToadTheme::RED);
    }

    #[test]
    fn test_editor_mode_all() {
        let modes = EditorMode::all();
        assert_eq!(modes.len(), 7);
    }

    #[test]
    fn test_editor_mode_default() {
        let mode = EditorMode::default();
        assert_eq!(mode, EditorMode::Normal);
    }

    #[test]
    fn test_editor_mode_display() {
        assert_eq!(format!("{}", EditorMode::Normal), "NORMAL");
        assert_eq!(format!("{}", EditorMode::Insert), "INSERT");
    }

    #[test]
    fn test_indicator_style_default() {
        let style = IndicatorStyle::default();
        assert_eq!(style, IndicatorStyle::Full);
    }

    #[test]
    fn test_mode_indicator_creation() {
        let indicator = ModeIndicator::new(EditorMode::Normal);
        assert_eq!(indicator.mode(), EditorMode::Normal);
    }

    #[test]
    fn test_mode_indicator_set_mode() {
        let mut indicator = ModeIndicator::new(EditorMode::Normal);
        assert_eq!(indicator.mode(), EditorMode::Normal);

        indicator.set_mode(EditorMode::Insert);
        assert_eq!(indicator.mode(), EditorMode::Insert);
    }

    #[test]
    fn test_mode_indicator_with_style() {
        let indicator = ModeIndicator::new(EditorMode::Normal).with_style(IndicatorStyle::Short);
        assert_eq!(indicator.to_string(), "N");
    }

    #[test]
    fn test_mode_indicator_with_border() {
        let indicator = ModeIndicator::new(EditorMode::Normal).with_border(true);
        assert!(indicator.show_border);
    }

    #[test]
    fn test_mode_text_full() {
        let indicator = ModeIndicator::new(EditorMode::Normal).with_style(IndicatorStyle::Full);
        assert_eq!(indicator.mode_text(), "NORMAL");
    }

    #[test]
    fn test_mode_text_short() {
        let indicator = ModeIndicator::new(EditorMode::Insert).with_style(IndicatorStyle::Short);
        assert_eq!(indicator.mode_text(), "I");
    }

    #[test]
    fn test_mode_text_block() {
        let indicator = ModeIndicator::new(EditorMode::Visual).with_style(IndicatorStyle::Block);
        assert_eq!(indicator.mode_text(), "VISUAL");
    }

    #[test]
    fn test_mode_indicator_default() {
        let indicator = ModeIndicator::default();
        assert_eq!(indicator.mode(), EditorMode::Normal);
    }

    #[test]
    fn test_to_string() {
        let indicator = ModeIndicator::new(EditorMode::Normal);
        assert_eq!(indicator.to_string(), "NORMAL");

        let indicator = ModeIndicator::new(EditorMode::Insert).with_style(IndicatorStyle::Short);
        assert_eq!(indicator.to_string(), "I");
    }

    #[test]
    fn test_all_modes() {
        for mode in EditorMode::all() {
            let indicator = ModeIndicator::new(*mode);
            assert_eq!(indicator.mode(), *mode);
            assert!(!indicator.to_string().is_empty());
        }
    }

    #[test]
    fn test_visual_modes() {
        let visual = EditorMode::Visual;
        let visual_line = EditorMode::VisualLine;
        let visual_block = EditorMode::VisualBlock;

        // All visual modes should have yellow color
        assert_eq!(visual.color(), ToadTheme::YELLOW);
        assert_eq!(visual_line.color(), ToadTheme::YELLOW);
        assert_eq!(visual_block.color(), ToadTheme::YELLOW);

        // Different names
        assert_ne!(visual.name(), visual_line.name());
        assert_ne!(visual.name(), visual_block.name());
    }
}
