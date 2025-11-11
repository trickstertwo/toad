# TOAD TUI/UX Improvement Plan

**Status**: Pre-release optimization phase
**Goal**: Transform TOAD into both a milestone testing center and a professional AI terminal with excellent UX

---

## 📊 Current TUI Analysis

### ✅ What We Have (Strong Foundation)

**Widgets Available** (129 files across 16 categories):
- ✅ **AI**: diff_view, suggestions
- ✅ **Charts**: bar, line, scatter, sparkline, live graphs
- ✅ **Performance**: FPS counter, memory monitor, event metrics, render profiler
- ✅ **Progress**: spinners, token counter, multi-stage progress
- ✅ **Git**: branch manager, commit dialog, diff viewer, graph, staging UI, conflict resolver
- ✅ **Files**: tree, preview manager, card preview
- ✅ **Input**: text areas, vim mode, command palette, input dialogs
- ✅ **Layout**: split panes, floating windows, panels, tabs, minimap
- ✅ **Core**: dialogs, help, breadcrumbs, collapsible sections, tables, statusline
- ✅ **Notifications**: toasts, modals, tutorials

**Current Screens**:
- Welcome screen
- Trust dialog
- Main (conversation view)
- Evaluation (milestone testing)

**Current Features**:
- Real-time evaluation progress tracking (tasks, steps, tokens, cost)
- Keyboard shortcuts bar at bottom
- Metadata line showing path + model
- Command palette (Ctrl+P)
- Help screen (?)

### ❌ Current Issues Identified

1. **Green border looks bad** - Using `ToadTheme::TOAD_GREEN` everywhere
2. **"Conversation" label not self-explanatory** - Unclear what panel shows
3. **Can't type "?" anymore** - Conflicts with help shortcut
4. **Limited status updates** - Not enough real-time feedback on operations
5. **No milestone testing dashboard** - Evaluation screen is basic
6. **Missing agent state visualization** - Can't see what agent is thinking/doing
7. **No diff view in main UI** - File changes not visible during conversations
8. **No context window indicator** - Users don't know token usage
9. **No keyboard-first navigation** - Can't quickly jump between panels

---

## 🎯 Vim/Neovim Patterns to Adopt

### Modal Editing Concept (Adapted for AI Terminal)

**NOT**: Full vim text editing (we have a chat interface, not a text editor)
**YES**: Modal interaction modes for different contexts

#### Proposed Modes

```
┌─────────────────────────────────────────────────────────┐
│ MODES (vim-inspired but AI-terminal specific)          │
├─────────────────────────────────────────────────────────┤
│ 1. CHAT MODE (default)     - Normal conversation       │
│ 2. COMMAND MODE (:)        - Run CLI commands          │
│ 3. PALETTE MODE (Ctrl+P)   - Quick actions             │
│ 4. EVAL MODE (E)           - Milestone testing          │
│ 5. FILE MODE (F)           - File navigation/preview   │
│ 6. DIFF MODE (D)           - Review changes             │
│ 7. HELP MODE (?)           - Contextual help            │
└─────────────────────────────────────────────────────────┘
```

### Vim-Style Keyboard Navigation

**Already Have**: Command palette
**Add**:
- `j/k` - Scroll conversation up/down (in browse mode)
- `gg/G` - Jump to top/bottom of conversation
- `/` - Search in conversation
- `n/N` - Next/previous search result
- `Ctrl+d/u` - Page down/up
- `Space` - Toggle panel focus
- `Tab` - Cycle through panels
- Numbers (1-9) - Quick panel switching
- `v` - Visual mode to select messages for copy

### Status Line (Like Neovim)

```
┌──────────────────────────────────────────────────────┐
│ 🐸 TOAD  MODE: Chat  AGENT: idle  CTX: 45k/200k    │
│ ~/project  󰘦 3 files changed  󰄵 claude-sonnet-4.5  │
└──────────────────────────────────────────────────────┘
```

**Show**:
- Current mode (Chat, Eval, Diff, etc.)
- Agent status (idle, thinking, running tool, waiting)
- Context window usage (tokens used / max)
- File change count
- Current model
- Git branch (if in repo)

### Command Line (`:` prefix like vim)

```
:eval --milestone 1 --count 10
:compare --baseline 1 --test 2
:show-config --milestone 3
:git status
:help eval
:theme catppuccin
```

---

## 🧪 Features for Milestone Testing Center

### 1. **Evaluation Dashboard** (New Screen)

```
┌─────────────────────────────────────────────────────────────┐
│ 📊 Milestone Testing Dashboard                  Press E     │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  QUICK ACTIONS                    RECENT RUNS                │
│  ━━━━━━━━━━━━━                    ━━━━━━━━━━━━━━━━          │
│  1  M1 Baseline (10 tasks)        ✓ M1: 57.3% (2h ago)      │
│  2  M2 vs M1 A/B Test             ✓ M2: 63.1% (5h ago)      │
│  3  Custom Eval...                ✗ M1: failed (1d ago)     │
│  4  SWE-bench Verified                                       │
│                                   MILESTONES                 │
│  ACTIVE RUNS                      ━━━━━━━━━━━━              │
│  ━━━━━━━━━━━━━                    M0: ✓ Complete            │
│  None                             M1: → In Progress (57%)   │
│                                   M2: ○ Not Started          │
│                                   M3: ○ Not Started          │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

**Features**:
- Quick-launch eval configs (1-9 number keys)
- Live status of running evaluations
- Historical results with sparklines
- Milestone progress bars
- One-click A/B comparisons
- Export results to JSON/CSV

### 2. **Real-Time Agent Visualization**

```
┌─────────────────────────────────────────────────────────────┐
│ 🤖 Agent: Working on task django__django-12345              │
├─────────────────────────────────────────────────────────────┤
│ Step 3/25 │ Tool: Read                                       │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━       │
│                                                               │
│ 🔍 Reading: src/django/core/management/commands/migrate.py  │
│                                                               │
│ THOUGHT: "I need to understand the current migration logic   │
│          before making changes..."                           │
│                                                               │
│ TOOLS USED: Read(2) → Grep(1) → Edit(0)                     │
│ TOKENS: 12,453 / 200,000  COST: $0.0234                     │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

**Shows**:
- Current agent thought process (extracted from responses)
- Tool execution history with icons
- Progress through max steps (3/25)
- Real-time token/cost tracking
- File being worked on (clickable to preview)

### 3. **Statistical Comparison View**

```
┌─────────────────────────────────────────────────────────────┐
│ 📈 A/B Comparison: M1 Baseline vs M2 AST Context            │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ACCURACY          M1: 57.3%  ▁▃▅▇█  M2: 63.1% (+5.8pp)    │
│  ━━━━━━━━━                                                   │
│  Welch's t-test: p=0.012 (significant ✓)                     │
│  Effect size: d=0.43 (medium)                                │
│  Recommendation: ADOPT M2                                     │
│                                                               │
│  COST/TASK         M1: $0.12  ▁▂▃▄▅  M2: $0.18 (+50%)      │
│  ━━━━━━━━━                                                   │
│  Trade-off: +5.8pp accuracy costs +$0.06/task                │
│                                                               │
│  BREAKDOWN                                                    │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━      │
│  Tasks solved only by M1:  8                                 │
│  Tasks solved only by M2: 14                                 │
│  Both solved: 49   Neither: 29                               │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 4. **Task Detail Drill-Down**

Press Enter on a task to see detailed execution:

```
┌─────────────────────────────────────────────────────────────┐
│ 📝 Task: django__django-12345                                │
├─────────────────────────────────────────────────────────────┤
│ Status: ✓ SOLVED   Time: 127s   Cost: $0.145   Steps: 18/25│
│                                                               │
│ TIMELINE                                                      │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━       │
│ 0s    Read problem statement                                 │
│ 5s    List directory structure                               │
│ 12s   Read migration file                                    │
│ 23s   Grep for related code                                  │
│ 45s   Edit migration file                                    │
│ 67s   Run tests (3 passed, 0 failed)                         │
│ 89s   Verify changes                                         │
│ 127s  ✓ Tests passed                                         │
│                                                               │
│ FILES MODIFIED                                                │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━       │
│ + src/django/core/management/commands/migrate.py (+12, -3)   │
│                                                               │
│ [V]iew Diff  [L]ogs  [R]eplay  [E]xport                     │
└─────────────────────────────────────────────────────────────┘
```

---

## 🎨 UI/UX Improvements for AI Terminal

### 1. **Theme Overhaul** (Fix Green Border Problem)

**Current**: Aggressive green everywhere
**Proposed**: Subtle, context-aware colors

```rust
// NEW color scheme (Catppuccin-inspired)
pub struct ToadTheme {
    // Backgrounds
    SURFACE_0: Color,      // #1e1e2e (dark background)
    SURFACE_1: Color,      // #313244 (panels)
    SURFACE_2: Color,      // #45475a (borders - SUBTLE!)

    // Text
    TEXT: Color,           // #cdd6f4 (primary text)
    SUBTEXT_0: Color,      // #a6adc8 (secondary)
    SUBTEXT_1: Color,      // #bac2de (tertiary)

    // Accents (use sparingly!)
    ACCENT_PRIMARY: Color,   // #89b4fa (blue - info)
    ACCENT_SUCCESS: Color,   // #a6e3a1 (green - success only)
    ACCENT_WARNING: Color,   // #f9e2af (yellow - warnings)
    ACCENT_ERROR: Color,     // #f38ba8 (red - errors)
    ACCENT_SPECIAL: Color,   // #cba6f7 (purple - AI/special)
}
```

**Border Usage**:
- Default borders: SURFACE_2 (subtle gray, not green!)
- Active panel: ACCENT_PRIMARY (blue)
- Success states: ACCENT_SUCCESS (green) - only for ✓ marks
- AI thinking: ACCENT_SPECIAL (purple pulse animation)
- Errors: ACCENT_ERROR (red)

### 2. **Multi-Panel Layout** (Not Just Conversation)

```
┌────────────────────────────────────────────────────────┐
│ 🐸 TOAD  Chat  Agent: idle  45k/200k  main  󰘦 3    │ Status
├────────────────┬───────────────────────────────────────┤
│                │ 🤖 You                                 │
│  📁 Files      │ > Implement user authentication       │
│  ━━━━━━━━      │                                        │
│  src/          │ 🧠 Assistant (thinking...)            │
│  ├─ auth/      │ I'll help you implement authentication│
│  │  ├─ mod.rs  │ Let me break this down:               │
│  │  └─ user.rs │                                        │
│  └─ main.rs    │ 1. First, I'll read your current      │
│                │    auth module structure...            │
│  🔧 Tools      │                                        │
│  ━━━━━━━━      │ [Read] src/auth/mod.rs                │
│  Read(3)       │ ```rust                                │
│  Edit(1)       │ pub mod user;                          │
│  Bash(0)       │ pub mod session;                       │
│                │ ```                                    │
│  💬 Chat       │                                        │
│  🧪 Eval       │ 2. Now I'll implement JWT tokens...   │
│  📊 Stats      │                                        │
│                │                                        │
│                │ [Scroll: j/k  Search: /  Copy: v]     │
├────────────────┴───────────────────────────────────────┤
│ > |                                             [Chat] │ Input
└────────────────────────────────────────────────────────┘
```

**Left Sidebar** (toggleable with `\`):
- File tree (live updates when agent modifies files)
- Tool usage counter (this session)
- Quick mode switcher (Chat, Eval, Stats)

**Main Panel**:
- Conversation view (current)
- Code diffs when agent edits
- Test output when agent runs tests
- Evaluation results when in Eval mode

**Right Sidebar** (optional, `|` to toggle):
- Token/cost tracker (live)
- Agent thought process (extracted from responses)
- Minimap of conversation
- Context window visualization

### 3. **Status Indicators Everywhere**

#### Top Status Bar (Expanded)
```
┌────────────────────────────────────────────────────────┐
│ 🐸 TOAD  MODE: Chat  AGENT: 🔄 thinking  CTX: ████░░░ │
│ ~/my-project (main)  󰘦 3 modified  󰄵 sonnet-4.5  $0.23│
└────────────────────────────────────────────────────────┘
```

**Agent Status States**:
- 🟢 `idle` - Waiting for input
- 🔵 `thinking` - Processing request
- 🟡 `tool:Read` - Reading files
- 🟡 `tool:Edit` - Editing files
- 🟡 `tool:Bash` - Running commands
- ✅ `complete` - Task finished
- ❌ `error` - Failed

#### Inline Status (Within Conversation)
```
🧠 Assistant [🔄 thinking... 12s]
┌─────────────────────────────────────┐
│ ⏱ Processing your request...       │
│ 🔍 Step 1/3: Reading project files │
│ 📊 Tokens: 1,234 / 200,000         │
└─────────────────────────────────────┘
```

### 4. **Better Help System** (Fix "?" Problem)

**Solution**: Context-aware help overlay

- Press `F1` (or `?` when NOT typing) - Global help
- Press `?` in input field - Types "?" character
- Help shows **current mode shortcuts**

```
┌─────────────────────────────────────────────────────────┐
│ 󰋖 Help - Chat Mode                           [F1/Esc]  │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  NAVIGATION              ACTIONS                         │
│  ━━━━━━━━━━━━━━━━━━━━━  ━━━━━━━━━━━━━━━━━━━━━━━━       │
│  j/k      Scroll         Enter    Send message           │
│  gg/G     Top/Bottom     Ctrl+C   Cancel/Exit            │
│  Ctrl+D/U Page Down/Up   Ctrl+P   Command Palette        │
│  /        Search         Ctrl+R   Recent messages        │
│  n/N      Next/Prev      Ctrl+L   Clear screen           │
│  Tab      Next panel                                     │
│  \        Toggle sidebar MODES                           │
│                          ━━━━━━━━━━━━━━━━━━━━━━━━       │
│  COPY/PASTE              :        Command mode           │
│  ━━━━━━━━━━━━━━━━━━━━━  E        Eval mode              │
│  v        Visual mode    D        Diff mode              │
│  y        Yank (copy)    F        File mode              │
│  p        Paste                                           │
│                                                           │
│  💡 Tip: Press : to run CLI commands like :eval         │
└─────────────────────────────────────────────────────────┘
```

### 5. **Context Window Visualization**

```
┌────────────────────────────────────┐
│ 📊 Context Window                  │
├────────────────────────────────────┤
│ Used: 45,234 / 200,000 (22.6%)     │
│ ████████████░░░░░░░░░░░░░░░░░░░░  │
│                                     │
│ Breakdown:                          │
│ • System prompt:    1,234 (2.7%)   │
│ • Conversation:    32,456 (71.7%)  │
│ • Tool results:    11,544 (25.5%)  │
│                                     │
│ ⚠ Warning at 80%: 160,000 tokens  │
│ 🚨 Auto-trim at 90%: 180,000       │
└────────────────────────────────────┘
```

### 6. **Smart Notifications** (Not Just Toasts)

**Toast Levels**:
- 🔔 Info (blue) - "File saved"
- ✅ Success (green) - "Tests passed"
- ⚠ Warning (yellow) - "Context 80% full"
- ❌ Error (red) - "API rate limit"
- 💡 Tip (purple) - "Try :eval --help"

**Notification Center** (like macOS):
- Press `Shift+N` to see notification history
- Dismissable or persistent
- Link to relevant context (e.g., click "Tests failed" → jump to test output)

### 7. **File Diff Inline** (Not Separate View)

When agent edits a file, show inline diff in conversation:

```
🧠 Assistant
I've updated the authentication module:

📝 src/auth/mod.rs
┌─────────────────────────────────────────┐
│  1   pub mod user;                      │
│  2   pub mod session;                   │
│  3 + pub mod jwt;        // NEW         │
│  4 +                                    │
│  5 + use jsonwebtoken::{decode, encode};│
│  6                                       │
│  7   pub struct AuthConfig {            │
│  8 -     secret: String, // OLD         │
│  9 +     jwt_secret: Vec<u8>, // NEW    │
│ 10   }                                  │
└─────────────────────────────────────────┘

[A]ccept  [R]eject  [E]dit  [V]iew Full
```

### 8. **Command Palette Enhancement**

Current: Basic palette
**Add**: Fuzzy search + recent commands

```
┌─────────────────────────────────────────────────────────┐
│ 🎯 Command Palette                            Ctrl+P    │
├─────────────────────────────────────────────────────────┤
│ > eval m1_                                               │
│                                                           │
│ 󱁐 RECENT                                                 │
│   eval --milestone 1 --count 10                          │
│   compare --baseline 1 --test 2                          │
│                                                           │
│ 🔍 MATCHING "eval m1"                                    │
│   Eval: Run M1 baseline (10 tasks)                      │
│   Eval: Run M1 full suite (500 tasks)                   │
│   Eval: M1 vs M2 comparison                             │
│                                                           │
│ 💡 SUGGESTIONS                                           │
│   show-config --milestone 1                              │
│   help eval                                              │
└─────────────────────────────────────────────────────────┘
```

---

## 🚀 Priority Recommendations

### Immediate (Pre-M1 Release)

1. **Fix Green Border Issue** ✅ CRITICAL
   - Replace with SURFACE_2 gray for default borders
   - Only use green for success states

2. **Add Status Bar** ✅ HIGH
   - Show agent status (idle/thinking/tool)
   - Show context window usage
   - Show current mode

3. **Fix "?" Help Conflict** ✅ HIGH
   - Use F1 for help overlay
   - Allow "?" to be typed in input

4. **Add Agent Activity Indicator** ✅ HIGH
   - Show "thinking..." with spinner
   - Show current tool being used
   - Show elapsed time

5. **Improve Evaluation Screen** ✅ MEDIUM
   - Add progress bars
   - Show more real-time metrics
   - Better completion summary

### Short-term (M1 - M2)

6. **Multi-Panel Layout** ⭐ GAME CHANGER
   - Left sidebar with file tree
   - Right sidebar with context/tools
   - Main panel stays conversation

7. **Vim-Style Navigation** ⭐ POWER USER
   - j/k scrolling
   - gg/G jumps
   - / search
   - v visual mode

8. **Evaluation Dashboard** ⭐ TESTING CENTER
   - Quick-launch eval configs
   - Historical results
   - Milestone progress tracking

9. **Inline Diffs** ✅ UX
   - Show file changes in conversation
   - Accept/reject controls
   - Syntax highlighting

10. **Command Mode** ✅ POWER
    - `:eval`, `:compare`, `:help` commands
    - Command history
    - Auto-completion

### Long-term (M2+)

11. **Theme System** 🎨
    - Multiple themes (Catppuccin, Nord, Tokyo Night)
    - User customization
    - Dark/light mode toggle

12. **Minimap** 🗺️
    - Conversation overview
    - Quick scroll to sections
    - Highlight code blocks

13. **Smart Context Management** 🧠
    - Auto-trim old messages when full
    - Pin important context
    - Context usage visualization

14. **Statistical Dashboard** 📊
    - Detailed A/B comparisons
    - Welch's t-test visualizations
    - Cost/accuracy trade-off charts

15. **Session Management** 💾
    - Save/restore conversations
    - Export to markdown
    - Share eval results

---

## 📐 Proposed UI Layout (Final Vision)

```
┌──────────────────────────────────────────────────────────────────┐
│ 🐸 TOAD  Chat  🔵 thinking  ████████░░░░ 45k/200k  󰘦 3  $0.23  │ Statusline
├──────────┬──────────────────────────────────────┬────────────────┤
│          │ 🤖 You                                │ 🧠 Agent       │
│ 📁 Files │ > Add JWT auth                        │ ━━━━━━━━━━━━━ │
│ ━━━━━━━  │                                       │ Step 3/25      │
│ src/     │ 🧠 Assistant [🔄 thinking... 5s]     │ Tool: Read     │
│ ├ auth/  │ ┌────────────────────────────────┐   │                │ Sidebar
│ │ ├mod.rs│ │ 🔍 Reading auth module...      │   │ Reading:       │ (optional)
│ │ └user  │ │ 📊 Tokens: 1.2k / 200k         │   │ auth/mod.rs    │
│ └ main   │ └────────────────────────────────┘   │                │
│          │                                       │ 💭 Thought:    │
│ 🔧 Tools │ I'll help you add JWT. First,        │ "Need to add   │
│ ━━━━━━━  │ let me check your current setup...   │  JWT crate"    │
│ Read(3)  │                                       │                │
│ Edit(1)  │ 📝 src/auth/mod.rs                   │ 📊 Tokens      │
│ Bash(0)  │ ┌──────────────────────────────┐     │ ████░░░ 22%    │
│          │ │  1   pub mod user;           │     │                │
│ 💬 Chat  │ │  2   pub mod session;        │     │ 💰 Cost        │
│ 🧪 Eval  │ │  3 + pub mod jwt; // NEW     │     │ $0.0234        │
│ 📊 Stats │ │  ...                          │     │                │
│          │ └──────────────────────────────┘     └────────────────┘
│          │                                       │
│          │ [A]ccept  [R]eject  [V]iew Full      │ Conversation
│          │                                       │
├──────────┴──────────────────────────────────────┴────────────────┤
│ > |                                                        [Chat] │ Input
├───────────────────────────────────────────────────────────────────┤
│ j/k:Scroll  /:Search  v:Visual  Tab:Panel  F1:Help  Ctrl+P:Cmd   │ Shortcuts
└───────────────────────────────────────────────────────────────────┘
```

---

## 🎯 Success Metrics

After implementing these improvements, measure:

1. **User Productivity**
   - Time to launch eval: < 5 seconds (vs current ~20s)
   - Time to review results: < 10 seconds
   - Keyboard-only workflow: 100% possible

2. **Information Density**
   - Context usage visible: ✅
   - Agent status visible: ✅
   - File changes visible inline: ✅
   - Tool usage visible: ✅

3. **Aesthetic Quality**
   - Default borders: Subtle gray (not green!)
   - Color usage: Purposeful, not decorative
   - Contrast ratio: WCAG AAA compliant
   - Theme: Professional, not toy-like

4. **Testing Efficiency**
   - Launch M1 eval: 1 keypress (number on dashboard)
   - Compare M1 vs M2: 1 command (`:compare 1 2`)
   - Export results: Built-in
   - Historical tracking: Automatic

---

## 📝 Implementation Priority List

### Phase 1: Critical Fixes (This Week)
- [ ] Fix green border → gray default borders
- [ ] Add agent status indicator (idle/thinking/tool)
- [ ] Add context window bar (tokens used/max)
- [ ] Fix "?" help → F1 for help, "?" types normally
- [ ] Improve evaluation progress display

### Phase 2: Core Features (Next 2 Weeks)
- [ ] Multi-panel layout (file tree sidebar)
- [ ] Vim navigation (j/k, gg/G, /, v)
- [ ] Command mode (`:eval`, `:help`, etc.)
- [ ] Inline file diffs in conversation
- [ ] Enhanced status bar (mode, git, model)

### Phase 3: Testing Center (M1 Milestone)
- [ ] Evaluation dashboard with quick-launch
- [ ] Historical results tracking
- [ ] Milestone progress visualization
- [ ] Statistical comparison view
- [ ] Task detail drill-down

### Phase 4: Power User (M2+)
- [ ] Theme system (Catppuccin, Nord, etc.)
- [ ] Minimap for conversation
- [ ] Smart context management
- [ ] Session save/restore
- [ ] Advanced keyboard shortcuts

---

**Next Steps**: Review this plan, prioritize features, and start with Phase 1 critical fixes.
