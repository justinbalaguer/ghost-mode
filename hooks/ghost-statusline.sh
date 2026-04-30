#!/bin/bash
# ghost-mode — statusline badge script for Claude Code
# Reads the ghost mode flag file and outputs a colored badge.
#
# Usage (via combined statusline or standalone):
#   "statusLine": { "type": "command", "command": "bash /path/to/ghost-statusline.sh" }

FLAG="${CLAUDE_CONFIG_DIR:-$HOME/.claude}/.ghost-active"

# Refuse symlinks — defense against flag-file injection via terminal escape sequences
[ -L "$FLAG" ] && exit 0
[ ! -f "$FLAG" ] && exit 0

# Cyan ghost badge
printf '\033[38;5;51m[\360\237\221\273 GHOST]\033[0m'
