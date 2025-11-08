# TUI Feature Roadmap: Basic → Platinum Grade
**For Ratatui + Crossterm AI Coding Terminal**

Inspired by: Bubbletea, Lazygit, gitui, bottom, yazi, and the Ratatui ecosystem

---

## 🟢 BASIC TIER - Essential Foundation

### Core Architecture
- [x] **Elm-style Architecture** (Init → Update → View) ✅ COMPLETED
  - Clean separation: Model (state), Messages (events), Update (logic), View (render)
  - Immutable state updates
  - Single source of truth
  - Multi-screen state machine (Welcome → TrustDialog → Main)

### Terminal Fundamentals
- [x] **Terminal Detection & Setup** ✅ COMPLETED
  - Raw mode activation
  - Alternate screen buffer
  - Proper cleanup on exit (restore terminal state)
  - Signal handling (SIGTERM, SIGINT)
  - Panic hook for terminal restoration

- [x] **Event Loop** ✅ COMPLETED
  - Keyboard event handling
  - Resize event handling
  - Graceful shutdown (Ctrl+C, q to quit)
  - Screen-specific event routing

### Basic Rendering
- [x] **Block Widget** - Borders, titles, basic containers ✅ COMPLETED
- [x] **Paragraph Widget** - Text rendering with wrapping ✅ COMPLETED
- [x] **Layout System** - Basic vertical/horizontal splits ✅ COMPLETED
- [x] **Status Bar** - Bottom bar with app state/help text ✅ COMPLETED
- [x] **Title Bar** - Top bar with app name/current view ✅ COMPLETED
- [x] **ASCII Branding** - TOAD logo (full, compact, minimal) ✅ COMPLETED

### Basic Styling
- [x] **Color Support** - RGB colors (toad green accent) ✅ COMPLETED
- [x] **Text Modifiers** - Bold, italic, underline ✅ COMPLETED
- [x] **Border Styles** - Box-drawing characters ✅ COMPLETED
- [x] **Theme Module** - Toad green + grayscale palette ✅ COMPLETED

### Navigation
- [x] **Single View Navigation** - Arrow keys navigation ✅ COMPLETED
- [x] **Basic Help Screen** - List of keybindings ✅ COMPLETED
- [x] **Quit Command** - q/Esc to exit ✅ COMPLETED

### Welcome & Onboarding
- [x] **Welcome Screen** - Split-pane logo + tips ✅ COMPLETED
- [x] **Trust Dialog** - Copilot-style folder confirmation ✅ COMPLETED
- [x] **Radio Button Selection** - Number keys + arrows ✅ COMPLETED

---

## 🟡 MEDIUM TIER - Enhanced Usability

### Advanced Widgets
- [x] **List Widget** - Scrollable lists with selection ✅ COMPLETED (via CommandPalette)
- [x] **Table Widget** - Multi-column data with headers ✅ COMPLETED
- [x] **Scrollbar** - Visual scroll indicators ✅ COMPLETED
- [x] **Input Field** - Single-line text input ✅ COMPLETED
- [x] **Textarea** - Multi-line text editing ✅ COMPLETED
- [x] **Progress Bars** - Task progress indicators ✅ COMPLETED
- [x] **Gauge/Meter** - Visual metrics display ✅ COMPLETED (via ProgressBar)

### Multi-Panel Layouts
- [x] **Split Panes** - Resizable horizontal/vertical splits ✅ COMPLETED
- [x] **Panel Focus System** - Tab/Shift+Tab to switch focus ✅ COMPLETED
- [x] **Panel Borders** - Visual indication of focused panel ✅ COMPLETED
- [x] **Dynamic Layout** - Panels can be shown/hidden ✅ COMPLETED

### Modal System
- [x] **Popup/Dialog Windows** - Centered overlays ✅ COMPLETED
- [x] **Confirmation Dialogs** - Yes/No prompts ✅ COMPLETED (TrustDialog)
- [x] **Input Prompts** - Modal text input ✅ COMPLETED (InputPrompt widget)
- [x] **Error Messages** - Modal error display ✅ COMPLETED
- [x] **ESC to Close** - Consistent modal dismissal ✅ COMPLETED

### Enhanced Navigation
- [x] **Vim-style Keybindings** - h/j/k/l navigation ✅ COMPLETED
- [x] **g/G Navigation** - Jump to top/bottom ✅ COMPLETED
- [x] **Page Up/Down** - Ctrl+u/d or PgUp/PgDn ✅ COMPLETED
- [x] **Tab Switching** - Number keys (1-9) or Alt+Number ✅ COMPLETED

### State Management
- [x] **Configuration File** - TOML/YAML settings ✅ COMPLETED
- [x] **State Persistence** - Save/restore session state ✅ COMPLETED (Session module)
- [x] **History Tracking** - Command/action history ✅ COMPLETED (Already implemented)

### Basic Search
- [x] **Forward Search** - / to search ✅ COMPLETED
- [x] **Next/Previous** - n/N to navigate results ✅ COMPLETED
- [x] **Highlight Matches** - Visual search feedback ✅ COMPLETED (SearchState implementation)

### Logging & Debugging
- [x] **File Logging** - Debug logs to toad.log ✅ COMPLETED
- [x] **Error Handling** - Graceful error display ✅ COMPLETED (Toast notifications)
- [x] **Performance Metrics** - Render time tracking ✅ COMPLETED

### Main Interface (from mockup)
- [x] **Input Prompt** - "Ask me anything or type a command..." at bottom ✅ COMPLETED
- [x] **Horizontal Separator** - Clean divider between content and input ✅ COMPLETED
- [x] **Keyboard Shortcuts Bar** - "Ctrl+C quit | ? help | / commands | Ctrl+P palette | Tab autocomplete" ✅ COMPLETED
- [x] **System Info Display** - Model (Sonnet 4.5), Runtime (Rust TUI) ✅ COMPLETED
- [x] **Plugin Counter** - "Active Plugins: N installed" ✅ COMPLETED
- [x] **Project Path Display** - Current working directory ✅ COMPLETED
- [x] **Placeholder Text** - Gray placeholder in input field ✅ COMPLETED
- [x] **Text Input Widget** - Cursor, character insertion, backspace ✅ COMPLETED
- [x] **Cursor Navigation** - Left/Right arrows, Home/End, Ctrl+A/E ✅ COMPLETED
- [x] **Input Editing** - Ctrl+U to clear ✅ COMPLETED

---

## 🔵 ADVANCED TIER - Standout Features

### Theming System
- [x] **Theme Support** - Multiple color schemes ✅ COMPLETED (Theme trait + ThemeManager)
- [x] **Built-in Themes** - Dark, light, high-contrast ✅ COMPLETED (DarkTheme, LightTheme, HighContrastTheme)
- [x] **Popular Themes** - Catppuccin, Nord, Everforest, Dracula, Tokyo Night ✅ COMPLETED (4 Catppuccin variants + Nord)
- [x] **Custom Themes** - User-defined themes from config ✅ COMPLETED (CustomTheme with TOML loading)
- [x] **256 Color Support** - Extended color palette ✅ COMPLETED (Ratatui Color::Rgb support)
- [x] **True Color (24-bit)** - RGB color support ✅ COMPLETED (All themes use RGB colors)
- [x] **Theme Hot-Reload** - Live theme switching ✅ COMPLETED (ThemeManager reload_custom_theme)

### Advanced Input
- [x] **Command Palette** - Ctrl+P fuzzy command search ✅ COMPLETED
- [x] **Autocomplete** - Tab completion for inputs ✅ COMPLETED (AutocompleteManager)
- [x] **Input Validation** - Real-time validation feedback ✅ COMPLETED (InputValidator with multiple validators)
- [x] **Multi-cursor Support** - Edit multiple locations ✅ COMPLETED (MultiCursor with position tracking, movement, primary cursor)
- [x] **Clipboard Integration** - Copy/paste support ✅ COMPLETED (Already implemented)

### Fuzzy Finding
- [x] **Fuzzy Search** - Skim/fzf-style searching ✅ COMPLETED (FuzzyMatcher with Exact/Substring/Fuzzy strategies)
- [x] **Smart Case** - Case-insensitive by default, smart switching ✅ COMPLETED (CaseMode::Smart)
- [x] **Preview Pane** - Show results in split pane ✅ COMPLETED (PreviewPane widget with scroll, line numbers, wrapping)
- [x] **Sorting & Ranking** - Relevance-based results ✅ COMPLETED (Score-based with consecutive bonus)
- [x] **Incremental Search** - Update results as you type ✅ COMPLETED (Real-time matching)

### Mouse Support
- [x] **Click to Focus** - Click panels to focus ✅ COMPLETED (MouseState system)
- [x] **Scroll Wheel** - Mouse scrolling in lists ✅ COMPLETED (ScrollDirection)
- [x] **Button Clicks** - Clickable UI elements ✅ COMPLETED (ClickAction)
- [x] **Drag & Drop** - Reorder items (advanced) ✅ COMPLETED (Drag tracking)
- [x] **Text Selection** - Mouse text selection ✅ COMPLETED (is_in_rect helper)

### Tab System
- [x] **Multiple Tabs** - Named workspaces ✅ COMPLETED (TabManager with add/close/navigation)
- [x] **Tab Bar** - Visual tab indicator ✅ COMPLETED (TabBar widget with selection, icons, close buttons)
- [x] **Tab Switching** - gt/gT or number keys ✅ COMPLETED (next_tab/previous_tab/switch_to_index)
- [x] **Tab Creation/Deletion** - :tabnew, :tabclose ✅ COMPLETED (add_tab/close_tab methods)
- [x] **Tab State** - Independent state per tab ✅ COMPLETED (Tab with title, icon, closable, modified)

### Advanced Layouts
- [x] **Resizable Panes** - Drag borders or keybinds ✅ COMPLETED (ResizablePaneManager with grow/shrink, min/max limits)
- [x] **Collapsible Sections** - Accordion-style panels ✅ COMPLETED (CollapsibleSection/CollapsibleList with expand/collapse)
- [x] **Floating Windows** - Draggable overlays ✅ COMPLETED (FloatingWindow/FloatingWindowManager with drag, minimize, close)
- [x] **Layout Presets** - Save/load layout configs ✅ COMPLETED (save_preset/load_preset with TOML)

### Performance Optimization
- [x] **Lazy Rendering** - Only render visible elements ✅ COMPLETED (LazyRenderState/LazyRenderManager with viewport, buffer zone)
- [x] **Virtual Scrolling** - Handle massive lists (1M+ items) ✅ COMPLETED (VirtualScrollState for efficient large datasets)
- [x] **Frame Rate Control** - Configurable FPS (30/60/120) ✅ COMPLETED (TargetFPS + FrameLimiter)
- [x] **Async Operations** - Non-blocking I/O ✅ COMPLETED (AsyncOperationManager with status, result tracking)
- [x] **Background Tasks** - Progress indicators for long ops ✅ COMPLETED (BackgroundTaskManager with status, progress, task lifecycle)

### Syntax Highlighting
- [ ] **Tree-sitter Integration** - AST-based highlighting
- [ ] **Language Support** - Common languages (Rust, JS, Python, etc.)
- [x] **Diff Highlighting** - Git-style diffs ✅ COMPLETED (DiffParser with unified diff format, ChunkHeader, DiffLine, FileDiff, DiffStats)
- [ ] **Semantic Colors** - Context-aware coloring

### Advanced Search & Filter
- [x] **Regex Search** - Full regex support ✅ COMPLETED (advanced_search.rs)
- [x] **Multi-field Filters** - Complex query syntax ✅ COMPLETED (FilterCondition with 7 operators)
- [x] **Saved Filters** - Bookmark common searches ✅ COMPLETED (SavedFilters with TOML persistence)
- [x] **Filter History** - Recent searches dropdown ✅ COMPLETED (FilterHistory with VecDeque)

### Notifications
- [x] **Toast Notifications** - Non-blocking alerts ✅ COMPLETED (toast.rs)
- [x] **Notification Queue** - Stack multiple notifications ✅ COMPLETED (ToastManager)
- [x] **Notification Levels** - Info/warning/error styling ✅ COMPLETED (ToastLevel enum)
- [x] **Auto-dismiss** - Time-based removal ✅ COMPLETED (is_visible + cleanup)

---

## 💎 PLATINUM TIER - Community-Beloved Excellence

### Visual Polish
- [ ] **Animations & Transitions** - Smooth panel transitions
- [x] **Loading Spinners** - Aesthetic async indicators ✅ COMPLETED (Spinner with 8 styles: Dots, Line, Bars, Bounce, Arrows, SimpleDots, Binary, Clock)
- [x] **Progress Animations** - Multi-stage task progress ✅ COMPLETED (MultiStageProgress widget with stage tracking, overall progress)
- [x] **Sparklines** - Inline graphs for metrics ✅ COMPLETED (Sparkline widget with Bars/Braille/Dots styles, min/max/avg, downsampling)
- [ ] **Canvas Drawing** - Custom graphics (charts, diagrams)
- [x] **Box Drawing Characters** - Beautiful Unicode borders ✅ COMPLETED (BoxChars with Light/Heavy/Double/Rounded/ASCII styles, BoxBuilder, draw utilities)
- [ ] **Nerd Font Icons** - Icon support (file types, status indicators)

### Graph & Data Visualization
- [ ] **Line Charts** - Time-series data
- [ ] **Bar Charts** - Comparison data
- [ ] **Scatter Plots** - Distribution visualization
- [ ] **Live Graphs** - Real-time updating charts
- [ ] **Git Graph** - Branch visualization (lazygit-style)

### Modal Editing (Vim-inspired)
- [x] **Multiple Modes** - Normal, Insert, Visual, Command ✅ COMPLETED (EditorMode enum: Normal/Insert/Visual/VisualLine/VisualBlock/Command/Replace)
- [x] **Mode Indicator** - Visual mode display ✅ COMPLETED (ModeIndicator widget with EditorMode enum, Full/Short/Block styles)
- [x] **Vim Motions** - w/b/e word movement, f/t character jump ✅ COMPLETED (VimMotions with w/b/e/W/B/E motions, f/F/t/T character jump, count support)
- [x] **Visual Selection** - V for line, v for char, Ctrl+v for block ✅ COMPLETED (VisualSelection with Character/Line/Block modes, contains/range checking)
- [x] **Macros** - Record and replay actions ✅ COMPLETED (MacroManager with recording, execution, save/load, 6 action types)
- [x] **Marks** - Set and jump to bookmarks ✅ COMPLETED (MarksManager with local/global/number/special marks, save/load to file)

### Power User Features
- [ ] **Custom Keybindings** - Fully remappable keys
- [ ] **Key Sequences** - Multi-key commands (like vim)
- [x] **Command Mode** - : for ex-style commands ✅ COMPLETED (CommandMode with buffer/cursor/history, CommandRegistry with handlers/aliases/suggestions)
- [ ] **Aliases** - Custom command shortcuts
- [ ] **Scripts/Plugins** - Extensibility (WASM, Lua, or native)

### Smart Features
- [x] **Context Menus** - Right-click or keybind for actions ✅ COMPLETED (ContextMenu with MenuItem, separators, icons, shortcuts, disabled items)
- [ ] **Quick Actions** - Frequently used commands surfaced
- [ ] **Smart Suggestions** - Context-aware hints
- [x] **Undo/Redo** - u/Ctrl+r for actions ✅ COMPLETED (UndoStack with Action trait, HistoryNavigator, dirty tracking)
- [x] **Session Management** - Save/restore entire sessions ✅ COMPLETED (Session with save/load, recent files, working directory, vim mode)
- [ ] **Workspace Switching** - Multiple project contexts

### Git Integration (for coding terminal)
- [ ] **Git Status Panel** - Live repository status
- [ ] **Commit Graph** - Visual branch history
- [ ] **Diff Viewer** - Inline/side-by-side diffs
- [ ] **Stage/Unstage** - Visual git add/reset
- [ ] **Commit UI** - Interactive commit creation
- [ ] **Branch Management** - Create/switch/delete branches
- [ ] **Conflict Resolution** - Merge conflict UI

### File Management
- [x] **Tree View** - Collapsible directory tree ✅ COMPLETED
- [ ] **File Preview** - Quick file preview pane
- [ ] **File Icons** - Type-based icons (Nerd Fonts)
- [ ] **File Operations** - Copy/move/delete/rename
- [ ] **Bookmarks** - Quick navigation to locations
- [ ] **Recent Files** - MRU list

### AI-Specific Features (for AI coding terminal)
- [ ] **Chat Panel** - Conversational AI interaction
- [ ] **Diff View** - Proposed changes visualization
- [ ] **Accept/Reject** - Quick code change approval
- [ ] **Streaming Responses** - Real-time AI output
- [ ] **Token Counter** - Usage tracking display
- [ ] **Model Selector** - Switch AI models
- [ ] **Context Display** - Show what AI sees

### Developer Experience
- [ ] **Command History** - Searchable command log
- [ ] **Breadcrumbs** - Navigation trail
- [ ] **Minimap** - Document overview (VSCode-style)
- [ ] **Multi-select** - Bulk operations on items
- [ ] **Batch Operations** - Apply actions to selections
- [ ] **Incremental Loading** - Stream large datasets
- [ ] **Export/Import** - Data portability

### Accessibility
- [ ] **Screen Reader Support** - Accessibility labels
- [ ] **High Contrast Mode** - Visual accessibility
- [ ] **Large Text Mode** - Configurable font size
- [ ] **Reduced Motion** - Disable animations option
- [ ] **Keyboard-only Mode** - Full functionality without mouse

### Performance Monitoring
- [ ] **FPS Counter** - Real-time performance
- [ ] **Memory Usage** - Resource monitoring
- [ ] **Event Metrics** - Track input lag
- [ ] **Render Profiling** - Debug slow renders

### Cross-Platform Excellence
- [ ] **Windows Support** - Full functionality on Windows
- [ ] **macOS Support** - Native experience
- [ ] **Linux Support** - Distro-agnostic
- [ ] **Terminal Detection** - Adapt to terminal capabilities
- [ ] **Fallback Modes** - Degrade gracefully on limited terminals

### Documentation & Onboarding
- [ ] **Interactive Tutorial** - First-run walkthrough
- [ ] **Contextual Help** - ? for context-specific help
- [ ] **Cheat Sheet** - Quick reference overlay
- [ ] **Demo Mode** - Showcase features
- [ ] **Tips on Startup** - Random helpful tips

---

## 🏆 INSPIRATION ANALYSIS

### What Makes TUIs Beloved

**Lazygit** (Git TUI):
- Multiple synchronized panels (status, files, branches, commits)
- One-key operations (space to stage, c to commit)
- Contextual help always visible
- Smooth workflow optimization

**gitui** (Rust Git TUI):
- Blazing fast performance (100x faster than lazygit in benchmarks)
- Minimal keystrokes for common operations
- Async operations don't block UI
- Memory efficient

**bottom/btm** (System Monitor):
- Beautiful real-time graphs
- Highly customizable layout
- Tree-mode process viewer
- Advanced filtering

**yazi** (File Manager):
- Asynchronous I/O for responsiveness
- Image preview in terminal
- Multi-tab support
- Plugin system

**Common Success Factors**:
1. **Speed** - Instant feedback, no lag
2. **Beauty** - Thoughtful use of colors, borders, spacing
3. **Efficiency** - Minimal keystrokes for common tasks
4. **Discoverability** - Help always accessible
5. **Polish** - Attention to details (animations, icons, themes)
6. **Reliability** - Never crashes, handles edge cases

---

## 📊 PRIORITIZATION STRATEGY

### For an AI Coding Terminal

**Must Have (MVP)**:
- Basic → Medium tier core features
- Command palette
- Fuzzy search
- File tree view
- Chat panel with streaming

**Should Have (v1.0)**:
- Advanced theming
- Git integration
- Syntax highlighting
- Modal editing system
- Tab support

**Could Have (v1.5+)**:
- Animations & polish
- Plugin system
- Advanced visualizations
- Session management
- Performance monitoring

**Nice to Have (v2.0+)**:
- Full Vim parity
- Advanced accessibility
- Screen sharing features
- Collaborative editing

---

## 🛠️ RATATUI/CROSSTERM SPECIFIC NOTES

### Ratatui Strengths
- Excellent widget library (Block, List, Table, Paragraph, Chart, Canvas)
- `StatefulWidget` for complex interactive components
- `Layout` constraint system for responsive designs
- `Frame` API for efficient rendering

### Crossterm Strengths
- Cross-platform terminal manipulation
- Mouse event support
- Async event stream
- Raw mode and alternate screen

### Recommended Crates for Platinum Features
- **tui-input** - Advanced text input
- **tui-textarea** - Multi-line editing
- **color-eyre** - Beautiful error reporting
- **tokio** - Async runtime
- **tree-sitter** - Syntax parsing
- **fuzzy-matcher** - Fuzzy finding
- **nucleo** - Modern fuzzy matching (used by Helix)
- **git2** - Git integration
- **notify** - File system watching
- **skim** - Fuzzy finder library

---

## 🎯 IMPLEMENTATION TIMELINE SUGGESTION

**Week 1-2**: Basic tier
**Week 3-4**: Medium tier core (widgets, panels, modals)
**Week 5-6**: Medium tier navigation & search
**Week 7-8**: Advanced tier theming & input
**Week 9-10**: Advanced tier performance & fuzzy finding
**Week 11-12**: Platinum tier visual polish
**Week 13+**: Platinum tier specialized features (Git, AI-specific)

---

**Remember**: Start simple, iterate fast, profile often. The best TUIs feel snappy and responsive above all else.
