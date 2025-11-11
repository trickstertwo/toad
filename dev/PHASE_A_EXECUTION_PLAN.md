# Phase A: Resolve Critical Duplicates - Detailed Execution Plan

**Priority**: CRITICAL - MUST BE COMPLETED FIRST
**Estimated Time**: 1-2 weeks
**Status**: READY TO START

---

## Overview

Phase A resolves the two critical duplicate implementations that are causing confusion and maintenance burden:
1. **ProgressBar**: Molecule vs Widget (2 implementations)
2. **AcceptRejectPanel**: Organism vs Widget (2 implementations)

---

## 🔍 Task A1: Audit & Consolidate ProgressBar

### A1.1: Audit Current Usage

**Command**: Find all usages of old ProgressBar widget
```bash
# Find direct imports
grep -rn "use.*widgets::progress::ProgressBar" src/ tests/
grep -rn "widgets::ProgressBar::new" src/ tests/

# Find test usages
grep -rn "ProgressBar::new" tests/ | grep -v "molecules::"

# Count occurrences
echo "Old widget usages:"
grep -r "widgets::progress::ProgressBar" src/ tests/ | wc -l

echo "New molecule usages:"
grep -r "molecules::ProgressBar" src/ tests/ | wc -l
```

**Expected Files**:
- `tests/ui_ux_widget_integration_tests.rs` - Lines 314, 322, 339, 349, 361, 499, 567
- `tests/animation_mode_workspace_integration_tests.rs` - Multiple lines
- `src/ui/widgets/progress/progress/state.rs` - Implementation
- `src/ui/widgets/progress/progress/tests.rs` - Tests
- `src/ui/widgets/progress/mod.rs` - Re-export

### A1.2: API Comparison

**Old Widget API** (`src/ui/widgets/progress/progress/state.rs`):
```rust
pub struct ProgressBar {
    title: String,
    progress: f64,  // 0.0 to 1.0
    message: Option<String>,
}

// Usage:
let mut progress = ProgressBar::new("Loading");
progress.set_progress(0.5);  // Mutable state
progress.set_message("Processing...");
// Renders with Gauge widget
```

**New Molecule API** (`src/ui/molecules/progress_bar.rs`):
```rust
pub struct ProgressBar {
    label: String,
    current: usize,
    total: usize,
    width: u16,
    // ... styles
}

// Usage:
let progress = ProgressBar::new("Loading", 5, 10);  // Immutable
progress.width(30);  // Builder pattern
// Returns Line (composable)
```

**Key Differences**:
| Aspect | Old Widget | New Molecule |
|--------|-----------|--------------|
| Progress representation | `f64` (0.0-1.0) | `usize` (current/total) |
| State | Mutable | Immutable (builder) |
| Rendering | Gauge widget | Text spans (Line) |
| Output | Widget trait | `to_line()` / `to_spans()` |
| Composability | No | Yes (pure function) |

### A1.3: Migration Strategy Decision

**DECISION**: Keep new molecule, deprecate old widget

**Rationale**:
1. ✅ Molecule follows Atomic Design principles
2. ✅ Molecule is pure and composable
3. ✅ Molecule already used in organisms (EvalPanel, AcceptRejectPanel)
4. ✅ Molecule easier to test and reason about
5. ❌ Old widget has Gauge rendering (can be replicated if needed)
6. ❌ Old widget has animation support (can be added to molecule later)

**Migration Path**:
- All current usages of old widget will be updated to use molecule
- Old widget will be deprecated with migration guide
- Old widget removed in v1.0.0 (major version)

### A1.4: Create Adapter (If Needed)

If some code relies on mutable state pattern, create adapter:

**File**: `src/ui/molecules/progress_bar_adapter.rs`
```rust
/// Adapter for old ProgressBar API using new molecule
///
/// Provides mutable state interface over immutable molecule.
/// Use for gradual migration from old widget.
#[deprecated(
    since = "0.2.0",
    note = "Use `ProgressBar` molecule directly with builder pattern"
)]
pub struct ProgressBarAdapter {
    title: String,
    progress: f64,
    total: usize,
}

impl ProgressBarAdapter {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            progress: 0.0,
            total: 100,  // Default
        }
    }

    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn to_molecule(&self) -> ProgressBar {
        let current = (self.progress * self.total as f64) as usize;
        ProgressBar::new(&self.title, current, self.total)
    }
}
```

### A1.5: Update All Usages

**File**: `tests/ui_ux_widget_integration_tests.rs`

**OLD** (Line 314):
```rust
use toad::widgets::ProgressBar;

let progress = ProgressBar::new("Loading");
assert_eq!(progress.progress(), 0.0);
```

**NEW**:
```rust
use toad::ui::molecules::ProgressBar;

let progress = ProgressBar::new("Loading", 0, 100);
assert_eq!(progress.current(), 0);
assert_eq!(progress.percentage(), 0.0);
```

**Migration Commands**:
```bash
# Update imports in tests
sed -i 's/use toad::widgets::ProgressBar/use toad::ui::molecules::ProgressBar/g' \
    tests/ui_ux_widget_integration_tests.rs

sed -i 's/use toad::widgets::ProgressBar/use toad::ui::molecules::ProgressBar/g' \
    tests/animation_mode_workspace_integration_tests.rs

# Manual review needed for API changes
# Cannot automate: new("title") → new("title", current, total)
```

**Manual Migration Checklist**:
- [ ] `tests/ui_ux_widget_integration_tests.rs` - 7 usages
- [ ] `tests/animation_mode_workspace_integration_tests.rs` - 14 usages
- [ ] Any src/ usages (verify none exist)

### A1.6: Deprecate Old Widget

**File**: `src/ui/widgets/progress/progress/state.rs`

Add deprecation attribute:
```rust
/// Progress bar widget for single tasks
///
/// # Deprecated
///
/// This widget is deprecated. Use the atomic `ProgressBar` molecule instead:
///
/// ```rust
/// // OLD
/// use toad::widgets::ProgressBar;
/// let mut progress = ProgressBar::new("Loading");
/// progress.set_progress(0.5);
///
/// // NEW
/// use toad::ui::molecules::ProgressBar;
/// let progress = ProgressBar::new("Loading", 50, 100);
/// ```
///
/// See migration guide: `ATOMIC_DESIGN_MIGRATION.md`
#[deprecated(
    since = "0.2.0",
    note = "Use `crate::ui::molecules::ProgressBar` instead. \
            See migration guide: ATOMIC_DESIGN_MIGRATION.md"
)]
pub struct ProgressBar {
    // ... existing fields
}
```

**File**: `src/ui/widgets/progress/mod.rs`

Add deprecation to re-export:
```rust
//! Progress indicator widgets
//!
//! # Deprecated
//!
//! The ProgressBar in this module is deprecated.
//! Use `crate::ui::molecules::ProgressBar` instead.

#[deprecated(since = "0.2.0", note = "Use crate::ui::molecules::ProgressBar")]
pub mod progress;

pub mod spinner;
pub mod token_counter;

// Deprecated re-export
#[allow(deprecated)]
pub use progress::*;

pub use spinner::*;
pub use token_counter::*;
```

### A1.7: Add Migration Guide Entry

**File**: `ATOMIC_DESIGN_MIGRATION.md` (create if doesn't exist)

```markdown
# Atomic Design Migration Guide

## ProgressBar: Widget → Molecule

### Overview
The old `widgets::progress::ProgressBar` has been replaced with the atomic
`molecules::ProgressBar` following Atomic Design principles.

### What Changed
- **Location**: `ui::widgets::progress::ProgressBar` → `ui::molecules::ProgressBar`
- **Progress Type**: `f64` (0.0-1.0) → `usize` (current/total)
- **State**: Mutable → Immutable (builder pattern)
- **Rendering**: Gauge widget → Text spans (Line)

### Migration Examples

#### Basic Usage
**Before:**
```rust
use toad::widgets::ProgressBar;

let mut progress = ProgressBar::new("Loading");
progress.set_progress(0.5);
```

**After:**
```rust
use toad::ui::molecules::ProgressBar;

let progress = ProgressBar::new("Loading", 50, 100);
// Immutable - create new instance for updates
```

#### With Custom Width
**Before:**
```rust
let mut progress = ProgressBar::new("Download");
progress.set_progress(0.75);
// Width was fixed
```

**After:**
```rust
let progress = ProgressBar::new("Download", 75, 100)
    .width(30);  // Builder pattern
```

#### Themed Progress
**Before:**
```rust
let mut progress = ProgressBar::new("Tasks");
// Manual styling required
```

**After:**
```rust
let progress = ProgressBar::success("Tasks", 10, 10);
// Or: .warning() / .error()
```

### Timeline
- **0.2.0**: Old widget deprecated
- **0.3.0**: Deprecation warnings
- **1.0.0**: Old widget removed

### See Also
- API Docs: `cargo doc --open --package toad`
- Example: `examples/progress_bar_migration.rs`
```

### A1.8: Verification

**Checklist**:
- [ ] All test imports updated
- [ ] All test API calls updated
- [ ] Tests pass: `cargo test --all-features`
- [ ] Deprecation warnings appear: `cargo build`
- [ ] Documentation updated: `ATOMIC_DESIGN_MIGRATION.md`
- [ ] No new usages of old widget possible

**Commands**:
```bash
# Verify no non-deprecated usages
grep -r "use.*widgets::progress::ProgressBar" src/ \
    | grep -v "#\[deprecated\]" \
    | grep -v "Old widget"

# Should return only deprecated definitions, not usages

# Run tests
cargo test --all-features

# Check for deprecation warnings
cargo build 2>&1 | grep "deprecated"

# Should see warnings for old widget
```

**Acceptance Criteria**:
- ✅ Zero non-test usages of old ProgressBar widget
- ✅ All tests use new molecule API
- ✅ All tests pass (100%)
- ✅ Deprecation warnings present
- ✅ Migration guide complete

---

## 🔍 Task A2: Audit & Consolidate AcceptRejectPanel

### A2.1: Audit Current Usage

**Command**: Find all usages
```bash
# Find old widget usages
grep -rn "use.*widgets::accept_reject_panel" src/ tests/
grep -rn "use.*widgets::AcceptRejectPanel" src/ tests/

# Find new organism usages
grep -rn "use.*organisms::AcceptRejectPanel" src/ tests/
grep -rn "use.*organisms::accept_reject_panel" src/ tests/

# Count
echo "Old widget usages:"
grep -r "widgets::AcceptRejectPanel\|widgets::accept_reject_panel" src/ | wc -l

echo "New organism usages:"
grep -r "organisms::AcceptRejectPanel\|organisms::accept_reject_panel" src/ | wc -l
```

**Expected Files**:
- `src/ui/widgets/accept_reject_panel.rs` - Old widget (15,477 bytes)
- `src/ui/organisms/accept_reject_panel.rs` - New organism (16,000+ bytes)
- `src/ui/screens/` - Possibly used in screens

### A2.2: API Comparison

**Old Widget** (`src/ui/widgets/accept_reject_panel.rs`):
```rust
pub struct AcceptRejectPanel {
    // Check implementation
    // Likely has mutable state
}

impl AcceptRejectPanel {
    pub fn new() -> Self { ... }
    pub fn add_change(&mut self, ...) { ... }  // Mutable
    pub fn accept_current(&mut self) { ... }
    pub fn reject_current(&mut self) { ... }
}
```

**New Organism** (`src/ui/organisms/accept_reject_panel.rs`):
```rust
pub struct AcceptRejectPanel {
    pub total_changes: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub pending: usize,
    pub changes: Vec<ChangeStatus>,
    pub title: String,
}

impl AcceptRejectPanel {
    pub fn new() -> Self { ... }
    pub fn add_change(mut self, ...) -> Self { ... }  // Builder
    pub fn changes(mut self, ...) -> Self { ... }
    pub fn render(&self, ...) { ... }  // Pure rendering
}
```

### A2.3: Feature Comparison

**Task**: Read both files and compare features

```bash
# Compare line counts
wc -l src/ui/widgets/accept_reject_panel.rs src/ui/organisms/accept_reject_panel.rs

# Check for unique features in old widget
grep -n "pub fn" src/ui/widgets/accept_reject_panel.rs > /tmp/old_methods.txt
grep -n "pub fn" src/ui/organisms/accept_reject_panel.rs > /tmp/new_methods.txt
diff /tmp/old_methods.txt /tmp/new_methods.txt
```

**Required**: Manual review of both implementations to identify:
- [ ] Features only in old widget
- [ ] Features only in new organism
- [ ] Behavioral differences

### A2.4: Migration Decision

**DECISION**: Keep new organism, remove old widget

**Rationale**:
1. ✅ Organism follows Atomic Design (composes MetricCard, TaskItem, ProgressBar)
2. ✅ Organism has better test coverage (640+ lines of tests)
3. ✅ Organism is immutable/builder pattern (easier to reason about)
4. ✅ Organism already integrated (used in screens via organisms module)
5. ❌ Old widget may have keyboard handling (can be preserved if critical)

**If old widget has unique features**:
- Port critical features to organism
- Document why features were removed (if any)
- Provide migration path for lost features

### A2.5: Check for Usages

**Command**: Verify usage
```bash
# Check if old widget is actually used anywhere
grep -r "widgets::accept_reject_panel::AcceptRejectPanel" src/ \
    --include="*.rs" \
    | grep -v "^src/ui/widgets/accept_reject_panel.rs"

# Check for widget module imports
grep -r "mod accept_reject_panel" src/ui/widgets/ \
    | grep -v "^src/ui/widgets/accept_reject_panel.rs"

# Check src/ui/widgets.rs or mod.rs for re-exports
grep "accept_reject_panel" src/ui/widgets.rs 2>/dev/null || \
grep "accept_reject_panel" src/ui/widgets/mod.rs 2>/dev/null
```

**Expected Result**: Old widget should NOT be used anywhere (if it is, migrate those usages first)

### A2.6: Remove Old Widget

**If not used anywhere** (most likely case):

1. **Delete file**:
```bash
git rm src/ui/widgets/accept_reject_panel.rs
```

2. **Update module exports** (if present):
   - Check `src/ui/widgets.rs` or `src/ui/widgets/mod.rs`
   - Remove any `pub mod accept_reject_panel;` line
   - Remove any `pub use accept_reject_panel::*;` line

3. **Verify compilation**:
```bash
cargo check --all-targets
cargo test --all-features
```

4. **Commit**:
```bash
git add -A
git commit -m "Remove duplicate AcceptRejectPanel widget

The AcceptRejectPanel widget was a duplicate of the organism implementation.
The organism follows Atomic Design principles and is already integrated.

- Removed: src/ui/widgets/accept_reject_panel.rs (15,477 bytes)
- Kept: src/ui/organisms/accept_reject_panel.rs (atomic organism)

All functionality preserved in organism.
No breaking changes (widget was not exported publicly).

Related: ATOMIC_DESIGN_COMPLETION_PLAN.md Phase A, Task A2"
```

### A2.7: Update Documentation

**If organism needs any documentation updates**:

**File**: `src/ui/organisms/accept_reject_panel.rs`

Ensure rustdoc mentions this is the canonical implementation:
```rust
//! AcceptRejectPanel organism - Code approval interface
//!
//! **Note**: This is the canonical implementation of AcceptRejectPanel.
//! Previous widget implementation has been removed in favor of this
//! atomic organism that properly composes molecules.
//!
//! # Architecture
//! ...
```

### A2.8: Verification

**Checklist**:
- [ ] Old widget file deleted
- [ ] Module exports updated (if needed)
- [ ] Compilation succeeds: `cargo check`
- [ ] All tests pass: `cargo test`
- [ ] No references to old widget path
- [ ] Organism documentation updated

**Commands**:
```bash
# Verify file is gone
ls src/ui/widgets/accept_reject_panel.rs
# Should not exist

# Verify no imports of old widget
grep -r "widgets::accept_reject_panel" src/ tests/
# Should return empty

# Verify organism is used
grep -r "organisms::accept_reject_panel" src/
# Should show usages

# Build and test
cargo build --all-targets
cargo test --all-features
```

**Acceptance Criteria**:
- ✅ Old widget file removed from codebase
- ✅ Zero references to `widgets::accept_reject_panel`
- ✅ Organism properly exported from `ui::organisms`
- ✅ All tests pass (100%)
- ✅ Documentation accurate

---

## 📝 Phase A Summary

### Deliverables
1. ✅ ProgressBar: Old widget deprecated, all usages migrated to molecule
2. ✅ AcceptRejectPanel: Old widget removed, organism is canonical
3. ✅ Migration guide created: `ATOMIC_DESIGN_MIGRATION.md`
4. ✅ All tests passing
5. ✅ Zero compilation errors
6. ✅ Deprecation warnings in place

### Metrics
- **Files Changed**: ~15 files
- **Lines Added**: ~200 (migration guide, deprecation docs)
- **Lines Removed**: ~400 (duplicate widget, old test code)
- **Net Change**: -200 lines (code reduction = good!)
- **Tests**: 100% passing
- **Duplicates Resolved**: 2 → 0

### Next Steps
After Phase A completion:
1. Update CHANGELOG.md with Phase A changes
2. Create GitHub milestone for Phase B
3. Begin Phase B: Migrate Top 20 High-Impact Widgets
4. Communicate deprecations to team/users

---

## 🚀 Quick Start Commands

### Start Phase A Today
```bash
# 1. Create feature branch
git checkout -b atomic-design-phase-a

# 2. Audit current state
bash ./scripts/audit_duplicates.sh  # Create this script with commands above

# 3. Start with AcceptRejectPanel (simpler - just remove)
git rm src/ui/widgets/accept_reject_panel.rs
# Update module exports if needed
cargo test

# 4. Then tackle ProgressBar (more complex - deprecate + migrate)
# Follow A1.5 - A1.7 above

# 5. Create migration guide
touch ATOMIC_DESIGN_MIGRATION.md
# Fill in content from A1.7

# 6. Verify everything
cargo test --all-features
cargo clippy --all-targets -- -D warnings

# 7. Commit and push
git add -A
git commit -m "Phase A: Resolve ProgressBar and AcceptRejectPanel duplicates"
git push origin atomic-design-phase-a

# 8. Create PR
gh pr create --title "Phase A: Resolve Critical Duplicates" \
    --body "Resolves #ISSUE_NUMBER. See ATOMIC_DESIGN_COMPLETION_PLAN.md Phase A"
```

---

**Phase Owner**: Development Team
**Status**: READY TO START
**Estimated Completion**: 1-2 weeks
**Last Updated**: 2025-11-10
