# Theme System Integration Status

## ✅ **What's Implemented**

### Core Infrastructure
- ✅ `ThemeManager` - Manages theme state (current theme selection)
- ✅ `ThemeName` enum - 8 built-in themes (Dark, Light, HighContrast, 4x Catppuccin, Nord)
- ✅ Session persistence - Theme saves/restores across app restarts
- ✅ Settings screen (F10) - UI to select and apply themes
- ✅ `ResolvedThemeColors` - **NEW** Dynamic theme color resolver

### NEW: ResolvedThemeColors Resolver Pattern

**File**: `src/ui/theme/resolver.rs`

**Purpose**: Provides runtime theme color resolution, allowing widgets to query colors from the active theme instead of using hardcoded constants.

**Usage Pattern**:
```rust
// In render function:
use crate::ui::theme::ResolvedThemeColors;

pub fn render(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
    // Instead of: ToadTheme::TOAD_GREEN
    let border_color = colors.accent();

    // Instead of: ToadTheme::FOREGROUND
    let text_color = colors.foreground();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().fg(text_color));
}

// At call site (e.g., in ui.rs):
let colors = ResolvedThemeColors::from_manager(&app.theme_manager);
widget.render(frame, area, &colors);
```

---

## 📊 **Migration Progress**

### Completed Files (3/56 files, 23/539 refs = 4.3%)
- ✅ `src/ui/widgets/core/settings_screen.rs` - 12 refs updated
- ✅ `src/ui/widgets/core/help.rs` - 6 refs updated
- ✅ `src/ui/widgets/core/theme_selector.rs` - 5 refs updated
- ✅ `src/core/ui.rs` - 2 refs updated (help/settings render calls)

### Remaining Work (516 references across 53 files)

**HIGH PRIORITY** (UI-visible, user-facing screens):
1. ⏳ `src/core/ui.rs` - 72 refs remaining (⚠️ 703 LOC - OVER 600 LOC LIMIT)
2. ⏳ `src/ui/widgets/core/welcome_screen.rs` - 39 refs
3. ⏳ `src/ui/widgets/layout/minimap.rs` - 27 refs
4. ⏳ `src/ui/widgets/layout/window_switcher.rs` - 19 refs
5. ⏳ `src/ui/screens/results.rs` - 19 refs

**MEDIUM PRIORITY** (functional widgets, 10-19 refs each):
- `src/ui/atoms/block.rs` - 19 refs
- `src/ui/widgets/input/mode_indicator.rs` - 16 refs
- `src/ui/widgets/input/input_prompt/state.rs` - 16 refs
- `src/ui/molecules/progress_bar.rs` - 16 refs
- `src/ui/widgets/files/card_preview.rs` - 14 refs
- `src/ui/molecules/metric_card.rs` - 14 refs
- `src/ui/widgets/input/palette/state.rs` - 13 refs
- `src/ui/organisms/eval_panel.rs` - 13 refs
- `src/ui/organisms/accept_reject_panel.rs` - 13 refs
- `src/ui/molecules/task_item.rs` - 13 refs
- `src/ui/widgets/input/input_dialog/state.rs` - 12 refs
- `src/ui/molecules/model_selector.rs` - 11 refs
- `src/ui/screens/main_screen.rs` - 10 refs

**LOW PRIORITY** (atoms/molecules/minor widgets, <10 refs each): 40 files, ~250 refs

---

## ❌ **What's NOT Integrated (Yet)**

### The Problem: 516 Hardcoded Color References Remaining

**Current State**: Most widgets still use hardcoded `ToadTheme::*` constants:
```rust
// 516 occurrences across 53 files:
Style::default().fg(ToadTheme::TOAD_GREEN)      // ❌ Hardcoded Dark theme
Style::default().bg(ToadTheme::BLACK)            // ❌ Always black
Style::default().fg(ToadTheme::FOREGROUND)       // ❌ Always light gray
```

**Impact**:
- Theme selection UI works ✅
- Theme persists across sessions ✅
- Settings/Help screens respect theme ✅
- **But most UI colors don't change** ❌

**Why This Happens**:
- Ratatui rendering is stateless (no global theme state)
- Colors are resolved at render time
- Most widgets don't have access to `ThemeManager` during rendering

---

## 🔧 **The Fix (In Progress)**

### Pattern to Follow

**Step 1**: Add `ResolvedThemeColors` parameter to widget render methods:
```rust
// Before:
pub fn render(&self, frame: &mut Frame, area: Rect) {
    let color = ToadTheme::TOAD_GREEN;  // ❌ Hardcoded
}

// After:
pub fn render(&self, frame: &mut Frame, area: Rect, colors: &ResolvedThemeColors) {
    let color = colors.accent();  // ✅ Dynamic
}
```

**Step 2**: Update call sites to pass colors:
```rust
// In ui.rs or parent widget:
let colors = ResolvedThemeColors::from_manager(&app.theme_manager);
widget.render(frame, area, &colors);
```

**Step 3**: Replace hardcoded constants:
```rust
// Mapping from ToadTheme to ResolvedThemeColors:
ToadTheme::FOREGROUND       → colors.foreground()
ToadTheme::BACKGROUND       → colors.background()
ToadTheme::TOAD_GREEN       → colors.accent()
ToadTheme::TOAD_GREEN_BRIGHT → colors.accent_bright()
ToadTheme::TOAD_GREEN_DARK  → colors.accent_dark()
ToadTheme::GRAY             → colors.gray()
ToadTheme::DARK_GRAY        → colors.dark_gray()
ToadTheme::ERROR            → colors.error()
ToadTheme::SUCCESS          → colors.success()
ToadTheme::WARNING          → colors.warning()
ToadTheme::INFO             → colors.info()
ToadTheme::WHITE            → colors.foreground() (or context-specific)
ToadTheme::BLACK            → colors.background() (or context-specific)
ToadTheme::BLUE             → colors.info() (or context-specific)
ToadTheme::YELLOW           → colors.warning() (or context-specific)
ToadTheme::RED              → colors.error()
```

---

## ✅ **Testing**

### Resolver Tests
- ✅ 11 unit tests in `src/ui/theme/resolver.rs`
- ✅ All 8 themes have color mappings
- ✅ `from_manager()` and `from_theme_name()` work correctly
- ✅ Clone and Copy traits work

### Widget Tests
- ✅ Settings screen tests updated (8 tests pass)
- ✅ Help screen tests updated (16 tests pass)
- ✅ Theme selector tests pass (no changes needed)

### Integration Tests Needed
- ❌ Test that changing theme in settings actually changes UI colors
- ❌ Test theme persistence (load → change → restart → verify)
- ❌ Visual regression tests (screenshot comparison per theme)

---

## 🎯 **Completion Estimate**

**Total Scope**: 56 files, 539 references
**Completed**: 3 files (3 fully done), 23 refs (4.3%)
**Remaining**: 53 files, 516 refs (95.7%)

**Estimated Time**: 8-12 hours total
- High Priority (5 files, 176 refs): 3-4 hours
- Medium Priority (13 files, 180 refs): 3-4 hours
- Low Priority (40 files, 160 refs): 2-4 hours

**Phases**:
1. ✅ Phase 1: Create `ResolvedThemeColors` resolver (DONE)
2. ✅ Phase 2: Update settings/help/theme_selector (DONE)
3. ⏳ Phase 3: Update core UI rendering (ui.rs) - IN PROGRESS
4. ⏳ Phase 4: Update high-priority widgets
5. ❌ Phase 5: Update medium/low priority widgets
6. ❌ Phase 6: Integration tests

---

## ⚠️ **Quality Gate Violations**

### File Size Violations (> 600 LOC)
- ❌ `src/core/ui.rs` - **703 LOC** (103 over limit)
  - **Recommendation**: Refactor into smaller modules after theme migration
  - Consider: `ui/render/welcome.rs`, `ui/render/main.rs`, `ui/render/evaluation.rs`

---

## 📝 **Developer Notes**

### Why Not Use Thread-Local?
```rust
// This would work but breaks Elm Architecture:
thread_local! {
    static THEME: RefCell<ThemeManager> = /*...*/;
}
```
**Problems**:
- Global mutable state (not functional)
- Hard to test
- Violates separation of concerns

### Why Not Global Static?
```rust
// Can't do this - Color isn't const:
pub static CURRENT_THEME_COLORS: ThemeColors = ThemeColors::dark();
```
**Problem**: Ratatui's `Color` isn't `const`, so can't use in statics.

### Why the Chosen Approach Works
- ✅ Maintains functional purity (explicit state passing)
- ✅ Testable (colors are parameters)
- ✅ Follows Elm Architecture (data flows down)
- ✅ Type-safe (compiler enforces color usage)
- ✅ Zero runtime overhead (colors are Copy)

---

## 🚀 **Next Steps**

### Immediate (Next Session)
1. **Update ui.rs** - Replace 72 remaining `ToadTheme::*` with `colors.*`
   - Create `colors` once at top of each render function
   - Pass down to child renders (metadata, separator, shortcuts, evaluation screens)

2. **Update welcome_screen.rs** - 39 refs
   - Add `colors` parameter to `render()` and `render_tips()`

3. **Update minimap.rs** - 27 refs

### Short-Term (This Week)
4. **Complete high-priority widgets** (window_switcher, results screen)
5. **Update atoms/block.rs** - Many widgets depend on this
6. **Document refactoring plan** for ui.rs (split into modules)

### Long-Term
7. **Complete all medium/low priority widgets**
8. **Write integration tests**
9. **Refactor ui.rs** into smaller modules
10. **Add visual regression tests** (screenshot per theme)

---

## 📚 **References**

- **Resolver Implementation**: `src/ui/theme/resolver.rs`
- **Theme Manager**: `src/ui/theme/manager.rs`
- **Example Usage**: `src/ui/widgets/core/settings_screen.rs`, `src/ui/widgets/core/help.rs`
- **Tests**: `src/ui/theme/resolver.rs::tests`
- **Session State**: `src/workspace/session.rs` (theme persistence)

---

**Last Updated**: 2025-01-12 (Session continuation)
**Status**: 🔄 IN PROGRESS (Phase 3/6 - Updating core UI)
**Progress**: 23/539 refs complete (4.3%)
