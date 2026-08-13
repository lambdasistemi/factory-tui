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

## Default tree

No file, or an empty file, means session → window. Session names can
be rewritten with `[sessions.alias]`. Sessions matching
`[sessions.infra]` are tagged `[infra]`. `[status]` classifies a
window from pane command names (`running`, `idle`) or a
`parked_substring`.

Config path: `$FACTORY_TUI_CONFIG`, else
`~/.config/factory-tui/config.toml`. A missing file is not an error.

## Projection file

Copy the generic example and edit it:

https://github.com/lambdasistemi/factory-tui/blob/main/examples/projection.toml

`[[rule]]` tables are tried in order. The first `window` regex that
matches wins. Named captures become fields (`project`, `milestone`,
`epic`, `ticket`, `goal`, `role`). An optional `session` regex must
also match.

`[tree] folders` is the fold path. A window that matches no rule
stays under its tmux session.

```toml
[tree]
folders = ["project", "milestone", "epic"]
desk_roles = ["desk"]
inherit_milestone_from_desk = true
```

There is no scripting language in the file.
