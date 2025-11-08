# TOAD Usage Guide

## Prerequisites

1. **Set API Key:**
   ```bash
   export ANTHROPIC_API_KEY="your-anthropic-api-key"
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

## Commands

### 1. Show Configuration

View the feature flags for any milestone:

```bash
# M1 baseline (simple)
cargo run --release -- show-config --milestone 1

# M2 (AST + routing)
cargo run --release -- show-config --milestone 2

# M3 (everything)
cargo run --release -- show-config --milestone 3
```

### 2. Run Evaluation

Evaluate TOAD on tasks:

```bash
# Quick test (10 tasks)
cargo run --release -- eval --count 10 --milestone 1

# Quality gate (50 tasks)
cargo run --release -- eval --count 50 --milestone 1 --output ./results

# With real SWE-bench dataset
cargo run --release -- eval --dataset ./swebench_lite.json --count 100 --milestone 1
```

**Output:**
- Results saved to `./results/eval_*.json`
- Console shows accuracy, cost, duration
- Individual task metrics in JSON

### 3. Compare Configurations (A/B Testing)

Compare two milestones:

```bash
# Compare M1 vs M2
cargo run --release -- compare --baseline 1 --test 2 --count 20

# With real dataset
cargo run --release -- compare \
  --dataset ./swebench_lite.json \
  --baseline 1 \
  --test 2 \
  --count 50 \
  --output ./ab_test_results
```

**Output:**
- Statistical comparison (p-value, Cohen's d)
- Decision: Adopt/Reject/Investigate/NeedMoreData
- Effect size analysis
- Cost comparison

### 4. Generate Test Data

Create synthetic test tasks:

```bash
cargo run --release -- generate-test-data --count 50 --output ./test_tasks.json
```

## M1 Quality Gates

### Quality Gate 1: No Crashes (10 tasks)

```bash
cargo run --release -- eval --count 10 --milestone 1 --output ./qg1_results
```

**Success Criteria:**
- ✅ All 10 tasks complete without panics
- ✅ Metrics collected for all tasks
- ✅ Results saved successfully

### Quality Gate 2: Baseline Measurement (50 tasks)

```bash
cargo run --release -- eval --count 50 --milestone 1 --output ./qg2_results
```

**Success Criteria:**
- ✅ Accuracy: 55-60% (target range)
- ✅ Avg cost: < $0.50 per task
- ✅ No crashes or errors
- ✅ Clear metrics for M1→M2 comparison

## Rate Limiting

TOAD automatically handles Anthropic API rate limits:
- **50 requests/minute** (Claude Sonnet 4.x)
- **30,000 input tokens/minute**
- **8,000 output tokens/minute**

The system uses **conservative limits (80% of max)** to ensure safety margin.

During evaluation, you'll see:
```
Rate limit approaching, waiting 15s for window reset
```

This is normal and ensures we stay within limits.

## Understanding Results

### Accuracy
Percentage of tasks solved successfully. For M1:
- **<50%**: Need debugging
- **50-55%**: Acceptable baseline
- **55-60%**: Target range ✅
- **>60%**: Excellent!

### Cost
Per-task cost in USD:
- **Input tokens**: $3 per million
- **Output tokens**: $15 per million
- **Cache read**: $0.30 per million
- **Cache write**: $3.75 per million

Target: < $0.50 per task average

### Duration
Wall-clock time per task (milliseconds).
- Includes API latency
- Rate limiting delays
- Tool execution time

Target: < 60 seconds per task

## Troubleshooting

### "API key error"
```bash
export ANTHROPIC_API_KEY="your-key-here"
```

### "Rate limit exceeded"
Wait 60 seconds for window reset, or use fewer concurrent evaluations.

### "Task timeout"
Increase max_steps in agent configuration (default: 25 steps).

### Slow evaluation
This is normal. 50 tasks may take 20-40 minutes depending on:
- API latency
- Task complexity
- Rate limiting

## Examples

### Quick Test
```bash
# Test that everything works
cargo run --release -- eval --count 3 --milestone 1
```

### Full M1 Baseline
```bash
# Run complete M1 evaluation
cargo run --release -- eval --count 50 --milestone 1 --output ./m1_baseline

# Review results
cat ./m1_baseline/eval_*.json | jq '.accuracy'
```

### A/B Test M1 vs M2
```bash
# Generate test set
cargo run --release -- generate-test-data --count 30 --output ./test_set.json

# Run comparison
cargo run --release -- compare \
  --dataset ./test_set.json \
  --baseline 1 \
  --test 2 \
  --count 30 \
  --output ./m1_vs_m2

# Check decision
cat ./m1_vs_m2/comparison_*.json | jq '.decision'
```

## Monitoring

During evaluation, watch for:
- Task completion messages
- Cost accumulation
- Rate limit warnings
- Error messages

Example output:
```
INFO Running task: test__example-001
INFO Task test__example-001 complete: solved=true, cost=$0.0234, tokens=1523, steps=12
```

## Next Steps

After M1 baseline:
1. **Review results**: Accuracy, cost, failure patterns
2. **Run A/B test**: Compare M1 vs M2
3. **Iterate**: Adopt features that show statistical improvement
4. **Document**: Record decisions and rationale

See `M1_IMPLEMENTATION_SUMMARY.md` for technical details.
