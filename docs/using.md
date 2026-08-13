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
