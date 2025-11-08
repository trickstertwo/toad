# Domain-Driven Restructuring TODO

This document tracks the step-by-step process of reorganizing the codebase into a domain-driven architecture.

## Phase 1: Infrastructure Setup ✅ PLANNING COMPLETE

- [x] Create DOMAIN_DRIVEN_STRUCTURE.md with detailed plan
- [x] Create TODO_RESTRUCTURE.md (this file)
- [ ] Review plan with team/stakeholders
- [ ] Get approval to proceed

## Phase 2: Create Domain Structure (Directories Only)

### 2.1 Core Domains
- [ ] Create `src/core/` directory and mod.rs
- [ ] Create `src/ui/` directory and mod.rs
- [ ] Create `src/ai/` directory and mod.rs
- [ ] Create `src/editor/` directory and mod.rs
- [ ] Create `src/workspace/` directory and mod.rs
- [ ] Create `src/navigation/` directory and mod.rs
- [ ] Create `src/commands/` directory and mod.rs
- [ ] Create `src/git/` directory and mod.rs
- [ ] Create `src/performance/` directory and mod.rs
- [ ] Create `src/infrastructure/` directory and mod.rs

### 2.2 UI Subdomains
- [ ] Create `src/ui/widgets/` directory and mod.rs
- [ ] Create `src/ui/widgets/primitives/` directory and mod.rs
- [ ] Create `src/ui/widgets/visualization/` directory and mod.rs
- [ ] Create `src/ui/widgets/layout/` directory and mod.rs
- [ ] Create `src/ui/widgets/navigation/` directory and mod.rs
- [ ] Create `src/ui/widgets/feedback/` directory and mod.rs
- [ ] Create `src/ui/widgets/editor/` directory and mod.rs
- [ ] Create `src/ui/widgets/performance/` directory and mod.rs
- [ ] Create `src/ui/widgets/special/` directory and mod.rs
- [ ] Create `src/ui/widgets/ai/` directory and mod.rs
- [ ] Create `src/ui/styling/` directory and mod.rs
- [ ] Create `src/ui/theme/` directory and mod.rs

### 2.3 AI Subdomains
- [ ] Create `src/ai/agent/` directory and mod.rs
- [ ] Create `src/ai/llm/` directory and mod.rs
- [ ] Create `src/ai/evaluation/` directory and mod.rs

### 2.4 Infrastructure Subdomains
- [ ] Create `src/infrastructure/config/` directory and mod.rs

## Phase 3: Migrate Core Domain

### 3.1 Core Application Files
- [ ] Copy `src/app.rs` → `src/core/app.rs`
- [ ] Copy `src/event.rs` → `src/core/event.rs`
- [ ] Create `src/core/mod.rs` with exports
- [ ] Update internal imports in core files
- [ ] Test core domain compiles

### 3.2 TUI Foundation
- [ ] Copy `src/tui.rs` → `src/ui/tui.rs`
- [ ] Copy `src/ui.rs` → `src/ui/render.rs`
- [ ] Create `src/ui/mod.rs` with exports
- [ ] Update imports
- [ ] Test ui foundation compiles

## Phase 4: Migrate UI Widgets (Biggest Task)

### 4.1 Primitives (Basic Widgets)
- [ ] Move `src/widgets/input.rs` → `src/ui/widgets/primitives/input.rs`
- [ ] Move `src/widgets/textarea.rs` → `src/ui/widgets/primitives/textarea.rs`
- [ ] Move `src/widgets/dialog.rs` → `src/ui/widgets/primitives/dialog.rs`
- [ ] Move `src/widgets/modal.rs` → `src/ui/widgets/primitives/modal.rs`
- [ ] Move `src/widgets/table.rs` → `src/ui/widgets/primitives/table.rs`
- [ ] Move `src/widgets/input_dialog.rs` → `src/ui/widgets/primitives/input_dialog.rs`
- [ ] Move `src/widgets/input_prompt.rs` → `src/ui/widgets/primitives/input_prompt.rs`
- [ ] Create `src/ui/widgets/primitives/mod.rs`
- [ ] Test primitives compile

### 4.2 Visualization (Charts & Graphs)
- [ ] Move `src/widgets/chart.rs` → `src/ui/widgets/visualization/chart.rs`
- [ ] Move `src/widgets/line_chart.rs` → `src/ui/widgets/visualization/line_chart.rs`
- [ ] Move `src/widgets/bar_chart.rs` → `src/ui/widgets/visualization/bar_chart.rs`
- [ ] Move `src/widgets/scatter_plot.rs` → `src/ui/widgets/visualization/scatter_plot.rs`
- [ ] Move `src/widgets/live_graph.rs` → `src/ui/widgets/visualization/live_graph.rs`
- [ ] Move `src/widgets/sparkline.rs` → `src/ui/widgets/visualization/sparkline.rs`
- [ ] Move `src/widgets/git_graph.rs` → `src/ui/widgets/visualization/git_graph.rs`
- [ ] Create `src/ui/widgets/visualization/mod.rs`
- [ ] Test visualization widgets compile

### 4.3 Layout Components
- [ ] Move `src/widgets/split.rs` → `src/ui/widgets/layout/split.rs`
- [ ] Move `src/widgets/panel.rs` → `src/ui/widgets/layout/panel.rs`
- [ ] Move `src/widgets/floating.rs` → `src/ui/widgets/layout/floating.rs`
- [ ] Move `src/widgets/collapsible.rs` → `src/ui/widgets/layout/collapsible.rs`
- [ ] Move `src/resizable.rs` → `src/ui/widgets/layout/resizable.rs`
- [ ] Create `src/ui/widgets/layout/mod.rs`
- [ ] Test layout widgets compile

### 4.4 Navigation Components
- [ ] Move `src/widgets/breadcrumbs.rs` → `src/ui/widgets/navigation/breadcrumbs.rs`
- [ ] Move `src/widgets/minimap.rs` → `src/ui/widgets/navigation/minimap.rs`
- [ ] Move `src/widgets/tabbar.rs` → `src/ui/widgets/navigation/tabbar.rs`
- [ ] Move `src/widgets/filetree.rs` → `src/ui/widgets/navigation/filetree.rs`
- [ ] Move `src/tabs.rs` → `src/ui/widgets/navigation/tabs.rs`
- [ ] Create `src/ui/widgets/navigation/mod.rs`
- [ ] Test navigation widgets compile

### 4.5 Feedback Widgets
- [ ] Move `src/widgets/toast.rs` → `src/ui/widgets/feedback/toast.rs`
- [ ] Move `src/widgets/progress.rs` → `src/ui/widgets/feedback/progress.rs`
- [ ] Move `src/widgets/spinner.rs` → `src/ui/widgets/feedback/spinner.rs`
- [ ] Move `src/widgets/help.rs` → `src/ui/widgets/feedback/help.rs`
- [ ] Create `src/ui/widgets/feedback/mod.rs`
- [ ] Test feedback widgets compile

### 4.6 Editor Widgets
- [ ] Move `src/widgets/vim_mode.rs` → `src/ui/widgets/editor/vim_mode.rs`
- [ ] Move `src/widgets/vim_macros.rs` → `src/ui/widgets/editor/vim_macros.rs`
- [ ] Move `src/widgets/mode_indicator.rs` → `src/ui/widgets/editor/mode_indicator.rs`
- [ ] Move `src/widgets/undo_redo.rs` → `src/ui/widgets/editor/undo_redo.rs`
- [ ] Create `src/ui/widgets/editor/mod.rs`
- [ ] Test editor widgets compile

### 4.7 Performance Widgets
- [ ] Move `src/widgets/fps.rs` → `src/ui/widgets/performance/fps.rs`
- [ ] Move `src/widgets/memory.rs` → `src/ui/widgets/performance/memory.rs`
- [ ] Move `src/widgets/event_metrics.rs` → `src/ui/widgets/performance/event_metrics.rs`
- [ ] Move `src/widgets/render_profiler.rs` → `src/ui/widgets/performance/render_profiler.rs`
- [ ] Create `src/ui/widgets/performance/mod.rs`
- [ ] Test performance widgets compile

### 4.8 Special Widgets
- [ ] Move `src/widgets/welcome.rs` → `src/ui/widgets/special/welcome.rs`
- [ ] Move `src/widgets/palette.rs` → `src/ui/widgets/special/palette.rs`
- [ ] Move `src/widgets/context_menu.rs` → `src/ui/widgets/special/context_menu.rs`
- [ ] Move `src/widgets/quick_actions.rs` → `src/ui/widgets/special/quick_actions.rs`
- [ ] Move `src/widgets/smart_suggestions.rs` → `src/ui/widgets/special/smart_suggestions.rs`
- [ ] Move `src/widgets/multiselect.rs` → `src/ui/widgets/special/multiselect.rs`
- [ ] Move `src/widgets/session_manager.rs` → `src/ui/widgets/special/session_manager.rs`
- [ ] Move `src/widgets/workspace.rs` → `src/ui/widgets/special/workspace.rs`
- [ ] Move `src/widgets/preview.rs` → `src/ui/widgets/special/preview.rs`
- [ ] Move `src/widgets/scrollbar.rs` → `src/ui/widgets/special/scrollbar.rs`
- [ ] Move `src/widgets/statusline.rs` → `src/ui/widgets/special/statusline.rs`
- [ ] Create `src/ui/widgets/special/mod.rs`
- [ ] Test special widgets compile

### 4.9 AI Widgets
- [ ] Move `src/widgets/chat_panel.rs` → `src/ui/widgets/ai/chat_panel.rs`
- [ ] Move `src/widgets/model_selector.rs` → `src/ui/widgets/ai/model_selector.rs`
- [ ] Move `src/widgets/token_counter.rs` → `src/ui/widgets/ai/token_counter.rs`
- [ ] Create `src/ui/widgets/ai/mod.rs`
- [ ] Test AI widgets compile

### 4.10 Widget Styling
- [ ] Move `src/widgets/animation.rs` → `src/ui/styling/animation.rs`
- [ ] Move `src/widgets/borders.rs` → `src/ui/styling/borders.rs`
- [ ] Move `src/widgets/canvas.rs` → `src/ui/styling/canvas.rs`
- [ ] Move `src/widgets/icons.rs` → `src/ui/styling/icons.rs`
- [ ] Move `src/box_drawing.rs` → `src/ui/styling/box_drawing.rs`
- [ ] Move `src/nerd_fonts.rs` → `src/ui/styling/nerd_fonts.rs`
- [ ] Create `src/ui/styling/mod.rs`
- [ ] Test styling modules compile

### 4.11 Theme System
- [ ] Move `src/theme/` → `src/ui/theme/`
- [ ] Update theme module exports
- [ ] Test theme system compiles

### 4.12 Update Main Widgets Module
- [ ] Delete old `src/widgets.rs`
- [ ] Create new `src/ui/widgets/mod.rs` with all submodule exports
- [ ] Test all widgets accessible

## Phase 5: Migrate AI Domain

- [ ] Move `src/agent/` → `src/ai/agent/`
- [ ] Move `src/llm/` → `src/ai/llm/`
- [ ] Move `src/evaluation/` → `src/ai/evaluation/`
- [ ] Create `src/ai/mod.rs` with exports
- [ ] Update imports
- [ ] Test AI domain compiles

## Phase 6: Migrate Editor Domain

- [ ] Move `src/vim_motions.rs` → `src/editor/vim_motions.rs`
- [ ] Move `src/visual_selection.rs` → `src/editor/visual_selection.rs`
- [ ] Move `src/multicursor.rs` → `src/editor/multicursor.rs`
- [ ] Move `src/macros.rs` → `src/editor/macros.rs`
- [ ] Move `src/marks.rs` → `src/editor/marks.rs`
- [ ] Move `src/undo.rs` → `src/editor/undo.rs`
- [ ] Move `src/clipboard.rs` → `src/editor/clipboard.rs`
- [ ] Create `src/editor/mod.rs` with exports
- [ ] Update imports
- [ ] Test editor domain compiles

## Phase 7: Migrate Workspace Domain

- [ ] Move `src/session.rs` → `src/workspace/session.rs`
- [ ] Move `src/workspaces.rs` → `src/workspace/workspaces.rs`
- [ ] Move `src/file_ops.rs` → `src/workspace/file_ops.rs`
- [ ] Move `src/recent_files.rs` → `src/workspace/recent_files.rs`
- [ ] Move `src/bookmarks.rs` → `src/workspace/bookmarks.rs`
- [ ] Create `src/workspace/mod.rs` with exports
- [ ] Update imports
- [ ] Test workspace domain compiles

## Phase 8: Migrate Navigation Domain

- [ ] Move `src/navigation.rs` → `src/navigation/navigation.rs`
- [ ] Move `src/search.rs` → `src/navigation/search.rs`
- [ ] Move `src/advanced_search.rs` → `src/navigation/advanced_search.rs`
- [ ] Move `src/fuzzy.rs` → `src/navigation/fuzzy.rs`
- [ ] Create `src/navigation/mod.rs` with exports
- [ ] Update imports
- [ ] Test navigation domain compiles

## Phase 9: Migrate Commands Domain

- [ ] Move `src/command_mode.rs` → `src/commands/command_mode.rs`
- [ ] Move `src/aliases.rs` → `src/commands/aliases.rs`
- [ ] Move `src/keybinds.rs` → `src/commands/keybinds.rs`
- [ ] Move `src/key_sequences.rs` → `src/commands/key_sequences.rs`
- [ ] Move `src/custom_keybindings.rs` → `src/commands/custom_keybindings.rs`
- [ ] Create `src/commands/mod.rs` with exports
- [ ] Update imports
- [ ] Test commands domain compiles

## Phase 10: Migrate Git Domain

- [ ] Move `src/diff.rs` → `src/git/diff.rs`
- [ ] Create `src/git/mod.rs` with exports
- [ ] Update imports
- [ ] Test git domain compiles

## Phase 11: Migrate Performance Domain

- [ ] Move `src/performance.rs` → `src/performance/performance.rs`
- [ ] Move `src/lazy_render.rs` → `src/performance/lazy_render.rs`
- [ ] Move `src/virtual_scroll.rs` → `src/performance/virtual_scroll.rs`
- [ ] Move `src/async_ops.rs` → `src/performance/async_ops.rs`
- [ ] Move `src/background_tasks.rs` → `src/performance/background_tasks.rs`
- [ ] Create `src/performance/mod.rs` with exports
- [ ] Update imports
- [ ] Test performance domain compiles

## Phase 12: Migrate Infrastructure

- [ ] Move `src/config/` → `src/infrastructure/config/`
- [ ] Move `src/errors.rs` → `src/infrastructure/errors.rs`
- [ ] Move `src/history.rs` → `src/infrastructure/history.rs`
- [ ] Move `src/validation.rs` → `src/infrastructure/validation.rs`
- [ ] Move `src/logo.rs` → `src/infrastructure/logo.rs`
- [ ] Create `src/infrastructure/mod.rs` with exports
- [ ] Update imports
- [ ] Test infrastructure compiles

## Phase 13: Keep Existing (No Changes)

- [ ] Keep `src/metrics/` as-is (already organized)
- [ ] Keep `src/stats/` as-is (already organized)
- [ ] Keep `src/tools/` as-is (already organized)

## Phase 14: Update Root Module

- [ ] Update `src/lib.rs` with new domain structure
- [ ] Add re-exports for backward compatibility (temporary)
- [ ] Update module documentation
- [ ] Test all re-exports work

## Phase 15: Update Main Entry Point

- [ ] Update `src/main.rs` imports
- [ ] Test binary compiles and runs
- [ ] Verify all features work

## Phase 16: Testing & Verification

- [ ] Run full test suite: `cargo test`
- [ ] Run clippy: `cargo clippy -- -D warnings`
- [ ] Run fmt check: `cargo fmt -- --check`
- [ ] Test TUI launches correctly
- [ ] Test all major features work
- [ ] Check for any broken imports

## Phase 17: Documentation Updates

- [ ] Update README.md with new structure
- [ ] Create ARCHITECTURE.md explaining domains
- [ ] Update CONTRIBUTING.md with structure guidelines
- [ ] Update module-level documentation
- [ ] Add domain-level README files

## Phase 18: Cleanup

- [ ] Remove backward compatibility re-exports
- [ ] Remove empty old directories
- [ ] Final cargo fmt
- [ ] Final cargo clippy
- [ ] Final cargo test

## Phase 19: Git Commit

- [ ] Commit Phase 1-3: "Setup domain structure"
- [ ] Commit Phase 4: "Migrate UI widgets to domain structure"
- [ ] Commit Phase 5-12: "Migrate remaining domains"
- [ ] Commit Phase 13-15: "Update root modules and main"
- [ ] Commit Phase 16-18: "Testing, docs, and cleanup"
- [ ] Push all changes

## Phase 20: Post-Migration

- [ ] Update CI/CD if needed
- [ ] Notify team of new structure
- [ ] Monitor for issues
- [ ] Address any regressions

---

## Estimated Timeline

- **Phase 1-3**: 1-2 hours (Setup and core)
- **Phase 4**: 4-6 hours (Widgets - biggest task)
- **Phase 5-12**: 3-4 hours (Other domains)
- **Phase 13-18**: 2-3 hours (Root updates and testing)
- **Total**: 10-15 hours

## Risk Mitigation

1. **Incremental commits**: Commit after each domain
2. **Keep old files**: Don't delete until fully verified
3. **Backward compatibility**: Add re-exports during transition
4. **Test frequently**: Run tests after each phase
5. **Rollback plan**: Each commit is a checkpoint

## Success Criteria

- ✅ All tests pass
- ✅ No compilation warnings
- ✅ All features work correctly
- ✅ Code is more organized and discoverable
- ✅ Dependencies flow correctly (no circular deps)
- ✅ Documentation is up-to-date
