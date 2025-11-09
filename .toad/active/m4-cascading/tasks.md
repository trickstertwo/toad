# M4 Tasks Checklist

## Phase 1: Cascade Metadata & Integration Tests

### Task 1.1: Add CascadeMetadata Struct
- [ ] Add `CascadeMetadata` struct to `src/ai/evaluation/mod.rs`
  - [ ] task_difficulty: String field
  - [ ] selected_tier: String field  
  - [ ] tier_cost_usd: f64 field
  - [ ] routing_duration_ms: u64 field
  - [ ] Derive: Debug, Clone, Serialize, Deserialize

### Task 1.2: Update TaskResult
- [ ] Add `cascade_metadata: Option<CascadeMetadata>` field
- [ ] Add `#[serde(skip_serializing_if = "Option::is_none")]` attribute

### Task 1.3: Extract Cascade Metadata in run_task()
- [ ] Track routing start time before classifier.classify()
- [ ] Store difficulty classification result
- [ ] Store selected tier from router
- [ ] Calculate routing duration
- [ ] Populate cascade_metadata in TaskResult
- [ ] Add logging for M4 cascade decisions

### Task 1.4: Integration Tests
- [ ] test_m4_config_has_cascading_enabled()
  - Verify FeatureFlags::milestone_4() has routing_cascade=true
  - Verify inherits M3 features (routing_multi_model, etc.)
  - Verify has M4-specific features
  
- [ ] test_m4_cascade_metadata_serialization()
  - Create CascadeMetadata instance
  - Serialize to JSON
  - Deserialize from JSON
  - Verify fields match
  
- [ ] test_cascade_tier_selection_easy()
  - Create easy task ("Fix typo in README")
  - Classify difficulty
  - Verify Easy classification
  - Verify Local7B tier selection
  
- [ ] test_cascade_tier_selection_hard()
  - Create hard task (architecture refactor)
  - Classify difficulty  
  - Verify Hard classification
  - Verify CloudPremium tier selection
  
- [ ] test_cascade_cost_tracking()
  - Verify tier_cost_usd calculations
  - Easy: $0
  - Medium: $0
  - Hard: $2 (CloudPremium)

### Task 1.5: Verification
- [ ] Run `cargo test --lib routing` - Verify 13 tests pass
- [ ] Run `cargo test --lib test_m4` - Verify 5+ tests pass
- [ ] Run `cargo test --lib test_cascade` - Verify all tests pass
- [ ] Total: 18+ tests passing

## Phase 2: Enhanced Features (DEFERRED TO M5)

**Note:** These are NOT required for M4 baseline

### Vector Embeddings (Future)
- [ ] Deferred to M5

### Failure Memory (Future)  
- [ ] Deferred to M5

### Code Graph (Future)
- [ ] Deferred to M5

### Context Reranking (Future)
- [ ] Deferred to M5

### Semantic Caching (Future)
- [ ] Deferred to M5

## Phase 3: Documentation

### Task 3.1: Update CHANGELOG.md
- [ ] Add M4 section under "### Added"
- [ ] Document cascading routing feature
- [ ] Document DavaJ research (70% cost reduction)
- [ ] List 4-tier model selection
- [ ] Mention Ollama requirement for local tiers
- [ ] Note M4 test count (18+ tests)

### Task 3.2: Update TODO_AI.md  
- [ ] Mark M4 as 100% complete
- [ ] Update "Overall Progress" section
- [ ] Update "Current Status" line
- [ ] Update "Total Tests" count
- [ ] Add M4 section with checkmarks
- [ ] Document deferred features (moved to M5)

### Task 3.3: Verify Rustdoc
- [ ] Check routing module rustdoc completeness
- [ ] Verify CascadeMetadata has doc comments
- [ ] Verify all public items documented
- [ ] Run `cargo doc --no-deps --open` to review

### Task 3.4: Ollama Setup Guide
- [ ] Verify CHANGELOG mentions Ollama requirement
- [ ] Document localhost:11434 default
- [ ] Note qwen2.5-coder:7b and :32b models
- [ ] Explain cloud-only mode alternative

## Phase 4: Commit & Push

### Task 4.1: Commit Phase 1
- [ ] Stage files: `git add src/ai/evaluation/mod.rs`
- [ ] Commit: `[M4] Phase 1: Add cascade metadata tracking`
- [ ] Include test count in commit message

### Task 4.2: Commit Documentation
- [ ] Stage files: `git add CHANGELOG.md TODO_AI.md`
- [ ] Commit: `[M4] Phase 3: Complete M4 documentation`
- [ ] Include final test count

### Task 4.3: Push to Remote
- [ ] Push: `git push -u origin claude/setup-server-011CUwCwjx6XyGAkKq7m1u1c`
- [ ] Verify push succeeded

## Success Criteria

- [x] Plan created (.toad/active/m4-cascading/plan.md)
- [x] Context documented (.toad/active/m4-cascading/context.md)  
- [x] Tasks checklist created (this file)
- [ ] CascadeMetadata struct implemented
- [ ] TaskResult updated with cascade_metadata
- [ ] Metadata extraction in run_task()
- [ ] 5+ integration tests passing
- [ ] 18+ total tests passing (13 routing + 5+ integration)
- [ ] CHANGELOG.md updated
- [ ] TODO_AI.md updated
- [ ] All commits pushed
- [ ] M4 marked as 100% complete

## Timeline Estimate

- Phase 1 (Metadata + Tests): 30-45 minutes
- Phase 3 (Documentation): 15-20 minutes
- Phase 4 (Commit & Push): 5 minutes

**Total:** ~60 minutes for M4 baseline completion
