# M4 Implementation Plan: Cascading Routing + Cost Optimization

## Status Assessment

### ✅ Already Complete (commit 16691e0)
- CascadingRouter with 4-tier model selection (13 tests)
- TaskClassifier for difficulty analysis (3 tests)
- Evaluation harness integration (3-way routing: M4/M3/M1-M2)
- ProviderConfig abstraction for Anthropic + Ollama
- Ollama setup documentation

### 🔧 Missing Components

**Phase 1: M4 Integration Tests** (PRIORITY)
- Integration tests verifying M4 config creates proper routing
- Cost tracking metadata (similar to M3 RaceMetadata)
- Cascade decision logging (which tier was selected)
- Serialization tests for cascade metadata

**Phase 2: Enhanced Features** (MOVED FROM M3)
These were originally planned for M3 but deferred to M4:
- Vector embeddings (context_embeddings feature)
- Failure memory (failure_memory feature)  
- Code graph analysis (context_graph feature)
- Context reranking (context_reranking feature)
- Semantic caching (semantic_caching feature)

**Phase 3: Documentation**
- CHANGELOG.md update
- TODO_AI.md progress tracking
- Rustdoc for routing module completeness
- Ollama setup verification guide

## Implementation Phases

### Phase 1: Cascade Metadata & Integration Tests (Priority)
**Goal:** Track cascade routing decisions like M3 tracks race winners

**Tasks:**
1. Add `CascadeMetadata` struct to evaluation/mod.rs
   - task_difficulty: Difficulty
   - selected_tier: ModelTier
   - tier_cost_usd: f64
   - routing_duration_ms: u64
   
2. Add `cascade_metadata: Option<CascadeMetadata>` to TaskResult

3. Extract cascade metadata in run_task() after routing
   - Store difficulty classification result
   - Log tier selection and estimated cost
   
4. Add integration tests (5 tests minimum):
   - test_m4_config_has_cascading_enabled()
   - test_m4_cascade_metadata_serialization()
   - test_cascade_tier_selection_easy()
   - test_cascade_tier_selection_hard()
   - test_cascade_cost_tracking()

**Success Criteria:** 18+ tests total (13 existing + 5 new)

### Phase 2: Optional Enhanced Features (Future Work)
**Note:** These are M5 candidates, not blockers for M4

**Vector Embeddings:**
- Add text-embedding-3-small dependency
- In-memory vector store
- Semantic code search for context retrieval
- Target: 10 tests

**Failure Memory:**
- JSON-based error pattern storage
- Similar failure detection via embeddings
- Suggested fixes from history
- Target: 6 tests

**Code Graph:**
- Build dependency graph with tree-sitter
- Call graph analysis
- Impact analysis for changes
- Target: 12 tests

**Skipping for M4 baseline** - Focus on proven cascading routing first

### Phase 3: Documentation & Validation
1. Update CHANGELOG.md with M4 cascading details
2. Update TODO_AI.md to mark M4 as complete
3. Verify all 18+ tests passing
4. Document Ollama setup requirements
5. Commit and push

## Evidence Base

**DavaJ Research:**
- 84.7% HumanEval accuracy
- 70% cost reduction vs cloud-only
- Local-first cascade: 7B → 32B → Cloud

**TOAD Implementation:**
- 4 tiers: Local7B, Local32B, CloudPremium, CloudBest
- Task classifier: Easy/Medium/Hard based on heuristics
- Already proven with 13 passing tests

## Timeline

**Phase 1 (Integration):** 30-45 minutes
- CascadeMetadata struct: 5 min
- Integration with TaskResult: 10 min  
- 5 integration tests: 20 min
- Verification: 10 min

**Phase 3 (Documentation):** 15-20 minutes
- CHANGELOG: 5 min
- TODO_AI: 10 min
- Commit & push: 5 min

**Total M4 Core:** ~60 minutes (Phase 2 deferred to M5)

## Success Metrics

- Tests: 18+ passing (13 routing + 5+ integration)
- Documentation: Complete rustdoc + CHANGELOG
- Integration: Full wiring to evaluation harness ✅ (already done)
- Cost model: 70% reduction validated in tests
- Ready for A/B testing: YES
