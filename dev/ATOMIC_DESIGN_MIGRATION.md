# Atomic Design Migration Guide

**Version**: 0.2.0
**Status**: Active Migration (Phase A Complete)
**Last Updated**: 2025-11-10

This guide helps you migrate from deprecated widgets to the new Atomic Design components.

---

## Overview

TOAD is transitioning to Atomic Design principles for all UI components:
- **Atoms**: Fundamental primitives (`Text`, `Block`, `Icon`)
- **Molecules**: Composite components (`MetricCard`, `ProgressBar`, `TaskItem`)
- **Organisms**: Complex compositions (`EvalPanel`, `AcceptRejectPanel`)
- **Screens**: Full layouts (`EvaluationScreen`, `ResultsScreen`, `MainScreen`)

**Benefits**:
- Consistent styling across entire TUI
- Easier to create new widgets by composing atoms
- Theme changes propagate correctly
- Better testability and maintainability

---

## Migration Status

### Phase A: Critical Duplicates (COMPLETE)
- ✅ `AcceptRejectPanel`: Widget removed, use `ui::organisms::AcceptRejectPanel`
- ✅ `ProgressBar`: Widget deprecated, use `ui::molecules::ProgressBar` or `MultiStageProgress`

### Future Phases
- Phase B-F: Migrate remaining 126 legacy widgets (see `ATOMIC_DESIGN_COMPLETION_PLAN.md`)
- Phase G: Update all tests to use atomic components
- Phase H: Final cleanup and removal of deprecated APIs

---

## Deprecated Components

### 1. ProgressBar: Widget → Molecule

**Status**: DEPRECATED (0.2.0) | Will be removed in 1.0.0

#### What Changed

The old stateful `widgets::progress::ProgressBar` is deprecated in favor of two replacements depending on your use case:

1. **For composable UIs**: Use `ui::molecules::ProgressBar`
2. **For stateful progress tracking**: Use `MultiStageProgress`

**Key Differences**:

| Aspect | Old Widget | New Molecule | MultiStageProgress |
|--------|-----------|--------------|-------------------|
| **Purpose** | Stateful animation widget | Pure composable component | Multi-stage tracker |
| **Progress Type** | `f64` (0.0-1.0) | `usize` (current/total) | Multiple stages |
| **State** | Mutable | Immutable (builder) | Mutable |
| **Rendering** | Gauge widget | Text spans (Line) | Gauge widget |
| **Composability** | No | Yes | No |
| **Use Case** | Direct rendering | Part of organism/screen | Complex workflows |

#### Migration Path A: Composable UI Components

If you're building organisms or screens that need to display progress:

**Before:**
```rust
use toad::widgets::ProgressBar;

// In your widget/organism
let mut progress = ProgressBar::new("Loading");
progress.set_progress(0.5);

// Render directly
progress.render(frame, area);
```

**After:**
```rust
use toad::ui::molecules::ProgressBar;
use ratatui::widgets::Paragraph;

// In your widget/organism
let progress = ProgressBar::new("Loading", 5, 10)  // current=5, total=10
    .width(20)
    .bar_style(Style::default().fg(ToadTheme::TOAD_GREEN));

// Compose into paragraph or other widget
let line = progress.to_line();
Paragraph::new(line).render(area, buf);
```

**Conversion Formula**:
```rust
// Old: progress = 0.75 (75%)
// New: current/total where current/total ≈ 0.75
let progress_f64 = 0.75;
let total = 100;  // or any reasonable total
let current = (progress_f64 * total as f64) as usize;
// Now: ProgressBar::new("Task", current, total) => ProgressBar::new("Task", 75, 100)
```

**Benefits**:
- Pure function (no mutable state)
- Composes with other molecules/organisms
- Consistent with Atomic Design
- Easy to test

#### Migration Path B: Stateful Progress Tracking

If you need mutable stateful progress tracking (e.g., for animations or long-running tasks):

**Before:**
```rust
use toad::widgets::ProgressBar;

let mut progress = ProgressBar::new("Download");
progress.set_progress(0.0);

// Update over time
progress.set_progress(0.25);
progress.set_message("Downloading files...");
progress.set_progress(0.50);
// etc.
```

**After (Option 1): Use MultiStageProgress**
```rust
use toad::widgets::MultiStageProgress;

let mut progress = MultiStageProgress::new(vec![
    ("Download", 100),
    ("Extract", 100),
    ("Install", 100),
]);

// Update current stage
progress.set_stage_progress(0, 50);  // Download 50%
progress.next_stage();  // Move to Extract
progress.set_stage_progress(1, 75);  // Extract 75%
```

**After (Option 2): Manage State Externally**
```rust
use toad::ui::molecules::ProgressBar;

// Store state in your app
struct MyAppState {
    progress: f64,  // 0.0 to 1.0
    total_items: usize,
}

impl MyAppState {
    fn render_progress(&self, area: Rect, buf: &mut Buffer) {
        let current = (self.progress * self.total_items as f64) as usize;
        let progress = ProgressBar::new("Processing", current, self.total_items);
        Paragraph::new(progress.to_line()).render(area, buf);
    }

    fn update_progress(&mut self, new_progress: f64) {
        self.progress = new_progress.clamp(0.0, 1.0);
    }
}
```

**Benefits**:
- Separates state management from rendering
- More flexible (can store state anywhere)
- Better for complex workflows

#### Migration Examples

##### Example 1: Simple Progress Display

**Before:**
```rust
fn render_download_progress(&self, frame: &mut Frame, area: Rect) {
    let mut progress = ProgressBar::new("Downloading");
    progress.set_progress(self.bytes_downloaded as f64 / self.total_bytes as f64);
    progress.render(frame, area);
}
```

**After:**
```rust
fn render_download_progress(&self, area: Rect, buf: &mut Buffer) {
    let progress = ProgressBar::new(
        "Downloading",
        self.bytes_downloaded,
        self.total_bytes
    );
    Paragraph::new(progress.to_line()).render(area, buf);
}
```

##### Example 2: Progress in Organism

**Before:**
```rust
pub struct EvalPanel {
    progress_widget: ProgressBar,  // Stored stateful widget
}

impl EvalPanel {
    fn update(&mut self, current: usize, total: usize) {
        self.progress_widget.set_progress(current as f64 / total as f64);
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        self.progress_widget.render(frame, area);
    }
}
```

**After:**
```rust
pub struct EvalPanel {
    current_task: usize,  // Store raw data, not widget
    total_tasks: usize,
}

impl EvalPanel {
    fn update(&mut self, current: usize, total: usize) {
        self.current_task = current;
        self.total_tasks = total;
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create molecule on-demand from data
        let progress = ProgressBar::new("Progress", self.current_task, self.total_tasks);
        Paragraph::new(progress.to_line()).render(area, buf);
    }
}
```

##### Example 3: Themed Progress

**Before:**
```rust
let mut progress = ProgressBar::new("Tasks");
progress.set_progress(1.0);
// Manual styling required
```

**After:**
```rust
let progress = ProgressBar::success("Tasks", 10, 10);
// Automatically styled with success theme (green)
// Also available: .warning() and .error()
```

#### Timeline

- **0.2.0** (Now): Old widget deprecated with warnings
- **0.3.0** (Q1 2026): Deprecation warnings in CI
- **1.0.0** (Q2 2026): Old widget removed completely

#### Testing Migration

**Old tests:**
```rust
#[test]
fn test_progress_update() {
    let mut progress = ProgressBar::new("Test");
    progress.set_progress(0.5);
    assert_eq!(progress.progress(), 0.5);
    assert!(!progress.is_complete());
}
```

**New tests:**
```rust
#[test]
fn test_progress_display() {
    let progress = ProgressBar::new("Test", 5, 10);
    assert_eq!(progress.current(), 5);
    assert_eq!(progress.total(), 10);
    assert_eq!(progress.percentage(), 50.0);

    let line = progress.to_line();
    // Test rendering output
}
```

---

### 2. AcceptRejectPanel: Widget → Organism

**Status**: REMOVED (0.2.0)

The duplicate `widgets::AcceptRejectPanel` has been removed. Use the organism implementation instead.

#### Migration

**Before:**
```rust
use toad::ui::widgets::AcceptRejectPanel;

let mut panel = AcceptRejectPanel::new();
panel.add_change("src/main.rs", "Add feature");
panel.accept_current();
```

**After:**
```rust
use toad::ui::organisms::AcceptRejectPanel;
use toad::ui::organisms::accept_reject_panel::{ChangeStatus, ChangeState};

let panel = AcceptRejectPanel::new()
    .title("Code Review")
    .add_change(ChangeStatus {
        description: "src/main.rs".to_string(),
        state: ChangeState::Accepted,
        details: Some("+42 -10 lines".to_string()),
    });

// Render to buffer
panel.render(area, buf);
```

**Key Differences**:
- Organism uses builder pattern (immutable)
- Organism composes atomic molecules (MetricCard, ProgressBar, TaskItem)
- Organism has better theming support
- Organism is more testable

**Timeline**:
- **0.2.0** (Now): Widget removed, organism is canonical

---

## Best Practices

### 1. Store Data, Not Widgets

**Anti-pattern:**
```rust
struct MyScreen {
    progress_widget: ProgressBar,  // DON'T store widgets
}
```

**Best practice:**
```rust
struct MyScreen {
    current_task: usize,  // Store raw data
    total_tasks: usize,
}

impl MyScreen {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        // Create widget from data when rendering
        let progress = ProgressBar::new("Tasks", self.current_task, self.total_tasks);
        Paragraph::new(progress.to_line()).render(area, buf);
    }
}
```

### 2. Compose Molecules in Organisms

**Anti-pattern:**
```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    // Don't use raw Ratatui widgets
    let gauge = Gauge::default()
        .percent(50)
        .label("Progress");
    gauge.render(area, buf);
}
```

**Best practice:**
```rust
fn render(&self, area: Rect, buf: &mut Buffer) {
    // Use atomic molecules
    let progress = ProgressBar::new("Progress", 5, 10);
    Paragraph::new(progress.to_line()).render(area, buf);
}
```

### 3. Use Theme Constants

**Anti-pattern:**
```rust
.style(Style::default().fg(Color::Green))  // Don't use raw colors
```

**Best practice:**
```rust
use crate::ui::theme::ToadTheme;

.style(Style::default().fg(ToadTheme::TOAD_GREEN))  // Use theme constants
```

---

## Getting Help

- **Documentation**: `cargo doc --open --package toad`
- **Examples**: See `src/ui/organisms/` for organism examples
- **Migration Template**: See `WIDGET_MIGRATION_TEMPLATE.md`
- **Full Plan**: See `ATOMIC_DESIGN_COMPLETION_PLAN.md`
- **Issues**: Report problems at https://github.com/trickstertwo/toad/issues

---

## FAQs

### Q: Why are there two ProgressBar implementations?

**A**: The old widget is a stateful animation widget with Gauge rendering. The new molecule is a pure composable component for organisms/screens. They serve different purposes. Use the molecule for new code.

### Q: Will my old code break immediately?

**A**: No. Deprecated components still work but emit warnings. You have until v1.0.0 to migrate.

### Q: How do I silence deprecation warnings temporarily?

**A**: Add `#[allow(deprecated)]` above the usage:
```rust
#[allow(deprecated)]
use toad::widgets::ProgressBar;

#[allow(deprecated)]
let progress = ProgressBar::new("Loading");
```

### Q: What if I need features from the old widget that aren't in the molecule?

**A**: For stateful tracking, use `MultiStageProgress`. For other features, file an issue and we'll consider adding them to the molecule.

### Q: Can I use both old and new implementations?

**A**: Yes during the transition period, but avoid it if possible. Pick one approach per module to avoid confusion.

---

**Last Updated**: 2025-11-10
**Next Review**: Phase B completion (Q1 2026)
