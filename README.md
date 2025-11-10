# TOAD - Terminal-Oriented Autonomous Developer

An AI coding terminal designed to rival Cursor, Claude Code, and GitHub Copilot CLI through evidence-based, iterative development.

## Project Status

**Current Milestone:** M0 (Infrastructure) ✅ COMPLETE & VALIDATED

- ✅ Evaluation framework with A/B testing
- ✅ Statistical significance testing (Welch's t-test)
- ✅ Feature flag architecture (13 toggleable features)
- ✅ Metrics collection (accuracy, cost, performance)
- ✅ Dataset management (SWE-bench Verified/Lite/Full)
- ✅ Experiment tracking system
- ✅ CLI interface with 4 commands
- ✅ 37 passing tests (29 unit + 8 integration)

**Next Milestone:** M1 (Simple Baseline) - Target: 55-60% accuracy

## Quick Start

```bash
# Show feature configurations
cargo run -- show-config --milestone 1

# Generate test data
cargo run -- generate-test-data --count 50

# Run evaluation
cargo run -- eval --count 10 --milestone 1

# Compare configurations (A/B test)
cargo run -- compare --baseline 1 --test 2 --count 20

# Run tests
cargo test
```

## Documentation

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete system architecture
- **[ITERATIVE_IMPLEMENTATION_PLAN.md](ITERATIVE_IMPLEMENTATION_PLAN.md)** - 5-milestone roadmap

## Research Reports

- **[AI_CODING_AGENTS_RESEARCH_REPORT.md](AI_CODING_AGENTS_RESEARCH_REPORT.md)** - SOTA agent analysis
- **[LLM_CONTEXT_OPTIMIZATION_RESEARCH.md](LLM_CONTEXT_OPTIMIZATION_RESEARCH.md)** - Context management
- **[llm_routing_research_report.md](llm_routing_research_report.md)** - LLM routing strategies

## Project Goals

1. **Beat SOTA**: Target 76-78% on SWE-bench Verified (current SOTA: 75.2%)
2. **Cost Efficient**: <$1/task with prompt caching and smart routing
3. **Evidence-Based**: Every feature validated with statistical testing
4. **Consumer Hardware**: Multi-session support without local LLMs

## Development Philosophy

- No feature without evidence (research papers, production systems, or A/B tests)
- Statistical rigor (p < 0.05) before adoption
- Iterative: Start simple (M1: 55-60%) → SOTA (M3: 70-75%) → World-class (M5: 76-78%)
- Feature flags for easy experimentation

## Technology Stack

- **Language**: Rust (for performance and safety)
- **Async**: Tokio
- **CLI**: Clap
- **Stats**: statrs (Welch's t-test, effect sizes)
- **Future TUI**: Ratatui + Crossterm

## Architecture Highlights

**Feature Flag System**: 13 toggleable innovations
- Context: AST, embeddings, graph, reranking
- Routing: Semantic, multi-model, speculative
- Intelligence: Smart tests, failure memory, planning
- Optimizations: Prompt caching, semantic caching, validation

**Evaluation Framework**:
- Task loading from SWE-bench datasets
- Metrics: accuracy, cost (USD), latency, quality
- Statistical comparison with automatic recommendations
- Results persistence for reproducibility

## Roadmap

| Milestone | Timeline | Target | Status |
|-----------|----------|--------|--------|
| M0: Infrastructure | Week 1 | Framework ready | ✅ COMPLETE |
| M1: Simple Baseline | Weeks 2-4 | 55-60% | 🔜 NEXT |
| M2: Enhanced | Weeks 5-7 | 61-66% | 📅 Planned |
| M3: Advanced | Weeks 8-10 | 70-75% | 📅 Planned |
| M4: Edge | Weeks 11-13 | 72-76% | 📅 Planned |
| M5: World-Class | Weeks 14-16 | 76-78% | 📅 Planned |

## Contributing

This is a research project. Contributions welcome via issues and PRs.

## License

See LICENSE file.
