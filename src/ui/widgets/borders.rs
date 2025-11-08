//! Beautiful Unicode box drawing characters for borders
//!
//! Provides various border styles using Unicode box-drawing characters for
//! enhanced visual aesthetics in terminal applications.
//!
//! # Examples
//!
//! ```
//! use toad::widgets::{BorderStyle, BorderSet};
//!
//! // Use predefined border style
//! let rounded = BorderStyle::Rounded.get_border_set();
//! assert_eq!(rounded.top_left, "╭");
//!
//! // Create custom border
//! let custom = BorderSet::new("┏", "┓", "┗", "┛", "━", "┃");
//! ```

/// Unicode box-drawing character sets for different border styles
///
/// # Examples
///
/// ```
/// use toad::widgets::BorderSet;
///
/// let set = BorderSet::rounded();
/// assert_eq!(set.top_left, "╭");
/// assert_eq!(set.horizontal, "─");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorderSet {
    /// Top-left corner character
    pub top_left: &'static str,
    /// Top-right corner character
    pub top_right: &'static str,
    /// Bottom-left corner character
    pub bottom_left: &'static str,
    /// Bottom-right corner character
    pub bottom_right: &'static str,
    /// Horizontal line character
    pub horizontal: &'static str,
    /// Vertical line character
    pub vertical: &'static str,
}

impl BorderSet {
    /// Create a custom border set
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let custom = BorderSet::new("┏", "┓", "┗", "┛", "━", "┃");
    /// assert_eq!(custom.top_left, "┏");
    /// ```
    pub const fn new(
        top_left: &'static str,
        top_right: &'static str,
        bottom_left: &'static str,
        bottom_right: &'static str,
        horizontal: &'static str,
        vertical: &'static str,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            horizontal,
            vertical,
        }
    }

    /// Plain single-line box drawing (│ ─ ┌ ┐ └ ┘)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let plain = BorderSet::plain();
    /// assert_eq!(plain.top_left, "┌");
    /// ```
    pub const fn plain() -> Self {
        Self::new("┌", "┐", "└", "┘", "─", "│")
    }

    /// Thick/bold box drawing (┃ ━ ┏ ┓ ┗ ┛)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let thick = BorderSet::thick();
    /// assert_eq!(thick.top_left, "┏");
    /// ```
    pub const fn thick() -> Self {
        Self::new("┏", "┓", "┗", "┛", "━", "┃")
    }

    /// Double-line box drawing (║ ═ ╔ ╗ ╚ ╝)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let double = BorderSet::double();
    /// assert_eq!(double.top_left, "╔");
    /// ```
    pub const fn double() -> Self {
        Self::new("╔", "╗", "╚", "╝", "═", "║")
    }

    /// Rounded corners (│ ─ ╭ ╮ ╰ ╯)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let rounded = BorderSet::rounded();
    /// assert_eq!(rounded.top_left, "╭");
    /// ```
    pub const fn rounded() -> Self {
        Self::new("╭", "╮", "╰", "╯", "─", "│")
    }

    /// ASCII-only borders (| - + + + +)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let ascii = BorderSet::ascii();
    /// assert_eq!(ascii.top_left, "+");
    /// assert_eq!(ascii.horizontal, "-");
    /// ```
    pub const fn ascii() -> Self {
        Self::new("+", "+", "+", "+", "-", "|")
    }

    /// Heavy/extra thick borders (┃ ━ ┏ ┓ ┗ ┛) - same as thick for compatibility
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let heavy = BorderSet::heavy();
    /// assert_eq!(heavy.horizontal, "━");
    /// ```
    pub const fn heavy() -> Self {
        Self::thick()
    }

    /// Dashed horizontal lines (┆ ╌ ┌ ┐ └ ┘)
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let dashed = BorderSet::dashed();
    /// assert_eq!(dashed.horizontal, "╌");
    /// ```
    pub const fn dashed() -> Self {
        Self::new("┌", "┐", "└", "┘", "╌", "┆")
    }

    /// Draw a horizontal line of specified width
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let border = BorderSet::plain();
    /// let line = border.horizontal_line(5);
    /// assert_eq!(line, "─────");
    /// ```
    pub fn horizontal_line(&self, width: usize) -> String {
        self.horizontal.repeat(width)
    }

    /// Draw a top border line with corners
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let border = BorderSet::plain();
    /// let top = border.top_border(7);
    /// assert_eq!(top, "┌─────┐");
    /// ```
    pub fn top_border(&self, width: usize) -> String {
        if width < 2 {
            return String::new();
        }
        format!(
            "{}{}{}",
            self.top_left,
            self.horizontal.repeat(width - 2),
            self.top_right
        )
    }

    /// Draw a bottom border line with corners
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let border = BorderSet::plain();
    /// let bottom = border.bottom_border(7);
    /// assert_eq!(bottom, "└─────┘");
    /// ```
    pub fn bottom_border(&self, width: usize) -> String {
        if width < 2 {
            return String::new();
        }
        format!(
            "{}{}{}",
            self.bottom_left,
            self.horizontal.repeat(width - 2),
            self.bottom_right
        )
    }

    /// Draw a middle line with vertical borders and content
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let border = BorderSet::plain();
    /// let line = border.middle_line("Hello", 12);
    /// assert_eq!(line, "│Hello     │");
    /// ```
    pub fn middle_line(&self, content: &str, width: usize) -> String {
        if width < 2 {
            return String::new();
        }
        let content_width = width - 2;
        let padded = format!("{:width$}", content, width = content_width);
        format!("{}{}{}", self.vertical, padded, self.vertical)
    }

    /// Draw a complete box with content
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderSet;
    ///
    /// let border = BorderSet::rounded();
    /// let lines = vec!["Hello", "World"];
    /// let box_str = border.draw_box(&lines, 12);
    /// assert!(box_str.contains("╭──────────╮"));
    /// assert!(box_str.contains("│Hello     │"));
    /// ```
    pub fn draw_box(&self, lines: &[&str], width: usize) -> String {
        if width < 2 {
            return String::new();
        }

        let mut result = String::new();

        // Top border
        result.push_str(&self.top_border(width));
        result.push('\n');

        // Content lines
        for line in lines {
            result.push_str(&self.middle_line(line, width));
            result.push('\n');
        }

        // Bottom border
        result.push_str(&self.bottom_border(width));

        result
    }
}

/// Predefined border styles
///
/// # Examples
///
/// ```
/// use toad::widgets::BorderStyle;
///
/// let style = BorderStyle::Rounded;
/// let border = style.get_border_set();
/// assert_eq!(border.top_left, "╭");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// Plain single-line borders (┌─┐│└─┘)
    #[default]
    Plain,
    /// Thick/bold borders (┏━┓┃┗━┛)
    Thick,
    /// Double-line borders (╔═╗║╚═╝)
    Double,
    /// Rounded corners (╭─╮│╰─╯)
    Rounded,
    /// ASCII-only borders (+-+|+-+)
    Ascii,
    /// Heavy/extra thick borders
    Heavy,
    /// Dashed line borders (┌╌┐┆└╌┘)
    Dashed,
}

impl BorderStyle {
    /// Get the border character set for this style
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderStyle;
    ///
    /// let border = BorderStyle::Double.get_border_set();
    /// assert_eq!(border.top_left, "╔");
    /// assert_eq!(border.horizontal, "═");
    /// ```
    pub const fn get_border_set(&self) -> BorderSet {
        match self {
            BorderStyle::Plain => BorderSet::plain(),
            BorderStyle::Thick => BorderSet::thick(),
            BorderStyle::Double => BorderSet::double(),
            BorderStyle::Rounded => BorderSet::rounded(),
            BorderStyle::Ascii => BorderSet::ascii(),
            BorderStyle::Heavy => BorderSet::heavy(),
            BorderStyle::Dashed => BorderSet::dashed(),
        }
    }

    /// Get all available border styles
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderStyle;
    ///
    /// let styles = BorderStyle::all();
    /// assert_eq!(styles.len(), 7);
    /// ```
    pub fn all() -> Vec<BorderStyle> {
        vec![
            BorderStyle::Plain,
            BorderStyle::Thick,
            BorderStyle::Double,
            BorderStyle::Rounded,
            BorderStyle::Ascii,
            BorderStyle::Heavy,
            BorderStyle::Dashed,
        ]
    }

    /// Get style name as string
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::widgets::BorderStyle;
    ///
    /// assert_eq!(BorderStyle::Rounded.name(), "Rounded");
    /// assert_eq!(BorderStyle::Double.name(), "Double");
    /// ```
    pub const fn name(&self) -> &'static str {
        match self {
            BorderStyle::Plain => "Plain",
            BorderStyle::Thick => "Thick",
            BorderStyle::Double => "Double",
            BorderStyle::Rounded => "Rounded",
            BorderStyle::Ascii => "Ascii",
            BorderStyle::Heavy => "Heavy",
            BorderStyle::Dashed => "Dashed",
        }
    }

    /// Draw a horizontal line
    pub fn horizontal_line(&self, width: usize) -> String {
        self.get_border_set().horizontal_line(width)
    }

    /// Draw a top border
    pub fn top_border(&self, width: usize) -> String {
        self.get_border_set().top_border(width)
    }

    /// Draw a bottom border
    pub fn bottom_border(&self, width: usize) -> String {
        self.get_border_set().bottom_border(width)
    }

    /// Draw a middle line with content
    pub fn middle_line(&self, content: &str, width: usize) -> String {
        self.get_border_set().middle_line(content, width)
    }

    /// Draw a complete box
    pub fn draw_box(&self, lines: &[&str], width: usize) -> String {
        self.get_border_set().draw_box(lines, width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_set_plain() {
        let border = BorderSet::plain();
        assert_eq!(border.top_left, "┌");
        assert_eq!(border.top_right, "┐");
        assert_eq!(border.bottom_left, "└");
        assert_eq!(border.bottom_right, "┘");
        assert_eq!(border.horizontal, "─");
        assert_eq!(border.vertical, "│");
    }

    #[test]
    fn test_border_set_thick() {
        let border = BorderSet::thick();
        assert_eq!(border.top_left, "┏");
        assert_eq!(border.horizontal, "━");
    }

    #[test]
    fn test_border_set_double() {
        let border = BorderSet::double();
        assert_eq!(border.top_left, "╔");
        assert_eq!(border.horizontal, "═");
    }

    #[test]
    fn test_border_set_rounded() {
        let border = BorderSet::rounded();
        assert_eq!(border.top_left, "╭");
        assert_eq!(border.bottom_right, "╯");
    }

    #[test]
    fn test_border_set_ascii() {
        let border = BorderSet::ascii();
        assert_eq!(border.top_left, "+");
        assert_eq!(border.horizontal, "-");
        assert_eq!(border.vertical, "|");
    }

    #[test]
    fn test_border_set_custom() {
        let border = BorderSet::new("A", "B", "C", "D", "E", "F");
        assert_eq!(border.top_left, "A");
        assert_eq!(border.top_right, "B");
        assert_eq!(border.bottom_left, "C");
        assert_eq!(border.bottom_right, "D");
        assert_eq!(border.horizontal, "E");
        assert_eq!(border.vertical, "F");
    }

    #[test]
    fn test_horizontal_line() {
        let border = BorderSet::plain();
        assert_eq!(border.horizontal_line(5), "─────");
        assert_eq!(border.horizontal_line(0), "");
    }

    #[test]
    fn test_top_border() {
        let border = BorderSet::plain();
        assert_eq!(border.top_border(5), "┌───┐");
        assert_eq!(border.top_border(2), "┌┐");
        assert_eq!(border.top_border(1), "");
    }

    #[test]
    fn test_bottom_border() {
        let border = BorderSet::plain();
        assert_eq!(border.bottom_border(5), "└───┘");
        assert_eq!(border.bottom_border(2), "└┘");
        assert_eq!(border.bottom_border(1), "");
    }

    #[test]
    fn test_middle_line() {
        let border = BorderSet::plain();
        assert_eq!(border.middle_line("Hi", 6), "│Hi  │");
        assert_eq!(border.middle_line("", 4), "│  │");
    }

    #[test]
    fn test_draw_box() {
        let border = BorderSet::plain();
        let lines = vec!["Hello"];
        let result = border.draw_box(&lines, 10);

        assert!(result.contains("┌────────┐"));
        assert!(result.contains("│Hello   │"));
        assert!(result.contains("└────────┘"));
    }

    #[test]
    fn test_border_style_get_border_set() {
        let plain = BorderStyle::Plain.get_border_set();
        assert_eq!(plain.top_left, "┌");

        let rounded = BorderStyle::Rounded.get_border_set();
        assert_eq!(rounded.top_left, "╭");
    }

    #[test]
    fn test_border_style_all() {
        let styles = BorderStyle::all();
        assert_eq!(styles.len(), 7);
        assert!(styles.contains(&BorderStyle::Plain));
        assert!(styles.contains(&BorderStyle::Rounded));
    }

    #[test]
    fn test_border_style_name() {
        assert_eq!(BorderStyle::Plain.name(), "Plain");
        assert_eq!(BorderStyle::Thick.name(), "Thick");
        assert_eq!(BorderStyle::Double.name(), "Double");
        assert_eq!(BorderStyle::Rounded.name(), "Rounded");
        assert_eq!(BorderStyle::Ascii.name(), "Ascii");
    }

    #[test]
    fn test_border_style_methods() {
        let style = BorderStyle::Rounded;

        let line = style.horizontal_line(5);
        assert_eq!(line, "─────");

        let top = style.top_border(5);
        assert_eq!(top, "╭───╮");

        let bottom = style.bottom_border(5);
        assert_eq!(bottom, "╰───╯");
    }

    #[test]
    fn test_border_style_draw_box() {
        let style = BorderStyle::Double;
        let lines = vec!["Test", "Box"];
        let result = style.draw_box(&lines, 8);

        assert!(result.contains("╔══════╗"));
        assert!(result.contains("║Test  ║"));
        assert!(result.contains("║Box   ║"));
        assert!(result.contains("╚══════╝"));
    }

    #[test]
    fn test_border_set_equality() {
        let border1 = BorderSet::plain();
        let border2 = BorderSet::plain();
        assert_eq!(border1, border2);
    }

    #[test]
    fn test_border_style_default() {
        let style = BorderStyle::default();
        assert_eq!(style, BorderStyle::Plain);
    }

    #[test]
    fn test_dashed_border() {
        let border = BorderSet::dashed();
        assert_eq!(border.horizontal, "╌");
        assert_eq!(border.vertical, "┆");
    }

    #[test]
    fn test_heavy_border() {
        let border = BorderSet::heavy();
        assert_eq!(border.horizontal, "━");
    }

    #[test]
    fn test_all_styles_render() {
        for style in BorderStyle::all() {
            let border = style.get_border_set();
            let result = border.draw_box(&["Test"], 10);
            assert!(!result.is_empty());
            assert!(result.contains("Test"));
        }
    }

    #[test]
    fn test_multiline_box() {
        let border = BorderSet::rounded();
        let lines = vec!["Line 1", "Line 2", "Line 3"];
        let result = border.draw_box(&lines, 12);

        assert!(result.contains("╭──────────╮"));
        assert!(result.contains("│Line 1    │"));
        assert!(result.contains("│Line 2    │"));
        assert!(result.contains("│Line 3    │"));
        assert!(result.contains("╰──────────╯"));
    }

    #[test]
    fn test_empty_box() {
        let border = BorderSet::plain();
        let lines: Vec<&str> = vec![];
        let result = border.draw_box(&lines, 5);

        assert!(result.contains("┌───┐"));
        assert!(result.contains("└───┘"));
    }
}
