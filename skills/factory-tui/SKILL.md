---
name: factory-tui
description: >
  Set up and use factory-tui, a popup tmux browser (tree of sessions
  and windows, snapshot preview, Enter to jump). Load when the
  operator says factory-tui, F1 browser, tmux factory tree, project
  the factory, fold window names, write config.toml, or "point another
  operator at factory-tui". Also load for first-run Nix install, tmux
  bind, or when factory-tui --dump is a flat session list that should
  be a project/milestone/epic tree. Triggers: factory-tui, F1,
  projection.toml, config.toml, tmux census, fold windows,
  nix profile add factory-tui.
---

# factory-tui setup

A tmux popup: left = tree, right = snapshot, Enter jumps. Default
tree is session → window. A **local** file may fold names. The crate
has no operator's product names.

If you are not in this repository, read this file from
https://github.com/lambdasistemi/factory-tui/blob/main/skills/factory-tui/SKILL.md
and the example from
https://github.com/lambdasistemi/factory-tui/blob/main/examples/projection.toml

## Stop — first run

If `factory-tui` is missing from PATH, or
`~/.config/factory-tui/config.toml` does not exist, do **not** guess
this host's factory. Interview, then write files.

Ask only what you cannot see from a census:

1. Popup key (default: `F1` and prefix+`S`)
2. Which sessions are infrastructure (not a product), if any
3. Short session names that should display as a longer product name
4. Whether window names already use `ms<N>`, `-e<id>-`, `-t<id>-`
   (if yes, start from `examples/projection.toml` and only add
   aliases)

Never commit `~/.config/factory-tui/config.toml`. Never copy another
operator's aliases into the repo.

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
   tmux list-windows -a -F '#{session_name}	#{window_name}'
   ```

3. **Write** `$FACTORY_TUI_CONFIG` if set, else
   `~/.config/factory-tui/config.toml`. Schema:
   [references/config.md](references/config.md). Start from
   `examples/projection.toml` when the names already match that
   grammar; otherwise write `[[rule]]` regexes from the census.
4. **Verify** `factory-tui --dump` folds the way the operator asked.
   Unmatched windows must still appear under their session.
5. **Bind** (idempotent — skip if already present):

   ```tmux
   bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
   bind-key S     display-popup -E -w 90% -h 90% factory-tui
   ```

   Then `tmux source-file` the operator's tmux.conf.
6. Tell the operator to press **F1**.

## Using it after setup

- F1 (or prefix+S): popup browser
- `--dump`: same tree, no UI
- Missing config file: session → window, not an error
- Reload config by restarting the popup (quit with `q` and open again)
