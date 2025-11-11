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

use crate::ui::atoms::{block::Block as AtomBlock, text::Text as AtomText};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Borders, Widget},
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
    pub(super) mode: EditMode,
    /// Previous mode (for ESC behavior)
    pub(super) previous_mode: EditMode,
    /// Visual mode selection
    pub(super) selection: Option<Selection>,
    /// Command buffer
    pub(super) command_buffer: String,
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
    pub(super) mode: EditMode,
    /// Show key hints
    pub(super) show_hints: bool,
    /// Compact mode
    pub(super) compact: bool,
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
        spans.push(
            AtomText::new(format!(" {} ", self.mode.name()))
                .style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(self.mode.color())
                        .add_modifier(Modifier::BOLD),
                )
                .to_span(),
        );

        // Key hints
        if self.show_hints && !self.compact {
            spans.push(AtomText::new(" ".to_string()).to_span());
            spans.push(
                AtomText::new(self.mode.key_hint().to_string())
                    .style(Style::default().fg(Color::DarkGray))
                    .to_span(),
            );
        }

        Line::from(spans)
    }
}

impl Widget for &ModeIndicator {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.render_line();
        let block = AtomBlock::new().borders(Borders::NONE).to_ratatui();
        let inner = block.inner(area);

        block.render(area, buf);
        buf.set_line(inner.x, inner.y, &line, inner.width);
    }
}
