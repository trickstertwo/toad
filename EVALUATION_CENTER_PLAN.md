# EVALUATION CENTER: Real-Time Monitoring & Testing Suite

**Status**: Critical Missing Features - Evaluation runs blind with no visibility
**Problem**: User can't see what the agent is doing, no conversation, no step details, no comparison tools

---

## 🚨 CRITICAL ISSUES (What's Broken Now)

### 1. **No Real-Time Agent Visibility**
- ❌ Can't see agent's thinking/conversation
- ❌ Can't see tool calls (Read/Edit/Bash) with outputs
- ❌ Can't see files being modified live
- ❌ Can't see error messages from tools
- ❌ Can't see test outputs when they run

### 2. **No Problem/Answer Context**
- ❌ Problem statement not shown during eval
- ❌ Expected solution not visible
- ❌ Agent's actual solution not captured
- ❌ Can't compare "what it should do" vs "what it did"
- ❌ No way to see WHY a task failed

### 3. **No Historical Data**
- ❌ Can't view past evaluation runs
- ❌ Can't compare new run with old runs
- ❌ No persistent storage of detailed results
- ❌ No trend analysis (is M2 better than M1 over time?)

### 4. **No Interactive Testing**
- ❌ Must start a new eval to see anything
- ❌ Can't browse old results
- ❌ Can't re-run single failed tasks
- ❌ Can't drill down into specific task execution

---

## 📊 P0 FEATURES (Must Have - Core Functionality)

### P0.1: **Real-Time Agent Conversation View**
**What**: Show the actual LLM conversation as it happens

```
┌─────────────────────────────────────────────────────────┐
│ 🤖 Agent Conversation - Task django__django-12345       │
├─────────────────────────────────────────────────────────┤
│ Step 1/25  Tool: None  Tokens: 0  Cost: $0.00           │
│                                                           │
│ 🧠 Assistant [thinking...]                              │
│ I need to understand the problem first. The issue is    │
│ about migration commands not handling dependencies      │
│ correctly. Let me start by reading the migration file.  │
│                                                           │
│ Step 2/25  Tool: Read  Tokens: 1,234  Cost: $0.02      │
│                                                           │
│ 🔧 Tool Call: Read                                       │
│ File: src/django/core/management/commands/migrate.py    │
│                                                           │
│ 📄 Tool Output:                                          │
│ ```python                                                 │
│ class Command(BaseCommand):                              │
│     def handle(self, *args, **options):                  │
│         # Migration logic here...                        │
│ ```                                                       │
│                                                           │
│ Step 3/25  Tool: None  Tokens: 2,456  Cost: $0.04      │
│                                                           │
│ 🧠 Assistant [thinking...]                              │
│ I see the issue. The dependency resolution doesn't      │
│ account for circular dependencies. I'll need to modify  │
│ the topological sort algorithm...                        │
│                                                           │
│ [Scroll: j/k  Search: /  Copy: v  Export: E]            │
└─────────────────────────────────────────────────────────┘
```

**Implementation**:
- Extend `EvaluationProgress` to include:
  ```rust
  pub conversation: Vec<ConversationMessage>,
  pub current_thinking: Option<String>,
  pub tool_calls: Vec<ToolCallDetail>,
  ```
- Agent loop sends message after each step
- TUI renders conversation in real-time

### P0.2: **Problem/Solution Context Panel**
**What**: Always show what the task is asking for

```
┌─────────────────────────────────────────────────────────┐
│ 📋 Task Context                                          │
├─────────────────────────────────────────────────────────┤
│ ID: django__django-12345                                 │
│ Complexity: Medium                                       │
│                                                           │
│ 📝 PROBLEM:                                              │
│ Migration dependencies are not correctly resolved when   │
│ there are circular references between apps. The system   │
│ should detect and break cycles gracefully.               │
│                                                           │
│ 🎯 EXPECTED BEHAVIOR:                                    │
│ Migrations should run in correct order, breaking cycles  │
│ by introducing intermediate steps.                        │
│                                                           │
│ ✅ SUCCESS CRITERIA:                                     │
│ All tests in tests/migrations/test_dependencies.py pass │
│                                                           │
│ 📁 FILES TO MODIFY:                                      │
│ • src/django/core/management/commands/migrate.py        │
│ • src/django/db/migrations/graph.py                     │
└─────────────────────────────────────────────────────────┘
```

**Implementation**:
- Add `problem_context: Task` to `EvaluationProgress`
- Right sidebar shows task details
- Updates when task changes

### P0.3: **Step-by-Step Tool Execution Log**
**What**: Show every tool call with inputs/outputs

```
┌─────────────────────────────────────────────────────────┐
│ 🔧 Tool Execution Log                                    │
├─────────────────────────────────────────────────────────┤
│ Step  Tool      File/Command              Status  Time  │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│  1    Read      migrate.py                ✅     120ms │
│  2    Read      graph.py                  ✅      80ms │
│  3    Grep      "topological.*sort"       ✅      45ms │
│  4    Edit      graph.py (+15, -3)        ✅     200ms │
│  5    Bash      python manage.py test     ❌    3.2s   │
│       └─ Error: AssertionError: cycle not detected     │
│  6    Edit      graph.py (+2, -1)         ✅     150ms │
│  7    Bash      python manage.py test     ✅    3.1s   │
│       └─ All tests passed (5/5)                         │
│                                                           │
│ [Click to expand tool output]                            │
└─────────────────────────────────────────────────────────┘
```

**Implementation**:
- Capture tool inputs/outputs in agent loop
- Send via `EvaluationProgress.tool_calls`
- Render as expandable list in TUI

### P0.4: **Task Results with Full Details**
**What**: After task completes, show comprehensive results

```
┌─────────────────────────────────────────────────────────┐
│ ✅ Task Result: django__django-12345                     │
├─────────────────────────────────────────────────────────┤
│ Status: SOLVED                                           │
│ Time: 127s    Cost: $0.145    Steps: 7/25    API: 12   │
│                                                           │
│ 📊 METRICS:                                              │
│ • Tests Passed: 5/5 (100%)                              │
│ • Files Modified: 2                                      │
│ • Lines Added: 17    Lines Removed: 4                   │
│ • Tool Calls: Read(3), Edit(2), Bash(2)                │
│                                                           │
│ ✅ SOLUTION:                                             │
│ Added cycle detection in topological_sort() and         │
│ introduced intermediate migration steps to break cycles │
│                                                           │
│ 📝 FILES CHANGED:                                        │
│ • src/django/db/migrations/graph.py                     │
│   +17 -4  (added detect_cycles method)                  │
│ • src/django/core/management/commands/migrate.py        │
│   +0 -0   (no changes, read only)                       │
│                                                           │
│ [V]iew Diff  [R]eplay Execution  [E]xport              │
└─────────────────────────────────────────────────────────┘
```

### P0.5: **Persistent Results Storage**
**What**: Save detailed results to database/JSON for later viewing

**Storage Schema**:
```rust
pub struct DetailedEvalResult {
    pub run_id: String,                    // UUID for this eval run
    pub timestamp: DateTime<Utc>,
    pub milestone: u8,
    pub dataset: String,                   // "swebench-verified"
    pub task_count: usize,

    pub tasks: Vec<DetailedTaskResult>,

    pub summary: EvaluationResults,        // Existing summary stats
}

pub struct DetailedTaskResult {
    pub task: Task,                        // Full problem statement
    pub result: TaskResult,                // Outcome (solved/failed)

    pub conversation: Vec<Message>,        // Full LLM conversation
    pub tool_calls: Vec<ToolExecution>,   // All tool calls with I/O
    pub files_changed: Vec<FileDiff>,     // Actual diffs
    pub test_output: String,               // Test execution output
    pub error_log: Option<String>,         // Errors if failed
}
```

**Implementation**:
- Save to `./results/{run_id}/detailed.json`
- Index of runs in `./results/index.json`
- Load on startup for historical view

### P0.6: **Evaluation History Browser**
**What**: Browse past evaluation runs without starting new ones

```
┌─────────────────────────────────────────────────────────┐
│ 📚 Evaluation History                  [Press H to open]│
├─────────────────────────────────────────────────────────┤
│ Date/Time          Milestone  Tasks  Accuracy  Cost     │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│ 2025-11-11 10:30   M1         10     57.3%     $1.45   │
│ 2025-11-11 09:15   M2         10     63.1%     $2.18   │
│ 2025-11-10 14:22   M1         50     55.8%     $7.20   │
│ 2025-11-10 11:05   M1         10     60.0%     $1.52   │
│ 2025-11-09 16:40   M1         10     56.7%     $1.38   │
│                                                           │
│ [↑/↓] Navigate  [Enter] View  [D]elete  [C]ompare       │
└─────────────────────────────────────────────────────────┘
```

**Drill-Down**:
```
┌─────────────────────────────────────────────────────────┐
│ 📊 Run Details - 2025-11-11 10:30                       │
├─────────────────────────────────────────────────────────┤
│ Milestone: M1    Dataset: SWE-bench Verified            │
│ Accuracy: 57.3%  Solved: 57/100  Cost: $14.52          │
│                                                           │
│ TASKS BY STATUS:                                         │
│ ✅ Solved (57)          [Click to view]                 │
│ ❌ Failed (43)          [Click to view]                 │
│                                                           │
│ TASK BREAKDOWN:                                          │
│ ID                      Status   Time    Cost    Steps  │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│ django__django-12345    ✅       127s    $0.15   7/25  │
│ requests__requests-789  ❌       245s    $0.28  25/25  │
│ flask__flask-456        ✅        89s    $0.12   5/25  │
│ ...                                                      │
│                                                           │
│ [Enter] Drill down  [R]eplay  [E]xport  [Back]         │
└─────────────────────────────────────────────────────────┘
```

---

## 📈 P1 FEATURES (Important - Better UX)

### P1.1: **Side-by-Side Comparison View**
**What**: Compare two evaluation runs side-by-side

```
┌───────────────────────────────┬───────────────────────────────┐
│ M1 Baseline (2025-11-10)      │ M2 AST Context (2025-11-11)   │
├───────────────────────────────┼───────────────────────────────┤
│ Accuracy: 57.3%               │ Accuracy: 63.1%   (+5.8pp) ✅│
│ Avg Cost: $0.12/task          │ Avg Cost: $0.18   (+50%)   ⚠│
│ Avg Time: 134s                │ Avg Time: 189s    (+41%)   ⚠│
│ Tasks Solved: 57/100          │ Tasks Solved: 63/100       ✅│
│                               │                                │
│ ONLY M1 SOLVED (8 tasks):    │ ONLY M2 SOLVED (14 tasks):    │
│ • django-11111                │ • flask-22222                 │
│ • requests-33333              │ • numpy-44444                 │
│ ...                           │ ...                            │
│                               │                                │
│ BOTH SOLVED (49 tasks)       │ NEITHER SOLVED (29 tasks)     │
│                               │                                │
│ [D]iff Tasks  [S]tatistics  [E]xport                         │
└───────────────────────────────┴───────────────────────────────┘
```

### P1.2: **Live Diff View for File Changes**
**What**: Show actual code changes as agent makes them

```
┌─────────────────────────────────────────────────────────┐
│ 📝 Live Diff: src/django/db/migrations/graph.py         │
├─────────────────────────────────────────────────────────┤
│ 123  class MigrationGraph:                               │
│ 124      def __init__(self):                             │
│ 125          self.nodes = {}                             │
│ 126 +        self.cycle_detector = CycleDetector()   NEW│
│ 127                                                       │
│ 128      def topological_sort(self):                     │
│ 129 -        return naive_sort(self.nodes)          OLD │
│ 130 +        # Detect cycles before sorting          NEW│
│ 131 +        if self.cycle_detector.has_cycle():     NEW│
│ 132 +            return self.break_cycles()          NEW│
│ 133 +        return smart_sort(self.nodes)           NEW│
│ 134                                                       │
│ [A]ccept  [R]eject  [V]iew Full  [E]xplain              │
└─────────────────────────────────────────────────────────┘
```

### P1.3: **Task Replay Mode**
**What**: Replay a past task execution step-by-step

```
┌─────────────────────────────────────────────────────────┐
│ 🎬 Replay: django__django-12345                         │
├─────────────────────────────────────────────────────────┤
│ [◀◀] [◀] [▶] [▶▶]  Step 3/7                           │
│                                                           │
│ Current Step: Edit graph.py                              │
│                                                           │
│ 🤖 Agent Thought:                                        │
│ "I need to add cycle detection logic here..."           │
│                                                           │
│ 🔧 Tool Call:                                            │
│ Edit(file="graph.py", diff="+15 -3")                    │
│                                                           │
│ 📄 Changes:                                              │
│ [Shows diff from previous step]                          │
│                                                           │
│ [Space] Play/Pause  [←/→] Prev/Next  [0] Restart       │
└─────────────────────────────────────────────────────────┘
```

### P1.4: **Failed Task Analysis**
**What**: Dedicated view for understanding failures

```
┌─────────────────────────────────────────────────────────┐
│ ❌ Failure Analysis: requests__requests-789              │
├─────────────────────────────────────────────────────────┤
│ WHY IT FAILED:                                           │
│ • Agent hit max steps (25/25) without solution          │
│ • Tests still failing: 2/5 passed                       │
│ • Error: ConnectionTimeout in test_retry_logic          │
│                                                           │
│ WHAT AGENT TRIED:                                        │
│ Step 1-5:   Read relevant files                         │
│ Step 6-12:  Made edits to retry logic                   │
│ Step 13-15: Ran tests, got failures                     │
│ Step 16-20: Tried to fix timeout handling               │
│ Step 21-25: Still failing, gave up                      │
│                                                           │
│ LIKELY ISSUE:                                            │
│ Agent didn't understand async timeout semantics         │
│                                                           │
│ SUGGESTIONS:                                             │
│ • May need better async code understanding (M2 feature) │
│ • Should add timeout hints to problem statement         │
│                                                           │
│ [R]eplay  [V]iew Conversation  [C]ompare with Similar   │
└─────────────────────────────────────────────────────────┘
```

### P1.5: **Statistical Dashboard**
**What**: Visual charts and graphs for trends

```
┌─────────────────────────────────────────────────────────┐
│ 📊 Statistical Dashboard                                 │
├─────────────────────────────────────────────────────────┤
│ ACCURACY TREND:                                          │
│ 70% │                                              ▄     │
│ 60% │                                   ▄▄▄▃▃▃▃▃███     │
│ 50% │                       ▄▄▄▃▃▃▃▃███                 │
│ 40% │           ▄▄▄▃▃▃▃▃███                             │
│ 30% │   ▃▃▃▃▃███                                        │
│     └─────────────────────────────────────────────────> │
│      M0   M1(v1) M1(v2) M1(v3)  M2(v1)  M2(v2)  M3    │
│                                                           │
│ COST vs ACCURACY:                                        │
│   ┌─────────────────────────────────────┐              │
│ A │              M3 ●                    │ Best balance │
│ c │           M2 ●                       │              │
│ c │        M1 ●                          │              │
│ u │     M0 ●                             │              │
│ r │                                      │              │
│ a └────────────────────────────────────>│              │
│ c              Cost per Task             │              │
│ y                                        │              │
│                                                           │
│ [V]iew Runs  [F]ilter  [E]xport                         │
└─────────────────────────────────────────────────────────┘
```

### P1.6: **Quick Re-Run Failed Tasks**
**What**: Re-run specific failed tasks from history

```
┌─────────────────────────────────────────────────────────┐
│ 🔄 Re-Run Failed Tasks                                   │
├─────────────────────────────────────────────────────────┤
│ From: M1 Baseline (2025-11-11 10:30)                    │
│                                                           │
│ Select tasks to re-run:                                  │
│ ☑ requests__requests-789     (Failed: timeout)          │
│ ☑ flask__flask-456           (Failed: max steps)        │
│ ☐ numpy__numpy-123           (Failed: test error)       │
│ ☑ pandas__pandas-999         (Failed: syntax error)     │
│                                                           │
│ Re-run with:                                             │
│ • Milestone: M2  [Change]                               │
│ • Max steps: 30  [Change]                               │
│ • Timeout: 300s  [Change]                               │
│                                                           │
│ [A]ll Failed  [N]one  [R]un Selected (3 tasks)          │
└─────────────────────────────────────────────────────────┘
```

---

## 🎨 P2 FEATURES (Nice to Have - Polish)

### P2.1: **Export to Multiple Formats**
- JSON (detailed results)
- CSV (for spreadsheet analysis)
- HTML (shareable report)
- Markdown (for GitHub/docs)

### P2.2: **Search/Filter Results**
```
Search: [django____________]  [Go]

Filters:
☑ Status: Solved  ☐ Failed
☑ Complexity: Medium  ☑ Hard
☐ Duration > 120s
☐ Cost > $0.20
```

### P2.3: **Annotations/Notes**
Add notes to specific runs or tasks:
```
📝 Notes for django__django-12345:
"This task requires understanding of Django's migration
 graph data structure. Good test case for AST context."
```

### P2.4: **Tag/Label System**
Tag runs for organization:
```
Tags: #baseline #m1 #verified-dataset #production-ready
```

### P2.5: **Automated Insights**
AI-generated insights:
```
💡 Insights:
• M2 performs 8.5% better on "Hard" complexity tasks
• Average cost increased 42% but accuracy gain is 5.8pp
• ROI: $0.13 per percentage point improvement
• Recommendation: ADOPT M2 for production
```

### P2.6: **Live Collaboration**
Share live evaluation links:
```
Share: https://toad.dev/eval/abc123
Others can watch your eval run in real-time
```

---

## 🏗️ IMPLEMENTATION PLAN

### Phase 1: Real-Time Visibility (P0.1-P0.3)
**Goal**: See what's happening during eval
**Time**: 2-3 days

1. Extend `EvaluationProgress` struct
2. Modify agent loop to capture conversation
3. Capture tool call details (inputs/outputs)
4. Update TUI to render conversation panel
5. Add tool execution log widget

### Phase 2: Persistent Storage (P0.4-P0.6)
**Goal**: Save and browse historical results
**Time**: 2-3 days

1. Design `DetailedEvalResult` schema
2. Save to JSON after each run
3. Create index file for fast lookup
4. Build history browser UI
5. Add drill-down into specific runs

### Phase 3: Comparison Tools (P1.1-P1.2)
**Goal**: Compare runs side-by-side
**Time**: 2 days

1. Load two runs simultaneously
2. Build side-by-side comparison view
3. Add diff highlighting
4. Show which tasks differ

### Phase 4: Replay & Analysis (P1.3-P1.5)
**Goal**: Understand failures better
**Time**: 3 days

1. Build task replay engine
2. Create failure analysis view
3. Add statistical dashboard
4. Implement trend charts

### Phase 5: Polish (P2.1-P2.6)
**Goal**: Professional-grade UX
**Time**: 2-3 days

1. Export formats
2. Search/filter
3. Notes/tags
4. Automated insights

---

## 📐 PROPOSED UI LAYOUT (Evaluation Screen)

```
┌──────────────────────────────────────────────────────────────┐
│ 🐸 TOAD  Eval: M1 Baseline  🔄 Running  Task 3/10  $0.42    │ Status
├──────────────┬──────────────────────────────┬────────────────┤
│ 📋 Task      │ 🤖 Agent Conversation        │ 🔧 Tools       │
│ ━━━━━━━━     │ ━━━━━━━━━━━━━━━━━━━━━━━━━━  │ ━━━━━━━━━━━━  │
│ ID: django-  │ Step 5/25  Tokens: 2.3k      │ Read     x3    │
│ 12345        │                               │ Edit     x1    │
│              │ 🧠 Assistant [thinking...]   │ Bash     x1    │
│ PROBLEM:     │ I need to add cycle          │ Grep     x0    │
│ Migration    │ detection here...             │                │
│ dependencies │                               │ CURRENT:       │
│ not resolved │ 🔧 Tool: Edit graph.py       │ Edit           │
│ correctly... │                               │ graph.py       │
│              │ 📄 Output:                    │ +15 -3         │
│ FILES:       │ ```python                     │                │
│ ☑ migrate.py │ +def detect_cycles():         │ STATUS:        │
│ ☑ graph.py   │   ...                         │ In progress... │
│              │ ```                            │                │
│ TESTS:       │                               │ ━━━━━━━━━━━━  │
│ 0/5 passed   │ [j/k scroll  / search]       │ Step  Tool     │
│              │                               │  1    Read     │
│ [V]iew Full  │                               │  2    Read     │
│              │                               │  3    Grep     │
├──────────────┴──────────────────────────────┴────────────────┤
│ Progress: ████████████████████░░░░░░░░ 60%  Est: 45s remain │ Progress
├───────────────────────────────────────────────────────────────┤
│ Ctrl+C:Cancel  H:History  R:Retry  S:Save  E:Export  Q:Quit │ Shortcuts
└───────────────────────────────────────────────────────────────┘
```

---

## 🎯 SUCCESS METRICS

After implementing P0-P2:

### Visibility
- ✅ Can see every agent step in real-time
- ✅ Can see tool calls with full I/O
- ✅ Can see problem + expected solution
- ✅ Can see why tasks fail

### Historical Analysis
- ✅ Can browse all past runs
- ✅ Can compare any two runs
- ✅ Can replay any task
- ✅ Can export detailed reports

### Developer Productivity
- ⏱ Time to debug failed task: < 2 minutes (vs ∞ now)
- ⏱ Time to compare two runs: < 30 seconds
- ⏱ Time to find regression: < 1 minute
- 📊 Confidence in eval results: 100% (vs ~50% now)

### User Experience
- 🎯 Evaluation feels "alive" with real-time updates
- 🔍 Can investigate any result thoroughly
- 📈 Can track progress over time
- 🚀 Testing center is professional-grade tool

---

## 📝 IMMEDIATE NEXT STEPS

1. **Extend EvaluationProgress** (30 min)
   - Add conversation, tool_calls, current_task fields

2. **Capture Agent Conversation** (2 hours)
   - Modify agent loop to save messages
   - Send via progress updates

3. **Build Real-Time Conversation View** (3 hours)
   - Create scrollable conversation widget
   - Render messages with syntax highlighting

4. **Test with Simple Eval** (1 hour)
   - Run `eval --count 1` and verify visibility

**Total for MVP visibility**: ~6 hours of focused work

---

**Ready to implement?** Start with Phase 1 P0.1-P0.3 for immediate impact!
