// .claude/hooks/post-tool-use.ts
import * as fs from 'fs';
import * as path from 'path';

export async function postToolUse(params: {
  toolName: string;
  arguments: any;
}): Promise<void> {
  const { toolName, arguments: args } = params;

  if (!['Edit', 'Write', 'MultiEdit'].includes(toolName)) {
    return;
  }

  const logPath = path.join(process.cwd(), '.claude', 'edit-log.json');
  const claudeDir = path.join(process.cwd(), '.claude');

  if (!fs.existsSync(claudeDir)) {
    fs.mkdirSync(claudeDir, { recursive: true });
  }

  let editLog = { files: [], timestamp: Date.now() };
  if (fs.existsSync(logPath)) {
    editLog = JSON.parse(fs.readFileSync(logPath, 'utf-8'));
  }

  const filePath = args.file_path || args.path;
  if (filePath && !editLog.files.includes(filePath)) {
    editLog.files.push(filePath);
  }

  editLog.timestamp = Date.now();
  fs.writeFileSync(logPath, JSON.stringify(editLog, null, 2));
}
