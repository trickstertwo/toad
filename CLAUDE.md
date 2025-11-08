# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TOAD (Terminal-Oriented Autonomous Developer) is an AI coding terminal designed to rival Cursor, Claude Code, and GitHub Copilot through evidence-based, iterative development. The project is structured around milestones (M0-M5) targeting progressive accuracy improvements on SWE-bench benchmarks, from 55-60% (M1) to 76-78% (M5).

**Current Status:** M0 (Infrastructure) is complete. Working toward M1 (Simple Baseline).

---

## 🚀 New Project Initialization

TOAD includes a reusable automation template for ANY new project (not just TOAD itself).

### Quick Start for New Projects

**Option A: Copy Script** (Fastest)
```bash
# Linux/Mac
cd /path/to/your/new/project
/path/to/toad/NEW_PROJECT/copy-to-project.sh .

# Windows
cd C:\path\to\your\new\project
C:\path\to\toad\NEW_PROJECT\copy-to-project.bat .

# Then in Claude Code:
/init-automation
# Answer 8 questions → Done! ✅
```

**Option B: Manual Copy**
```bash
# Copy .claude folder to your project
cp -r /path/to/toad/NEW_PROJECT/.claude /path/to/your/project/

# Open project in Claude Code
# Type: /init-automation
```

**What It Does**:
- Asks 8 questions about your tech stack (2 minutes)
- Generates ALL automation files customized to YOUR stack:
  - Hooks (build checker, skill auto-activation)
  - Skills (language-specific patterns)
  - Agents (code reviewer, error resolver, testing expert)
- Takes 10 minutes total → Saves 2-4 hours of manual setup

**Questions Asked**:
1. Language (Rust/TypeScript/Python/Go/Java/C++)
2. Framework (if applicable)
3. Build command (e.g., `cargo build`)
4. Test command (e.g., `cargo test`)
5. Linter command (e.g., `cargo clippy`)
6. Format command (e.g., `cargo fmt`)
7. Dev docs tracking (yes/no)
8. Project type (CLI/API/Library/Desktop/Full-stack)

**See**: `NEW_PROJECT/README.md` for full details and troubleshooting.

---

## Common Commands

### Build & Run
```bash
# Build the project
cargo build

# Build optimized release binary
cargo build --release

# Run TUI mode interactively (default if no command)
cargo run
# or explicitly:
cargo run -- tui

# Show feature configuration for a milestone
cargo run -- show-config --milestone 1
```

### Testing
```bash
# Run all tests (unit + integration)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run only integration tests
cargo test --test '*'

# Run tests for a specific module
cargo test evaluation::

# Run with verbose output
cargo test -- --show-output
```

### Evaluation Framework (M0)

**CLI Mode** (batch processing):
```bash
# Generate synthetic test data
cargo run -- generate-test-data --count 50

# Run evaluation with synthetic data
cargo run -- eval --count 10 --milestone 1

# Run evaluation with SWE-bench dataset (auto-download)
cargo run -- eval --swebench verified --count 10 --milestone 1

# Run evaluation with local dataset
cargo run -- eval --dataset ./test_data.json --count 10

# Compare two configurations (A/B test)
cargo run -- compare --baseline 1 --test 2 --count 20

# Compare with SWE-bench dataset
cargo run -- compare --swebench verified --baseline 1 --test 2 --count 20
```

**TUI Mode** (interactive with real-time progress):
```bash
# Launch TUI
cargo run

# Then type commands in the input field:
eval --swebench verified --count 10 --milestone 1
eval --count 5 --milestone 2
compare --baseline 1 --test 2 --count 20
show-config --milestone 3

# Press Ctrl+C or Esc to cancel running evaluation
# Press q or Esc after completion to return to main screen
```

TUI evaluation features:
- Real-time progress updates (task X/Y, agent steps, tool calls)
- Live token/cost tracking during execution
- Background execution (TUI remains responsive)
- Cancellation support
- Completion screen with accuracy, cost, duration metrics

### Code Quality
```bash
# Run clippy with strict warnings
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check
```

## Architecture

TOAD follows a **dual architecture**:

### 1. M0 Evaluation Framework
Located in `src/evaluation/`, `src/metrics/`, `src/stats/`, `src/agent/`, `src/llm/`, `src/tools/`

**Key Components:**
- **Evaluation Harness** (`src/evaluation/mod.rs`): Runs tasks and collects results
- **Dataset Manager** (`src/evaluation/dataset_manager.rs`): Handles SWE-bench datasets (Verified/Lite/Full)
- **Task Loader** (`src/evaluation/task_loader.rs`): Loads and validates tasks
- **Agent** (`src/agent/mod.rs`): Core agent loop with tool use (max 25 steps)
- **LLM Client** (`src/llm/mod.rs`): Anthropic Claude API integration with rate limiting
- **Tools** (`src/tools/`): Read, Write, Edit, Grep, Bash, Git, List tools
- **Metrics** (`src/metrics/mod.rs`): Tracks accuracy, cost, latency, quality
- **Stats** (`src/stats/mod.rs`): Welch's t-test, effect sizes, statistical comparison

### 2. TUI Application (Elm Architecture)
Located in `src/app.rs`, `src/ui.rs`, `src/event.rs`, `src/tui.rs`, and `src/widgets/`

**Pattern:** Model-Update-View
- **Model** (`src/app.rs`): Application state (App struct)
- **Event** (`src/event.rs`): Events and messages
- **Update** (`src/app.rs`): State transitions in `App::update()`
- **View** (`src/ui.rs`): Rendering logic using Ratatui

**Key Screens:**
- Welcome: Initial screen with logo
- TrustDialog: Directory trust confirmation
- Main: Main application interface
- Evaluation: Shows running evaluation progress

### Feature Flags System
The project uses a sophisticated feature flag system (`src/config/mod.rs`) with 13 toggleable features across 4 categories:

1. **Context Strategies**: AST, embeddings, graph, reranking
2. **Routing Strategies**: Semantic router, multi-model, speculative
3. **Intelligence**: Smart test selection, failure memory, opportunistic planning
4. **Optimizations**: Prompt caching, semantic caching, tree-sitter validation

**Milestone Configurations:**
- M1 (55-60%): Minimal features - basic context, prompt caching, validation
- M2 (61-66%): + AST context, smart test selection
- M3 (70-75%): + Multi-model routing

## Module Organization

### M0 Core Modules
- `evaluation/`: Task evaluation framework
- `metrics/`: Performance metrics collection
- `stats/`: Statistical analysis (Welch's t-test)
- `agent/`: Agent execution loop and prompts
- `llm/`: LLM client (Anthropic), rate limiting, errors
- `tools/`: Tool implementations (Read, Write, Edit, Grep, Bash, Git, List)
- `config/`: Feature flags and configuration

### TUI Modules
- `app.rs`: Application state and update logic
- `ui.rs`: Main rendering function
- `event.rs`: Event types (keyboard, evaluation progress)
- `tui.rs`: Terminal initialization and management
- `widgets/`: Reusable UI components (60+ widgets)
- `theme/`: Theme system (Catppuccin, Nord, built-in themes)
- `layout.rs`: Split pane management
- `tabs.rs`: Tab management for workspaces
- `session.rs`: Session persistence

### Infrastructure
- `clipboard.rs`: Cross-platform clipboard
- `performance.rs`: FPS limiting, metrics
- `history.rs`: Command history
- `keybinds.rs`: Keyboard shortcuts

## Key Architectural Patterns

### 1. Async Runtime
Uses Tokio for async operations. The TUI runs in a Tokio runtime with `tokio::select!` for handling both terminal events and async evaluation progress.

### 2. Event Loop (TUI)
```rust
while !app.should_quit() {
    tui.draw(|frame| render(&mut app, frame))?;

    tokio::select! {
        terminal_event = spawn_blocking(handler.next()) => { ... }
        Some(async_event) = event_rx.recv() => { ... }
    }
}
```

### 3. Agent Loop (M0)
```rust
loop {
    step_count += 1;
    if step_count > MAX_AGENT_STEPS { break; }

    let response = llm_client.send_message(conversation, tools).await?;

    match response.stop_reason {
        StopReason::ToolUse => { /* execute tools */ }
        StopReason::EndTurn => { /* done */ }
        _ => { /* handle error */ }
    }
}
```

### 4. Error Handling
- Uses `anyhow::Result<T>` for application errors
- Uses `thiserror` for custom error types (LLM errors, tool errors)
- No `unwrap()` in production code
- All I/O operations return `Result`

### 5. Configuration
- Feature flags stored in `ToadConfig` with `FeatureFlags`
- TUI configuration in `Config` (UI, Editor, AI, Session settings)
- Loaded from `~/.config/toad/config.toml` with serde

## Testing Strategy

### Test Structure
- **Unit tests**: Inline `#[cfg(test)]` modules in source files
- **Integration tests**: `tests/integration_test.rs`, `tests/m0_validation_tests.rs`
- **Mock support**: Uses `mockall` crate for LLM client mocking

### Test Coverage Requirements
- M0 validation: 37 passing tests (29 unit + 8 integration)
- Statistical tests require p < 0.05 for significance
- Integration tests validate end-to-end workflows

## Development Workflow

### Adding a New Feature
1. Add feature flag to `FeatureFlags` in `src/config/mod.rs`
2. Document evidence/research in comments
3. Implement feature with proper error handling
4. Add unit tests (inline `#[cfg(test)]`)
5. Add integration test if needed
6. Update relevant milestone configuration
7. Run A/B comparison to validate impact

### Working with Datasets
- **Synthetic**: Use `generate-test-data` for quick testing
- **SWE-bench Verified**: 500 tasks, highest quality
- **SWE-bench Lite**: 300 tasks, representative sample
- **SWE-bench Full**: 2,294 tasks, complete dataset

Auto-download from HuggingFace using `--swebench` flag.

### Evaluation Results
Results saved to `./results/` directory:
- `evaluation_TIMESTAMP.json`: Full task results
- Contains: task_id, solved, tests_passed, duration_ms, cost_usd, metrics

### Statistical Validation
Use `ComparisonResult::compare()` for A/B tests:
- Computes Welch's t-test (unequal variances)
- Calculates Cohen's d effect size
- Provides automatic recommendations (adopt/reject/inconclusive)
- Requires p < 0.05 for significance

## Important Implementation Details

### LLM Integration
- **Primary model**: Claude Sonnet 3.5 (claude-sonnet-3-5-20241022)
- **Rate limiting**: 50 req/min, 40k TPM, 400k TPD (Tier 1)
- **Prompt caching**: Enabled by default (90% cost reduction)
- **Max tokens**: 8192 output limit
- **Temperature**: 1.0 (default)

### Tool System
Tools implement `Tool` trait with `execute()` method:
- **Read**: Read file contents
- **Write**: Write/create files
- **Edit**: Apply unified diff patches
- **Grep**: Search file contents with regex
- **Bash**: Execute shell commands
- **Git**: Git operations (diff, log, status, etc.)
- **List**: List directory contents

### Agent Execution
- Max 25 steps per task (configurable)
- Conversation history maintained across steps
- Tool schemas sent to LLM for tool use
- Metrics tracked per step (API calls, tokens, cost)

## Environment Setup

### Required
- Rust 2024 edition (nightly features)
- `ANTHROPIC_API_KEY` environment variable

### Optional
- `.env` file for local development (loaded via `dotenvy`)
- `~/.config/toad/config.toml` for TUI settings

## Performance Considerations

- **TUI rendering**: Frame limiter (60 FPS default)
- **Virtual scrolling**: Supports 1M+ items efficiently
- **Lazy rendering**: Only render visible elements
- **Release builds**: LTO enabled, stripped binaries
- **Target binary size**: < 10MB stripped

## Cross-Platform Notes

- Windows 11 development environment
- Uses `crossterm` for cross-platform terminal handling
- Clipboard via `copypasta` (cross-platform)
- Paths use `PathBuf` for platform independence

## Documentation

Key documentation files:
- **README.md**: Quick start and project status
- **ARCHITECTURE.md**: Detailed architecture and quality gates
- **ITERATIVE_IMPLEMENTATION_PLAN.md**: 5-milestone roadmap
- **AI_CODING_AGENTS_RESEARCH_REPORT.md**: SOTA agent analysis
- **LLM_CONTEXT_OPTIMIZATION_RESEARCH.md**: Context management research
- **llm_routing_research_report.md**: LLM routing strategies

## Agent Workflow (Automated)

**MANDATORY READING**: `RUST_WORKFLOW.md` - 5-stage workflow with hooks, skills, dev docs, agents

**Quick Reference** (Full details in RUST_WORKFLOW.md):

### Stage 1: ANALYZE (Planning + Dev Docs)
1. Check `CHANGELOG.md` → `🚧 IN PROGRESS` and `.toad/active/`
2. `/strategic-plan [task-name]` (for tasks > 3 files or > 100 LOC)
3. Review plan thoroughly (catches 40%+ of issues)
4. `/create-dev-docs [task-name]` → creates plan.md, context.md, tasks.md
5. Declare in CHANGELOG → `🚧 IN PROGRESS`

### Stage 2: IMPLEMENT (Rustdoc First, Tests Immediately)
1. Implement in sections: "Only do Phase 1, then stop"
2. Rustdoc BEFORE code
3. Tests IMMEDIATELY (same file)
4. Update tasks.md as you go
5. **Hooks auto-check**: Build errors, unwrap(), unsafe

### Stage 3: INTEGRATE (Code Reviews Between Phases)
1. `/code-review` between implementation sections
2. Run integration tests
3. `/update-dev-docs [task-name]` before compaction
4. Resume after compaction: "Read .toad/active/[task-name]/"

### Stage 4: DOCUMENT (Skills Auto-Activate)
1. **Skills auto-inject** via hooks (no manual invocation needed)
2. Verify rustdoc complete (zero warnings)
3. Update CHANGELOG (Added/Changed/Fixed/Security)

### Stage 5: VALIDATE (Automated Final Checks)
1. `/build-and-fix` - cargo check all workspaces
2. `/coverage-check` - verify layer targets
3. `/code-review` - final comprehensive review
4. Archive dev docs to `.toad/archive/`
5. Update CHANGELOG, end session (NO summary docs)

**Automation Infrastructure** (See RUST_WORKFLOW.md "Automation Setup"):
- `.toad/active/` - Dev docs (plan/context/tasks) survive compaction
- `.claude/hooks/` - Build checker, error detector, skill activator
- `.claude/skills/` - Pattern libraries (< 500 lines, auto-activate)
- `.claude/agents/` - Code reviewer, error resolver, planners
- `.claude/commands/` - Slash commands for common workflows

## Common Pitfalls

1. **Async context**: TUI runs in async runtime, use `tokio::spawn_blocking` for blocking operations
2. **Terminal state**: Always restore terminal on panic (panic hook installed)
3. **Rate limits**: LLM client has built-in rate limiting, don't bypass
4. **Dataset validation**: Always validate task count vs requested count
5. **Session saving**: Save session state on graceful exit only
6. **Tool schemas**: Must be sent with each LLM request for tool use
7. **Windows paths**: Use `PathBuf` and avoid hardcoded `/` separators
8. **No `unwrap()` in src/**: Use `?` or `expect()` with justification
9. **Documentation**: Code-first (rustdoc), not markdown for implementation details
10. **CHANGELOG is single source**: No summary docs (README_SUMMARY.md, etc.)
