//! Vim-style modal editing system
//!
//! Provides multiple editing modes (Normal, Insert, Visual, Command) with
//! mode transitions and state management.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{VimMode, EditMode};
//!
//! let mut mode = VimMode::new();
//! assert_eq!(mode.current_mode(), EditMode::Normal);
//!
//! mode.enter_insert_mode();
//! assert_eq!(mode.current_mode(), EditMode::Insert);
//!
//! mode.exit_to_normal();
//! assert_eq!(mode.current_mode(), EditMode::Normal);
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

/// Editing mode
///
/// # Examples
///
/// ```
/// use toad::widgets::EditMode;
///
/// let mode = EditMode::Normal;
/// assert_eq!(mode.name(), "NORMAL");
/// assert_eq!(mode.color(), ratatui::style::Color::Cyan);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditMode {
    /// Normal mode - navigation and commands
    #[default]
    Normal,
    /// Insert mode - text editing
    Insert,
    /// Visual mode - text selection
    Visual,
    /// Visual Line mode - line selection
    VisualLine,
    /// Visual Block mode - block selection
    VisualBlock,
    /// Command mode - ex-style commands
    Command,
}

impl EditMode {
    /// Get mode name
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EditMode;
    ///
    /// assert_eq!(EditMode::Normal.name(), "NORMAL");
    /// assert_eq!(EditMode::Insert.name(), "INSERT");
    /// assert_eq!(EditMode::Visual.name(), "VISUAL");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::Normal => "NORMAL",
            Self::Insert => "INSERT",
            Self::Visual => "VISUAL",
            Self::VisualLine => "VISUAL LINE",
            Self::VisualBlock => "VISUAL BLOCK",
            Self::Command => "COMMAND",
        }
    }

    /// Get mode color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EditMode;
    /// use ratatui::style::Color;
    ///
    /// assert_eq!(EditMode::Normal.color(), Color::Cyan);
    /// assert_eq!(EditMode::Insert.color(), Color::Green);
    /// ```
    pub fn color(&self) -> Color {
        match self {
            Self::Normal => Color::Cyan,
            Self::Insert => Color::Green,
            Self::Visual | Self::VisualLine | Self::VisualBlock => Color::Yellow,
            Self::Command => Color::Magenta,
        }
    }

    /// Get mode key hint
    pub fn key_hint(&self) -> &str {
        match self {
            Self::Normal => "i=insert v=visual :=command",
            Self::Insert => "ESC=normal",
            Self::Visual => "ESC=normal y=yank d=delete",
            Self::VisualLine => "ESC=normal y=yank d=delete",
            Self::VisualBlock => "ESC=normal y=yank d=delete",
            Self::Command => "Enter=execute ESC=cancel",
        }
    }

    /// Check if mode allows text input
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EditMode;
    ///
    /// assert!(!EditMode::Normal.is_input_mode());
    /// assert!(EditMode::Insert.is_input_mode());
    /// assert!(EditMode::Command.is_input_mode());
    /// ```
    pub fn is_input_mode(&self) -> bool {
        matches!(self, Self::Insert | Self::Command)
    }

    /// Check if mode is a visual mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::EditMode;
    ///
    /// assert!(!EditMode::Normal.is_visual_mode());
    /// assert!(EditMode::Visual.is_visual_mode());
    /// assert!(EditMode::VisualLine.is_visual_mode());
    /// ```
    pub fn is_visual_mode(&self) -> bool {
        matches!(self, Self::Visual | Self::VisualLine | Self::VisualBlock)
    }
}

/// Selection range for visual modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Start position (row, col)
    pub start: (usize, usize),
    /// End position (row, col)
    pub end: (usize, usize),
}

impl Selection {
    /// Create a new selection
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }

    /// Get normalized range (min to max)
    pub fn range(&self) -> ((usize, usize), (usize, usize)) {
        let (start_row, start_col) = self.start;
        let (end_row, end_col) = self.end;

        if start_row < end_row || (start_row == end_row && start_col <= end_col) {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }
}

/// Vim mode manager
///
/// Manages mode transitions and state for vim-style modal editing.
///
/// # Examples
///
/// ```
/// use toad::widgets::{VimMode, EditMode};
///
/// let mut mode = VimMode::new();
///
/// // Enter insert mode
/// mode.enter_insert_mode();
/// assert_eq!(mode.current_mode(), EditMode::Insert);
///
/// // Exit to normal
/// mode.exit_to_normal();
/// assert_eq!(mode.current_mode(), EditMode::Normal);
///
/// // Enter visual mode
/// mode.enter_visual_mode();
/// assert!(mode.current_mode().is_visual_mode());
/// ```
#[derive(Debug, Clone)]
pub struct VimMode {
    /// Current mode
    mode: EditMode,
    /// Previous mode (for ESC behavior)
    previous_mode: EditMode,
    /// Visual mode selection
    selection: Option<Selection>,
    /// Command buffer
    command_buffer: String,
}

impl Default for VimMode {
    fn default() -> Self {
        Self::new()
    }
}

impl VimMode {
    /// Create a new vim mode manager
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mode = VimMode::new();
    /// assert_eq!(mode.current_mode(), EditMode::Normal);
    /// ```
    pub fn new() -> Self {
        Self {
            mode: EditMode::Normal,
            previous_mode: EditMode::Normal,
            selection: None,
            command_buffer: String::new(),
        }
    }

    /// Get current mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mode = VimMode::new();
    /// assert_eq!(mode.current_mode(), EditMode::Normal);
    /// ```
    pub fn current_mode(&self) -> EditMode {
        self.mode
    }

    /// Enter insert mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_insert_mode();
    /// assert_eq!(mode.current_mode(), EditMode::Insert);
    /// ```
    pub fn enter_insert_mode(&mut self) {
        self.previous_mode = self.mode;
        self.mode = EditMode::Insert;
        self.clear_selection();
    }

    /// Enter visual mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_visual_mode();
    /// assert_eq!(mode.current_mode(), EditMode::Visual);
    /// ```
    pub fn enter_visual_mode(&mut self) {
        self.previous_mode = self.mode;
        self.mode = EditMode::Visual;
    }

    /// Enter visual line mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_visual_line_mode();
    /// assert_eq!(mode.current_mode(), EditMode::VisualLine);
    /// ```
    pub fn enter_visual_line_mode(&mut self) {
        self.previous_mode = self.mode;
        self.mode = EditMode::VisualLine;
    }

    /// Enter visual block mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_visual_block_mode();
    /// assert_eq!(mode.current_mode(), EditMode::VisualBlock);
    /// ```
    pub fn enter_visual_block_mode(&mut self) {
        self.previous_mode = self.mode;
        self.mode = EditMode::VisualBlock;
    }

    /// Enter command mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_command_mode();
    /// assert_eq!(mode.current_mode(), EditMode::Command);
    /// ```
    pub fn enter_command_mode(&mut self) {
        self.previous_mode = self.mode;
        self.mode = EditMode::Command;
        self.command_buffer.clear();
        self.clear_selection();
    }

    /// Exit to normal mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, EditMode};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_insert_mode();
    /// mode.exit_to_normal();
    /// assert_eq!(mode.current_mode(), EditMode::Normal);
    /// ```
    pub fn exit_to_normal(&mut self) {
        self.previous_mode = self.mode;
        self.mode = EditMode::Normal;
        self.clear_selection();
        self.command_buffer.clear();
    }

    /// Set selection range
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{VimMode, Selection};
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_visual_mode();
    /// mode.set_selection(Selection::new((0, 0), (0, 5)));
    /// assert!(mode.has_selection());
    /// ```
    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = Some(selection);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    /// Check if there's an active selection
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::VimMode;
    ///
    /// let mut mode = VimMode::new();
    /// assert!(!mode.has_selection());
    ///
    /// mode.enter_visual_mode();
    /// assert!(!mode.has_selection()); // No range set yet
    /// ```
    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    /// Get current selection
    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    /// Add character to command buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::VimMode;
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_command_mode();
    /// mode.add_command_char('w');
    /// mode.add_command_char('q');
    /// assert_eq!(mode.command_buffer(), "wq");
    /// ```
    pub fn add_command_char(&mut self, c: char) {
        if self.mode == EditMode::Command {
            self.command_buffer.push(c);
        }
    }

    /// Remove last character from command buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::VimMode;
    ///
    /// let mut mode = VimMode::new();
    /// mode.enter_command_mode();
    /// mode.add_command_char('w');
    /// mode.add_command_char('q');
    /// mode.remove_command_char();
    /// assert_eq!(mode.command_buffer(), "w");
    /// ```
    pub fn remove_command_char(&mut self) {
        if self.mode == EditMode::Command {
            self.command_buffer.pop();
        }
    }

    /// Get command buffer
    pub fn command_buffer(&self) -> &str {
        &self.command_buffer
    }

    /// Clear command buffer
    pub fn clear_command_buffer(&mut self) {
        self.command_buffer.clear();
    }
}

/// Mode indicator widget
///
/// Displays the current vim mode with color coding and key hints.
///
/// # Examples
///
/// ```
/// use toad::widgets::{ModeIndicator, EditMode};
///
/// let indicator = ModeIndicator::new(EditMode::Normal);
/// ```
#[derive(Debug, Clone)]
pub struct ModeIndicator {
    /// Current mode
    mode: EditMode,
    /// Show key hints
    show_hints: bool,
    /// Compact mode
    compact: bool,
}

impl ModeIndicator {
    /// Create a new mode indicator
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ModeIndicator, EditMode};
    ///
    /// let indicator = ModeIndicator::new(EditMode::Insert);
    /// ```
    pub fn new(mode: EditMode) -> Self {
        Self {
            mode,
            show_hints: true,
            compact: false,
        }
    }

    /// Show or hide key hints
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ModeIndicator, EditMode};
    ///
    /// let indicator = ModeIndicator::new(EditMode::Normal)
    ///     .with_hints(false);
    /// ```
    pub fn with_hints(mut self, show: bool) -> Self {
        self.show_hints = show;
        self
    }

    /// Enable compact mode
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{ModeIndicator, EditMode};
    ///
    /// let indicator = ModeIndicator::new(EditMode::Normal)
    ///     .with_compact(true);
    /// ```
    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Render as a line
    pub fn render_line(&self) -> Line<'static> {
        let mut spans = vec![];

        // Mode name
        spans.push(Span::styled(
            format!(" {} ", self.mode.name()),
            Style::default()
                .fg(Color::Black)
                .bg(self.mode.color())
                .add_modifier(Modifier::BOLD),
        ));

        // Key hints
        if self.show_hints && !self.compact {
            spans.push(Span::raw(" ".to_string()));
            spans.push(Span::styled(
                self.mode.key_hint().to_string(),
                Style::default().fg(Color::DarkGray),
            ));
        }

        Line::from(spans)
    }
}

impl Widget for &ModeIndicator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.render_line();
        let block = Block::default().borders(Borders::NONE);
        let inner = block.inner(area);

        block.render(area, buf);
        buf.set_line(inner.x, inner.y, &line, inner.width);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_mode_default() {
        assert_eq!(EditMode::default(), EditMode::Normal);
    }

    #[test]
    fn test_edit_mode_name() {
        assert_eq!(EditMode::Normal.name(), "NORMAL");
        assert_eq!(EditMode::Insert.name(), "INSERT");
        assert_eq!(EditMode::Visual.name(), "VISUAL");
        assert_eq!(EditMode::VisualLine.name(), "VISUAL LINE");
        assert_eq!(EditMode::VisualBlock.name(), "VISUAL BLOCK");
        assert_eq!(EditMode::Command.name(), "COMMAND");
    }

    #[test]
    fn test_edit_mode_color() {
        assert_eq!(EditMode::Normal.color(), Color::Cyan);
        assert_eq!(EditMode::Insert.color(), Color::Green);
        assert_eq!(EditMode::Visual.color(), Color::Yellow);
        assert_eq!(EditMode::Command.color(), Color::Magenta);
    }

    #[test]
    fn test_edit_mode_is_input_mode() {
        assert!(!EditMode::Normal.is_input_mode());
        assert!(EditMode::Insert.is_input_mode());
        assert!(!EditMode::Visual.is_input_mode());
        assert!(EditMode::Command.is_input_mode());
    }

    #[test]
    fn test_edit_mode_is_visual_mode() {
        assert!(!EditMode::Normal.is_visual_mode());
        assert!(!EditMode::Insert.is_visual_mode());
        assert!(EditMode::Visual.is_visual_mode());
        assert!(EditMode::VisualLine.is_visual_mode());
        assert!(EditMode::VisualBlock.is_visual_mode());
    }

    #[test]
    fn test_selection_new() {
        let sel = Selection::new((0, 0), (0, 5));
        assert_eq!(sel.start, (0, 0));
        assert_eq!(sel.end, (0, 5));
    }

    #[test]
    fn test_selection_range() {
        let sel = Selection::new((0, 5), (0, 0));
        let (start, end) = sel.range();
        assert_eq!(start, (0, 0));
        assert_eq!(end, (0, 5));
    }

    #[test]
    fn test_vim_mode_new() {
        let mode = VimMode::new();
        assert_eq!(mode.current_mode(), EditMode::Normal);
        assert!(!mode.has_selection());
    }

    #[test]
    fn test_vim_mode_default() {
        let mode = VimMode::default();
        assert_eq!(mode.current_mode(), EditMode::Normal);
    }

    #[test]
    fn test_vim_mode_enter_insert() {
        let mut mode = VimMode::new();
        mode.enter_insert_mode();
        assert_eq!(mode.current_mode(), EditMode::Insert);
    }

    #[test]
    fn test_vim_mode_enter_visual() {
        let mut mode = VimMode::new();
        mode.enter_visual_mode();
        assert_eq!(mode.current_mode(), EditMode::Visual);
    }

    #[test]
    fn test_vim_mode_enter_visual_line() {
        let mut mode = VimMode::new();
        mode.enter_visual_line_mode();
        assert_eq!(mode.current_mode(), EditMode::VisualLine);
    }

    #[test]
    fn test_vim_mode_enter_visual_block() {
        let mut mode = VimMode::new();
        mode.enter_visual_block_mode();
        assert_eq!(mode.current_mode(), EditMode::VisualBlock);
    }

    #[test]
    fn test_vim_mode_enter_command() {
        let mut mode = VimMode::new();
        mode.enter_command_mode();
        assert_eq!(mode.current_mode(), EditMode::Command);
    }

    #[test]
    fn test_vim_mode_exit_to_normal() {
        let mut mode = VimMode::new();
        mode.enter_insert_mode();
        mode.exit_to_normal();
        assert_eq!(mode.current_mode(), EditMode::Normal);
    }

    #[test]
    fn test_vim_mode_selection() {
        let mut mode = VimMode::new();
        mode.enter_visual_mode();

        assert!(!mode.has_selection());

        mode.set_selection(Selection::new((0, 0), (0, 5)));
        assert!(mode.has_selection());

        mode.clear_selection();
        assert!(!mode.has_selection());
    }

    #[test]
    fn test_vim_mode_command_buffer() {
        let mut mode = VimMode::new();
        mode.enter_command_mode();

        mode.add_command_char('w');
        mode.add_command_char('q');
        assert_eq!(mode.command_buffer(), "wq");

        mode.remove_command_char();
        assert_eq!(mode.command_buffer(), "w");

        mode.clear_command_buffer();
        assert_eq!(mode.command_buffer(), "");
    }

    #[test]
    fn test_mode_indicator_new() {
        let indicator = ModeIndicator::new(EditMode::Normal);
        assert_eq!(indicator.mode, EditMode::Normal);
        assert!(indicator.show_hints);
        assert!(!indicator.compact);
    }

    #[test]
    fn test_mode_indicator_with_hints() {
        let indicator = ModeIndicator::new(EditMode::Normal).with_hints(false);
        assert!(!indicator.show_hints);
    }

    #[test]
    fn test_mode_indicator_with_compact() {
        let indicator = ModeIndicator::new(EditMode::Normal).with_compact(true);
        assert!(indicator.compact);
    }

    #[test]
    fn test_mode_indicator_render_line() {
        let indicator = ModeIndicator::new(EditMode::Insert);
        let line = indicator.render_line();
        assert!(!line.spans.is_empty());
    }

    #[test]
    fn test_mode_transitions() {
        let mut mode = VimMode::new();

        // Normal -> Insert -> Normal
        assert_eq!(mode.current_mode(), EditMode::Normal);
        mode.enter_insert_mode();
        assert_eq!(mode.current_mode(), EditMode::Insert);
        mode.exit_to_normal();
        assert_eq!(mode.current_mode(), EditMode::Normal);

        // Normal -> Visual -> Normal
        mode.enter_visual_mode();
        assert_eq!(mode.current_mode(), EditMode::Visual);
        mode.exit_to_normal();
        assert_eq!(mode.current_mode(), EditMode::Normal);

        // Normal -> Command -> Normal
        mode.enter_command_mode();
        assert_eq!(mode.current_mode(), EditMode::Command);
        mode.exit_to_normal();
        assert_eq!(mode.current_mode(), EditMode::Normal);
    }
}
