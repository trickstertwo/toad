# M1 Baseline Results

**Status:** PENDING - Awaiting ANTHROPIC_API_KEY

## Overview

This document will contain the baseline evaluation results for M1 (Simple Baseline Agent) on SWE-bench tasks.

**Target Metrics:**
- Accuracy: 55-60% on SWE-bench verified
- Cost: < $0.50 per task
- Duration: < 5 minutes per task

## Implementation Status

✅ **100% IMPLEMENTATION COMPLETE** (Commit: 6a3472e)

**Features Implemented:**
- 8 baseline tools (Read, Write, List, Edit, Bash, Grep, GitDiff, GitStatus)
- Prompt caching (90% cost reduction)
- Tree-sitter validation (Python, JavaScript, TypeScript)
- Multi-provider LLM support (Anthropic, GitHub, Ollama)
- Agent loop with 25-step limit
- Metrics collection and persistence

**Test Coverage:**
- Total: 1619 tests passing
- M1 Tools: 48 tests
- M1 Agent: 6 tests
- M1 LLM: 15 tests (12 client + 3 caching)
- M0 Framework: 45 tests
- M2 AST: 59 tests

## Quality Gate 1: Smoke Test (10 tasks)

**Command:**
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
cargo run --release -- eval --count 10 --milestone 1 --swebench verified
```

**Expected:**
- No crashes
- All 10 tasks complete
- Cost < $5 total
- Average duration < 5 minutes

**Results:** *PENDING API KEY*

## Quality Gate 2: Baseline Measurement (50 tasks)

**Command:**
```bash
cargo run --release -- eval --count 50 --milestone 1 --swebench verified
```

**Expected:**
- Accuracy: 55-60%
- Average cost: $0.30-0.50 per task
- Average duration: 2-5 minutes per task
- Success rate: > 95% (task completion, not solving)

**Results:** *PENDING API KEY*

## Error Analysis

*To be populated after QG2*

### Common Failure Patterns
- [ ] Tool selection errors
- [ ] Context limitation issues
- [ ] Syntax errors (should be prevented by tree-sitter)
- [ ] Test execution failures
- [ ] Git operation failures

### Cost Breakdown
- [ ] Input tokens
- [ ] Output tokens
- [ ] Cached tokens (should be ~90% after first request)
- [ ] Cache hit rate

## Comparison vs. SOTA

**Target:** 55-60% accuracy (competitive with OpenDevin baseline)

*Actual results pending API key...*

## Next Steps After Validation

1. If accuracy ≥ 55%: Proceed to M2 (AST context)
2. If accuracy < 55%: Investigate and fix issues
3. Run A/B test: M1 baseline vs M2 AST context
4. Document failure patterns for future improvements

---

**To populate this document:**
```bash
# 1. Set API key
export ANTHROPIC_API_KEY="your-key-here"

# 2. Run QG1 (smoke test)
cargo run --release -- eval --count 10 --milestone 1 --swebench verified > qg1.log 2>&1

# 3. Review results and update this doc

# 4. Run QG2 (baseline)
cargo run --release -- eval --count 50 --milestone 1 --swebench verified > qg2.log 2>&1

# 5. Analyze results and complete this document
```
