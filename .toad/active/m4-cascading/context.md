# M4 Context: Cascading Routing + Cost Optimization

## Background

M4 implements the DavaJ cascading routing approach to achieve 70% cost reduction while maintaining accuracy. Unlike M3 (parallel racing), M4 uses **sequential** routing: start with cheap local models, escalate to expensive cloud models only when necessary.

## Key Research

**DavaJ (2024):**
- 84.7% accuracy on HumanEval
- 70% cost reduction vs cloud-only
- Approach: Local 7B → Local 32B → Cloud Premium
- Key insight: 40% of tasks are "easy" and can be solved with free local models

## Current State

### ✅ Infrastructure Complete
From commit 16691e0, we have:

1. **CascadingRouter** (`src/ai/routing/cascade.rs`):
   - 4-tier model selection
   - Local-first strategy (Ollama qwen2.5-coder)
   - Cloud fallback (Claude Sonnet/Opus)
   - 9 tests passing

2. **TaskClassifier** (`src/ai/routing/classifier.rs`):
   - Easy/Medium/Hard classification
   - Heuristics: statement length, file count, keywords
   - 3 tests passing

3. **Router Trait** (`src/ai/routing/mod.rs`):
   - Abstraction for routing strategies
   - Used by evaluation harness
   - 1 integration test

4. **Evaluation Integration** (`src/ai/evaluation/mod.rs:315-333`):
   - 3-way routing: M4 cascade | M3 racing | M1/M2 direct
   - Already wired and functional
   - Creates LLMClient from ProviderConfig

### 🔧 Missing Components

**What M4 Needs:**
1. **CascadeMetadata** - Track routing decisions (like M3's RaceMetadata)
2. **Integration Tests** - Verify M4 config and cascade behavior
3. **Documentation** - CHANGELOG and TODO_AI updates

**What M4 Does NOT Need:**
- Enhanced features (embeddings, failure memory, etc.) → Deferred to M5
- These are nice-to-have, not evidence-based requirements for M4

## Architecture

### Data Flow

```
Task → TaskClassifier → Difficulty → CascadingRouter → ModelTier → ProviderConfig → LLMClient
                                                                                        ↓
                                                                                    Agent.execute()
                                                                                        ↓
                                                                                   TaskResult
                                                                                        ↓
                                                                            cascade_metadata (NEW)
```

### Cascade Logic

```rust
match difficulty {
    Easy => Local7B (qwen2.5-coder:7b) - $0
    Medium => Local32B (qwen2.5-coder:32b) - $0  
    Hard => CloudPremium (Sonnet 4) - $2
}
```

**Cloud-only mode** (no Ollama):
```rust
match difficulty {
    Easy => CloudPremium - $2
    Medium => CloudPremium - $2
    Hard => CloudBest (Opus 4) - $10
}
```

## Key Files

### Files to Modify
- `src/ai/evaluation/mod.rs` - Add CascadeMetadata struct and extraction
- `CHANGELOG.md` - Document M4 completion
- `TODO_AI.md` - Mark M4 as complete

### Files Already Complete (No Changes)
- `src/ai/routing/cascade.rs` - CascadingRouter (9 tests)
- `src/ai/routing/classifier.rs` - TaskClassifier (3 tests)
- `src/ai/routing/mod.rs` - Router trait (1 test)
- `src/config/mod.rs` - routing_cascade feature flag

## Feature Flags

M4 uses:
- `routing_cascade` - Main M4 feature ✅
- `context_ast` - From M2 ✅
- `smart_test_selection` - From M2 ✅
- `prompt_caching` - From M1 ✅
- `routing_multi_model` - M3 (should work together!)

**Important:** M4 and M3 can coexist! The routing logic is:
```rust
if config.features.routing_cascade {
    // M4: Cascade
} else if config.features.routing_multi_model {
    // M3: Racing
} else {
    // M1/M2: Direct
}
```

## Metadata Design

Following M3 RaceMetadata pattern:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeMetadata {
    /// Task difficulty classification
    pub task_difficulty: String, // "Easy" | "Medium" | "Hard"
    
    /// Selected model tier
    pub selected_tier: String, // "Local7B" | "Local32B" | "CloudPremium" | "CloudBest"
    
    /// Estimated cost for this tier
    pub tier_cost_usd: f64,
    
    /// Time spent classifying + routing
    pub routing_duration_ms: u64,
}
```

Added to TaskResult:
```rust
pub struct TaskResult {
    // ... existing fields ...
    
    /// Cascade metadata (M4 only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascade_metadata: Option<CascadeMetadata>,
}
```

## Cost Model

### SWE-bench Task Distribution (Estimated)
- Easy (40%): 200 tasks × $0 = $0
- Medium (40%): 200 tasks × $0 = $0  
- Hard (20%): 100 tasks × $2 = $200

**Total: $200 for 500 tasks**

### Cloud-Only Baseline
- All tasks: 500 × $2 = $1,000

**Savings: $800 (80%)**

DavaJ achieved 70%, we're targeting 80% with better classification.

## Testing Strategy

### Unit Tests (Already Done)
- CascadingRouter: 9 tests ✅
- TaskClassifier: 3 tests ✅
- Router trait: 1 test ✅

### Integration Tests (New)
1. **test_m4_config_has_cascading_enabled()** - Verify M4 config
2. **test_m4_cascade_metadata_serialization()** - JSON round-trip
3. **test_cascade_tier_selection_easy()** - Easy task → Local7B
4. **test_cascade_tier_selection_hard()** - Hard task → CloudPremium
5. **test_cascade_cost_tracking()** - Verify cost calculations

### Quality Gates
- All 18+ tests passing (13 routing + 5+ integration)
- Zero rustdoc warnings
- CHANGELOG and TODO_AI updated
- Ready for A/B test: M3 vs M4 (50 tasks minimum)

## Implementation Notes

### Keep It Simple
- M4 core = cascade metadata + tests
- No embeddings, no failure memory, no graph analysis
- Those are M5 features (nice-to-have, not evidence-based)

### Parallel with M3
- M4 and M3 are independent routing strategies
- User can toggle between them via feature flags
- Future: Could combine (cascade THEN race within tier)

### Ollama Requirement
- M4 local tiers require Ollama running on localhost:11434
- Cloud-only mode works without Ollama (uses routing_cascade but no local models)
- Document setup in CHANGELOG

## Success Criteria

1. ✅ CascadeMetadata struct added
2. ✅ cascade_metadata field in TaskResult  
3. ✅ Metadata extraction in run_task()
4. ✅ 5+ integration tests passing
5. ✅ Documentation complete
6. ✅ Total 18+ tests passing
7. ✅ Ready for A/B testing

