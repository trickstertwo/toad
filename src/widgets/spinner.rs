//! Loading spinner widgets for async operations
//!
//! Provides animated spinners for visual feedback during long-running operations.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{Spinner, SpinnerStyle};
//!
//! // Create a dots spinner
//! let mut spinner = Spinner::new(SpinnerStyle::Dots);
//! spinner.tick(); // Advance animation
//!
//! assert!(spinner.is_active());
//! ```

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

/// Spinner animation styles
///
/// # Examples
///
/// ```
/// use toad::widgets::SpinnerStyle;
///
/// let dots = SpinnerStyle::Dots;
/// let bar = SpinnerStyle::Bar;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinnerStyle {
    /// Rotating dots: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
    Dots,
    /// Horizontal bar: ▁ ▂ ▃ ▄ ▅ ▆ ▇ █ ▇ ▆ ▅ ▄ ▃
    Bar,
    /// Growing arc: ◜ ◝ ◞ ◟
    Arc,
    /// Simple line: - \ | /
    Line,
    /// Bouncing ball: ⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈
    Bounce,
    /// Clock: 🕐 🕑 🕒 🕓 🕔 🕕 🕖 🕗 🕘 🕙 🕚 🕛
    Clock,
}

impl SpinnerStyle {
    /// Get animation frames for this style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::SpinnerStyle;
    ///
    /// let frames = SpinnerStyle::Dots.frames();
    /// assert!(frames.len() > 0);
    /// ```
    pub fn frames(self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Bar => &[
                "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃",
            ],
            SpinnerStyle::Arc => &["◜", "◝", "◞", "◟"],
            SpinnerStyle::Line => &["-", "\\", "|", "/"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerStyle::Clock => &[
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
        }
    }

    /// Get frame count for this style
    pub fn frame_count(self) -> usize {
        self.frames().len()
    }
}

/// Animated loading spinner
///
/// # Examples
///
/// ```
/// use toad::widgets::{Spinner, SpinnerStyle};
///
/// let mut spinner = Spinner::new(SpinnerStyle::Dots);
/// spinner.set_message("Loading...".to_string());
///
/// assert_eq!(spinner.message(), Some("Loading..."));
/// ```
#[derive(Debug, Clone)]
pub struct Spinner {
    /// Spinner style
    style: SpinnerStyle,
    /// Current frame index
    frame_index: usize,
    /// Whether spinner is active
    active: bool,
    /// Optional message to display
    message: Option<String>,
    /// Spinner color
    color: Color,
}

impl Spinner {
    /// Create a new spinner with the given style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let spinner = Spinner::new(SpinnerStyle::Dots);
    /// assert!(spinner.is_active());
    /// ```
    pub fn new(style: SpinnerStyle) -> Self {
        Self {
            style,
            frame_index: 0,
            active: true,
            message: None,
            color: Color::Green,
        }
    }

    /// Set the spinner message
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.set_message("Loading data...".to_string());
    /// assert_eq!(spinner.message(), Some("Loading data..."));
    /// ```
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }

    /// Get the spinner message
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Clear the spinner message
    pub fn clear_message(&mut self) {
        self.message = None;
    }

    /// Set the spinner color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    /// use ratatui::style::Color;
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.set_color(Color::Cyan);
    /// ```
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the spinner color
    pub fn color(&self) -> Color {
        self.color
    }

    /// Advance the animation by one frame
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// let initial_frame = spinner.current_frame();
    ///
    /// spinner.tick();
    /// // Frame should have advanced
    /// ```
    pub fn tick(&mut self) {
        if self.active {
            self.frame_index = (self.frame_index + 1) % self.style.frame_count();
        }
    }

    /// Reset animation to first frame
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.tick();
    /// spinner.tick();
    ///
    /// spinner.reset();
    /// assert_eq!(spinner.frame_index(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.frame_index = 0;
    }

    /// Get current frame index
    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    /// Get current frame character
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let spinner = Spinner::new(SpinnerStyle::Dots);
    /// let frame = spinner.current_frame();
    /// assert!(frame.len() > 0);
    /// ```
    pub fn current_frame(&self) -> &str {
        self.style.frames()[self.frame_index]
    }

    /// Check if spinner is active
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// assert!(spinner.is_active());
    ///
    /// spinner.stop();
    /// assert!(!spinner.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Start the spinner
    pub fn start(&mut self) {
        self.active = true;
    }

    /// Stop the spinner
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.stop();
    /// assert!(!spinner.is_active());
    /// ```
    pub fn stop(&mut self) {
        self.active = false;
    }

    /// Toggle spinner active state
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Get the spinner style
    pub fn style(&self) -> SpinnerStyle {
        self.style
    }

    /// Set the spinner style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.set_style(SpinnerStyle::Bar);
    /// assert_eq!(spinner.style(), SpinnerStyle::Bar);
    /// ```
    pub fn set_style(&mut self, style: SpinnerStyle) {
        self.style = style;
        self.reset(); // Reset frame index when style changes
    }

    /// Render the spinner to a string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.set_message("Loading...".to_string());
    ///
    /// let output = spinner.render_string();
    /// assert!(output.contains("Loading..."));
    /// ```
    pub fn render_string(&self) -> String {
        if let Some(msg) = &self.message {
            format!("{} {}", self.current_frame(), msg)
        } else {
            self.current_frame().to_string()
        }
    }
}

impl Widget for Spinner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let text = self.render_string();
        let style = Style::default().fg(self.color);

        // Render spinner text at top-left of area
        buf.set_string(area.x, area.y, text, style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_style_frames() {
        let dots = SpinnerStyle::Dots;
        assert!(!dots.frames().is_empty());

        let bar = SpinnerStyle::Bar;
        assert!(!bar.frames().is_empty());
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new(SpinnerStyle::Dots);
        assert!(spinner.is_active());
        assert_eq!(spinner.frame_index(), 0);
        assert_eq!(spinner.message(), None);
    }

    #[test]
    fn test_spinner_tick() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        let initial_frame = spinner.frame_index();

        spinner.tick();
        assert_eq!(spinner.frame_index(), initial_frame + 1);
    }

    #[test]
    fn test_spinner_tick_wraps() {
        let mut spinner = Spinner::new(SpinnerStyle::Arc);
        let frame_count = spinner.style().frame_count();

        // Tick through all frames
        for _ in 0..frame_count {
            spinner.tick();
        }

        // Should wrap to 0
        assert_eq!(spinner.frame_index(), 0);
    }

    #[test]
    fn test_spinner_reset() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        spinner.tick();
        spinner.tick();

        spinner.reset();
        assert_eq!(spinner.frame_index(), 0);
    }

    #[test]
    fn test_spinner_start_stop() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        assert!(spinner.is_active());

        spinner.stop();
        assert!(!spinner.is_active());

        spinner.start();
        assert!(spinner.is_active());
    }

    #[test]
    fn test_spinner_toggle() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        assert!(spinner.is_active());

        spinner.toggle();
        assert!(!spinner.is_active());

        spinner.toggle();
        assert!(spinner.is_active());
    }

    #[test]
    fn test_spinner_message() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        assert_eq!(spinner.message(), None);

        spinner.set_message("Loading...".to_string());
        assert_eq!(spinner.message(), Some("Loading..."));

        spinner.clear_message();
        assert_eq!(spinner.message(), None);
    }

    #[test]
    fn test_spinner_color() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        assert_eq!(spinner.color(), Color::Green);

        spinner.set_color(Color::Cyan);
        assert_eq!(spinner.color(), Color::Cyan);
    }

    #[test]
    fn test_spinner_style() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        assert_eq!(spinner.style(), SpinnerStyle::Dots);

        spinner.set_style(SpinnerStyle::Bar);
        assert_eq!(spinner.style(), SpinnerStyle::Bar);
        assert_eq!(spinner.frame_index(), 0); // Should reset
    }

    #[test]
    fn test_spinner_render_string() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        let output = spinner.render_string();
        assert!(!output.is_empty());

        spinner.set_message("Loading data...".to_string());
        let output = spinner.render_string();
        assert!(output.contains("Loading data..."));
    }

    #[test]
    fn test_inactive_spinner_no_tick() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        spinner.stop();

        let initial_frame = spinner.frame_index();
        spinner.tick();

        // Should not advance when inactive
        assert_eq!(spinner.frame_index(), initial_frame);
    }

    #[test]
    fn test_all_spinner_styles() {
        let styles = [
            SpinnerStyle::Dots,
            SpinnerStyle::Bar,
            SpinnerStyle::Arc,
            SpinnerStyle::Line,
            SpinnerStyle::Bounce,
            SpinnerStyle::Clock,
        ];

        for style in &styles {
            let spinner = Spinner::new(*style);
            assert!(!spinner.current_frame().is_empty());
        }
    }
}
