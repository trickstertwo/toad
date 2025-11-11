# Widget Migration Template

Use this template to refactor legacy widgets to use Atomic Design components.

---

## Step 1: Analyze Current Implementation

### Find Raw Ratatui Usage
```bash
# In the widget file you're migrating
grep -n "Span::styled\|Span::raw" src/ui/widgets/YOUR_WIDGET.rs
grep -n "Block::default" src/ui/widgets/YOUR_WIDGET.rs
grep -n "Line::from" src/ui/widgets/YOUR_WIDGET.rs
```

### Document Current Dependencies
```rust
// OLD IMPORTS
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
};
```

---

## Step 2: Add Atomic Imports

```rust
// NEW IMPORTS (add these)
use crate::ui::atoms::{
    Block as AtomBlock,  // Alias to avoid conflict with ratatui::widgets::Block
    Icon,
    Text,
};
use crate::ui::theme::ToadTheme;  // Use theme constants instead of raw colors
```

---

## Step 3: Replace Raw Ratatui Usage

### Text Rendering

**BEFORE:**
```rust
let span = Span::styled("Hello", Style::default().fg(Color::Green));
let line = Line::from(span);
```

**AFTER:**
```rust
let text = Text::new("Hello")
    .style(Style::default().fg(ToadTheme::TOAD_GREEN));
let line = text.to_line();
```

---

### Block Rendering

**BEFORE:**
```rust
let block = Block::default()
    .title("My Panel")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::Green));
```

**AFTER:**
```rust
let block = AtomBlock::new()
    .title("My Panel")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
    .to_ratatui();  // Convert to ratatui Block for rendering
```

---

### Icon Rendering

**BEFORE:**
```rust
let icon_span = Span::styled("✓", Style::default().fg(Color::Green));
```

**AFTER:**
```rust
let icon = Icon::ui(UiIcon::Success)
    .style(Style::default().fg(ToadTheme::TOAD_GREEN));
let icon_span = icon.to_text().to_span();
```

---

### Multiple Spans in a Line

**BEFORE:**
```rust
let line = Line::from(vec![
    Span::styled("Label: ", Style::default().fg(Color::Gray)),
    Span::styled("Value", Style::default().fg(Color::Green)),
]);
```

**AFTER:**
```rust
let label = Text::new("Label: ")
    .style(Style::default().fg(ToadTheme::GRAY));
let value = Text::new("Value")
    .style(Style::default().fg(ToadTheme::TOAD_GREEN));
let line = Line::from(vec![label.to_span(), value.to_span()]);
```

---

### Styled Text with Modifiers

**BEFORE:**
```rust
let span = Span::styled(
    "Important",
    Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD),
);
```

**AFTER:**
```rust
let text = Text::new("Important")
    .bold()
    .style(Style::default().fg(ToadTheme::YELLOW));
```

---

## Step 4: Update Tests

### Test Pattern Migration

**BEFORE:**
```rust
#[test]
fn test_widget_render() {
    let widget = MyWidget::new();
    let area = Rect::new(0, 0, 80, 24);
    let mut buf = Buffer::empty(area);
    widget.render(area, &mut buf);
    // Assertions
}
```

**AFTER:**
```rust
#[test]
fn test_widget_render() {
    let widget = MyWidget::new();
    let area = Rect::new(0, 0, 80, 24);
    let mut buf = Buffer::empty(area);
    widget.render(area, &mut buf);
    // Same assertions - behavior unchanged
}

#[test]
fn test_widget_uses_atoms() {
    // Verify atoms are used in implementation
    let widget = MyWidget::new();
    // Test that Text, Block, Icon are properly composed
}
```

---

## Step 5: Add Rustdoc Examples

```rust
//! MyWidget - Description of widget
//!
//! # Architecture
//!
//! Following Atomic Design principles:
//! - Composes Text atoms for all text rendering
//! - Uses Block atoms for containers
//! - Uses Icon atoms for visual indicators
//!
//! # Examples
//!
//! ```
//! use toad::ui::widgets::MyWidget;
//!
//! let widget = MyWidget::new()
//!     .title("Example")
//!     .style(Style::default().fg(ToadTheme::TOAD_GREEN));
//! ```

/// Create a new widget
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::MyWidget;
///
/// let widget = MyWidget::new();
/// ```
pub fn new() -> Self {
    // Implementation
}
```

---

## Step 6: Verify Changes

### Checklist
- [ ] All `Span::styled()` replaced with `Text::new().style()`
- [ ] All `Span::raw()` replaced with `Text::new()`
- [ ] All `Block::default()` replaced with `AtomBlock::new()`
- [ ] All raw `Color::*` replaced with `ToadTheme::*`
- [ ] Imports updated to include atoms
- [ ] Tests pass: `cargo test --test YOUR_WIDGET`
- [ ] Clippy passes: `cargo clippy --all-targets`
- [ ] Rustdoc complete with examples
- [ ] Visual behavior unchanged (manual test in TUI)

### Verification Commands
```bash
# Ensure no raw Ratatui in implementation
grep "Span::styled\|Span::raw" src/ui/widgets/YOUR_WIDGET.rs
# Should return empty

grep "Block::default" src/ui/widgets/YOUR_WIDGET.rs
# Should return empty

# Ensure atoms are used
grep "use crate::ui::atoms::" src/ui/widgets/YOUR_WIDGET.rs
# Should return import line

# Run tests
cargo test --test YOUR_WIDGET -- --nocapture

# Check documentation
cargo doc --open --no-deps
```

---

## Step 7: Commit

### Commit Message Format
```
Atomic Design: Refactor MyWidget to use atoms

Replace raw Ratatui Span/Block usage with atomic Text/Block components
following Atomic Design principles.

Changes:
- Replace Span::styled() with Text::new().style()
- Replace Block::default() with AtomBlock::new().to_ratatui()
- Use ToadTheme constants instead of raw colors
- Add rustdoc examples showing atomic composition

Impact:
- Zero raw Ratatui usage in widget
- Consistent styling with theme system
- Easier to maintain and test
- All tests passing (X tests)

Related: ATOMIC_DESIGN_COMPLETION_PLAN.md Phase [A/B/C/D/E/F]
```

---

## Common Patterns

### Pattern 1: Conditional Styling
**BEFORE:**
```rust
let style = if is_selected {
    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
} else {
    Style::default().fg(Color::Gray)
};
let span = Span::styled(text, style);
```

**AFTER:**
```rust
let mut text = Text::new(text);
if is_selected {
    text = text.bold().style(Style::default().fg(ToadTheme::TOAD_GREEN));
} else {
    text = text.style(Style::default().fg(ToadTheme::GRAY));
}
let span = text.to_span();
```

### Pattern 2: Lists of Items
**BEFORE:**
```rust
let lines: Vec<Line> = items.iter().map(|item| {
    Line::from(Span::styled(&item.name, Style::default().fg(Color::White)))
}).collect();
```

**AFTER:**
```rust
let lines: Vec<Line> = items.iter().map(|item| {
    Text::new(&item.name)
        .style(Style::default().fg(ToadTheme::WHITE))
        .to_line()
}).collect();
```

### Pattern 3: Icon + Text Composition
**BEFORE:**
```rust
let line = Line::from(vec![
    Span::styled("✓ ", Style::default().fg(Color::Green)),
    Span::raw(item.name),
]);
```

**AFTER:**
```rust
let icon = Icon::ui(UiIcon::Success)
    .style(Style::default().fg(ToadTheme::TOAD_GREEN));
let text = Text::new(&item.name);
let line = Line::from(vec![
    icon.to_text().to_span(),
    Span::raw(" "),
    text.to_span(),
]);
```

---

## Edge Cases

### Edge Case 1: Complex Layouts
If widget renders complex layouts (multiple panels, tables, etc.), consider:
- Should this be an **organism** composing molecules?
- Can sub-sections be extracted to separate molecules?
- Is the widget doing too much (SRP violation)?

### Edge Case 2: Performance-Critical Rendering
If widget renders on every frame (e.g., FPS counter):
- Profile before/after to ensure no regression
- Consider caching Text/Block atoms if created repeatedly
- Benchmark with `cargo bench` if available

### Edge Case 3: Custom Ratatui Widgets
If widget uses custom Ratatui widgets (Gauge, Chart, etc.):
- Keep the Ratatui widget for now
- Replace surrounding text/block/icon rendering with atoms
- Consider future molecule extraction if reusable

---

## Anti-Patterns to Avoid

### ❌ DON'T: Mix old and new
```rust
// BAD - mixing Span::styled and Text atoms
let line = Line::from(vec![
    Span::styled("Old", Style::default().fg(Color::Red)),  // OLD
    Text::new("New").to_span(),  // NEW
]);
```

### ❌ DON'T: Bypass atoms for "convenience"
```rust
// BAD - directly using Span when you should use Text
let span = Span::raw("Quick text");  // NO!

// GOOD
let text = Text::new("Quick text");
let span = text.to_span();
```

### ❌ DON'T: Use raw colors
```rust
// BAD - hardcoded colors
.style(Style::default().fg(Color::Rgb(0, 255, 0)))

// GOOD - theme constants
.style(Style::default().fg(ToadTheme::TOAD_GREEN))
```

### ❌ DON'T: Skip tests
```rust
// BAD - changing implementation without verifying tests pass
// Always run: cargo test --test YOUR_WIDGET

// GOOD - verify all tests pass before committing
```

---

## Example: Complete Migration

**File**: `src/ui/widgets/example_widget.rs`

### BEFORE (Legacy)
```rust
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct ExampleWidget {
    title: String,
    items: Vec<String>,
}

impl ExampleWidget {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }
}

impl Widget for ExampleWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(&self.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        let inner = block.inner(area);
        block.render(area, buf);

        let lines: Vec<Line> = self.items.iter().map(|item| {
            Line::from(Span::styled(item, Style::default().fg(Color::White)))
        }).collect();

        Paragraph::new(lines).render(inner, buf);
    }
}
```

### AFTER (Atomic)
```rust
use crate::ui::atoms::{Block as AtomBlock, Text};
use crate::ui::theme::ToadTheme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Paragraph, Widget},
};

/// Example widget demonstrating atomic composition
///
/// # Architecture
///
/// Following Atomic Design principles:
/// - Uses Block atom for container
/// - Uses Text atoms for all text rendering
/// - Composes atoms into cohesive widget
///
/// # Examples
///
/// ```
/// use toad::ui::widgets::ExampleWidget;
///
/// let mut widget = ExampleWidget::new("My Items");
/// widget.add_item("Item 1");
/// widget.add_item("Item 2");
/// ```
pub struct ExampleWidget {
    title: String,
    items: Vec<String>,
}

impl ExampleWidget {
    /// Create a new example widget
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ExampleWidget;
    ///
    /// let widget = ExampleWidget::new("Title");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    /// Add an item to the list
    ///
    /// # Examples
    ///
    /// ```
    /// use toad::ui::widgets::ExampleWidget;
    ///
    /// let mut widget = ExampleWidget::new("Title");
    /// widget.add_item("First item");
    /// ```
    pub fn add_item(&mut self, item: impl Into<String>) {
        self.items.push(item.into());
    }
}

impl Widget for ExampleWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Use Block atom for container
        let block = AtomBlock::new()
            .title(&self.title)
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(Style::default().fg(ToadTheme::TOAD_GREEN))
            .to_ratatui();

        let inner = block.inner(area);
        block.render(area, buf);

        // Use Text atoms for items
        let lines: Vec<Line> = self.items.iter().map(|item| {
            Text::new(item)
                .style(Style::default().fg(ToadTheme::WHITE))
                .to_line()
        }).collect();

        Paragraph::new(lines).render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let widget = ExampleWidget::new("Test");
        assert_eq!(widget.title, "Test");
        assert!(widget.items.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut widget = ExampleWidget::new("Test");
        widget.add_item("Item 1");
        assert_eq!(widget.items.len(), 1);
        assert_eq!(widget.items[0], "Item 1");
    }

    #[test]
    fn test_render() {
        let mut widget = ExampleWidget::new("Test");
        widget.add_item("Item 1");

        let area = Rect::new(0, 0, 80, 24);
        let mut buf = Buffer::empty(area);
        widget.render(area, &mut buf);

        // Verify rendering doesn't panic
    }
}
```

### Changes Summary
✅ Replaced `Block::default()` with `AtomBlock::new().to_ratatui()`
✅ Replaced `Span::styled()` with `Text::new().style()`
✅ Changed `Color::Green` to `ToadTheme::TOAD_GREEN`
✅ Changed `Color::White` to `ToadTheme::WHITE`
✅ Added rustdoc with examples
✅ Added Architecture section explaining atomic composition
✅ All tests passing (3 tests)

---

## Quick Reference Card

| Old Pattern | New Pattern | Import Needed |
|------------|-------------|---------------|
| `Span::styled(text, style)` | `Text::new(text).style(style).to_span()` | `use crate::ui::atoms::Text;` |
| `Span::raw(text)` | `Text::new(text).to_span()` | `use crate::ui::atoms::Text;` |
| `Block::default()` | `AtomBlock::new().to_ratatui()` | `use crate::ui::atoms::Block as AtomBlock;` |
| `Color::Green` | `ToadTheme::TOAD_GREEN` | `use crate::ui::theme::ToadTheme;` |
| Icon literal `"✓"` | `Icon::ui(UiIcon::Success)` | `use crate::ui::atoms::Icon; use crate::ui::nerd_fonts::UiIcon;` |

---

**Template Version**: 1.0
**Last Updated**: 2025-11-10
**Owner**: Development Team
