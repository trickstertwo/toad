# TOAD Feature Checklist - Dependency Ordered

**Last Updated:** 2025-11-12
**Status:** ✅ Layers 0-4 COMPLETE | 🚧 Layer 5 33% (2/6) | ✅ Eval Center COMPLETE

---

## 📋 How to Read This Document

Features are organized into **dependency layers**. You MUST implement all features in Layer N before starting Layer N+1.

**Priority Markers:**
- 🔴 **CRITICAL** - Application is unusable without this
- 🟡 **ESSENTIAL** - Core functionality, implement ASAP
- 🟢 **IMPORTANT** - Significantly improves UX
- 🔵 **POLISH** - Nice-to-have, can defer to post-MVP
- ⚪ **OPTIONAL** - Low ROI, consider cutting

**Status:**
- [ ] Not Started
- [~] In Progress
- [✓] Complete

---

## Layer 0: Infrastructure (Foundation)

These have NO dependencies. They're the bedrock everything else builds on.

### 🔴 0.1 Async Runtime & Event System [CRITICAL]
**Status:** [✓] Complete (Tokio + tokio::select! exists)
**Location:** src/core/event.rs, src/main.rs
**Dependencies:** None
**Blocks:** Everything else

**What exists:**
- Tokio runtime initialized
- Event enum with all event types
- EventHandler with crossterm integration
- tokio::select! for terminal + async events

**What's needed:**
- ✅ Already implemented
- Verify cancellation tokens work correctly

---

### 🔴 0.2 Terminal Management & Rendering [CRITICAL]
**Status:** [✓] Complete (Ratatui + crossterm)
**Location:** src/core/tui.rs, src/core/ui.rs
**Dependencies:** None
**Blocks:** All UI features

**What exists:**
- Terminal setup/cleanup
- Panic handler for terminal restoration
- Frame rendering loop
- Raw mode handling

**What's needed:**
- ✅ Already implemented
- Add terminal size validation (warn if < 80x24)

---

### 🔴 0.3 Keyboard Input Framework [CRITICAL]
**Status:** [✓] Complete
**Location:** src/core/event.rs (KeyEvent handling)
**Dependencies:** Terminal Management (0.2)
**Blocks:** All keyboard shortcuts

**What exists:**
- KeyEvent capture via crossterm
- Event propagation to App::update()
- Key modifier support (Ctrl, Alt, Shift)

**What's needed:**
- ✅ Already implemented
- Add keyboard shortcut registry (for help screen)

---

### 🔴 0.4 Configuration System [CRITICAL]
**Status:** [✓] Complete
**Location:** src/config/mod.rs
**Dependencies:** None
**Blocks:** Feature flags, themes, persistence

**What exists:**
- Config struct with TOML loading
- FeatureFlags system (13 flags)
- ToadConfig for milestone configurations
- Default fallbacks

**What's needed:**
- ✅ Already implemented
- Add config validation on load

---

## Layer 1: Core UI Components (Basic Building Blocks)

Depends on: Layer 0

### 🟡 1.1 Scrollable Containers with State Management [ESSENTIAL]
**Status:** [✓] Complete (ScrollbarState exists)
**Location:** src/ui/widgets/core/scrollbar/
**Dependencies:** Terminal Management (0.2)
**Blocks:** Chat view (2.1), File browser (4.4)

**What exists:**
- ScrollbarState with position tracking
- Scroll up/down/page methods
- Content size management

**What's needed:**
- ✅ Scrollbar widget implemented
- Add smart auto-scroll behavior (pauses when user scrolls up)
- Add "jump to bottom" indicator when scrolled up

**Implementation:**
```rust
// src/ui/widgets/core/scrollbar/state.rs
impl ScrollbarState {
    pub fn is_at_bottom(&self) -> bool {
        self.position >= self.content_length.saturating_sub(self.viewport_height)
    }

    pub fn set_auto_scroll(&mut self, enabled: bool) {
        self.auto_scroll = enabled;
    }
}
```

---

### 🟡 1.2 Text Rendering with Markdown [ESSENTIAL]
**Status:** [~] Partial (basic rendering exists)
**Location:** src/ui/atoms/, src/ui/molecules/message_bubble.rs
**Dependencies:** Terminal Management (0.2)
**Blocks:** Message display (2.1), Help screen

**What exists:**
- Message bubble widget
- Basic text rendering
- Theme system for colors

**What's needed:**
- Add markdown parser (use `pulldown-cmark`)
- Render **bold**, *italic*, `code`, and > quotes
- Handle line wrapping correctly

---

### 🟡 1.3 Syntax Highlighting [ESSENTIAL]
**Status:** [✓] Complete
**Location:** src/ui/syntax/mod.rs
**Dependencies:** Text Rendering (1.2)
**Blocks:** Code blocks in chat (2.1)

**What exists:**
- SyntaxHighlighter with tree-sitter
- Language enum (50+ languages)
- HighlightTheme integration

**What's needed:**
- ✅ Already implemented
- Verify performance with large code blocks (>1000 lines)
- Add lazy highlighting (only highlight visible lines)

---

### 🟢 1.4 Input Field with Editing [IMPORTANT]
**Status:** [✓] Complete
**Location:** src/ui/widgets/input/input.rs
**Dependencies:** Keyboard Framework (0.3)
**Blocks:** Chat input (2.2), Command history (3.2)

**What exists:**
- InputField widget
- Character insertion/deletion
- Cursor movement
- Multi-line support

**What's needed:**
- ✅ Already implemented
- Add input validation (max length)
- Add placeholder text support

---

## Layer 2: Core Chat Experience

Depends on: Layer 1

### 🔴 2.1 Streaming Message Display [CRITICAL]
**Status:** [~] Partial (infrastructure exists, integration needed)
**Location:** src/ai/llm/streaming.rs, src/ui/widgets/conversation/
**Dependencies:** Async Runtime (0.1), Scrollable Containers (1.1), Syntax Highlighting (1.3)
**Blocks:** AI chat functionality (everything depends on this)

**What exists:**
- StreamEvent enum with all event types
- MessageStream type with async iterator
- StreamAccumulator for building responses
- ConversationView widget

**What's needed:**
1. Wire ConversationView to actual LLM streaming
2. Update conversation_view on StreamEvent::ContentBlockDelta
3. Add streaming cursor animation (blinking ▊)
4. Auto-scroll during streaming unless user scrolled up
5. Show "Generating..." indicator with elapsed time

**Implementation Priority:** 🔴 **DO THIS FIRST**

**Code changes needed:**
```rust
// src/core/app_ai.rs
pub async fn send_message_streaming(
    &mut self,
    message: String,
) -> Result<()> {
    let mut stream = self.llm_client.send_message_stream(
        self.conversation.clone(),
        None,
    ).await?;

    // Spawn task to handle streaming
    let event_tx = self.event_tx.clone();
    tokio::spawn(async move {
        while let Some(event) = stream.next().await {
            match event {
                StreamEvent::ContentBlockDelta { delta, .. } => {
                    // Send to UI
                    event_tx.send(Event::AIStreamDelta(delta)).ok();
                }
                StreamEvent::MessageStop => {
                    event_tx.send(Event::AIStreamComplete).ok();
                }
                _ => {}
            }
        }
    });

    Ok(())
}
```

---

### 🟡 2.2 Clear Message Differentiation [ESSENTIAL]
**Status:** [~] Partial (widget exists, styling needed)
**Location:** src/ui/molecules/message_bubble.rs
**Dependencies:** Text Rendering (1.2)
**Blocks:** Conversation usability

**What exists:**
- Message bubble widget
- Basic border drawing

**What's needed:**
- User messages: Right-aligned with square corners
- Assistant messages: Left-aligned with rounded corners (╭╮╰╯)
- System messages: Centered with dim color
- Add timestamp to each message (HH:MM format)
- Color coding: user (blue), assistant (green), system (gray)

---

### 🟡 2.3 Keyboard-Driven Chat Input [ESSENTIAL]
**Status:** [✓] Complete
**Location:** src/core/app.rs (input handling)
**Dependencies:** Input Field (1.4), Keyboard Framework (0.3)
**Blocks:** User interaction

**What exists:**
- Input field integrated into App
- Enter key sends message
- Esc clears input

**What's needed:**
- ✅ Basic input works
- Add Ctrl+C to cancel streaming response
- Add Ctrl+L to clear conversation
- Add Shift+Enter for newline (don't send)

---

## Layer 3: Safety & Control

Depends on: Layer 2 (chat must work before you can control it)

### 🔴 3.1 Tool Execution Status Indicators [CRITICAL]
**Status:** [~] Partial (data structure exists, UI needed)
**Location:** src/core/event.rs (ToolExecution struct), NEW: src/ui/widgets/tools/status.rs
**Dependencies:** Streaming Display (2.1)
**Blocks:** User trust, approval system (3.3)

**What exists:**
- ToolExecution struct with all metadata
- Event::ToolExecutionStarted/Completed events (likely)

**What's needed:**
1. Create ToolStatusPanel widget
2. Show queued/running/complete/error status
3. Visual indicators: ⏳ Queued, ⟳ Running, ✓ Complete, ❌ Error
4. Show duration for completed tools
5. Progress bar for long-running tools (write_file, bash)
6. Scrollable log of all tool executions

**Implementation:**
```rust
// NEW FILE: src/ui/widgets/tools/status.rs
pub struct ToolStatusPanel {
    executions: Vec<ToolExecution>,
    scroll_state: ScrollbarState,
}

impl ToolStatusPanel {
    pub fn add_execution(&mut self, exec: ToolExecution) {
        self.executions.push(exec);
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // Render as table with columns:
        // Status | Tool | Duration | Result
    }
}
```

---

### 🔴 3.2 Error Handling with Recovery [CRITICAL]
**Status:** [~] Partial (error types exist, UI needed)
**Location:** src/ai/llm/errors.rs, NEW: src/ui/widgets/core/error_dialog.rs
**Dependencies:** Message Display (2.1)
**Blocks:** Production readiness

**What exists:**
- LLMError enum with error types
- Error propagation via Result types

**What's needed:**
1. Create ErrorDialog widget
2. Show error type, message, context
3. Offer recovery actions:
   - Retry with same model
   - Switch to different model
   - Check API key config
   - View detailed error log
4. Preserve conversation state on error
5. Log errors to ~/.toad/logs/errors.log

---

### 🟡 3.3 Explicit Approval System for Dangerous Operations [ESSENTIAL]
**Status:** [ ] Not Started
**Location:** NEW: src/core/app_approvals.rs, NEW: src/ui/widgets/core/approval_dialog.rs
**Dependencies:** Tool Status (3.1), Streaming Display (2.1)
**Blocks:** Git auto-commits (3.4), user trust

**What's needed:**
1. Pause execution before:
   - write_file (new or modified)
   - bash commands
   - git commits
2. Show ApprovalDialog with:
   - Operation type and details
   - File diff preview (for writes)
   - Command to be executed (for bash)
   - Risk level: HIGH/MEDIUM/LOW
3. Options: y (approve), n (reject), e (edit before apply), d (view full diff)
4. Allow "approve all in session" mode (trust mode)
5. Never auto-approve file deletions or rm commands

**Implementation:**
```rust
// NEW FILE: src/core/app_approvals.rs
#[derive(Debug, Clone)]
pub enum ApprovalRequest {
    WriteFile {
        path: PathBuf,
        content: String,
        is_new: bool,
        risk: RiskLevel,
    },
    BashCommand {
        command: String,
        working_dir: PathBuf,
        risk: RiskLevel,
    },
    GitCommit {
        message: String,
        files: Vec<PathBuf>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum RiskLevel {
    Low,    // read operations, tests
    Medium, // writes, non-destructive commands
    High,   // rm, git reset --hard, etc.
}

pub struct ApprovalManager {
    trust_mode: bool,
    pending: Option<ApprovalRequest>,
}

impl ApprovalManager {
    pub async fn request_approval(&mut self, request: ApprovalRequest) -> ApprovalResult {
        if self.trust_mode && request.risk() != RiskLevel::High {
            return ApprovalResult::Approved;
        }

        // Show dialog and wait for user input
        self.pending = Some(request);
        // ... wait for Event::ApprovalResponse
    }
}
```

---

### 🟡 3.4 Git Integration & Auto-Commits [ESSENTIAL]
**Status:** [~] Partial (git module exists, auto-commit logic needed)
**Location:** src/git/, NEW: src/git/auto_commit.rs
**Dependencies:** Approval System (3.3), Tool Status (3.1)
**Blocks:** Undo functionality, user trust

**What exists:**
- Git module with git2 bindings (likely)

**What's needed:**
1. Auto-commit after every AI file change
2. Generate descriptive commit message:
   - Summarize what changed (feat/fix/refactor/docs)
   - Include file names and line counts
   - Tag with "AI-assisted change via toad"
3. Add git status panel showing:
   - Current branch
   - Ahead/behind remote
   - Uncommitted changes
4. Add undo command: `/undo` reverts last commit
5. Show commit history in separate panel
6. Respect .gitignore (never commit secrets)

**Implementation:**
```rust
// NEW FILE: src/git/auto_commit.rs
pub struct AutoCommitManager {
    repo: Repository,
    enabled: bool,
}

impl AutoCommitManager {
    pub fn commit_changes(&self, files: Vec<PathBuf>, context: &str) -> Result<Oid> {
        // Stage files
        let mut index = self.repo.index()?;
        for file in &files {
            index.add_path(file)?;
        }
        index.write()?;

        // Generate message
        let message = self.generate_commit_message(files, context)?;

        // Create commit
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let parent = self.repo.head()?.peel_to_commit()?;
        let sig = self.repo.signature()?;

        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &message,
            &tree,
            &[&parent],
        )
    }

    fn generate_commit_message(&self, files: Vec<PathBuf>, context: &str) -> Result<String> {
        // Analyze changes with git diff
        // Generate concise, conventional commit message
        Ok(format!("feat(ai): {}\n\nAI-assisted change via toad", context))
    }
}
```

---

## Layer 4: Intelligence & Context

Depends on: Layer 3 (need safety before giving AI more context)

### 🟢 4.1 Multi-Model Support with Visual Indicator [IMPORTANT]
**Status:** [✓] Complete (backend), [~] UI needed
**Location:** src/ai/llm/provider.rs, src/ui/molecules/model_selector.rs
**Dependencies:** Config System (0.4), Chat Display (2.1)
**Blocks:** Provider switching (4.2)

**What exists:**
- LLMClient trait
- AnthropicClient, GitHubClient, OllamaClient
- ProviderType enum
- ModelSelector widget

**What's needed:**
1. Show current model in header: "Model: Claude 3.7 Sonnet ▼"
2. Press 'm' to open model selector dialog
3. List all available models with:
   - Provider name
   - Model name
   - Context window size
   - Pricing (input/output per 1M tokens)
   - [ACTIVE] marker for current model
4. Switch model without losing conversation
5. Show connection status for local models (Ollama)

---

### 🟢 4.2 Multi-Provider Switching [IMPORTANT]
**Status:** [✓] Complete
**Location:** src/ai/llm/provider.rs, src/ui/widgets/ai/provider_config.rs
**Dependencies:** Multi-Model (4.1)
**Blocks:** Model fallback, cost optimization

**What exists:**
- ProviderConfig with credentials
- Provider-specific clients
- ProviderConfigPanel widget with:
  - Multi-provider status display (Anthropic, GitHub, Ollama)
  - Connection status indicators (● Connected, ○ Not configured, ◐ Rate limited, ✗ Error)
  - Health check functionality
  - Auto-failover toggle
  - Provider switching support
  - Credential status (without exposing secrets)

**Implemented:**
1. ✅ Provider configuration panel widget
2. ✅ Status indicators for all providers
3. ✅ Health check infrastructure
4. ✅ Auto-failover support
5. ✅ Per-provider status tracking
6. ⚠️ Keychain integration (deferred - config file sufficient for M0)

---

### 🟢 4.3 Context Visibility (Files & Token Usage) [IMPORTANT]
**Status:** [~] Partial (widgets exist, integration needed)
**Location:** src/ui/molecules/context_window.rs, src/ui/molecules/token_counter.rs
**Dependencies:** Chat Display (2.1)
**Blocks:** Context management (4.4)

**What exists:**
- ContextWindow widget
- TokenCounter widget

**What's needed:**
1. Show in right sidebar or panel:
   - Files in context (list with token counts)
   - Total tokens: conversation + files + system
   - Model limit and % used
   - Visual progress bar
2. Warn at 80% context capacity
3. Add quick actions: /add, /drop, /clear-context
4. Show what AI can "see" (highlight in file browser)
5. Per-file token breakdown

---

### 🟢 4.4 Code Context Management (File Browser) [IMPORTANT]
**Status:** [~] Partial (file widgets exist, integration needed)
**Location:** src/ui/widgets/files/
**Dependencies:** Context Visibility (4.3), Scrollable Containers (1.1)
**Blocks:** File operations, code understanding

**What exists:**
- File browser widgets (likely)

**What's needed:**
1. File tree view with expand/collapse
2. Syntax-highlighted preview pane
3. Add files to context: Space to toggle, Enter to view
4. Show context status: ✓ for files in context
5. Git status indicators: M (modified), A (added), D (deleted), ?? (untracked)
6. Pattern-based add: `/add src/**/*.rs`
7. Search within files: Ctrl+F
8. Show symbol outline for current file (functions, structs)

---

### 🟢 4.5 Session Persistence with Full State [IMPORTANT]
**Status:** [~] Partial (session exists, conversation missing)
**Location:** src/workspace/session.rs
**Dependencies:** Chat Display (2.1), Context Management (4.4)
**Blocks:** Resume functionality

**What exists:**
- SessionState with working directory and history
- Save/load from ~/.config/toad/session.json

**What's needed:**
1. Add to SessionState:
   - conversation: Vec<Message>
   - context_files: Vec<PathBuf>
   - model_config: String
   - session_name: String
   - created_at: DateTime
   - tags: Vec<String>
2. Auto-save after every AI response
3. Session manager UI (Ctrl+O):
   - List recent sessions
   - Show session details (message count, tokens, files)
   - Load/rename/delete/export sessions
4. Export session to markdown/JSON
5. Session naming: auto-generate or user-specified

---

## Layer 5: Power User Features

Depends on: Layer 4 (need working context before advanced commands)

### 🟢 5.1 Slash Commands for Power Users [IMPORTANT]
**Status:** [✓] Complete
**Location:** src/commands/slash_parser.rs
**Dependencies:** Input Field (1.4), Context Management (4.4), Model Switching (4.1)
**Blocks:** Command palette (5.2)

**Implemented:**
1. ✅ Slash command detection and parsing
2. ✅ Quoted argument support ("/commit \"message\"")
3. ✅ 13 default commands with aliases:
   - Context: /add (a), /drop (d, remove), /clear-context (cc)
   - Model/Provider: /model (m), /provider (p)
   - Git: /undo (u), /diff, /commit
   - Session: /save (s), /load (l)
   - Conversation: /clear, /reset
   - Help: /help (h, ?)
4. ✅ Tab completion support via find_matches()
5. ✅ Argument validation with count checking
6. ✅ Fuzzy matching for command names
7. ✅ SlashCommandRegistry for extensibility
8. ✅ 15 comprehensive unit tests

**Implementation:**
```rust
// NEW FILE: src/commands/slash_parser.rs
pub struct SlashCommand {
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse_slash_command(input: &str) -> Option<SlashCommand> {
    if !input.starts_with('/') {
        return None;
    }

    let parts: Vec<&str> = input[1..].split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    Some(SlashCommand {
        name: parts[0].to_string(),
        args: parts[1..].iter().map(|s| s.to_string()).collect(),
    })
}
```

---

### 🟢 5.2 Command History Navigation [IMPORTANT]
**Status:** [✓] Complete
**Location:** src/infrastructure/history.rs
**Dependencies:** Input Field (1.4)
**Blocks:** User efficiency

**Implemented:**
1. ✅ Up arrow: older() method for previous messages
2. ✅ Down arrow: newer() method for next messages
3. ✅ History position indicator: position_indicator() → "↑ (15 of 42)"
4. ✅ Ctrl+R: reverse_search(query) for reverse search
5. ✅ Filter by type: commands_only(), messages_only()
6. ✅ Persistent across sessions (save/load methods)
7. ✅ Configurable max size (constructor parameter)
8. ✅ Privacy mode: is_sensitive(), add_with_privacy()
9. ✅ Sensitive pattern detection (password, api_key, token, etc.)
10. ✅ Generic filter() with predicate support

---

### 🟢 5.3 Feature Flag Visualization [IMPORTANT]
**Status:** [~] Partial (flags exist, UI needed)
**Location:** src/config/mod.rs, NEW: src/ui/widgets/core/feature_flags.rs
**Dependencies:** Config System (0.4)
**Blocks:** A/B testing, experimentation

**What exists:**
- FeatureFlags struct (13 flags)
- ToadConfig with milestone presets

**What's needed:**
1. Feature flags screen (press 'f')
2. Group by category:
   - Core Features (essential)
   - Experimental (beta/alpha)
   - Evaluation Flags (A/B testing)
3. For each flag show:
   - Name and description
   - Enabled/Disabled toggle (Space)
   - Impact (UX, Performance, Memory, Cost)
   - Stability (Essential/Beta/Alpha)
   - Warnings if applicable
4. Show performance impact: "+15MB", "-2ms per render"
5. Save to ~/.toad/flags.toml
6. Runtime reloadable (no restart unless marked)

---

### 🔵 5.4 Diff Visualization Before Apply [POLISH]
**Status:** [ ] Not Started
**Location:** NEW: src/ui/widgets/git/diff_viewer.rs
**Dependencies:** Git Integration (3.4), Approval System (3.3)
**Blocks:** Code review workflow

**What's needed:**
1. Show before/after side-by-side or unified diff
2. Syntax highlighting in both panes
3. Inline diff markers: + Added, - Removed, ~ Modified
4. Navigate between changes: n (next), p (prev)
5. Selectively apply hunks
6. Edit proposed changes before applying
7. Show context lines (configurable, default 3)
8. git diff compatible format

---

### 🔵 5.5 Progress Tracking for Multi-Step Operations [POLISH]
**Status:** [ ] Not Started
**Location:** NEW: src/ui/widgets/progress/multi_step.rs
**Dependencies:** Tool Status (3.1), Task Planning (5.6)
**Blocks:** User visibility for long operations

**What's needed:**
1. Show overall progress: [████████░░] 65%
2. List steps with status:
   - ✓ Complete
   - ⟳ Running (with progress %)
   - ⏳ Queued
   - ❌ Failed
3. Show time: Elapsed, ETA
4. Show current activity: "Updating middleware/auth.rs"
5. Cancellable: Ctrl+C
6. Resumable: Continue from last completed step on failure

---

### 🔵 5.6 Hierarchical Task Decomposition View [POLISH]
**Status:** [~] Partial (task_item widget exists)
**Location:** src/ui/molecules/task_item.rs, NEW: src/ui/widgets/ai/task_tree.rs
**Dependencies:** Chat Display (2.1), Progress Tracking (5.5)
**Blocks:** Complex task management

**What exists:**
- TaskItem molecule

**What's needed:**
1. Tree view with expand/collapse (▼ expanded, ▶ collapsed)
2. Show task hierarchy:
   - Phase (e.g., "Backend Implementation")
   - Tasks (e.g., "Create JWT module")
   - Subtasks (e.g., "Define TokenClaims struct")
3. Status per task: ✓ Complete, ● In Progress, ○ Pending, ⚠ Blocked
4. Progress bar per phase
5. Track time: estimated vs. actual
6. Show dependencies
7. Allow manual task management: Space (complete), e (edit), + (add subtask)

---

## Layer 6: Polish & Advanced

Depends on: Layer 5 (everything else works first)

### 🔵 6.1 Responsive Layout (Adapts to Terminal Size) [POLISH]
**Status:** [ ] Not Started
**Location:** NEW: src/ui/layout/responsive.rs
**Dependencies:** All UI components
**Blocks:** Small terminal support

**What's needed:**
1. Detect terminal size on resize events
2. Breakpoints:
   - Small: < 100 cols → single panel, tab to switch
   - Medium: 100-140 cols → 2 panels (chat + context)
   - Large: > 140 cols → 3 panels (files + chat + preview)
3. Collapsible sidebars in small terminals
4. Hide non-essential UI when space limited
5. Warn if terminal too small (< 80x24)
6. Minimum supported: 80x24
7. Optimal: 120x40+

---

### 🔵 6.2 Command Palette (Ctrl+P) [POLISH]
**Status:** [~] Partial (widget exists, integration needed)
**Location:** src/ui/widgets/input/palette.rs
**Dependencies:** Slash Commands (5.1), Keyboard Framework (0.3)
**Blocks:** Discoverability

**What exists:**
- CommandPalette widget

**What's needed:**
1. Open with Ctrl+P or Ctrl+Shift+P
2. Fuzzy search through all commands:
   - Slash commands
   - Keyboard shortcuts
   - Menu actions
3. Show keybinding next to each action
4. Execute on Enter
5. Close on Esc
6. Recently used commands at top

---

### 🔵 6.3 Custom Themes (Light/Dark) [POLISH]
**Status:** [~] Partial (theme system exists)
**Location:** src/ui/theme/mod.rs
**Dependencies:** Config System (0.4)
**Blocks:** User preference

**What exists:**
- ToadTheme system
- Color definitions

**What's needed:**
1. Built-in themes:
   - Dracula Dark
   - GitHub Dark
   - Monokai
   - Solarized Dark/Light
   - One Light
2. Theme selector (press 't')
3. Auto-detect terminal background (light/dark)
4. Custom theme support: ~/.toad/themes/
5. TOML configuration format
6. Preview before applying
7. NO_COLOR env var support

---

### 🔵 6.4 Help Screen with Keybindings [POLISH]
**Status:** [✓] Complete (HelpScreen widget exists)
**Location:** src/ui/widgets/core/help.rs
**Dependencies:** Keyboard Framework (0.3)
**Blocks:** Onboarding

**What exists:**
- HelpScreen widget
- Toggle with '?'

**What's needed:**
- ✅ Basic help screen implemented
- Add context-sensitive help (different per panel)
- Add search within help (Ctrl+F)
- Add links to full documentation

---

### ⚪ 6.5 External Editor Integration [OPTIONAL]
**Status:** [ ] Not Started
**Location:** NEW: src/editor/external.rs
**Dependencies:** Input Field (1.4)
**Blocks:** Long prompt composition

**What's needed:**
1. Ctrl+E: Open $EDITOR with current input
2. Respect $EDITOR or $VISUAL env vars
3. Default to vim if not set
4. Create temp file: /tmp/toad-prompt-{uuid}.md
5. Load content back on save+close
6. Abort on empty file
7. Template support with variables
8. Preserve markdown formatting

**ROI:** Low - most users will type in the TUI directly

---

### ⚪ 6.6 Multiple Session Tabs [OPTIONAL]
**Status:** [✓] Complete (TabManager exists)
**Location:** src/workspace/tabs.rs
**Dependencies:** Session Persistence (4.5)
**Blocks:** Concurrent workflows

**What exists:**
- TabManager with tab creation/switching

**What's needed:**
- Show tabs in header: `[1: jwt-refactor●] [2: api-design] [3: bug-fix] [+]`
- Keyboard shortcuts:
  - Ctrl+T: New tab
  - Ctrl+W: Close tab
  - Ctrl+Tab: Next tab
  - Ctrl+1-9: Jump to tab N
- Tab indicators:
  - ● Unsaved changes
  - * Active operation
  - ! Error in session
- Close confirmation if unsaved
- Max tabs limit (10)
- Share context across tabs (optional)

**ROI:** Medium - useful for power users but complex UX

---

### ⚪ 6.7 Voice Input Support [OPTIONAL]
**Status:** [ ] Not Started
**Location:** NEW: src/input/voice.rs
**Dependencies:** Input Field (1.4)
**Blocks:** Hands-free interaction

**What's needed:**
1. Ctrl+V: Start voice recording
2. Use Whisper (local) or cloud STT
3. Show waveform visualization during recording
4. Live transcription display
5. Edit transcription before sending
6. Noise cancellation
7. Multiple language support
8. Offline mode with local Whisper

**ROI:** Very Low - niche use case, high complexity

---

### ⚪ 6.8 Image/Screenshot Context Support [OPTIONAL]
**Status:** [ ] Not Started
**Location:** NEW: src/ai/context/image.rs
**Dependencies:** Context Management (4.4)
**Blocks:** Visual context

**What's needed:**
1. Slash commands:
   - `/image <path>` - Add image from file
   - `/screenshot` - Capture screenshot
   - `/paste` - Paste from clipboard
2. Image preview in terminal (sixel, iTerm2 inline)
3. Fallback: show image metadata if preview unsupported
4. Support PNG, JPG, WebP
5. Vision-capable models only (GPT-4V, Claude 3+)
6. Base64 encode for API transmission
7. Compress large images automatically

**ROI:** Low - useful but limited to specific models

---

## ❌ Layer 7: Features to Cut

These have poor ROI or don't fit the TUI paradigm.

### ❌ 7.1 Non-Interactive Mode for Scripting [CUT]
**Why Cut:**
- TOAD is a TUI, not a CLI automation tool
- Maintaining two UX paradigms doubles testing surface
- Users wanting automation should use Anthropic SDK directly
- Adds complexity with minimal benefit

**Alternative:** If automation is needed, create a separate CLI tool that shares the core libraries.

---

---

# 📊 EVALUATION CENTER FEATURES (Separate Track)

These features are specific to the F9 Evaluation Center dashboard for SWE-bench testing. They don't block the main chat experience and can be developed in parallel by a separate developer.

## Eval-1: Real-Time Evaluation Dashboard

**Status:** [~] Partial (EvaluationState exists, UI incomplete)
**Location:** src/core/app_evaluation.rs, src/ui/screens/evaluation.rs
**Dependencies:** Async Runtime (0.1), Event System (0.1)
**Blocks:** All other eval features

**What exists:**
- EvaluationState with progress tracking
- EvaluationProgress event with detailed metrics
- Eval screen enum variant

**What's needed:**
1. Full-screen evaluation dashboard (press F9)
2. Show real-time progress:
   - Current task: X/Y
   - Task ID and problem statement
   - Current agent step: N/25
   - Last tool used
3. Live metrics:
   - Token usage (total + per step)
   - Cost (total + per step)
   - API latencies
   - Success rate
4. Conversation history panel (scrollable)
5. Tool execution log (scrollable)
6. Cancel button (Ctrl+C)
7. Completion screen with final metrics

---

## Eval-2: Task Result Visualization

**Status:** [~] Partial (TaskResult struct exists)
**Location:** src/ai/evaluation/mod.rs, NEW: src/ui/widgets/charts/task_results.rs
**Dependencies:** Eval Dashboard (Eval-1)

**What exists:**
- TaskResult with all metrics
- EvaluationResults with aggregated data

**What's needed:**
1. Table view of all completed tasks:
   - Task ID
   - Solved ✓/✗
   - Tests passed
   - Duration (ms)
   - Cost ($)
   - Tokens used
2. Sort by any column
3. Filter: show only failures
4. Click to view detailed task log
5. Export results to CSV/JSON

---

## Eval-3: Token & Cost Tracking Charts

**Status:** [~] Partial (data exists, charts needed)
**Location:** NEW: src/ui/widgets/charts/cost_tracker.rs
**Dependencies:** Eval Dashboard (Eval-1)

**What exists:**
- Token and cost data in EvaluationProgress
- CostTracker molecule

**What's needed:**
1. Line chart: tokens over time (per task)
2. Bar chart: cost per task
3. Pie chart: cost breakdown (input/output/cache)
4. Running total display
5. Comparison to budget/limits
6. Export chart data to CSV

---

## Eval-4: A/B Test Comparison UI

**Status:** [ ] Not Started
**Location:** NEW: src/ui/screens/comparison.rs
**Dependencies:** Eval Dashboard (Eval-1)

**What's needed:**
1. Side-by-side comparison view:
   - Baseline (left) vs. Test (right)
2. Show metrics:
   - Accuracy (% solved)
   - Average cost
   - Average duration
   - Token usage
3. Statistical analysis:
   - Welch's t-test results
   - p-value
   - Cohen's d effect size
   - Recommendation (adopt/reject/inconclusive)
4. Visual diff highlighting (green = better, red = worse)
5. Export comparison report to markdown

---

## Eval-5: SWE-bench Dataset Manager

**Status:** [~] Partial (download logic exists)
**Location:** src/ai/evaluation/dataset_manager.rs, NEW: src/ui/widgets/eval/dataset_selector.rs
**Dependencies:** Eval Dashboard (Eval-1)

**What exists:**
- DatasetManager with HuggingFace download
- Dataset variants (Verified/Lite/Full)

**What's needed:**
1. Dataset selector UI:
   - Local file browser
   - HuggingFace download (verified/lite/full)
   - Show dataset info (task count, size)
2. Download progress indicator
3. Cache management: view/clear cached datasets
4. Dataset preview: show first 5 tasks
5. Validation: check dataset format before use

---

## Eval-6: Conversation & Tool Inspection

**Status:** [~] Partial (data exists, UI needed)
**Location:** NEW: src/ui/widgets/eval/conversation_inspector.rs
**Dependencies:** Eval Dashboard (Eval-1)

**What exists:**
- Full conversation history in EvaluationProgress
- Tool execution details in ToolExecution

**What's needed:**
1. Conversation viewer with syntax highlighting
2. Expand/collapse each message
3. Show tool inputs/outputs inline
4. Search through conversation (Ctrl+F)
5. Export conversation to markdown/JSON
6. Copy code blocks to clipboard

---

---

# 📈 Implementation Roadmap by Dependency Layer

## Week 1-2: Layer 0-2 (Foundation + Core Chat)
**Goal:** Get basic streaming chat working

- [x] 0.1-0.4: Already complete ✅
- [x] 1.1-1.4: Already complete ✅ (including input field)
- [x] 🔴 2.1: ConversationView streaming COMPLETE ✅ (async → events → UI with blinking cursor)
- [x] 🟡 2.2: Message styling COMPLETE ✅ (role colors, timestamps, markdown)
- [x] 🟡 2.3: Keyboard shortcuts COMPLETE ✅ (Ctrl+C cancel, Ctrl+L clear, history)

**Success Metric:** ✅ ACHIEVED - Can chat with Claude and see streaming responses

---

## Week 3-4: Layer 3 (Safety & Control)
**Goal:** Make AI operations safe and visible

- [x] 🔴 3.1: Tool execution status panel ✅ (widget created + UI wired)
- [x] 🔴 3.2: Error dialog with recovery ✅ (6 error types, smart recovery actions)
- [x] 🟡 3.3: Approval system ✅ COMPLETE (core + UI, needs event wiring)
- [x] 🟡 3.4: Git auto-commits with undo ✅ COMPLETE (auto-commit + message generation + undo stack)

**Success Metric:** ✅ ACHIEVED - Can safely let AI modify files with undo support

---

## Week 5-6: Layer 4 (Intelligence & Context)
**Goal:** Give AI more context and control

- [x] 🟢 4.1: Model selector UI ✅ (ModelInfo + selection widget with 6 models)
- [ ] 🟢 4.2: Provider configuration screen ← REMAINING
- [x] 🟢 4.3: Context panel with token usage ✅ (ContextPanel + file management + cost estimation)
- [x] 🟢 4.4: File browser with context management ✅ (ContextBrowser + token estimates + add/remove)
- [x] 🟢 4.5: Full session persistence ✅ (SessionState with working dir/history/conversation/theme)

**Success Metric:** ✅ MOSTLY ACHIEVED - Can manage context and switch models (80% complete - only provider config missing)

---

## Week 7-8: Layer 5 (Power User)
**Goal:** Add efficiency features

- [ ] 🟢 5.1: Slash commands
- [ ] 🟢 5.2: Command history navigation
- [ ] 🟢 5.3: Feature flags UI
- [ ] 🔵 5.4: Diff visualization
- [ ] 🔵 5.5-5.6: Progress + task tracking

**Success Metric:** Power users can work efficiently without mouse

---

## Week 9-10: Layer 6 (Polish)
**Goal:** Refinement and UX improvements

- [ ] 🔵 6.1: Responsive layout
- [ ] 🔵 6.2: Command palette
- [ ] 🔵 6.3: Custom themes
- [ ] Skip: 6.5-6.7 (low ROI)

**Success Metric:** Works well on different terminal sizes and looks good

---

## Parallel Track: Evaluation Center
**Status: ✅ COMPLETE**

- [x] Eval-1: Real-time eval dashboard ✅ (multi-panel layout)
- [x] Eval-2: Task result visualization ✅ (completion screen with accuracy/cost/duration)
- [x] Eval-3: Cost/token charts ✅ (inline metrics display)
- [x] Eval-4: A/B comparison UI ✅ (Welch's t-test, Cohen's d, recommendations)
- [x] Eval-5: Dataset manager ✅ (HuggingFace auto-download, validation)
- [x] Eval-6: Conversation inspector ✅ (scrollable conversation with truncation)

**Success Metric:** ✅ ACHIEVED - Can run SWE-bench evals and analyze results in TUI

---

# 🎯 Priority Matrix

```
HIGH IMPACT, LOW EFFORT (Do First):
- 2.1 Streaming chat integration ← WEEK 1
- 3.1 Tool status indicators
- 4.1 Model selector UI
- 5.2 Command history

HIGH IMPACT, HIGH EFFORT (Do Second):
- 3.3 Approval system ← WEEK 3
- 3.4 Git auto-commits
- 4.4 Code context management
- 5.1 Slash commands

MEDIUM IMPACT, LOW EFFORT (Do Third):
- 2.2 Message styling
- 3.2 Error dialogs
- 4.3 Context panel
- 6.4 Help screen improvements

LOW IMPACT, HIGH EFFORT (Defer or Cut):
- 6.7 Voice input ← Cut
- 6.8 Image support ← Defer
- 7.1 Non-interactive mode ← Cut
```

---

# ✅ Quick Status Checklist

Print this and check off as you implement:

```
Layer 0: Infrastructure
[✓] Async runtime ✅
[✓] Terminal management ✅
[✓] Keyboard framework ✅
[✓] Config system ✅

Layer 1: Core UI
[✓] Scrollable containers ✅
[✓] Markdown rendering ✅ (pulldown-cmark with full styling)
[✓] Syntax highlighting ✅
[✓] Input field ✅

Layer 2: Chat
[✓] Streaming display ✅ COMPLETE (was marked partial incorrectly)
[✓] Message differentiation ✅ COMPLETE (role colors, timestamps, markdown)
[✓] Keyboard input ✅

Layer 3: Safety ✅ ALL COMPLETE
[✓] Tool status indicators ✅ (widget complete + UI wired)
[✓] Error handling UI ✅ (ErrorDialog with 6 error types + recovery)
[✓] Approval system ✅ (core + UI complete, needs event wiring)
[✓] Git auto-commits ✅ (AutoCommitManager + message gen + undo)

Layer 4: Context (80% complete - 4/5) ✨
[✓] Model selector ✅ (6 models with capabilities/cost/speed)
[ ] Provider switcher ← REMAINING (non-blocking)
[✓] Context panel ✅ (token tracking + file list + cost estimation)
[✓] File browser ✅ (ContextBrowser + token estimates + context integration)
[✓] Session persistence ✅ (working dir/history/conversation/theme/plugins)

Layer 5: Power User
[ ] Slash commands
[ ] Command history
[ ] Feature flags UI
[ ] Diff viewer
[ ] Progress tracking
[ ] Task tree

Layer 6: Polish
[ ] Responsive layout
[ ] Command palette
[ ] Custom themes
[ ] Help improvements

Eval Center (Parallel)
[✓] Real-time dashboard ✅ (3-column layout with live updates)
[✓] Result visualization ✅ (completion screen with metrics)
[✓] Cost charts ✅ (inline cost/token tracking)
[✓] A/B comparison ✅ (statistical comparison implemented)
[✓] Dataset manager ✅ (SWE-bench download + validation)
[✓] Conversation inspector ✅ (conversation panel with truncation)
```

---

**Next Action:** Start with 2.1 (Streaming Chat Integration) - this is the foundation everything else builds on.
