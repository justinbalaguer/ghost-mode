# ghost-mode

A Claude Code plugin that uses a Rust-powered Tree-sitter engine to skeletonize source files — extracting function and class signatures while hiding bodies. Saves up to 90% of input tokens when navigating large codebases.

## How It Works

For any file over 100 lines, Claude runs `ghost signature` to get a structural map:

```
// GHOST SIGNATURE: server.ts (bodies hidden)
Line 10: function handleRequest(req, res) { ... }
Line 45: class AuthMiddleware { ... }
Line 78: interface TokenPayload { ... }
```

Claude then fetches only the function bodies it needs:

```bash
ghost read server.ts --lines 10-32
```

**Supported languages:** Python, JavaScript, TypeScript, TSX

## Requirements

- [Claude Code](https://claude.ai/code) with plugin support
- [Rust toolchain](https://rustup.rs) (for building the binary)
- Node.js (for the session hook)

No Python. No pip packages.

## Installation

**1. Install the plugin:**

```bash
claude plugin install https://github.com/justinbalaguer/ghost-mode
```

**2. Build the ghost engine:**

```bash
cd ~/.claude/plugins/marketplaces/ghost-mode
bash build.sh
```

**3. Restart Claude Code.** The `[👻 GHOST]` statusline badge confirms ghost-mode is active.

### Manual Installation

```bash
git clone https://github.com/justinbalaguer/ghost-mode
cd ghost-mode
bash build.sh
claude plugin install ./
```

### Static Binary (Linux, optional)

For a fully portable binary with no shared library dependencies:

```bash
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl
```

## Usage

Ghost-read is **on by default** when the plugin is installed. Toggle it per session with slash commands:

| Command | Effect |
|---------|--------|
| `/ghost` or `/ghost_read` | Enable ghost-read, show `[👻 GHOST]` badge |
| `/ghost_off` | Disable ghost-read, remove badge, read files normally |

**Statusline badge** — when active, your Claude Code statusline shows:

```
[👻 GHOST]
```

If the badge is missing after install, Claude will prompt you to add the statusline config to `~/.claude/settings.json`:

```json
"statusLine": {
  "type": "command",
  "command": "bash \"/path/to/ghost-mode/hooks/ghost-statusline.sh\""
}
```

## Ghost CLI

The `ghost` binary can also be used directly:

```bash
# Map file structure
ghost signature src/main.rs

# Read a specific range
ghost read src/main.rs --lines 42-67
```

## Updating

```bash
claude plugin update ghost-mode
cd ~/.claude/plugins/marketplaces/ghost-mode && bash build.sh
```

## Uninstalling

```bash
claude plugin uninstall ghost-mode
```

## Configuration

No configuration required. The plugin activates via a `SessionStart` hook.

| File Type | Constructs Skeletonized |
|-----------|------------------------|
| `.py`     | functions, classes |
| `.js`     | functions, classes, methods |
| `.ts`     | functions, classes, methods, interfaces, type aliases |
| `.tsx`    | functions, classes, methods, interfaces, type aliases |

Files with unsupported extensions are read in full as normal.

## Troubleshooting

**`ghost: command not found`** — Run `bash build.sh` from the plugin directory. Ensure `target/release/ghost` exists.

**`failed to load language`** — Rust toolchain version issue. Run `rustup update stable` and rebuild.

**Badge not showing** — Add the `statusLine` config snippet to `~/.claude/settings.json` (Claude will suggest the exact snippet on first session start).

**Plugin not activating** — Confirm with `claude plugin list` and restart Claude Code after install.

## License

MIT — [Justin Balaguer](https://github.com/justinbalaguer)
