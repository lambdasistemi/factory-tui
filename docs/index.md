# factory-tui

A popup browser over the tmux windows on this host.

With no config file it shows **session → window**. Enter jumps. The
snapshot does not resize the live window.

An optional file may fold matching names into another tree. That file
is yours. The crate does not know any host's product names.

[Install](install.md) · [Using](using.md)

An agent on a new host loads `AGENTS.md` and
`skills/factory-tui/SKILL.md` from the repository. Nix install is
`nix profile add github:lambdasistemi/factory-tui`.

## Today

- One row is one tmux **window**. Panes stay inside that window.
- Right-hand view is `capture-pane`. Opening the browser does not
  resize live windows.
- `Enter` or a double-click jumps and closes the popup.
- `--dump` prints the same tree as text.

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

## Research

Older notes under *Research* are an unvalidated experiment. They are
not the shipped default.
