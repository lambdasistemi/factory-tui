---
name: factory-tui
description: >
  Set up and use factory-tui, a popup tmux browser (tree of sessions,
  windows and panes, snapshot preview, Enter to jump). Load when the
  operator says factory-tui, F1 browser, tmux factory tree, rename
  window rows, write config.toml, or "point another operator at
  factory-tui". Also load for first-run Nix install, tmux bind, or when
  factory-tui --dump does not show every live pane. Triggers:
  factory-tui, F1, config.toml, reinterpreter, tmux census,
  nix profile add factory-tui.
---

# factory-tui setup

A tmux popup: left = tree, right = snapshot, Enter jumps. The tree is
tmux itself — session → window → pane. A **local** file may rewrite
what a row displays; nothing may change the shape of the tree. The
crate has no operator's product names.

If you are not in this repository, read this file from
https://github.com/lambdasistemi/factory-tui/blob/main/skills/factory-tui/SKILL.md
and the example from
https://github.com/lambdasistemi/factory-tui/blob/main/examples/config.toml

## Stop — first run

If `factory-tui` is missing from PATH, or
`~/.config/factory-tui/config.toml` does not exist, do **not** guess
this host's naming. Interview, then write files.

Ask only what you cannot see from a census:

1. Popup key (default: `F1` and prefix+`S`)
2. Which sessions are infrastructure (not a product), if any
3. Which cryptic session, window or pane names should read as
   something else, and what

A config file is optional: the raw tree is already correct and
complete, and reinterpreters only make it easier to read. Never commit
`~/.config/factory-tui/config.toml`, and never copy another operator's
labels into the repo.

## Procedure

1. **Install with Nix** so `command -v factory-tui` succeeds:

   ```
   nix profile add github:lambdasistemi/factory-tui
   ```

   Pin a release with `github:lambdasistemi/factory-tui/v0.1.0`.
   A checkout uses `nix build .#cli` (do not bind `target/release`).
   On NixOS, add the flake's `packages.<system>.default` to
   `environment.systemPackages`.

   Only if Nix is not available: Homebrew
   (`brew tap lambdasistemi/tap && brew install factory-tui`) or a
   binary from
   https://github.com/lambdasistemi/factory-tui/releases/latest

2. **Census** (must be inside tmux):

   ```
   skills/factory-tui/scripts/census
   factory-tui --dump
   ```

   If `census` is not in the worktree, run:

   ```
   tmux list-panes -a -F '#{session_name}	#{window_name}	#{pane_title}'
   ```

3. **Write** `$FACTORY_TUI_CONFIG` if set, else
   `~/.config/factory-tui/config.toml`. Schema:
   [references/config.md](references/config.md). Start from
   `examples/config.toml` and replace its example patterns with ones
   built from the census.
4. **Verify** every live pane is reachable — these two counts must
   agree — and that the rows the operator asked about now read the way
   they wanted. A row nothing matched stays raw, which is correct.

   ```
   tmux list-panes -a -F '#{pane_id}' | sort -u | wc -l
   factory-tui --dump | grep -cE '\[pane=[^]]+\]'
   ```
5. **Bind** (idempotent — skip if already present):

   ```tmux
   bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
   bind-key S     display-popup -E -w 90% -h 90% factory-tui
   ```

   Then `tmux source-file` the operator's tmux.conf.
6. Tell the operator to press **F1**.

## Using it after setup

- F1 (or prefix+S): popup browser
- `--dump`: same tree, no UI, with `[window=…]` / `[pane=…]` markers
- Missing config file: the raw tree, not an error
- Reload config by restarting the popup (quit with `q` and open again)
- A pane row jumps to that exact pane; `Tab` cycles the previewed pane
  of a multi-pane window
