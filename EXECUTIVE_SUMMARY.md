# AI Coding Agents: Executive Summary

**Research Date:** November 2025

## Top-Line Findings

### Best Performance (SWE-bench Verified)
- **TRAE:** 75.2% (Jun 2025) - Multi-model ensemble (Claude 4 Sonnet, Opus, 3.7)
- **Warp:** 71-75.8% - Single-agent architecture
- **mini-SWE-agent:** 65% - Just 100 lines of Python

### Key Insight: Simplicity Wins
The trend from 2024-2025 shows simpler architectures competing with or beating complex multi-agent systems:
- mini-SWE-agent: 100 lines of code achieves 65%
- Warp: Single-agent beats multi-agent alternatives
- Agentless: No tools, just 3-phase pipeline achieves 50.8%

## Architecture Recommendations

### Start Here (Phase 1: MVP)
```
Single Agent
  ├─ 10-15 core tools (bash, read, edit, search)
  ├─ Search/replace edit format
  ├─ Prompt caching
  └─ Test-driven validation
```

**Expected Results:** 30-40% success rate, $2-5 per issue

### Optimize (Phase 2)
- AST-based context retrieval (+2.7-5.5 points)
- Repository map/knowledge graph
- Multi-patch generation + ranking
- Cost optimization via caching

**Expected Results:** 40-50% success rate, $1-2 per issue

### Advanced (Phase 3, only if needed)
- Multi-agent (Architect/Editor pattern)
- Ensemble approaches
- Specialized sub-agents

**Expected Results:** 50-65%+ success rate, varies widely

## Critical Success Factors

### 1. Model Quality (Highest Impact)
- **Best:** Claude 3.5/3.7 Sonnet, GPT-5, Gemini 2.5 Pro
- **Budget:** Qwen2.5, DeepSeek, grok-code-fast-1
- **Ensemble:** Multiple SOTA models + LLM-as-judge

### 2. Context Management
- **AST-based chunking:** Tree-sitter parsing, semantic boundaries
- **Smart retrieval:** Hybrid search (semantic + grep + graph)
- **Prompt caching:** 90% cost reduction
- **Avoid:** Dumping entire codebase, fixed-size chunks

### 3. Tool Design
- **10-15 tools max** (more causes confusion)
- **Compact & efficient** (batch operations)
- **Clear feedback** (guide next actions)
- **Examples:** edit_files (multi-file), view_file (windowed read)

### 4. Validation
- **Test-driven:** Run tests after every change
- **Linter integration:** Catch errors early
- **Spectrum-based fault localization:** Use test coverage
- **Never trust without execution:** Always validate

## Cost Analysis

| Approach | Cost/Problem | Performance | Efficiency |
|----------|--------------|-------------|------------|
| Agentless | $0.70 | 50.8% | ⭐⭐⭐⭐⭐ |
| AutoCodeRover | <$0.70 | 46.2% | ⭐⭐⭐⭐⭐ |
| Claude Sonnet 4 (cached) | $0.91 | 65% | ⭐⭐⭐⭐⭐ |
| Claude Sonnet 4 (no cache) | $5.29 | 65% | ⭐⭐ |
| SWE-Agent + GPT-4 | $0.24 | ~12% | ⭐ |

**Key Finding:** 40x cost variation for similar performance. Prompt caching = 83% cost reduction.

## What to Avoid (Common Pitfalls)

### Architecture
- ❌ Complex multi-agent as first attempt
- ❌ 50+ specialized tools
- ❌ Ignoring latency for user-facing products

### Context
- ❌ Dumping entire codebase into context window
- ❌ Fixed-size chunking (breaks semantic units)
- ❌ No prompt caching strategy

### Tools
- ❌ Inconsistent interfaces
- ❌ Unbounded output (no truncation)
- ❌ Poor error messages

### Testing
- ❌ Trusting LLM verification without execution
- ❌ Weak test suites
- ❌ Only testing at the end

## Single-Agent vs Multi-Agent

### Warp's Finding: Single-Agent Won
After testing dedicated testing agents, reasoning agents, planning agents, and best@k systems, **single-agent remained most consistent and reliable**.

**When Single-Agent:**
- User-facing products (latency critical)
- Simple to medium complexity
- Cost-conscious
- Starting out

**When Multi-Agent:**
- Architect/Editor split (cost optimization)
- Ensemble for diversity
- Complex, decomposable tasks
- Research/benchmark optimization

## Top Open-Source Agents

1. **mini-SWE-agent** (100 lines, 65%)
   - https://github.com/SWE-agent/mini-swe-agent
   - Radical simplicity, bash-only

2. **Agentless** (50.8%, $0.70)
   - https://github.com/OpenAutoCoder/Agentless
   - 3-phase pipeline, no tools

3. **AutoCodeRover** (46.2%, <$0.70)
   - https://github.com/AutoCodeRoverSG/auto-code-rover
   - AST-based, structure-aware

4. **SWE-agent** (NeurIPS 2024)
   - https://github.com/princeton-nlp/SWE-agent
   - Agent-Computer Interface focus

5. **Aider** (85% on edits)
   - https://github.com/paul-gauthier/aider
   - Architect/Editor pattern

## Key Papers

1. **Agentless** (2407.01489) - Simple pipeline beats complex agents
2. **AutoCodeRover** (2404.05427) - AST-based approach
3. **SWE-agent** (2405.15793) - Tool design principles
4. **MASAI** (2406.11638) - Modular multi-agent
5. **CodeAct** (2402.01030) - Unified action space
6. **cAST** (2506.15655) - AST-based chunking

## Implementation Checklist

### Week 1: MVP
- [ ] Single agent with 10-15 tools
- [ ] Search/replace edit format
- [ ] Basic context retrieval
- [ ] Test-driven validation

### Week 2-4: Core Features
- [ ] Prompt caching implementation
- [ ] Repository map
- [ ] AST-based chunking (Tree-sitter)
- [ ] Linter integration

### Month 2: Optimization
- [ ] Cost tracking and optimization
- [ ] Failure analysis
- [ ] Tool refinement
- [ ] Context retrieval tuning

### Month 3+: Advanced
- [ ] Consider multi-agent (if single-agent plateaus)
- [ ] Ensemble approaches
- [ ] Learning/memory systems

## Performance Targets

### Good (Baseline)
- 30-40% success rate
- <$5 per issue
- <10 minutes per task

### Great (Optimized)
- 40-50% success rate
- $1-2 per issue
- <5 minutes per task

### Excellent (SOTA)
- 50-65% success rate
- <$1 per issue
- Varies

## Bottom Line

**The 80/20 Rule:**
- 80% of performance: Model quality + Context management + Validation
- 20% from: Architecture complexity + Specialized tools + Multi-agent

**Start Simple:**
- mini-SWE-agent approach (100 lines)
- AST-based context
- Prompt caching
- Test-driven development

**Optimize Incrementally:**
- Measure everything (cost, performance, time)
- Add complexity only when justified
- Keep it as simple as possible

**The best agent architecture is the simplest one that achieves your goals.**

---

## Quick Reference Links

- **Full Report:** AI_CODING_AGENTS_RESEARCH_REPORT.md
- **SWE-bench Leaderboard:** https://www.swebench.com/
- **Claude Code Best Practices:** https://www.anthropic.com/engineering/claude-code-best-practices
- **Aider Docs:** https://aider.chat/docs/
