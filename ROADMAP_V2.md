# TOAD Roadmap V2: Evidence-Based AI Terminal + Automation Platform

> **Mission:** Build the world's best AI coding terminal, then expand to a universal automation platform

**Last Updated:** 2025-11-10
**Current Status:** M0-M4 Complete (Tests Broken) → Phase 0 Required
**First Release Target:** Q1 2026 (AI Commanding Center)
**Platform Vision:** Q3 2026 (Universal Automation)

---

## 🎯 Strategic Vision

### **Short-Term (6 months): AI Commanding Center**
Rival Cursor/Claude Code with evidence-based AI development targeting 76-78% SWE-bench accuracy.

### **Long-Term (12+ months): Universal Automation Platform**
Expand beyond coding to email automation, web generation, business automation, and more.

---

## 📊 Development Principles

1. **Evidence-Based:** Every feature backed by research papers or production systems
2. **Atomic UI:** Small, composable, testable components (NOT 1000-line widgets)
3. **Separation of Concerns:** Business logic ≠ UI widgets (Fix P0 violations)
4. **Quality Gates:** No feature ships without tests + validation
5. **Focus:** Kill features that don't serve the current phase's mission

---

## 🚀 Phase Roadmap

| Phase | Focus | Duration | Status |
|-------|-------|----------|--------|
| **Phase 0** | Foundation (Fix Architecture) | 2-3 weeks | 🔴 REQUIRED |
| **Phase 1** | AI Commanding Center | 8-10 weeks | ⏸️ BLOCKED |
| **Phase 2** | Developer Productivity | 6-8 weeks | 📅 PLANNED |
| **Phase 3** | Automation Platform | 12+ weeks | 🔮 FUTURE |

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
- [ ] Verify all 37 claimed tests actually pass
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

**New Structure:**
```
src/ui/
├── atoms/              # 5-8 primitives (50-100 LOC each)
│   ├── text.rs         # Styled text primitive
│   ├── block.rs        # Reusable bordered block
│   ├── button.rs       # Button with states (focus, hover, etc.)
│   ├── icon.rs         # Nerd font icons
│   └── input.rs        # Single-line input (pure)
│
├── molecules/          # 3-5 composites (100-200 LOC)
│   ├── metric_card.rs  # Label + Value + Icon (for eval metrics)
│   ├── task_item.rs    # Icon + Name + Status (for eval tasks)
│   └── progress_bar.rs # Label + Bar + Percentage
│
├── organisms/          # 2-3 complex (200-400 LOC)
│   ├── eval_panel.rs   # MetricCard[] + ProgressBar + TaskItem[]
│   └── input_bar.rs    # Icon + Input + Shortcuts
│
├── screens/            # 3-4 full screens (100-200 LOC)
│   ├── welcome.rs      # Logo + Tips + Start prompt
│   ├── main.rs         # InputBar + Content area
│   ├── evaluation.rs   # EvalPanel + Progress + Results
│   └── trust.rs        # Trust dialog (existing)
│
└── theme.rs            # Theme system (keep existing)
```

**Tasks:**
- [ ] **Week 1: Extract Atoms**
  - [ ] Create atoms module structure
  - [ ] Implement 5 core atoms (text, block, button, icon, input)
  - [ ] Write tests for each atom (5-10 tests per atom)

- [ ] **Week 2: Build Evaluation Molecules**
  - [ ] MetricCard (accuracy, cost, latency display)
  - [ ] TaskItem (task status display)
  - [ ] ProgressBar (evaluation progress)
  - [ ] Test molecule composition

- [ ] **Week 2-3: Compose Organisms & Screens**
  - [ ] EvalPanel (combine molecules)
  - [ ] Evaluation screen (main AI commanding center view)
  - [ ] Welcome screen (simplified)
  - [ ] Main screen (input + status)

- [ ] **Week 3: Kill The Bloat**
  - [ ] Move `ui/widgets/archived/` (100+ files):
    - psx_frogger.rs (851 lines) → Archive
    - demo_mode.rs (592 lines) → Archive
    - render_profiler.rs (739 lines) → Archive
    - conflict_resolver.rs (831 lines) → Archive
    - suggestions_widget.rs (779 lines) → Archive
    - All git widgets → Archive (defer to Phase 2)
    - All chart widgets → Archive (defer to Phase 2)
    - session_manager → Archive
    - vim_macros widget → Archive
  - [ ] Keep only 10-15 files for Phase 1 mission
  - [ ] Document archived features for Phase 2 restoration

**Time Estimate:** 2-3 weeks (parallel with fix work)
**Owner:** UI Team

**Quality Gate:**
- [ ] Total UI code: <2000 LOC (from ~12,000+)
- [ ] Maximum file size: 400 LOC
- [ ] All atoms have 5+ tests
- [ ] Consistent theme/styling across all components
- [ ] Zero duplication of border/scroll/focus logic

---

## P1: Documentation Cleanup (Week 2-3)

### 4. Honest Documentation ⚠️
**Status:** CHANGELOG claims tests pass (they don't), NEW_PROJECT missing
**Impact:** Loss of credibility, confusion

**Tasks:**
- [ ] Update CHANGELOG: Remove "37 passing tests" claim
- [ ] Update CHANGELOG: Remove M5 "IN PROGRESS" (nothing started)
- [ ] Update README: Accurate status (M0-M4 implemented, tests broken)
- [ ] CLAUDE.md: Remove NEW_PROJECT references OR implement it
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
- ✅ All tests passing (actual count documented)
- ✅ Zero I/O in UI widgets
- ✅ App state is pure data (no widgets)
- ✅ Atomic UI implemented (atoms + molecules + organisms)
- ✅ Widget count: <20 files
- ✅ Total UI LOC: <2000
- ✅ Honest documentation

**Quality Gates:**
```bash
# All must pass
cargo test --lib --all          # ✅ 0 errors, all tests pass
cargo clippy -- -D warnings     # ✅ 0 warnings
cargo fmt --check               # ✅ Formatted
tokei src/ui/                   # ✅ <2000 LOC total
find src/ui/widgets -name "*.rs" | wc -l  # ✅ <20 files
grep "std::fs\|tokio::fs" src/ui/widgets/*.rs  # ✅ 0 matches
```

---

# Phase 1: AI COMMANDING CENTER (Alpha Release)

> **Goal:** Ship working AI terminal that validates M0-M5 milestones
> **Duration:** 8-10 weeks
> **Target Accuracy:** 76-78% on SWE-bench Verified
> **Users:** Developers (ourselves + early adopters)

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
**Status:** Atomic UI ready (from Phase 0)
**Target:** Minimal, focused interface for M0-M5 evaluation

**Features:**

#### 1. Welcome Screen (Atom-Based)
**Status:** Refactor existing with atoms
**Tasks:**
- [ ] Logo (using `theme.rs` existing logo)
- [ ] Quick start tips (3-5 tips)
- [ ] "Press any key to continue"
- [ ] Uses: `atoms/text.rs`, `atoms/block.rs`

**LOC Target:** <100 lines
**Time Estimate:** 1 day

---

#### 2. Main Screen (Input + Status)
**Status:** Refactor existing with atoms/molecules
**Tasks:**
- [ ] Top area: Status message + metadata (path, model)
- [ ] Input bar (using `atoms/input.rs` + `molecules/input_bar.rs`)
- [ ] Placeholder: "eval --count 10 --milestone 1" examples
- [ ] Keyboard shortcuts bar at bottom
- [ ] Uses: `atoms/input.rs`, `atoms/text.rs`, `molecules/input_bar.rs`

**LOC Target:** <150 lines
**Time Estimate:** 2 days

---

#### 3. Evaluation Screen (Real-time Progress)
**Status:** NEW - core value proposition
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

**Uses:**
- `molecules/metric_card.rs`
- `molecules/task_item.rs`
- `molecules/progress_bar.rs`
- `organisms/eval_panel.rs` (composes above)
- `screens/evaluation.rs`

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
**Status:** Already implemented
**Tasks:**
- [ ] Verify still works with new architecture
- [ ] Minor styling updates for consistency

**Time Estimate:** 1 hour

---

**Total TUI LOC:** ~750 lines (vs current 12,000+)
**Total Time:** 1-2 weeks

**Quality Gates:**
- [ ] Frame rate: 60 FPS
- [ ] Memory usage: <50MB
- [ ] Startup time: <100ms
- [ ] All keyboard shortcuts work
- [ ] Cross-platform tested (Linux, macOS, Windows)

---

## P2: Core Commands (Week 9)

### Milestone 1.7: Essential Commands Only
**Status:** Refactor existing
**Target:** Minimal command set for AI work

**Commands:**

1. **`eval --count N --milestone M`**
   - Status: ✅ Implemented
   - Action: Verify works with new TUI

2. **`compare --baseline M1 --test M2 --count N`**
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
**Deferred Commands:** All non-essential commands removed

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
- ✅ TUI works flawlessly (<750 LOC)
- ✅ 4 core commands work
- ✅ Documentation complete
- ✅ 5+ early adopters using it

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
cargo test --all  # ✅ 100+ tests passing
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

## P0: Essential Developer Features (Weeks 1-4)

### Milestone 2.1: Git Integration 🔧
**Status:** Some widgets exist (archived in Phase 0)
**Target:** Visual git workflow in TUI

**Features:**
- [ ] Git status panel (widget exists, needs atomic refactor)
- [ ] Git diff viewer (widget exists, needs atomic refactor)
- [ ] Stage/unstage files
- [ ] Commit with message
- [ ] Branch switching
- [ ] Merge conflict helper

**Restore from Archive:**
- `ui/widgets/git/` (refactor with atomic UI)
- git_status_panel.rs → Rebuild with atoms/molecules
- git_diff_viewer.rs → Rebuild with atoms/molecules

**Time Estimate:** 2 weeks
**Tests Required:** 20+

---

### Milestone 2.2: File Browser 📁
**Status:** FileTree exists but violates SoC (does I/O)
**Target:** Fast, keyboard-driven file navigation

**Tasks:**
- [ ] Refactor `FileTree` to use `FilesystemService` (from Phase 0)
- [ ] Pure state-based tree widget
- [ ] Keyboard navigation (vim keys)
- [ ] Fuzzy search integration
- [ ] File preview pane
- [ ] Quick file open (Ctrl+P)

**Time Estimate:** 1-2 weeks
**Tests Required:** 15+

---

### Milestone 2.3: Enhanced Chat Mode 💬
**Status:** `ChatPanel` widget exists (archived)
**Target:** Conversational AI for coding

**Features:**
- [ ] Restore `ChatPanel` (refactor with atomic UI)
- [ ] Streaming responses (word-by-word)
- [ ] Code block syntax highlighting
- [ ] Copy code to clipboard
- [ ] Chat history persistence
- [ ] Context from open files
- [ ] @-mention files for context

**Time Estimate:** 2 weeks
**Tests Required:** 12+

---

### Milestone 2.4: Code Editor Widget 📝
**Status:** Partial (textarea exists, needs enhancement)
**Target:** Vim-mode editing in TUI

**Features:**
- [ ] Restore vim motion support (editor/vim_motions.rs exists)
- [ ] Syntax highlighting (tree-sitter based)
- [ ] Line numbers + column indicator
- [ ] Search and replace (regex)
- [ ] Multi-cursor support (editor/multicursor.rs exists)
- [ ] Undo/redo (editor/undo.rs exists)

**Restore from Archive:**
- Keep `editor/` module as-is
- Integrate with atomic UI atoms for display

**Time Estimate:** 2-3 weeks
**Tests Required:** 25+

---

## P1: Quality of Life (Weeks 5-6)

### Milestone 2.5: Session Management 💾
**Status:** `SessionState` exists in workspace module
**Target:** Resume work seamlessly

**Features:**
- [ ] Auto-save session state
- [ ] Restore open files + chat history
- [ ] Multiple workspaces
- [ ] Session switcher
- [ ] Export/import sessions

**Time Estimate:** 1 week
**Tests Required:** 10+

---

### Milestone 2.6: Search & Navigation 🔍
**Status:** Some infrastructure exists (navigation/search.rs)
**Target:** Find anything fast

**Features:**
- [ ] Fuzzy file finder (Ctrl+P)
- [ ] Global code search (Ctrl+Shift+F)
- [ ] Symbol search (classes, functions)
- [ ] Go to definition
- [ ] Find usages
- [ ] Navigation history (back/forward)

**Time Estimate:** 1-2 weeks
**Tests Required:** 15+

---

### Milestone 2.7: Configuration UI ⚙️
**Status:** Config system exists (TOML-based)
**Target:** Easy customization

**Features:**
- [ ] Settings screen in TUI
- [ ] Theme picker (Catppuccin, Nord, etc.)
- [ ] Keybinding customization
- [ ] AI model configuration
- [ ] Performance settings
- [ ] Live preview of changes

**Time Estimate:** 1 week
**Tests Required:** 8+

---

## P2: Advanced Features (Weeks 7-8)

### Milestone 2.8: Smart Suggestions 💡
**Status:** suggestions_widget.rs exists (archived)
**Target:** Proactive AI assistance

**Features:**
- [ ] Inline code suggestions (like Copilot)
- [ ] Error explanation + fixes
- [ ] Refactoring suggestions
- [ ] Performance optimization tips
- [ ] Security vulnerability detection

**Time Estimate:** 2 weeks
**Tests Required:** 10+

---

### Milestone 2.9: Terminal Integration 🖥️
**Status:** BashTool exists for agent
**Target:** Run commands without leaving TUI

**Features:**
- [ ] Embedded terminal pane
- [ ] Command history
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

**Quality Gates:**
- [ ] User surveys: 8/10+ satisfaction
- [ ] Daily active usage: 50+ developers
- [ ] GitHub stars: 500+
- [ ] Community contributions: 5+ PRs

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

## P0: Platform Infrastructure (Weeks 1-4)

### Milestone 3.1: Plugin System 🔌
**Status:** NEW
**Target:** Extensible architecture

**Features:**
- [ ] Plugin API (Rust trait-based)
- [ ] Dynamic plugin loading
- [ ] Plugin marketplace (web UI)
- [ ] Sandboxed execution
- [ ] Version management
- [ ] Plugin discovery

**Examples:**
- Email plugin
- Web scraping plugin
- Database plugin
- API client generators

**Time Estimate:** 3-4 weeks
**Tests Required:** 30+

---

### Milestone 3.2: Workflow Engine 🔄
**Status:** NEW
**Target:** Chain actions into automations

**Features:**
- [ ] Visual workflow builder (TUI)
- [ ] Trigger system (schedule, event-based)
- [ ] Action library (send email, scrape web, etc.)
- [ ] Conditional logic (if/else)
- [ ] Loop support
- [ ] Error handling + retries
- [ ] Workflow templates

**Time Estimate:** 3-4 weeks
**Tests Required:** 25+

---

## P1: Business Automation (Weeks 5-8)

### Milestone 3.3: Email Automation 📧
**Status:** NEW
**Target:** Smart email management

**Features:**
- [ ] Email client integration (IMAP/SMTP)
- [ ] AI-powered email drafting
- [ ] Auto-categorization
- [ ] Template management
- [ ] Scheduled sending
- [ ] Bulk email with personalization
- [ ] Response suggestions

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
**Status:** NEW
**Target:** Transform and analyze data

**Features:**
- [ ] CSV/JSON/Excel import
- [ ] AI-powered data cleaning
- [ ] Transformation pipeline builder
- [ ] Visualization generation
- [ ] Export to multiple formats
- [ ] Database integration (SQL)

**Time Estimate:** 2-3 weeks
**Tests Required:** 18+

---

## P2: Advanced Automation (Weeks 9-12)

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
**Status:** NEW
**Target:** Insights from data

**Features:**
- [ ] Connect to data sources (DB, APIs, files)
- [ ] AI-powered query generation (natural language → SQL)
- [ ] Chart generation
- [ ] Dashboard builder
- [ ] Report scheduling
- [ ] Anomaly detection

**Time Estimate:** 3 weeks
**Tests Required:** 20+

---

## Phase 3 Success Criteria

**Must Have:**
- ✅ Plugin ecosystem with 10+ plugins
- ✅ 50+ workflow templates
- ✅ Email automation works for 1000+ users
- ✅ Web generation produces production-ready sites
- ✅ 10,000+ active users

**Quality Gates:**
- [ ] Plugin marketplace: 50+ plugins
- [ ] Community-created workflows: 200+
- [ ] Revenue: $10k+ MRR (premium features)
- [ ] GitHub stars: 5,000+
- [ ] Featured on Product Hunt (Top 5)

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

---

## 📊 Success Metrics by Phase

| Metric | Phase 0 | Phase 1 | Phase 2 | Phase 3 |
|--------|---------|---------|---------|---------|
| **Users** | Dev team | 5-10 early | 100-500 | 10,000+ |
| **GitHub Stars** | - | 50+ | 500+ | 5,000+ |
| **Tests Passing** | 37 | 100+ | 200+ | 400+ |
| **Code Quality** | Tests broken | All passing | CI/CD | Production-grade |
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

| Phase | Start Date | End Date | Duration |
|-------|------------|----------|----------|
| **Phase 0** | 2025-11-11 | 2025-11-29 | 2-3 weeks |
| **Phase 1** | 2025-12-02 | 2026-02-14 | 8-10 weeks |
| **Phase 2** | 2026-02-17 | 2026-04-11 | 6-8 weeks |
| **Phase 3** | 2026-04-14 | 2026-07-11 | 12+ weeks |

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

**Competitive Analysis:**
- Cursor: AI-first IDE
- Claude Code: Conversational coding
- GitHub Copilot: Inline suggestions
- Windsurf: Multi-agent system

---

## ✅ Phase 0 Next Actions (This Week)

1. **Monday:** Fix compilation errors (192 errors) → Owner: @core-team
2. **Tuesday:** Create FilesystemService, move I/O out of widgets → Owner: @architecture
3. **Wednesday:** Start Atomic UI extraction (atoms module) → Owner: @ui-team
4. **Thursday:** Extract state from widgets (InputState, etc.) → Owner: @architecture
5. **Friday:** Documentation cleanup (honest status) → Owner: @docs-team

**Daily Standup:** 9 AM (async in Discord)
**Weekly Review:** Friday 4 PM
**Roadmap Check-in:** Every 2 weeks

---

**Last Updated:** 2025-11-10
**Next Review:** 2025-11-24 (Phase 0 completion check)
**Roadmap Owner:** @product-team

---

## 🎉 Vision Statement

> "TOAD will be the **terminal that thinks**—starting as the world's most accurate AI coding assistant, evolving into the **universal automation platform** that empowers developers and businesses to automate anything repetitive. Evidence-based, brutally focused, and built to last."

Let's build it. 🐸
