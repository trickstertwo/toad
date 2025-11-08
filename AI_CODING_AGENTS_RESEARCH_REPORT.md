# State-of-the-Art AI Coding Agent Architectures: Comprehensive Research Report

**Date:** November 2025
**Focus:** SWE-bench Performance & Architecture Analysis (2024-2025)

---

## Executive Summary

This report analyzes state-of-the-art AI coding agent architectures with emphasis on SWE-bench performance. Key findings:

- **Top Performance:** TRAE achieved 75.2% on SWE-bench Verified (June 2025) using multi-model ensemble
- **Architecture Debate:** Single-agent systems (Warp: 71%) competitive with multi-agent approaches
- **Simplicity Wins:** mini-SWE-agent achieves 65%+ in just 100 lines of Python
- **Context is Critical:** AST-based chunking shows 2.7-5.5 point gains over naive approaches
- **Cost Efficiency:** Agentless approach: $0.70/problem vs. SWE-Agent+GPT-4: $32.5/fix

---

## 1. Current Best Architectures (2024-2025)

### 1.1 SWE-bench Leaderboard Snapshot (2025)

| Agent/System | SWE-bench Verified | SWE-bench Lite | Cost/Problem | Architecture Type |
|--------------|-------------------|----------------|--------------|-------------------|
| TRAE | 75.2% | - | - | Multi-model ensemble |
| Warp 2.0 | 71-75.8% | - | - | Single-agent |
| Devlo | 70.2% | - | - | Multi-LLM ensemble |
| Augment Code | 65.4% | - | - | Claude 3.7 + O1 |
| mini-SWE-agent | 65% | - | - | Minimalist single-agent |
| CodeStory Midwit | 62% | - | - | Brute-force + search |
| Agentless | 50.8% | 40.7% | $0.70 | Pipeline (no tools) |
| AutoCodeRover | 46.2% | 37.3% | <$0.70 | AST-based agent |
| OpenHands CodeAct | - | 26% | - | Unified action space |
| Devin | - | 13.86% (2024) | - | Multi-tool agent |

**Key Trends:**
- Claude 3.5/3.7 Sonnet is the most popular model (17+ entries)
- Multi-model ensembles dominate top spots
- Simple architectures competitive with complex ones
- Cost varies 40x between most/least efficient

---

### 1.2 Detailed Architecture Breakdowns

#### A. **Agentless** (Simple Pipeline Approach)
**Paper:** "Agentless: Demystifying LLM-based Software Engineering Agents" (July 2024)
**arXiv:** 2407.01489
**GitHub:** https://github.com/OpenAutoCoder/Agentless

**Architecture:**
```
Issue Description
    ↓
[Phase 1: Localization]
    ├─ File-level localization
    ├─ Function-level localization
    └─ Edit location identification
    ↓
[Phase 2: Repair]
    ├─ Generate multiple patches
    ├─ Rank by confidence
    └─ Filter invalid patches
    ↓
[Phase 3: Validation]
    └─ Test execution
```

**Key Innovation:** No complex agent loop, no tool use - just a straightforward 3-phase pipeline

**Performance:**
- SWE-bench Lite: 32% (GPT-4), 40.7% (Claude 3.5), 50.8% (Claude 3.5 verified)
- Cost: $0.70 per problem
- Highest performance-to-cost ratio among open-source solutions

**Why It Works:**
- Simplicity reduces compounding errors
- Multiple patch generation with ranking beats iterative refinement
- No tool-use overhead or planning failures
- Focused context at each phase

---

#### B. **AutoCodeRover** (AST-Aware Agent)
**Paper:** "AutoCodeRover: Autonomous Program Improvement" (ISSTA 2024)
**arXiv:** 2404.05427
**GitHub:** https://github.com/AutoCodeRoverSG/auto-code-rover

**Architecture:**
```
Issue + Optional Test Suite
    ↓
[Context Retrieval Stage]
    ├─ AST-based code search
    ├─ Class/method structure analysis
    ├─ Spectrum-based fault localization (if tests available)
    └─ Build context with relevant code snippets
    ↓
[Patch Generation Stage]
    ├─ Generate candidate patches
    ├─ Validate against test suite
    └─ Iterative refinement
```

**Key Innovations:**
- Operates on AST representations, not raw text
- Exploits program structure (classes/methods) for context
- Spectrum-based fault localization using test coverage
- Project structure awareness

**Performance:**
- SWE-bench Lite: 37.3%
- SWE-bench Verified: 46.2%
- Cost: <$0.70 per task
- 10-minute time limit per task

**Why It Works:**
- AST-based navigation finds relevant code faster
- Structural understanding > keyword search
- Test-driven fault localization when available
- Efficient context gathering

---

#### C. **SWE-agent** (Agent-Computer Interface Focus)
**Paper:** "SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering" (NeurIPS 2024)
**arXiv:** 2405.15793
**GitHub:** https://github.com/princeton-nlp/SWE-agent

**Architecture:**
```
GitHub Issue
    ↓
[Agent-Computer Interface (ACI)]
    ├─ Custom LM-centric commands
    ├─ Optimized feedback formats
    └─ Repository navigation tools
    ↓
[Agent Loop]
    ├─ Browse repository
    ├─ View/edit code files
    ├─ Execute tests
    └─ Iterative refinement
```

**Key Innovations:**
- **Agent-Computer Interface (ACI):** Custom command/feedback design for LLMs
- Consolidated actions (navigation + editing in minimal commands)
- LM-optimized output formats
- Compact, efficient action space

**Performance:**
- SWE-bench: 12.5% (2024 baseline)
- HumanEvalFix: 87.7%
- Open-source SOTA on SWE-bench Lite (2025)

**Tool Design Principles:**
- Actions should be compact and efficient
- Important operations consolidated into few actions
- Clear, consistent feedback formats
- Optimized for LLM token usage

---

#### D. **OpenHands/CodeAct** (Unified Action Space)
**Paper:** "OpenHands: An Open Platform for AI Software Developers as Generalist Agents" (ICLR 2025)
**arXiv:** 2407.16741
**GitHub:** https://github.com/All-Hands-AI/OpenHands

**Architecture:**
```
[CodeAct Framework]
All actions expressed as executable Python code
    ↓
[Agent Runtime]
    ├─ Shell access
    ├─ Code editor
    ├─ Browser (for web search)
    └─ Sandboxed execution environment
    ↓
[Multi-Agent Delegation]
    ├─ Generalist agent (CodeActAgent)
    ├─ Web browsing specialist
    └─ Code editing specialist
```

**Key Innovation:** **CodeAct Framework** - Consolidates all LLM actions into unified code action space

**Performance:**
- SWE-bench Lite: 26% (unassisted)
- HumanEvalFix: 79.3%
- 17% improvement over previous SOTA

**Why It Works:**
- Executable Python code more natural for code-pretrained LLMs
- No JSON/text action format constraints
- Dynamic revision through multi-turn interaction
- Leverages existing LLM code training

**CodeAct Benefits:**
- Up to 20% higher success rate vs JSON/Text formats
- Natural control flow and data structures
- Extensive software packages available
- Self-debugging capability

---

#### E. **Warp AI** (Production Single-Agent)
**Blog:** https://www.warp.dev/blog/swe-bench-verified
**Type:** Commercial product

**Architecture:**
```
[Single Agent with Specialized Tools]
    ├─ edit_files (bulk multi-file editing)
    ├─ Code understanding (grep/find/cat)
    ├─ Long-running command support (REPLs, vim)
    └─ TODO list (lightweight planning)
    ↓
[Recovery Mechanisms]
    ├─ Tool failure feedback to LLM
    ├─ Alternate model fallback
    └─ Tool use restrictions
```

**Performance:**
- SWE-bench Verified: 71% → 75.8% (with GPT-5)
- Single-agent, single-attempt architecture
- Production-optimized for low latency

**Key Design Decisions:**
- **Rejected multi-agent:** After testing dedicated testing agents, reasoning agents, planning agents, best@k systems - single agent proved most reliable
- **Single-attempt architecture:** Critical for user-facing systems (latency)
- **Smart tools over complex orchestration:** edit_files enables bulk edits efficiently

**Why Single-Agent Won:**
- Most consistent and reliable
- Lower latency (critical for UX)
- Easier to debug and improve
- Quality improvements compound better

---

#### F. **MASAI** (Modular Multi-Agent)
**Paper:** "MASAI: Modular Architecture for Software-engineering AI Agents" (June 2024)
**arXiv:** 2406.11638
**Website:** https://masai-dev-agent.github.io/

**Architecture:**
```
[5 Specialized Sub-Agents]
    ├─ Test Template Generator
    ├─ Issue Reproducer
    ├─ Edit Localizer
    ├─ Fixer
    └─ Ranker
    ↓
Each with tuned strategies and objectives
```

**Performance:**
- SWE-bench Lite: 28.33% (highest at time of publication)
- Cost: <$2 per issue average
- June 2024 SOTA

**Key Advantages:**
1. Different problem-solving strategies per sub-agent
2. Information gathering from scattered sources
3. Avoid unnecessarily long trajectories
4. Reduced cost and extraneous context

**When Multi-Agent Works:**
- Complex, decomposable tasks
- Specialized expertise needed
- Parallel exploration beneficial
- Clear sub-task boundaries

---

#### G. **mini-SWE-agent** (Radical Minimalism)
**GitHub:** https://github.com/SWE-agent/mini-swe-agent
**Philosophy:** 100-line AI agent

**Architecture:**
```
[Minimalist Design - 100 lines of Python]
    ├─ Only tool: bash (no special interfaces)
    ├─ Linear history (trajectory = messages)
    └─ LM utilizes shell to full potential
```

**Performance:**
- SWE-bench Verified: 65%+
- Radically simple

**Key Philosophy:**
- "As LMs have become more capable, much of [complex scaffolding] is not needed"
- Hackable tool, not black box
- Full power through bash, not specialized tools
- Linear conversation = simpler debugging

**Why It Works (2025):**
- Modern LLMs are capable enough
- Bash provides complete toolkit
- Simplicity reduces failure modes
- Easier to understand and modify

---

#### H. **Aider** (Architect/Editor Pattern)
**Website:** https://aider.chat
**GitHub:** Open-source
**Type:** Terminal-based coding assistant

**Architecture:**
```
[Architect Mode] (Optional)
    ↓
High-level solution design (o1-preview)
    ↓
[Editor Mode]
    ↓
Specific code edits (DeepSeek/o1-mini)
```

**Key Innovation:** **Separation of "code reasoning" and "code editing"**

**Performance:**
- 85% on code editing benchmarks (o1-preview as Architect + DeepSeek as Editor)
- State-of-the-art code editing results

**Context Management:**
- Repository map of entire codebase
- Prompt cache warming (--cache-keepalive-pings)
- Multi-file awareness
- Git-based code tracking

**Why Architect/Editor Works:**
- Reasoning separated from implementation
- Cheaper editor model for routine edits
- Architect handles complex logic
- Cost optimization without sacrificing quality

---

#### I. **Devin** (Early Commercial Pioneer)
**Company:** Cognition AI
**Announcement:** March 2024
**GitHub:** https://github.com/CognitionAI/devin-swebench-results

**Architecture:**
```
[Sandboxed Development Environment]
    ├─ Shell access
    ├─ Code editor
    └─ Browser
    ↓
[Long-Term Reasoning & Planning]
    ├─ Multi-step plan execution (1000+ decisions)
    ├─ Context recall
    ├─ Learn over time
    └─ Self-correction
```

**Performance:**
- SWE-bench: 13.86% (March 2024)
- Previous SOTA: 1.96% unassisted
- Even "assisted" best was 4.80%
- Evaluated on 25% subset (570/2,294 issues)

**Key Capabilities:**
- Active collaboration with user
- Multi-step complex engineering tasks
- 72% of passing tests took >10 minutes
- Iteration critical to success

**Historical Significance:**
- First widely-publicized "AI software engineer"
- Demonstrated viability of autonomous coding
- Set baseline for commercial agents
- Showed importance of iteration time

---

#### J. **Cursor** (IDE-Integrated Agent)
**Website:** https://cursor.com
**Type:** AI-native IDE (VS Code fork)

**Architecture:**
```
[Context System]
    ├─ @ Symbol References
    │   ├─ @Files (specific files)
    │   ├─ @Folders (entire directories)
    │   ├─ @Code / @Symbols (functions, classes)
    │   └─ @Web (live search)
    ├─ Context Gathering Tools
    │   ├─ codebase_search
    │   ├─ read_file
    │   ├─ grep_search
    │   ├─ file_search
    │   └─ web_search
    └─ Configuration
        ├─ Notepads (reusable context bundles)
        └─ Project rules
    ↓
[Dual Context Types]
    ├─ Intent Context (what user wants)
    └─ State Context (current environment)
```

**Key Technical Features:**

1. **Context Windows:** 200k+ tokens (2024) → 1M tokens (2025)
2. **Tab Completions:** Low-latency sync engine (<1 second)
3. **Privacy:** Merkle trees to avoid storing source code
4. **Prompt Engineering:** "Be THOROUGH when gathering information. TRACE every symbol back to definitions."

**Why It Works:**
- Surgical context via @ symbols
- Sophisticated context gathering strategy
- IDE integration = immediate feedback
- Two-type context (intent + state)

**Performance Note:**
- Not on SWE-bench leaderboard (IDE, not benchmark agent)
- Wide commercial adoption
- Focus on developer productivity over benchmark scores

---

#### K. **Claude Code** (Anthropic's Official Agent)
**Documentation:** https://docs.claude.com/en/docs/claude-code/overview
**Blog:** https://www.anthropic.com/engineering/claude-code-best-practices

**Architecture:**
```
[Low-Level, Unopinionated Design]
    ├─ Raw model access
    ├─ Minimal scaffolding
    └─ Flexible tool system
    ↓
[Core Tools]
    ├─ File operations (read, write, edit)
    ├─ Bash/terminal
    ├─ Search (grep, glob)
    └─ Git integration
    ↓
[Features]
    ├─ Checkpoints (save/rollback)
    ├─ CLAUDE.md (auto-loaded config)
    ├─ Subagents (for verification)
    └─ Memory tool (long-running agents)
```

**Built With:** TypeScript, React, Ink, Yoga, Bun
**Philosophy:** "90% of code written by Claude Code itself"

**Design Patterns:**
- **Multi-agent option:** Opus 4 lead + Sonnet 4 subagents (90.2% better on internal evals)
- **Workflow orchestration:** LLMs + tools through predefined paths
- **Artifact systems:** Specialized agents create persistent outputs

**Best Practices:**
1. Gather context first (read files) before coding
2. Use subagents for verification early
3. CLAUDE.md for project-specific guidance
4. Test-driven development with explicit TDD mention

**Context Optimization:**
- Auto-compact at 95% context window
- Summarizes full trajectory
- Prompt caching (90% cost reduction)
- Cache-aware rate limits

---

### 1.3 Architecture Comparison Matrix

| Feature | Agentless | AutoCodeRover | SWE-agent | OpenHands | Warp | MASAI | mini-SWE | Aider |
|---------|-----------|---------------|-----------|-----------|------|-------|----------|-------|
| **Type** | Pipeline | Agent | Agent | Agent | Agent | Multi-Agent | Agent | Architect/Editor |
| **Tools** | None | AST search | Custom ACI | Python code | Specialized | Per sub-agent | Bash only | Standard |
| **Iterations** | Fixed 3-phase | Yes | Yes | Yes | Single-attempt | Yes | Yes | Yes |
| **Context Method** | Localization | AST-based | ACI | Code actions | Smart retrieval | Distributed | Linear | Repo map |
| **Cost** | Low ($0.70) | Low (<$0.70) | Medium | Medium | N/A | Low (<$2) | Low | Varies |
| **Performance** | 50.8% | 46.2% | 12.5% (2024) | 26% | 71-75.8% | 28.33% | 65% | 85% (edits) |
| **Complexity** | Low | Medium | Medium | High | Medium | High | Very Low | Low |
| **Open Source** | Yes | Yes | Yes | Yes | No | Yes | Yes | Yes |

---

## 2. Context Management Strategies

### 2.1 The Core Challenge

**Problem:** Modern codebases can be millions of tokens, but:
- Claude Sonnet 4: 1M token context (but expensive, slow)
- Claude Opus 4: 200k tokens
- GPT-4: 128k tokens
- Cost scales linearly (or worse) with context size
- Retrieval accuracy drops in large contexts ("Lost in the Middle" - 20-25% variance)

**Key Insight:** Smart retrieval > large context windows

---

### 2.2 AST-Based Context Selection

**Research:** "cAST: Enhancing Code Retrieval-Augmented Generation with Structural Chunking via Abstract Syntax Tree"
**arXiv:** 2506.15655

**Approach:**
```
Source Code
    ↓
[Parse to AST]
    ↓
Hierarchical tree with typed nodes (classes, functions, etc.)
    ↓
[Split-then-Merge Algorithm]
    ↓
Chunks aligned with syntax boundaries
    ↓
Self-contained, semantically meaningful chunks
```

**Performance Gains:**
- StarCoder2-7B: +5.5 points on RepoEval
- +4.3 points on CrossCodeEval
- **+2.7 points on SWE-bench**

**Why It Works:**
- Chunks preserve semantic meaning
- Don't split functions/classes mid-definition
- Each chunk is syntactically valid
- Better embeddings for retrieval

**Implementation:**
- Parse with Tree-sitter (supports 81 languages)
- Traverse AST respecting boundaries
- Chunk at function/class/module level
- Maintain structural context

---

### 2.3 Repository Map Techniques

**Aider's Approach:**
- Generate map of entire codebase
- Track file structure, class/function definitions
- Update incrementally as code changes
- Use for quick navigation and context

**Refact.ai Approach:**
- Look for identifiers around cursor
- Search definitions across entire project
- Real-time search (10ms for 10k files)
- Provide definitions + similar code examples

**Benefits:**
- Fast lookup without re-reading
- Understand code relationships
- Navigate large codebases efficiently
- Minimal token cost

---

### 2.4 RAG for Codebases

**Two-Stage Retrieval:**
```
Query
    ↓
[Stage 1: Vector Search]
Retrieve top-k candidates from embedding store
    ↓
[Stage 2: LLM Ranking]
Re-rank by relevance and context
    ↓
Top-n most relevant code snippets
```

**Advanced Techniques:**

1. **Knowledge Graphs:**
   - Analyze codebase with Tree-sitter
   - Build graph of relationships (imports, calls, inheritance)
   - Enable graph-based queries
   - Example: "What calls this function?"

2. **Hybrid Search:**
   - Combine keyword search (grep)
   - Embedding search (semantic)
   - Graph traversal (relationships)
   - Re-rank with LLM

3. **Chunking Strategies:**
   - AST-based (as above)
   - Sliding window with overlap
   - Function/class boundaries
   - Documentation + code together

**Key Insight:** "Embedding search becomes unreliable as codebase grows"
→ Must combine techniques

---

### 2.5 Prompt Caching (Context Reuse)

**Anthropic Prompt Caching (December 2024):**
- Cache frequently used context
- **90% cost reduction** on cached tokens
- **85% latency reduction** for long prompts
- Cache-aware rate limits (cached reads don't count toward ITPM)

**Best Practices:**
1. Cache stable content (system instructions, tool definitions)
2. Place cached content at prompt beginning
3. Use up to 4 cache breakpoints:
   - Tools cache
   - Instructions cache
   - RAG context cache
   - Conversation history cache

**Aider Implementation:**
- `--cache-keepalive-pings N`: Ping every 5 min to keep cache warm
- Prevents cache expiration during sessions

**Cost Impact Example (Claude Sonnet 4):**
- Without caching: $5.29 per problem
- With caching: $0.91 per problem
- **83% cost reduction**

---

### 2.6 Semantic Chunking

**Traditional Chunking Problems:**
- Fixed-size chunks split semantic units
- Miss context boundaries
- Poor embedding quality

**Agentic Chunking (IBM):**
- LLM decides chunk boundaries
- Considers semantic coherence
- Adaptive sizing based on content
- Better for RAG retrieval

**Code-Specific Chunking:**
- Chunk at function/class level (AST)
- Include docstrings with code
- Preserve imports/dependencies
- Maintain syntax validity

---

### 2.7 Context Window Utilization Strategies

**Research Findings:**

**200K vs 1M Token Comparison:**
- Augment (200K, smart retrieval): 83% accuracy, 4.1s latency
- GitHub Copilot (1M): 67% accuracy, 12.8s latency
- Cursor Composer (1M): 64% accuracy, 15.2s latency

**Key Lessons:**
1. **Bigger ≠ Better:** Smart retrieval in 200K beats dump-all in 1M
2. **Latency Matters:** 5x memory requirements, slower responses
3. **Lost in the Middle:** 20-25% accuracy variance based on position
4. **Cost:** 200K→1M = ~5x memory, higher API costs

**Optimal Strategy:**
- Use retrieval for initial context (<200K tokens)
- Reserve large windows for deep multi-step reasoning
- Compress/summarize conversation history
- Cache stable context

**Claude Code Auto-Compact:**
- Triggers at 95% context window
- Summarizes full trajectory
- Preserves key information
- Frees space for new context

---

### 2.8 Avoiding Re-Reading Context

**Strategies:**

1. **Incremental Updates:**
   - Track file modifications
   - Only re-read changed files
   - Use git diff for changes

2. **State Management:**
   - Maintain in-memory file content
   - Update selectively
   - Cache parsed ASTs

3. **Smart Invalidation:**
   - Invalidate only affected chunks
   - Re-parse modified functions
   - Keep unchanged context cached

4. **Tools Over Re-reading:**
   - grep_search for specific content
   - file_search for file location
   - symbol_search for definitions
   - Don't read entire file for one function

---

## 3. Tool Design Patterns

### 3.1 Core Principles from SWE-agent Research

**From "Agent-Computer Interfaces Enable Automated Software Engineering":**

1. **Compactness:** Consolidate related operations
   - Bad: separate navigate, open, scroll, read commands
   - Good: single `view_file` with line range

2. **Efficiency:** Minimize token overhead
   - Return only relevant output
   - Truncate long outputs intelligently
   - Provide summaries over full dumps

3. **Clear Feedback:** LM-optimized formats
   - Structured, consistent responses
   - Error messages guide next action
   - Success/failure clearly indicated

4. **Minimal Action Space:** Fewer, powerful tools
   - 10-15 well-designed tools > 50 specialized ones
   - Each tool should be distinct in purpose
   - Avoid redundancy

---

### 3.2 Edit Format Comparison

**From Aider Research:**

| Format | Description | Accuracy | Models | Pros | Cons |
|--------|-------------|----------|--------|------|------|
| **Search/Replace** | `<<<SEARCH`<br/>`old code`<br/>`=======`<br/>`new code`<br/>`>>>REPLACE` | 60-85% | GPT-4, Opus | Intuitive, precise | Requires exact match |
| **Unified Diff** | Modified unified diff format | 70-80% | GPT-4 Turbo | Efficient, partial files | Complex format |
| **Whole File** | Return complete updated file | ~60% | Weak models | Simple | Slow, costly, lazy |
| **Semantic (Morph)** | Code understanding-based | **98%** | Modern LLMs | Most accurate | Proprietary |

**Key Issues:**
- Pattern matching fails with whitespace differences
- "Cannot find matching context" errors common
- Hallucinated line numbers in diffs
- Models generate valid changes but wrong format

**Best Practices:**
1. Search/replace for GPT-4, Claude Opus
2. Unified diff for GPT-4 Turbo (reduces laziness)
3. Whole file only for weak models
4. Multiple search/replace blocks for multi-location edits

**Warp's Solution:** `edit_files` tool
- Enables bulk multi-file editing
- Single tool call for related changes
- Reduces back-and-forth
- Higher success rate

---

### 3.3 Essential Tool Categories

**1. Code Understanding Tools:**
```
- read_file(path, [lines])      # Read specific file/range
- grep_search(pattern, [path])  # Search by regex
- file_search(name_pattern)     # Find files by name
- symbol_search(symbol)         # Find definitions/usages
- list_files([path])            # Directory structure
```

**2. Code Modification Tools:**
```
- edit_file(path, edits)        # Apply edits
- create_file(path, content)    # New file
- delete_file(path)             # Remove file
- move_file(src, dest)          # Rename/move
```

**3. Execution Tools:**
```
- bash(command)                 # Execute shell command
- run_tests([test_path])        # Execute tests
- run_linter([path])            # Check code quality
```

**4. Version Control:**
```
- git_diff([path])              # Show changes
- git_status()                  # Repository state
- git_log([n])                  # Commit history
- git_show(commit)              # Specific commit
```

**5. Context Tools:**
```
- codebase_search(query)        # Semantic search
- get_definition(symbol)        # Jump to definition
- find_references(symbol)       # Find all usages
- get_call_hierarchy(function)  # Call graph
```

---

### 3.4 Cross-Platform Considerations

**Challenges:**
- File path separators (/ vs \\)
- Line endings (LF vs CRLF)
- Shell differences (bash vs cmd vs PowerShell)
- Permission models

**Solutions:**

1. **Normalize Paths:**
   ```python
   import os.path
   path = os.path.normpath(path)
   ```

2. **Abstract Shell Commands:**
   ```python
   # Don't expose raw bash
   # Provide cross-platform abstractions
   run_command(cmd, platform_agnostic=True)
   ```

3. **Handle Line Endings:**
   ```python
   # Normalize on read/write
   content.replace('\r\n', '\n')
   ```

4. **Sandboxing:**
   - Docker containers (OpenHands approach)
   - Virtual environments
   - Restricted file access
   - Network isolation for security

---

### 3.5 Tool Call Efficiency

**Research Finding:** "More tools confuse AI agents and increase chance of using tools incorrectly"

**Optimization Strategies:**

1. **Batching:**
   - Single tool call for multiple operations
   - Warp's `edit_files`: multiple files at once
   - Reduce round-trips

2. **Smart Defaults:**
   - Reasonable limits on output (100 lines)
   - Auto-truncation with "...more" indicator
   - Context-aware tool behavior

3. **Tool Composition:**
   ```
   # Bad: Separate calls
   find_files("*.py") → [file1, file2, ...]
   for file in files:
       read_file(file)

   # Good: Composed operation
   grep_search("pattern", "*.py")  # Finds + reads
   ```

4. **Feedback Loops:**
   - Tool failures reported back to LLM
   - LLM can retry with corrections
   - Error messages suggest fixes

---

### 3.6 Git Integration Patterns

**Essential Git Operations:**

1. **Pre-Change:**
   - `git status` - See current state
   - `git diff` - Review uncommitted changes
   - `git log` - Understand history

2. **Making Changes:**
   - Work in feature branch
   - Commit logical units
   - Clear commit messages

3. **Validation:**
   - Run tests before commit
   - Linter checks
   - Build verification

4. **Safety:**
   - Never force push to main
   - Checkpoint before major changes
   - Easy rollback mechanism

**Warp's Checkpoint System:**
- Save progress at milestones
- Instant rollback to any checkpoint
- Doesn't pollute git history
- User-controlled save points

---

### 3.7 Long-Running Commands

**Challenge:** REPLs, vim, interactive prompts, pagers

**Warp's Solution:**
- Support for alt-screen commands
- Detect and handle:
  - Python REPL
  - Node.js REPL
  - `git log` (pager)
  - `vim`/`nano`
  - Interactive installers

**Best Practice:**
- Detect interactive mode
- Provide appropriate termination
- Timeout protection
- Stream output for long operations

---

### 3.8 TODO List / Planning Tools

**Warp's TODO System:**
- Lightweight planning mechanism
- Agent records tasks
- Updates as work progresses
- Helps with step-by-step execution

**Benefits:**
- Prevents agent drift
- Maintains focus
- User visibility into progress
- Self-organizing work

**Implementation:**
```
todo_create(description)
todo_update(id, status)
todo_complete(id)
todo_list()
```

**When to Use:**
- Complex multi-step tasks
- Long-running issues
- Multiple sub-goals
- Helps prevent "getting lost"

---

## 4. Multi-Agent vs Single-Agent

### 4.1 The Warp Finding: Single-Agent Often Wins

**Warp's Experimentation:**
- Tested dedicated testing agents
- Reasoning agents
- Planning and context gathering agents
- Best@k systems (generate multiple diffs, pick best)

**Result:** "Most consistent and reliable architecture remained our single primary agent"

**Why Single-Agent Won:**
1. **Consistency:** Fewer failure modes
2. **Latency:** Critical for user-facing products
3. **Simplicity:** Easier to debug and improve
4. **Quality Compounds:** Improvements to single agent benefit all tasks

**Key Insight:** "Single-attempt architectures are competitive at coding tasks"

---

### 4.2 When Multi-Agent Makes Sense

**MASAI Success (28.33% on SWE-bench Lite):**

**5 Sub-Agents:**
1. **Test Template Generator:** Create test cases
2. **Issue Reproducer:** Reproduce the bug
3. **Edit Localizer:** Find where to edit
4. **Fixer:** Generate the fix
5. **Ranker:** Select best patch

**Advantages:**
1. **Specialized Strategies:** Each agent optimized for sub-task
2. **Information Gathering:** From different sources
3. **Avoid Long Trajectories:** No single agent wandering
4. **Cost Efficiency:** Right-sized model per task

---

### 4.3 Multi-Agent Patterns

**1. Architect/Editor (Aider):**
```
[Architect] (Expensive, powerful)
    ↓ (High-level solution)
[Editor] (Cheaper, focused)
    ↓ (Detailed implementation)
Code Changes
```
- **When:** Clear reasoning vs execution split
- **Benefit:** Cost optimization + quality

**2. Ensemble/Voting:**
```
[Agent 1] → Patch A
[Agent 2] → Patch B
[Agent 3] → Patch C
    ↓
[Selector/Judge]
    ↓
Best Patch
```
- **When:** Multiple valid approaches
- **Benefit:** Diversity + robustness
- **Example:** TRAE (75.2%), Devlo (70.2%)

**3. Parallel Specialists:**
```
Task → [Decomposer]
    ├─ Subtask A → [Specialist A]
    ├─ Subtask B → [Specialist B]
    └─ Subtask C → [Specialist C]
    ↓
[Combiner] → Final Result
```
- **When:** Independent subtasks
- **Benefit:** Parallelization, expertise

**4. Sequential Pipeline:**
```
[Localizer] → Relevant code
    ↓
[Reproducer] → Test case
    ↓
[Fixer] → Patch
    ↓
[Validator] → Verified fix
```
- **When:** Clear stage dependencies
- **Benefit:** Focused context per stage
- **Example:** MASAI, Agentless

---

### 4.4 Communication Patterns

**Shared State:**
- Common workspace/repository
- Shared file system
- Database/vector store

**Message Passing:**
- Explicit handoff between agents
- Structured messages with schema
- Acknowledgment/confirmation

**Artifact Systems (Claude Code pattern):**
- Subagents create persistent artifacts
- Pass lightweight references, not full content
- Coordinator manages artifact lifecycle

**Best Practice:** Clear interfaces between agents
```python
class AgentInterface:
    def receive_task(self, task: Task) -> Artifact
    def can_handle(self, task: Task) -> bool
    def estimate_cost(self, task: Task) -> float
```

---

### 4.5 Task Decomposition Strategies

**1. Functional Decomposition (MASAI):**
- By operation type (test, localize, fix, validate)
- Each function = one agent
- Natural boundaries

**2. Temporal Decomposition:**
- Phases of workflow (plan → execute → verify)
- Sequential stages
- Clear handoffs

**3. Complexity-Based:**
- Simple tasks → fast, cheap agent
- Complex tasks → powerful, expensive agent
- Route based on difficulty

**4. Domain-Based:**
- Frontend specialist
- Backend specialist
- Database specialist
- Test specialist

---

### 4.6 Cost Considerations

**Multi-Agent Token Overhead:**
- Anthropic notes: 15x more tokens vs standard chat
- Reason: Coordination, message passing, redundant context

**Cost/Performance Trade-offs:**

| Approach | Cost | Performance | Latency | Complexity |
|----------|------|-------------|---------|------------|
| Single-Agent | Low | Good | Low | Low |
| Architect/Editor | Medium | High | Medium | Medium |
| Multi-Specialist | High | Variable | High | High |
| Ensemble/Voting | Very High | Highest | Very High | Medium |

**Best Practice:**
- Start with single-agent
- Add multi-agent only if:
  - Clear benefit demonstrated
  - Cost justified by quality
  - Latency acceptable
  - Task naturally decomposes

---

### 4.7 Failure Modes

**Multi-Agent Specific Failures:**
1. **Coordination Failures:** Agents work at cross-purposes
2. **Communication Errors:** Misunderstood handoffs
3. **Infinite Loops:** Agents passing tasks in circles
4. **Context Loss:** Information lost between agents
5. **Compounding Errors:** Early agent error cascades

**Single-Agent Failures:**
1. **Cognitive Overload:** Too many tools/context
2. **Getting Lost:** Long trajectories without progress
3. **Tool Misuse:** Incorrect tool for task
4. **Context Overflow:** Runs out of tokens

**Mitigation:**
- Clear success/failure signals
- Timeouts and max iterations
- Checkpoints for rollback
- Progress monitoring
- Human-in-the-loop for critical decisions

---

## 5. SWE-bench Performance Analysis

### 5.1 Performance Progression Over Time

| Date | Agent/Model | Score | Benchmark |
|------|-------------|-------|-----------|
| Mar 2024 | Devin | 13.86% | SWE-bench |
| Apr 2024 | AutoCodeRover | 22% | SWE-bench Lite |
| Jun 2024 | MASAI | 28.33% | SWE-bench Lite |
| Jul 2024 | Agentless + Claude 3.5 | 40.7% | SWE-bench Lite |
| Oct 2024 | Agentless + Claude 3.5 | 50.8% | SWE-bench Verified |
| Dec 2024 | mini-SWE-agent | 65% | SWE-bench Verified |
| Jan 2025 | Warp | 71% | SWE-bench Verified |
| Jun 2025 | TRAE | 75.2% | SWE-bench Verified |
| Aug 2025 | Warp + GPT-5 | 75.8% | SWE-bench Verified |

**Key Trend:** 13.86% → 75.8% in ~18 months (5.5x improvement)

---

### 5.2 Key Differentiators

**What Separates Top Performers:**

1. **Model Quality:**
   - Claude 3.5/3.7 Sonnet dominates (17+ entries)
   - GPT-5, Gemini 2.5 Pro in top systems
   - Ensemble of multiple SOTA models (TRAE, Devlo)

2. **Simplicity:**
   - mini-SWE-agent: 65% in 100 lines
   - Agentless: 50.8% with no tools
   - Simple often beats complex

3. **Smart Context Management:**
   - AST-based retrieval (+2.7-5.5 points)
   - Repository maps
   - Efficient token usage

4. **Test-Driven Approaches:**
   - Spectrum-based fault localization
   - Generate tests, reproduce bugs
   - Validate against test suites

5. **Iteration & Refinement:**
   - Multiple patch generation + ranking
   - Self-correction loops
   - Learn from failures

6. **Cost Efficiency:**
   - Prompt caching (90% reduction)
   - Right-sized models per task
   - Avoid unnecessary tool calls

---

### 5.3 Cost vs Performance Analysis

**Cost Per Problem:**

| Agent | Performance | Cost/Problem | Cost/Fix | Efficiency Score |
|-------|-------------|--------------|----------|------------------|
| Agentless | 50.8% | $0.70 | $1.38 | ★★★★★ |
| AutoCodeRover | 46.2% | <$0.70 | $1.52 | ★★★★★ |
| MASAI | 28.33% | <$2.00 | $7.06 | ★★★ |
| grok-code-fast-1 | 29% | $0.03 | $0.10 | ★★★★★ |
| Claude Sonnet 4 (cached) | 65% | $0.91 | $1.40 | ★★★★★ |
| Claude Sonnet 4 (no cache) | 65% | $5.29 | $8.14 | ★★ |
| SWE-Agent + GPT-4 | ~12% | $0.24 | $32.50 | ★ |

**Key Insights:**
- **40x cost variation** for similar performance
- Caching = **83% cost reduction**
- Agentless is **23x cheaper** per fix than SWE-Agent+GPT-4
- Budget models (grok-code-fast-1) competitive on cost

---

### 5.4 Benchmark Variants

**SWE-bench (Original):**
- 2,294 real GitHub issues
- 12 Python repositories
- Some issues may not be solvable

**SWE-bench Lite:**
- 300 curated issues
- Subset of original
- Faster evaluation

**SWE-bench Verified:**
- 500 verified solvable issues
- Real engineers confirmed
- Most reliable benchmark
- **Current standard for comparison**

**SWE-bench Pro:**
- Longer-horizon tasks
- More complex issues
- Recent addition

**SWE-bench Live:**
- Continuously updated
- Prevents overfitting
- New issues added regularly

**Note:** Not directly comparable across variants!

---

### 5.5 Common Failure Modes (Research-Based)

**Taxonomy from "An Empirical Study on Failures in Automated Issue Solving":**

**3 Primary Phases:**
1. Understanding & Localization
2. Solution Generation
3. Validation & Integration

**9 Main Categories:**
1. Diagnostic errors (wrong file/function identified)
2. Reasoning failures (flawed logic)
3. Syntax errors
4. Tool use problems
5. Context management (overflow, infinite loops)
6. Semantic/algorithmic incorrectness
7. Test suite weaknesses
8. Integration failures
9. Premature termination

**25 Fine-Grained Subcategories**

**Key Findings:**

**Model-Specific Patterns:**
- **Large/sophisticated models:** Semantic and algorithmic errors (correct syntax, wrong logic)
- **Smaller/open-weight models:** Operational challenges (syntax, tools, context)

**Architecture-Specific:**
- **Pipeline-based (Agentless):** Early-stage diagnostic errors
- **Agent-based (SWE-agent, etc.):** Unproductive iterative loops, cognitive deadlocks

**Python Execution Errors:**
- Primary challenge
- Increased reasoning overhead
- Lower success rates
- Need better error handling

**Test Suite Issues:**
- Weak test suites allow wrong patches
- Inflates performance scores
- Need stronger validation

---

### 5.6 What Makes Agents Succeed

**Success Factors (Ranked by Impact):**

1. **Model Quality (Highest Impact):**
   - Claude 3.5/3.7, GPT-5, Gemini 2.5 dominate
   - Ensemble > single model
   - Code-trained models excel

2. **Context Management:**
   - AST-based retrieval: +2.7-5.5 points
   - Smart chunking over large windows
   - Caching: -83% cost, same performance

3. **Test-Driven Validation:**
   - Generate tests early
   - Spectrum-based fault localization
   - Validate patches thoroughly

4. **Simplicity & Reliability:**
   - mini-SWE (100 lines) beats complex systems
   - Single-agent competitive with multi-agent
   - Fewer tools = fewer mistakes

5. **Iteration Strategy:**
   - Multiple patch generation + ranking
   - Self-correction with feedback
   - Know when to stop (avoid loops)

6. **Tool Design:**
   - Compact, efficient tools
   - Clear error messages
   - Batch operations
   - Smart defaults

7. **Edit Format:**
   - Search/replace for GPT-4, Opus
   - Semantic understanding (Morph: 98%)
   - Avoid whole-file for modern models

8. **Cost Optimization:**
   - Prompt caching
   - Right-sized models
   - Efficient tool use
   - Avoid redundant reads

---

## 6. Specific Technical Recommendations

### 6.1 For Building a New Coding Agent

**Phase 1: MVP (Minimum Viable Product)**

1. **Start Simple:**
   - Single agent
   - 10-15 core tools
   - Bash + file operations
   - Follow mini-SWE-agent philosophy

2. **Essential Tools:**
   ```
   - read_file(path, lines)
   - edit_file(path, search, replace)
   - bash(command)
   - grep_search(pattern)
   - list_files(path)
   ```

3. **Pick One Edit Format:**
   - Search/replace for Claude, GPT-4
   - Make it robust with fuzzy matching

4. **Context Strategy:**
   - Start with simple repo map
   - Add AST-based chunking
   - Implement prompt caching early

5. **Validation:**
   - Run tests after every change
   - Linter integration
   - Git status checks

**Phase 2: Optimization**

1. **Measure & Profile:**
   - Token usage per task
   - Success/failure rates
   - Cost per issue
   - Time to completion

2. **Context Improvements:**
   - AST-based retrieval
   - Knowledge graph for relationships
   - Hybrid search (keyword + semantic + graph)

3. **Tool Refinement:**
   - Batch related operations
   - Add specialized tools based on failures
   - Remove redundant tools

4. **Cost Optimization:**
   - Prompt caching for stable context
   - Cache-aware rate limiting
   - Right-size model for subtasks

**Phase 3: Advanced Features**

1. **Multi-Agent (if justified):**
   - Only if single-agent plateaus
   - Start with Architect/Editor pattern
   - Clear metrics showing benefit

2. **Specialized Tools:**
   - Code understanding (call graphs)
   - Semantic search
   - Refactoring tools

3. **Learning & Memory:**
   - Track successful patterns
   - Learn repo-specific conventions
   - User preferences

---

### 6.2 Model Selection Guide

**For Different Tasks:**

| Task | Recommended Model | Rationale |
|------|-------------------|-----------|
| High-level design | Claude Opus 4, GPT-5, o1 | Best reasoning |
| Code editing | Claude Sonnet 4, GPT-4.5 | Fast, accurate, cost-effective |
| Simple edits | DeepSeek, Qwen2.5 | Very cheap, good enough |
| Verification | Claude Sonnet 4 | Good at catching errors |
| Test generation | Claude Sonnet 4 | Understands intent |
| Documentation | Claude Sonnet 4 | Natural language strength |

**Ensemble Strategy (TRAE-style):**
1. Generate patches with 3+ models (Claude 3.7, Gemini 2.5, GPT-5)
2. Select with reasoning model (o1)
3. Cost: High, Performance: Highest (75.2%)

**Single Model Strategy (Warp-style):**
1. Use best single model (GPT-5, Claude Sonnet 4)
2. Optimize prompts and tools
3. Cost: Medium, Performance: High (71-75.8%)

---

### 6.3 Context Management Recommendations

**Do:**
- ✅ Use AST-based chunking (Tree-sitter)
- ✅ Build repository map early
- ✅ Implement prompt caching from day 1
- ✅ Hybrid search (grep + semantic + graph)
- ✅ Cache stable context (tools, instructions)
- ✅ Auto-compact when approaching limits

**Don't:**
- ❌ Dump entire codebase into context
- ❌ Re-read unchanged files
- ❌ Use fixed-size chunking for code
- ❌ Rely solely on embedding search
- ❌ Ignore context window limits
- ❌ Skip repository structure analysis

**Optimal Context Budget (200K example):**
```
20K:  System instructions, tool definitions (cached)
30K:  Repository map, project structure (cached)
50K:  Relevant code snippets (AST-based retrieval)
20K:  Conversation history (summarized)
80K:  Working space for agent
---
200K: Total
```

---

### 6.4 Tool Design Recommendations

**Essential Principles:**

1. **Compact & Efficient:**
   ```python
   # Bad: Separate tools
   navigate_to_file(path)
   open_file(path)
   scroll_to_line(line)
   read_current_view()

   # Good: Single tool
   view_file(path, start_line, end_line)
   ```

2. **Clear Feedback:**
   ```python
   # Bad
   return "done"

   # Good
   return {
       "success": true,
       "files_modified": ["app.py", "test.py"],
       "tests_passed": 5,
       "tests_failed": 0
   }
   ```

3. **Smart Defaults:**
   ```python
   def read_file(path, max_lines=100, context_lines=5):
       # Limit output by default
       # But allow override
   ```

4. **Batching:**
   ```python
   def edit_files(edits: List[FileEdit]):
       # Single tool call, multiple files
   ```

**Cross-Platform:**
```python
import os
import platform

def run_command(cmd, shell=None):
    if shell is None:
        shell = "bash" if platform.system() != "Windows" else "powershell"
    # Normalize paths, handle differences
```

**Error Handling:**
```python
def edit_file(path, search, replace):
    if not os.path.exists(path):
        return {"error": "File not found", "suggestion": "Use file_search to locate file"}

    if search not in content:
        return {
            "error": "Search string not found",
            "suggestion": "Use grep_search to verify exact content"
        }
```

---

### 6.5 Testing & Validation Strategy

**Test-Driven Repair Workflow:**

1. **Before Coding:**
   ```
   - Run existing tests (baseline)
   - Reproduce the issue with test
   - Confirm test fails
   ```

2. **During Development:**
   ```
   - Run tests after each change
   - Linter on every file edit
   - Type checker (if available)
   ```

3. **Before Completion:**
   ```
   - All tests pass
   - No new linter errors
   - Code builds successfully
   - No regressions in coverage
   ```

**Spectrum-Based Fault Localization:**
```python
# If test suite available
1. Run tests, collect coverage
2. Identify passing vs failing tests
3. Calculate suspiciousness scores for each line
4. Focus on highest-scoring lines
```

**Validation Checklist:**
- [ ] Tests pass
- [ ] Linter clean
- [ ] Type checker passes
- [ ] No syntax errors
- [ ] Builds successfully
- [ ] Meets requirements
- [ ] No regressions

---

### 6.6 Cost Optimization Strategies

**High-Impact:**

1. **Prompt Caching (90% reduction):**
   ```python
   # Structure prompts with cacheable prefixes
   [System Instructions]    # Cached (20K)
   [Tool Definitions]       # Cached (10K)
   [Repository Context]     # Cached (30K)
   [Current Task]           # Not cached (variable)
   ```

2. **Right-Size Models:**
   ```python
   # Architect/Editor pattern
   def solve_issue(issue):
       solution = architect_model(issue)  # Expensive, smart
       code = editor_model(solution)      # Cheap, focused
       return code
   ```

3. **Avoid Redundant Reads:**
   ```python
   # Cache file contents in memory
   file_cache = {}

   def read_file_cached(path):
       if path not in file_cache:
           file_cache[path] = read_file(path)
       return file_cache[path]
   ```

4. **Efficient Tool Use:**
   ```python
   # Batch operations
   edit_files([
       FileEdit("app.py", search1, replace1),
       FileEdit("test.py", search2, replace2),
   ])  # 1 LLM call instead of 2
   ```

**Medium-Impact:**

5. **Context Compression:**
   - Summarize long conversations
   - Remove redundant context
   - Keep only relevant history

6. **Incremental Updates:**
   - Track file modifications
   - Only re-read changed sections
   - Use git diff for efficiency

7. **Tool Call Reduction:**
   - grep_search instead of reading all files
   - Smart filtering before LLM processing
   - Pre-compute repository maps

---

## 7. Common Pitfalls & What to Avoid

### 7.1 Architecture Pitfalls

**1. Over-Engineering:**
- ❌ Complex multi-agent systems as first attempt
- ❌ 50+ specialized tools
- ❌ Sophisticated planning/reasoning modules
- ✅ **Instead:** Start simple, add complexity only when justified

**2. Tool Overload:**
- ❌ Too many similar tools confuse agent
- ❌ Redundant functionality (3 ways to read files)
- ✅ **Instead:** 10-15 well-designed, distinct tools

**3. Ignoring Latency:**
- ❌ Multi-attempt systems in user-facing products
- ❌ Synchronous multi-agent coordination
- ✅ **Instead:** Single-attempt optimization, async where needed

**4. Context Mismanagement:**
- ❌ Dumping entire codebase into context
- ❌ Re-reading unchanged files every iteration
- ❌ No caching strategy
- ✅ **Instead:** Smart retrieval, caching, incremental updates

---

### 7.2 Tool Design Pitfalls

**1. Poor Error Messages:**
- ❌ "Error occurred"
- ❌ Technical stack traces to LLM
- ✅ **Instead:** "File not found. Use file_search('config') to locate it."

**2. Inconsistent Interfaces:**
- ❌ Some tools return strings, others JSON
- ❌ Different error handling per tool
- ✅ **Instead:** Uniform response format across all tools

**3. Unbounded Output:**
- ❌ Returning 10,000-line files
- ❌ No truncation on long outputs
- ✅ **Instead:** Smart limits, "...more" indicators, summaries

**4. Missing Validation:**
- ❌ Accepting malformed edits
- ❌ No syntax checking before applying
- ✅ **Instead:** Validate inputs, preview changes, rollback on error

---

### 7.3 Context Management Pitfalls

**1. "Lost in the Middle":**
- ❌ Assuming LLM reads all context equally
- ❌ Burying important info in middle of large context
- ✅ **Instead:** Put critical info at start/end, use retrieval

**2. Fixed-Size Chunking:**
- ❌ Split code at arbitrary character counts
- ❌ Break functions/classes mid-definition
- ✅ **Instead:** AST-based semantic chunking

**3. No Caching:**
- ❌ Re-sending same tool definitions every call
- ❌ Ignoring prompt caching features
- ✅ **Instead:** Cache stable context (90% cost reduction)

**4. Embedding-Only Search:**
- ❌ Rely solely on vector search at scale
- ❌ No keyword or graph-based retrieval
- ✅ **Instead:** Hybrid search (semantic + grep + graph)

---

### 7.4 Testing & Validation Pitfalls

**1. Weak Test Suites:**
- ❌ Accepting patches that pass insufficient tests
- ❌ No edge case coverage
- ✅ **Instead:** Generate comprehensive tests, validate thoroughly

**2. No Incremental Validation:**
- ❌ Only test at the end
- ❌ Accumulate errors before catching them
- ✅ **Instead:** Test after every change, fail fast

**3. Ignoring Linter/Type Checker:**
- ❌ Skip static analysis
- ❌ Introduce type errors
- ✅ **Instead:** Run linter, type checker on every edit

**4. Trusting LLM Verification:**
- ❌ "Tests pass" without actually running them
- ❌ Hallucinated validation
- ✅ **Instead:** Always execute tests, never trust without verification

---

### 7.5 Cost Pitfalls

**1. No Cost Tracking:**
- ❌ Don't measure cost per task
- ❌ Optimize for accuracy only
- ✅ **Instead:** Track cost, optimize efficiency score

**2. Wrong Model for Task:**
- ❌ GPT-4 Opus for simple edits
- ❌ Weakest model for complex reasoning
- ✅ **Instead:** Right-size model per subtask

**3. Redundant Processing:**
- ❌ Re-embed unchanged code
- ❌ Re-read entire files for small changes
- ✅ **Instead:** Incremental updates, caching

**4. Unoptimized Prompts:**
- ❌ Verbose instructions
- ❌ Redundant examples
- ✅ **Instead:** Concise prompts, cache stable parts

---

### 7.6 Multi-Agent Pitfalls

**1. Premature Multi-Agent:**
- ❌ Start with multi-agent architecture
- ❌ No baseline single-agent comparison
- ✅ **Instead:** Prove multi-agent beats single-agent

**2. Poor Coordination:**
- ❌ Agents work at cross-purposes
- ❌ No clear handoff protocols
- ✅ **Instead:** Well-defined interfaces, clear responsibilities

**3. Compounding Errors:**
- ❌ Early agent error cascades
- ❌ No validation between stages
- ✅ **Instead:** Validate at each handoff, allow backtracking

**4. Token Overhead:**
- ❌ 15x token usage vs single-agent
- ❌ Redundant context in every agent
- ✅ **Instead:** Shared context, lightweight messages, artifacts

---

### 7.7 Debugging & Iteration Pitfalls

**1. Infinite Loops:**
- ❌ Agent repeats same failed action
- ❌ No max iteration limits
- ✅ **Instead:** Detect loops, timeout, track progress

**2. Context Overflow:**
- ❌ Keep entire history forever
- ❌ No summarization
- ✅ **Instead:** Auto-compact, summarize old interactions

**3. No Rollback:**
- ❌ Can't undo bad changes
- ❌ No checkpoints
- ✅ **Instead:** Git-based or checkpoint-based rollback

**4. Poor Failure Signals:**
- ❌ Agent doesn't know it failed
- ❌ No clear success criteria
- ✅ **Instead:** Explicit validation, clear done conditions

---

### 7.8 User Experience Pitfalls

**1. Over-Reliance Encouragement:**
- ❌ Agent does everything without user input
- ❌ No explanation of changes
- ✅ **Instead:** Explain reasoning, involve user in decisions

**2. No Progress Visibility:**
- ❌ Silent operation for minutes
- ❌ No status updates
- ✅ **Instead:** Stream output, TODO lists, progress indicators

**3. Brittle Workflows:**
- ❌ Fails on minor variations
- ❌ No graceful degradation
- ✅ **Instead:** Robust error handling, ask for clarification

**4. Documentation Failures:**
- ❌ Agent doesn't update docs
- ❌ Changes not explained
- ✅ **Instead:** Update docs, generate change summaries

---

## 8. Key Research Papers & Resources

### 8.1 Essential Papers (2024-2025)

**SWE-bench & Benchmarks:**
1. **SWE-bench: Can Language Models Resolve Real-World GitHub Issues?**
   - Original benchmark paper
   - https://www.swebench.com/

2. **Introducing SWE-bench Verified**
   - OpenAI, 500 verified issues
   - https://openai.com/index/introducing-swe-bench-verified/

3. **Dissecting the SWE-Bench Leaderboards** (2506.17208)
   - Profiling submitters and architectures
   - Comprehensive analysis of approaches

**Agent Architectures:**
4. **Agentless: Demystifying LLM-based Software Engineering Agents** (2407.01489)
   - https://arxiv.org/abs/2407.01489
   - Simple 3-phase pipeline
   - 50.8% on verified, $0.70/problem

5. **AutoCodeRover: Autonomous Program Improvement** (2404.05427)
   - https://arxiv.org/abs/2404.05427
   - ISSTA 2024
   - AST-based approach

6. **SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering** (2405.15793)
   - https://arxiv.org/abs/2405.15793
   - NeurIPS 2024
   - Tool design focus

7. **OpenHands: An Open Platform for AI Software Developers** (2407.16741)
   - https://arxiv.org/abs/2407.16741
   - ICLR 2025
   - Multi-agent platform

8. **MASAI: Modular Architecture for Software-engineering AI Agents** (2406.11638)
   - https://arxiv.org/abs/2406.11638
   - 5 specialized sub-agents

9. **Executable Code Actions Elicit Better LLM Agents** (2402.01030)
   - https://arxiv.org/abs/2402.01030
   - ICML 2024
   - CodeAct framework

**Context Management:**
10. **cAST: Enhancing Code Retrieval-Augmented Generation with Structural Chunking** (2506.15655)
    - https://arxiv.org/abs/2506.15655
    - AST-based chunking
    - +2.7-5.5 point gains

11. **Context Engineering for Agents**
    - https://blog.langchain.com/context-engineering-for-agents/
    - Practical strategies

**Failure Analysis:**
12. **An Empirical Study on Failures in Automated Issue Solving** (2509.13941)
    - https://arxiv.org/abs/2509.13941
    - Comprehensive failure taxonomy

13. **Understanding Code Agent Behaviour: Success and Failure Trajectories** (2511.00197)
    - https://arxiv.org/abs/2511.00197
    - Empirical analysis

### 8.2 Key GitHub Repositories

**Open-Source Agents:**
- **Agentless:** https://github.com/OpenAutoCoder/Agentless
- **AutoCodeRover:** https://github.com/AutoCodeRoverSG/auto-code-rover
- **SWE-agent:** https://github.com/princeton-nlp/SWE-agent
- **mini-SWE-agent:** https://github.com/SWE-agent/mini-swe-agent
- **OpenHands:** https://github.com/All-Hands-AI/OpenHands
- **Aider:** https://github.com/paul-gauthier/aider
- **CodeAct:** https://github.com/xingyaoww/code-act

**Benchmarks:**
- **SWE-bench:** https://github.com/SWE-bench/SWE-bench
- **Devin Results:** https://github.com/CognitionAI/devin-swebench-results

### 8.3 Important Blog Posts & Technical Reports

**Company Blogs:**
1. **Warp: SWE-bench Verified 71%**
   - https://www.warp.dev/blog/swe-bench-verified
   - Single-agent architecture insights

2. **Claude Code Best Practices (Anthropic)**
   - https://www.anthropic.com/engineering/claude-code-best-practices
   - Official guidelines

3. **Building Effective AI Agents (Anthropic)**
   - https://www.anthropic.com/research/building-effective-agents
   - Design principles

4. **How Claude Code is Built**
   - https://newsletter.pragmaticengineer.com/p/how-claude-code-is-built
   - Architecture deep-dive

5. **Cognition: SWE-bench Technical Report**
   - https://cognition.ai/blog/swe-bench-technical-report
   - Devin methodology

6. **Augment Code: #1 Open-Source Agent**
   - https://www.augmentcode.com/blog/1-open-source-agent-on-swe-bench-verified-by-combining-claude-3-7-and-o1

**Technical Guides:**
7. **Aider Edit Formats**
   - https://aider.chat/docs/more/edit-formats.html
   - Comprehensive format comparison

8. **Prompt Caching (Anthropic)**
   - https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching
   - Implementation guide

### 8.4 Leaderboards & Benchmarks

- **SWE-bench Main:** https://www.swebench.com/
- **SWE-bench Verified:** Track verified issues
- **SWE-bench Live:** https://swe-bench-live.github.io/
- **SWE-bench Pro:** https://scale.com/leaderboard/swe_bench_pro_public
- **SWE-rebench:** https://swe-rebench.com (cost analysis)

---

## 9. Actionable Recommendations

### 9.1 Quick Start: Building Your First Agent

**Day 1: MVP**
```python
# Core architecture
class SimpleCodeAgent:
    def __init__(self, model="claude-sonnet-4"):
        self.model = model
        self.tools = [
            read_file,
            edit_file,  # search/replace format
            bash,
            grep_search,
            list_files,
        ]

    def solve_issue(self, issue_description):
        # 1. Understand the issue
        context = self.gather_context(issue_description)

        # 2. Generate fix
        fix = self.generate_fix(context)

        # 3. Validate
        success = self.validate_fix()

        return success
```

**Day 2-3: Context Management**
- Implement repository map (file tree + symbols)
- Add AST-based file parsing (tree-sitter)
- Prompt caching for tool definitions

**Week 2: Testing & Iteration**
- Test-driven repair workflow
- Linter integration
- Failure analysis and improvements

**Month 2: Optimization**
- Cost tracking and optimization
- Context retrieval improvements
- Tool refinement based on usage

---

### 9.2 Evaluation Checklist

**Before Launch:**
- [ ] Tested on SWE-bench Lite subset
- [ ] Cost per issue <$5
- [ ] Success rate >30% on eval set
- [ ] No infinite loops (max iterations enforced)
- [ ] Rollback mechanism works
- [ ] Tests validated (not just trusted)
- [ ] Prompt caching implemented
- [ ] Error messages guide next actions
- [ ] Cross-platform compatible (or documented limits)

**Performance Metrics:**
- [ ] Success rate (% issues resolved)
- [ ] Cost per issue (average)
- [ ] Cost per successful fix
- [ ] Time to completion (median)
- [ ] Token usage per task
- [ ] Tool call efficiency (successful calls / total calls)
- [ ] Context overflow rate

**Quality Metrics:**
- [ ] Test pass rate
- [ ] Linter error rate
- [ ] Regression rate (working code broken)
- [ ] Code review score (human eval)

---

### 9.3 Optimization Priorities (by Impact)

**Tier 1 (Highest Impact):**
1. ⭐⭐⭐ Model quality (try Claude 3.7, GPT-5)
2. ⭐⭐⭐ Prompt caching (90% cost reduction)
3. ⭐⭐⭐ AST-based context (2.7-5.5 point gain)
4. ⭐⭐⭐ Test-driven validation

**Tier 2 (Medium Impact):**
5. ⭐⭐ Edit format optimization
6. ⭐⭐ Tool batching
7. ⭐⭐ Repository map/knowledge graph
8. ⭐⭐ Right-size models per task

**Tier 3 (Lower Impact, Do Later):**
9. ⭐ Multi-agent (only if single-agent plateaus)
10. ⭐ Complex planning modules
11. ⭐ Extensive tool library (>20 tools)

---

### 9.4 When to Use What

**Single-Agent:**
- ✅ User-facing products (latency critical)
- ✅ Simple to medium complexity tasks
- ✅ Cost-conscious applications
- ✅ Starting out / MVP stage

**Multi-Agent (Architect/Editor):**
- ✅ Clear reasoning vs execution split
- ✅ Cost optimization with quality preservation
- ✅ Tasks benefit from specialized expertise

**Multi-Agent (Full Pipeline):**
- ✅ Complex, decomposable problems
- ✅ Research/benchmark optimization
- ✅ Cost is secondary to performance
- ✅ Natural task boundaries

**Agentless (Pipeline):**
- ✅ Ultra-low cost requirement
- ✅ Simple, well-defined problems
- ✅ No interactive user feedback needed
- ✅ Batch processing

---

### 9.5 Future Trends (2025+)

**Emerging Patterns:**

1. **Simplicity Wins:** mini-SWE trend continues
2. **Model Ensembles:** TRAE-style multi-model approaches
3. **Context Efficiency:** Smart retrieval beats large windows
4. **Test-Driven:** More emphasis on validation
5. **Cost Awareness:** Efficiency metrics gain importance

**Watch These Developments:**
- SWE-bench Live (prevent overfitting)
- Longer-horizon benchmarks (SWE-bench Pro)
- Multimodal agents (handle UIs, diagrams)
- Learning/memory systems
- Formal verification integration

**Model Trends:**
- Longer context windows (but diminishing returns)
- Specialized code models
- Faster, cheaper alternatives
- Built-in tool use optimization

---

## 10. Conclusion & Summary

### 10.1 Key Takeaways

**Architecture:**
- ✅ Simple often beats complex (mini-SWE: 100 lines → 65%)
- ✅ Single-agent competitive with multi-agent (Warp: 71%)
- ✅ Tool design matters more than tool quantity
- ✅ Iteration + validation > perfect first try

**Context Management:**
- ✅ AST-based chunking: +2.7-5.5 points
- ✅ Smart retrieval > large context windows
- ✅ Prompt caching: 90% cost reduction
- ✅ Hybrid search (semantic + grep + graph)

**Performance:**
- ✅ SOTA: 75.2% on SWE-bench Verified (June 2025)
- ✅ Cost: $0.70 - $32.50 per fix (40x variation!)
- ✅ Model quality is highest-impact factor
- ✅ Test-driven validation critical

**What Works:**
- ✅ Claude 3.5/3.7 Sonnet, GPT-5, Gemini 2.5 Pro
- ✅ Search/replace edit format
- ✅ Compact, efficient tools (10-15)
- ✅ Repository maps + AST navigation
- ✅ Multiple patch generation + ranking

**What to Avoid:**
- ❌ Over-engineering (complex multi-agent too early)
- ❌ Tool overload (50+ tools)
- ❌ Ignoring cost/latency
- ❌ Trusting LLM without validation
- ❌ Fixed-size chunking for code
- ❌ No prompt caching

---

### 10.2 The Path Forward

**For Practitioners:**
1. Start with mini-SWE-agent approach (simplicity)
2. Implement prompt caching immediately
3. Use AST-based context retrieval
4. Test-driven development workflow
5. Measure cost and performance
6. Optimize incrementally

**For Researchers:**
1. Focus on context efficiency
2. Better failure detection and recovery
3. Learning from successful patterns
4. Formal verification integration
5. Longer-horizon task handling
6. Cost-performance tradeoffs

**For the Field:**
- We've gone from 13.86% → 75.8% in 18 months
- Simplicity and efficiency increasingly important
- Cost optimization now part of SOTA discussion
- Test-driven approaches gaining traction
- Still room for improvement (24.2% unsolved on verified)

---

### 10.3 Bottom Line

**What Makes a Successful AI Coding Agent:**

1. **Right Model** (Claude 3.7, GPT-5, etc.)
2. **Smart Context** (AST + retrieval + caching)
3. **Good Tools** (compact, efficient, 10-15)
4. **Validation** (tests, linter, verification)
5. **Simplicity** (start simple, add complexity only when justified)
6. **Cost Awareness** (track, optimize, cache)

**The 80/20 Rule:**
- 80% of performance comes from: model quality, context management, validation
- 20% from: architecture complexity, specialized tools, multi-agent coordination

**Start Simple, Stay Simple:**
The best agent architecture is the simplest one that achieves your goals.

---

## Appendix A: SWE-bench Verified Top 20 (June 2025)

| Rank | Agent | Score | Architecture | Model(s) |
|------|-------|-------|--------------|----------|
| 1 | TRAE | 75.2% | Multi-model ensemble | Claude 4 Sonnet, Opus, 3.7 |
| 2 | Warp (GPT-5) | 75.8% | Single-agent | GPT-5 |
| 3 | Warp | 71% | Single-agent | GPT-4.5 |
| 4 | Devlo | 70.2% | Multi-LLM | Claude 3.7, o3, Gemini 2.5 |
| 5 | Augment Code | 65.4% | Ensemble | Claude 3.7, O1 |
| 6 | mini-SWE-agent | 65% | Minimalist | Various |
| 7 | CodeStory Midwit | 62% | Brute-force | Various |
| 8 | Agentless | 50.8% | Pipeline | Claude 3.5 |
| 9 | AutoCodeRover | 46.2% | AST-agent | Various |
| 10 | OpenHands | 26% | Multi-agent | Various |

*(Note: Scores from different time periods; not all directly comparable)*

---

## Appendix B: Tool Design Examples

### Example 1: Efficient File Viewing
```python
def view_file(path: str, start_line: int = 1,
              end_line: int = None, max_lines: int = 100) -> Dict:
    """
    View contents of a file with optional line range.

    Combines: navigate, open, scroll, read into one operation.
    """
    if not os.path.exists(path):
        return {
            "error": "File not found",
            "suggestion": f"Use file_search('{os.path.basename(path)}') to locate it"
        }

    with open(path) as f:
        lines = f.readlines()

    if end_line is None:
        end_line = min(start_line + max_lines, len(lines))

    content = ''.join(lines[start_line-1:end_line])

    return {
        "success": True,
        "path": path,
        "total_lines": len(lines),
        "showing_lines": f"{start_line}-{end_line}",
        "content": content,
        "truncated": end_line < len(lines)
    }
```

### Example 2: Batch File Editing
```python
@dataclass
class FileEdit:
    path: str
    search: str
    replace: str

def edit_files(edits: List[FileEdit]) -> Dict:
    """
    Apply multiple edits in a single operation.
    Atomic: all succeed or all fail.
    """
    # Validate all edits first
    for edit in edits:
        if not os.path.exists(edit.path):
            return {"error": f"File not found: {edit.path}"}

        with open(edit.path) as f:
            content = f.read()

        if edit.search not in content:
            return {
                "error": f"Search string not found in {edit.path}",
                "search": edit.search[:100]
            }

    # All validated, apply edits
    modified = []
    for edit in edits:
        with open(edit.path, 'r+') as f:
            content = f.read()
            content = content.replace(edit.search, edit.replace)
            f.seek(0)
            f.write(content)
            f.truncate()
        modified.append(edit.path)

    return {
        "success": True,
        "files_modified": modified,
        "edit_count": len(edits)
    }
```

### Example 3: Smart Grep Search
```python
def grep_search(pattern: str, file_pattern: str = "*",
                max_results: int = 50, context_lines: int = 2) -> Dict:
    """
    Search for pattern in files matching file_pattern.
    Returns results with context.
    """
    import subprocess

    cmd = [
        "rg",  # ripgrep
        pattern,
        "--glob", file_pattern,
        "-n",  # line numbers
        f"-C{context_lines}",  # context
        "--json"  # structured output
    ]

    result = subprocess.run(cmd, capture_output=True, text=True)

    # Parse JSON output
    matches = []
    for line in result.stdout.splitlines():
        if line.strip():
            data = json.loads(line)
            if data["type"] == "match":
                matches.append({
                    "file": data["data"]["path"]["text"],
                    "line": data["data"]["line_number"],
                    "content": data["data"]["lines"]["text"],
                })

    if len(matches) > max_results:
        matches = matches[:max_results]
        truncated = True
    else:
        truncated = False

    return {
        "success": True,
        "pattern": pattern,
        "matches": matches,
        "total_found": len(matches),
        "truncated": truncated
    }
```

---

## Appendix C: Glossary

**Agent-Computer Interface (ACI):** Custom command and feedback formats designed specifically for LLM agents (SWE-agent concept)

**AST (Abstract Syntax Tree):** Hierarchical tree representation of code structure

**cAST:** Chunking via Abstract Syntax Trees - method for creating semantically meaningful code chunks

**CodeAct:** Framework that consolidates LLM agent actions into executable Python code

**Prompt Caching:** Reusing previously processed context to reduce cost and latency

**RAG (Retrieval-Augmented Generation):** Using retrieval to find relevant context before generation

**Repository Map:** High-level index of codebase structure (files, functions, classes)

**SBFL (Spectrum-Based Fault Localization):** Using test coverage to identify likely bug locations

**Search/Replace Format:** Edit format showing search block and replacement block

**SWE-bench:** Benchmark of real GitHub issues for evaluating coding agents

**SWE-bench Verified:** Subset of 500 verified solvable issues (most reliable)

**Tree-sitter:** Incremental parsing library supporting 81 languages

**Unified Diff:** Standard diff format (modified for LLM use)

---

**End of Report**

*For updates and latest developments, monitor:*
- SWE-bench leaderboard: https://www.swebench.com/
- arXiv cs.SE and cs.AI categories
- Major AI lab blogs (Anthropic, OpenAI, Google DeepMind)
- Open-source repositories listed in Appendix B
