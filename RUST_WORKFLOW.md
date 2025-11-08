# TOAD Rust Workflow & Quality Gates

> **Agent-specific documentation for consistent, high-quality work across sessions**

**Verified Environment**:
- Rust: 1.89.0 (2025-08-04)
- Cargo: 1.89.0
- Clippy: 0.1.89
- Platform: Windows 11

---

## 🔄 Quality-Gated Workflow (5 Stages with Automation)

### 1. ANALYZE 🔍 (Planning + Dev Docs Setup)

**Read First**:
- Check `CHANGELOG.md` → `🚧 IN PROGRESS` section (avoid conflicts)
- Check `.toad/active/` → existing task directories (resume if needed)

**Planning Mode** (MANDATORY for features requiring > 3 files or > 100 LOC):
1. Enter Plan mode or run `/strategic-plan [task-name]`
2. Let Claude research (don't interrupt - it uses Explore agent internally)
3. Review plan thoroughly:
   - Check for misunderstandings (catches 40%+ of issues early)
   - Verify Rust patterns (no `unwrap()`, proper `Result` usage)
   - Ask neutral questions: "What alternatives did you consider?"
   - Look for missing edge cases, error paths

**Create Dev Docs** (Prevents "losing the plot"):
```bash
# After approving plan, run:
/create-dev-docs [task-name]

# This creates:
.toad/active/[task-name]/
├── plan.md        # Phases, tasks, timeline from planning
├── context.md     # Key files, decisions, module boundaries
└── tasks.md       # Checklist with [ ] items
```

**Declare Work**:
- Add to `CHANGELOG.md` → `🚧 IN PROGRESS` section:
  ```markdown
  - [Module] Brief description (@your-name)
  ```

**Scope Analysis**:
- Identify affected modules (M0 evaluation vs TUI vs both)
- Check feature flag implications, async boundaries, trait requirements
- List dependencies (crates, modules, data structures)

**Success Criteria**: Define observable outcomes (test passes, benchmarks, accuracy targets)

**Gate**: ✅ Plan approved + dev docs created + work declared in CHANGELOG

### 2. IMPLEMENT ⚙️ (Per Module with Automation)

**Implement in Sections** (Not all at once):
```bash
# Tell Claude explicitly:
"Implement only Phase 1 (tasks 1.1-1.3), then stop for review.
Update tasks.md as you complete items."
```

**Rustdoc FIRST**:
```rust
//! Module purpose and responsibilities
//!
//! # Architecture
//! Brief explanation of key design decisions
//!
//! # Examples
//! ```
//! use toad::module::Type;
//! let example = Type::new();
//! ```

/// Brief description of what this does
///
/// # Examples
/// ```
/// use toad::Type;
/// let value = Type::new();
/// assert!(value.is_valid());
/// ```
///
/// # Errors
/// Returns `Err` if X condition occurs
///
/// # Panics
/// Does not panic (or document panic conditions)
pub fn example() -> Result<()> { ... }
```

**Tests IMMEDIATELY** (Same file):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        let result = function_under_test();
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_condition() {
        let result = function_with_invalid_input();
        assert!(result.is_err());
    }
}
```

**Mandatory Rules**:
- ✅ **Rustdoc BEFORE code**: Write `//!` and `///` docs first
- ✅ **Tests IMMEDIATELY**: Write `#[test]` in same file as implementation
- ✅ **No `unwrap()`**: Use `?`, `expect()` with justification, or proper error handling
- ✅ **Async boundaries**: Document `async fn` Send requirements, tokio runtime assumptions
- ✅ **Feature flags**: Document which `FeatureFlags` affect behavior
- ✅ **Update dev docs**: "Update tasks.md - mark completed items, add new discoveries to context.md"

**Automated Quality Checks** (Via Hooks):

**Post-Edit Build Checker** (`.claude/hooks/post-tool-use.ts` + `.claude/hooks/stop.ts`):
- Tracks which files were edited
- On Stop event, runs `cargo check` on affected workspace members
- If < 5 errors: Shows them immediately for fixing
- If ≥ 5 errors: Suggests launching `cargo-error-resolver` agent

**Error Pattern Detector** (`.claude/hooks/stop.ts`):
```
After Claude finishes, scans for:
- unwrap() in src/ (not tests/)
- unsafe without safety docs
- todo!() or unimplemented!()
- Missing error handling in Result types

Shows gentle reminder (non-blocking):
"⚠️ Pattern Detected: unwrap() on line 45
Did you replace with proper error handling?"
```

**Self-QA Before Moving On**:
```bash
cargo fmt --check           # ✅ Formatting
cargo clippy -- -D warnings # ✅ Zero warnings
cargo test --lib            # ✅ All tests pass
cargo doc --no-deps 2>&1 | grep warning  # ✅ Zero doc warnings
```

**Gate**: ✅ Zero clippy warnings + tests pass + rustdoc complete + no `unwrap()` + hooks passed + tasks.md updated

### 3. INTEGRATE 🔗 (Cross-Module Validation)

**Code Review** (Between Implementation Phases):
```bash
# After completing a section, BEFORE next section:
/code-review

# Launches rust-code-reviewer agent which checks:
# - unwrap() usage
# - unsafe blocks (with safety docs)
# - Error handling patterns
# - Test coverage (layer-specific targets)
# - Rustdoc completeness
# - Async Send bounds
```

**Integration Testing**:
- ✅ **Unit tests**: Module-level correctness (80%+ coverage)
- ✅ **Integration tests**: `tests/*.rs` for cross-module workflows
- ✅ **M0 validation**: If touching evaluation framework, run `cargo run -- eval --count 5`
- ✅ **TUI validation**: If touching UI, run `cargo run -- tui` and test manually
- ✅ **No regressions**: All existing tests still pass

**Statistical Requirements (M0 only)**:
- If implementing feature flags: Document expected impact range (e.g., "+2-5 points")
- If changing agent/LLM: Run A/B comparison with p < 0.05 threshold
- Use `cargo run -- compare --baseline 1 --test 2 --count 20` for validation

**Update Dev Docs** (Before Compaction):
```bash
# When approaching context limit:
/update-dev-docs [task-name]

# Updates:
# - context.md: Current state, next steps, new decisions
# - tasks.md: Completed items marked [x], new tasks added
# - Last Updated timestamp

# After compaction, resume with:
"Read all files in .toad/active/[task-name]/ and continue"
```

**Gate**: ✅ All tests pass + no regressions + code review passed + dev docs updated + domain-specific validation complete

### 4. DOCUMENT 📚 (Code-First + Architecture)

**Skill Activation** (Automatic via hooks):

Skills auto-activate based on:
- Keywords in prompt ("agent", "LLM" → `m0-evaluation` skill)
- Files being edited (`src/ui.rs` → `tui-development` skill)
- Content patterns (async, unsafe → `rust-best-practices` skill)

**Hook**: `.claude/hooks/user-prompt-submit.ts` injects before Claude sees prompt:
```
🎯 SKILL ACTIVATION: Use rust-best-practices skill
(Detected: editing src/agent/mod.rs with async functions)
```

**Skills Structure** (< 500 lines each):
```
.claude/skills/
├── rust-best-practices.md (~400 lines)
│   └── resources/
│       ├── error-handling.md
│       ├── async-patterns.md
│       └── testing-patterns.md
├── m0-evaluation.md (~350 lines)
│   └── resources/
│       ├── statistical-validation.md
│       └── agent-loop.md
└── tui-development.md (~380 lines)
    └── resources/
        ├── elm-architecture.md
        └── ratatui-widgets.md
```

**Code Documentation** (MANDATORY):

| Item | Format | Verification |
|------|--------|--------------|
| **Modules** | `//!` with purpose, architecture, examples | `cargo doc --open` |
| **Public items** | `///` with description, examples, panics, errors | `cargo doc --no-deps` |
| **Complex algorithms** | Inline `//` comments explaining "why" | Code review |
| **Async functions** | Document Send bounds, runtime requirements | Rustdoc |
| **Feature flags** | Document in `src/config/mod.rs` with evidence | Comments |

**Architecture Docs** (ONLY if decisions changed):
- ✅ `ARCHITECTURE.md`: Updated ONLY for new patterns or layer changes
- ✅ `QUALITY_GATES.md`: Updated ONLY for new quality requirements
- ❌ **NO entity lists**: Use `cargo doc`, not markdown files with struct/function lists
- ❌ **NO implementation details in markdown**: Code comments are source of truth

**CHANGELOG.md** (MANDATORY - Follows [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/)):

**Format Rules**:
- ✅ **Human-focused**: Write for users/developers, not machines
- ✅ **Standard categories**: Added, Changed, Deprecated, Removed, Fixed, Security (in this order)
- ✅ **ISO 8601 dates**: YYYY-MM-DD format only
- ✅ **Version format**: `## [X.Y.Z] - YYYY-MM-DD` with optional milestone note
- ✅ **Bullet points**: Main point describes WHAT changed (user-facing), sub-bullets add technical HOW
- ✅ **Unreleased section**: Always exists at top for work in progress

**Entry Format**:
```markdown
## [Unreleased]

### 🚧 IN PROGRESS
<!-- Agent coordination: Declare work BEFORE starting -->
- [Module] Brief description (@agent-name)

### Added
- User-facing feature description (what users get)
  - Technical implementation detail (how it works)
  - Related technical detail
  - Evidence or rationale

### Changed
- Description of what changed from user perspective
  - Technical details of refactoring
  - Migration notes if breaking change

### Fixed
- Bug description (what was broken for users)
  - Root cause technical explanation
  - Solution approach

### Security
- Vulnerability description (user-facing risk)
  - Technical fix details
  - CVE reference if applicable

## PROJECT STATUS
<!-- Helps agents understand current state -->
**Current Milestone**: M0 ✅ | M1 🚧
**Tests**: X passing (Y unit + Z integration)
**Last Updated**: YYYY-MM-DD
```

**Documentation Gates**:
- ✅ Rustdoc on ALL public items (`cargo doc --no-deps` has zero warnings)
- ✅ Module-level docs explain "why", not just "what"
- ✅ CHANGELOG updated following Keep a Changelog 1.1.0:
  - Removed from `🚧 IN PROGRESS` section
  - Added to correct category (Added/Changed/Deprecated/Removed/Fixed/Security)
  - Main bullet is user-facing "what changed"
  - Sub-bullets provide technical "how it works"
  - No date in bullets (version header has date)
  - Human-focused, not just commit messages
- ❌ NO summary docs (CHANGELOG is single source of truth)
- ✅ Session ends after CHANGELOG update (no redundant files)

**Gate**: ✅ Rustdoc complete + CHANGELOG properly formatted + Skills activated + NO summary docs created

### 5. VALIDATE ✅ (Final Quality Check + Archive)

**Automated Final Checks**:
```bash
/build-and-fix  # Runs cargo check on all workspaces, fixes any remaining errors
/coverage-check # Verifies layer-specific coverage targets met
```

**Agent-Assisted Validation**:
```bash
# Final comprehensive review:
/code-review

# If async-heavy code:
/async-debug-check  # Checks Send bounds, runtime assumptions

# If test coverage concerns:
/coverage-analyze  # Analyzes coverage gaps by layer
```

**Criteria**:
- ✅ Original requirements met (verify against user request)
- ✅ Quality gates passed (see checklist below)
- ✅ No performance regression (if applicable)
- ✅ No security issues (no SQL injection equivalent, no unsafe without docs)

**Performance Checks** (if applicable):
```bash
# Binary size (target: ≤ 10MB stripped)
cargo build --release
strip target/release/toad.exe  # Windows: strip.exe from MSYS2
ls -lh target/release/toad.exe

# Startup time (target: ≤ 100ms)
hyperfine "target/release/toad.exe --version"

# Benchmarks (no regression > 5%)
cargo bench
```

**Security Checks**:
- ✅ No `unsafe` without documented safety invariants
- ✅ No hardcoded credentials/API keys
- ✅ No command injection in tool execution (use proper escaping)
- ✅ Terminal state always restored (panic hook verified)

**Archive Completed Work**:
```bash
# Move dev docs to archive:
mv .toad/active/[task-name] .toad/archive/[task-name]

# Or keep active/ clean:
rm -rf .toad/active/[task-name]  # If documented in CHANGELOG
```

**Final CHANGELOG Update**:
- Remove from `🚧 IN PROGRESS`
- Add to appropriate category (Added/Changed/Fixed/Security)
- Main bullet = user-facing WHAT
- Sub-bullets = technical HOW
- Update PROJECT STATUS if metrics changed

**Gate**: ✅ ALL criteria met + performance acceptable + security validated + dev docs archived + CHANGELOG updated + session ended (no summary docs)

---

## 📝 Documentation: CODE-FIRST (Critical)

**Philosophy**: Implementation details live in CODE (rustdoc), decisions live in ARCHITECTURE.md, work tracking lives in CHANGELOG.md.

### What Goes Where

| Location | Include | Exclude |
|----------|---------|---------|
| **Rustdoc** (`///`, `//!`) | Module purpose, struct/enum/fn descriptions, examples, panics, errors, safety invariants, complexity notes | - |
| **Architecture Docs** | Design decisions (why Elm pattern), cross-cutting patterns (error handling strategy), quality gates | Struct/enum lists, function signatures, module trees (use `cargo doc`) |
| **CHANGELOG.md** | User-facing changes + technical sub-bullets for complex work, IN PROGRESS declarations, PROJECT STATUS | Implementation details (use rustdoc) |
| **CLAUDE.md** | Common commands, high-level architecture, key patterns, development workflow | Obvious instructions, entity lists, generic practices |

### Rustdoc Requirements (Mandatory)

**Module docs**:
```rust
//! Brief module purpose (one line)
//!
//! # Architecture
//! Key design decisions and patterns used
//!
//! # Examples
//! ```
//! use toad::module::Struct;
//! let example = Struct::default();
//! // Show typical usage
//! ```
//!
//! # Feature Flags
//! This module is affected by: `context_ast`, `prompt_caching`
```

**Public item docs**:
```rust
/// Brief description of what this does (one line)
///
/// More detailed explanation if needed. Explain "why" this exists,
/// not just "what" it does.
///
/// # Examples
/// ```
/// use toad::Type;
/// let value = Type::new();
/// assert!(value.is_valid());
/// ```
///
/// # Errors
/// Returns `Err` if X condition occurs
///
/// # Panics
/// Panics if Y is not satisfied (or state "does not panic")
///
/// # Safety (for unsafe fn only)
/// Caller must ensure that...
pub fn example() -> Result<()> { ... }
```

**Verification**:
```bash
# Generate docs and check for warnings
cargo doc --no-deps --document-private-items 2>&1 | grep warning

# Expected output: nothing (zero warnings)
```

### Architecture Docs (Only If Needed)

**When to Update**:
- ✅ New architectural pattern (e.g., adding CQRS, new agent loop)
- ✅ Major technology decision (e.g., switching from X to Y, adding new dependency)
- ✅ Cross-cutting concern change (e.g., new error handling strategy)
- ❌ New struct/enum/function (use rustdoc)
- ❌ New module (use rustdoc module docs)
- ❌ Implementation details (use code comments)

**Keep SHORT**: Introduction + decision rationale only. Details in code.

---

## 🧪 Testing Strategy (Mandatory)

**Philosophy**: Test business logic thoroughly, integration points adequately, happy paths minimally.

**Reference**: `QUALITY_GATES.md` sections 2-3 (single source of truth for thresholds)

### Coverage Targets by Layer

| Layer | Target | Rationale | Files |
|-------|--------|-----------|-------|
| **Models** (`model/`, domain structs) | 95%+ | Pure business logic, no I/O | `evaluation/mod.rs`, `config/mod.rs` |
| **Services** (`agent/`, `llm/`) | 80%+ | Orchestration, can mock I/O | `agent/mod.rs`, `llm/anthropic.rs` |
| **Tools** (`tools/`) | 80%+ | External interactions, mockable | `tools/*.rs` |
| **Infrastructure** (`tui.rs`, `event.rs`) | 60%+ | Harder to test, focus on logic | `tui.rs`, `app.rs` |
| **UI Rendering** (`ui.rs`, `widgets/`) | 40%+ | Visual, test interactions only | `widgets/*.rs` |

### Test Structure

**Unit Tests** (inline in source files):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        let result = function_under_test();
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_condition() {
        let result = function_with_invalid_input();
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "invalid state")]
    fn test_panic_condition() {
        function_that_panics();
    }
}
```

**Integration Tests** (`tests/*.rs`):
```rust
// tests/feature_test.rs
use toad::*;

#[test]
fn test_end_to_end_workflow() {
    // Test cross-module interaction
}

#[tokio::test]
async fn test_async_workflow() {
    // Test async integration
}
```

**Property-Based Tests** (for critical algorithms):
```rust
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_invariant(input in any::<String>()) {
            let result = parse(input);
            // Test invariant holds for all inputs
        }
    }
}
```

### Testing Best Practices

**DO**:
- ✅ Test error paths explicitly
- ✅ Use `#[tokio::test]` for async tests
- ✅ Mock external dependencies (LLM client, file I/O)
- ✅ Use `mockall` crate for trait mocking
- ✅ Test observable behavior, not implementation
- ✅ Use table-driven tests for multiple cases:
  ```rust
  #[test]
  fn test_multiple_cases() {
      let cases = vec![
          ("input1", Ok("output1")),
          ("input2", Err("error")),
      ];
      for (input, expected) in cases {
          assert_eq!(function(input), expected);
      }
  }
  ```

**DON'T**:
- ❌ Skip tests "to save time"
- ❌ Test private implementation details
- ❌ Use `unwrap()` in tests without justification
- ❌ Ignore async runtime context (use `#[tokio::test]`)
- ❌ Test UI rendering pixel-by-pixel (test interactions)

### Validation Commands

```bash
# Run all tests
cargo test

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html --output-dir coverage

# Run specific module tests
cargo test evaluation::

# Run integration tests only
cargo test --test '*'

# Run with output visible
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Coverage Gates

**Overall Project**: 80%+ (verified via `cargo tarpaulin`)
**Core Modules** (evaluation, agent, metrics, stats): 90%+
**Critical Paths** (agent loop, LLM client, statistical validation): 100%

**Verification**:
```bash
cargo tarpaulin --out Html --output-dir coverage
# Open coverage/index.html and verify targets met
```

---

## 🎯 TOAD-Specific Patterns (Verified)

### Dual Architecture Pattern

TOAD has TWO distinct systems:

**M0 Evaluation Framework**:
- **Purpose**: Benchmark AI agents on SWE-bench
- **Key modules**: `evaluation/`, `agent/`, `llm/`, `tools/`, `metrics/`, `stats/`
- **Testing**: Statistical validation (p < 0.05), A/B comparisons
- **Entry point**: `cargo run -- eval|compare|show-config|generate-test-data`

**TUI Application**:
- **Purpose**: Interactive terminal interface
- **Pattern**: Elm Architecture (Model-Update-View)
- **Key modules**: `app.rs`, `ui.rs`, `event.rs`, `tui.rs`, `widgets/`
- **Testing**: Integration tests, manual testing
- **Entry point**: `cargo run -- tui`

**When working on a task, identify EARLY which system you're modifying** to apply correct validation strategy.

### Feature Flag Pattern (M0 only)

```rust
// src/config/mod.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureFlags {
    /// Use AST-based context (tree-sitter parsing)
    /// Evidence: Aider proven, +2-5 points (cAST paper)
    pub context_ast: bool,
    // ... 12 more flags
}

impl FeatureFlags {
    /// M1 baseline: Simple agent (55-60% target)
    pub fn milestone_1() -> Self { /* minimal features */ }

    /// M2 enhanced: + AST + Smart tests (61-66% target)
    pub fn milestone_2() -> Self { /* proven features */ }

    /// M3 advanced: + Multi-model (70-75% target)
    pub fn milestone_3() -> Self { /* experimental features */ }
}
```

**Rules**:
- ✅ Document evidence in comments (research papers, production systems)
- ✅ Default to proven features only (milestone_1 = baseline)
- ✅ New features start disabled, enabled after A/B validation
- ✅ All features must be toggleable independently

### Async Pattern (Tokio)

```rust
// Agent execution (async)
pub async fn execute_task(&self, task: &Task) -> Result<AgentResult> {
    // LLM calls are async
    let response = self.llm_client.send_message(messages).await?;
    // ...
}

// TUI event loop (async with select)
while !app.should_quit() {
    tui.draw(|frame| render(&mut app, frame))?;

    tokio::select! {
        terminal_event = spawn_blocking(handler.next()) => { /* ... */ }
        Some(async_event) = event_rx.recv() => { /* ... */ }
    }
}
```

**Rules**:
- ✅ Use `#[tokio::test]` for async tests
- ✅ Document Send bounds in rustdoc
- ✅ Use `tokio::spawn_blocking` for blocking operations in async context
- ✅ Always use `tokio::select!` for multiple async sources

### Error Handling Pattern

```rust
// Use anyhow::Result for application errors
pub type Result<T> = anyhow::Result<T>;

// Use thiserror for custom errors
#[derive(Debug, thiserror::Error)]
pub enum LLMError {
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("API error: {0}")]
    ApiError(#[from] reqwest::Error),
}

// Never unwrap() in production code
let value = maybe_value?; // ✅ Propagate error
let value = maybe_value.unwrap(); // ❌ FORBIDDEN in src/
let value = maybe_value.expect("justified reason"); // ⚠️ Only if truly impossible to fail
```

**Rules**:
- ✅ All errors implement `std::error::Error`
- ✅ Use `anyhow::Context` to add context to errors
- ✅ Never log AND return error (choose one)
- ✅ Terminal panic hook MUST restore terminal state
- ❌ No `unwrap()` in `src/` (tests are OK)

### Statistical Validation Pattern (M0 only)

```rust
// When comparing configurations
let comparison = ComparisonResult::compare(&results_a, &results_b);

// Check statistical significance
if comparison.p_value < 0.05 {
    // Statistically significant difference
    if comparison.effect_size.abs() > 0.5 {
        // Large effect, consider adopting
    }
}
```

**Rules**:
- ✅ Use Welch's t-test for comparisons (unequal variances)
- ✅ Require p < 0.05 for significance
- ✅ Consider effect size (Cohen's d) not just p-value
- ✅ Run minimum 20 samples for statistical power
- ✅ Document expected impact in feature flag comments

---

## ✅ Quality Gates Checklist

### Pre-Work (Stage 1: ANALYZE)
- [ ] Read `CHANGELOG.md` → `🚧 IN PROGRESS` section (avoid conflicts)
- [ ] Check `.toad/active/` → resume existing task OR start new
- [ ] **Planning mode**: Used for tasks > 3 files or > 100 LOC
- [ ] **Dev docs created**: `.toad/active/[task-name]/` with plan/context/tasks
- [ ] Declare work in `CHANGELOG.md` → `🚧 IN PROGRESS`
- [ ] Identify system: M0 (evaluation) or TUI (interface) or both
- [ ] Success criteria defined (tests pass, benchmarks, accuracy)

### Code Quality (Stage 2: IMPLEMENT)
- [ ] **Implemented in sections** (not all at once, reviewed between phases)
- [ ] **Rustdoc written FIRST** (before code implementation)
- [ ] **Tests written IMMEDIATELY** (in same file, same session)
- [ ] **Dev docs updated** (tasks.md marked, context.md updated)
- [ ] **Hooks passed**:
  - [ ] Build checker: `cargo check` passed (< 5 errors)
  - [ ] Error pattern detector: No unwrap() warnings unaddressed
- [ ] Zero clippy warnings: `cargo clippy -- -D warnings`
- [ ] Formatted: `cargo fmt --check`
- [ ] No `unwrap()` in `src/` (verified via grep AND hooks)
- [ ] No unsafe code OR documented safety invariants
- [ ] Async bounds documented (Send, 'static)

### Testing (Stage 3: INTEGRATE - Mandatory)
- [ ] **Code review run** (`/code-review` between implementation phases)
- [ ] **Dev docs updated before compaction** (`/update-dev-docs`)
- [ ] Test files exist for every impl file
- [ ] Unit tests pass: `cargo test --lib`
- [ ] Integration tests pass: `cargo test --test '*'`
- [ ] Doc tests pass: `cargo test --doc`
- [ ] Coverage meets targets BY LAYER:
  - [ ] Models: 95%+
  - [ ] Services: 80%+
  - [ ] Tools: 80%+
  - [ ] Infrastructure: 60%+
  - [ ] UI: 40%+
- [ ] Property-based tests for critical algorithms (if applicable)
- [ ] No regressions (all existing tests still pass)

### Documentation (Stage 4: DOCUMENT - Mandatory)
- [ ] **Skills auto-activated** (verified via hook injecting "🎯 SKILL ACTIVATION")
- [ ] Rustdoc on ALL modules (`//!`)
- [ ] Rustdoc on ALL public items (`///`)
- [ ] Examples in docs (verified: `cargo doc --open`)
- [ ] Zero doc warnings: `cargo doc --no-deps 2>&1 | grep warning`
- [ ] Complex algorithms have inline comments
- [ ] Feature flags documented with evidence
- [ ] NO entity lists in markdown (use `cargo doc`)

### Architecture Docs (Only If Needed)
- [ ] Updated ONLY if design decisions changed
- [ ] NO new entity/module docs in markdown
- [ ] Kept SHORT (intro + decisions only)
- [ ] Details deferred to rustdoc

### CHANGELOG.md (Mandatory - Keep a Changelog 1.1.0)
- [ ] Created if missing (use template above)
- [ ] Removed from `🚧 IN PROGRESS` section
- [ ] Added to correct category in this order:
  - [ ] Added (new features)
  - [ ] Changed (modifications to existing functionality)
  - [ ] Deprecated (features soon to be removed)
  - [ ] Removed (deleted features)
  - [ ] Fixed (bug fixes)
  - [ ] Security (vulnerability patches)
- [ ] Main bullet describes WHAT changed (user-facing)
- [ ] Sub-bullets add technical HOW (implementation details)
- [ ] Human-focused language (not raw commit messages)
- [ ] No dates in bullets (version header has ISO 8601 date)
- [ ] PROJECT STATUS updated if metrics changed
- [ ] NO summary document created (README_SUMMARY.md, etc.)
- [ ] Session ended (no redundant files)

### Final Validation (Stage 5: VALIDATE)
- [ ] **Automated checks run**:
  - [ ] `/build-and-fix` (cargo check all workspaces)
  - [ ] `/coverage-check` (layer-specific targets)
  - [ ] `/code-review` (final comprehensive review)
- [ ] **Agent validation** (if applicable):
  - [ ] `/async-debug-check` (for async-heavy code)
  - [ ] `/coverage-analyze` (if coverage concerns)

**Domain-Specific**:

**M0 Evaluation Framework**:
- [ ] If feature flag changed: documented evidence
- [ ] If agent changed: run `cargo run -- eval --count 5`
- [ ] If statistical code changed: verify p < 0.05 threshold logic
- [ ] If LLM client changed: verify rate limiting still works
- [ ] If tools changed: test with actual file operations

**TUI Application**:
- [ ] Manual test: `cargo run -- tui` works without panic
- [ ] Terminal restored on Ctrl+C
- [ ] Keyboard navigation works
- [ ] Rendering doesn't exceed 16ms (60 FPS target)
- [ ] No flicker or visual artifacts

**Performance** (If Applicable):
- [ ] Binary size ≤ 10MB stripped
- [ ] Startup time ≤ 100ms
- [ ] No benchmark regression > 5%
- [ ] Memory usage reasonable (idle ≤ 50MB)

**Security**:
- [ ] No hardcoded secrets (grep for `ANTHROPIC_API_KEY`, `sk-`)
- [ ] No command injection (tools use proper escaping)
- [ ] Terminal state restored on panic
- [ ] No unsafe code OR safety invariants documented

**Cleanup**:
- [ ] **Dev docs archived**: Moved to `.toad/archive/` or deleted
- [ ] **CHANGELOG updated**: Removed from IN PROGRESS, added to category
- [ ] **SESSION ENDED**: No summary docs created

---

## 🐘 Problem-Solving: "Eating the Elephant"

**Philosophy**: When stuck, decompose into smaller, independently verifiable chunks.

### Example: Adding New Feature Flag

**Original**: "Add context embeddings feature"

**Decomposed**:
```
1. Add flag to FeatureFlags struct → ✅ compiles, tests pass
2. Add embedding computation fn → ✅ unit tests pass, rustdoc complete
3. Integrate into agent prompt → ✅ integration test passes
4. Add milestone config → ✅ show-config displays correctly
5. Document evidence in comments → ✅ research cited
6. Run A/B comparison → ✅ statistical validation (p < 0.05)
7. Update CHANGELOG → ✅ session complete
```

**Each step MUST pass its quality gates before proceeding to next.**

### Example: Fixing TUI Bug

**Original**: "Fix flickering in help screen"

**Decomposed**:
```
1. Reproduce bug → ✅ identified trigger condition
2. Add test for bug → ✅ test fails (confirms bug)
3. Fix rendering logic → ✅ test passes
4. Manual validation → ✅ no flicker in `cargo run -- tui`
5. Update rustdoc → ✅ explain fix in code comments
6. Update CHANGELOG → ✅ session complete
```

### Common Pitfalls

**DON'T**:
- ❌ Work on multiple modules simultaneously (conflicts)
- ❌ Skip tests "to save time" (breaks later)
- ❌ Add features without evidence (violates research-driven approach)
- ❌ Create summary docs (CHANGELOG is single source of truth)
- ❌ Forget to declare work in `🚧 IN PROGRESS` (multi-agent conflicts)

**DO**:
- ✅ One module at a time, complete quality gates before next
- ✅ Test immediately (TDD where possible)
- ✅ Document evidence for new features (research papers, production systems)
- ✅ Update CHANGELOG as final step, end session
- ✅ Declare work early, remove when complete

---

## 🔗 References (Verified Sources)

**Project Documentation**:
- `README.md` - Project overview, quick start, status (verified: exists)
- `ARCHITECTURE.md` - Layered architecture, Elm pattern, quality gates (verified: exists)
- `QUALITY_GATES.md` - Specific thresholds and validation commands (verified: exists)
- `CLAUDE.md` - Common commands, high-level architecture (verified: created)
- `CHANGELOG.md` - Work tracking, IN PROGRESS, PROJECT STATUS (create if missing)

**Rust Resources**:
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Official style guide
- [Effective Rust](https://www.lurklurk.org/effective-rust/) - Best practices
- [Rustdoc Guide](https://doc.rust-lang.org/rustdoc/index.html) - Documentation standards
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async patterns

**TOAD Research**:
- `AI_CODING_AGENTS_RESEARCH_REPORT.md` - SOTA agent analysis (verified: exists)
- `LLM_CONTEXT_OPTIMIZATION_RESEARCH.md` - Context management (verified: exists)
- `llm_routing_research_report.md` - LLM routing strategies (verified: exists)

**Testing Resources**:
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html) - Official testing chapter
- [Mockall Documentation](https://docs.rs/mockall/) - Mocking framework
- [Proptest Documentation](https://docs.rs/proptest/) - Property-based testing

**Tools**:
- `cargo-tarpaulin` - Coverage reporting (install: `cargo install cargo-tarpaulin`)
- `cargo-audit` - Security vulnerability scanning (install: `cargo install cargo-audit`)
- `cargo-udeps` - Unused dependency detection (install: `cargo install cargo-udeps`)
- `hyperfine` - Benchmarking (install: `cargo install hyperfine`)

---

## 📋 Quick Command Reference

```bash
# Quality Gate Validation (run before committing)
cargo fmt --check && \
cargo clippy -- -D warnings && \
cargo test && \
cargo doc --no-deps

# Coverage Report
cargo tarpaulin --out Html --output-dir coverage

# M0 Validation
cargo run -- show-config --milestone 1
cargo run -- eval --count 5
cargo run -- compare --baseline 1 --test 2 --count 20

# TUI Validation
cargo run -- tui

# Performance Check
cargo build --release
hyperfine "target/release/toad.exe --version"

# Security Audit
cargo audit
grep -r "unwrap()" src/  # Should return empty for production code
```

---

## 🛠️ Automation Setup (Required Infrastructure)

The 5-stage workflow above requires automation infrastructure. Set these up once:

### 1. Dev Docs Directory Structure

**Purpose**: Prevents Claude from "losing the plot" across compactions.

**Setup** (one-time):
```bash
mkdir -p .toad/active
mkdir -p .toad/archive
```

**File Template** (auto-created by `/create-dev-docs`):
```markdown
# [task-name]-plan.md
## Feature: [Name]
### Phases
1. Phase 1: [Description]
   - Task 1.1
   - Task 1.2
2. Phase 2: [Description]
...

# [task-name]-context.md
## Key Files
- `src/agent/mod.rs` - Agent execution loop (line 45: MAX_AGENT_STEPS)
- `src/llm/anthropic.rs` - LLM client (uses reqwest for API calls)

## Decisions Made
- Using Welch's t-test because variances may be unequal
- Agent loop limited to 25 steps to prevent infinite loops

## Last Updated: YYYY-MM-DD HH:MM

# [task-name]-tasks.md
## Implementation Checklist
- [x] Create Task struct with serde
- [x] Add unit tests for Task::example()
- [ ] Implement agent loop
  - [x] Message sending
  - [ ] Tool execution
  - [ ] Result aggregation
- [ ] Integration tests
```

### 2. Hooks (Zero Errors Left Behind)

**Purpose**: Automated quality enforcement without manual checking.

**Required Hooks** (`.claude/hooks/`):

#### `post-tool-use.ts` (Edit Tracker)
```typescript
// Runs after Edit/Write operations
// Logs: file paths, repo/workspace, timestamps
// Used by stop hook to determine what to check
```

#### `stop.ts` (Build Checker + Error Pattern Detector)
```typescript
// Runs when Claude finishes responding

// 1. Build Checker:
//    - Reads edit logs from post-tool-use
//    - Runs `cargo check` on affected workspace members
//    - If < 5 errors: Shows immediately
//    - If ≥ 5 errors: Suggests /build-and-fix or cargo-error-resolver agent

// 2. Error Pattern Detector:
//    - Scans edited files for:
//      * unwrap() in src/ (not tests/)
//      * unsafe without safety docs ("// SAFETY: ...")
//      * todo!() or unimplemented!()
//      * Missing error handling in Result types
//    - Shows gentle reminder (non-blocking):
//      "⚠️ Pattern Detected: unwrap() on line 45
//       Did you replace with proper error handling?"
```

#### `user-prompt-submit.ts` (Skill Auto-Activator)
```typescript
// Runs BEFORE Claude sees your prompt

// Analyzes prompt for:
// - Keywords: "agent", "LLM", "evaluation" → m0-evaluation skill
// - Keywords: "TUI", "widget", "render" → tui-development skill
// - Keywords: "async", "tokio", "Send" → rust-best-practices skill

// Checks file context:
// - Editing src/agent/*.rs → m0-evaluation skill
// - Editing src/ui.rs or src/widgets/*.rs → tui-development skill

// Injects BEFORE your prompt:
// "🎯 SKILL ACTIVATION: Use m0-evaluation skill
//  (Detected: editing src/agent/mod.rs)"
```

**Installation**: Copy hook templates to `.claude/hooks/`, customize for TOAD workspace structure.

### 3. Skills (Pattern Enforcement)

**Purpose**: Consistent code patterns without repeating instructions.

**Rule**: Main skill file MUST be < 500 lines (Anthropic best practice for token efficiency).

**TOAD Skills Structure**:
```
.claude/skills/
├── rust-best-practices.md (400 lines)
│   └── resources/
│       ├── error-handling.md
│       ├── async-patterns.md
│       └── testing-patterns.md
├── m0-evaluation.md (350 lines)
│   └── resources/
│       ├── statistical-validation.md
│       └── agent-loop.md
└── tui-development.md (380 lines)
    └── resources/
        ├── elm-architecture.md
        └── ratatui-widgets.md
```

**Creation**:
```bash
# Create skill files in .claude/skills/
# Keep main file < 500 lines
# Link to resource files for details
```

**Content Guidelines**:
- Main file: Patterns, quick reference, "when to use X"
- Resource files: Detailed examples, edge cases, rationale
- Include verification commands (`cargo doc`, `cargo check`)
- Attach utility scripts (testing, validation)

**Token Savings**: 40-60% compared to monolithic docs (verified from 6 months production use).

### 4. Specialized Agents

**Purpose**: Automated reviews, debugging, planning without manual work.

**Required Agents** (`.claude/agents/`):

#### Quality Control
- **rust-code-reviewer** - Checks unwrap(), unsafe, error handling, test coverage, rustdoc
- **cargo-error-resolver** - Systematically fixes compiler/clippy errors
- **test-coverage-analyzer** - Verifies layer-specific coverage targets met

#### Planning
- **strategic-plan-architect** - Creates comprehensive plans (phases, risks, metrics, timeline)
- **refactor-planner** - Safe refactoring plans with test preservation strategy

#### Debugging
- **async-debugger** - Diagnoses Send bound issues, tokio runtime problems
- **compile-error-explainer** - Explains complex Rust errors with examples and fixes

**Usage**: Integrated into workflow (Stage 1: planning, Stage 3: reviews, Stage 5: validation).

### 5. Custom Slash Commands

**Purpose**: Eliminate repetitive prompting.

**Required Commands** (`.claude/commands/`):

```bash
/strategic-plan [task-name]   # Enter planning mode with research
/create-dev-docs [task-name]  # Generate plan.md, context.md, tasks.md
/update-dev-docs [task-name]  # Update before compaction
/code-review                   # Launch rust-code-reviewer agent
/build-and-fix                 # cargo check all + fix errors
/coverage-check                # Verify layer-specific targets
/async-debug-check             # Check Send bounds, runtime issues
```

**Implementation**: Each command = `.md` file with full prompt template.

### 6. Complete Directory Structure

```
toad/
├── .toad/                          # Task tracking (survives compaction)
│   ├── active/                     # Current work
│   │   └── [task-name]/
│   │       ├── plan.md             # Phases, timeline, tasks
│   │       ├── context.md          # Key files, decisions, next steps
│   │       └── tasks.md            # [ ] checklist
│   └── archive/                    # Completed (reference)
│
├── .claude/                        # Automation infrastructure
│   ├── hooks/                      # Auto-enforcement
│   │   ├── post-tool-use.ts        # Edit tracker
│   │   ├── stop.ts                 # Build checker + error detector
│   │   └── user-prompt-submit.ts   # Skill auto-activator
│   │
│   ├── skills/                     # Pattern libraries (< 500 lines each)
│   │   ├── rust-best-practices.md  # Error handling, async, testing
│   │   │   └── resources/
│   │   │       ├── error-handling.md
│   │   │       ├── async-patterns.md
│   │   │       └── testing-patterns.md
│   │   ├── m0-evaluation.md        # Agent loop, LLM, stats, feature flags
│   │   │   └── resources/
│   │   │       ├── statistical-validation.md
│   │   │       └── agent-loop.md
│   │   └── tui-development.md      # Elm arch, ratatui, widgets
│   │       └── resources/
│   │           ├── elm-architecture.md
│   │           └── ratatui-widgets.md
│   │
│   ├── agents/                     # Specialized assistants
│   │   ├── rust-code-reviewer/
│   │   ├── cargo-error-resolver/
│   │   ├── strategic-plan-architect/
│   │   └── async-debugger/
│   │
│   └── commands/                   # Slash commands
│       ├── strategic-plan.md
│       ├── create-dev-docs.md
│       ├── update-dev-docs.md
│       ├── code-review.md
│       ├── build-and-fix.md
│       └── coverage-check.md
│
├── src/                            # Source code
├── CHANGELOG.md                    # Single source of truth (work tracking)
├── CLAUDE.md                       # Project-specific quick reference
├── RUST_WORKFLOW.md                # This file (5-stage workflow)
└── QUALITY_GATES.md                # Validation thresholds
```

**Setup Time**: ~2-4 hours initial setup. Pays for itself after first major feature.

---

**This workflow is the single source of truth for agent-specific development practices in TOAD.**

**Last Updated**: 2025-11-08
**Verified**: All commands tested on Windows 11, Rust 1.89.0

**Inspiration**: Workflow incorporates best practices from [diet103/claude-code-infrastructure-showcase](https://github.com/diet103/claude-code-infrastructure-showcase) adapted for Rust development.
