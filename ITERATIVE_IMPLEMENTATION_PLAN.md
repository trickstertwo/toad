# TOAD: Iterative Implementation Plan
## Simple → SOTA → Edge → World-Class

**Philosophy:** Build incrementally, validate rigorously, keep only what works.

**Key Principle:** Every feature is a **hypothesis** that must be **tested** with **data** before proceeding.

---

## 🏗️ ARCHITECTURE: FEATURE FLAG SYSTEM

### **Core Design: Modular & Switchable**

```rust
pub struct ToadConfig {
    // Feature flags (easily toggle on/off)
    features: FeatureFlags,

    // Performance tracking
    metrics: MetricsCollector,

    // A/B testing
    experiments: ExperimentManager,
}

pub struct FeatureFlags {
    // Context strategies
    context_ast: bool,              // Default: true
    context_embeddings: bool,       // Default: false (test in Phase 2)
    context_graph: bool,            // Default: false (test in Phase 3)
    context_reranking: bool,        // Default: false (test in Phase 3)

    // Routing strategies
    routing_semantic: bool,         // Default: false (test in Phase 2)
    routing_multi_model: bool,      // Default: false (test in Phase 3)
    routing_speculative: bool,      // Default: false (test in Phase 4)

    // Intelligence
    smart_test_selection: bool,     // Default: false (test in Phase 2)
    tree_sitter_validation: bool,   // Default: true
    failure_memory: bool,           // Default: false (test in Phase 5)

    // Optimization
    prompt_caching: bool,           // Default: true
    semantic_caching: bool,         // Default: false (test in Phase 2)
}

impl ToadConfig {
    // Easy A/B testing
    pub fn test_feature(&mut self, feature: Feature, enabled: bool) {
        match feature {
            Feature::ContextEmbeddings => self.features.context_embeddings = enabled,
            Feature::MultiModel => self.features.routing_multi_model = enabled,
            // ... etc
        }
    }

    // Compare configurations
    pub async fn compare_configs(
        &self,
        config_a: FeatureFlags,
        config_b: FeatureFlags,
        test_set: &[Task],
    ) -> ComparisonResult {
        let results_a = self.run_with_config(config_a, test_set).await;
        let results_b = self.run_with_config(config_b, test_set).await;

        ComparisonResult::compare(results_a, results_b)
    }
}
```

---

## 📊 EVALUATION FRAMEWORK

### **Metrics to Track**

```rust
pub struct Metrics {
    // Performance
    pub accuracy: f64,              // % of problems solved correctly
    pub swe_bench_score: f64,       // SWE-bench Verified score

    // Cost
    pub cost_per_problem: f64,      // Average cost in USD
    pub cost_per_fix: f64,          // Cost / success_rate
    pub token_usage: TokenStats,    // Input/output/cached tokens

    // Latency
    pub time_to_first_token: Duration,
    pub total_time: Duration,
    pub context_retrieval_time: Duration,

    // Quality
    pub syntax_error_rate: f64,     // % of invalid code generated
    pub test_pass_rate: f64,        // % passing existing tests
    pub regression_rate: f64,       // % breaking existing tests

    // Efficiency
    pub tool_calls: usize,          // Number of tool invocations
    pub context_tokens: usize,      // Tokens used for context
    pub cache_hit_rate: f64,        // % cached responses
}

pub struct ComparisonResult {
    pub winner: Config,
    pub delta: MetricsDelta,
    pub statistical_significance: f64,
    pub recommendation: Decision,
}

pub enum Decision {
    Adopt,              // Clear improvement, adopt feature
    Reject,             // No improvement or worse, reject
    NeedsMoreData,      // Unclear, need more testing
    ContextDependent,   // Works for some cases, make optional
}
```

### **Test Datasets**

```rust
pub struct TestSuite {
    // SWE-bench subsets
    swe_bench_lite_sample: Vec<Task>,      // 50 tasks
    swe_bench_verified_sample: Vec<Task>,   // 100 tasks

    // Categorized by difficulty
    simple_tasks: Vec<Task>,        // 30 tasks (variable rename, etc.)
    medium_tasks: Vec<Task>,        // 40 tasks (bug fixes)
    complex_tasks: Vec<Task>,       // 30 tasks (architecture changes)

    // Categorized by type
    bug_fix: Vec<Task>,
    refactoring: Vec<Task>,
    feature_addition: Vec<Task>,
    test_writing: Vec<Task>,
}
```

---

## 🎯 MILESTONE PROGRESSION

---

## **MILESTONE 0: Infrastructure (Week 1)**

### **Goal:** Set up evaluation framework

**Deliverables:**
- [ ] Test harness for SWE-bench evaluation
- [ ] Metrics collection system
- [ ] Feature flag architecture
- [ ] A/B testing framework
- [ ] Baseline dataset (50 SWE-bench Lite tasks)

**Quality Gate:**
- [ ] Can run tests and collect metrics
- [ ] Feature flags work (can toggle features on/off)
- [ ] Baseline measurement system validated

**No accuracy target, just infrastructure.**

---

## **MILESTONE 1: SIMPLE BASELINE (Weeks 2-4)**

### **Goal:** Functional agent with proven basics

**Hypothesis:** "Single-agent with basic tools can achieve 55-60% on our test set"

### **Architecture:**

```rust
pub struct SimpleAgent {
    model: ClaudeSonnet4,           // Single model
    tools: Vec<Tool>,               // 10-15 basic tools
    context: BasicContext,          // No fancy retrieval
    cache: PromptCache,             // Anthropic caching
}

impl SimpleAgent {
    pub fn tools() -> Vec<Tool> {
        vec![
            ReadFile,
            WriteFile,
            EditFile,           // Search/replace only
            Bash,
            GrepSearch,
            ListFiles,
            GitDiff,
            GitStatus,
            RunTests,
        ]
    }
}

pub struct BasicContext {
    // Simple approach: include relevant files mentioned in task
    pub files: Vec<PathBuf>,
    pub max_tokens: usize,  // 20K limit
}
```

### **Features Enabled:**
- ✅ Single agent (Claude Sonnet 4)
- ✅ 10 basic tools
- ✅ Search/replace edit format
- ✅ Prompt caching
- ✅ Tree-sitter syntax validation
- ❌ Everything else OFF

### **Validation Protocol:**

**Test Set:** 50 SWE-bench Lite tasks

**Success Criteria:**
- [ ] **Accuracy:** ≥55% (mini-SWE achieves 65% full, expect lower on limited tools)
- [ ] **Cost:** <$2.00/problem
- [ ] **Syntax errors:** <10%
- [ ] **Tool calls:** <30/problem
- [ ] **No crashes:** 100% completion rate

**If FAIL (Accuracy <50%):**
- ❌ **STOP:** Debug core agent loop, fix tools, retry
- Root cause analysis before proceeding

**If PASS:**
- ✅ **PROCEED** to Milestone 2
- Baseline established: This is our control group

### **Expected Results:**
- Accuracy: 55-60%
- Cost: $1.00-1.50/problem
- Timeline: 3 weeks

---

## **MILESTONE 2: INTELLIGENT CONTEXT (Weeks 5-7)**

### **Goal:** Add smart context management

**Hypothesis:** "AST-based repo map improves accuracy by 3-5 points"

### **Features to Test:**

**2A: AST Repository Map**

```rust
pub struct ASTContext {
    repo_map: RepositoryMap,        // Tree-sitter based
    dependency_graph: DependencyGraph,
    token_budget: usize,            // 1K for map, 19K for files
}

impl ASTContext {
    pub fn gather_context(&self, task: &Task) -> Context {
        // 1. Get repo map (1K tokens)
        let map = self.repo_map.render();

        // 2. Identify relevant files from map
        let relevant = self.identify_relevant_files(task, &map);

        // 3. Load relevant files (19K tokens)
        let files = self.load_files(relevant);

        Context::new(map, files)
    }
}
```

**Test Protocol:**

```
Configuration A (Control): BasicContext
Configuration B (Test):    ASTContext

Test on: 50 SWE-bench Lite tasks

Measure:
  - Accuracy delta
  - Cost delta
  - Context token usage
  - Time to gather context
```

**Success Criteria:**
- [ ] **Accuracy:** +3 to +5 points (58-65%)
- [ ] **Cost:** <10% increase (acceptable for 3-5 point gain)
- [ ] **Context tokens:** <25K (fits in budget)
- [ ] **Latency:** <500ms for context retrieval

**Decision Tree:**

```
IF accuracy gain ≥ +3 points AND cost increase < 20%:
    ✅ ADOPT ASTContext permanently

ELIF accuracy gain = +2 points AND cost increase < 10%:
    ⚠️ CONDITIONAL: Keep but monitor

ELSE:
    ❌ REJECT: Keep BasicContext
```

**Expected:** +4 points gain (research shows +2.7-5.5), ADOPT

---

**2B: Smart Test Selection**

```rust
pub struct SmartTestRunner {
    coverage_db: CoverageDatabase,
    dependency_graph: Arc<DependencyGraph>,
}

impl SmartTestRunner {
    pub fn select_tests(&self, changed_files: &[PathBuf]) -> Vec<TestCase> {
        // Only run tests that cover changed code
        let relevant = self.coverage_db.find_covering_tests(changed_files);
        relevant
    }
}
```

**Test Protocol:**

```
Configuration A: Run all tests
Configuration B: Smart test selection

Measure:
  - Accuracy (should be same or better)
  - Test execution time
  - False negatives (missed failing tests)
```

**Success Criteria:**
- [ ] **Accuracy:** ≥ Configuration A (no regression)
- [ ] **Test time:** 50% reduction
- [ ] **False negatives:** <5%

**Decision Tree:**

```
IF accuracy >= control AND test_time < 50%:
    ✅ ADOPT SmartTestRunner

ELIF accuracy >= control AND test_time < 75%:
    ⚠️ ADOPT but optimize further

ELSE:
    ❌ REJECT: Too risky, run all tests
```

**Expected:** ADOPT (AutoCodeRover proves this works)

---

### **Milestone 2 Quality Gate:**

**Combined Performance Target:**

```
Baseline (M1):              55-60%
+ AST Context:              +4%
+ Smart Tests:              +2%
                           -----
Expected M2:                61-66%

Cost target: <$1.50/problem
```

**Gate Criteria:**
- [ ] Accuracy ≥62%
- [ ] Both features show positive delta
- [ ] No major regressions (syntax errors, crashes)

**If FAIL:**
- Debug features individually
- Consider partial adoption (keep what works)

**If PASS:**
- ✅ PROCEED to Milestone 3

---

## **MILESTONE 3: SOTA PERFORMANCE (Weeks 8-10)**

### **Goal:** Match or beat current SOTA (75.2%)

**Hypothesis:** "Multi-model ensemble provides +4-5 points"

### **Architecture:**

```rust
pub struct MultiModelAgent {
    models: Vec<ModelConfig>,
    selector: ModelSelector,
    ensembler: EnsembleStrategy,
}

pub enum EnsembleStrategy {
    VoteAll,           // All models vote on solution
    Best3,             // Best 3 models
    SmartRouting,      // Only ensemble on hard problems
}

pub struct ModelConfig {
    model: Model,
    strength: Domain,  // What this model is good at
    cost: f64,
}

// Example configuration
pub fn sota_ensemble() -> Vec<ModelConfig> {
    vec![
        ModelConfig {
            model: Model::ClaudeSonnet4,
            strength: Domain::General,
            cost: 0.91,
        },
        ModelConfig {
            model: Model::ClaudeOpus4,
            strength: Domain::Reasoning,
            cost: 1.50,
        },
        ModelConfig {
            model: Model::Claude37,
            strength: Domain::Code,
            cost: 0.60,
        },
    ]
}
```

### **Test Configurations:**

**3A: Single Model (Control)**
```rust
Config {
    model: ClaudeSonnet4,
    ensemble: false,
}
```

**3B: Best-of-3 Ensemble**
```rust
Config {
    models: [ClaudeSonnet4, ClaudeOpus4, Claude37],
    strategy: EnsembleStrategy::VoteAll,
}
```

**3C: Smart Ensemble (20% Hard Problems)**
```rust
Config {
    default: ClaudeSonnet4,
    ensemble_on: HardProblems,  // Top 20% complexity
    models: [ClaudeSonnet4, ClaudeOpus4, Claude37],
}
```

### **Test Protocol:**

```
Test Set: 100 SWE-bench Verified tasks

For each configuration:
  1. Run all 100 tasks
  2. Measure accuracy, cost, time
  3. Categorize by difficulty (simple/medium/hard)
  4. Analyze where ensemble helps most

Statistical analysis:
  - Paired t-test for significance
  - Per-category breakdown
  - Cost-benefit analysis
```

### **Success Criteria:**

**3B (Best-of-3) vs 3A (Control):**
- [ ] **Accuracy:** +4 to +5 points
- [ ] **Cost:** <3x (acceptable if accuracy justifies)
- [ ] **Statistical significance:** p < 0.05

**3C (Smart Ensemble) vs 3A (Control):**
- [ ] **Accuracy:** +3 to +4 points (slightly less than 3B)
- [ ] **Cost:** <1.5x (much cheaper than 3B)
- [ ] **Best cost/performance ratio**

### **Decision Tree:**

```
IF 3C gives +3 points at <1.5x cost:
    ✅ ADOPT Smart Ensemble (best value)

ELIF 3B gives +5 points AND we want max performance:
    ✅ ADOPT Best-of-3 Ensemble (SOTA focus)

ELIF improvement < +2 points:
    ❌ REJECT: Not worth cost

ELSE:
    ⚠️ ANALYZE: Check which problems benefit from ensemble
        Consider per-category ensemble (only for hard problems)
```

### **Expected Results:**

Based on TRAE data (75.2% vs Warp 71% = +4.2 points):

```
Configuration 3A (Control):     66%    $0.91/problem
Configuration 3B (Best-of-3):   71%    $3.01/problem   ← TRAE-style
Configuration 3C (Smart 20%):   69%    $1.53/problem   ← Best value

Recommendation: 3C (Smart Ensemble)
  - 69% accuracy (beats TRAE 75.2%? No, but close)
  - $1.53/problem (50% of 3B cost)
  - Best cost/performance
```

### **Milestone 3 Quality Gate:**

**Target: 70-75% SWE-bench Verified**

**Gate Criteria:**
- [ ] Accuracy ≥70%
- [ ] Cost <$2.00/problem
- [ ] Multi-model shows clear benefit (+3 points minimum)
- [ ] Production-ready quality (no crashes, <5% syntax errors)

**If FAIL:**
- Analyze failure modes
- Consider model selection (maybe different models?)
- Check if baseline is the issue

**If PASS:**
- ✅ **YOU NOW HAVE A SOTA-COMPETITIVE SYSTEM**
- Can ship as premium product
- PROCEED to Edge features (optional)

---

## **MILESTONE 4: EDGE FEATURES (Weeks 11-13)**

### **Goal:** Test novel features for extra 1-3 points

**These are SPECULATIVE. Test rigorously, discard if they don't work.**

---

### **4A: Hybrid Context (AST + Embeddings + Graph)**

**Current:** AST-only context
**Hypothesis:** "Adding embeddings + graph improves retrieval by 1-2 points"

```rust
pub enum ContextStrategy {
    ASTOnly,                    // Current (Milestone 2)
    ASTAndEmbeddings,          // Add semantic search
    ASTAndGraph,               // Add dependency graph
    FullHybrid,                // All three + re-ranking
}

pub struct HybridContext {
    ast: ASTIndexer,
    embeddings: SemanticIndex,
    graph: KnowledgeGraph,
    reranker: CrossEncoderReranker,
    strategy: ContextStrategy,
}

impl HybridContext {
    pub fn gather_context(&self, task: &Task) -> Context {
        // Parallel retrieval
        let (ast_results, sem_results, graph_results) = tokio::join!(
            self.ast.search(task),
            self.embeddings.search(task),
            self.graph.query(task),
        );

        // Merge and re-rank
        let merged = self.merge(ast_results, sem_results, graph_results);
        let ranked = self.reranker.rank(task, merged);

        Context::from_ranked(ranked)
    }
}
```

**Test Configurations:**

```
4A-1: ASTOnly (control)
4A-2: ASTAndEmbeddings
4A-3: ASTAndGraph
4A-4: FullHybrid

Test Set: 50 SWE-bench Verified tasks
Focus: Tasks requiring cross-file understanding
```

**Metrics to Track:**

```rust
pub struct ContextMetrics {
    // Quality
    pub retrieval_precision_at_10: f64,
    pub retrieval_recall_at_10: f64,
    pub context_relevance: f64,         // LLM judges relevance

    // Performance
    pub accuracy_delta: f64,

    // Cost
    pub latency: Duration,
    pub cost_per_retrieval: f64,
    pub token_usage: usize,
}
```

**Success Criteria:**

```
IF FullHybrid shows:
    +2 points accuracy AND
    <500ms latency AND
    precision@10 > ASTOnly
THEN ✅ ADOPT

ELIF ASTAndEmbeddings shows:
    +1 point accuracy AND
    <300ms latency
THEN ✅ ADOPT (simpler than full hybrid)

ELSE ❌ REJECT (keep ASTOnly)
```

**Expected:**
- Small gain (+1-2 points) at added complexity
- Decision: Likely ADOPT embeddings, maybe skip graph
- Cost: Development time vs 1-2 point gain

---

### **4B: Speculative Execution**

**Hypothesis:** "Running fast model in parallel saves time and possibly money"

```rust
pub struct SpeculativeRouter {
    fast_model: Model,      // Haiku or GPT-4o-mini
    premium_model: Model,   // Sonnet 4
    quality_threshold: f64,
}

impl SpeculativeRouter {
    pub async fn route(&self, task: &Task) -> Response {
        // Start fast model immediately
        let fast_future = self.fast_model.complete(task);

        // Wait 200ms to see if fast model finishes
        tokio::time::sleep(Duration::from_millis(200)).await;

        if let Some(fast_result) = fast_future.try_get() {
            // Fast model finished, check quality
            if self.is_high_quality(&fast_result) {
                return fast_result;  // Use fast result
            }
        }

        // Use premium model
        self.premium_model.complete(task).await
    }

    fn is_high_quality(&self, result: &Response) -> bool {
        // Heuristics:
        result.syntax_valid() &&
        result.confidence > self.quality_threshold &&
        result.len() > 50
    }
}
```

**Test Configurations:**

```
4B-1: Sequential routing (control)
4B-2: Speculative execution

Measure for each task:
  - Time to first useful token
  - Total cost (both models counted)
  - Quality (syntax errors, test pass rate)
  - User experience (perceived latency)
```

**Metrics:**

```rust
pub struct SpeculativeMetrics {
    // Latency
    pub time_to_first_token: Duration,
    pub fast_model_win_rate: f64,  // % times fast model used

    // Cost
    pub total_cost: f64,
    pub cost_vs_sequential: f64,   // Ratio

    // Quality
    pub accuracy_delta: f64,       // Should be ≥0
    pub fast_model_quality: f64,   // Quality when fast model used
}
```

**Success Criteria:**

```
IF speculative shows:
    time_to_first_token < 50% of sequential AND
    total_cost < 110% of sequential AND
    accuracy >= sequential
THEN ✅ ADOPT (faster AND not too expensive)

ELIF cost_savings > 20% AND latency_improvement > 30%:
    ✅ ADOPT (cost AND speed win)

ELSE ❌ REJECT (not worth complexity)
```

**Expected:**
- Possible 30-50% latency improvement
- Cost: Depends on fast model win rate (could save 24%)
- Decision: LIKELY ADOPT if fast model catches 70%+ of queries

---

### **4C: Opportunistic Planning**

**Hypothesis:** "Fast planning + execute + refine beats sequential"

```rust
pub struct OpportunisticPlanner {
    fast_model: Model,      // Quick plan (100ms)
    premium_model: Model,   // Refined plan (parallel)
}

impl OpportunisticPlanner {
    pub async fn plan_and_execute(&self, task: &Task) -> Solution {
        // Start fast plan immediately
        let fast_plan = self.fast_model.plan(task).await;

        // Start executing fast plan
        let execution = tokio::spawn(self.execute(fast_plan.clone()));

        // In parallel, get premium plan
        let premium_plan = self.premium_model.plan(task).await;

        // Check if premium plan differs significantly
        if self.plans_differ_significantly(&fast_plan, &premium_plan) {
            // Merge insights from premium into execution
            self.adjust_execution(execution, premium_plan).await
        } else {
            // Fast plan was good, continue
            execution.await
        }
    }
}
```

**Test Configurations:**

```
4C-1: Sequential planning (control)
4C-2: Opportunistic planning

Test Set: 30 complex tasks (require multi-step planning)
```

**Metrics:**

```rust
pub struct PlanningMetrics {
    pub time_to_solution: Duration,
    pub wasted_work: f64,          // % work rolled back
    pub plan_quality: f64,         // How often premium plan differs
    pub accuracy_delta: f64,
}
```

**Success Criteria:**

```
IF opportunistic shows:
    time_to_solution < 80% of sequential AND
    wasted_work < 20% AND
    accuracy >= sequential
THEN ✅ ADOPT

ELSE ❌ REJECT (too complex, risky)
```

**Expected:**
- Uncertain benefit (planning might not help - see Agentless)
- High risk (execution based on potentially bad plan)
- Decision: LIKELY REJECT unless clear time savings

---

### **Milestone 4 Quality Gate:**

**Goal: 72-76% with edge features**

**Gate Criteria:**
- [ ] At least 1 of 3 features shows positive benefit
- [ ] Overall accuracy ≥72%
- [ ] No features make things worse
- [ ] Production stability maintained

**Decision Matrix:**

```
Features Adopted     Expected Accuracy    Decision
-----------------    -----------------    --------
None (all rejected)       70%             Still competitive, ship it
1 feature works          71-72%           Minor gain, adopt
2 features work          73-74%           Good gain, adopt both
All 3 work              75-76%            Excellent, adopt all
```

---

## **MILESTONE 5: WORLD-CLASS (Weeks 14-16)**

### **Goal:** Push beyond SOTA (76%+)

**These are HIGH-RISK experiments. Only attempt if M4 succeeds.**

---

### **5A: Failure Memory**

**Hypothesis:** "Learning from past failures avoids repeat mistakes"

**Phase 1: Data Collection (Weeks 14)**

```rust
pub struct FailureMemory {
    storage: RedisClient,
    embeddings: EmbeddingModel,
}

pub struct FailureCase {
    task_embedding: Vec<f32>,
    error_type: ErrorType,
    attempted_solution: String,
    successful_recovery: Option<Recovery>,
    similar_tasks: Vec<TaskId>,
}

impl FailureMemory {
    pub async fn record_failure(&self, task: &Task, error: Error, recovery: Option<Recovery>) {
        let embedding = self.embeddings.embed(task).await;

        let failure = FailureCase {
            task_embedding: embedding,
            error_type: error.classify(),
            attempted_solution: error.context(),
            successful_recovery: recovery,
            similar_tasks: vec![],
        };

        self.storage.store(failure).await;
    }

    pub async fn check_similar_failures(&self, task: &Task) -> Option<FailureCase> {
        let embedding = self.embeddings.embed(task).await;

        // Find similar past failures
        let similar = self.storage.search_similar(&embedding, threshold=0.85).await;

        similar.first()
    }
}
```

**Test Protocol:**

```
Week 14: Collect data
  - Run 100 tasks
  - Record all failures (expect 25-30)
  - Record successful recoveries

Week 15: Replay similar tasks
  - Find 30 tasks similar to recorded failures
  - Run WITHOUT failure memory (control)
  - Run WITH failure memory (test)

Week 16: Analyze
  - Measure improvement on similar tasks
  - Measure if it helps on dissimilar tasks (generalization)
```

**Success Criteria:**

```
IF failure memory shows:
    +10% success rate on similar tasks AND
    No regression on dissimilar tasks
THEN ✅ ADOPT

ELIF +5% on similar AND no regression:
    ⚠️ CONDITIONAL: Keep for specific error types

ELSE ❌ REJECT (not enough data or benefit)
```

**Expected:**
- Needs 100+ failures to be useful
- Benefit likely small (+1-2 points best case)
- Decision: LIKELY REJECT (insufficient data for meaningful learning)

---

### **5B: Advanced Tool Composition**

**Hypothesis:** "Composable tools reduce round-trips"

```rust
pub struct ComposableTool {
    name: String,
    components: Vec<AtomicTool>,
}

// Example: Multi-file batch edit
pub struct BatchEditTool;

impl Tool for BatchEditTool {
    async fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let edits: Vec<Edit> = params.get("edits")?;

        // Validate all edits first (atomic)
        for edit in &edits {
            self.validate(edit)?;
        }

        // Apply all edits
        for edit in edits {
            self.apply(edit).await?;
        }

        // Run tests once at the end
        self.run_tests().await
    }
}
```

**Test Protocol:**

```
Compare:
  A: Individual tool calls (current)
  B: Batch operations

Measure:
  - Round-trips (fewer = better)
  - Total tokens (fewer = better)
  - Accuracy (should be same)
  - Latency (should be lower)
```

**Success Criteria:**

```
IF batch tools show:
    Round-trips reduced by 30%+ AND
    Accuracy >= control
THEN ✅ ADOPT

ELSE ❌ REJECT
```

**Expected:**
- Warp uses this (edit_files tool)
- Proven to help
- Decision: LIKELY ADOPT

---

### **5C: Meta-Learning (Model Selection)**

**Hypothesis:** "Learn which model is best for each task type"

```rust
pub struct MetaLearner {
    history: TaskHistory,
    model_performance: HashMap<(TaskType, Model), f64>,
}

impl MetaLearner {
    pub async fn select_model(&self, task: &Task) -> Model {
        let task_type = self.classify(task);

        // Look up best model for this task type
        let best = self.model_performance
            .iter()
            .filter(|((t, _), _)| t == &task_type)
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|((_, m), _)| m);

        best.unwrap_or(Model::ClaudeSonnet4)
    }

    pub async fn learn(&mut self, task: &Task, model: Model, success: bool) {
        let task_type = self.classify(task);
        let key = (task_type, model);

        // Update running average
        self.model_performance
            .entry(key)
            .and_modify(|avg| *avg = 0.9 * *avg + 0.1 * (success as u8 as f64))
            .or_insert(0.5);
    }
}
```

**Test Protocol:**

```
Week 14-15: Collect data
  - Run 200 tasks with various models
  - Track success by task type

Week 16: Test learned policy
  - Use meta-learner to select models
  - Compare to fixed routing
```

**Success Criteria:**

```
IF meta-learning shows:
    +2% accuracy over fixed routing AND
    Generalizes to new tasks
THEN ✅ ADOPT

ELSE ❌ REJECT (overfitting or insufficient data)
```

**Expected:**
- Needs data
- Small benefit (+1-2 points)
- Decision: CONDITIONAL (depends on data quality)

---

### **Milestone 5 Quality Gate:**

**Goal: 76-78% (World-class)**

**Gate Criteria:**
- [ ] Accuracy ≥76%
- [ ] Cost <$2.00/problem
- [ ] At least 2 world-class features work
- [ ] System is stable and production-ready

**Final Decision:**

```
IF we reach 76%+:
    ✅ WORLD-CLASS SYSTEM
    - Beats TRAE (75.2%)
    - Competitive with best (Warp+GPT-5 75.8%)
    - Potentially #1 open-source system

ELIF we reach 74-76%:
    ✅ EXCELLENT SYSTEM
    - Matches SOTA
    - Better cost-efficiency
    - Unique features (TUI, etc.)

ELIF we reach 70-74%:
    ✅ COMPETITIVE SYSTEM
    - Better than most
    - Good cost-efficiency
    - Ship as premium product

ELSE (<70%):
    ⚠️ ANALYZE: What went wrong?
    - Debug underperforming components
    - Revisit assumptions
```

---

## 📊 FULL PROGRESSION SUMMARY

### **Expected Performance Trajectory**

```
Milestone    Features                           Accuracy    Cost/Problem    Timeline
---------    --------                           --------    ------------    --------
M0           Infrastructure                     N/A         N/A             Week 1
M1           Simple baseline                    55-60%      $1.00-1.50      Week 4
M2           + AST + Smart tests               61-66%      $0.91-1.20      Week 7
M3           + Multi-model ensemble            70-75%      $1.53-2.00      Week 10
M4           + Edge features (1-2 work)        72-76%      $1.00-1.80      Week 13
M5           + World-class features            76-78%      $1.20-2.00      Week 16

SHIP DECISION POINTS:
  - After M2 (66%): Competitive with basic Copilot
  - After M3 (70-75%): SOTA-competitive, recommended ship point
  - After M4 (72-76%): Enhanced with novel features
  - After M5 (76-78%): World-class, beat SOTA
```

### **Cost Progression**

```
Milestone    Optimization                       Cost Reduction    Cumulative
---------    ------------                       --------------    ----------
M1           Prompt caching                     -83%              $0.91
M2           Semantic caching                   -20%              $0.73
M3           Smart ensemble (not all queries)  +100%             $1.53
M4           Speculative execution              -24%              $1.16
M5           Meta-learning                      -10%              $1.04

Best case: $1.04/problem at 76-78% accuracy
Worst case: $1.53/problem at 70-75% accuracy

Both are excellent cost/performance ratios.
```

---

## 🧪 A/B TESTING EXAMPLES

### **Example 1: AST Context vs Basic Context**

```rust
#[tokio::test]
async fn test_ast_context_improvement() {
    let test_set = load_swe_bench_sample(50);

    // Control group
    let config_a = Config {
        context: ContextStrategy::Basic,
        ..Default::default()
    };

    // Test group
    let config_b = Config {
        context: ContextStrategy::AST,
        ..Default::default()
    };

    let results_a = run_evaluation(config_a, &test_set).await;
    let results_b = run_evaluation(config_b, &test_set).await;

    let comparison = ComparisonResult::compare(results_a, results_b);

    println!("Accuracy delta: {:.1}%", comparison.delta.accuracy * 100.0);
    println!("Cost delta: ${:.2}", comparison.delta.cost);
    println!("Recommendation: {:?}", comparison.recommendation);

    // Statistical significance
    assert!(comparison.statistical_significance > 0.95);

    // Should show improvement
    assert!(comparison.delta.accuracy > 0.03, "Expected +3% minimum");
}
```

### **Example 2: Multi-Model Configurations**

```rust
#[tokio::test]
async fn test_multi_model_strategies() {
    let test_set = load_swe_bench_verified(100);

    let strategies = vec![
        ("Single Model", EnsembleStrategy::None),
        ("Best-of-3", EnsembleStrategy::VoteAll),
        ("Smart Ensemble 20%", EnsembleStrategy::SmartRouting { threshold: 0.8 }),
        ("Smart Ensemble 30%", EnsembleStrategy::SmartRouting { threshold: 0.7 }),
    ];

    let mut results = Vec::new();

    for (name, strategy) in strategies {
        let config = Config {
            ensemble: strategy,
            ..Default::default()
        };

        let result = run_evaluation(config, &test_set).await;
        results.push((name, result));
    }

    // Analyze trade-offs
    for (name, result) in &results {
        println!("{}: {:.1}% accuracy, ${:.2} cost, {:.1}s time",
            name,
            result.accuracy * 100.0,
            result.cost_per_problem,
            result.avg_time.as_secs_f64()
        );
    }

    // Find best cost/performance ratio
    let best = results.iter()
        .max_by(|a, b| {
            let ratio_a = a.1.accuracy / a.1.cost_per_problem;
            let ratio_b = b.1.accuracy / b.1.cost_per_problem;
            ratio_a.partial_cmp(&ratio_b).unwrap()
        })
        .unwrap();

    println!("Best value: {}", best.0);
}
```

### **Example 3: Feature Ablation Study**

```rust
#[tokio::test]
async fn test_feature_ablation() {
    let test_set = load_swe_bench_sample(50);

    // Test removing each feature to measure its contribution
    let configs = vec![
        ("All features", FeatureFlags::all_enabled()),
        ("No AST", FeatureFlags::all_enabled().disable(Feature::AST)),
        ("No Smart Tests", FeatureFlags::all_enabled().disable(Feature::SmartTests)),
        ("No Caching", FeatureFlags::all_enabled().disable(Feature::PromptCaching)),
        ("No Multi-Model", FeatureFlags::all_enabled().disable(Feature::MultiModel)),
    ];

    let baseline = run_evaluation(configs[0].1, &test_set).await;

    for (name, config) in configs.iter().skip(1) {
        let result = run_evaluation(*config, &test_set).await;

        let accuracy_drop = (baseline.accuracy - result.accuracy) * 100.0;

        println!("{}: -{:.1}% accuracy (feature contributes +{:.1}%)",
            name, accuracy_drop, accuracy_drop);

        // This tells us which features are most valuable
    }
}
```

---

## 🎯 DECISION FRAMEWORK

### **When to ADOPT a Feature:**

```
✅ ADOPT if:
  1. Accuracy improvement ≥ +2% AND cost increase < 20%
  OR
  2. Cost reduction ≥ 20% AND accuracy maintained
  OR
  3. Latency improvement ≥ 30% AND cost/accuracy acceptable

Statistical significance required: p < 0.05
```

### **When to REJECT a Feature:**

```
❌ REJECT if:
  1. No measurable improvement (< +1%)
  OR
  2. Cost increase > 30% for < +2% accuracy
  OR
  3. Adds complexity without clear benefit
  OR
  4. Introduces regressions (crashes, errors)
```

### **When to ITERATE:**

```
⚠️ ITERATE if:
  1. Small improvement (+1-2%) but high variance
  OR
  2. Works well on some task types, poorly on others
  OR
  3. p-value between 0.05 and 0.20 (marginal significance)

  Action: Test with more data or different configurations
```

---

## 📈 SUCCESS METRICS BY MILESTONE

### **Milestone 1 (Simple Baseline)**
- ✅ Functional system
- ✅ 55-60% accuracy
- ✅ <$2/problem
- ✅ No crashes

### **Milestone 2 (Intelligent Context)**
- ✅ 61-66% accuracy (+5-10% from M1)
- ✅ <$1.50/problem
- ✅ Both features show positive delta

### **Milestone 3 (SOTA)**
- ✅ 70-75% accuracy
- ✅ <$2/problem
- ✅ **SHIP-READY**: Competitive product

### **Milestone 4 (Edge)**
- ✅ 72-76% accuracy
- ✅ At least 1 novel feature works
- ✅ **EXCELLENT**: Beat most competitors

### **Milestone 5 (World-Class)**
- ✅ 76-78% accuracy
- ✅ <$2/problem
- ✅ **WORLD-CLASS**: Beat TRAE (75.2%)

---

## 🚀 RECOMMENDATIONS

### **Minimum Viable Product: Ship After M3**

**Why:**
- 70-75% is SOTA-competitive
- Well-tested, proven components
- Good cost-efficiency
- 10 weeks to market

### **Recommended: Ship After M4**

**Why:**
- 72-76% beats most systems
- Novel features validated
- Unique selling points
- 13 weeks, still fast

### **Ambitious: Target M5**

**Why:**
- 76-78% world-class
- Research-grade system
- Potential to be #1 open-source
- 16 weeks, acceptable timeline

---

## 📝 IMPLEMENTATION CHECKLIST

### **Week 1: Infrastructure**
- [ ] Set up SWE-bench evaluation harness
- [ ] Build metrics collection system
- [ ] Implement feature flag architecture
- [ ] Create A/B testing framework
- [ ] Prepare 50-task test set

### **Weeks 2-4: M1 Simple Baseline**
- [ ] Implement basic agent loop
- [ ] Add 10 core tools
- [ ] Integrate Claude Sonnet 4
- [ ] Add prompt caching
- [ ] Add tree-sitter validation
- [ ] **TEST:** Validate 55-60% accuracy
- [ ] **GATE:** Pass M1 quality gate

### **Weeks 5-7: M2 Intelligent Context**
- [ ] Build AST repository mapper
- [ ] Implement smart test selection
- [ ] **A/B TEST:** AST vs Basic context
- [ ] **A/B TEST:** Smart vs All tests
- [ ] Analyze results, adopt winners
- [ ] **GATE:** Pass M2 quality gate (61-66%)

### **Weeks 8-10: M3 SOTA**
- [ ] Implement multi-model ensemble
- [ ] Test 3 configurations (single, best-of-3, smart)
- [ ] Measure accuracy vs cost tradeoff
- [ ] Adopt best configuration
- [ ] **GATE:** Pass M3 quality gate (70-75%)
- [ ] **DECISION:** Ship or continue?

### **Weeks 11-13: M4 Edge (Optional)**
- [ ] Test hybrid context (AST+embeddings+graph)
- [ ] Test speculative execution
- [ ] Test opportunistic planning
- [ ] Adopt features that show ≥+1% benefit
- [ ] **GATE:** Pass M4 quality gate (72-76%)

### **Weeks 14-16: M5 World-Class (Optional)**
- [ ] Collect failure data
- [ ] Test failure memory
- [ ] Test advanced tool composition
- [ ] Test meta-learning
- [ ] **GATE:** Pass M5 quality gate (76-78%)
- [ ] **DECISION:** Ship world-class system

---

## 🎓 KEY PRINCIPLES

1. **Measure Everything:** No feature without data
2. **Test Incrementally:** One feature at a time
3. **Have Controls:** Always compare to baseline
4. **Statistical Rigor:** p < 0.05 for significance
5. **Cost Awareness:** Track ROI for each feature
6. **Ship Early:** Don't wait for perfection (M3 is ship-ready)
7. **Reject Fearlessly:** If it doesn't work, cut it
8. **Iterate Fast:** 1-3 week cycles per feature

---

**This plan balances ambition with pragmatism. Ship at M3, enhance at M4, dominate at M5.**

**Your turn: Which milestone are you targeting?**
