# Domain-Driven Folder Structure Plan

## Current State Analysis

Our project currently has a flat `src/` structure with ~100 files mixing different concerns:
- UI/TUI widgets (60+ files in src/widgets/)
- AI/LLM integration (agent/, llm/, evaluation/)
- Core application logic (app.rs, tui.rs, event.rs)
- Infrastructure (config/, tools/, theme/)
- Domain utilities (search, navigation, session, etc.)

## Proposed Structure (Domain-Driven Design)

```
toad/
├── Cargo.toml
├── README.md
├── TODO_TUI.md
├── TODO_AI.md
│
├── src/
│   ├── main.rs                    # Binary entry point
│   ├── lib.rs                     # Library root with re-exports
│   │
│   ├── core/                      # Core domain - Application foundation
│   │   ├── mod.rs
│   │   ├── app.rs                 # Main App struct
│   │   ├── event.rs               # Event handling
│   │   ├── state.rs               # Application state
│   │   └── lifecycle.rs           # Init, Update, View cycle
│   │
│   ├── ui/                        # UI Domain - Terminal User Interface
│   │   ├── mod.rs
│   │   ├── tui.rs                 # TUI initialization
│   │   ├── render.rs              # Rendering orchestration
│   │   │
│   │   ├── widgets/               # UI Components
│   │   │   ├── mod.rs
│   │   │   ├── primitives/        # Basic widgets
│   │   │   │   ├── mod.rs
│   │   │   │   ├── input.rs
│   │   │   │   ├── textarea.rs
│   │   │   │   ├── dialog.rs
│   │   │   │   ├── modal.rs
│   │   │   │   └── table.rs
│   │   │   │
│   │   │   ├── visualization/     # Charts & graphs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── chart.rs
│   │   │   │   ├── line_chart.rs
│   │   │   │   ├── bar_chart.rs
│   │   │   │   ├── scatter_plot.rs
│   │   │   │   ├── live_graph.rs
│   │   │   │   ├── sparkline.rs
│   │   │   │   └── git_graph.rs
│   │   │   │
│   │   │   ├── layout/            # Layout management
│   │   │   │   ├── mod.rs
│   │   │   │   ├── split.rs
│   │   │   │   ├── panel.rs
│   │   │   │   ├── floating.rs
│   │   │   │   └── collapsible.rs
│   │   │   │
│   │   │   ├── navigation/        # Nav components
│   │   │   │   ├── mod.rs
│   │   │   │   ├── breadcrumbs.rs
│   │   │   │   ├── minimap.rs
│   │   │   │   ├── tabs (tabbar.rs)
│   │   │   │   └── filetree.rs
│   │   │   │
│   │   │   ├── feedback/          # User feedback
│   │   │   │   ├── mod.rs
│   │   │   │   ├── toast.rs
│   │   │   │   ├── progress.rs
│   │   │   │   ├── spinner.rs
│   │   │   │   └── help.rs
│   │   │   │
│   │   │   ├── editor/            # Text editing
│   │   │   │   ├── mod.rs
│   │   │   │   ├── vim_mode.rs
│   │   │   │   ├── vim_macros.rs
│   │   │   │   ├── mode_indicator.rs
│   │   │   │   └── undo_redo.rs
│   │   │   │
│   │   │   ├── performance/       # Performance widgets
│   │   │   │   ├── mod.rs
│   │   │   │   ├── fps.rs
│   │   │   │   ├── memory.rs
│   │   │   │   ├── event_metrics.rs
│   │   │   │   └── render_profiler.rs
│   │   │   │
│   │   │   ├── special/           # Special-purpose widgets
│   │   │   │   ├── mod.rs
│   │   │   │   ├── welcome.rs
│   │   │   │   ├── palette.rs
│   │   │   │   ├── context_menu.rs
│   │   │   │   ├── quick_actions.rs
│   │   │   │   ├── smart_suggestions.rs
│   │   │   │   ├── multiselect.rs
│   │   │   │   ├── session_manager.rs
│   │   │   │   └── workspace.rs
│   │   │   │
│   │   │   └── ai/                # AI-specific widgets
│   │   │       ├── mod.rs
│   │   │       ├── chat_panel.rs
│   │   │       ├── model_selector.rs
│   │   │       └── token_counter.rs
│   │   │
│   │   ├── styling/               # Visual styling
│   │   │   ├── mod.rs
│   │   │   ├── animation.rs
│   │   │   ├── borders.rs
│   │   │   ├── canvas.rs
│   │   │   ├── icons.rs
│   │   │   └── box_drawing.rs
│   │   │
│   │   └── theme/                 # Theme system
│   │       ├── mod.rs
│   │       ├── manager.rs
│   │       ├── builtin.rs
│   │       ├── catppuccin.rs
│   │       └── nord.rs
│   │
│   ├── ai/                        # AI Domain - LLM Integration
│   │   ├── mod.rs
│   │   ├── agent/                 # Agent system
│   │   │   ├── mod.rs
│   │   │   └── prompts.rs
│   │   │
│   │   ├── llm/                   # LLM clients
│   │   │   ├── mod.rs
│   │   │   ├── anthropic.rs
│   │   │   ├── errors.rs
│   │   │   └── rate_limiter.rs
│   │   │
│   │   └── evaluation/            # Evaluation framework
│   │       ├── mod.rs
│   │       ├── dataset_manager.rs
│   │       ├── experiment_manager.rs
│   │       └── task_loader.rs
│   │
│   ├── git/                       # Git Domain - Version Control
│   │   ├── mod.rs
│   │   └── diff.rs                # Diff parsing/display
│   │
│   ├── editor/                    # Editor Domain - Text Editing
│   │   ├── mod.rs
│   │   ├── vim_motions.rs
│   │   ├── visual_selection.rs
│   │   ├── multicursor.rs
│   │   ├── macros.rs
│   │   ├── marks.rs
│   │   ├── undo.rs
│   │   └── clipboard.rs
│   │
│   ├── workspace/                 # Workspace Domain - Project Management
│   │   ├── mod.rs
│   │   ├── session.rs
│   │   ├── workspaces.rs
│   │   ├── file_ops.rs
│   │   ├── recent_files.rs
│   │   └── bookmarks.rs
│   │
│   ├── navigation/                # Navigation Domain
│   │   ├── mod.rs
│   │   ├── navigation.rs
│   │   ├── search.rs
│   │   ├── advanced_search.rs
│   │   └── fuzzy.rs
│   │
│   ├── commands/                  # Command Domain - User Commands
│   │   ├── mod.rs
│   │   ├── command_mode.rs
│   │   ├── aliases.rs
│   │   ├── keybinds.rs
│   │   ├── key_sequences.rs
│   │   └── custom_keybindings.rs
│   │
│   ├── infrastructure/            # Infrastructure - Cross-cutting concerns
│   │   ├── mod.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   └── tui.rs
│   │   │
│   │   ├── errors.rs
│   │   ├── history.rs
│   │   ├── validation.rs
│   │   └── logo.rs
│   │
│   ├── performance/               # Performance Domain
│   │   ├── mod.rs
│   │   ├── performance.rs         # Core performance tracking
│   │   ├── lazy_render.rs
│   │   ├── virtual_scroll.rs
│   │   ├── async_ops.rs
│   │   └── background_tasks.rs
│   │
│   ├── metrics/                   # Metrics & Statistics
│   │   ├── mod.rs
│   │   └── ... (existing metrics files)
│   │
│   ├── stats/                     # Statistical analysis
│   │   ├── mod.rs
│   │   └── ... (existing stats files)
│   │
│   └── tools/                     # Development Tools
│       ├── mod.rs
│       ├── bash.rs
│       ├── edit.rs
│       ├── git.rs
│       ├── grep.rs
│       ├── list.rs
│       ├── read.rs
│       └── write.rs
│
└── tests/
    ├── integration_test.rs
    └── m0_validation_tests.rs
```

## Domain Descriptions

### 1. **core/** - Application Core
   - **Purpose**: Application lifecycle, state management, event loop
   - **Dependencies**: None (or minimal)
   - **Exports**: App, Event, State

### 2. **ui/** - User Interface
   - **Purpose**: All terminal UI rendering and interaction
   - **Dependencies**: core, theme
   - **Exports**: Tui, all widgets grouped by category

### 3. **ai/** - Artificial Intelligence
   - **Purpose**: LLM integration, agents, evaluation
   - **Dependencies**: core, infrastructure/config
   - **Exports**: Agent, LLM clients, Evaluation framework

### 4. **editor/** - Text Editing
   - **Purpose**: Vim-style editing, selections, cursor management
   - **Dependencies**: core
   - **Exports**: VimMotions, MultiCursor, Clipboard

### 5. **workspace/** - Project Management
   - **Purpose**: Session, files, bookmarks
   - **Dependencies**: core, infrastructure/config
   - **Exports**: Session, Workspace, FileOps

### 6. **navigation/** - Finding & Moving
   - **Purpose**: Search, fuzzy finding, navigation
   - **Dependencies**: core
   - **Exports**: Search, FuzzyFinder, Navigation

### 7. **commands/** - Command System
   - **Purpose**: Command palette, keybindings, aliases
   - **Dependencies**: core
   - **Exports**: CommandMode, KeyBindings, Aliases

### 8. **infrastructure/** - Cross-cutting
   - **Purpose**: Config, errors, logging, utilities
   - **Dependencies**: None
   - **Exports**: Config, ErrorHandler, History

### 9. **performance/** - Performance
   - **Purpose**: Rendering optimization, async ops
   - **Dependencies**: core
   - **Exports**: LazyRender, VirtualScroll, AsyncOps

### 10. **git/** - Version Control
   - **Purpose**: Git integration and diff viewing
   - **Dependencies**: core, workspace
   - **Exports**: Diff, GitOperations

## Migration Strategy

### Phase 1: Create New Structure (No Breaking Changes)
1. Create all new domain directories
2. Copy files to new locations (keep originals)
3. Update mod.rs files in new structure
4. Verify new structure compiles independently

### Phase 2: Update Imports
1. Update lib.rs to export from new structure
2. Add compatibility re-exports for old paths
3. Update internal imports gradually

### Phase 3: Remove Old Structure
1. Delete old files once all imports updated
2. Remove compatibility re-exports
3. Final cleanup and verification

### Phase 4: Documentation
1. Update README with new structure
2. Add ARCHITECTURE.md explaining domains
3. Update CONTRIBUTING.md with structure guidelines

## Benefits

1. **Clear Separation of Concerns**: Each domain has a single responsibility
2. **Better Discoverability**: Developers know where to find/add features
3. **Reduced Coupling**: Dependencies flow in one direction
4. **Easier Testing**: Test domains independently
5. **Scalability**: Easy to add new domains or features within domains
6. **Industry Standard**: Follows Rust best practices and DDD principles

## Next Steps

1. Review and approve this structure
2. Begin Phase 1 implementation
3. Incrementally migrate (one domain at a time)
4. Test after each domain migration
5. Complete migration and cleanup
