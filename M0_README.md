# TOAD - Milestone 0: Infrastructure and Evaluation Framework

## Overview

Milestone 0 (M0) implements the core evaluation infrastructure for TOAD (Terminal-Oriented Autonomous Developer). This provides the foundation for iterative, data-driven development of AI coding agent features.

**Status:** ✅ COMPLETE (Week 1)

## What's Implemented

### 1. Core Evaluation Types (`src/evaluation/`)

- **Task**: Represents a single SWE-bench coding task
  - Problem statement, repository info, test patches
  - Complexity categorization (Simple/Medium/Hard)
  - Ground truth solution for validation

- **TaskResult**: Captures execution metrics for a task
  - Solved status, test results
  - Cost (USD), duration (ms), API calls
  - Full metrics integration

- **EvaluationHarness**: Orchestrates running evaluations
  - Run single configuration
  - A/B comparison of two configurations
  - Results persistence to JSON

- **TaskLoader**: Loads tasks from SWE-bench datasets
  - JSON parsing from official datasets
  - Stratified sampling by complexity
  - Test data generation for development

### 2. Feature Flag System (`src/config/`)

- **FeatureFlags**: 12 toggleable features for experiments
  - Context strategies (AST, embeddings, graph, reranking)
  - Routing strategies (semantic, multi-model, speculative)
  - Intelligence features (smart tests, failure memory, planning)
  - Optimizations (caching, validation)

- **ToadConfig**: Complete configuration management
  - Milestone-specific presets (M1, M2, M3)
  - Serializable for reproducibility
  - Feature descriptions for transparency

### 3. Metrics Collection (`src/metrics/`)

- **Metrics**: Comprehensive per-task measurements
  - Accuracy: solved, quality scores
  - Cost: USD, API calls, token counts (input/output/cached)
  - Performance: duration, latency, retrieval time
  - Behavior: edits, file ops, test runs, agent steps

- **MetricsCollector**: Real-time tracking during execution
  - Start/stop timing
  - Incremental metric recording
  - Snapshot support

- **AggregateMetrics**: Statistical summaries across runs
  - Mean, std dev, min, max
  - Percentiles (p50, p95, p99)
  - Accuracy rates by complexity

### 4. Statistical Testing (`src/stats/`)

- **ComparisonResult**: A/B test analysis
  - Delta metrics (accuracy, cost, duration)
  - Welch's t-test for significance (p-values)
  - Automatic recommendations (Adopt/Reject/Investigate/NeedMoreData)

- **Decision Criteria** (from implementation plan):
  ```
  ✅ ADOPT if:
    - Accuracy +2% AND cost <+20%
    - Cost -20% AND accuracy maintained
    - Latency -30% AND cost/accuracy acceptable

  ❌ REJECT if:
    - No improvement (<+1%)
    - Cost >+30% for <+2% accuracy
  ```

- **StatisticalTest**: Utilities
  - Cohen's d effect size
  - Sample size adequacy checks

### 5. CLI Interface (`src/main.rs`)

Four commands for evaluation workflows:

```bash
# Show feature configuration
toad show-config --milestone 1

# Generate test data
toad generate-test-data --count 50 --output test_data.json

# Run evaluation
toad eval --count 10 --milestone 1 --output ./results

# A/B comparison
toad compare --baseline 1 --test 2 --count 20 --output ./results
```

### 6. Test Suite

- **18 unit tests** covering all modules (100% pass)
- **8 integration tests** for end-to-end workflows
- Test coverage for:
  - Configuration serialization
  - Milestone progression
  - Task complexity estimation
  - Metrics aggregation
  - Statistical comparisons
  - Evaluation harness

## Project Structure

```
toad/
├── src/
│   ├── lib.rs              # Library root
│   ├── main.rs             # CLI entry point
│   ├── config/
│   │   └── mod.rs          # Feature flags & config
│   ├── evaluation/
│   │   ├── mod.rs          # Core evaluation types
│   │   └── task_loader.rs  # SWE-bench loading
│   ├── metrics/
│   │   └── mod.rs          # Metrics collection
│   └── stats/
│       └── mod.rs          # Statistical testing
├── tests/
│   └── integration_test.rs # Integration tests
├── Cargo.toml              # Dependencies
└── M0_README.md           # This file
```

## Dependencies

- **tokio**: Async runtime
- **serde/serde_json**: Serialization
- **anyhow/thiserror**: Error handling
- **tracing**: Logging
- **statrs**: Statistical functions
- **chrono**: Timestamps
- **clap**: CLI parsing

## Usage Examples

### Example 1: Show Milestone Configurations

```bash
$ cargo run -- show-config --milestone 2

=== Milestone 2 Configuration ===

Enabled features: 4/12

Context Strategies:
  AST-based context:        true
  Vector embeddings:        false
  Code graph analysis:      false
  Re-ranking:               false

Routing Strategies:
  Semantic router:          false
  Multi-model ensemble:     false
  Speculative execution:    false

Intelligence Features:
  Smart test selection:     true
  Failure memory:           false
  Opportunistic planning:   false

Optimizations:
  Prompt caching:           true
  Tree-sitter validation:   true
```

### Example 2: Run Evaluation

```bash
$ cargo run -- eval --count 10 --milestone 1

INFO TOAD v0.1.0 - Terminal-Oriented Autonomous Developer
INFO Running evaluation...
INFO Generating 10 test tasks
INFO Loaded 10 tasks
INFO Using Milestone 1 configuration

=== Evaluation Results: 2 features ===
Accuracy: 40.00% (4/10)
Avg Cost: $0.0100/task
Avg Duration: 1.00s/task
Total Tasks: 10

INFO Results saved to: "./results"
```

### Example 3: A/B Comparison

```bash
$ cargo run -- compare --baseline 1 --test 2 --count 20

=== A/B Test Comparison ===
Config A: 2 features
Config B: 4 features

Delta Metrics:
  Accuracy: +5.00% (p=0.0234)
  Cost: +0.0005 USD (+5.0%)
  Duration: +50 ms (+5.0%)

Statistical Significance:
  Accuracy significant: true (p < 0.05)
  Cost significant: false

Recommendation: Investigate
🔍 INVESTIGATE - marginal improvement, consider tradeoffs
```

## Validation Gates

M0 establishes the quality gates for all future milestones:

1. **Statistical Significance**: p < 0.05 for accuracy improvements
2. **Cost Ceiling**: <20% increase for +2% accuracy
3. **Sample Size**: Minimum 20-50 tasks for meaningful comparisons
4. **Reproducibility**: All results saved with configurations

## Next Steps (Milestone 1)

Milestone 1 will implement the basic agent loop using this evaluation framework:

1. **Agent Core** (Weeks 2-4)
   - Tool execution engine (read, write, edit, bash, etc.)
   - LLM integration (Claude Sonnet 4)
   - Basic prompting

2. **Target**: 55-60% on test set
3. **Validation**: Use M0 harness to measure against baseline

## Testing

Run all tests:
```bash
cargo test
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

## Building

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run -- eval --count 5
```

## Metrics

M0 implementation metrics:

- **Lines of Code**: ~2,000 (excluding tests)
- **Test Coverage**: 26 tests, all passing
- **Build Time**: <10s cold, <1s warm
- **Module Count**: 4 core modules
- **Feature Flags**: 12 toggleable features
- **CLI Commands**: 4 commands

## Key Design Decisions

1. **Feature Flags Over Monolith**: Every innovation is toggleable for A/B testing
2. **Statistical Rigor**: Welch's t-test + decision criteria prevent premature adoption
3. **Separation of Concerns**: Evaluation, metrics, and statistics are independent modules
4. **Test-First**: Integration tests before implementation ensures API stability
5. **JSON Persistence**: All results serializable for analysis in external tools

## Evidence-Based Approach

From ARCHITECTURE_DATA_ANALYSIS.md, M0 enables testing these claims:

- **AST chunking**: Expected +2-5% (cAST paper)
- **Smart test selection**: Expected +3-5% (AutoCodeRover)
- **Multi-model ensemble**: Expected +4-5% (TRAE vs Warp)
- **Prompt caching**: Expected -90% cost (Anthropic)

Each will be validated with M0's statistical framework before adoption.

## License

This is a research project. See repository LICENSE for details.

---

**Milestone 0 Complete** ✅
**Ready for Milestone 1 Implementation** 🚀
