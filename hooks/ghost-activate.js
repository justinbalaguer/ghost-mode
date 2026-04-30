#!/usr/bin/env node
// ghost-mode — SessionStart hook
// Writes flag file so the statusline can show [👻 GHOST] badge.
// Ghost-read is always active when this plugin is loaded.

const fs = require('fs');
const path = require('path');
const os = require('os');

const claudeDir = process.env.CLAUDE_CONFIG_DIR || path.join(os.homedir(), '.claude');
const flagPath = path.join(claudeDir, '.ghost-active');
const settingsPath = path.join(claudeDir, 'settings.json');

// Refuse to follow symlinks — same defense as ghost-statusline.sh
try {
  const stat = fs.lstatSync(flagPath);
  if (stat.isSymbolicLink()) process.exit(0);
} catch (e) {
  // File doesn't exist yet — fine
}

try {
  fs.writeFileSync(flagPath, 'active', { mode: 0o600 });
} catch (e) {}

// Detect missing statusline config and nudge Claude to set it up.
// Uses CLAUDE_PLUGIN_ROOT so the path is correct for any user's install.
try {
  let hasStatusline = false;
  if (fs.existsSync(settingsPath)) {
    const settings = JSON.parse(fs.readFileSync(settingsPath, 'utf8'));
    if (settings.statusLine) hasStatusline = true;
  }

  if (!hasStatusline) {
    const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT || __dirname.replace(/[/\\]hooks$/, '');
    const scriptPath = path.join(pluginRoot, 'hooks', 'ghost-statusline.sh');
    const command = `bash "${scriptPath}"`;
    const snippet = `"statusLine": { "type": "command", "command": ${JSON.stringify(command)} }`;

    process.stdout.write(
      `GHOST-MODE STATUSLINE SETUP NEEDED: The ghost-mode plugin includes a statusline badge [👻 GHOST]. ` +
      `It is not configured yet. ` +
      `To enable, add this to ${settingsPath}: ${snippet} ` +
      `Proactively offer to set this up for the user on first interaction.`
    );
    process.exit(0);
  }
} catch (e) {
  // Silent fail — don't block session start
}

process.stdout.write('');
process.exit(0);
