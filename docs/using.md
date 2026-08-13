# Using factory-tui

## Bind and keys

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

| Key / gesture | Action |
|---|---|
| `j` `k` / arrows | move |
| `h` `l` / `Space` | collapse / expand |
| `Enter` / double-click | jump and quit |
| `Tab` / `[` `]` | cycle the previewed pane of a multi-pane window |
| `r` | refresh |
| `q` / `Esc` | quit |

`factory-tui --dump` prints the tree with no UI.

## Version and build provenance

`factory-tui --version` prints one line and exits. It needs no tmux
server, no terminal, and no config file:

```
factory-tui 0.1.0 (revision 40c0c518992af708b9e140bd846a3d073fb9799c)
```

| Field | Meaning |
|---|---|
| version | The product version, from `Cargo.toml`. The Nix package and every release artifact carry this same value. |
| revision | The source the binary was built from. |

The popup shows that same line at the right of its bottom chrome, so a
running window and the command always agree.

### What the revision can say

| Shown | Means |
|---|---|
| `<40 hex digits>` | An exact commit. The build came from clean flake source. |
| `<40 hex digits>-dirty` | That commit, plus uncommitted local edits. The binary is *not* that commit. |
| `unknown` | The build supplied no source metadata — for example a bare `cargo build` outside Nix. |

The revision comes from the flake's own source metadata, fixed when Nix
evaluates the flake. The build runs no `git` process, reads no clock, and
makes no network call, so rebuilding one revision reproduces one identity.
The tradeoff is the last row of that table: a build path that never gave
Nix a revision cannot be given one after the fact, and factory-tui says
`unknown` rather than guessing a commit.

Every run of CI reconciles all of it against two authorities — `Cargo.toml`
for the version, the flake source for the revision. The Nix package
metadata, the release and dev artifact names, and the lines the shipped
glibc and musl binaries print are each checked against those, so a build
whose artifacts disagree with its manifest never gets a green run.

## The tree

The tree is tmux: session → window → pane. Every live pane appears
exactly once.

A window with **one** pane *is* that pane: no child row, and selecting
it previews and jumps there. A window with **several** panes has one
child row per pane, each previewing and jumping to exactly that pane;
selecting the window row keeps the pane you were watching, or its
active one.

Sessions matching `[sessions] infra` are tagged `[infra]`. `[status]`
classifies a window from pane command names (`running`, `idle`) or a
`parked_substring`.

Config path: `$FACTORY_TUI_CONFIG`, else
`~/.config/factory-tui/config.toml`. A missing or empty file is not an
error, and means the raw tree.

## Reinterpreters

Configuration is **label-only**. Copy the generic example and edit it:

https://github.com/lambdasistemi/factory-tui/blob/main/examples/config.toml

```toml
[[reinterpreter]]
scope = "window"
pattern = "^(?P<service>[a-z]+)-deploy-(?P<env>[a-z]+)$"
label = "$service to $env"
```

- `scope` is `session`, `window`, or `pane`; any other value is a
  configuration error, not a rule that silently never fires.
- `pattern` is a regex matched against the raw tmux name, and the span
  it matches is replaced by `label`, where `$name` is a named capture.
- Entries are tried in order and the first match wins. A row that
  matches nothing, or whose replacement would display nothing, renders
  raw.
- A rewritten row keeps its raw tmux name beside the new text, so rows
  that reinterpret alike stay distinct and you can always read back
  what tmux calls a seat.

What a reinterpreter cannot do: add, drop, merge, split, reorder or
reparent a row. The tree is built from the tmux census before any of
this is read, so there is nothing for a rule to reach. To display a
session under another name, scope a rule to `session` — there is no
alias table, and no scripting language in the file.
