---
name: ghost_read
description: Activate ghost-read skeletonization for this session. Invoke ONLY when the user explicitly runs /ghost or /ghost_read. Do NOT auto-trigger for file reading, analysis, or any other task.
---

# Ghost Mode Protocol

**Activation only.** This skill must only run when the user explicitly types `/ghost` or `/ghost_read`. Do not invoke for general file reading or analysis tasks.

Once activated:

1. **Signature scan:** For any file over 100 lines, use the `Bash` tool to run:
   `${CLAUDE_PLUGIN_ROOT}/target/release/ghost signature <path_to_file>`
   Output is `Line N: <signature>` — use line numbers to map architecture.

2. **Selective read:** Fetch only the logic blocks you need:
   `${CLAUDE_PLUGIN_ROOT}/target/release/ghost read <path_to_file> --lines <start>-<end>`

3. **Goal:** 90% input token savings via structural analysis before targeted reads.

4. **Statusline:** Confirm `[👻 GHOST]` badge is active.

5. **Confirm:** Tell the user ghost-read is now active. To disable, run `/ghost_off`.

**Slash Command:** Users trigger this by typing `/ghost` or `/ghost_read`.
