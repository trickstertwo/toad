/// Loading spinner widget
///
/// Aesthetic async indicators with various animation styles
///
/// # Examples
///
/// ```
/// use toad::widgets::{Spinner, SpinnerStyle};
///
/// let spinner = Spinner::new(SpinnerStyle::Dots);
/// assert_eq!(spinner.current_frame(), 0);
/// ```

use crate::theme::ToadTheme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use serde::{Deserialize, Serialize};

/// Spinner animation style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpinnerStyle {
    /// Rotating dots: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
    Dots,
    /// Rotating line: | / - \
    Line,
    /// Growing bars: ▁ ▂ ▃ ▄ ▅ ▆ ▇ █
    Bars,
    /// Bouncing ball: ⠁ ⠂ ⠄ ⡀ ⢀ ⠠ ⠐ ⠈
    Bounce,
    /// Arrow chase: ← ↖ ↑ ↗ → ↘ ↓ ↙
    Arrows,
    /// Simple dots: . .. ...
    SimpleDots,
    /// Binary: 0 1
    Binary,
    /// Clock: 🕐 🕑 🕒 🕓 🕔 🕕 🕖 🕗 🕘 🕙 🕚 🕛
    Clock,
}

impl SpinnerStyle {
    /// Get animation frames for this style
    pub fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            SpinnerStyle::Line => &["|", "/", "-", "\\"],
            SpinnerStyle::Bars => &["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▂"],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerStyle::Arrows => &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            SpinnerStyle::SimpleDots => &["   ", ".  ", ".. ", "..."],
            SpinnerStyle::Binary => &["0", "1"],
            SpinnerStyle::Clock => &["🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛"],
        }
    }

    /// Get frame count for this style
    pub fn frame_count(&self) -> usize {
        self.frames().len()
    }

    /// Get name of this style
    pub fn name(&self) -> &'static str {
        match self {
            SpinnerStyle::Dots => "Dots",
            SpinnerStyle::Line => "Line",
            SpinnerStyle::Bars => "Bars",
            SpinnerStyle::Bounce => "Bounce",
            SpinnerStyle::Arrows => "Arrows",
            SpinnerStyle::SimpleDots => "Simple Dots",
            SpinnerStyle::Binary => "Binary",
            SpinnerStyle::Clock => "Clock",
        }
    }
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        SpinnerStyle::Dots
    }
}

/// Loading spinner widget
#[derive(Debug, Clone)]
pub struct Spinner {
    /// Spinner style
    style: SpinnerStyle,
    /// Current frame index
    frame: usize,
    /// Optional label text
    label: Option<String>,
    /// Color for spinner
    color: ratatui::style::Color,
}

impl Spinner {
    /// Create a new spinner
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let spinner = Spinner::new(SpinnerStyle::Dots);
    /// ```
    pub fn new(style: SpinnerStyle) -> Self {
        Self {
            style,
            frame: 0,
            label: None,
            color: ToadTheme::TOAD_GREEN,
        }
    }

    /// Set label text
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let spinner = Spinner::new(SpinnerStyle::Dots).label("Loading...");
    /// ```
    pub fn label(mut self, text: impl Into<String>) -> Self {
        self.label = Some(text.into());
        self
    }

    /// Set spinner color
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    /// use toad::theme::ToadTheme;
    ///
    /// let spinner = Spinner::new(SpinnerStyle::Dots).color(ToadTheme::BLUE);
    /// ```
    pub fn color(mut self, color: ratatui::style::Color) -> Self {
        self.color = color;
        self
    }

    /// Advance to next frame
    ///
    /// Call this on each tick/update cycle
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// assert_eq!(spinner.current_frame(), 0);
    /// spinner.tick();
    /// assert_eq!(spinner.current_frame(), 1);
    /// ```
    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % self.style.frame_count();
    }

    /// Get current frame index
    pub fn current_frame(&self) -> usize {
        self.frame
    }

    /// Get current frame symbol
    pub fn current_symbol(&self) -> &str {
        self.style.frames()[self.frame]
    }

    /// Reset to first frame
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::{Spinner, SpinnerStyle};
    ///
    /// let mut spinner = Spinner::new(SpinnerStyle::Dots);
    /// spinner.tick();
    /// spinner.tick();
    /// assert_eq!(spinner.current_frame(), 2);
    /// spinner.reset();
    /// assert_eq!(spinner.current_frame(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.frame = 0;
    }

    /// Set frame index directly
    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame % self.style.frame_count();
    }

    /// Get spinner style
    pub fn style(&self) -> SpinnerStyle {
        self.style
    }

    /// Set spinner style
    pub fn set_style(&mut self, style: SpinnerStyle) {
        self.style = style;
        self.frame = 0; // Reset frame when changing style
    }

    /// Render the spinner
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let symbol = self.current_symbol();
        let spinner_style = Style::default()
            .fg(self.color)
            .add_modifier(Modifier::BOLD);

        let text = if let Some(label) = &self.label {
            Line::from(vec![
                Span::styled(symbol, spinner_style),
                Span::raw(" "),
                Span::styled(label, Style::default().fg(ToadTheme::FOREGROUND)),
            ])
        } else {
            Line::from(Span::styled(symbol, spinner_style))
        };

        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        frame.render_widget(paragraph, area);
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new(SpinnerStyle::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new(SpinnerStyle::Dots);
        assert_eq!(spinner.current_frame(), 0);
        assert_eq!(spinner.style(), SpinnerStyle::Dots);
    }

    #[test]
    fn test_spinner_tick() {
        let mut spinner = Spinner::new(SpinnerStyle::Line);
        assert_eq!(spinner.current_frame(), 0);

        spinner.tick();
        assert_eq!(spinner.current_frame(), 1);

        spinner.tick();
        assert_eq!(spinner.current_frame(), 2);

        spinner.tick();
        assert_eq!(spinner.current_frame(), 3);

        // Should wrap around
        spinner.tick();
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_reset() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        spinner.tick();
        spinner.tick();
        assert_eq!(spinner.current_frame(), 2);

        spinner.reset();
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_with_label() {
        let spinner = Spinner::new(SpinnerStyle::Dots).label("Loading...");
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_set_frame() {
        let mut spinner = Spinner::new(SpinnerStyle::Arrows);
        spinner.set_frame(5);
        assert_eq!(spinner.current_frame(), 5);

        // Test wrap around
        spinner.set_frame(20);
        assert!(spinner.current_frame() < SpinnerStyle::Arrows.frame_count());
    }

    #[test]
    fn test_spinner_current_symbol() {
        let mut spinner = Spinner::new(SpinnerStyle::Line);
        assert_eq!(spinner.current_symbol(), "|");

        spinner.tick();
        assert_eq!(spinner.current_symbol(), "/");

        spinner.tick();
        assert_eq!(spinner.current_symbol(), "-");

        spinner.tick();
        assert_eq!(spinner.current_symbol(), "\\");
    }

    #[test]
    fn test_spinner_styles() {
        assert_eq!(SpinnerStyle::Dots.name(), "Dots");
        assert_eq!(SpinnerStyle::Line.name(), "Line");
        assert_eq!(SpinnerStyle::Bars.name(), "Bars");
        assert_eq!(SpinnerStyle::Bounce.name(), "Bounce");
        assert_eq!(SpinnerStyle::Arrows.name(), "Arrows");
        assert_eq!(SpinnerStyle::SimpleDots.name(), "Simple Dots");
        assert_eq!(SpinnerStyle::Binary.name(), "Binary");
        assert_eq!(SpinnerStyle::Clock.name(), "Clock");
    }

    #[test]
    fn test_spinner_frame_counts() {
        assert_eq!(SpinnerStyle::Dots.frame_count(), 10);
        assert_eq!(SpinnerStyle::Line.frame_count(), 4);
        assert_eq!(SpinnerStyle::Bars.frame_count(), 14);
        assert_eq!(SpinnerStyle::Bounce.frame_count(), 8);
        assert_eq!(SpinnerStyle::Arrows.frame_count(), 8);
        assert_eq!(SpinnerStyle::SimpleDots.frame_count(), 4);
        assert_eq!(SpinnerStyle::Binary.frame_count(), 2);
        assert_eq!(SpinnerStyle::Clock.frame_count(), 12);
    }

    #[test]
    fn test_spinner_set_style() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        spinner.tick();
        spinner.tick();
        assert_eq!(spinner.current_frame(), 2);

        // Changing style should reset frame
        spinner.set_style(SpinnerStyle::Line);
        assert_eq!(spinner.current_frame(), 0);
        assert_eq!(spinner.style(), SpinnerStyle::Line);
    }

    #[test]
    fn test_spinner_default() {
        let spinner = Spinner::default();
        assert_eq!(spinner.style(), SpinnerStyle::Dots);
        assert_eq!(spinner.current_frame(), 0);
    }
}
