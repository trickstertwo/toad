# State-of-the-Art LLM Context Optimization for Coding Agents
## Comprehensive Research Report (2024-2025)

**Research Date:** November 2025
**Focus:** Minimizing redundant reading and maximizing information density in AI coding agents

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Context Compaction Methods](#1-context-compaction-methods)
3. [Smart Context Selection](#2-smart-context-selection)
4. [Caching Strategies](#3-caching-strategies)
5. [Memory Architectures](#4-memory-architectures)
6. [Practical Implementations](#5-practical-implementations)
7. [Recommendations for Rust Implementation](#6-recommendations-for-rust-implementation)
8. [Tools and Libraries](#7-tools-and-libraries)
9. [References](#8-references)

---

## Executive Summary

The context window bottleneck remains the fundamental challenge for LLM-based coding agents. While context windows have grown from 4K tokens (ChatGPT 2022) to 1M+ tokens (Gemini 1.5 Pro, GPT-4.1) and even 10M tokens (Llama 4), computational costs scale quadratically: **doubling sequence length requires 4x memory and compute**.

Key findings:
- **Prompt compression** achieves 4-20x reduction with minimal performance loss (<2%)
- **Semantic caching** reduces API calls by 60-70% with >97% accuracy
- **Repository mapping** enables whole-codebase awareness in 1K tokens
- **Context rot** degrades performance as input grows beyond ~100K tokens
- **Production agents** save developers 5-6 hours/week with 2x coding speed

---

## 1. Context Compaction Methods

### 1.1 LLMLingua Series (Microsoft Research)

#### **LLMLingua (EMNLP 2023)**

**Paper:** https://arxiv.org/abs/2310.05736
**GitHub:** https://github.com/microsoft/LLMLingua
**Website:** https://llmlingua.com/

**Approach:** Uses a well-trained small language model (GPT2-small or LLaMA-7B) to identify and remove unimportant tokens from prompts.

**Quantitative Results:**
- **20x compression** with minimal performance loss
- GSM8K: Only **1.5% performance drop** at 20x compression
- Exact Match scores: -1.44 at 14x, -1.52 at 20x compression
- End-to-end latency: **1.7-5.7x speedup**

**Key Innovation:** Token-level importance scoring using perplexity from small LM

**Example:**
```
Original (100 tokens): "The function calculateTotal() takes an array of numbers as input and iterates through each element, accumulating the sum into a variable called total, which is initialized to zero at the beginning..."

Compressed (20 tokens): "calculateTotal() array numbers iterates elements accumulating sum total initialized zero"
```

#### **LongLLMLingua (ACL 2024)**

**Paper:** https://arxiv.org/abs/2310.06839
**PDF:** https://aclanthology.org/2024.acl-long.91.pdf

**Designed for:** Long-context scenarios (RAG, chatbots, 10K+ token prompts)

**Quantitative Results:**
- **21.4% performance improvement** with 4x compression (NaturalQuestions)
- **17.1% average improvement** across benchmarks
- **1.4-2.6x latency reduction** for 10K token prompts at 2-6x compression
- Mitigates "lost in the middle" problem

**Key Innovation:** Question-aware coarse-to-fine compression
- Coarse-grained: Removes irrelevant documents
- Fine-grained: Compresses remaining content

**Benchmarks:** NaturalQuestions, LongBench, ZeroSCROLLS, MuSicQue, LooGLE

#### **LLMLingua-2 (ACL 2024)**

**Paper:** https://llmlingua.com/llmlingua2.html

**Approach:** Data distillation from GPT-4 → train BERT-level encoder for token classification

**Quantitative Results:**
- **14x compression** on GSM8K with maintained performance
- **3-6x faster** than LLMLingua
- Superior out-of-domain generalization
- Handles complex 9-step Chain-of-Thought prompts

**Key Innovation:** Task-agnostic compression via learned annotations

**Benchmarks:** MeetingBank, LongBench, ZeroScrolls, GSM8K, BBH

### 1.2 Semantic Deduplication

**Approach:** Remove semantically redundant information using embeddings

**Implementation Pattern:**
```rust
// Pseudo-code for semantic deduplication
fn deduplicate_context(chunks: Vec<String>) -> Vec<String> {
    let embeddings = embed_chunks(&chunks);
    let mut selected = vec![0]; // Start with first chunk

    for i in 1..chunks.len() {
        let similarities = selected.iter()
            .map(|&j| cosine_similarity(embeddings[i], embeddings[j]))
            .collect();

        // Only include if sufficiently different
        if similarities.iter().all(|&sim| sim < 0.85) {
            selected.push(i);
        }
    }

    selected.into_iter().map(|i| chunks[i].clone()).collect()
}
```

### 1.3 Differential Context (Only Send Changes)

**Git Context Controller (GCC)** - ACL 2024
**Paper:** https://arxiv.org/abs/2508.00031

**Approach:** Integrate version control semantics (COMMIT, BRANCH, MERGE) into reasoning loop

**Results:**
- **48% success rate** on SWE-Bench-Lite
- Manages context growth in long-horizon tasks
- State-of-the-art performance

**Key Insight:** Instead of sending full files, track incremental changes like Git

**Challenge:** Git diffs can explode to 100K+ tokens in large PRs (package updates, renames)

**Solutions:**
- **Difftastic:** Language-aware syntax diff (structure-based, not line-based)
- **Tree-sitter diffs:** Incremental AST parsing for minimal change detection
- **Context limits:** Focus on changed functions + immediate dependencies

### 1.4 Conversation Summarization

**Recursive Summarization (ACL 2024)**
**Paper:** https://arxiv.org/abs/2308.15022

**Approach:** Recursively generate summaries/memory
1. Memorize small dialogue contexts
2. Produce new memory from previous memory + new context
3. Replace old context with summary

**LoCoMo Benchmark (ACL 2024)**
**Paper:** https://arxiv.org/abs/2402.17753
**Website:** https://snap-research.github.io/locomo/

**Dataset:**
- 300 turns, 9K tokens average
- Up to 35 sessions
- Question answering, event summarization, multi-modal generation

**Key Finding:** LLMs struggle with long-term conversational memory
- Challenges in understanding lengthy conversations
- Poor long-range temporal and causal dynamics
- RAG offers improvements but still lags human performance

**Best Practice:** Transform dialogues into database of assertions about each speaker

---

## 2. Smart Context Selection

### 2.1 Repository Mapping (Aider's Approach)

**Blog:** https://aider.chat/docs/repomap.html
**GitHub:** https://github.com/Aider-AI/aider
**Tree-sitter Post:** https://aider.chat/2023/10/22/repomap.html

#### **How It Works**

1. **Build Repository Map:** Use tree-sitter to extract all function/class signatures
2. **Graph Ranking:** Create dependency graph where:
   - Nodes = source files
   - Edges = dependencies (imports, references)
3. **Optimize Selection:** PageRank-style algorithm to select most relevant portions
4. **Token Budget:** Default 1K tokens for repo map

**Example Repository Map:**
```
src/main.rs:
  ⎮pub fn main()
  ⎮pub struct Config

src/parser.rs:
  ⎮pub fn parse_file(path: &Path) -> Result<AST>
  ⎮pub struct AST { nodes: Vec<Node> }

src/analyzer.rs:
  ⎮pub fn analyze(ast: &AST) -> Analysis
  ⎮  dependencies: parser::AST
```

**Key Advantage:** LLM can request specific files based on map without loading entire codebase

**Modern Implementation:** Tree-sitter (richer) replaced ctags (original)

### 2.2 AST-Based Relevance Detection

**Semantic Code Indexing (October 2024)**
**Article:** https://medium.com/@email2dineshkuppan/semantic-code-indexing-with-ast-and-tree-sitter-for-ai-agents-part-1-of-3-eb5237ba687a

**Key Distinction:**
- **AST:** Distilled semantic view (ignores formatting, focuses on meaning)
- **Tree-sitter:** Keeps all tokens, whitespace, delimiters (precise mapping to source)

**Best Practice:** Use both together
- ASTs for indexing (semantic understanding)
- Tree-sitter for retrieval (exact line/column mapping)

#### **RAG with AST-Based Chunking**

**Blog:** https://vxrl.medium.com/enhancing-llm-code-generation-with-rag-and-ast-based-chunking-5b81902ae9fc

**Approach:** Split code at meaningful boundaries
- Function definitions
- Class definitions
- Control structures
- Each chunk remains syntactically valid

**Advantages:**
- Preserves syntactic integrity
- Meaningful context in each chunk
- Better embedding quality

**Implementation:**
```python
import tree_sitter

def chunk_by_ast(code: str, language: str) -> List[str]:
    parser = tree_sitter.Parser()
    parser.set_language(tree_sitter.Language(language))
    tree = parser.parse(bytes(code, "utf8"))

    chunks = []
    for node in tree.root_node.children:
        if node.type in ['function_definition', 'class_definition']:
            chunks.append(code[node.start_byte:node.end_byte])

    return chunks
```

### 2.3 Dependency Graph Analysis

**CodexGraph (SIGKDD 2024)**
**Paper:** https://arxiv.org/abs/2408.03910
**Blog:** https://www.marktechpost.com/2024/08/11/codexgraph-an-artificial-intelligence-ai-system-that-integrates-llm-agents-with-graph-database-interfaces-extracted-from-code-repositories/

**Approach:** Graph database for code structure
- Task-agnostic schema
- Universal interface for LLM agents
- Precise, structure-aware context retrieval

**CodePlan (ICLR 2024)**
**Paper:** https://huggingface.co/papers/2309.12499

**Approach:** Repository-level coding with planning
- **Spatial context:** Dependency graph analysis
- **Temporal context:** Plan graph for change sequence
- Incremental dependency analysis
- Change may-impact analysis

**Custom Prompts:**
- Task instructions
- Previous changes
- Causes for change
- Spatial context (related code)
- Snippet to change

**LLM4FPM (2024)**
**Paper:** https://arxiv.org/abs/2411.03079

**eCPG-Slicer:**
- Extended Code Property Graph
- Line-level precise code contexts
- File Reference Graph (FARF algorithm)
- Linear time to detect all associated files

**Knowledge Graph Benefits:**
- Capture direct relationships (inheritance, dependencies, usage)
- Support reasoning (impact analysis)
- Rich contextual information for RAG
- Better than pure semantic search

### 2.4 Tree-sitter Integration

**Tree-sitter:** https://tree-sitter.github.io/

**Capabilities:**
- Incremental parsing (fast updates on edits)
- Error recovery (partial AST with ERROR nodes)
- Language-agnostic (100+ languages)
- Binding for Rust, Python, C, JavaScript, etc.

**Use Cases in Coding Agents:**

1. **Repository Mapping** (Aider)
   - Extract all function signatures
   - Build dependency graph
   - 1K token overview of entire codebase

2. **Linting for LLMs** (Aider)
   - Identify ERROR nodes in AST
   - Show context around errors (containing function)
   - Language-agnostic error detection

3. **Semantic Chunking**
   - Split at function/class boundaries
   - Preserve syntactic validity
   - Better RAG retrieval

4. **Code Completion** (Sourcegraph Cody)
   - Post-process LLM output
   - Syntactic filtering of bad suggestions
   - Truncate invalid completions

**Rust Integration:**
```rust
use tree_sitter::{Parser, Language, Query};

extern "C" { fn tree_sitter_rust() -> Language; }

fn parse_rust_code(code: &str) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();
    parser.parse(code, None).unwrap()
}

fn extract_functions(tree: &tree_sitter::Tree) -> Vec<String> {
    let query = Query::new(
        unsafe { tree_sitter_rust() },
        "(function_item name: (identifier) @func.name)"
    ).unwrap();

    // Extract function names...
}
```

### 2.5 Import/Reference Following

**IDECoder (ICSE 2024)**
**Paper:** https://arxiv.org/abs/2402.03630

**Problem:** Information dispersed across files
- Imported classes and member functions
- Global variables
- External classes with unknown types

**Solution:** IDE native static context
- Cross-file construction
- Type information propagation
- Rich cross-context information

**Challenge:** Current retrieval methods struggle with:
- Inheritance
- Polymorphism
- Complex namespaces

**Need:** Static analysis-enhanced retrieval

**IRIS (ACL 2024)**
**Paper:** https://arxiv.org/abs/2405.17238

**Neuro-symbolic approach:**
- LLM + static analysis
- Whole-repository reasoning
- Security vulnerability detection

**Results:**
- GPT-4: **55 vulnerabilities** detected
- CodeQL: **27 vulnerabilities**
- **5% lower** false discovery rate

---

## 3. Caching Strategies

### 3.1 Prompt Caching (Anthropic)

**Documentation:** https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching
**Blog:** https://www.anthropic.com/news/prompt-caching
**Guide:** https://medium.com/@mcraddock/unlocking-efficiency-a-practical-guide-to-claude-prompt-caching-3185805c0eef

#### **How It Works**

Store specific prompt contexts that can be reused across API calls

**Benefits:**
- **90% cost reduction** for cached tokens
- **85% latency reduction** for long prompts

**Requirements:**
- **Minimum:** 1024 tokens (Opus 4.1, Sonnet 4.5, Sonnet 4, Sonnet 3.7)
- **Minimum:** 2048 tokens (Haiku 3.5, Haiku 3)
- **Cache breakpoints:** Up to 4 available

**Recent Updates (2025):**
- Cache read tokens don't count toward ITPM (Input Tokens Per Minute) limit
- Tool calling: **70% output token reduction**, 14% average reduction
- Claude 3.7 Sonnet optimizations

**Best Practices:**
1. Place static content first (tool definitions, system instructions, RAG context)
2. Use all 4 cache breakpoints:
   - Tools cache
   - Reusable instructions cache
   - RAG context cache
   - Conversation history cache
3. Mark end of reusable content with `cache_control` parameter

**Example:**
```json
{
  "model": "claude-sonnet-4-5-20250929",
  "max_tokens": 1024,
  "system": [
    {
      "type": "text",
      "text": "You are a Rust expert...",
      "cache_control": {"type": "ephemeral"}
    }
  ],
  "messages": [...]
}
```

### 3.2 Prompt Caching (OpenAI)

**Documentation:** https://openai.com/index/api-prompt-caching/
**Blog:** https://blog.getbind.co/2024/10/03/openai-prompt-caching-how-does-it-compare-to-claude-prompt-caching/

#### **Launch:** October 1, 2024

**Models:** GPT-4o, GPT-4o mini, o1-preview, o1-mini, fine-tuned variants

**Key Differences from Claude:**
- **Automatic** (no code changes required)
- **No additional fees**
- **Transparent** (works behind the scenes)

**How It Works:**
- Enabled automatically for prompts **≥1024 tokens**
- Caches in **128-token increments** beyond initial 1024
- Cache active for **5-10 minutes** of inactivity

**Benefits:**
- **50% discount** on cached input tokens
- **80% latency reduction** for long prompts

**What Can Be Cached:**
- Complete messages array
- Images in user messages
- Messages array + tools list
- Structured output schema

**Comparison:**
- **OpenAI:** Fully automatic, 1 cache point
- **Anthropic:** Manual control, 4 cache breakpoints

### 3.3 KV-Cache Optimization

**Survey:** https://arxiv.org/abs/2412.19442
**GitHub:** https://github.com/TreeAI-Lab/Awesome-KV-Cache-Management
**Blog:** https://medium.com/@plienhar/llm-inference-series-4-kv-caching-a-deeper-look-4ba9a77746c8

#### **What is KV Cache?**

Key-Value cache stores attention keys and values to avoid recomputing for previous tokens

**Problem:** Memory intensive for long contexts
- Scales with sequence length × model size
- Dominates memory in long contexts

#### **Optimization Strategies**

**1. Token-Level Optimization**

**SentenceKV (2024)**
**Paper:** https://arxiv.org/abs/2504.00970
**GitHub:** https://github.com/zzbright1998/SentenceKV

- Sentence-level semantic management
- Richer semantic information than isolated tokens
- Dynamic cache allocation

**ClusterKV (2024)**
- Group tokens into semantic clusters
- Selectively recall during inference
- Balance accuracy and efficiency

**ChunkKV (2024)**
- Semantic chunking
- Retain informative segments
- Discard redundant ones

**2. Model-Level Optimization**

**FastGen (ICLR 2024)**
- **50% memory reduction**
- Maintained performance
- Optimized KV cache allocation

**3. System-Level Optimization**

**Entropy-Guided KV Caching (2024)**
**Paper:** https://www.mdpi.com/2227-7390/13/15/2366

- Higher-entropy layers → broader attention dispersion
- Allocate larger KV cache budgets to high-entropy layers
- Efficient cache distribution

**Microsoft Research (2024)**
**Blog:** https://www.microsoft.com/en-us/research/blog/llm-profiling-guides-kv-cache-optimization/

- LLM profiling to guide optimization
- Layer-specific analysis
- Workload-specific tuning

### 3.4 Semantic Caching

**GPTCache**
**GitHub:** https://github.com/zilliztech/GPTCache
**Tutorial:** https://www.datacamp.com/tutorial/gptcache-tutorial-enhancing-efficiency-in-llm-applications

**Approach:** Cache results based on semantic similarity, not exact match

**How It Works:**
1. Convert query to embedding
2. Store embedding in vector database
3. For new query: Find similar cached queries
4. Return cached result if similarity > threshold

**Quantitative Results:**
- **68.8% API call reduction** (max across categories)
- **61.6-68.8% cache hit rates**
- **>97% positive hit accuracy**
- **40-50% latency reduction**

**Integration:** Fully integrated with LangChain and llama_index

**GPT Semantic Cache (2024)**
**Paper:** https://arxiv.org/abs/2411.05276

**Implementation:** Redis for in-memory storage of query embeddings

**Results:**
- **68.8% reduction** in API calls
- **97%+ accuracy** on cache hits
- Significant cost savings for repetitive queries

**MeanCache (2024)**
**Paper:** https://arxiv.org/abs/2403.02694

**User-centric approach:**
- **31% of queries** similar to previous by same user
- Personalized cache per user
- Eliminate expensive redundant LLM queries

**Implementation Pattern:**
```rust
use redis::Client;

struct SemanticCache {
    redis: Client,
    embedding_model: Box<dyn EmbeddingModel>,
    similarity_threshold: f32,
}

impl SemanticCache {
    async fn get_or_compute<F, T>(&self, query: &str, compute_fn: F) -> T
    where F: FnOnce(&str) -> T
    {
        let embedding = self.embedding_model.embed(query);

        // Search for similar queries in vector store
        if let Some(cached) = self.find_similar(&embedding, self.similarity_threshold) {
            return cached;
        }

        // Compute and cache
        let result = compute_fn(query);
        self.cache(query, &embedding, &result);
        result
    }
}
```

### 3.5 Result Memoization

**Approach:** Cache LLM outputs keyed by input hash

**Use Cases:**
- Repeated code analysis
- Deterministic transformations
- Static documentation generation

**Simple Implementation:**
```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct ResultCache<K, V> {
    cache: HashMap<u64, V>,
}

impl<K: Hash, V: Clone> ResultCache<K, V> {
    fn get_or_insert_with<F>(&mut self, key: &K, f: F) -> V
    where F: FnOnce() -> V
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        self.cache.entry(hash)
            .or_insert_with(f)
            .clone()
    }
}
```

---

## 4. Memory Architectures

### 4.1 Short-Term vs Long-Term Memory

**Mem0 Framework**
**Paper:** https://arxiv.org/abs/2504.19413
**Website:** https://mem0.ai/
**Blog:** https://mem0.ai/blog/memory-in-agents-what-why-and-how

#### **Memory Types**

**Short-Term Memory:**
- Immediate context
- Current conversation state
- Working memory for active session
- Cleared after session ends

**Long-Term Memory:**
- Persistent across sessions
- User preferences
- Historical interactions
- Learned patterns

**Implementation Architectures:**

1. **Vector Databases**
   - Semantic search over past conversations
   - Embedding-based retrieval
   - Fast similarity search

2. **Knowledge Graphs**
   - Explicit relationships
   - Reasoning support
   - Structured knowledge

**Memory Consolidation:**
- Move from short-term → long-term based on:
  - Usage patterns
  - Recency
  - Significance
- Optimize recall speed vs storage

### 4.2 Working Memory for Current Task

**LangGraph + MongoDB (2024)**
**Blog:** https://www.mongodb.com/company/blog/product-release-announcements/powering-long-term-memory-for-agents-langgraph

**Checkpointer Pattern:**
- Save every step in agent graph
- Messages, transitions, agent state
- Enable replayability
- Mid-graph recovery
- Consistent context propagation

**Redis for Short-Term Memory (2024)**
**Blog:** https://redis.io/blog/build-smarter-ai-agents-manage-short-term-and-long-term-memory-with-redis/

**Fast access for working memory:**
- In-memory storage
- Low latency (<1ms)
- Session-specific context

### 4.3 Background Context vs Active Context

**Pattern:** Two-tier context management

**Active Context:**
- Currently edited files
- Recent conversation (last N turns)
- Immediate dependencies
- Limited to ~10-20K tokens

**Background Context:**
- Repository map
- Cached system instructions
- Tool definitions
- Project documentation
- Can use prompt caching (90% cost reduction)

**Subagent Pattern:**
**Article:** https://docs.langchain.com/oss/python/langchain/context-engineering

**Approach:**
- Main agent with lean context
- Spawn subagents for deep dives
- Subagent returns only final result
- Prevents context pollution

**Example:**
```
Main Agent Context (5K tokens):
  - Current file
  - Task description
  - Repository map

Subagent Context (50K tokens):
  - Deep analysis of dependency
  - Returns: "Function X is safe, no side effects"

Main Agent receives only: "safe, no side effects" (3 tokens)
```

### 4.4 A-Mem: Agentic Memory

**Paper:** https://arxiv.org/abs/2502.12110

**Architecture:**
- Structured memory management
- Task-specific memory partitions
- Automatic memory consolidation
- Context-aware retrieval

---

## 5. Practical Implementations

### 5.1 Aider

**GitHub:** https://github.com/Aider-AI/aider
**Website:** https://aider.chat/

**Context Optimization Techniques:**

1. **Repository Map**
   - Tree-sitter based
   - 1K token budget
   - Dependency graph ranking
   - Full codebase awareness

2. **Language-Agnostic Linting**
   - Tree-sitter ERROR nodes
   - Show errors in context of containing function
   - Avoid line number errors

3. **Smart File Selection**
   - LLM requests specific files from map
   - Only add necessary files to context
   - Dynamic context management

**Production Results:**
- Handles multi-file coordinated changes
- Works well in large repositories
- 100+ languages supported

### 5.2 Sourcegraph Cody

**Website:** https://sourcegraph.com/cody
**Blog:** https://sourcegraph.com/blog/the-lifecycle-of-a-code-ai-completion
**Research:** https://arxiv.org/abs/2408.05344

**Context Engine:**

1. **Retrieval Stage**
   - Find N most relevant code sections
   - Codebase-wide search

2. **Ranking Stage**
   - Score and prioritize results
   - Essential Recall, Essential Concision, Helpfulness

**Long Context Optimization (Gemini 1.5 Flash):**
- 1M token context window
- Time to first token: **5 seconds** (down from 30-40s)
- Prefetching mechanism
- Layered context model architecture

**Results:**
- Engineers save **5-6 hours/week**
- **2x faster** coding
- Enterprise deployments: Palo Alto Networks, Leidos

**Tree-sitter Usage:**
- Post-process LLM output
- Syntactic filtering
- Truncate invalid completions

### 5.3 Cursor

**Website:** https://cursor.sh/

**Context Awareness:**
- Index entire codebase
- Dramatically reduces hallucinations
- Better answer quality

**User Reports:**
- **20-55% time savings** per week
- Superior context awareness vs competitors

### 5.4 Augment Code

**Website:** https://www.augmentcode.com/

**Results:**
- **>40% productivity increase** across the board
- Quick understanding from zero context
- Very large codebase support

### 5.5 Google Gemini 1.5 Flash for Coding

**Blog:** https://developers.googleblog.com/en/supercharging-ai-coding-assistants-with-massive-context/

**Massive Context Window:**
- 1M tokens
- Entire repositories in single prompt

**Optimization:**
- Prefetching for fast first token
- Layered context architecture

---

## 6. Recommendations for Rust Implementation

### 6.1 Core Architecture

```rust
// Recommended architecture for Rust coding agent

pub struct ContextManager {
    // Active context (hot path)
    active_files: LruCache<PathBuf, FileContext>,
    conversation_history: ConversationBuffer,

    // Background context (cached)
    repo_map: Arc<RepositoryMap>,
    dependency_graph: Arc<DependencyGraph>,

    // Caching layers
    prompt_cache: PromptCache,
    semantic_cache: SemanticCache,
    result_cache: ResultCache,

    // Smart selection
    tree_sitter_parser: TreeSitterParser,
    embedding_model: Box<dyn EmbeddingModel>,
}

pub struct RepositoryMap {
    file_signatures: HashMap<PathBuf, Vec<Symbol>>,
    dependency_edges: Graph<PathBuf, DependencyType>,
    pagerank_scores: HashMap<PathBuf, f64>,
}

pub struct ConversationBuffer {
    messages: VecDeque<Message>,
    token_count: usize,
    max_tokens: usize,
    summarizer: ConversationSummarizer,
}
```

### 6.2 Specific Techniques to Implement

**Priority 1 (Immediate Impact):**

1. **Repository Map with Tree-sitter**
   - Use `tree-sitter` crate
   - Extract function signatures
   - Build dependency graph
   - PageRank for relevance

2. **Prompt Caching (Claude API)**
   - Anthropic client with cache_control
   - 4 breakpoints: tools, instructions, repo map, history
   - 90% cost reduction

3. **Conversation Summarization**
   - Summarize every 10-20 turns
   - Keep last 5 turns + summary
   - Reduce token growth

**Priority 2 (Significant Gains):**

4. **Semantic Caching**
   - Redis for vector storage
   - Embedding model: `nomic-embed-code`
   - 0.85 similarity threshold

5. **AST-Based Chunking**
   - Tree-sitter for boundaries
   - Chunk at function/class level
   - Better RAG retrieval

6. **Differential Context**
   - Track file versions
   - Send only changed functions
   - Include immediate dependencies

**Priority 3 (Refinement):**

7. **LLMLingua Integration**
   - Microsoft LLMLingua library
   - 4-6x compression
   - Use for historical context

8. **Knowledge Graph**
   - FalkorDB or Neo4j
   - Code entity relationships
   - GraphRAG for queries

### 6.3 Example: Tree-sitter Repository Map

```rust
use tree_sitter::{Parser, Language, Query, QueryCursor};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

extern "C" {
    fn tree_sitter_rust() -> Language;
    fn tree_sitter_python() -> Language;
}

pub struct RepositoryMapper {
    parsers: HashMap<String, Language>,
}

impl RepositoryMapper {
    pub fn new() -> Self {
        let mut parsers = HashMap::new();
        unsafe {
            parsers.insert("rust".to_string(), tree_sitter_rust());
            parsers.insert("python".to_string(), tree_sitter_python());
        }
        Self { parsers }
    }

    pub fn extract_symbols(&self, path: &Path, code: &str) -> Vec<Symbol> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let language = match ext {
            "rs" => self.parsers.get("rust"),
            "py" => self.parsers.get("python"),
            _ => return vec![],
        };

        let language = match language {
            Some(lang) => lang,
            None => return vec![],
        };

        let mut parser = Parser::new();
        parser.set_language(*language).unwrap();

        let tree = match parser.parse(code, None) {
            Some(tree) => tree,
            None => return vec![],
        };

        // Query for functions and classes
        let query_str = match ext {
            "rs" => "(function_item name: (identifier) @name) @def",
            "py" => "(function_definition name: (identifier) @name) @def",
            _ => return vec![],
        };

        let query = Query::new(*language, query_str).unwrap();
        let mut cursor = QueryCursor::new();
        let matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

        let mut symbols = Vec::new();
        for match_ in matches {
            let def_node = match_.captures[0].node;
            let name_node = match_.captures[1].node;

            let name = &code[name_node.byte_range()];
            let signature = &code[def_node.byte_range()];

            symbols.push(Symbol {
                name: name.to_string(),
                signature: signature.lines().next().unwrap_or("").to_string(),
                line: def_node.start_position().row,
            });
        }

        symbols
    }

    pub fn build_map(&self, repo_root: &Path) -> RepositoryMap {
        // Walk repository, extract symbols, build dependency graph
        // ... implementation ...
    }
}

pub struct Symbol {
    pub name: String,
    pub signature: String,
    pub line: usize,
}
```

### 6.4 Example: Semantic Cache with Redis

```rust
use redis::{Client, Commands};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CachedResult {
    query: String,
    embedding: Vec<f32>,
    result: String,
    timestamp: u64,
}

pub struct SemanticCache {
    redis: Client,
    threshold: f32,
}

impl SemanticCache {
    pub fn new(redis_url: &str, threshold: f32) -> redis::RedisResult<Self> {
        Ok(Self {
            redis: Client::open(redis_url)?,
            threshold,
        })
    }

    pub async fn get_similar(
        &self,
        query_embedding: &[f32]
    ) -> Option<String> {
        let mut con = self.redis.get_connection().unwrap();

        // In production, use Redis Vector Similarity Search
        // For now, simple iteration (use RediSearch in real impl)
        let keys: Vec<String> = con.keys("cache:*").unwrap();

        for key in keys {
            let cached_json: String = con.get(&key).unwrap();
            let cached: CachedResult = serde_json::from_str(&cached_json).unwrap();

            let similarity = cosine_similarity(query_embedding, &cached.embedding);
            if similarity >= self.threshold {
                return Some(cached.result);
            }
        }

        None
    }

    pub fn cache(
        &self,
        query: &str,
        embedding: Vec<f32>,
        result: &str
    ) -> redis::RedisResult<()> {
        let mut con = self.redis.get_connection()?;

        let cached = CachedResult {
            query: query.to_string(),
            embedding,
            result: result.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        let key = format!("cache:{}", uuid::Uuid::new_v4());
        let value = serde_json::to_string(&cached).unwrap();

        con.set_ex(&key, value, 3600)?; // 1 hour TTL

        Ok(())
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (mag_a * mag_b)
}
```

### 6.5 Example: Conversation Summarization

```rust
pub struct ConversationManager {
    messages: VecDeque<Message>,
    token_count: usize,
    max_active_tokens: usize,
    summary: Option<String>,
}

impl ConversationManager {
    pub async fn add_message(&mut self, msg: Message, llm: &LLMClient) {
        let msg_tokens = estimate_tokens(&msg.content);
        self.token_count += msg_tokens;
        self.messages.push_back(msg);

        // Summarize if exceeding budget
        if self.token_count > self.max_active_tokens {
            self.summarize(llm).await;
        }
    }

    async fn summarize(&mut self, llm: &LLMClient) {
        let keep_recent = 5;

        if self.messages.len() <= keep_recent {
            return;
        }

        // Get messages to summarize (all except last N)
        let to_summarize: Vec<_> = self.messages
            .iter()
            .take(self.messages.len() - keep_recent)
            .collect();

        // Create summary prompt
        let conversation = to_summarize.iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n\n");

        let summary_prompt = format!(
            "Summarize the following conversation, preserving key technical details, \
             decisions made, and context needed for future responses:\n\n{}",
            conversation
        );

        let new_summary = llm.complete(&summary_prompt).await;

        // Keep only recent messages + summary
        let recent: Vec<_> = self.messages
            .iter()
            .rev()
            .take(keep_recent)
            .cloned()
            .collect();

        self.messages.clear();
        self.messages.extend(recent.into_iter().rev());
        self.summary = Some(new_summary);

        // Recalculate token count
        self.token_count = self.messages.iter()
            .map(|m| estimate_tokens(&m.content))
            .sum();
        self.token_count += estimate_tokens(self.summary.as_ref().unwrap());
    }

    pub fn get_context(&self) -> String {
        let mut context = String::new();

        if let Some(summary) = &self.summary {
            context.push_str("Previous conversation summary:\n");
            context.push_str(summary);
            context.push_str("\n\n");
        }

        context.push_str("Recent messages:\n");
        for msg in &self.messages {
            context.push_str(&format!("{}: {}\n\n", msg.role, msg.content));
        }

        context
    }
}

fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: ~4 chars per token
    text.len() / 4
}
```

### 6.6 Token Budget Allocation

**Recommended Distribution:**

```rust
pub struct TokenBudget {
    total: usize,
    allocation: TokenAllocation,
}

pub struct TokenAllocation {
    system_instructions: usize,    // 500 tokens (cached)
    repository_map: usize,          // 1000 tokens (cached)
    tools_definition: usize,        // 1000 tokens (cached)
    conversation_summary: usize,    // 1000 tokens
    recent_conversation: usize,     // 2000 tokens
    active_files: usize,           // 10000 tokens
    dependency_context: usize,      // 3000 tokens
    retrieved_docs: usize,         // 2500 tokens
    buffer: usize,                 // 1000 tokens
}

impl Default for TokenBudget {
    fn default() -> Self {
        Self {
            total: 21000,
            allocation: TokenAllocation {
                system_instructions: 500,
                repository_map: 1000,
                tools_definition: 1000,
                conversation_summary: 1000,
                recent_conversation: 2000,
                active_files: 10000,
                dependency_context: 3000,
                retrieved_docs: 2500,
                buffer: 1000,
            }
        }
    }
}

// With Claude prompt caching, first 3 sections (2500 tokens) cost 10%
// after first request, saving ~2250 tokens worth of cost per request
```

---

## 7. Tools and Libraries

### 7.1 Code Analysis

| Tool | Language | Purpose | Link |
|------|----------|---------|------|
| **tree-sitter** | Rust, C, many bindings | Incremental parsing, AST extraction | https://tree-sitter.github.io/ |
| **universal-ctags** | C | Symbol extraction (legacy) | https://github.com/universal-ctags/ctags |
| **rust-analyzer** | Rust | Semantic analysis, IDE features | https://rust-analyzer.github.io/ |

### 7.2 Embedding Models

| Model | Context | Dimensions | Best For | Link |
|-------|---------|------------|----------|------|
| **nomic-embed-code** | 8192 | 768 | Code retrieval | https://huggingface.co/nomic-ai/nomic-embed-code |
| **nomic-embed-text-v1** | 8192 | 768 | General text | https://huggingface.co/nomic-ai/nomic-embed-text-v1 |
| **VoyageCode-3** | 16000 | 1024 | Code understanding | Voyage AI API |
| **text-embedding-3-small** | 8191 | 1536 | OpenAI baseline | OpenAI API |

### 7.3 Vector Databases

| Database | Best For | Link |
|----------|----------|------|
| **Redis** | Semantic caching, fast retrieval | https://redis.io/ |
| **Qdrant** | Rust-native, high performance | https://qdrant.tech/ |
| **Milvus** | Scalable, production | https://milvus.io/ |
| **Chroma** | Development, easy integration | https://www.trychroma.com/ |

### 7.4 Graph Databases

| Database | Best For | Link |
|----------|----------|------|
| **FalkorDB** | GraphRAG, LLM integration, fast | https://www.falkordb.com/ |
| **Neo4j** | Mature, rich ecosystem | https://neo4j.com/ |
| **Graphiti** | Real-time knowledge graphs | https://github.com/getzep/graphiti |

### 7.5 LLM Frameworks

| Framework | Language | Features | Link |
|-----------|----------|----------|------|
| **Swiftide** | Rust | Fast streaming, tree-sitter | https://swiftide.rs/ |
| **Rig** | Rust | RAG, tool calling | https://rig.rs/ |
| **LangChain** | Python/JS | Comprehensive, mature | https://www.langchain.com/ |
| **LlamaIndex** | Python | Data connectors, RAG | https://www.llamaindex.ai/ |

### 7.6 Compression Tools

| Tool | Compression | Latency | Link |
|------|-------------|---------|------|
| **LLMLingua** | 20x | 5.7x faster | https://github.com/microsoft/LLMLingua |
| **LongLLMLingua** | 4x | 2.6x faster | https://github.com/microsoft/LLMLingua |
| **LLMLingua-2** | 14x | 6x faster | https://llmlingua.com/llmlingua2.html |

### 7.7 Caching Solutions

| Solution | Type | Best For | Link |
|----------|------|----------|------|
| **Claude Prompt Caching** | API-level | Anthropic models, 90% cost reduction | https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching |
| **OpenAI Prompt Caching** | API-level | OpenAI models, automatic | https://openai.com/index/api-prompt-caching/ |
| **GPTCache** | Semantic | Any LLM, 70% hit rate | https://github.com/zilliztech/GPTCache |
| **Redis** | General | Fast in-memory | https://redis.io/ |

### 7.8 Recommended Rust Crates

```toml
[dependencies]
# Core LLM interaction
anthropic-sdk = "0.1"
openai-async = "0.3"

# Tree-sitter parsing
tree-sitter = "0.20"
tree-sitter-rust = "0.20"
tree-sitter-python = "0.20"

# Embeddings and vectors
qdrant-client = "1.7"
fastembed-rs = "0.8"  # Rust embeddings

# Graph analysis
petgraph = "0.6"

# Caching
redis = { version = "0.24", features = ["tokio-comp"] }
moka = "0.12"  # High-performance cache

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Utilities
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
```

---

## 8. References

### 8.1 Key Papers

**Prompt Compression:**
- LLMLingua (EMNLP 2023): https://arxiv.org/abs/2310.05736
- LongLLMLingua (ACL 2024): https://arxiv.org/abs/2310.06839
- LLMLingua-2 (ACL 2024): https://llmlingua.com/llmlingua2.html

**Context Management:**
- Git Context Controller (ACL 2024): https://arxiv.org/abs/2508.00031
- LoCoMo - Long-term Conversational Memory (ACL 2024): https://arxiv.org/abs/2402.17753
- Recursive Summarization (2024): https://arxiv.org/abs/2308.15022

**Code Understanding:**
- IDECoder (ICSE 2024): https://arxiv.org/abs/2402.03630
- CodexGraph (SIGKDD 2024): https://arxiv.org/abs/2408.03910
- CodePlan (ICLR 2024): https://huggingface.co/papers/2309.12499

**KV Cache Optimization:**
- Survey on KV Cache Management (2024): https://arxiv.org/abs/2412.19442
- SentenceKV (2024): https://arxiv.org/abs/2504.00970

**Semantic Caching:**
- GPT Semantic Cache (2024): https://arxiv.org/abs/2411.05276
- MeanCache (2024): https://arxiv.org/abs/2403.02694
- GPTCache: https://github.com/zilliztech/GPTCache

**Memory Systems:**
- Mem0 (2024): https://arxiv.org/abs/2504.19413
- A-Mem (2025): https://arxiv.org/abs/2502.12110

### 8.2 Production Systems

**Aider:**
- GitHub: https://github.com/Aider-AI/aider
- Repository Map: https://aider.chat/docs/repomap.html
- Tree-sitter Integration: https://aider.chat/2023/10/22/repomap.html

**Sourcegraph Cody:**
- Website: https://sourcegraph.com/cody
- Lifecycle Blog: https://sourcegraph.com/blog/the-lifecycle-of-a-code-ai-completion
- Research Paper: https://arxiv.org/abs/2408.05344

**Anthropic:**
- Prompt Caching: https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching
- Announcement: https://www.anthropic.com/news/prompt-caching

**OpenAI:**
- Prompt Caching: https://openai.com/index/api-prompt-caching/
- Documentation: https://platform.openai.com/docs/guides/prompt-caching

### 8.3 Benchmarks

**SWE-bench:**
- Website: https://www.swebench.com/
- GitHub: https://github.com/SWE-bench/SWE-bench
- OpenAI Verified: https://openai.com/index/introducing-swe-bench-verified/

**LongBench:**
- Evaluates long-context capabilities
- SDQA, MDQA, summarization tasks

### 8.4 Tutorials and Guides

**Tree-sitter:**
- Official Docs: https://tree-sitter.github.io/tree-sitter/
- Implementation Guide: https://tree-sitter.github.io/tree-sitter/5-implementation.html

**RAG Chunking:**
- Databricks Guide: https://community.databricks.com/t5/technical-blog/the-ultimate-guide-to-chunking-strategies-for-rag-applications/ba-p/113089
- 11 Chunking Strategies: https://masteringllm.medium.com/11-chunking-strategies-for-rag-simplified-visualized-df0dbec8e373

**Semantic Code Indexing:**
- AST + Tree-sitter: https://medium.com/@email2dineshkuppan/semantic-code-indexing-with-ast-and-tree-sitter-for-ai-agents-part-1-of-3-eb5237ba687a

**Knowledge Graphs:**
- FalkorDB Blog: https://www.falkordb.com/blog/
- Neo4j LLM Builder: https://neo4j.com/blog/developer/llm-knowledge-graph-builder/

---

## Summary of Key Metrics

| Technique | Reduction/Improvement | Quality Impact | Source |
|-----------|----------------------|----------------|---------|
| **LLMLingua** | 20x compression | -1.5% performance | Microsoft Research |
| **LongLLMLingua** | 4x compression | +21.4% performance | ACL 2024 |
| **Prompt Caching (Claude)** | 90% cost, 85% latency | No degradation | Anthropic |
| **Semantic Cache** | 68.8% API reduction | >97% accuracy | 2024 Research |
| **Repository Map** | Entire repo in 1K tokens | Full awareness | Aider |
| **AST Chunking** | Better retrieval | Higher precision | Multiple sources |
| **Conversation Summary** | Constant context size | Maintained quality | Best practice |
| **KV Cache Optimization** | 50% memory | No degradation | FastGen ICLR 2024 |
| **Production Agents** | 5-6 hrs/week saved | 2x faster coding | Industry reports |

---

## Conclusion

The state-of-the-art in LLM context optimization combines multiple techniques:

1. **Architectural:** Repository maps, dependency graphs, AST-based selection
2. **Compression:** LLMLingua for 4-20x reduction with <2% quality loss
3. **Caching:** API-level (90% cost reduction) + semantic (70% hit rate)
4. **Memory:** Tiered architecture with summarization and consolidation
5. **Production:** Proven in tools like Aider, Cody, Cursor

For a Rust implementation, prioritize:
- Tree-sitter for repository mapping and AST analysis
- Claude prompt caching for 90% cost reduction
- Semantic caching with Redis and nomic-embed-code
- Conversation summarization to bound growth
- Differential context to minimize redundancy

These techniques enable coding agents to work effectively within token limits while maintaining full codebase awareness, resulting in measurable productivity gains in production environments.
