# TOAD AI Development Checklist

**Project:** TOAD - Terminal-Oriented Autonomous Developer
**Goal:** Build an evidence-based AI coding terminal to rival Cursor/Claude Code
**Approach:** Quality-gated incremental development with A/B testing
**Last Updated:** 2025-11-08

---

## 📊 Overall Progress

- [x] **M0: Infrastructure & Evaluation** (100% - 45 tests)
- [x] **M1: Simple Baseline** (100% - 96 tests)
- [x] **M2: Context + Routing** (90% - 74 tests) ✅ NEW
- [ ] **M3: Multi-Model + Intelligence** (0%)
- [ ] **M4: Advanced Features** (0%)
- [ ] **M5: Production Ready** (0%)

**Current Status:** ✅ M1 + M2 Implementation Complete, Ready for Quality Gates
**Total Tests:** 1635 passing (1620 base + 15 smart test selection)

---

## ✅ Milestone 0: Infrastructure & Evaluation Framework

**Status:** COMPLETE (45 tests passing)
**Commits:** 3 commits (918420f, e6ac461, 724a6b2)

### Core Infrastructure
- [x] Configuration system with 13 feature flags
- [x] Milestone progression (M1 ⊂ M2 ⊂ M3)
- [x] Feature flag serialization/deserialization
- [x] Environment variable support

### Evaluation Framework
- [x] Task structure (SWE-bench compatible)
- [x] TaskResult with metrics
- [x] EvaluationHarness for running experiments
- [x] TaskLoader for dataset management
- [x] DatasetManager for SWE-bench integration
- [x] ExperimentManager for tracking experiments

### Metrics System
- [x] MetricsCollector with comprehensive tracking
- [x] Cost tracking (USD per task)
- [x] Token counting (input/output/cached)
- [x] Duration tracking (first response, total)
- [x] Quality metrics (edit distance, AST similarity)
- [x] Aggregate metrics computation

### Statistical Framework
- [x] Welch's t-test implementation
- [x] Cohen's d effect size calculation
- [x] ComparisonResult with decision logic
- [x] Sample size validation
- [x] Quality gates (Adopt/Reject/Investigate/NeedMoreData)

### Documentation
- [x] M0_VALIDATION.md - Completeness checklist
- [x] M0_FINAL_AUDIT.md - Best practices audit
- [x] Architecture documentation

**Quality Gates:**
- [x] All 45 tests passing
- [x] Feature flag validation (13/13)
- [x] Experimental design complete (10/10 elements)
- [x] Statistical framework validated

---

## ✅ Milestone 1: Simple Baseline Agent

**Status:** 100% IMPLEMENTATION COMPLETE (1619 tests passing)
**Commits:** 9 commits (99a2279 → 6a3472e)
**Target Accuracy:** 55-60% on SWE-bench

**Completed Features:**
- [x] Prompt caching: IMPLEMENTED ✅ (3 new tests)
  - Cache control on system + tools
  - Beta header auto-added
  - Wired to config.features.prompt_caching
  - 90% cost reduction on repeated prompts

- [x] Tree-sitter validation: IMPLEMENTED ✅ (6 new tests)
  - Validates Python, JavaScript, TypeScript syntax before write
  - Prevents writing syntactically invalid code
  - Graceful fallback for unsupported file types
  - Wired to config.features.tree_sitter_validation

### Tool System (8 Tools - 48 tests)
- [x] Tool trait with async execution
- [x] ToolRegistry with m1_baseline() + m1_with_features()
- [x] ToolResult for success/error handling
- [x] JSON schema for LLM tool use
- [x] **ReadTool** - Read file contents (4 tests)
- [x] **WriteTool** - Write files with validation (10 tests ✅ +6 validation)
- [x] **ListTool** - List directory contents (5 tests)
- [x] **EditTool** - Search/replace editing (6 tests)
- [x] **BashTool** - Execute shell commands, cross-platform (6 tests)
- [x] **GrepTool** - Pattern search with context (7 tests)
- [x] **GitDiffTool** - Show repository changes (5 tests)
- [x] **GitStatusTool** - Show repository status (5 tests)

### LLM Client (12 tests)
- [x] Anthropic Claude Sonnet 4 integration (2 tests)
- [x] Message/Role types for conversations
- [x] ToolUse support for function calling
- [x] Usage tracking with cost calculation (3 tests)
- [x] LLMError with 9 error types (3 tests)
- [x] Retry logic (retryable vs permanent errors)
- [x] Rate limiting for API safety (4 tests)
    - [x] 50 RPM, 30K ITPM, 8K OTPM limits
    - [x] Conservative mode (80% of max)
    - [x] Sliding window implementation

### Agent System (6 tests)
- [x] Agent loop with tool execution (2 tests)
- [x] Iterative problem solving (max 25 steps)
- [x] Tool discovery and registration
- [x] Metrics collection integration
- [x] PromptBuilder for task-specific prompts (4 tests)
- [x] Aider-inspired system prompt
- [x] Error handling and recovery

### Integration
- [x] Agent + EvaluationHarness integration
- [x] CLI commands (eval, compare, show-config, generate-test-data)
- [x] Results persistence (JSON)
- [x] Logging and tracing

### Documentation
- [x] M1_IMPLEMENTATION_SUMMARY.md
- [x] USAGE.md with command examples
- [x] Cross-platform support notes

### Quality Gates (BLOCKED - Requires ANTHROPIC_API_KEY)
**Implementation:** ✅ 100% COMPLETE
**Validation:** ⏸️ PENDING API KEY

- [ ] **QG1:** Run on 10 tasks, verify no crashes
- [ ] **QG2:** Run on 50 tasks, measure baseline (target: 55-60%)
- [ ] Document baseline metrics
- [ ] Identify failure patterns

**To Run Quality Gates:**
```bash
export ANTHROPIC_API_KEY="sk-ant-..."  # Required
cargo run --release -- eval --count 10 --milestone 1
cargo run --release -- eval --count 50 --milestone 1
```

**Implementation Complete:**
- All 8 tools implemented with 48 tests ✅
- Prompt caching enabled (90% cost reduction) ✅
- Tree-sitter validation enabled ✅
- Multi-provider LLM support (Anthropic/GitHub/Ollama) ✅
- Agent loop with max 25 steps ✅
- Metrics collection and persistence ✅

---

## ✅ Milestone 2: Context + Routing

**Status:** 90% IMPLEMENTATION COMPLETE (74 tests passing)
**Commits:** 8 commits (f05ae9f → 26b6a0a)
**Prerequisites:** M1 baseline measured
**Target:** +5% accuracy vs M1 (60-65%)

**Completed:**
- [x] AST infrastructure with tree-sitter (59 tests)
  - Python/JavaScript/TypeScript parsers
  - ContextBuilder with fluent API
  - PromptBuilder integration
  - Agent integration with feature flag
  - eval_runner wiring with graceful fallback

- [x] Smart test selection (15 tests) ✅ NEW
  - Test file discovery (Python/Rust/JS/TS)
  - Dependency mapping (source → test files)
  - Git diff integration for change detection
  - Language-specific test command generation
  - Evidence: AutoCodeRover proven, +3-5 points

**Deferred (Not Required for Core M2):**
- [ ] Semantic routing (not critical for initial M2, separate milestone)

### AST-Based Context (Feature: context_ast) - ✅ FULLY INTEGRATED
- [x] Add tree-sitter dependency (54 tests)
- [x] Implement AST parser for common languages
    - [x] Python parser (5 tests)
    - [x] JavaScript/TypeScript parser (12 tests)
    - [ ] Rust parser (optional - grammar already added)
- [x] Context extraction from AST
    - [x] Function definitions with signatures
    - [x] Class structures with docstrings
    - [x] Import statements
- [x] ExtractorRegistry for auto-detection (11 tests)
- [x] ContextBuilder with fluent API (8 tests)
- [x] Integration with PromptBuilder (1 test)
- [x] **Wire to Agent** - ✅ COMPLETE (eval_runner.rs:233-257)
- [x] Tests (54/15 target ✅ infrastructure complete)

### Smart Test Selection (Feature: smart_test_selection) - ✅ IMPLEMENTED
- [x] Test file discovery (3 tests)
  - Multi-language patterns (Python, Rust, JS/TS)
  - Directory tree walking
  - Smart exclusions (node_modules, target, etc.)
- [x] Dependency analysis (4 tests)
  - Name-based matching (test_foo.py ↔ foo.py)
  - Directory similarity calculation
  - Source-to-test file mapping
- [x] Selective test running (3 tests)
  - Git diff integration
  - Test selection with fallback
  - Reduction metrics tracking
- [x] Test execution (4 tests)
  - Python: pytest with specific files
  - Rust: cargo test with modules
  - JavaScript/TypeScript: npm test with files
- [x] Tests (15/8 target ✅ exceeded)

### Semantic Routing (Feature: routing_semantic) - ⏸️ DEFERRED
- [ ] Implement routing logic
- [ ] Task classification system
- [ ] Route selection based on task type
- [ ] Fallback to M1 baseline
- [ ] Tests (target: 8 tests)

### Quality Gates
- [ ] A/B test M1 vs M2 (30 tasks minimum)
- [ ] Statistical significance (p < 0.05)
- [ ] Decision: Adopt/Reject/Investigate
- [ ] Document results
- [ ] If Adopt: Update baseline to M2

**Success Criteria:**
- Accuracy improvement: ≥+2% AND p < 0.05
- Cost increase: <+20%
- No regressions in stability

---

## 🎯 Milestone 3: Multi-Model + Intelligence

**Status:** NOT STARTED (0%)
**Prerequisites:** M2 validated
**Target:** +8% accuracy vs M1 (63-68%)

### Multi-Model Ensemble (Feature: routing_multi_model)
- [ ] Add support for multiple LLM providers
    - [ ] OpenAI GPT-4 integration
    - [ ] Anthropic Claude Opus integration
    - [ ] Model selection strategy
- [ ] Ensemble voting mechanism
- [ ] Cost optimization (use cheaper models when possible)
- [ ] Tests (target: 12 tests)

### Vector Embeddings (Feature: context_embeddings)
- [ ] Add embedding model (text-embedding-3-small)
- [ ] Vector database integration (in-memory for M3)
- [ ] Semantic code search
- [ ] Context ranking by relevance
- [ ] Tests (target: 10 tests)

### Smart Test Selection (Feature: smart_test_selection)
- [ ] Test file discovery
- [ ] Dependency analysis
- [ ] Selective test running
- [ ] Test result caching
- [ ] Tests (target: 8 tests)

### Failure Memory (Feature: failure_memory)
- [ ] Error pattern tracking
- [ ] Similar failure detection
- [ ] Suggested fixes from history
- [ ] Memory persistence
- [ ] Tests (target: 6 tests)

### Quality Gates
- [ ] A/B test M2 vs M3 (50 tasks)
- [ ] Statistical validation
- [ ] Cost analysis
- [ ] Decision and documentation

**Success Criteria:**
- Accuracy improvement: ≥+5% vs M2
- Effect size: Cohen's d > 0.3 (medium)
- Cost: Managed within budget

---

## 🚀 Milestone 4: Advanced Features

**Status:** NOT STARTED (0%)
**Prerequisites:** M3 validated
**Target:** +10% accuracy vs M1 (65-70%)

### Code Graph Analysis (Feature: context_graph)
- [ ] Build code dependency graph
- [ ] Call graph analysis
- [ ] Data flow tracking
- [ ] Impact analysis for changes
- [ ] Tests (target: 12 tests)

### Context Re-ranking (Feature: context_reranking)
- [ ] Implement re-ranking algorithm
- [ ] Relevance scoring
- [ ] Integration with context selection
- [ ] Tests (target: 6 tests)

### Speculative Execution (Feature: routing_speculative)
- [ ] Parallel execution paths
- [ ] Result selection strategy
- [ ] Cost management
- [ ] Tests (target: 8 tests)

### Opportunistic Planning (Feature: opportunistic_planning)
- [ ] Multi-step planning
- [ ] Plan refinement
- [ ] Backtracking on failure
- [ ] Tests (target: 8 tests)

### Semantic Caching (Feature: semantic_caching)
- [ ] Semantic similarity detection
- [ ] Cache by meaning, not exact match
- [ ] Integration with prompt system
- [ ] Tests (target: 6 tests)

### Quality Gates
- [ ] A/B test M3 vs M4 (50 tasks)
- [ ] Performance benchmarking
- [ ] Statistical validation
- [ ] Cost-benefit analysis

**Success Criteria:**
- Accuracy: Approaching SOTA (65-70%)
- Maintained or improved cost efficiency
- No stability regressions

---

## 🏆 Milestone 5: Production Ready

**Status:** NOT STARTED (0%)
**Prerequisites:** M4 validated
**Target:** Production deployment

### Performance Optimization
- [ ] Profiling and benchmarking
- [ ] Memory optimization
- [ ] Latency reduction
- [ ] Batch processing optimization

### Reliability
- [ ] Comprehensive error handling
- [ ] Retry mechanisms
- [ ] Graceful degradation
- [ ] Circuit breakers
- [ ] Health checks

### Monitoring & Observability
- [ ] Structured logging
- [ ] Metrics export (Prometheus format)
- [ ] Tracing integration (OpenTelemetry)
- [ ] Dashboard templates
- [ ] Alerting rules

### Security
- [ ] API key management
- [ ] Secrets handling
- [ ] Input sanitization
- [ ] Rate limiting (production-grade)
- [ ] Audit logging

### Documentation
- [ ] Complete API documentation
- [ ] User guide
- [ ] Deployment guide
- [ ] Troubleshooting guide
- [ ] Architecture decision records (ADRs)

### Testing
- [ ] Integration test suite (100+ tests)
- [ ] Load testing
- [ ] Chaos testing
- [ ] Security testing
- [ ] Performance regression tests

### Deployment
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Release automation
- [ ] Version management
- [ ] Rollback procedures

**Success Criteria:**
- 99.9% uptime
- <100ms p95 latency for non-LLM operations
- Zero critical security vulnerabilities
- Complete documentation
- Production monitoring

---

## 🎨 Future: Interactive TUI Mode

**Status:** PLANNED (Design Phase)
**Prerequisites:** M1 validated
**Note:** This is independent of M2-M5 autonomous improvements

### Project Structure Reorganization
- [ ] Create `src/autonomous/` for M1-M5 agent code
- [ ] Create `src/interactive/` for TUI mode
- [ ] Create `src/shared/` for common code
- [ ] Update module paths and imports
- [ ] Ensure all tests still pass

### Core TUI Infrastructure
- [ ] Add dependencies (ratatui, crossterm)
- [ ] Event loop implementation
- [ ] Terminal management
- [ ] Screen routing system
- [ ] State management

### UI Components
- [ ] Chat interface widget
- [ ] Code editor widget
- [ ] File tree widget
- [ ] Command palette
- [ ] Status bar
- [ ] Toast notifications
- [ ] Dialog boxes

### Features
- [ ] Real-time chat with agent
- [ ] File browsing and editing
- [ ] Git integration UI
- [ ] Search and filtering
- [ ] Session management
- [ ] Theme system

### Integration
- [ ] Connect TUI to existing Agent
- [ ] Use same LLM client
- [ ] Share tool system
- [ ] Unified configuration

### Quality Gates
- [ ] Responsive UI (<16ms frame time)
- [ ] Memory usage <100MB
- [ ] Cross-platform testing (Linux, macOS, Windows)
- [ ] Keyboard navigation complete
- [ ] User testing feedback

**Launch Command:**
```bash
cargo run --tui  # Interactive mode
cargo run --eval # Autonomous mode (existing)