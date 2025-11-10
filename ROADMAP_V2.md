# TOAD Roadmap V2: Evidence-Based AI Terminal + Automation Platform

> **Mission:** Build the world's best AI coding terminal, then expand to a universal automation platform

**Last Updated:** 2025-11-10
**Current Status:** M0-M4 Complete (Tests Broken) → Phase 0 Required
**First Release Target:** Q1 2026 (AI Commanding Center)
**Platform Vision:** Q3 2026 (Universal Automation)
**TUI Feature Status:** 197/212 features implemented (92.9%) - See TODO_TUI.md

---

## 🎯 Strategic Vision

### **Short-Term (6 months): AI Commanding Center**
Rival Cursor/Claude Code with evidence-based AI development targeting 76-78% SWE-bench accuracy.

### **Long-Term (12+ months): Universal Automation Platform**
Expand beyond coding to email automation, web generation, business automation, project management (Kanban boards, team collaboration), and more.

---

## 📊 Development Principles

1. **Evidence-Based:** Every feature backed by research papers or production systems
2. **Atomic UI:** Small, composable, testable components (NOT 1000-line widgets)
3. **Separation of Concerns:** Business logic ≠ UI widgets (Fix P0 violations)
4. **Quality Gates:** No feature ships without tests + validation
5. **Focus:** Kill features that don't serve the current phase's mission
6. **TUI Tier System:** Basic (essential) → Medium (usability) → Advanced (standout) → Platinum (excellence)

---

## 🚀 Phase Roadmap

| Phase | Focus | Duration | TUI Features | Status |
|-------|-------|----------|--------------|--------|
| **Phase 0** | Foundation (Fix Architecture) | 2-3 weeks | N/A (refactor) | 🔴 REQUIRED |
| **Phase 1** | AI Commanding Center | 8-10 weeks | BASIC + AI PLATINUM | ⏸️ BLOCKED |
| **Phase 2** | Developer Productivity | 6-8 weeks | MEDIUM + Dev ADVANCED/PLATINUM | 📅 PLANNED |
| **Phase 3** | Automation Platform | 12+ weeks | PM PLATINUM (Kanban, etc.) | 🔮 FUTURE |

---

# Phase 0: FOUNDATION (Architecture Overhaul)

> **Goal:** Fix critical violations, implement Atomic UI, establish clean architecture
> **Duration:** 2-3 weeks
> **Success Criteria:** Tests compile and pass, clean separation of concerns, <2000 LOC for TUI

## P0: Critical Fixes (Week 1)

### 1. Fix Compilation Errors ❌ BLOCKING
**Status:** 192 errors in test code
**Impact:** Can't validate anything without tests

**Tasks:**
- [ ] Fix missing imports (`ProviderType`, `KeyCode`, `KeyModifiers`, `TableColumn`, `DataTable`)
- [ ] Resolve module visibility issues
- [ ] Run `cargo test --lib` and achieve 0 errors
- [ ] Verify all 2,572+ tests actually pass (per TODO_TUI.md)
- [ ] Update CHANGELOG with honest test count

**Time Estimate:** 2-3 days
**Owner:** Core Team
**Blocker For:** Everything else

---

### 2. Fix Separation of Concerns Violations ❌ BLOCKING
**Status:** UI widgets doing I/O, widgets stored in App state
**Impact:** Untestable code, violates Elm Architecture

**Tasks:**
- [ ] Create `src/infrastructure/filesystem.rs` service
- [ ] Move `FileTree::load_children()` I/O to filesystem service
- [ ] Move `FilePreviewManager` tokio::fs usage to filesystem service
- [ ] Extract state from widgets:
  - [ ] `InputField` → `InputState` (data) + `InputField` (view)
  - [ ] `CommandPalette` → `PaletteState` + `CommandPalette` (view)
  - [ ] `HelpScreen` → Pure view with reference data
- [ ] Update `App` struct to store state, not widgets
- [ ] Verify widgets are pure rendering functions

**Time Estimate:** 3-5 days
**Owner:** Architecture Team
**Blocker For:** Phase 1

**Quality Gate:**
- [ ] Zero `std::fs` or `tokio::fs` imports in `src/ui/widgets/`
- [ ] App struct contains only `Serialize`-able state
- [ ] All widgets can be tested without filesystem

---

### 3. Implement Atomic UI (Lightweight) ⚠️ CRITICAL
**Status:** 127 widget files with massive duplication
**Impact:** Unmaintainable, inconsistent, bloated
**Reference:** TODO_TUI.md - BASIC tier widgets already exist, need refactoring

**New Structure:**
```
src/ui/
├── atoms/              # 5-8 primitives (50-100 LOC each)
│   ├── text.rs         # Styled text primitive
│   ├── block.rs        # Reusable bordered block (refactor existing)
│   ├── button.rs       # Button with states (NEW)
│   ├── icon.rs         # Nerd font icons (use existing NerdFonts module)
│   └── input.rs        # Single-line input (refactor existing InputField)
│
├── molecules/          # 3-5 composites (100-200 LOC)
│   ├── metric_card.rs  # Label + Value + Icon (for eval metrics)
│   ├── task_item.rs    # Icon + Name + Status (for eval tasks)
│   └── progress_bar.rs # Label + Bar + Percentage (refactor existing)
│
├── organisms/          # 2-3 complex (200-400 LOC)
│   ├── eval_panel.rs   # MetricCard[] + ProgressBar + TaskItem[]
│   └── input_bar.rs    # Icon + Input + Shortcuts
│
├── screens/            # 3-4 full screens (100-200 LOC)
│   ├── welcome.rs      # Logo + Tips (refactor existing)
│   ├── main.rs         # InputBar + Content (refactor existing)
│   ├── evaluation.rs   # EvalPanel + Progress (NEW - Phase 1)
│   └── trust.rs        # Trust dialog (keep existing)
│
└── theme.rs            # Theme system (keep existing)
```

**Tasks:**
- [ ] **Week 1: Extract Atoms**
  - [ ] Create atoms module structure
  - [ ] Refactor `Block` widget → `atoms/block.rs` (pure primitive)
  - [ ] Create `atoms/button.rs` (NEW - for future use)
  - [ ] Wrap existing `NerdFonts` module as `atoms/icon.rs`
  - [ ] Extract `InputField` state → Refactor as `atoms/input.rs`
  - [ ] Write tests for each atom (5-10 tests per atom)

- [ ] **Week 2: Build Evaluation Molecules**
  - [ ] `MetricCard` (accuracy, cost, latency display)
  - [ ] `TaskItem` (task status display)
  - [ ] Refactor existing `ProgressBar` → `molecules/progress_bar.rs`
  - [ ] Test molecule composition

- [ ] **Week 2-3: Compose Organisms & Screens**
  - [ ] `EvalPanel` (combine molecules)
  - [ ] Refactor `WelcomeScreen` → `screens/welcome.rs` (use atoms)
  - [ ] Refactor `MainScreen` → `screens/main.rs` (use atoms)
  - [ ] NEW: `screens/evaluation.rs` (Phase 1 core)

- [ ] **Week 3: Archive The Bloat**
  - [ ] Create `src/ui/archived/` directory
  - [ ] Move non-essential widgets to archive:
    - **Project Management Features** (Phase 3):
      - Rich task cards, Kanban boards, dependencies
      - Team collaboration, comments, attachments
      - Automation, time tracking, achievements
      - Dashboard, analytics, calendar integration
      - Communication integrations (Slack, Discord)
    - **Developer Tools** (Phase 2):
      - Git widgets (status, diff, graph) - 72 tests
      - File management (tree, preview, operations)
      - Editor features (vim motions, macros, marks)
      - Advanced search, bookmarks, recent files
    - **Polish Features** (Phase 2):
      - Animations, sparklines, charts, canvas
      - Multi-window system, cross-window context
      - Floating windows, collapsible sections
      - Performance monitoring widgets
    - **Easter Eggs**:
      - psx_frogger.rs (851 lines)
      - demo_mode.rs (592 lines)
  - [ ] Document archived features with restoration plan
  - [ ] Update imports to remove archived widget references

**Archive Summary:**
```
Archive → Phase 3 (Automation Platform):
- Project Management: ~40 widgets (Kanban, tasks, collaboration, automation)

Archive → Phase 2 (Developer Productivity):
- Git Integration: ~6 widgets (72 tests)
- File Management: ~6 widgets
- Editor Features: ~8 widgets
- Search & Navigation: ~4 widgets
- Charts & Visualization: ~5 widgets

Delete (Non-essential):
- psx_frogger.rs (easter egg)
- demo_mode.rs (not needed for Phase 1)
- render_profiler.rs (use performance module instead)
```

**Time Estimate:** 2-3 weeks (parallel with fix work)
**Owner:** UI Team

**Quality Gate:**
- [ ] Total UI code for Phase 1: <2000 LOC (from ~12,000+)
- [ ] Maximum file size: 400 LOC
- [ ] All atoms have 5+ tests
- [ ] Consistent theme/styling across all components
- [ ] Zero duplication of border/scroll/focus logic

---

## P1: Documentation Cleanup (Week 2-3)

### 4. Honest Documentation ⚠️
**Status:** CHANGELOG claims tests pass (they don't), features marked complete that aren't
**Impact:** Loss of credibility, confusion

**Tasks:**
- [ ] Update CHANGELOG: Accurate test status (2,572 tests per TODO_TUI.md)
- [ ] Update CHANGELOG: Mark M5 as "NOT STARTED" (not "IN PROGRESS")
- [ ] Update README: Accurate implementation status
- [ ] Update TODO_TUI.md: Reflect Phase 0 archival decisions
- [ ] Create ARCHITECTURE_DEBT.md documenting known violations
- [ ] Update all "✅ Complete" markers with honest assessment

**Time Estimate:** 1 day
**Owner:** Documentation Team

---

## P2: Create Command Service (Week 3)

### 5. Command Abstraction ⚠️
**Status:** Command parsing mixed in App
**Impact:** Growing complexity, hard to test

**Tasks:**
- [ ] Create `src/commands/service.rs`
- [ ] Move parsing logic from `app_commands.rs` to service
- [ ] Implement `CommandService::parse()` and `execute()`
- [ ] Add 10+ tests for command parsing
- [ ] Update App to delegate to service

**Time Estimate:** 2-3 days
**Owner:** Core Team

---

## Phase 0 Success Criteria

**Must Have:**
- ✅ Zero compilation errors
- ✅ All tests passing (2,500+ tests documented)
- ✅ Zero I/O in UI widgets
- ✅ App state is pure data (no widgets)
- ✅ Atomic UI implemented (atoms + molecules + organisms)
- ✅ Widget count: <20 files for Phase 1
- ✅ Total UI LOC: <2000
- ✅ Honest documentation
- ✅ Archive plan executed (100+ widgets archived with restoration docs)

**Quality Gates:**
```bash
# All must pass
cargo test --lib --all          # ✅ 0 errors, 2500+ tests pass
cargo clippy -- -D warnings     # ✅ 0 warnings
cargo fmt --check               # ✅ Formatted
tokei src/ui/atoms src/ui/molecules src/ui/organisms src/ui/screens  # ✅ <2000 LOC total
find src/ui -name "*.rs" -not -path "*/archived/*" | wc -l  # ✅ <20 files active
grep "std::fs\|tokio::fs" src/ui/widgets/*.rs  # ✅ 0 matches (all archived or refactored)
```

---

# Phase 1: AI COMMANDING CENTER (Alpha Release)

> **Goal:** Ship working AI terminal that validates M0-M5 milestones
> **Duration:** 8-10 weeks
> **Target Accuracy:** 76-78% on SWE-bench Verified
> **Users:** Developers (ourselves + early adopters)
> **TUI Features:** BASIC (100%) + MEDIUM (select) + AI PLATINUM

## TUI Feature Requirements (from TODO_TUI.md)

### ✅ BASIC Tier (Already Complete - Refactor Only)
**Status:** 19/19 features (100%)
**Action:** Keep but refactor with Atomic UI

**Core Architecture:**
- [x] Elm-style Architecture (Model-Update-View) - KEEP
- [x] Terminal Detection & Setup - KEEP
- [x] Event Loop - KEEP

**Basic Rendering:**
- [x] Block Widget - REFACTOR → atoms/block.rs
- [x] Paragraph Widget - KEEP as-is
- [x] Layout System - KEEP
- [x] Status Bar - REFACTOR with atoms
- [x] Title Bar - REFACTOR with atoms
- [x] ASCII Branding - KEEP (TOAD logo)

**Basic Styling:**
- [x] Color Support - KEEP
- [x] Text Modifiers - KEEP
- [x] Border Styles - KEEP
- [x] Theme Module - KEEP (ToadTheme)

**Navigation:**
- [x] Arrow keys - KEEP
- [x] Basic Help Screen - REFACTOR
- [x] Quit Command - KEEP

**Welcome & Onboarding:**
- [x] Welcome Screen - REFACTOR with atoms/molecules
- [x] Trust Dialog - KEEP (already good)

---

### 🟡 MEDIUM Tier (Select Features Only)
**Status:** 39/39 features complete, but archive most for Phase 2
**Action:** Keep only essentials for AI commanding center

**KEEP for Phase 1:**
- [x] Input Field - REFACTOR → atoms/input.rs
- [x] Progress Bars - REFACTOR → molecules/progress_bar.rs
- [x] Modal System (dialogs) - KEEP
- [x] Vim-style Keybindings (basic) - KEEP
- [x] Configuration File - KEEP
- [x] State Persistence - KEEP (Session module)
- [x] File Logging - KEEP
- [x] Toast Notifications - KEEP
- [x] Performance Metrics - KEEP

**ARCHIVE to Phase 2:**
- [ ] ~~List Widget~~ - Defer (not needed for eval screen)
- [ ] ~~Table Widget~~ - Defer
- [ ] ~~Scrollbar~~ - Defer (keep simple for now)
- [ ] ~~Textarea~~ - Defer (use simple input for Phase 1)
- [ ] ~~Split Panes~~ - Defer
- [ ] ~~Panel Focus System~~ - Defer
- [ ] ~~Tab Switching~~ - Defer
- [ ] ~~Search (/)~~ - Defer
- [ ] ~~Autocomplete~~ - Defer

---

### 💎 PLATINUM Tier (AI-Specific Only)
**Status:** Select AI features for Phase 1, defer rest
**Action:** Keep 5-7 AI features, archive 90+ project management features

**KEEP for Phase 1 (AI Features):**
- [x] **Chat Panel** - Conversational AI interaction (refactor)
- [x] **Streaming Responses** - Real-time AI output (keep)
- [x] **Token Counter** - Usage tracking (refactor → molecule)
- [x] **Model Selector** - Switch AI models (refactor → molecule)
- [x] **Context Display** - Show what AI sees (refactor)
- [ ] **Diff View** - AI proposed changes (implement in Phase 1)
- [ ] **Accept/Reject Panel** - Code change approval (implement in Phase 1)

**ARCHIVE to Phase 2 (Developer Tools):**
- [ ] ~~Git Status Panel~~ - 72 tests, archive entire git/ module
- [ ] ~~Commit Graph~~ - Archive
- [ ] ~~Diff Viewer~~ - Archive (separate from AI Diff View)
- [ ] ~~File Tree View~~ - Archive
- [ ] ~~File Preview~~ - Archive
- [ ] ~~Syntax Highlighting~~ - Archive (keep simple for Phase 1)
- [ ] ~~Vim Modal Editing~~ - Archive (keep basic vim keys only)
- [ ] ~~Command Mode (:)~~ - Archive
- [ ] ~~Macros~~ - Archive
- [ ] ~~Marks~~ - Archive
- [ ] ~~Undo/Redo~~ - Archive
- [ ] ~~Search & Filter~~ - Archive
- [ ] ~~Bookmarks~~ - Archive
- [ ] ~~Recent Files~~ - Archive
- [ ] ~~Canvas Drawing~~ - Archive
- [ ] ~~Line/Bar/Scatter Charts~~ - Archive
- [ ] ~~Live Graphs~~ - Archive
- [ ] ~~Sparklines~~ - Archive
- [ ] ~~Animations~~ - Archive
- [ ] ~~Loading Spinners~~ - Archive (use simple indicator)
- [ ] ~~Nerd Font Icons~~ - KEEP module, use sparingly
- [ ] ~~Multi-cursor~~ - Archive
- [ ] ~~Clipboard Integration~~ - Archive
- [ ] ~~Fuzzy Finding~~ - Archive
- [ ] ~~Mouse Support~~ - Archive
- [ ] ~~Tab System~~ - Archive
- [ ] ~~Floating Windows~~ - Archive
- [ ] ~~Minimap~~ - Archive
- [ ] ~~Breadcrumbs~~ - Archive

**ARCHIVE to Phase 3 (Project Management - MASSIVE):**
- [ ] ~~Visual Kanban Board~~ - 16 tests, archive entire BoardManager
- [ ] ~~Multiple Views~~ - 18 tests, archive ViewManager
- [ ] ~~Rich Task Cards~~ - 20 tests, archive RichTaskCardManager
- [ ] ~~Task Dependencies~~ - 17 tests, archive DependencyManager
- [ ] ~~File Attachments~~ - 18 tests, archive AttachmentManager
- [ ] ~~Card Comments System~~ - 23 tests, archive CommentManager
- [ ] ~~Team Collaboration~~ - 21 tests, archive CollaborationManager
- [ ] ~~Built-in Automation~~ - 21 tests, archive AutomationManager
- [ ] ~~AI Task Intelligence~~ - 23 tests, archive AITaskIntelligence
- [ ] ~~Dashboard & Metrics~~ - 21 tests, archive DashboardMetrics
- [ ] ~~Custom Reports~~ - 25 tests, archive ReportManager
- [ ] ~~Integrated Time Tracking~~ - 24 tests, archive TimeTracker
- [ ] ~~Achievement System~~ - 24 tests, archive AchievementSystem
- [ ] ~~Projects & Workspaces~~ - 25 tests (archive, conflicts with Phase 1 workspace)
- [ ] ~~Filtering & Search~~ - 27 tests, archive FilterManager
- [ ] ~~Import/Export~~ - 30 tests, archive Importer/Exporter
- [ ] ~~Calendar Integration~~ - 24 tests, archive CalendarEvent
- [ ] ~~Communication Integrations~~ - 25 tests (Slack/Discord/Teams/Email)
- [ ] ~~Git Card Integration~~ - 26 tests, archive GitCardIntegrationManager
- [ ] ~~Cross-Window Context~~ - 22 tests, archive CrossWindowContextManager
- [ ] ~~Window Management~~ - 15 tests, archive WindowManager
- [ ] ~~GitHub OAuth Integration~~ - NOT STARTED, defer entirely

**Total Archive:** ~90+ widgets, ~500+ tests (keep passing but not active in Phase 1)

---

## P0: Core AI Validation (Weeks 1-4)

### Milestone 1.1: M0 Validation ✅
**Status:** Infrastructure complete, needs validation
**Target:** Prove evaluation framework works

**Tasks:**
- [ ] Set up `ANTHROPIC_API_KEY` in environment
- [ ] Run M1 baseline evaluation (10 tasks)
  ```bash
  cargo run --release -- eval --count 10 --milestone 1
  ```
- [ ] Verify no crashes, collect metrics
- [ ] Run M1 extended evaluation (50 tasks)
- [ ] Document baseline accuracy (target: 55-60%)
- [ ] Identify failure patterns
- [ ] Create M0_VALIDATION_RESULTS.md

**Time Estimate:** 1 week
**Success Criteria:** Baseline measured, framework validated

---

### Milestone 1.2: M2 Validation (AST + Smart Tests) ✅
**Status:** Implementation complete, needs A/B test
**Target:** +5% accuracy vs M1 (60-65%)

**Tasks:**
- [ ] Run A/B test: M1 vs M2 (30 tasks minimum)
  ```bash
  cargo run --release -- compare --baseline 1 --test 2 --count 30
  ```
- [ ] Statistical validation (p < 0.05, Cohen's d)
- [ ] Decision: Adopt/Reject/Investigate
- [ ] If Adopt: Update baseline to M2
- [ ] Document results in M2_VALIDATION_RESULTS.md

**Time Estimate:** 1 week
**Success Criteria:** +2% accuracy improvement with statistical significance

---

### Milestone 1.3: M3 Validation (Racing) ✅
**Status:** Implementation complete, needs A/B test
**Target:** +5-7% accuracy vs M1 (63-68%)

**Tasks:**
- [ ] Configure racing models (Sonnet 4 + Sonnet 3.5)
- [ ] Run A/B test: M2 vs M3 (50 tasks)
- [ ] Analyze latency improvements (target: 20-40% P50)
- [ ] Cost analysis (vs M2)
- [ ] Decision: Adopt/Reject/Investigate
- [ ] Document results in M3_VALIDATION_RESULTS.md

**Time Estimate:** 1 week
**Success Criteria:** +4% accuracy (TRAE paper baseline), latency improvement

---

### Milestone 1.4: M4 Validation (Cascading) ✅
**Status:** Implementation complete, needs A/B test
**Target:** 70% cost reduction, maintain/improve accuracy

**Tasks:**
- [ ] Install Ollama + qwen2.5-coder models (7B, 32B)
  ```bash
  ollama pull qwen2.5-coder:7b
  ollama pull qwen2.5-coder:32b
  ```
- [ ] Run A/B test: M3 vs M4 (50 tasks)
- [ ] Cost analysis (target: 70-80% reduction vs cloud-only)
- [ ] Accuracy validation (maintain M3 level)
- [ ] Document results in M4_VALIDATION_RESULTS.md

**Time Estimate:** 1 week
**Success Criteria:** 70% cost reduction with no accuracy loss

---

### Milestone 1.5: M5 Feature Development 🔜
**Status:** NOT STARTED
**Target:** +3-5% accuracy (76-78% total)

**Features to Implement:**

#### 1. Vector Embeddings (context_embeddings)
**Evidence:** RAG systems show 15-20% context improvement
**Tasks:**
- [ ] Add `async-openai` dependency for text-embedding-3-small
- [ ] Implement `EmbeddingClient` trait
- [ ] Create `InMemoryVectorStore`
- [ ] Integrate with `ContextBuilder`
- [ ] 10 tests (embedding generation, similarity, search)
- [ ] A/B test vs M4

**Time Estimate:** 1 week

---

#### 2. Code Graph Analysis (context_graph)
**Evidence:** Dependency-aware context improves cross-file changes
**Tasks:**
- [ ] Add `petgraph` dependency
- [ ] Implement `CodeGraph` structure
- [ ] AST → Graph builder
- [ ] Dependency resolution (imports, calls)
- [ ] Impact analysis for changes
- [ ] 12 tests (graph construction, dependencies, impact)
- [ ] A/B test vs M4+Embeddings

**Time Estimate:** 1-2 weeks

---

#### 3. Semantic Caching (semantic_caching)
**Evidence:** 40% cost reduction on repeated queries
**Tasks:**
- [ ] Implement `SemanticCache` using embeddings
- [ ] Similarity threshold tuning
- [ ] TTL support
- [ ] Integration with `LLMClient`
- [ ] 6 tests (cache hit/miss, similarity, TTL)
- [ ] Measure cost reduction in real usage

**Time Estimate:** 3-5 days

---

#### 4. Failure Memory (failure_memory)
**Evidence:** 30% fewer repeat errors in AutoGPT
**Tasks:**
- [ ] Implement `FailureMemory` with JSON persistence
- [ ] Error pattern categorization
- [ ] Similar failure detection (using embeddings)
- [ ] Integration with `PromptBuilder`
- [ ] 6 tests (storage, retrieval, similarity)
- [ ] A/B test for error reduction

**Time Estimate:** 3-5 days

---

#### 5. Context Re-ranking (context_reranking)
**Evidence:** Cohere Rerank improves relevance by 25%
**Tasks:**
- [ ] Add `cohere-rust` dependency (optional)
- [ ] Implement `Reranker` trait
- [ ] `CohereReranker` + `EmbeddingReranker` fallback
- [ ] Integration with `ContextBuilder`
- [ ] 6 tests (ranking, relevance, fallback)
- [ ] A/B test vs M4+All

**Time Estimate:** 3-5 days

---

**Total M5 Time Estimate:** 4-6 weeks
**Success Criteria:** 76-78% accuracy on SWE-bench Verified

---

## P1: Simple TUI (Weeks 5-8)

### Milestone 1.6: AI Commanding Center TUI 🎯
**Status:** Atomic UI ready (from Phase 0), TUI features archived
**Target:** Minimal, focused interface for M0-M5 evaluation
**Reference:** TODO_TUI.md BASIC tier (refactored with Atomic UI)

**Features:**

#### 1. Welcome Screen (Atom-Based)
**Status:** Refactor existing with atoms
**LOC:** Existing 377 lines → Target <100 lines
**Tasks:**
- [ ] Logo (using `theme.rs` existing logo)
- [ ] Quick start tips (3-5 tips from existing StartupTips widget)
- [ ] "Press any key to continue"
- [ ] Uses: `atoms/text.rs`, `atoms/block.rs`

**LOC Target:** <100 lines
**Time Estimate:** 1 day

---

#### 2. Main Screen (Input + Status)
**Status:** Refactor existing with atoms/molecules
**LOC:** Existing complexity → Target <150 lines
**Tasks:**
- [ ] Top area: Status message + metadata (path, model)
- [ ] Input bar (using `atoms/input.rs` + `molecules/input_bar.rs`)
- [ ] Placeholder: "eval --count 10 --milestone 1" examples
- [ ] Keyboard shortcuts bar at bottom
- [ ] Uses: `atoms/input.rs`, `atoms/text.rs`, `molecules/input_bar.rs`

**LOC Target:** <150 lines
**Time Estimate:** 2 days

---

#### 3. Evaluation Screen (Real-time Progress) ⭐ **NEW - CORE VALUE**
**Status:** NEW - this is THE killer feature
**Components:**
- [ ] Header: "Evaluating M2 vs M1 (Task 5/30)"
- [ ] `MetricCard` molecules for:
  - Current accuracy: "67.5% (5/8 solved)"
  - Total cost: "$2.34 / ~$14.00 est"
  - Time elapsed: "3m 24s / ~18m est"
  - Agent steps: "Step 12/25"
- [ ] `ProgressBar` molecule: Visual progress bar (####-----)
- [ ] `TaskItem` molecules: Scrollable list of tasks
  - ✅ task-1: Fixed import bug (0.5s, $0.02)
  - ⏳ task-2: Refactoring class... (current)
  - ⏸️ task-3: Pending
- [ ] Bottom bar: "Press Ctrl+C to cancel | Esc for details"
- [ ] Real-time updates from async evaluation runner

**Uses:**
- `molecules/metric_card.rs` (NEW)
- `molecules/task_item.rs` (NEW)
- `molecules/progress_bar.rs` (refactored from existing)
- `organisms/eval_panel.rs` (NEW - composes above)
- `screens/evaluation.rs` (NEW)

**LOC Target:** <300 lines total
**Time Estimate:** 3-4 days

---

#### 4. Results Screen (Post-Evaluation)
**Status:** NEW
**Components:**
- [ ] Summary panel:
  - Final accuracy: "62.5% (15/24 solved)"
  - Statistical significance: "p = 0.032 (significant) ✅"
  - Recommendation: "ADOPT M2 (+7.5% vs baseline)"
  - Total cost: "$14.23"
  - Duration: "18m 32s"
- [ ] Detailed task breakdown (scrollable)
- [ ] Actions: "S: Save report | R: Retry failed | Q: Quit"

**Uses:** Same molecules as evaluation screen
**LOC Target:** <200 lines
**Time Estimate:** 2 days

---

#### 5. Trust Dialog (Keep Existing)
**Status:** Already implemented, good quality
**Tasks:**
- [ ] Verify still works with new architecture
- [ ] Minor styling updates for consistency with atoms

**Time Estimate:** 1 hour

---

#### 6. AI Diff View + Accept/Reject (NEW for Phase 1)
**Status:** Widgets exist in archive (AIDiffView, AcceptRejectPanel)
**Action:** Restore and integrate
**Tests:** 7 + 11 = 18 tests already exist
**Tasks:**
- [ ] Restore `AIDiffView` widget (unified/side-by-side modes)
- [ ] Restore `AcceptRejectPanel` (accept/reject per hunk)
- [ ] Integrate into evaluation flow (show AI-proposed changes)
- [ ] Add keyboard shortcuts (a=accept, r=reject, n=next hunk)

**LOC Target:** ~300 lines (already implemented)
**Time Estimate:** 2 days (integration only)

---

**Total TUI LOC:** ~850 lines (vs original 12,000+)
**Total Time:** 2 weeks
**Archive Status:** 100+ widgets archived with 500+ passing tests preserved

**Quality Gates:**
- [ ] Frame rate: 60 FPS
- [ ] Memory usage: <50MB
- [ ] Startup time: <100ms
- [ ] All keyboard shortcuts work
- [ ] Cross-platform tested (Linux, macOS, Windows)
- [ ] Real-time evaluation progress updates work

---

## P2: Core Commands (Week 9)

### Milestone 1.7: Essential Commands Only
**Status:** Refactor existing
**Target:** Minimal command set for AI work

**Commands:**

1. **`eval --count N --milestone M [--swebench verified|lite]`**
   - Status: ✅ Implemented
   - Action: Verify works with new TUI

2. **`compare --baseline M1 --test M2 --count N [--swebench verified|lite]`**
   - Status: ✅ Implemented
   - Action: Verify works with new TUI

3. **`show-config --milestone M`**
   - Status: ✅ Implemented
   - Action: Integrate into TUI results screen

4. **`/help`**
   - Status: ✅ Implemented
   - Action: Update help text for Phase 1 focus

5. **`/quit` or Ctrl+C**
   - Status: ✅ Implemented
   - Action: Verify graceful shutdown

**Time Estimate:** 2-3 days
**Deferred Commands:** All non-essential commands removed or archived

---

## P3: Documentation (Week 10)

### Milestone 1.8: User Documentation
**Status:** NEW
**Target:** Enable early adopters

**Documents to Create:**

1. **QUICKSTART.md**
   - Install Rust + dependencies
   - Clone repo
   - Set `ANTHROPIC_API_KEY`
   - Run first evaluation
   - Interpret results

2. **USER_GUIDE.md**
   - TUI navigation
   - Command reference
   - Understanding metrics
   - A/B testing workflow
   - Troubleshooting

3. **CONTRIBUTING.md**
   - Code standards
   - Atomic UI guidelines
   - Testing requirements
   - PR process

4. **BENCHMARK_RESULTS.md**
   - M0-M5 validation results
   - Evidence for each milestone
   - Cost analysis
   - Accuracy progression graph

**Time Estimate:** 3-4 days

---

## Phase 1 Success Criteria

**Must Have:**
- ✅ M0-M4 validated with real results
- ✅ M5 features implemented and tested
- ✅ Target accuracy achieved: 76-78% on SWE-bench
- ✅ TUI works flawlessly (<850 LOC active)
- ✅ 4 core commands work
- ✅ Documentation complete
- ✅ 5+ early adopters using it
- ✅ Archive system works (100+ widgets preserved but inactive)

**Quality Gates:**
```bash
# M5 Target Accuracy
cargo run --release -- eval --swebench verified --count 100 --milestone 5
# Expected: 76-78% accuracy

# Cost Efficiency
# Expected: <$1 per task with caching + cascading

# Performance
# TUI: 60 FPS, <50MB RAM, <100ms startup

# Tests
cargo test --all  # ✅ 2,500+ tests passing (including archived features)

# Active Code
tokei src/ui/atoms src/ui/molecules src/ui/organisms src/ui/screens
# Expected: <850 LOC active UI code
```

**Launch Checklist:**
- [ ] GitHub release v0.1.0-alpha
- [ ] Blog post: "We Built an AI Terminal That Beats Cursor"
- [ ] Reddit post: r/rust, r/MachineLearning
- [ ] Hacker News submission
- [ ] 5-10 invited beta testers

---

# Phase 2: DEVELOPER PRODUCTIVITY (Beta Release)

> **Goal:** Expand beyond benchmarks to daily developer use
> **Duration:** 6-8 weeks
> **Target Users:** 100-500 developers
> **Focus:** Real-world coding tasks, not just SWE-bench
> **TUI Features:** Restore MEDIUM + Developer ADVANCED/PLATINUM from archive

## Restore from Archive (Week 1)

### Milestone 2.0: Restore Core Developer Widgets
**Status:** Archived in Phase 0, need restoration
**Action:** Unarchive and integrate ~25 developer-focused widgets

**Restore List:**
- [ ] **Git Integration** (6 widgets, 72 tests):
  - GitStatusPanel, CommitGraph, DiffViewer
  - GitStageUI, GitCommitDialog, GitBranchManager

- [ ] **File Management** (6 widgets):
  - FileTree, FilePreviewManager, FileOps
  - BookmarkManager, RecentFiles

- [ ] **Editor Features** (8 widgets):
  - Textarea (multi-line), VimMotions, MacroManager
  - MarksManager, UndoStack, MultiCursor
  - CommandMode, AliasManager

- [ ] **Search & Navigation** (4 widgets):
  - FuzzyFinder, AdvancedSearch, FilterManager
  - Breadcrumbs

- [ ] **Layout Features** (5 widgets):
  - Split panes, TabManager, TabBar
  - FloatingWindows, CollapsibleSections

**Time Estimate:** 1 week (integration testing)

---

## P0: Essential Developer Features (Weeks 2-5)

### Milestone 2.1: Git Integration 🔧
**Status:** Widgets exist (archived in Phase 0), 72 tests passing
**Target:** Visual git workflow in TUI

**Features:**
- [ ] Restore GitStatusPanel (19 tests)
- [ ] Restore CommitGraph + GitGraphService (43 tests)
- [ ] Restore DiffViewer (10 tests)
- [ ] Stage/unstage files (GitStageUI)
- [ ] Commit with message (GitCommitDialog)
- [ ] Branch switching (GitBranchManager)
- [ ] Merge conflict helper (ConflictResolver - 10 tests)

**Time Estimate:** 2 weeks
**Tests:** 72 existing + 10 integration

---

### Milestone 2.2: File Browser 📁
**Status:** FileTree exists but violates SoC (does I/O)
**Target:** Fast, keyboard-driven file navigation

**Tasks:**
- [ ] Refactor `FileTree` to use `FilesystemService` (from Phase 0)
- [ ] Pure state-based tree widget
- [ ] Keyboard navigation (vim keys)
- [ ] Restore FuzzyFinder integration (archived)
- [ ] File preview pane (restore FilePreviewManager)
- [ ] Quick file open (Ctrl+P)

**Time Estimate:** 1-2 weeks
**Tests Required:** 15+

---

### Milestone 2.3: Enhanced Chat Mode 💬
**Status:** `ChatPanel` widget exists (archived)
**Target:** Conversational AI for coding

**Features:**
- [ ] Restore `ChatPanel` (refactor with atomic UI if needed)
- [ ] Streaming responses (word-by-word) - already exists
- [ ] Code block syntax highlighting (use existing SyntaxHighlighter)
- [ ] Copy code to clipboard (restore Clipboard integration)
- [ ] Chat history persistence
- [ ] Context from open files
- [ ] @-mention files for context (NEW feature)

**Time Estimate:** 2 weeks
**Tests Required:** 12+

---

### Milestone 2.4: Code Editor Widget 📝
**Status:** Partial (textarea exists, needs enhancement)
**Target:** Vim-mode editing in TUI

**Features:**
- [ ] Restore vim motion support (VimMotions exists)
- [ ] Syntax highlighting (tree-sitter based - already exists)
- [ ] Line numbers + column indicator
- [ ] Search and replace (regex)
- [ ] Multi-cursor support (MultiCursor exists)
- [ ] Undo/redo (UndoStack exists)

**Restore from Archive:**
- VimMotions, MacroManager, MarksManager
- MultiCursor, UndoStack
- Textarea widget (enhanced version)

**Time Estimate:** 2-3 weeks
**Tests Required:** 25+

---

## P1: Quality of Life (Weeks 6-7)

### Milestone 2.5: Session Management 💾
**Status:** `SessionState` exists in workspace module
**Target:** Resume work seamlessly

**Features:**
- [ ] Auto-save session state (already exists)
- [ ] Restore open files + chat history
- [ ] Multiple workspaces (WorkspaceManager exists - archived)
- [ ] Session switcher
- [ ] Export/import sessions

**Time Estimate:** 1 week
**Tests Required:** 10+

---

### Milestone 2.6: Search & Navigation 🔍
**Status:** Infrastructure exists (archived)
**Target:** Find anything fast

**Features:**
- [ ] Fuzzy file finder (Ctrl+P) - restore FuzzyFinder
- [ ] Global code search (Ctrl+Shift+F) - restore AdvancedSearch
- [ ] Symbol search (classes, functions) - NEW
- [ ] Go to definition - NEW
- [ ] Find usages - NEW
- [ ] Navigation history (back/forward) - NEW

**Time Estimate:** 1-2 weeks
**Tests Required:** 15+

---

### Milestone 2.7: Configuration UI ⚙️
**Status:** Config system exists (TOML-based)
**Target:** Easy customization

**Features:**
- [ ] Settings screen in TUI (NEW)
- [ ] Theme picker (use existing ThemeManager)
- [ ] Keybinding customization (use CustomKeybindings)
- [ ] AI model configuration
- [ ] Performance settings
- [ ] Live preview of changes

**Time Estimate:** 1 week
**Tests Required:** 8+

---

## P2: Advanced Features (Week 8)

### Milestone 2.8: Smart Suggestions 💡
**Status:** SmartSuggestions widget exists (archived)
**Target:** Proactive AI assistance

**Features:**
- [ ] Restore SmartSuggestions widget
- [ ] Inline code suggestions (like Copilot) - NEW
- [ ] Error explanation + fixes - NEW
- [ ] Refactoring suggestions - NEW
- [ ] Performance optimization tips - NEW
- [ ] Security vulnerability detection - NEW

**Time Estimate:** 2 weeks
**Tests Required:** 10+

---

### Milestone 2.9: Terminal Integration 🖥️
**Status:** BashTool exists for agent
**Target:** Run commands without leaving TUI

**Features:**
- [ ] Embedded terminal pane (NEW)
- [ ] Command history (use existing History)
- [ ] Split terminal views
- [ ] Quick commands (npm test, cargo build)
- [ ] Output streaming

**Time Estimate:** 1-2 weeks
**Tests Required:** 12+

---

## Phase 2 Success Criteria

**Must Have:**
- ✅ Git integration works smoothly
- ✅ File browser is fast and intuitive
- ✅ Chat mode provides real value
- ✅ Code editor is usable for daily work
- ✅ 100+ active users (dogfooding + community)
- ✅ All archived developer widgets restored and working

**Quality Gates:**
- [ ] User surveys: 8/10+ satisfaction
- [ ] Daily active usage: 50+ developers
- [ ] GitHub stars: 500+
- [ ] Community contributions: 5+ PRs
- [ ] All 2,500+ tests still passing

**Launch Checklist:**
- [ ] GitHub release v0.2.0-beta
- [ ] Demo videos (YouTube)
- [ ] Documentation updates
- [ ] Community Discord server
- [ ] Blog: "TOAD Beta: Daily Driver for Developers"

---

# Phase 3: AUTOMATION PLATFORM (Public Release)

> **Goal:** Universal automation tool for developers and businesses
> **Duration:** 12+ weeks
> **Target Users:** 10,000+ users
> **Vision:** "If it's repetitive, TOAD can automate it"
> **TUI Features:** Restore PROJECT MANAGEMENT PLATINUM tier from archive

## Restore from Archive (Week 1)

### Milestone 3.0: Restore Project Management Suite
**Status:** Archived in Phase 0, 500+ tests passing
**Action:** Unarchive and integrate ~40 project management widgets

**Restore List:**
- [ ] **Kanban System** (40+ widgets, 300+ tests):
  - BoardManager (16 tests)
  - ViewManager (18 tests) - 6 view types
  - RichTaskCardManager (20 tests)
  - DependencyManager (17 tests)
  - AttachmentManager (18 tests)
  - CommentManager (23 tests)
  - CollaborationManager (21 tests)
  - AutomationManager (21 tests)
  - AITaskIntelligence (23 tests)
  - DashboardMetrics (21 tests)
  - ReportManager (25 tests)
  - TimeTracker (24 tests)
  - AchievementSystem (24 tests)
  - ProjectManager (25 tests)
  - FilterManager (27 tests)
  - Importer/Exporter (30 tests)
  - CalendarEvent (24 tests)
  - IntegrationManager (25 tests) - Slack/Discord/Teams/Email
  - GitCardIntegrationManager (26 tests)
  - CrossWindowContextManager (22 tests)
  - WindowManager (15 tests)

**Time Estimate:** 1-2 weeks (integration testing + UI refresh)

---

## P0: Platform Infrastructure (Weeks 2-5)

### Milestone 3.1: Plugin System 🔌
**Status:** PluginManager exists (23 tests, archived)
**Target:** Extensible architecture

**Features:**
- [ ] Restore PluginManager (already has 5 runtime types, 8 capabilities, 10 hooks)
- [ ] Plugin marketplace (web UI) - NEW
- [ ] Sandboxed execution (already supported)
- [ ] Version management - NEW
- [ ] Plugin discovery - NEW

**Examples:**
- Email plugin
- Web scraping plugin
- Database plugin
- API client generators

**Time Estimate:** 3-4 weeks
**Tests:** 23 existing + 20 new

---

### Milestone 3.2: Workflow Engine 🔄
**Status:** Partial (AutomationManager exists with 12 triggers, 11 actions)
**Target:** Chain actions into automations

**Features:**
- [ ] Visual workflow builder (TUI) - NEW
- [ ] Trigger system (schedule, event-based) - EXISTS (restore)
- [ ] Action library (send email, scrape web, etc.) - EXTEND
- [ ] Conditional logic (if/else) - NEW
- [ ] Loop support - NEW
- [ ] Error handling + retries - NEW
- [ ] Workflow templates - NEW

**Time Estimate:** 3-4 weeks
**Tests:** 21 existing + 30 new

---

## P1: Business Automation (Weeks 6-9)

### Milestone 3.3: Email Automation 📧
**Status:** EmailConfig exists in IntegrationManager (archived)
**Target:** Smart email management

**Features:**
- [ ] Email client integration (IMAP/SMTP) - EXISTS (restore)
- [ ] AI-powered email drafting - NEW
- [ ] Auto-categorization - NEW
- [ ] Template management - NEW
- [ ] Scheduled sending - NEW
- [ ] Bulk email with personalization - NEW
- [ ] Response suggestions - NEW

**Time Estimate:** 2 weeks
**Tests Required:** 15+

---

### Milestone 3.4: Web Generation 🌐
**Status:** NEW
**Target:** Build websites from descriptions

**Features:**
- [ ] Natural language → HTML/CSS/JS
- [ ] Template library (landing pages, portfolios, blogs)
- [ ] Responsive design auto-generation
- [ ] Preview + edit in TUI
- [ ] Export to static files
- [ ] Deploy to Netlify/Vercel integration

**Time Estimate:** 3-4 weeks
**Tests Required:** 20+

---

### Milestone 3.5: Data Processing 📊
**Status:** Partial (CSV/JSON export exists in Exporter)
**Target:** Transform and analyze data

**Features:**
- [ ] CSV/JSON/Excel import - EXISTS (restore)
- [ ] AI-powered data cleaning - NEW
- [ ] Transformation pipeline builder - NEW
- [ ] Visualization generation (use archived charts) - EXISTS (restore)
- [ ] Export to multiple formats - EXISTS (restore)
- [ ] Database integration (SQL) - NEW

**Time Estimate:** 2-3 weeks
**Tests Required:** 18+

---

## P2: Advanced Automation (Weeks 10-13)

### Milestone 3.6: API Automation 🔗
**Status:** NEW
**Target:** Integrate any API without coding

**Features:**
- [ ] API explorer (discover endpoints)
- [ ] Request builder (no code)
- [ ] Response mapping
- [ ] Authentication handling (OAuth, JWT, API keys)
- [ ] Rate limiting + retry logic
- [ ] Webhook receiver
- [ ] API documentation generator

**Time Estimate:** 2 weeks
**Tests Required:** 15+

---

### Milestone 3.7: Content Creation ✍️
**Status:** NEW
**Target:** Generate marketing content

**Features:**
- [ ] Blog post generation
- [ ] Social media post drafting
- [ ] Image generation integration (DALL-E, Midjourney)
- [ ] SEO optimization
- [ ] Multi-platform formatting
- [ ] Content calendar

**Time Estimate:** 2 weeks
**Tests Required:** 10+

---

### Milestone 3.8: Business Intelligence 📈
**Status:** Partial (DashboardMetrics exists with 8 chart types)
**Target:** Insights from data

**Features:**
- [ ] Connect to data sources (DB, APIs, files) - NEW
- [ ] AI-powered query generation (natural language → SQL) - NEW
- [ ] Chart generation - EXISTS (restore DashboardMetrics)
- [ ] Dashboard builder - EXISTS (restore)
- [ ] Report scheduling - EXISTS (restore ReportManager)
- [ ] Anomaly detection - NEW

**Time Estimate:** 3 weeks
**Tests:** 21 existing + 20 new

---

### Milestone 3.9: Project Management (Kanban & Beyond) 📋
**Status:** Fully implemented (archived, 300+ tests)
**Target:** Restore best-in-class Kanban system

**Features (Restore from Archive):**
- [ ] **Visual Kanban Board** (BoardManager - 16 tests)
  - Customizable columns, WIP limits, swimlanes
  - Drag & drop card movement

- [ ] **Multiple Views** (ViewManager - 18 tests)
  - Kanban, List, Calendar, Timeline/Gantt, Table, Mind Map

- [ ] **Rich Task Cards** (RichTaskCardManager - 20 tests)
  - Markdown descriptions, subtasks, assignees
  - Due dates, priorities, tags, effort estimation
  - Progress bars, custom fields, cover images

- [ ] **Task Dependencies** (DependencyManager - 17 tests)
  - Blocks, BlockedBy, RelatesTo, Duplicates
  - Critical path calculation, topological sort

- [ ] **Collaboration** (CommentManager + CollaborationManager - 44 tests)
  - Threaded comments, @mentions, reactions
  - Team members, permissions, watchers
  - Activity feed, notifications

- [ ] **Automation** (AutomationManager - 21 tests)
  - When/then rules, recurring tasks
  - Task templates, bulk actions

- [ ] **Time Tracking** (TimeTracker - 24 tests)
  - Start/stop timer, manual entry
  - Billable hours, timesheet view

- [ ] **Analytics** (DashboardMetrics + ReportManager - 46 tests)
  - Cumulative flow, cycle time, lead time
  - Velocity, WIP, burndown/burnup charts
  - Custom reports, export options

- [ ] **Gamification** (AchievementSystem - 24 tests)
  - Badges, streaks, leaderboards
  - Points system, hidden achievements

- [ ] **Integrations** (IntegrationManager - 25 tests)
  - Slack, Discord, Microsoft Teams, Email
  - Webhook management, event filtering

- [ ] **Calendar Integration** (CalendarEvent - 24 tests)
  - iCal/Google Calendar export
  - Recurring events, priority colors

**Time Estimate:** 2-3 weeks (restoration + UI polish)
**Tests:** 300+ existing tests

---

### Milestone 3.10: GitHub Integration (Optional) 🐙
**Status:** NOT STARTED (planned in TODO_TUI.md)
**Target:** Full GitHub workflow in TOAD

**Features (from TODO_TUI.md):**
- [ ] OAuth authentication
- [ ] GitHub Projects import/sync
- [ ] Complete issue management (create, edit, close, labels, milestones)
- [ ] Complete PR management (create, review, merge, checks)
- [ ] Repository management
- [ ] Branch management
- [ ] Releases & tags
- [ ] GitHub Actions (workflow runs, logs, triggers)
- [ ] Discussions, sponsors, security alerts
- [ ] Real-time updates via webhooks
- [ ] Smart card enrichment (auto-fetch PR details)

**Time Estimate:** 4-6 weeks (large feature)
**Tests Required:** 50+
**Note:** This is a major feature set that could be a Phase 4 on its own

---

## Phase 3 Success Criteria

**Must Have:**
- ✅ Plugin ecosystem with 10+ plugins
- ✅ 50+ workflow templates
- ✅ Email automation works for 1000+ users
- ✅ Web generation produces production-ready sites
- ✅ Kanban system fully functional with all features
- ✅ 10,000+ active users

**Quality Gates:**
- [ ] Plugin marketplace: 50+ plugins
- [ ] Community-created workflows: 200+
- [ ] Revenue: $10k+ MRR (premium features)
- [ ] GitHub stars: 5,000+
- [ ] Featured on Product Hunt (Top 5)
- [ ] All 2,500+ tests still passing

**Launch Checklist:**
- [ ] GitHub release v1.0.0
- [ ] Product Hunt launch
- [ ] Paid tier: $20/month (advanced features)
- [ ] Documentation: 100+ guides
- [ ] Video tutorials: 20+ videos
- [ ] Case studies: 10+ businesses using TOAD

---

## 🎨 Future Explorations (Phase 4+)

### Potential Features (Not Committed)
- [ ] Mobile app (iOS/Android) for workflow monitoring
- [ ] Cloud-hosted version (TOAD Cloud)
- [ ] Team collaboration features
- [ ] Integration marketplace revenue sharing
- [ ] White-label licensing for enterprises
- [ ] AI model fine-tuning on user workflows
- [ ] Multi-agent system (specialized agents)
- [ ] Voice control integration
- [ ] AR/VR interface experiments
- [ ] GitHub OAuth Integration (full suite from TODO_TUI.md)

---

## 📊 Success Metrics by Phase

| Metric | Phase 0 | Phase 1 | Phase 2 | Phase 3 |
|--------|---------|---------|---------|---------|
| **Users** | Dev team | 5-10 early | 100-500 | 10,000+ |
| **GitHub Stars** | - | 50+ | 500+ | 5,000+ |
| **Tests Passing** | 2,500+ | 2,500+ | 2,500+ | 2,500+ |
| **Active UI LOC** | - | <850 | ~2,000 | ~4,000 |
| **Archived LOC** | - | ~11,000 | ~10,000 | ~8,000 |
| **Code Quality** | Tests broken | All passing | CI/CD | Production |
| **Revenue** | $0 | $0 | $0 | $10k+ MRR |
| **SWE-bench Accuracy** | Unknown | 76-78% | 76-78% | 76-78% |
| **Cost per Task** | Unknown | <$1 | <$1 | <$1 |

---

## 🚦 Risk Management

### High-Risk Items
1. **Phase 0 delays** → Blocks everything
   - Mitigation: Parallel work where possible, strict time-boxing
2. **M5 accuracy doesn't reach 76-78%**
   - Mitigation: Evidence-based features, A/B test each
3. **TUI performance issues**
   - Mitigation: Atomic UI reduces complexity, performance budgets
4. **Plugin ecosystem doesn't take off**
   - Mitigation: Build 10 core plugins ourselves, seed marketplace

### Medium-Risk Items
1. **API costs too high**
   - Mitigation: Cascading routing (M4), semantic caching
2. **Community adoption slow**
   - Mitigation: Aggressive marketing, demo videos, open-source
3. **Competition from Cursor/Claude Code**
   - Mitigation: Focus on evidence-based differentiation

---

## 🎯 Definition of Done (Per Milestone)

**Code:**
- [ ] Implementation complete
- [ ] All tests passing
- [ ] Zero clippy warnings
- [ ] Code reviewed by 1+ team member
- [ ] Rustdoc complete

**Quality:**
- [ ] Performance benchmarks met
- [ ] Manual testing complete
- [ ] Cross-platform tested (Linux, macOS, Windows)
- [ ] Security audit (if applicable)

**Documentation:**
- [ ] User guide updated
- [ ] CHANGELOG updated
- [ ] Architecture docs updated (if structural changes)
- [ ] Examples provided

**Validation:**
- [ ] A/B tested (if applicable)
- [ ] User feedback collected (5+ users)
- [ ] Metrics tracked in dashboard

---

## 📅 Timeline Summary

| Phase | Start Date | End Date | Duration | Key Deliverable |
|-------|------------|----------|----------|-----------------|
| **Phase 0** | 2025-11-11 | 2025-11-29 | 2-3 weeks | Architecture fixed, Atomic UI, tests passing |
| **Phase 1** | 2025-12-02 | 2026-02-14 | 8-10 weeks | AI Commanding Center (76-78% accuracy) |
| **Phase 2** | 2026-02-17 | 2026-04-11 | 6-8 weeks | Developer productivity tools |
| **Phase 3** | 2026-04-14 | 2026-07-11 | 12+ weeks | Automation platform + Kanban |

**First Public Release:** Q1 2026 (Phase 1 Complete)
**Platform Launch:** Q3 2026 (Phase 3 Complete)

---

## 🤝 Team Structure (Recommended)

**Phase 0-1 (Minimum Viable):**
- 1-2 Core Developers (Rust + AI)
- 1 UI/UX Designer (part-time for Atomic UI)
- 1 DevOps/Testing (CI/CD setup)

**Phase 2 (Growth):**
- Add 1-2 Feature Developers
- Add 1 Community Manager
- Add 1 Technical Writer

**Phase 3 (Scale):**
- Add 2-3 Backend Engineers (plugin system, cloud)
- Add 1 Product Manager
- Add 1 Marketing Lead

---

## 📚 References

**Evidence-Based Research:**
- TRAE Paper (2024): Multi-model racing
- DavaJ (2024): Cascading routing
- AutoCodeRover: Smart test selection
- Aider: Prompt engineering
- SWE-bench: Evaluation framework
- Atomic Design (Brad Frost): UI component architecture

**TUI Inspiration:**
- Lazygit, gitui: Git TUIs with excellent UX
- bottom/btm: System monitoring with beautiful graphs
- yazi: File manager with plugin system
- TODO_TUI.md: Comprehensive feature checklist (197/212 complete)

**Competitive Analysis:**
- Cursor: AI-first IDE
- Claude Code: Conversational coding
- GitHub Copilot: Inline suggestions
- Windsurf: Multi-agent system

---

## ✅ Phase 0 Next Actions (This Week)

**Monday:** Fix compilation errors (192 errors) → Owner: @core-team
**Tuesday:** Create FilesystemService, move I/O out of widgets → Owner: @architecture
**Wednesday:** Start Atomic UI extraction (atoms module) → Owner: @ui-team
**Thursday:** Extract state from widgets (InputState, etc.) → Owner: @architecture
**Friday:** Documentation cleanup + archive plan → Owner: @docs-team

**Daily Standup:** 9 AM (async in Discord)
**Weekly Review:** Friday 4 PM
**Roadmap Check-in:** Every 2 weeks

---

## 🗂️ TUI Feature Mapping Reference

### Already Implemented (from TODO_TUI.md)

**Phase 1 (Keep Active):**
- ✅ BASIC: 19/19 (100%) - Core architecture, widgets, theme
- ✅ AI PLATINUM: 7 features (Chat, Streaming, Token Counter, Model Selector, Context Display, Diff View, Accept/Reject)

**Phase 2 (Archived, Restore Later):**
- ✅ MEDIUM: 39/39 (100%) - Panels, modals, search, state
- ✅ ADVANCED: 48/48 (100%) - Themes, input, fuzzy, performance, syntax
- ✅ Dev PLATINUM: ~25 widgets (Git, file mgmt, editor, search)

**Phase 3 (Archived, Restore Later):**
- ✅ PM PLATINUM: ~40 widgets (Kanban, tasks, collaboration, automation, analytics)

**Total:** 197/212 features (92.9% implemented, strategically archived)

---

**Last Updated:** 2025-11-10
**Next Review:** 2025-11-24 (Phase 0 completion check)
**Roadmap Owner:** @product-team

---

## 🎉 Vision Statement

> "TOAD will be the **terminal that thinks**—starting as the world's most accurate AI coding assistant (76-78% SWE-bench), evolving into the **universal automation platform** with best-in-class Kanban boards and team collaboration that empowers developers and businesses to automate anything repetitive. Evidence-based, brutally focused, and built to last."

Let's build it. 🐸
