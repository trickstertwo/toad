# Evaluation Center - Comprehensive Design Document

## Vision

Transform the Evaluation Center from a simple progress viewer into a **sophisticated analytics and decision-support hub** for evidence-based AI agent development.

## Core Purpose

Help developers answer critical questions:
- Which features should I enable to maximize accuracy?
- What's the cost/benefit tradeoff of each feature?
- Which configuration performs best for my use case?
- How do I design experiments to test hypotheses?
- What evidence do I have that a feature works?

---

## Architecture Overview

### 1. Data Model

#### EvaluationRun (stored in results/)
```rust
struct EvaluationRun {
    id: Uuid,
    timestamp: DateTime<Utc>,
    config: ToadConfig,           // Feature flags snapshot
    dataset: DatasetInfo,          // Which SWE-bench variant, task count
    results: Vec<TaskResult>,      // Per-task outcomes
    aggregate_metrics: AggregateMetrics,
    statistical_data: StatisticalData,
    metadata: RunMetadata,         // Git commit, environment, notes
}

struct AggregateMetrics {
    accuracy: f64,                 // % tasks solved
    mean_cost: f64,                // Average USD per task
    median_cost: f64,
    p95_latency: u64,              // 95th percentile latency (ms)
    total_tokens: u64,
    cache_hit_rate: f64,           // % tokens from cache
    mean_steps: f64,               // Average agent steps per task
    tool_use_efficiency: f64,      // % tool calls that succeeded
}

struct StatisticalData {
    sample_size: usize,
    std_dev_accuracy: f64,
    confidence_interval_95: (f64, f64),
    cost_variance: f64,
}
```

#### ComparisonAnalysis
```rust
struct ComparisonAnalysis {
    baseline: EvaluationRun,
    experiment: EvaluationRun,
    statistical_test: WelchTTest,  // Already implemented!
    effect_size: CohenD,           // Already implemented!
    recommendation: Recommendation,
    cost_benefit: CostBenefitAnalysis,
}

struct CostBenefitAnalysis {
    accuracy_delta: f64,           // +5.2% accuracy improvement
    cost_delta: f64,               // +$0.30 per task cost increase
    cost_per_accuracy_point: f64,  // $0.30 / 5.2 = $0.058 per %
    roi_score: f64,                // Higher is better
}

enum Recommendation {
    Adopt { confidence: f64, rationale: String },
    Reject { confidence: f64, rationale: String },
    Inconclusive { reason: String, suggestion: String },
}
```

#### FeatureImpact (derived from multiple runs)
```rust
struct FeatureImpact {
    feature_name: String,
    enabled_runs: Vec<Uuid>,
    disabled_runs: Vec<Uuid>,

    // Impact metrics (comparing enabled vs disabled)
    accuracy_impact: f64,          // +5.2% when enabled
    cost_impact: f64,              // +$0.30 when enabled
    latency_impact: i64,           // -500ms when enabled (negative is better)

    statistical_significance: f64, // p-value
    effect_size: f64,              // Cohen's d
    confidence: f64,               // How confident are we? (based on sample size)

    evidence_quality: EvidenceQuality,
}

enum EvidenceQuality {
    Strong,      // p < 0.01, large effect size, n > 30
    Moderate,    // p < 0.05, medium effect size, n > 10
    Weak,        // p < 0.1, small effect size, n < 10
    Insufficient // Not enough data
}
```

---

## 2. UI Architecture (TUI Screens)

### Screen Hierarchy

```
Evaluation Center (F9)
├─ Dashboard (default)
├─ Configure
├─ History
├─ Compare
├─ Charts
└─ Live (during evaluation)
```

### 2.1 Dashboard Screen

**Purpose:** High-level overview, quick insights, calls to action

**Layout:**
```
┌─ Evaluation Center: Dashboard ────────────────────────────────────────────┐
│                                                                            │
│  📊 Latest Results                    🎯 Best Configuration               │
│  ┌────────────────────────────────┐   ┌────────────────────────────────┐ │
│  │ M2 + AST + Smart Tests         │   │ Milestone: M2                  │ │
│  │ 63.2% accuracy                 │   │ 63.2% accuracy                 │ │
│  │ $2.45/task                     │   │ $2.45/task                     │ │
│  │ 2.3 min/task                   │   │ Features: 5 enabled            │ │
│  │ 2 hours ago                    │   │ [View Details]                 │ │
│  └────────────────────────────────┘   └────────────────────────────────┘ │
│                                                                            │
│  📈 Accuracy Trend (Last 10 Runs)     💰 Cost Efficiency                 │
│  ┌────────────────────────────────┐   ┌────────────────────────────────┐ │
│  │ 70% ┤                        ●  │   │  M3: $0.05 per accuracy point  │ │
│  │ 65% ┤                   ●       │   │  M2: $0.04 per accuracy point  │ │
│  │ 60% ┤          ●   ●            │   │  M1: $0.03 per accuracy point  │ │
│  │ 55% ┤    ●  ●                   │   │                                │ │
│  │ 50% ┼────────────────────────   │   │  Winner: M2 (best balance)     │ │
│  └────────────────────────────────┘   └────────────────────────────────┘ │
│                                                                            │
│  🔬 Feature Impact Summary             ⚠️  Insights & Recommendations     │
│  ┌────────────────────────────────┐   ┌────────────────────────────────┐ │
│  │ ✅ Smart Test Selection        │   │ • AST Context shows strong     │ │
│  │    +8.2% accuracy, -$0.30/task │   │   evidence of improvement      │ │
│  │ ✅ AST Context                 │   │                                │ │
│  │    +5.1% accuracy, +$0.10/task │   │ • Consider enabling Failure    │ │
│  │ ⚠️  Prompt Caching             │   │   Memory (untested)            │ │
│  │    -90% cost (needs validation)│   │                                │ │
│  │ ❓ Embeddings (no data)        │   │ • M3 Multi-Model needs more    │ │
│  └────────────────────────────────┘   │   evaluation runs (n=2)        │ │
│                                        └────────────────────────────────┘ │
│                                                                            │
│  [N]ew Run  [C]ompare  [H]istory  [G]raphs  [F]eature Config            │
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Latest run results card
- Best configuration card (by accuracy, cost efficiency, or custom metric)
- Trend chart (sparkline or mini chart)
- Feature impact summary (top 5 by effect size)
- Actionable insights (what to test next)
- Quick action buttons

---

### 2.2 Configure Screen

**Purpose:** Visual feature flag editor with evidence-based annotations

**Layout:**
```
┌─ Configuration Editor ─────────────────────────────────────────────────────┐
│                                                                            │
│  Preset: [Custom Configuration ▾]  [Load M1] [Load M2] [Load M3]         │
│                                                                            │
│  Context Strategies                                        Evidence        │
│  ├─ [✓] AST Context                           +5.1% acc, +$0.10 (strong) │
│  ├─ [ ] Embeddings                            +2.0% acc, +$0.50 (weak)   │
│  ├─ [ ] Graph Context                         No data yet                │
│  └─ [ ] Reranking                             +1.2% acc, +$0.05 (moderate)│
│                                                                            │
│  Routing Strategies                                                        │
│  ├─ [ ] Semantic Router                       No data yet                │
│  ├─ [ ] Multi-Model                           -10% cost, -1% acc (weak)  │
│  └─ [ ] Speculative Execution                 +30% speed, +$1.00 (weak)  │
│                                                                            │
│  Intelligence                                                              │
│  ├─ [✓] Smart Test Selection                  +8.2% acc, -$0.30 (strong) │
│  ├─ [ ] Failure Memory                        No data yet                │
│  └─ [ ] Opportunistic Planning                No data yet                │
│                                                                            │
│  Optimizations                                                             │
│  ├─ [✓] Prompt Caching                        -90% cost (theory only)    │
│  ├─ [ ] Semantic Caching                      No data yet                │
│  └─ [✓] Tree-sitter Validation                +2.1% acc, +$0.02 (moderate)│
│                                                                            │
│  ┌─ Prediction (based on historical data) ──────────────────────────────┐ │
│  │ Expected Accuracy: 58-62% (confidence: 75%)                          │ │
│  │ Expected Cost: $2.20-2.60 per task                                   │ │
│  │ Based on 8 similar runs                                              │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                                                            │
│  ↑/↓: Navigate  Space: Toggle  Enter: Start Eval  S: Save Preset  Esc: Back│
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Visual tree with checkboxes
- Evidence annotations next to each feature (accuracy impact, cost impact, confidence)
- Evidence quality indicators (strong/moderate/weak/none)
- Prediction engine (ML model predicting outcome based on config)
- Preset loading/saving
- Real-time cost/accuracy estimates

**Evidence Display Format:**
```
[✓] AST Context     +5.1% acc, +$0.10 (strong)
                    ↑         ↑        ↑
                    impact    cost     confidence
```

---

### 2.3 History Screen

**Purpose:** Browse all past evaluation runs, filter, sort, select for comparison

**Layout:**
```
┌─ Evaluation History ───────────────────────────────────────────────────────┐
│                                                                            │
│  Filters: [All Milestones ▾] [Last 30 Days ▾] [All Datasets ▾]           │
│  Sort by: [Date (newest) ▾]                          [Clear Filters]      │
│                                                                            │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ ☐ │ Date/Time       │ Config  │ Dataset │ Accuracy │ Cost   │ Tasks│  │
│  ├───┼─────────────────┼─────────┼─────────┼──────────┼────────┼──────┤  │
│  │ ☐ │ 2025-11-11 14:32│ M2-mod  │ Verified│  63.2%   │ $2.45  │  50  │  │
│  │ ☑ │ 2025-11-11 12:15│ M2      │ Verified│  61.8%   │ $2.30  │  50  │  │
│  │ ☑ │ 2025-11-10 16:45│ M1      │ Verified│  57.5%   │ $1.80  │  50  │  │
│  │ ☐ │ 2025-11-10 10:20│ M3-beta │ Lite    │  68.3%   │ $3.50  │  30  │  │
│  │ ☐ │ 2025-11-09 18:10│ M2      │ Verified│  62.0%   │ $2.28  │  50  │  │
│  │ ☐ │ 2025-11-09 14:30│ Custom  │ Verified│  59.4%   │ $2.10  │  25  │  │
│  │ ☐ │ 2025-11-08 11:00│ M1      │ Verified│  56.8%   │ $1.75  │  50  │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                            │
│  2 runs selected                                                           │
│                                                                            │
│  ↑/↓: Navigate  Space: Select  Enter: View Details  C: Compare Selected   │
│  D: Delete  E: Export  /: Search                                           │
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Sortable table (by date, accuracy, cost, duration)
- Multi-select with checkboxes
- Filters (milestone, date range, dataset, accuracy range)
- Search by notes/metadata
- Bulk operations (delete, export)
- Quick view details (Enter)
- Compare selected runs (C key)

---

### 2.4 Compare Screen

**Purpose:** Side-by-side comparison with statistical analysis

**Layout:**
```
┌─ Compare Evaluations ──────────────────────────────────────────────────────┐
│                                                                            │
│  Baseline: M1 (2025-11-10)            Experiment: M2-mod (2025-11-11)     │
│  ┌────────────────────────────────┐   ┌────────────────────────────────┐ │
│  │ Configuration                  │   │ Configuration                  │ │
│  │ ├─ [ ] AST Context             │   │ ├─ [✓] AST Context             │ │
│  │ ├─ [ ] Smart Test Selection    │   │ ├─ [✓] Smart Test Selection    │ │
│  │ ├─ [✓] Prompt Caching          │   │ ├─ [✓] Prompt Caching          │ │
│  │ └─ [✓] Tree-sitter Validation  │   │ └─ [✓] Tree-sitter Validation  │ │
│  │                                │   │                                │ │
│  │ Results                        │   │ Results                        │ │
│  │ Accuracy:     57.5%            │   │ Accuracy:     63.2%  (+5.7%)  │ │
│  │ Mean Cost:    $1.80            │   │ Mean Cost:    $2.45  (+$0.65) │ │
│  │ Median Cost:  $1.75            │   │ Median Cost:  $2.40  (+$0.65) │ │
│  │ P95 Latency:  3200ms           │   │ P95 Latency:  4100ms (+900ms) │ │
│  │ Tasks:        50               │   │ Tasks:        50              │ │
│  └────────────────────────────────┘   └────────────────────────────────┘ │
│                                                                            │
│  ┌─ Statistical Analysis ────────────────────────────────────────────────┐ │
│  │ Welch's t-test: p = 0.0023 (✓ statistically significant)             │ │
│  │ Effect Size (Cohen's d): 0.82 (large effect)                         │ │
│  │ 95% Confidence Interval: [2.1%, 9.3%] accuracy improvement           │ │
│  │                                                                       │ │
│  │ Recommendation: ✅ ADOPT                                              │ │
│  │ Rationale: Strong evidence of improvement (p < 0.01, large effect).  │ │
│  │            The +5.7% accuracy gain justifies the +$0.65 cost increase.│ │
│  │            Cost per accuracy point: $0.11 (acceptable).               │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                            │
│  ┌─ Cost/Benefit Analysis ────────────────────────────────────────────────┐│
│  │ ROI Score: 8.8 / 10 (excellent)                                       ││
│  │ • Accuracy improvement: +5.7 percentage points                        ││
│  │ • Cost increase: +36% (+$0.65 per task)                               ││
│  │ • Cost per accuracy point: $0.11 (good value)                         ││
│  │ • Marginal gains: Excellent (high impact for moderate cost)           ││
│  └───────────────────────────────────────────────────────────────────────┘│
│                                                                            │
│  [E]xport Report  [V]iew Failure Diff  [S]ave Comparison  Esc: Back      │
└────────────────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Side-by-side configuration diff
- Highlighted differences
- Delta calculations (+/-) with visual indicators
- Statistical significance testing (Welch's t-test, already implemented!)
- Effect size (Cohen's d, already implemented!)
- Recommendation engine with rationale
- Cost/benefit analysis
- ROI scoring
- Export comparison report (markdown, JSON)

---

### 2.5 Charts Screen

**Purpose:** Visual analytics, trends, distributions

**Layout:**
```
┌─ Analytics & Charts ───────────────────────────────────────────────────────┐
│                                                                            │
│  Chart Type: [Time Series ▾]  Metric: [Accuracy ▾]  Filter: [All ▾]      │
│                                                                            │
│  ┌─ Accuracy Over Time ──────────────────────────────────────────────────┐│
│  │ 70% ┤                                                             ●   ││
│  │ 65% ┤                                               ●                 ││
│  │ 60% ┤                           ●       ●   ●                         ││
│  │ 55% ┤             ●   ●   ●                                           ││
│  │ 50% ┤   ●   ●                                                         ││
│  │ 45% ┼─────────────────────────────────────────────────────────────────││
│  │     Oct-28    Nov-2     Nov-7     Nov-12    Nov-17                   ││
│  │                                                                        ││
│  │  ■ M1   ● M2   ▲ M3   ◆ Custom                                        ││
│  └────────────────────────────────────────────────────────────────────────┘│
│                                                                            │
│  ┌─ Cost vs Accuracy Scatter ────────────────────────────────────────────┐│
│  │ 70% ┤                                                    ◆ M3          ││
│  │ 65% ┤                                        ● M2                      ││
│  │ 60% ┤                            ● M2   ●                              ││
│  │ 55% ┤              ■ M1   ■ M1                                         ││
│  │ 50% ┤   ■ M1                                                           ││
│  │ 45% ┼──────────────────────────────────────────────────────────────────││
│  │     $1.50     $2.00      $2.50      $3.00      $3.50                  ││
│  │                                                                        ││
│  │  Efficiency Frontier: Higher accuracy, lower cost is better           ││
│  └────────────────────────────────────────────────────────────────────────┘│
│                                                                            │
│  [T]ime Series  [D]istribution  [S]catter  [H]eatmap  [F]eature Impact   │
└────────────────────────────────────────────────────────────────────────────┘
```

**Available Chart Types:**
1. **Time Series** - Accuracy, cost, latency over time
2. **Distribution** - Histograms (cost per task, latency, steps to completion)
3. **Scatter Plot** - Cost vs accuracy, latency vs accuracy
4. **Heatmap** - Feature correlation matrix
5. **Feature Impact** - Bar chart of accuracy/cost impact per feature
6. **Box Plot** - Distribution comparison (M1 vs M2 vs M3)

**Key Features:**
- Interactive chart selection
- Metric selection (accuracy, cost, latency, tokens)
- Filter by milestone, date range
- Export charts (ASCII art, CSV data, PNG if terminal supports)
- Hover/inspect data points (if supported)

---

### 2.6 Live Progress Screen

**This is what we already built!**

**Purpose:** Real-time monitoring during evaluation execution

**Layout:** (Keep existing 3-panel design)
```
┌─ Evaluation in Progress ───────────────────────────────────────────────────┐
│ Task 3/50: django__django-12345                                           │
│                                                                            │
│ ┌─ Conversation ──────┐ ┌─ Tool Executions ──┐ ┌─ Metrics ─────────────┐│
│ │ [Agent] Let me      │ │ Read               │ │ Current Step: 5/25    ││
│ │ analyze the issue...│ │ └─ models.py (✓)   │ │                       ││
│ │                     │ │                    │ │ Tokens:               ││
│ │ [Agent] I'll fix    │ │ Edit               │ │   Input: 12.5K        ││
│ │ the bug in...       │ │ └─ views.py (✓)    │ │   Output: 2.3K        ││
│ │                     │ │                    │ │   Cached: 45.2K       ││
│ │ [Tool] Read models  │ │ Bash               │ │                       ││
│ │ <output truncated>  │ │ └─ pytest (✓)      │ │ Cost: $0.45           ││
│ │                     │ │    PASSED          │ │ Latency: 2.3s/call    ││
│ └─────────────────────┘ └────────────────────┘ │                       ││
│                          ┌─ Files Modified ──┐ │ Overall Progress:     ││
│                          │ • models.py       │ │ Solved: 2/3 (66.7%)   ││
│                          │ • views.py        │ │ Total Cost: $1.23     ││
│                          │ • tests.py        │ │ Avg Time: 2.1 min     ││
│                          └───────────────────┘ └───────────────────────┘│
│                                                                            │
│  [Ctrl+C] Cancel  [P] Pause  [Esc] Minimize to background                 │
└────────────────────────────────────────────────────────────────────────────┘
```

**Keep existing features, add:**
- Pause/resume capability
- Minimize to background (evaluation continues, returns to dashboard)
- Estimated time remaining
- Progress percentage bar

---

## 3. Starting an Evaluation: Modal Wizard

**Press 'N' or 'R' (New Run) from any screen:**

### Step 1: Quick Start Modal
```
┌─────────────────────────────────────────────────────────────┐
│                   Start New Evaluation                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Configuration:  [M2: AST + Smart Tests  ▾]                │
│                   • AST Context                             │
│                   • Smart Test Selection                    │
│                   • Prompt Caching                          │
│                   • Tree-sitter Validation                  │
│                                                             │
│                   [Customize Features...]                   │
│                                                             │
│  Dataset:        [SWE-bench Verified     ▾]                │
│                   • 500 tasks available                     │
│                   • High-quality subset                     │
│                                                             │
│  Task Count:     [20          ]  (max: 500)               │
│  Random Seed:    [42          ]  (optional)               │
│                                                             │
│  ┌─ Prediction (based on 8 similar runs) ────────────────┐ │
│  │ Expected: 61-65% accuracy, $2.20-2.60/task            │ │
│  │ Estimated: $50 total, ~40 minutes                     │ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
│  [ Advanced Options... ]                                    │
│                                                             │
│  ↑/↓: Navigate  Enter: Confirm  Esc: Cancel               │
│                                                             │
│  [ Back ]                      [ Start Evaluation ]         │
└─────────────────────────────────────────────────────────────┘
```

### Step 2: Customize Features (if clicked)
Opens the Configuration Editor (Section 2.2)

### Step 3: Advanced Options (if clicked)
```
┌─────────────────────────────────────────────────────────────┐
│                    Advanced Options                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  LLM Settings:                                              │
│    Model:       [claude-sonnet-3-5-20241022  ▾]            │
│    Temperature: [1.0        ]  (0.0 - 2.0)                 │
│    Max Tokens:  [8192       ]                              │
│                                                             │
│  Agent Settings:                                            │
│    Max Steps:   [25         ]  (steps per task)            │
│    Timeout:     [600        ]  (seconds per task)          │
│                                                             │
│  Dataset Settings:                                          │
│    Randomize:   [✓] Shuffle tasks                          │
│    Filter:      [All        ▾]  (all/bugs/features/docs)   │
│                                                             │
│  Metadata:                                                  │
│    Run Name:    [M2 baseline test          ]               │
│    Notes:       [Testing AST context impact]               │
│                                                             │
│  ↑/↓: Navigate  Enter: Confirm  Esc: Back                  │
│                                                             │
│  [ Back ]                      [ Confirm ]                  │
└─────────────────────────────────────────────────────────────┘
```

**Navigation:**
- Tab/Shift+Tab to move between fields
- Arrow keys to navigate dropdowns
- Space to toggle checkboxes
- Enter to confirm selection
- Esc to cancel and return

---

## 4. Feature Impact Analysis Engine

**Goal:** Automatically compute which features improve accuracy/cost

### Algorithm:

```rust
fn compute_feature_impact(all_runs: &[EvaluationRun]) -> Vec<FeatureImpact> {
    let features = [
        "ast_context",
        "embeddings",
        "graph_context",
        "reranking",
        "semantic_router",
        "multi_model",
        "speculative_execution",
        "smart_test_selection",
        "failure_memory",
        "opportunistic_planning",
        "prompt_caching",
        "semantic_caching",
        "tree_sitter_validation",
    ];

    let mut impacts = vec![];

    for feature in features {
        // Split runs into enabled vs disabled
        let (enabled, disabled): (Vec<_>, Vec<_>) = all_runs
            .iter()
            .partition(|run| run.config.is_feature_enabled(feature));

        if enabled.len() < 2 || disabled.len() < 2 {
            // Not enough data
            impacts.push(FeatureImpact {
                feature_name: feature.to_string(),
                evidence_quality: EvidenceQuality::Insufficient,
                ..Default::default()
            });
            continue;
        }

        // Compute means
        let enabled_accuracy: Vec<f64> = enabled.iter()
            .map(|r| r.aggregate_metrics.accuracy)
            .collect();
        let disabled_accuracy: Vec<f64> = disabled.iter()
            .map(|r| r.aggregate_metrics.accuracy)
            .collect();

        let enabled_mean = mean(&enabled_accuracy);
        let disabled_mean = mean(&disabled_accuracy);
        let accuracy_impact = enabled_mean - disabled_mean;

        // Similar for cost
        let enabled_cost: Vec<f64> = enabled.iter()
            .map(|r| r.aggregate_metrics.mean_cost)
            .collect();
        let disabled_cost: Vec<f64> = disabled.iter()
            .map(|r| r.aggregate_metrics.mean_cost)
            .collect();

        let enabled_cost_mean = mean(&enabled_cost);
        let disabled_cost_mean = mean(&disabled_cost);
        let cost_impact = enabled_cost_mean - disabled_cost_mean;

        // Statistical test (already implemented!)
        let comparison = ComparisonResult::compare(
            &enabled_accuracy,
            &disabled_accuracy,
        );

        let p_value = comparison.p_value;
        let effect_size = comparison.effect_size;

        // Determine evidence quality
        let evidence_quality = match (p_value, effect_size.abs(), enabled.len() + disabled.len()) {
            (p, d, n) if p < 0.01 && d > 0.8 && n > 30 => EvidenceQuality::Strong,
            (p, d, n) if p < 0.05 && d > 0.5 && n > 10 => EvidenceQuality::Moderate,
            (p, d, n) if p < 0.1 && d > 0.2 && n > 5 => EvidenceQuality::Weak,
            _ => EvidenceQuality::Insufficient,
        };

        impacts.push(FeatureImpact {
            feature_name: feature.to_string(),
            enabled_runs: enabled.iter().map(|r| r.id).collect(),
            disabled_runs: disabled.iter().map(|r| r.id).collect(),
            accuracy_impact,
            cost_impact,
            latency_impact: 0, // TODO: compute from latency data
            statistical_significance: p_value,
            effect_size,
            confidence: 1.0 - p_value, // Simple confidence metric
            evidence_quality,
        });
    }

    // Sort by impact (accuracy improvement)
    impacts.sort_by(|a, b| b.accuracy_impact.partial_cmp(&a.accuracy_impact).unwrap());

    impacts
}
```

### Display Format:

```
Feature Impact Summary:
1. ✅ Smart Test Selection     +8.2% acc, -$0.30/task (strong, n=15)
2. ✅ AST Context              +5.1% acc, +$0.10/task (strong, n=20)
3. ⚠️  Prompt Caching          -90% cost (theory only, n=2)
4. ✅ Tree-sitter Validation   +2.1% acc, +$0.02/task (moderate, n=12)
5. ⚠️  Reranking               +1.2% acc, +$0.05/task (weak, n=6)
6. ❓ Embeddings               No data yet (n=0)
7. ❓ Graph Context            No data yet (n=0)
```

---

## 5. Recommendation Engine

**Goal:** Automatically suggest what to do next

### Types of Recommendations:

#### 5.1 Feature Recommendations
```rust
enum FeatureRecommendation {
    EnableNow(String, Rationale),      // Strong evidence, enable immediately
    Test(String, ExperimentDesign),    // No data, design experiment to test
    Disable(String, Rationale),        // Negative impact, disable
    MoreData(String, usize),           // Need N more runs for confidence
}
```

**Example Output:**
```
🎯 Recommendations:

1. ✅ Enable "Smart Test Selection" immediately
   Rationale: Strong evidence (+8.2% accuracy, p=0.002, large effect)
   Impact: Saves $0.30 per task while improving accuracy

2. 🔬 Test "Failure Memory" feature
   Design: Run 10 tasks with feature enabled, 10 with disabled
   Hypothesis: May improve accuracy on repeated error patterns
   Cost: ~$50, 1 hour

3. ❌ Disable "Speculative Execution"
   Rationale: Increases cost (+$1.00/task) with minimal gains (+0.5% acc)
   Effect size too small to justify cost

4. 📊 Collect more data on "Embeddings"
   Current: Only 2 runs, need 8 more for statistical confidence
   Preliminary: +2.0% accuracy, but p=0.15 (not significant)
```

#### 5.2 Configuration Recommendations
```rust
fn recommend_best_config(
    all_runs: &[EvaluationRun],
    optimization_target: OptimizationTarget,
) -> (ToadConfig, Rationale) {
    match optimization_target {
        OptimizationTarget::MaxAccuracy => {
            // Find config with highest accuracy (with confidence)
        }
        OptimizationTarget::MinCost => {
            // Find config with lowest cost above accuracy threshold
        }
        OptimizationTarget::BestBalance => {
            // Pareto frontier: best accuracy/cost ratio
        }
        OptimizationTarget::Custom(metric) => {
            // User-defined optimization function
        }
    }
}
```

#### 5.3 Experiment Design Recommendations

When a feature has no data, suggest experiment:
```
Suggested Experiment: Test "Failure Memory"

Design:
  • Group A (Baseline): M2 without Failure Memory
  • Group B (Experiment): M2 with Failure Memory
  • Sample Size: 20 tasks each (80% power, α=0.05)
  • Dataset: SWE-bench Verified (randomized)

Hypothesis: Failure Memory improves accuracy on tasks with repeated
            error patterns by learning from past mistakes.

Expected Cost: $100 ($2.50 × 40 tasks)
Duration: ~1.5 hours
```

---

## 6. Data Persistence

### File Structure:
```
./results/
├── runs/
│   ├── 2025-11-11T14-32-15_M2-mod.json
│   ├── 2025-11-11T12-15-30_M2.json
│   └── 2025-11-10T16-45-22_M1.json
├── comparisons/
│   ├── M2_vs_M1_2025-11-11.json
│   └── M3_vs_M2_2025-11-10.json
├── feature_impacts.json         # Cached feature impact analysis
└── metadata.json                 # Run index, tags, notes
```

### Persistent State:
- All runs saved as JSON with full config snapshot
- Comparisons cached (don't recompute on every view)
- Feature impact analysis cached (recompute when new runs added)
- User notes/tags stored in metadata.json

---

## 7. Implementation Phases

### Phase 1: Data Model & Persistence (Foundation)
**Priority: P0**
- [x] EvaluationRun struct already exists
- [ ] Extend with full config snapshot
- [ ] Add metadata (notes, tags, git commit)
- [ ] Implement JSON serialization/deserialization
- [ ] Create results/ directory structure
- [ ] Add run history manager (load all runs, filter, sort)

### Phase 2: Feature Impact Analysis (Core Analytics)
**Priority: P0**
- [ ] Implement feature impact computation algorithm
- [ ] Use existing Welch's t-test and Cohen's d implementations
- [ ] Cache results in feature_impacts.json
- [ ] Regenerate when new runs added

### Phase 3: Dashboard Screen (MVP)
**Priority: P0**
- [ ] Create Dashboard struct and rendering logic
- [ ] Latest run card
- [ ] Best config card
- [ ] Feature impact summary (top 5)
- [ ] Quick action buttons (N for new run)

### Phase 4: Configuration Editor
**Priority: P1**
- [ ] Visual tree widget with checkboxes
- [ ] Evidence annotations (load from feature_impacts.json)
- [ ] Preset loading (M1/M2/M3)
- [ ] Custom config editing
- [ ] Prediction engine (ML model or simple average)

### Phase 5: New Run Modal Wizard
**Priority: P1**
- [ ] Modal dialog widget
- [ ] Quick start screen (config dropdown, dataset, task count)
- [ ] Advanced options screen
- [ ] Integration with existing eval runner

### Phase 6: History Browser
**Priority: P1**
- [ ] Table widget with sortable columns
- [ ] Multi-select with checkboxes
- [ ] Filters (milestone, date, dataset)
- [ ] Load from results/ directory

### Phase 7: Compare Screen
**Priority: P1**
- [ ] Side-by-side layout
- [ ] Config diff highlighting
- [ ] Statistical analysis display (use existing implementations!)
- [ ] Recommendation engine
- [ ] Cost/benefit analysis
- [ ] ROI scoring

### Phase 8: Charts & Analytics
**Priority: P2**
- [ ] Time series chart (ASCII art using ratatui Chart widget)
- [ ] Scatter plot (cost vs accuracy)
- [ ] Distribution histograms
- [ ] Feature impact bar chart
- [ ] Export functionality

### Phase 9: Recommendation Engine
**Priority: P2**
- [ ] Feature recommendation logic
- [ ] Experiment design generator
- [ ] Best config finder
- [ ] Display in Dashboard insights card

### Phase 10: Live Progress Enhancements
**Priority: P2**
- [ ] Pause/resume evaluation
- [ ] Minimize to background
- [ ] Estimated time remaining
- [ ] Progress percentage

---

## 8. Key Interactions & Keybindings

### Global (Available from any Eval Center screen):
- `F9` - Open/close Evaluation Center
- `Esc` - Go back / Close
- `1-6` - Switch between screens (Dashboard, Configure, History, Compare, Charts, Live)
- `N` or `R` - New Run (opens modal wizard)
- `?` - Help overlay

### Dashboard Screen:
- `Enter` - View latest run details
- `C` - Compare (prompts to select 2 runs)
- `G` - Go to Charts
- `F` - Go to Feature Config

### Configuration Editor:
- `↑/↓` - Navigate tree
- `Space` - Toggle feature
- `Enter` - Start evaluation with this config
- `L` - Load preset (M1/M2/M3)
- `S` - Save as custom preset

### History Browser:
- `↑/↓` - Navigate table
- `Space` - Select/deselect for comparison
- `Enter` - View run details
- `C` - Compare selected runs (need 2+)
- `D` - Delete selected runs
- `/` - Search/filter

### Compare Screen:
- `E` - Export comparison report
- `V` - View failure diff (which tasks failed in A but not B)
- `S` - Save comparison

### Charts Screen:
- `T` - Time series view
- `D` - Distribution view
- `S` - Scatter plot view
- `H` - Heatmap view
- `F` - Feature impact bar chart

---

## 9. Technical Implementation Notes

### 9.1 Ratatui Widgets to Use:
- `Table` - History browser, comparison tables
- `List` - Feature tree, navigation menu
- `Paragraph` - Metrics cards, text content
- `Block` with `Borders` - All panels
- `Gauge` - Progress bars
- `Chart` - Time series, scatter plots (ASCII art)
- `Tabs` - Screen navigation
- Custom widgets for:
  - Evidence badges (strong/moderate/weak)
  - Recommendation cards
  - Modal dialogs

### 9.2 State Management:
```rust
struct EvalCenterApp {
    screen: EvalCenterScreen,
    run_history: Vec<EvaluationRun>,
    feature_impacts: Vec<FeatureImpact>,
    selected_runs: Vec<Uuid>,  // For comparison
    config_editor: ConfigEditor,
    dashboard: Dashboard,
    history_browser: HistoryBrowser,
    compare_view: CompareView,
    charts: ChartsView,
    modal_state: Option<ModalState>,
}

enum EvalCenterScreen {
    Dashboard,
    Configure,
    History,
    Compare,
    Charts,
    Live(EvaluationProgress),
}

enum ModalState {
    NewRunWizard(NewRunWizardState),
    ConfirmDelete(Vec<Uuid>),
    ExportOptions,
}
```

### 9.3 Data Loading Strategy:
- Load all runs on Eval Center open (cache in memory)
- Lazy-load task details (only when viewing run details)
- Recompute feature impacts when new run added
- Save to disk immediately after each run

### 9.4 Performance Considerations:
- Limit history to last 100 runs (or configurable)
- Paginate history table if > 50 runs
- Lazy render charts (only compute when Charts screen opened)
- Cache computed statistics

---

## 10. Success Metrics

How do we know the Eval Center is successful?

1. **Usage Metrics:**
   - % of evaluations started via UI (vs CLI)
   - Time spent in Eval Center per session
   - Most viewed screens (Dashboard, Compare, etc.)

2. **Decision Support:**
   - # of features enabled/disabled based on evidence
   - # of comparisons run per week
   - # of experiments designed via recommendation engine

3. **User Feedback:**
   - "I can quickly see which features improve accuracy"
   - "The comparison view helps me make evidence-based decisions"
   - "Starting an evaluation is now much easier"

4. **Quality of Decisions:**
   - Accuracy improvements after adopting recommended features
   - Cost reductions after disabling low-impact features
   - Fewer blind experiments (more hypothesis-driven)

---

## 11. Future Enhancements (Post-MVP)

### Machine Learning Prediction Model:
- Train model on past runs to predict accuracy/cost for new configs
- Use features: config vector, dataset, task count
- Target: accuracy, cost, latency

### Experiment Queue:
- Queue multiple evaluations to run sequentially
- Overnight batch processing

### Collaborative Features:
- Export reports for sharing (markdown, HTML)
- Import runs from teammates
- Team leaderboard (who found best config?)

### Advanced Analytics:
- Task clustering (which tasks are similar?)
- Failure pattern analysis (common error types)
- Tool use efficiency (which tools are most effective?)

### Integration:
- GitHub Issues integration (auto-test on new SWE-bench tasks)
- Slack/Discord notifications (evaluation complete)
- API endpoint (trigger evaluations remotely)

---

## Summary

This Evaluation Center transforms TOAD from "run evaluations blindly" to "make evidence-based decisions about AI agent development."

**Core Principles:**
1. **Evidence-based:** Every feature shows impact data
2. **Statistical rigor:** Use t-tests, effect sizes, confidence intervals
3. **Decision support:** Recommend what to do next
4. **Easy to use:** Modal wizards, visual editors, keyboard-driven
5. **Comprehensive:** Dashboard → Configure → Run → Compare → Decide

**Next Steps:**
1. Review this design document
2. Approve/modify the plan
3. Start with Phase 1 (Data Model) and Phase 3 (Dashboard MVP)
4. Iterate based on usage feedback

This is how you build a world-class AI coding assistant. 🚀
