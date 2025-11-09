# TOAD AI Development Checklist

**Project:** TOAD - Terminal-Oriented Autonomous Developer
**Goal:** Build an evidence-based AI coding terminal to rival Cursor/Claude Code
**Approach:** Quality-gated incremental development with A/B testing
**Last Updated:** 2025-11-08

---

## 📊 Overall Progress

- [x] **M0: Infrastructure & Evaluation** (100% - 45 tests)
- [x] **M1: Simple Baseline** (100% - 90 tests)
- [ ] **M2: Context + Routing** (0% - Next)
- [ ] **M3: Multi-Model + Intelligence** (0%)
- [ ] **M4: Advanced Features** (0%)
- [ ] **M5: Production Ready** (0%)

**Current Status:** ✅ M1 Complete, Ready for Quality Gates

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

**Status:** COMPLETE (90 tests passing, 1 ignored)
**Commits:** 6 commits (99a2279 → b722f8d)
**Target Accuracy:** 55-60% on SWE-bench

### Tool System (8 Tools - 42 tests)
- [x] Tool trait with async execution
- [x] ToolRegistry with m1_baseline()
- [x] ToolResult for success/error handling
- [x] JSON schema for LLM tool use
- [x] **ReadTool** - Read file contents (4 tests)
- [x] **WriteTool** - Write files with directory creation (4 tests)
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

### Quality Gates (PENDING - Requires API Key)
- [ ] **QG1:** Run on 10 tasks, verify no crashes
- [ ] **QG2:** Run on 50 tasks, measure baseline (target: 55-60%)
- [ ] Document baseline metrics
- [ ] Identify failure patterns

**Next Steps:**
1. Set `ANTHROPIC_API_KEY` environment variable
2. Run: `cargo run --release -- eval --count 10 --milestone 1`
3. Run: `cargo run --release -- eval --count 50 --milestone 1`
4. Document results in `M1_BASELINE_RESULTS.md`

---

## 🔄 Milestone 2: Context + Routing

**Status:** NOT STARTED (0%)
**Prerequisites:** M1 baseline measured
**Target:** +5% accuracy vs M1 (60-65%)

### AST-Based Context (Feature: context_ast)
- [ ] Add tree-sitter dependency
- [ ] Implement AST parser for common languages
  - [ ] Python parser
  - [ ] JavaScript/TypeScript parser
  - [ ] Rust parser (optional)
- [ ] Context extraction from AST
  - [ ] Function definitions
  - [ ] Class structures
  - [ ] Import statements
- [ ] AST-based code search
- [ ] Integration with PromptBuilder
- [ ] Tests (target: 15 tests)

### Semantic Routing (Feature: routing_semantic)
- [ ] Implement routing logic
- [ ] Task classification system
- [ ] Route selection based on task type
- [ ] Fallback to M1 baseline
- [ ] Tests (target: 8 tests)

### Optimizations
- [ ] Prompt caching implementation (Feature: prompt_caching)
  - [ ] Cache key generation
  - [ ] Cache hit/miss tracking
  - [ ] Integration with Anthropic API
- [ ] Tree-sitter validation (Feature: tree_sitter_validation)
  - [ ] Syntax validation before file write
  - [ ] Parse error detection
  - [ ] Tests (target: 6 tests)

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
```

---

## 📈 Current Metrics

### Test Coverage
- **Total:** 100 tests (100 passing, 6 ignored)
- **M0:** 45 tests (infrastructure) + 6 validation tests
- **M1:** 45 tests (tools + llm + agent)
- **Integration:** 5 tests (end-to-end workflows)
- **Ignored:** 6 tests (require ANTHROPIC_API_KEY)
- **Target for M2:** +30 tests
- **Target for M3:** +36 tests

### Code Statistics
- **Total Lines:** ~4,800
- **Tool System:** ~1,900 lines
- **LLM Client:** ~900 lines (including rate limiting)
- **Agent Loop:** ~480 lines
- **Test Code:** ~1,400 lines

### Documentation
- README.md
- ARCHITECTURE.md
- ITERATIVE_IMPLEMENTATION_PLAN.md
- M0_VALIDATION.md
- M0_FINAL_AUDIT.md
- M1_IMPLEMENTATION_SUMMARY.md
- USAGE.md
- AI_TODO.md (this file)

---

## 🎯 Immediate Next Steps

**Priority 1: Validate M1 Baseline**
1. [ ] Set `ANTHROPIC_API_KEY` environment variable
2. [ ] Run Quality Gate 1 (10 tasks):
   ```bash
   cargo run --release -- eval --count 10 --milestone 1 --output ./qg1
   ```
3. [ ] Verify no crashes, review results
4. [ ] Run Quality Gate 2 (50 tasks):
   ```bash
   cargo run --release -- eval --count 50 --milestone 1 --output ./qg2
   ```
5. [ ] Document baseline metrics in `M1_BASELINE_RESULTS.md`

**Priority 2: Analyze M1 Results**
6. [ ] Calculate accuracy percentage
7. [ ] Analyze failure patterns
8. [ ] Identify common error types
9. [ ] Review cost per task
10. [ ] Check token usage patterns

**Priority 3: Decision Point**
- If accuracy **55-60%**: ✅ Proceed to M2
- If accuracy **50-55%**: 🔍 Analyze and optimize M1
- If accuracy **<50%**: ⚠️ Debug M1 issues

**Priority 4: Begin M2 (If M1 Validated)**
11. [ ] Design AST integration approach
12. [ ] Add tree-sitter dependencies
13. [ ] Implement Python AST parser
14. [ ] Create M2 quality gate plan

---

## 🔧 Technical Debt & Improvements

### Code Quality
- [x] Fix 17 compiler warnings (unused imports, etc.) ✅ **Completed 2025-11-09**
- [x] Add rustfmt.toml for consistent formatting ✅ **Completed 2025-11-09**
- [x] Add clippy configuration ✅ **Completed 2025-11-09**
- [x] Implement suggested fixes from `cargo fix` ✅ **Completed 2025-11-09**
  - Zero compiler warnings
  - Zero clippy warnings
  - 100 tests passing (89 lib + 5 integration + 6 m0_validation)
  - 6 tests require API key (marked as #[ignore])

### Testing
- [ ] Increase integration test coverage
- [ ] Add property-based tests (proptest)
- [ ] Add benchmark suite
- [ ] Mock external dependencies better

### Documentation
- [ ] Add inline code examples
- [ ] Improve error message clarity
- [ ] Add troubleshooting section
- [ ] Create video tutorial (future)

### Performance
- [ ] Profile agent execution
- [ ] Optimize prompt building
- [ ] Reduce allocations in hot paths
- [ ] Consider async batching for multiple tasks

---

## 📚 References

- **SWE-bench:** https://www.swebench.com/
- **Aider:** https://aider.chat/ (64.3% baseline to beat)
- **Anthropic API Docs:** https://docs.anthropic.com/
- **Tree-sitter:** https://tree-sitter.github.io/
- **Ratatui:** https://ratatui.rs/

---

## 🎓 Key Learnings

### What Worked Well
1. **Quality-gated approach:** Prevented feature creep, ensured each piece works
2. **Comprehensive testing:** 90 tests caught issues early
3. **Incremental development:** M0 → M1 progression kept momentum
4. **Rate limiting:** Prevented API issues before they happened
5. **Documentation first:** Clear goals and success criteria

### What to Improve
1. **Earlier integration testing:** Caught harness integration late
2. **API key validation:** Should check early in development
3. **Mock LLM client:** Needed for faster testing
4. **Cost estimation:** Should estimate before running full eval

### Decisions Made
1. **Single-agent over multi-agent:** Simpler for M1, can add later
2. **Tool-first approach:** Let LLM choose tools vs hard-coded workflow
3. **Conservative rate limits:** 80% of max to prevent issues
4. **Async everywhere:** Future-proof for concurrency

---

**Last Updated:** 2025-11-08
**Next Review:** After M1 Quality Gates Complete
**Maintained By:** Claude AI Development Team
