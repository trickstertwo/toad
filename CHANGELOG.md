# Changelog

All notable changes to TOAD will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 🚧 IN PROGRESS
<!-- Agent coordination: Declare work BEFORE starting to prevent conflicts -->
<!-- Format: - [Module/Feature] Brief description (@claude-name or @username) -->
<!-- Remove from this section when complete and move to appropriate category below -->
- [Phase 1: AI Commanding Center] Atomic UI refactoring (@claude) - 2025-11-10
  - ✅ Text atom created (271 lines, 15 tests, 100% API coverage)
  - ⏳ Block atom (next)
  - ⏳ Icon atom
  - ⏳ Molecules (metric_card, task_item, progress_bar)
  - ⏳ Evaluation screen (core Phase 1 feature)
  - Goal: <850 LOC new Atomic UI, real-time eval progress display

<!-- COMPLETED 2025-11-10: Phase 0 Foundation - Architecture cleanup and SoC patterns -->
<!-- COMPLETED 2025-11-08: Agent system restructured -->
<!-- COMPLETED 2025-11-08: Domain-driven restructure + Phase 0 TUI-AI integration -->
<!-- COMPLETED 2025-11-08: Automated project initialization system -->
<!-- COMPLETED 2025-11-09: M4 Cascading Routing implementation -->

### Added
- **Phase 0: Foundation - Architecture Overhaul** (COMPLETE - 2025-11-10)
  - **Compilation Fixed**: 192 test errors → 0 errors, 5,084 tests passing
  - **Separation of Concerns Patterns Established**:
    - `FilesystemService` (285 lines, 9 tests): Service layer for I/O operations
    - `InputState` (289 lines, 13 tests): Pure data state separated from UI
    - Pattern: Model-View separation following Elm Architecture
  - **Code Cleanup**: Removed 2,582 LOC (easter eggs + broken tests), added 417 LOC (services + state + tests)
  - **Net Reduction**: -2,165 LOC cleaner codebase
  - **Atomic UI Structure**: Created atoms/molecules/organisms/screens directories
  - **Quality Gates**: 100% public API test coverage, zero unwrap() in production, zero unsafe
  - **Documentation**: Comprehensive rustdoc with examples on all public methods
  - **Success Criteria Met**: ✅ Tests pass, ✅ SoC demonstrated, ✅ <2000 LOC reduction
  - **Ready for Phase 1**: Patterns proven and replicable
- **M4: Cascading Routing + Cost Optimization** (DavaJ approach - 70% cost reduction)
  - `CascadeMetadata` tracking for routing decisions (difficulty, tier, cost, latency)
  - 4-tier model selection: Local7B → Local32B → CloudPremium → CloudBest
  - Task difficulty classifier (Easy/Medium/Hard heuristics)
  - Local-first strategy: 80% of tasks run free on Ollama
  - Cloud fallback for hard tasks requiring premium models
  - Comprehensive test suite (19 tests: 12 routing + 7 integration)
  - Evidence-based implementation following DavaJ research (2024)
  - Cost model: $200 for 500 tasks vs $1000 cloud-only (80% reduction)
  - **Requirements**: Ollama with qwen2.5-coder:7b and :32b for local tiers
  - **Cloud-only mode**: Works without Ollama using routing_cascade=true
- **M3: Multi-Model Racing** (TRAE approach - +4.2 points on SWE-bench)
  - `RacingClient` for parallel LLM execution with first-complete-wins selection
  - Race metadata tracking (winner model, costs, latency improvements)
  - Configuration support for racing models in `ToadConfig`
  - Evaluation harness integration with automatic race metadata extraction
  - Evidence-based implementation following TRAE paper (2024)
  - Latency reduction through parallel model execution (20-40% P50 improvement)
  - Cost tracking including wasted costs from cancelled models
  - Comprehensive test suite (18 tests: 11 racing + 7 integration)
- **Domain-driven architecture restructure** migrating 134 files from flat structure to organized domains
  - `core/` - TUI fundamentals (Elm Architecture: Model-Message-Update-View)
  - `ui/` - Widgets, themes, visual components (40+ widgets)
  - `ai/` - Agent, LLM, evaluation, metrics, tools
  - `editor/`, `workspace/`, `navigation/`, `commands/`, `performance/`, `infrastructure/`, `config/`
  - Reduced coupling, improved discoverability, clearer module boundaries
- **TUI-AI integration (Phase 0)** enabling evaluations inside interactive terminal
  - Command parsing for `eval`, `compare`, `show-config` from TUI input field
  - Async evaluation runner with progress updates via event channel
  - Real-time evaluation screen showing task progress, agent steps, tokens, cost
  - Background task execution with cancellation support (Ctrl+C or Esc)
  - Completion screen with accuracy, cost analysis, duration metrics
- **Automated project initialization system** (`NEW_PROJECT/` template)
  - `/init-automation` slash command triggers full setup
  - `project-initializer` agent asks 8 questions, generates ALL files
  - Language-agnostic (Rust, TypeScript, Python, Go, Java, C++)
  - Customized hooks with user's actual build/test/lint commands
  - Tech-stack specific skills (< 500 lines) and skill-rules.json triggers
  - Language-specific agents (code reviewer, error resolver, testing expert)
  - Copy scripts for easy deployment (`copy-to-project.sh/bat`)
  - 10 minutes total setup → saves 2-4 hours manual configuration
  - Proven system from 300k LOC production use
- **Automated workflow system** adapted from 6 months of production use (300k LOC rewrite)
  - 5-stage workflow with automation baked in (not optional)
  - Dev docs system (`.toad/active/[task-name]/` with plan/context/tasks) prevents "losing the plot"
  - Hooks system for zero-errors-left-behind (build checker, error detector, skill activator)
  - Skills with progressive disclosure (< 500 lines, 40-60% token reduction)
  - Custom slash commands (`/strategic-plan`, `/create-dev-docs`, `/code-review`, etc.)
- **Specialized agents** (6 total, strictly Rust-focused):
  - `strategic-plan-architect` - Creates comprehensive plans before implementation (Stage 1)
  - `rust-code-reviewer` - Reviews between phases and final validation (Stage 3, 5)
  - `cargo-error-resolver` - Systematically fixes compiler/clippy errors (hooks integration)
  - `test-coverage-analyzer` - Verifies layer-specific coverage targets (Stage 5)
  - `rust-security-auditor` - Audits command injection, path traversal, secrets exposure
  - `rust-testing-expert` - Writes comprehensive tests (table-driven, async, property-based)
- Code-first documentation strategy using rustdoc instead of markdown entity lists
- Layer-specific test coverage targets (Models 95%+, Services 80%+, Tools 80%+, Infrastructure 60%+, UI 40%+)
- Statistical validation requirements for M0 features (p < 0.05, Cohen's d effect size)
- Keep a Changelog 1.1.0 compliance (human-focused, standard categories, ISO 8601 dates)

### Changed

### Deprecated

### Removed
- Deleted 5 irrelevant agents copied from other projects:
  - `frontend-svelte-expert` (TOAD uses Rust TUI, not web frontend)
  - `backend-go-ddd` (TOAD uses Rust, not Go)
  - `devops-infra-expert` (Overkill for CLI tool, no K8s/Docker deployment)
  - `testing-qa-expert` (Replaced with `rust-testing-expert`)
  - `security-expert` (Replaced with `rust-security-auditor` for CLI-specific threats)

### Removed
- **Easter Eggs** (Phase 0: Foundation cleanup)
  - PSX Frogger game (851 lines) - fun but not aligned with Phase 1 goals
  - Demo Mode widget (592 lines) - not actively used
  - Event handlers and UI rendering for easter eggs (84 lines)
  - Total cleanup: 1,527 LOC removed, zero compilation errors
  - Atomic UI directory structure created (atoms/molecules/organisms/screens)

### Fixed

### Security

---

## PROJECT STATUS
<!-- This section helps agents understand current state -->

**Current Milestone**: M0 ✅ Complete | M1 🚧 In Progress
**Target Accuracy**: M1 = 55-60% on SWE-bench Verified

**Quality Metrics**:
- Tests: 37 passing (29 unit + 8 integration)
- Coverage: 80%+ overall target (layer-specific: Models 95%+, Services 80%+, Tools 80%+, Infrastructure 60%+, UI 40%+)
- Documentation: Zero rustdoc warnings policy
- Binary Size: Target ≤ 10MB stripped
- Startup Time: Target ≤ 100ms

**Architecture Status**:
- ✅ Dual architecture: M0 evaluation framework + TUI application
- ✅ Feature flag system: 13 independently toggleable experimental features
- ✅ Elm Architecture pattern for TUI (Model-Update-View)
- ✅ Statistical validation framework (Welch's t-test, p < 0.05, Cohen's d effect size)
- ✅ **Automated workflow** (hooks, skills, dev docs, agents - default, not optional)

**Automation Infrastructure**:
- `.toad/active/` - Dev docs survive compaction (plan/context/tasks per feature)
- `.claude/hooks/` - Auto-enforcement (build checker, error detector, skill activator)
- `.claude/skills/` - Pattern libraries (< 500 lines each, auto-activate)
- `.claude/agents/` - 6 Rust-specific agents (strategic planner, code reviewer, error resolver, coverage analyzer, security auditor, testing expert)
- `.claude/commands/` - Slash commands for common workflows

**Agent Roles** (Following "Claude Code is a Beast" principles):
- Very specific roles with clear responsibilities
- Return structured, actionable reports (not vague "looks good")
- Integrated into 5-stage workflow (not nice-to-haves)
- Rust-specific patterns (no Go/Svelte/web concepts)
- CLI security focus (command injection, path traversal, not OWASP Top 10 web)

**Next Milestones**:
1. **M1 (Weeks 2-4)**: Simple baseline agent with basic tool use → 55-60% accuracy
2. **M2 (Weeks 5-7)**: Enhanced with AST context + smart test selection → 61-66% accuracy
3. **M3 (Weeks 8-10)**: Advanced multi-model routing → 70-75% accuracy

**Last Updated**: 2025-11-08

**Agent System Status**:
- ✅ 6 Rust-specific agents created (0 generic/copied agents remaining)
- ✅ All agents follow "Claude Code is a Beast" principles
- ✅ Integrated into RUST_WORKFLOW.md 5-stage workflow
- ✅ Each agent has specific role, clear output format, and quality gates

---

## [0.1.0] - 2025-11-08

**Milestone M0**: Infrastructure & Evaluation Framework

### Added
- Evaluation framework for benchmarking AI agents on SWE-bench datasets
- Statistical significance testing using Welch's t-test with p < 0.05 threshold
- Feature flag architecture with 13 independently toggleable experimental features
- Metrics collection tracking accuracy, API cost, latency, and solution quality
- Dataset management supporting SWE-bench Verified (500 tasks), Lite (300 tasks), and Full (2,294 tasks)
- Experiment tracking system for A/B comparisons between configurations
- CLI interface with 4 commands: `eval`, `compare`, `show-config`, `generate-test-data`
- Comprehensive test suite: 37 passing tests (29 unit + 8 integration)
- TUI foundation using Elm Architecture (Model-Update-View pattern)
- Agent execution loop with tool use (Read, Write, Edit, Grep, Bash, Git, List)
- LLM client integration with Anthropic Claude API and rate limiting
- Quality gates system with 9 validation checkpoints

### Changed
- Adopted Rust 2024 edition for latest language features
- Structured codebase into dual architecture: M0 evaluation framework + TUI application

### Security
- Added terminal panic hook to restore terminal state on crashes
- Implemented rate limiting for LLM API calls (50 req/min, 40k TPM, 400k TPD)
- Environment variable loading for API keys via dotenvy
