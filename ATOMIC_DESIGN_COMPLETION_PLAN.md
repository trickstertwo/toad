# Atomic Design Refactoring Completion Plan

**Status**: Phase 1-8 created foundation (11% complete), Phases A-H will complete the refactoring (89% remaining)

**Goal**: Migrate all 126 legacy widgets to use Atomic Design components, eliminate duplicates, update tests

---

## 📊 Current State Assessment

- ✅ **Atoms**: 3 complete (Text, Block, Icon)
- ✅ **Molecules**: 12 complete (MetricCard, ProgressBar, TaskItem, etc.)
- ✅ **Organisms**: 2 complete (EvalPanel, AcceptRejectPanel)
- ❌ **Legacy Widgets**: 126 not refactored
- ❌ **Duplicates**: 2 critical (ProgressBar, AcceptRejectPanel)
- ❌ **Raw Ratatui Usage**: 335 Span::styled/raw, 70 Block::default()
- ❌ **Tests**: Mostly use legacy widgets

---

## 🎯 Phase A: Resolve Critical Duplicates

**Priority**: CRITICAL - Must be done first to prevent confusion

### Task A1: Consolidate ProgressBar
**Current State:**
- NEW: `src/ui/molecules/progress_bar.rs` (atomic, takes current/total, returns Line)
- OLD: `src/ui/widgets/progress/progress/state.rs` (stateful, uses 0.0-1.0, Gauge widget)

**Decision Required**: Which implementation to keep?
- **Option 1**: Keep molecule, deprecate widget (RECOMMENDED)
  - Molecule is pure, composable, follows Atomic Design
  - Stateless = easier to test and reason about
  - Already used in organisms
- **Option 2**: Keep widget, remove molecule
  - Widget has more features (animation, Gauge rendering)
  - BUT: Not composable, breaks Atomic Design

**Action Items:**
1. ✅ Audit all usages of old ProgressBar widget
   - Find: `grep -r "use.*widgets::progress::ProgressBar" src/`
   - Find: `grep -r "widgets::ProgressBar" src/`
2. ✅ Create migration molecule for advanced features if needed
   - If old widget has features molecule lacks, extract to new molecule
3. ✅ Update all imports to use molecule
4. ✅ Deprecate old widget with `#[deprecated]` attribute
5. ✅ Add migration guide in rustdoc
6. ✅ Schedule removal for next major version

**Acceptance Criteria:**
- Zero imports of old ProgressBar widget
- All tests pass with molecule version
- Rustdoc shows deprecation warning
- Performance is equivalent or better

---

### Task A2: Consolidate AcceptRejectPanel
**Current State:**
- NEW: `src/ui/organisms/accept_reject_panel.rs` (atomic organism)
- OLD: `src/ui/widgets/accept_reject_panel.rs` (legacy widget)

**Action Items:**
1. ✅ Compare APIs of both implementations
2. ✅ Identify unique features in old widget
3. ✅ Port missing features to organism if critical
4. ✅ Update all imports to use organism
5. ✅ Delete old widget file
6. ✅ Update module exports

**Acceptance Criteria:**
- Only one AcceptRejectPanel exists in codebase
- All functionality preserved
- All tests pass
- Zero references to old widget path

---

## 🎯 Phase B: Migrate High-Impact Widgets (Top 20)

**Priority**: HIGH - Maximum ROI, most-used widgets

**Strategy**: Find most-used widgets by grep count, migrate to use atoms/molecules

### Task B1: Audit Widget Usage Frequency
```bash
# Find usage counts for all widgets
for widget in $(find src/ui/widgets -name "*.rs" -type f); do
  count=$(grep -r "$(basename $widget .rs)" src/ | wc -l)
  echo "$count $widget"
done | sort -rn | head -20
```

**Top 20 Candidates** (need verification):
1. Dialog (core/dialog.rs) - PARTIALLY DONE in Phase 1
2. Statusline (core/statusline/) - PARTIALLY DONE in Phase 1
3. Input (input/input.rs)
4. TextArea (input/textarea/)
5. Table (core/table/)
6. Scrollbar (core/scrollbar/)
7. ModeIndicator (mode_indicator.rs) - DONE in Phase 1
8. ChatPanel (chat_panel/)
9. FileTree (filetree.rs)
10. GitStatusPanel (git/git_status_panel.rs)
11. Minimap (layout/minimap.rs) - DONE in Phase 8
12. MemoryMonitor (memory.rs) - DONE in Phase 8
13. Spinner (progress/spinner.rs)
14. Toast (notifications/toast/)
15. CommandPalette (input/command_palette.rs)
16. ContextMenu (selection/context_menu.rs)
17. Collapsible (collapsible.rs)
18. ConflictResolver (conflict_resolver.rs)
19. VimMode (input/vim_mode/)
20. TabBar (layout/tabbar.rs) - DONE in Phase 8

### Task B2: Refactor Each Widget (Template)
For each widget in top 20:

1. **Analyze Current Implementation**
   - Identify all `Span::styled()` → replace with `Text::new().style()`
   - Identify all `Block::default()` → replace with `Block::new()`
   - Check if widget should be molecule or keep as complex widget

2. **Refactor to Use Atoms**
   - Replace all text rendering with Text atoms
   - Replace all block rendering with Block atoms
   - Replace all icon rendering with Icon atoms

3. **Maintain Backward Compatibility**
   - Keep same public API
   - Keep same behavior
   - Keep same tests passing

4. **Add Rustdoc Examples**
   - Show how to use with atomic components
   - Document migration path from old usage

5. **Update Module Imports**
   ```rust
   use crate::ui::atoms::{Text, Block, Icon};
   ```

**Acceptance Criteria per Widget:**
- Zero `Span::styled()` or `Span::raw()` in implementation
- Zero `Block::default()` in implementation
- All existing tests pass unchanged
- New tests added for atomic composition
- Rustdoc complete with examples

---

## 🎯 Phase C: Migrate Core UI Widgets

**Priority**: MEDIUM-HIGH - Foundation widgets used by many others

**Widgets to Migrate** (34 files in `src/ui/widgets/core/`):
- animation.rs
- borders.rs
- breadcrumbs.rs
- cheat_sheet.rs - DONE (refactored in Phase 1-8)
- context_display.rs
- dialog.rs - PARTIALLY DONE
- help.rs
- icons.rs
- preview/ (mod.rs, state.rs, tests.rs) - DONE in Phase 8
- scrollbar/ (mod.rs, state.rs, tests.rs)
- statusline/ (mod.rs, state.rs, tests.rs) - PARTIALLY DONE
- table/ (mod.rs, state.rs)
- vector_canvas/ (mod.rs, state.rs, tests.rs)
- welcome_screen.rs

**Strategy**: Bottom-up (simplest first, dependencies last)

**Milestone**: All core widgets use atoms for text/block/icon rendering

---

## 🎯 Phase D: Migrate Input Widgets

**Priority**: MEDIUM - Complex state management, careful refactoring needed

**Widgets to Migrate** (25 files in `src/ui/widgets/input/`):
- command_palette.rs
- input.rs - HIGH PRIORITY (used everywhere)
- input_dialog/ (mod.rs, state.rs, tests.rs)
- input_prompt/ (mod.rs, state.rs, tests.rs)
- palette/ (mod.rs, state.rs, tests.rs)
- textarea/ (mod.rs, state.rs, tests.rs)
- vim_mode/ (mod.rs, state.rs, tests.rs)

**Special Considerations**:
- These widgets have complex state (cursor position, selection)
- Must preserve all keyboard handling
- Must maintain performance (rendering on every keystroke)
- Tests are critical (input handling is tricky)

**Strategy**:
1. Extract rendering logic to use atoms (view layer)
2. Keep state management unchanged (model layer)
3. Ensure separation of concerns (Elm Architecture)

---

## 🎯 Phase E: Migrate Layout & Selection Widgets

**Priority**: MEDIUM - Many dependencies on these

**Layout Widgets** (11 files in `src/ui/widgets/layout/`):
- floating/ (mod.rs, state.rs, tests.rs)
- minimap.rs - DONE in Phase 8
- panel.rs - DONE (uses atoms)
- split/ (mod.rs, state.rs, tests.rs)
- tabbar.rs - DONE in Phase 8
- window_switcher.rs

**Selection Widgets** (9 files in `src/ui/widgets/selection/`):
- context_menu.rs
- model_selector/ (mod.rs, state.rs, tests.rs)
- multiselect/ (mod.rs, state.rs, tests.rs)
- quick_actions_panel.rs

**Strategy**: These are often containers, ensure they render children correctly with atomic styles

---

## 🎯 Phase F: Migrate Specialized Widgets

**Priority**: MEDIUM-LOW - Domain-specific, lower usage

**Git Widgets** (8 files in `src/ui/widgets/git/`):
- git_branch_manager.rs
- git_commit_dialog.rs
- git_diff_viewer/ (mod.rs, state.rs, tests.rs)
- git_graph/ (mod.rs, state.rs, tests.rs)
- git_stage_ui.rs
- git_status_panel.rs

**Chart Widgets** (12 files in `src/ui/widgets/charts/`):
- bar_chart.rs
- chart/ (mod.rs, state.rs, tests.rs)
- line_chart/ (mod.rs, state.rs, tests.rs)
- live_graph.rs
- scatter_plot/ (mod.rs, state.rs, tests.rs)
- sparkline.rs

**Notification Widgets** (7 files in `src/ui/widgets/notifications/`):
- modal.rs
- startup_tips.rs
- toast/ (mod.rs, render.rs, state.rs, tests.rs)
- tutorial.rs

**Other Specialized** (remaining 21 files):
- ai_diff_view.rs
- card_preview.rs
- collapsible.rs
- conflict_resolver.rs
- contextual_help.rs
- conversation/ (mod.rs, view.rs)
- event_metrics.rs
- file_preview_manager.rs
- filetree.rs
- fps.rs
- memory.rs - DONE in Phase 8
- mode_indicator.rs - DONE in Phase 1
- render_profiler.rs
- session_manager/ (mod.rs, state.rs, tests.rs)
- suggestions_widget.rs
- undo_redo.rs
- vim_macros.rs
- workspace.rs

**Strategy**: Migrate in order of usage frequency within each category

---

## 🎯 Phase G: Update All Tests

**Priority**: HIGH - Ensures migrations don't break functionality

### Task G1: Audit Test Suite
```bash
# Find all test files using legacy widgets
grep -r "Span::styled\|Span::raw" tests/ --include="*.rs"
grep -r "Block::default" tests/ --include="*.rs"
grep -r "use.*widgets::" tests/ --include="*.rs"
```

### Task G2: Migrate Test Files
**Files to Update**:
- `tests/ui_ux_widget_integration_tests.rs` - Uses old ProgressBar (335 lines)
- `tests/animation_mode_workspace_integration_tests.rs` - Uses old ProgressBar
- All widget-specific test files in `src/ui/widgets/*/tests.rs`

**Migration Pattern**:
```rust
// OLD
use toad::widgets::ProgressBar;
let progress = ProgressBar::new("Test").with_progress(0.5);

// NEW
use toad::ui::molecules::ProgressBar;
let progress = ProgressBar::new("Test", 5, 10); // current/total
```

### Task G3: Add Integration Tests for Atomic Components
- Test that molecules compose atoms correctly
- Test that organisms compose molecules correctly
- Test that screens compose organisms correctly
- Verify rendering output matches expected

**Acceptance Criteria**:
- All tests pass
- Zero tests use legacy widget APIs
- Coverage maintained or improved

---

## 🎯 Phase H: Final Cleanup & Deprecation

**Priority**: CRITICAL - Must be done last

### Task H1: Mark All Legacy APIs as Deprecated
```rust
#[deprecated(
    since = "0.2.0",
    note = "Use `crate::ui::molecules::ProgressBar` instead. \
            See migration guide: https://docs.toad.dev/migration/progress-bar"
)]
pub struct ProgressBar { ... }
```

### Task H2: Add Compiler Warnings
- Enable `#![warn(deprecated)]` in relevant modules
- Ensure CI fails if new code uses deprecated APIs

### Task H3: Create Migration Guide
**Document**: `docs/ATOMIC_DESIGN_MIGRATION.md`
- List all deprecated widgets
- Show old → new API mappings
- Provide code examples for each
- Link from CHANGELOG.md

### Task H4: Update CHANGELOG.md
```markdown
## [0.2.0] - 2025-MM-DD

### Changed (BREAKING)
- **UI Architecture**: Migrated entire TUI to Atomic Design principles
  - All widgets now compose Text, Block, Icon atoms
  - 126 legacy widgets refactored to use atomic components
  - See migration guide: docs/ATOMIC_DESIGN_MIGRATION.md

### Deprecated
- `ui::widgets::progress::ProgressBar` → use `ui::molecules::ProgressBar`
- `ui::widgets::accept_reject_panel::AcceptRejectPanel` → use `ui::organisms::AcceptRejectPanel`
- [Full list in migration guide]

### Removed
- [If any APIs removed immediately]
```

### Task H5: Schedule Removal (Major Version)
- Create GitHub milestone for v1.0.0
- Create issues for removing each deprecated widget
- Set target date (e.g., 3 months from deprecation)

### Task H6: Final Verification
```bash
# Ensure NO raw Ratatui usage in widgets
grep -r "Span::styled\|Span::raw" src/ui/widgets/ --include="*.rs" | wc -l  # Should be 0
grep -r "Block::default" src/ui/widgets/ --include="*.rs" | wc -l  # Should be 0

# Ensure all widgets use atoms
grep -r "use crate::ui::atoms::" src/ui/widgets/ --include="*.rs" | wc -l  # Should be ~126

# Verify test coverage
cargo test --all-features
cargo tarpaulin --workspace --exclude-files 'tests/*' --out Html

# Verify no circular dependencies
cargo tree | grep -i "cycle"  # Should be empty
```

**Acceptance Criteria**:
- Zero raw Span/Block usage in widgets
- All 126 widgets use atomic components
- All tests pass
- Deprecation warnings in place
- Migration guide complete
- CHANGELOG updated
- CI enforces atomic design

---

## 📈 Success Metrics

**Quantitative**:
- ✅ 100% of widgets use atoms for text/block/icon (currently 11%)
- ✅ 0 raw `Span::styled/Span::raw` in widgets (currently 335)
- ✅ 0 `Block::default()` in widgets (currently 70)
- ✅ 0 duplicate implementations (currently 2)
- ✅ Test coverage ≥ 85% (current unknown)

**Qualitative**:
- ✅ Consistent visual appearance across TUI
- ✅ Easy to create new widgets by composing atoms
- ✅ Theme changes propagate correctly
- ✅ New contributors understand architecture
- ✅ Maintenance burden reduced

---

## 🚀 Execution Strategy

### Incremental Rollout
**Week 1-2**: Phase A (Critical duplicates)
**Week 3-4**: Phase B (High-impact widgets, 20 widgets)
**Week 5-6**: Phase C (Core UI widgets, 34 widgets)
**Week 7-8**: Phase D (Input widgets, 25 widgets)
**Week 9-10**: Phase E (Layout & Selection, 20 widgets)
**Week 11-12**: Phase F (Specialized widgets, 48 widgets)
**Week 13**: Phase G (Test migrations)
**Week 14**: Phase H (Final cleanup)

**Total**: ~14 weeks for complete migration (126 widgets + tests + cleanup)

### Parallelization
- Multiple developers can work on different widget categories simultaneously
- Each widget is independent (minimize merge conflicts)
- Tests updated alongside each widget

### Risk Mitigation
- Maintain backward compatibility during migration
- Deprecate before removing (give users time to migrate)
- Feature flags for gradual rollout if needed
- Comprehensive testing at each phase

### Automation Opportunities
- Script to find all `Span::styled` → suggest `Text::new().style()` replacements
- Linter rule to forbid raw Ratatui in widget code
- CI check to enforce atomic design patterns

---

## 📋 Checklist per Widget

For each of 126 widgets:

- [ ] **Audit**: Identify all Span/Block/Icon usage
- [ ] **Refactor**: Replace with atoms (Text/Block/Icon)
- [ ] **Test**: Ensure all tests pass
- [ ] **Document**: Add rustdoc with atomic examples
- [ ] **Import**: Update to use `crate::ui::atoms::{...}`
- [ ] **Verify**: No raw Ratatui in implementation
- [ ] **Commit**: Single commit per widget with tests

---

## 🎯 Next Actions

1. **Start with Phase A**: Resolve ProgressBar and AcceptRejectPanel duplicates
2. **Create tracking issues**: One GitHub issue per phase
3. **Set up automation**: Linter rules, CI checks
4. **Begin Phase B**: Migrate top 20 high-impact widgets
5. **Iterate**: One widget at a time, verify tests, commit

---

## 📚 Resources

- **Atomic Design Reference**: https://atomicdesign.bradfrost.com/
- **Current Implementation**: `src/ui/atoms/`, `src/ui/molecules/`, `src/ui/organisms/`
- **Migration Template**: (Create template file for copy-paste refactoring)
- **CI Integration**: `.github/workflows/atomic-design-check.yml` (to create)

---

**Document Owner**: Development Team
**Last Updated**: 2025-11-10
**Status**: READY FOR EXECUTION - Phase A can start immediately
