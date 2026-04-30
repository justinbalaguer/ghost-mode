---
name: ghost_off
description: Disable ghost-read protocol for this session. Stop skeletonizing files; read normally.
---

# Disable Ghost Mode

Ghost-read is now OFF for this session.

1. **Protocol:** Stop using `ghost.py` to skeletonize files. Use the `Read` tool normally for all files regardless of size. Do NOT invoke the `ghost-mode:ghost-read` skill for any reason until the user explicitly re-enables it.
2. **Flag:** Run this command to remove the statusline badge:
   `rm -f "${CLAUDE_CONFIG_DIR:-$HOME/.claude}/.ghost-active"`
3. **Confirm:** Tell the user ghost-read is disabled. Badge is removed. To re-enable, run `/ghost_read`.

**Slash Command:** Users trigger this by typing `/ghost_off`.
