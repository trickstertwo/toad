// .claude/hooks/user-prompt-submit.ts
import * as fs from 'fs';
import * as path from 'path';

export async function userPromptSubmit(params: {
  prompt: string;
  context?: any;
}): Promise<{ prompt: string }> {
  const skillRulesPath = path.join(process.cwd(), '.claude', 'skill-rules.json');

  if (!fs.existsSync(skillRulesPath)) {
    return { prompt: params.prompt };
  }

  const rules = JSON.parse(fs.readFileSync(skillRulesPath, 'utf-8'));
  const activatedSkills: string[] = [];

  for (const [skillName, rule] of Object.entries(rules)) {
    const r = rule as any;

    // Check keyword triggers
    const hasKeyword = r.promptTriggers.keywords.some((k: string) =>
      params.prompt.toLowerCase().includes(k.toLowerCase())
    );

    // Check intent pattern triggers
    const hasIntent = r.promptTriggers.intentPatterns.some((p: string) =>
      new RegExp(p, 'i').test(params.prompt)
    );

    if (hasKeyword || hasIntent) {
      activatedSkills.push(skillName);
    }
  }

  if (activatedSkills.length > 0) {
    const reminder = `\n\nSKILL ACTIVATION CHECK\n\nBased on your prompt, these skills may be relevant:\n${activatedSkills.map(s => `- ${s}`).join('\n')}\n\nConsider using them if they apply to this task.\n\n`;
    return { prompt: reminder + params.prompt };
  }

  return { prompt: params.prompt };
}
