# Project Initialization Guide for Claude Code

> **Based on 6 months of production use (300k LOC rewrite)** - Verified principles only

**Purpose**: Set up Claude Code infrastructure for ANY new project to achieve consistent, high-quality output and prevent "losing the plot."

**Time Investment**: 2-4 hours initial setup | **Payoff**: Proven effective for 300k+ LOC projects

---

## Philosophy: The Holy Trinity

1. **Skills + Hooks** - Auto-activation ensures consistency
2. **Dev Docs System** - Prevents context loss across sessions
3. **Specialized Agents** - Automated reviews, planning, error resolution

**Critical Insight**: Skills alone DON'T work. You MUST use hooks to force auto-activation.

---

## Prerequisites

**Required**:
- Claude Code installed
- Project with version control (git)
- TypeScript/JavaScript for hooks (Claude Code requirement)
- Node.js installed (for hook scripts)

**Recommended**:
- Planning mode enabled
- 20x Max plan (or be mindful of context limits)

---

## Step 1: Core Directory Structure

Create this structure in your project root:

```bash
mkdir -p .claude/skills
mkdir -p .claude/skills/resources  # For progressive disclosure
mkdir -p .claude/agents
mkdir -p .claude/commands
mkdir -p .claude/hooks
mkdir -p dev/active              # Task tracking (survives compaction)
mkdir -p dev/archive             # Completed tasks (reference)
```

**Purpose**:
- `.claude/skills/` - Pattern libraries (< 500 lines each)
- `.claude/agents/` - Specialized assistants
- `.claude/commands/` - Slash commands (reusable prompts)
- `.claude/hooks/` - Automation (build checks, skill activation)
- `dev/active/` - Dev docs per task (plan/context/tasks)
- `dev/archive/` - Completed task reference

---

## Step 2: Skills System Setup

### 2.1 Create skill-rules.json

**Location**: `.claude/skill-rules.json`

**Purpose**: Defines when skills auto-activate (used by hooks)

```json
{
  "core-dev-guidelines": {
    "type": "domain",
    "enforcement": "suggest",
    "priority": "high",
    "promptTriggers": {
      "keywords": ["backend", "frontend", "api", "database", "test"],
      "intentPatterns": [
        "(create|add|implement).*?(feature|function|class|module)",
        "(how to|best practice).*?"
      ]
    },
    "fileTriggers": {
      "pathPatterns": ["src/**/*", "lib/**/*"],
      "contentPatterns": ["import\\s", "export\\s", "function\\s", "class\\s"]
    }
  }
}
```

**Customize**: Add patterns specific to your tech stack.

### 2.2 Create Your First Skill

**Critical Rule**: Main file MUST be < 500 lines (Anthropic best practice)

**File**: `.claude/skills/core-dev-guidelines.md`

```markdown
# Core Development Guidelines

**Auto-Activates**: When editing code files, implementing features, or asking about best practices

## Purpose

This skill provides coding standards, patterns, and best practices for this project.

## Tech Stack

- **Language**: [Your language - Rust, TypeScript, Python, etc.]
- **Framework**: [Your framework]
- **Testing**: [Your testing framework]
- **Build Tool**: [Your build tool]

## Critical Patterns

### 1. Error Handling

**DO**:
- Use proper error types (Result<T, E>, try/catch with specific errors)
- Provide context in error messages
- Log errors at appropriate levels

**DON'T**:
- Suppress errors silently
- Use generic "error" without context
- Expose sensitive data in error messages

[See resources/error-handling.md for detailed examples]

### 2. Testing

**Coverage Targets**:
- Core logic: 90%+
- Services/API: 80%+
- UI/Views: 60%+

**Test Pattern**:
[Language-specific test pattern here]

[See resources/testing-patterns.md for examples]

### 3. Code Organization

[Your project's module/file structure conventions]

### 4. Documentation

**Required**:
- All public APIs documented
- Complex algorithms explained inline
- Examples for non-obvious usage

## Quality Gates

Before marking work complete:
- [ ] All tests pass
- [ ] Code formatted (auto-formatter run)
- [ ] No linter warnings
- [ ] Documentation updated
- [ ] CHANGELOG updated (if applicable)

## References

- [Link to your project's docs]
- [Style guide URL]
- [Framework best practices]
```

### 2.3 Create Resource Files (Progressive Disclosure)

**File**: `.claude/skills/resources/error-handling.md`

```markdown
# Error Handling Patterns

[Detailed examples specific to your language/framework]

## Pattern 1: [Example]
...

## Pattern 2: [Example]
...
```

**Benefits**: Main skill stays < 500 lines, Claude only loads details when needed (40-60% token reduction).

---

## Step 3: Hooks System Setup

### 3.1 UserPromptSubmit Hook (Skill Auto-Activation)

**File**: `.claude/hooks/user-prompt-submit.ts`

**Purpose**: Injects skill reminder BEFORE Claude sees your prompt

```typescript
// .claude/hooks/user-prompt-submit.ts
import * as fs from 'fs';
import * as path from 'path';

interface SkillRule {
  type: string;
  enforcement: string;
  priority: string;
  promptTriggers: {
    keywords: string[];
    intentPatterns: string[];
  };
  fileTriggers?: {
    pathPatterns: string[];
    contentPatterns: string[];
  };
}

interface SkillRules {
  [skillName: string]: SkillRule;
}

export async function userPromptSubmit(params: {
  prompt: string;
  context?: any;
}): Promise<{ prompt: string }> {
  const { prompt } = params;

  // Load skill rules
  const rulesPath = path.join(process.cwd(), '.claude', 'skill-rules.json');
  if (!fs.existsSync(rulesPath)) {
    return { prompt }; // No rules, return unchanged
  }

  const rules: SkillRules = JSON.parse(fs.readFileSync(rulesPath, 'utf-8'));

  // Check which skills should activate
  const activatedSkills: string[] = [];

  for (const [skillName, rule] of Object.entries(rules)) {
    // Check keyword matches
    const hasKeyword = rule.promptTriggers.keywords.some((keyword) =>
      prompt.toLowerCase().includes(keyword.toLowerCase())
    );

    // Check intent patterns
    const hasIntent = rule.promptTriggers.intentPatterns.some((pattern) =>
      new RegExp(pattern, 'i').test(prompt)
    );

    if (hasKeyword || hasIntent) {
      activatedSkills.push(skillName);
    }
  }

  // Inject skill reminder if any activated
  if (activatedSkills.length > 0) {
    const skillList = activatedSkills.join(', ');
    const reminder = `\n\n🎯 SKILL ACTIVATION CHECK - Use the following skills: ${skillList}\n\n`;
    return { prompt: reminder + prompt };
  }

  return { prompt };
}
```

### 3.2 Stop Hook (Build Checker + Error Reminder)

**File**: `.claude/hooks/stop.ts`

**Purpose**: Runs AFTER Claude finishes responding

```typescript
// .claude/hooks/stop.ts
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

interface EditLog {
  files: string[];
  timestamp: number;
}

export async function stop(params: { response: string }): Promise<void> {
  // Read edit logs from post-tool-use hook
  const logPath = path.join(process.cwd(), '.claude', 'edit-log.json');
  if (!fs.existsSync(logPath)) {
    return; // No edits tracked
  }

  const editLog: EditLog = JSON.parse(fs.readFileSync(logPath, 'utf-8'));
  const editedFiles = editLog.files || [];

  if (editedFiles.length === 0) {
    return; // Nothing to check
  }

  // Determine build command based on project type
  let buildCommand = '';
  if (fs.existsSync('Cargo.toml')) {
    buildCommand = 'cargo check --message-format=json';
  } else if (fs.existsSync('package.json')) {
    buildCommand = 'npm run build || tsc --noEmit';
  } else if (fs.existsSync('go.mod')) {
    buildCommand = 'go build ./...';
  }
  // Add more project types as needed

  if (!buildCommand) {
    console.log('⚠️ No build command detected for this project type');
    return;
  }

  try {
    console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('🔍 BUILD CHECKER');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

    execSync(buildCommand, { stdio: 'inherit' });

    console.log('\n✅ Build check passed - no errors detected\n');
  } catch (error: any) {
    // Build failed
    const errorOutput = error.stdout?.toString() || error.stderr?.toString() || '';

    // Count errors (rough heuristic)
    const errorCount = (errorOutput.match(/error/gi) || []).length;

    if (errorCount >= 5) {
      console.log(`\n❌ ${errorCount} errors detected`);
      console.log('💡 Recommendation: Use /build-and-fix or launch error-resolver agent\n');
    } else if (errorCount > 0) {
      console.log(`\n⚠️ ${errorCount} error(s) found - please review and fix\n');
    }
  }

  // Error handling reminder (gentle, non-blocking)
  const hasRiskyPatterns = editedFiles.some((file) => {
    if (!fs.existsSync(file)) return false;
    const content = fs.readFileSync(file, 'utf-8');
    return (
      content.includes('try') ||
      content.includes('catch') ||
      content.includes('async') ||
      content.includes('await') ||
      content.includes('unwrap()') ||
      content.includes('panic!')
    );
  });

  if (hasRiskyPatterns) {
    console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('📋 ERROR HANDLING SELF-CHECK');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
    console.log('⚠️  Risky patterns detected in edited files\n');
    console.log('   ❓ Did you add proper error handling?');
    console.log('   ❓ Are panics/unwraps justified with comments?');
    console.log('   ❓ Do async functions handle cancellation?\n');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  }

  // Clear edit log for next session
  fs.unlinkSync(logPath);
}
```

### 3.3 Post-Tool-Use Hook (Edit Tracker)

**File**: `.claude/hooks/post-tool-use.ts`

**Purpose**: Tracks which files Claude edited (used by stop hook)

```typescript
// .claude/hooks/post-tool-use.ts
import * as fs from 'fs';
import * as path from 'path';

export async function postToolUse(params: {
  toolName: string;
  arguments: any;
}): Promise<void> {
  const { toolName, arguments: args } = params;

  // Track Edit/Write/MultiEdit operations
  if (!['Edit', 'Write', 'MultiEdit'].includes(toolName)) {
    return;
  }

  const logPath = path.join(process.cwd(), '.claude', 'edit-log.json');

  let editLog = { files: [], timestamp: Date.now() };
  if (fs.existsSync(logPath)) {
    editLog = JSON.parse(fs.readFileSync(logPath, 'utf-8'));
  }

  // Extract file path from arguments
  const filePath = args.file_path || args.path;
  if (filePath && !editLog.files.includes(filePath)) {
    editLog.files.push(filePath);
  }

  editLog.timestamp = Date.now();

  // Ensure .claude directory exists
  const claudeDir = path.join(process.cwd(), '.claude');
  if (!fs.existsSync(claudeDir)) {
    fs.mkdirSync(claudeDir, { recursive: true });
  }

  fs.writeFileSync(logPath, JSON.stringify(editLog, null, 2));
}
```

**⚠️ Do NOT implement Prettier hook** - Causes 160k token bloat from system reminders (learned the hard way).

---

## Step 4: Agents Creation

### 4.1 Agent Template

**Critical**: Agents need VERY specific roles and MUST return structured output.

**File**: `.claude/agents/strategic-plan-architect.md`

```markdown
---
name: strategic-plan-architect
description: Creates comprehensive implementation plans BEFORE coding starts. Use for any feature requiring > 3 files or > 100 LOC.\n\n**Examples**:\n- "I want to add user authentication"\n- "Refactor the API layer"\n- "Implement caching system"
model: sonnet
color: purple
---

You are a Strategic Planning Architect. Your role is to create detailed, structured implementation plans BEFORE code is written.

## Your Output Format (MANDATORY)

Return plans in this EXACT structure:

```markdown
# Implementation Plan: [Feature Name]

## Executive Summary
- **Complexity**: Small | Medium | Large
- **Estimated LOC**: ~X lines
- **Files Affected**: Y files
- **Timeline**: Z hours

## Phases

### Phase 1: [Name]
**Goal**: [What this accomplishes]
**Tasks**:
1.1. [Specific task]
1.2. [Specific task]

**Success Criteria**:
- [ ] [Testable criterion]

**Risks**:
- [Risk] → Mitigation: [approach]

### Phase 2-N: [Continue pattern]

## Technical Decisions
- **Architecture**: [Why this approach]
- **Dependencies**: [What's needed]
- **Testing Strategy**: [How to verify]

## Rollback Plan
1. [How to revert if fails]
```

## Key Principles

- Break large tasks into 3-5 phases
- Each phase independently testable
- Identify risks with mitigations
- Provide concrete rollback plan
- NO vague "should work" - be specific
```

### 4.2 Essential Agents (Minimum)

Create these for any project:

1. **`strategic-plan-architect`** - Planning (shown above)
2. **`code-reviewer`** - Reviews between phases
3. **`error-resolver`** - Fixes build errors systematically
4. **`test-coverage-analyzer`** - Verifies coverage targets

**Customize** based on your tech stack (e.g., `rust-security-auditor` for Rust, `typescript-lint-fixer` for TS).

---

## Step 5: Slash Commands

### 5.1 /dev-docs (Create Dev Docs)

**File**: `.claude/commands/dev-docs.md`

```markdown
You are a documentation specialist creating dev docs for a task.

**Your Mission**: Create three files in `dev/active/[task-name]/`:

1. **`[task-name]-plan.md`** - The implementation plan (phases, tasks, timeline)
2. **`[task-name]-context.md`** - Key files, decisions, module boundaries
3. **`[task-name]-tasks.md`** - Checklist with `[ ]` items

**Format**:

# [task-name]-plan.md
## Feature: [Name]
### Phase 1: [Name]
- Task 1.1
- Task 1.2
...

# [task-name]-context.md
## Key Files
- `path/to/file.ext` - Purpose, line refs

## Decisions Made
- [Why X over Y]

## Last Updated: YYYY-MM-DD HH:MM

# [task-name]-tasks.md
## Implementation Checklist
- [ ] Task 1
- [ ] Task 2
...

**After creating files, respond**: "Dev docs created in `dev/active/[task-name]/`. Ready to implement."
```

### 5.2 /update-dev-docs (Before Compaction)

**File**: `.claude/commands/update-dev-docs.md`

```markdown
Update dev docs in `dev/active/[task-name]/` before conversation compaction.

**Actions**:
1. Mark completed tasks in `tasks.md` with `[x]`
2. Add new tasks discovered during implementation
3. Update `context.md` with:
   - New architectural decisions
   - Files modified (with line refs)
   - Next steps after compaction
4. Update "Last Updated" timestamp

**Response Format**:
"Dev docs updated:
- X tasks completed
- Y new tasks added
- Next: [Brief description of what to do after compaction]"
```

### 5.3 /code-review

**File**: `.claude/commands/code-review.md`

```markdown
Launch the code-reviewer agent to review recent changes for:
- Code quality issues
- Missing tests
- Documentation gaps
- Architecture violations

**Return**: Prioritized list of issues (Critical/High/Medium/Low) with specific fixes.
```

---

## Step 6: CLAUDE.md Template

**File**: `CLAUDE.md` (project root)

**Critical**: Keep this < 200 lines. Move patterns to skills, architecture to docs.

```markdown
# CLAUDE.md

This file provides guidance to Claude Code when working with code in this repository.

## Quick Commands

```bash
# Build
[your build command]

# Test
[your test command]

# Lint
[your lint command]

# Format
[your format command]
```

## Development Workflow

### Starting Large Tasks (MANDATORY for > 3 files or > 100 LOC)

1. **Enter Planning Mode** or run `/strategic-plan [task-name]`
2. **Review plan thoroughly** - Catch misunderstandings early (40%+ of issues)
3. **Create dev docs**: `/dev-docs [task-name]`
   - Creates `dev/active/[task-name]/` with plan/context/tasks
4. **Declare in CHANGELOG** (if applicable): Add to `🚧 IN PROGRESS`
5. **Implement in sections**: "Only do Phase 1, then stop for review"
6. **Review between phases**: `/code-review`
7. **Before compaction**: `/update-dev-docs [task-name]`
8. **After compaction**: "Read all files in `dev/active/[task-name]/` and continue"

### Resuming Tasks

If you see files in `dev/active/`:
1. Read plan.md, context.md, tasks.md
2. Check "Last Updated" timestamp
3. Continue from last checkpoint in tasks.md

## Skills (Auto-Activate via Hooks)

- **core-dev-guidelines**: Coding standards, patterns, testing
- [Add your other skills here]

**How it works**: Hooks inject skill reminders before you see prompts. No manual invocation needed.

## Agents (Specialized Assistants)

- **strategic-plan-architect**: Creates implementation plans
- **code-reviewer**: Reviews code between phases
- **error-resolver**: Fixes build errors systematically
- [Add your other agents here]

## Tech Stack

- **Language**: [Your language + version]
- **Framework**: [Your framework + version]
- **Database**: [Your database if applicable]
- **Testing**: [Your test framework]
- **Build**: [Your build tool]

## Project-Specific Patterns

[Add ONLY patterns unique to THIS project, not general best practices]

## Common Pitfalls

1. [Project-specific gotcha]
2. [Another project-specific issue]

## Documentation

- **Architecture**: [Link to architecture docs]
- **API Docs**: [Link if applicable]
- **Contributing**: [Link to contributing guide]

---

**Skills handle "how to write code"**
**CLAUDE.md handles "how THIS project works"**
```

---

## Step 7: First Session Workflow

### 7.1 Initialize Repository

```bash
# In your project root
git init  # If not already a repo

# Add .gitignore entries
echo ".claude/edit-log.json" >> .gitignore
echo "dev/active/*" >> .gitignore  # Optional: keep local-only
echo ".env" >> .gitignore
```

### 7.2 Test Hooks

Create a test file and verify hooks work:

```bash
# Create a test file
echo "console.log('test');" > test.js

# In Claude Code, edit the file
# Verify:
# - UserPromptSubmit hook injects skill reminder
# - Stop hook runs build check
# - Edit log created in .claude/edit-log.json
```

### 7.3 Create First Skill

Use the template from Step 2.2, customize for your tech stack.

### 7.4 Test Planning Workflow

```
user: "I want to add [simple feature]"

1. Claude should enter planning mode (or you run /strategic-plan)
2. Review plan
3. Run /dev-docs [feature-name]
4. Verify three files created in dev/active/[feature-name]/
5. Implement Phase 1 only
6. Run /code-review
7. Continue or fix issues
```

---

## Step 8: Maintenance & Iteration

### 8.1 When to Update Skills

**Add new patterns when**:
- You've implemented a pattern 3+ times
- You catch Claude repeating the same mistake
- You establish a new convention

**Keep main file < 500 lines**:
- Move details to `resources/` files
- Use progressive disclosure

### 8.2 When to Create New Agents

**Create an agent when**:
- You're repeating the same prompt > 5 times
- A task requires specific domain knowledge
- You need consistent output format

**Agent ROI Test**:
- Time to create: X hours
- Time saved per use: Y minutes
- If used > (X hours / Y minutes) times → Worth it

### 8.3 Hook Debugging

If hooks aren't working:

```bash
# Check hook files exist
ls .claude/hooks/

# Check permissions (Unix)
chmod +x .claude/hooks/*.ts

# Check logs
# Claude Code shows hook errors in terminal
```

### 8.4 Skill Activation Debugging

If skills don't activate:

1. Check `skill-rules.json` has correct patterns
2. Verify `user-prompt-submit.ts` is reading rules file
3. Test with explicit keywords from rules
4. Check Claude Code terminal for hook errors

---

## Language-Specific Customizations

### For Rust Projects

**Build command in stop.ts**:
```typescript
buildCommand = 'cargo check --message-format=json';
```

**Core skill patterns**:
- Error handling: `Result<T, E>`, no `unwrap()` in src/
- Testing: `#[cfg(test)]` modules, `#[tokio::test]`
- Documentation: `///` and `//!` rustdoc
- Async: Send bounds, tokio runtime

### For TypeScript/JavaScript Projects

**Build command in stop.ts**:
```typescript
buildCommand = 'npm run build || tsc --noEmit';
```

**Core skill patterns**:
- Error handling: try/catch with specific errors
- Testing: Jest/Vitest patterns
- Documentation: JSDoc/TSDoc
- Async: Promise handling, async/await

### For Python Projects

**Build command in stop.ts**:
```typescript
buildCommand = 'python -m py_compile $(find . -name "*.py")';
```

**Core skill patterns**:
- Error handling: try/except with specific exceptions
- Testing: pytest patterns
- Documentation: docstrings (Google/NumPy style)
- Type hints: mypy validation

### For Go Projects

**Build command in stop.ts**:
```typescript
buildCommand = 'go build ./...';
```

**Core skill patterns**:
- Error handling: `if err != nil` with wrapped errors
- Testing: table-driven tests
- Documentation: godoc comments
- Concurrency: goroutines, channels

---

## Verification Checklist

Before starting development, verify:

**Infrastructure**:
- [ ] Directory structure created (`.claude/`, `dev/`)
- [ ] `skill-rules.json` created
- [ ] At least 1 skill created (< 500 lines)
- [ ] Hooks created (user-prompt-submit, stop, post-tool-use)
- [ ] At least 2 agents created (planner + reviewer)
- [ ] At least 2 slash commands created (/dev-docs, /code-review)
- [ ] CLAUDE.md created (< 200 lines)

**Testing**:
- [ ] Hooks run without errors
- [ ] Skills auto-activate on relevant prompts
- [ ] Agents launch correctly
- [ ] Slash commands work
- [ ] Dev docs system creates files correctly

**Ready to Code**:
- [ ] First feature planned via planning mode
- [ ] Dev docs created for first feature
- [ ] Code review agent tested
- [ ] Build checker hook validated

---

## Common Mistakes to Avoid

**❌ Don't**:
- Create skills > 500 lines (defeats progressive disclosure)
- Skip planning mode for large features (causes "losing the plot")
- Forget to update dev docs before compaction (context loss)
- Create vague agents that return "I fixed it!" (not actionable)
- Put general patterns in CLAUDE.md (belongs in skills)
- Use Prettier hook (160k token bloat - learned the hard way)

**✅ Do**:
- Keep skills < 500 lines, use resource files for details
- ALWAYS plan features > 3 files or > 100 LOC
- Update dev docs before every compaction
- Give agents specific roles and structured output formats
- Separate "how to code" (skills) from "how THIS project works" (CLAUDE.md)
- Review plans thoroughly (catches 40%+ of issues)

---

## Success Metrics

**After Setup, You Should See**:
- ✅ Skills auto-activate without manual prompting
- ✅ Build errors caught immediately after Claude responds
- ✅ No "lost the plot" moments across sessions
- ✅ Consistent code patterns throughout codebase
- ✅ Fewer manual reviews needed (hooks catch issues)
- ✅ Faster context recovery after compaction (dev docs)

**Proven Results** (from 6 months production use):
- 300k LOC rewritten solo
- Consistent quality throughout
- Zero errors left behind
- Seamless continuation across sessions
- 40-60% token reduction (progressive disclosure)

---

## Next Steps

1. **Set up infrastructure** (Steps 1-6)
2. **Test with small feature** (verify hooks, skills, agents work)
3. **Iterate on skills** (add patterns as you discover them)
4. **Create domain-specific agents** (as needed)
5. **Refine based on your workflow** (add slash commands, update rules)

---

## References

**Original Post**: [Claude Code is a Beast - Reddit](https://www.reddit.com/r/ClaudeCode/comments/1oivs81/claude_code_is_a_beast_tips_from_6_months_of/)

**Example Repo**: https://github.com/diet103/claude-code-infrastructure-showcase

**Anthropic Skill Docs**: Check official docs for < 500 line guidance and progressive disclosure

---

**This guide is based on 6 months of production use rewriting 300k LOC. All patterns are verified and proven.**

**Last Updated**: 2025-11-08
**Verified**: Principles from "Claude Code is a Beast" Reddit post (diet103)
