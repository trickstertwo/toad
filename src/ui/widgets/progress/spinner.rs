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
use crate::ui::{atoms::text::Text, theme::ToadTheme};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};
use serde::{Deserialize, Serialize};

/// Spinner animation style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SpinnerStyle {
    /// Rotating dots: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏
    #[default]
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
            SpinnerStyle::Bars => &[
                "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▂",
            ],
            SpinnerStyle::Bounce => &["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
            SpinnerStyle::Arrows => &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            SpinnerStyle::SimpleDots => &["   ", ".  ", ".. ", "..."],
            SpinnerStyle::Binary => &["0", "1"],
            SpinnerStyle::Clock => &[
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
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
        let spinner_style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);

        // Use Text atoms for rendering
        let text = if let Some(label) = &self.label {
            let symbol_text = Text::new(symbol).style(spinner_style);
            let space_text = Text::new(" ");
            let label_text = Text::new(label).style(Style::default().fg(ToadTheme::FOREGROUND));

            Line::from(vec![symbol_text.to_span(), space_text.to_span(), label_text.to_span()])
        } else {
            let symbol_text = Text::new(symbol).style(spinner_style);
            Line::from(symbol_text.to_span())
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

    // Comprehensive SpinnerStyle tests

    #[test]
    fn test_spinner_style_frames_dots() {
        let frames = SpinnerStyle::Dots.frames();
        assert_eq!(frames.len(), 10);
        assert_eq!(frames[0], "⠋");
        assert_eq!(frames[9], "⠏");
    }

    #[test]
    fn test_spinner_style_frames_line() {
        let frames = SpinnerStyle::Line.frames();
        assert_eq!(frames.len(), 4);
        assert_eq!(frames, &["|", "/", "-", "\\"]);
    }

    #[test]
    fn test_spinner_style_frames_bars() {
        let frames = SpinnerStyle::Bars.frames();
        assert_eq!(frames.len(), 14);
        assert_eq!(frames[0], "▁");
        assert_eq!(frames[7], "█");
    }

    #[test]
    fn test_spinner_style_frames_binary() {
        let frames = SpinnerStyle::Binary.frames();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames, &["0", "1"]);
    }

    #[test]
    fn test_spinner_style_default() {
        assert_eq!(SpinnerStyle::default(), SpinnerStyle::Dots);
    }

    // Comprehensive Spinner tests

    #[test]
    fn test_spinner_multiple_ticks_full_rotation() {
        let mut spinner = Spinner::new(SpinnerStyle::Binary); // 2 frames

        spinner.tick();
        assert_eq!(spinner.current_frame(), 1);

        spinner.tick();
        assert_eq!(spinner.current_frame(), 0); // Wrapped

        spinner.tick();
        assert_eq!(spinner.current_frame(), 1);
    }

    #[test]
    fn test_spinner_tick_many_times() {
        let mut spinner = Spinner::new(SpinnerStyle::Line); // 4 frames

        for _ in 0..100 {
            spinner.tick();
        }

        // After 100 ticks on 4 frames: 100 % 4 = 0
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_set_frame_exact_boundary() {
        let mut spinner = Spinner::new(SpinnerStyle::Line); // 4 frames

        // Set to exactly frame_count (should wrap to 0)
        spinner.set_frame(4);
        assert_eq!(spinner.current_frame(), 0);

        // Set to frame_count - 1 (last valid frame)
        spinner.set_frame(3);
        assert_eq!(spinner.current_frame(), 3);
    }

    #[test]
    fn test_spinner_set_frame_zero() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        spinner.tick();
        spinner.tick();

        spinner.set_frame(0);
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_set_frame_large_value() {
        let mut spinner = Spinner::new(SpinnerStyle::Line); // 4 frames

        spinner.set_frame(1000);
        // 1000 % 4 = 0
        assert_eq!(spinner.current_frame(), 0);

        spinner.set_frame(1001);
        // 1001 % 4 = 1
        assert_eq!(spinner.current_frame(), 1);
    }

    #[test]
    fn test_spinner_change_style_multiple_times() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);
        spinner.tick();
        spinner.tick();

        spinner.set_style(SpinnerStyle::Line);
        assert_eq!(spinner.current_frame(), 0);

        spinner.tick();
        spinner.tick();
        assert_eq!(spinner.current_frame(), 2);

        spinner.set_style(SpinnerStyle::Binary);
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_all_styles_have_valid_frames() {
        let styles = [
            SpinnerStyle::Dots,
            SpinnerStyle::Line,
            SpinnerStyle::Bars,
            SpinnerStyle::Bounce,
            SpinnerStyle::Arrows,
            SpinnerStyle::SimpleDots,
            SpinnerStyle::Binary,
            SpinnerStyle::Clock,
        ];

        for style in &styles {
            let spinner = Spinner::new(*style);
            assert_eq!(spinner.current_frame(), 0);
            assert!(!spinner.current_symbol().is_empty());

            let frames = style.frames();
            assert!(frames.len() > 0);
            assert_eq!(frames.len(), style.frame_count());
        }
    }

    #[test]
    fn test_spinner_label_content() {
        let spinner = Spinner::new(SpinnerStyle::Dots).label("Loading...");
        assert!(spinner.label.is_some());
        assert_eq!(spinner.label.unwrap(), "Loading...");
    }

    #[test]
    fn test_spinner_label_empty_string() {
        let spinner = Spinner::new(SpinnerStyle::Dots).label("");
        assert!(spinner.label.is_some());
        assert_eq!(spinner.label.unwrap(), "");
    }

    #[test]
    fn test_spinner_no_label() {
        let spinner = Spinner::new(SpinnerStyle::Dots);
        assert!(spinner.label.is_none());
    }

    #[test]
    fn test_spinner_color_setting() {
        use ratatui::style::Color;

        let spinner = Spinner::new(SpinnerStyle::Dots).color(Color::Red);
        assert_eq!(spinner.color, Color::Red);

        let spinner = Spinner::new(SpinnerStyle::Dots).color(Color::Blue);
        assert_eq!(spinner.color, Color::Blue);
    }

    #[test]
    fn test_spinner_builder_chaining() {
        use ratatui::style::Color;

        let spinner = Spinner::new(SpinnerStyle::Arrows)
            .label("Processing...")
            .color(Color::Cyan);

        assert_eq!(spinner.style(), SpinnerStyle::Arrows);
        assert_eq!(spinner.label, Some("Processing...".to_string()));
        assert_eq!(spinner.color, Color::Cyan);
        assert_eq!(spinner.current_frame(), 0);
    }

    #[test]
    fn test_spinner_symbols_cycle_correctly() {
        let mut spinner = Spinner::new(SpinnerStyle::SimpleDots);
        let frames = SpinnerStyle::SimpleDots.frames();

        for (i, expected_frame) in frames.iter().enumerate() {
            assert_eq!(spinner.current_symbol(), *expected_frame);
            assert_eq!(spinner.current_frame(), i);
            spinner.tick();
        }

        // Should wrap back to first frame
        assert_eq!(spinner.current_frame(), 0);
        assert_eq!(spinner.current_symbol(), frames[0]);
    }

    #[test]
    fn test_spinner_reset_after_many_ticks() {
        let mut spinner = Spinner::new(SpinnerStyle::Dots);

        for _ in 0..50 {
            spinner.tick();
        }

        spinner.reset();
        assert_eq!(spinner.current_frame(), 0);
        assert_eq!(spinner.current_symbol(), SpinnerStyle::Dots.frames()[0]);
    }

    #[test]
    fn test_spinner_style_equality() {
        assert_eq!(SpinnerStyle::Dots, SpinnerStyle::Dots);
        assert_ne!(SpinnerStyle::Dots, SpinnerStyle::Line);
        assert_ne!(SpinnerStyle::Bars, SpinnerStyle::Binary);
    }
}
