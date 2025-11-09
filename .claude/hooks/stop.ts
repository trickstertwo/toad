// .claude/hooks/stop.ts
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

export async function stop(params: { response: string }): Promise<void> {
  const logPath = path.join(process.cwd(), '.claude', 'edit-log.json');

  if (!fs.existsSync(logPath)) {
    return; // No edits this session
  }

  const editLog = JSON.parse(fs.readFileSync(logPath, 'utf-8'));
  const editedFiles = editLog.files || [];

  if (editedFiles.length === 0) {
    return;
  }

  console.log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('Build Check (Zero Errors Left Behind)');
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

  // Run cargo check
  try {
    console.log('Running: cargo check\n');
    const output = execSync('cargo check', {
      encoding: 'utf-8',
      stdio: 'pipe',
      timeout: 30000
    });

    // Check for errors/warnings in output
    const lines = output.split('\n');
    const errorLines = lines.filter(l =>
      l.includes('error') || l.includes('Error') ||
      l.includes('warning') || l.includes('Warning')
    );

    if (errorLines.length > 0 && errorLines.length < 5) {
      console.log('Issues detected:\n');
      errorLines.forEach(line => console.log(`   ${line}`));
      console.log('\nFix these before continuing\n');
    } else if (errorLines.length >= 5) {
      console.log(`${errorLines.length} issues detected`);
      console.log('Run this to see all errors:\n');
      console.log('   cargo check\n');
    } else {
      console.log('Build passed - no errors detected\n');
    }
  } catch (error: any) {
    console.log('Build failed\n');
    const output = error.stdout || error.stderr || '';
    const lines = output.split('\n').slice(0, 10);
    lines.forEach((line: string) => console.log(`   ${line}`));
    console.log('\nFix errors before continuing\n');
  }

  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

  // Clear edit log
  fs.unlinkSync(logPath);
}
