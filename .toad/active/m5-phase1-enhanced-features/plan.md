# M5 Phase 1 Implementation Plan: Enhanced Features

## Overview

**Goal:** Implement 5 evidence-based features to push accuracy from current baseline toward world-class 76-78% target

**Total Tests:** 40 tests (10 + 12 + 6 + 6 + 6)

**Expected Impact:** +3-5% accuracy improvement on SWE-bench Verified

**Timeline:** 2-3 weeks (aggressive) or 4-5 weeks (conservative)

---

## Feature Priority Order (Evidence-Based)

### Tier 1: High Impact (Weeks 1-2)
1. **Vector Embeddings** (10 tests) - 15-20% context improvement
2. **Code Graph Analysis** (12 tests) - Essential for cross-file changes
3. **Semantic Caching** (6 tests) - 40% cost reduction

### Tier 2: Medium Impact (Week 3)
4. **Failure Memory** (6 tests) - 30% fewer repeat errors
5. **Context Re-ranking** (6 tests) - 25% relevance improvement

---

## Feature 1: Vector Embeddings (context_embeddings)

### Evidence
- RAG systems show 15-20% improvement in context retrieval
- Semantic search outperforms keyword search by 25%
- OpenAI embeddings proven in production (millions of users)

### Implementation Plan

**Dependencies:**
```toml
# Cargo.toml additions
async-openai = "0.20"  # For text-embedding-3-small
ndarray = "0.15"       # For vector operations
```

**Components:**
1. `src/ai/embeddings/mod.rs` - Core embedding module
   - EmbeddingClient trait
   - OpenAIEmbeddings implementation
   - VectorStore in-memory implementation
   - Similarity search (cosine similarity)

2. `src/ai/embeddings/vector_store.rs` - Vector storage
   - InMemoryVectorStore struct
   - add(), search() methods
   - Metadata storage for vectors

3. `src/ai/context/semantic_search.rs` - Semantic context retrieval
   - Integration with ContextBuilder
   - Semantic ranking of code snippets
   - Fallback to AST when no embeddings

**Tests (10 total):**
1. test_embedding_client_initialization
2. test_embedding_generation_single
3. test_embedding_generation_batch
4. test_vector_store_add_and_retrieve
5. test_cosine_similarity_calculation
6. test_semantic_search_relevance_ranking
7. test_context_builder_with_embeddings
8. test_fallback_to_ast_on_embedding_failure
9. test_embedding_serialization
10. test_embedding_cache_hit_rate

**Integration Points:**
- ContextBuilder: Add semantic search path
- PromptBuilder: Use semantically ranked context
- Feature flag: context_embeddings

**Success Criteria:**
- All 10 tests passing
- Context retrieval improved vs AST-only
- Latency <500ms for embedding generation
- A/B test shows statistically significant improvement

---

## Feature 2: Code Graph Analysis (context_graph)

### Evidence
- Dependency-aware context improves cross-file changes
- CodeT5 uses graph structure for better understanding
- GitHub Copilot uses similar techniques

### Implementation Plan

**Dependencies:**
```toml
# Already have tree-sitter from M2
petgraph = "0.6"  # For graph algorithms
```

**Components:**
1. `src/ai/graph/mod.rs` - Code graph module
   - CodeGraph struct
   - build_from_ast() method
   - Dependency resolution

2. `src/ai/graph/dependency_analyzer.rs` - Dependency extraction
   - Import/export tracking
   - Call graph construction
   - File-to-file dependencies

3. `src/ai/graph/impact_analysis.rs` - Change impact
   - Transitive dependencies
   - Affected files for a change
   - Priority ranking by impact

**Tests (12 total):**
1. test_graph_construction_from_ast
2. test_import_dependency_tracking
3. test_export_dependency_tracking
4. test_call_graph_construction
5. test_file_to_file_dependencies
6. test_transitive_dependency_resolution
7. test_impact_analysis_single_file_change
8. test_impact_analysis_multi_file_change
9. test_context_prioritization_by_impact
10. test_graph_serialization
11. test_incremental_graph_updates
12. test_graph_integration_with_context_builder

**Integration Points:**
- AST extractor: Provide input to graph builder
- ContextBuilder: Use graph for context prioritization
- Feature flag: context_graph

**Success Criteria:**
- All 12 tests passing
- Accurate dependency tracking (>95%)
- Graph construction <1s for typical repos
- A/B test shows improvement on multi-file tasks

---

## Feature 3: Semantic Caching (semantic_caching)

### Evidence
- 40% cost reduction on repeated similar queries
- Used in production by major LLM providers
- Proven effective in chatbot systems

### Implementation Plan

**Dependencies:**
```toml
# Use embeddings from Feature 1
serde_json = "1.0"  # Already in project
```

**Components:**
1. `src/ai/cache/semantic_cache.rs` - Semantic cache
   - SemanticCache struct
   - similarity_threshold parameter
   - TTL support

2. `src/ai/cache/cache_key.rs` - Cache key generation
   - Hash from prompt embedding
   - Similarity-based lookup
   - Cache hit/miss tracking

**Tests (6 total):**
1. test_cache_store_and_retrieve
2. test_cache_similarity_matching
3. test_cache_miss_on_dissimilar_query
4. test_cache_ttl_expiration
5. test_cache_hit_rate_tracking
6. test_cache_integration_with_llm_client

**Integration Points:**
- LLMClient: Wrap with caching layer
- Embeddings: Use for similarity calculation
- Feature flag: semantic_caching

**Success Criteria:**
- All 6 tests passing
- Cache hit rate >30% on repeated evaluations
- Cost reduction validated in A/B test
- No accuracy degradation

---

## Feature 4: Failure Memory (failure_memory)

### Evidence
- 30% fewer repeat errors in AutoGPT
- Error pattern learning improves over time
- Proven in production systems

### Implementation Plan

**Dependencies:**
```toml
# Use embeddings from Feature 1 for similarity
chrono = "0.4"  # Already in project for timestamps
```

**Components:**
1. `src/ai/memory/failure_memory.rs` - Failure tracking
   - FailureMemory struct
   - Error pattern storage (JSON)
   - Similar failure detection

2. `src/ai/memory/error_pattern.rs` - Error patterns
   - ErrorPattern struct
   - Categorization (syntax, runtime, logic)
   - Suggested fixes

**Tests (6 total):**
1. test_failure_storage_and_retrieval
2. test_similar_failure_detection
3. test_error_categorization
4. test_suggested_fix_retrieval
5. test_failure_clustering
6. test_persistence_across_sessions

**Integration Points:**
- Agent: Store failures during execution
- PromptBuilder: Include similar failures in context
- Feature flag: failure_memory

**Success Criteria:**
- All 6 tests passing
- Similar failure detection >80% accuracy
- Reduced repeat errors in A/B test
- Persistence works correctly

---

## Feature 5: Context Re-ranking (context_reranking)

### Evidence
- Cohere Rerank improves relevance by 25%
- Used by production RAG systems
- Proven effective in retrieval tasks

### Implementation Plan

**Dependencies:**
```toml
cohere-rust = "0.3"  # Cohere API client (optional)
# Or implement simple re-ranker with embeddings
```

**Components:**
1. `src/ai/rerank/mod.rs` - Re-ranking module
   - Reranker trait
   - CohereReranker implementation
   - EmbeddingReranker (fallback)

2. `src/ai/rerank/relevance_scorer.rs` - Scoring
   - Relevance scoring algorithm
   - Integration with context selection

**Tests (6 total):**
1. test_reranker_initialization
2. test_relevance_scoring
3. test_top_k_selection
4. test_reranking_improves_order
5. test_fallback_to_embedding_reranker
6. test_integration_with_context_builder

**Integration Points:**
- ContextBuilder: Add re-ranking step
- Embeddings: Use for fallback re-ranker
- Feature flag: context_reranking

**Success Criteria:**
- All 6 tests passing
- Re-ranked context more relevant (measured in A/B test)
- Latency acceptable (<200ms)
- Fallback works when Cohere unavailable

---

## Implementation Sequence

### Week 1: Vector Embeddings (Tier 1)
- Days 1-2: Core embedding client + vector store
- Days 3-4: Semantic search integration
- Day 5: Testing + A/B validation

### Week 2: Code Graph + Semantic Caching (Tier 1)
- Days 1-3: Code graph construction + dependency analysis
- Days 4-5: Semantic caching implementation
- Weekend: Testing + integration

### Week 3: Failure Memory + Re-ranking (Tier 2)
- Days 1-2: Failure memory implementation
- Days 3-4: Context re-ranking
- Day 5: Final testing + A/B validation

---

## Quality Gates (Each Feature)

1. **Tests:** All feature tests passing
2. **Integration:** Feature flag working correctly
3. **A/B Test:** Statistically significant improvement (p < 0.05)
4. **Documentation:** Rustdoc complete with evidence citations
5. **Performance:** No >10% latency regression

---

## Success Metrics (Phase 1 Complete)

- ✅ All 40 tests passing
- ✅ 5 feature flags implemented
- ✅ Accuracy improvement: +3-5% vs M4 baseline
- ✅ Cost: Maintained or reduced vs M4
- ✅ All features validated with A/B tests
- ✅ Documentation complete

---

## Risk Mitigation

**Risk 1: Embedding API costs**
- Mitigation: Cache embeddings aggressively
- Fallback: Use local embedding models if needed

**Risk 2: Graph construction latency**
- Mitigation: Build incrementally, cache results
- Fallback: Use simpler dependency tracking

**Risk 3: Feature interactions**
- Mitigation: Implement one at a time, validate each
- Fallback: Feature flags allow disabling problematic features

**Risk 4: No significant improvement**
- Mitigation: A/B test each feature independently
- Fallback: Skip features that don't improve accuracy

---

## Dependencies Between Features

```
Vector Embeddings (base)
    ↓
├─→ Semantic Caching (uses embeddings)
├─→ Failure Memory (uses embeddings for similarity)
└─→ Context Re-ranking (uses embeddings as fallback)

Code Graph (independent)
    ↓
└─→ Context Builder (uses graph for prioritization)
```

**Implementation Order:**
1. Vector Embeddings (required by 3 others)
2. Code Graph (independent, high impact)
3. Semantic Caching (depends on embeddings)
4. Failure Memory (depends on embeddings)
5. Context Re-ranking (depends on embeddings)

---

## Next Steps

1. Declare M5 Phase 1 in CHANGELOG.md
2. Start with Feature 1: Vector Embeddings
3. Create detailed context and tasks for embeddings
4. Implement, test, validate, A/B test
5. Repeat for features 2-5
