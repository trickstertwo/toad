---
description: Initialize Claude Code automation infrastructure for a new project (hooks, skills, agents, dev docs)
---

You are helping initialize the Claude Code automation infrastructure for a new project.

**Your Task**: Launch the `project-initializer` agent to guide the user through automated setup.

**Important**: Use the Task tool to launch the agent:

```
Task tool with:
- subagent_type: "project-initializer"
- description: "Initialize project automation"
- prompt: "Initialize Claude Code automation infrastructure for this project. Ask focused questions about the tech stack, then generate all required files (hooks, skills, agents, directory structure). Follow the initialization workflow defined in your agent instructions."
```

The agent will:
1. Ask 5-10 focused questions about the tech stack
2. Generate ALL automation files customized to their stack
3. Create directory structure
4. Test that hooks work
5. Provide verification steps

**Do NOT manually create files** - let the agent handle everything for consistency.
