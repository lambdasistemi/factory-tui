# factory-tui

A popup browser over the tmux windows on this host.

The tree is tmux itself: **session → window → pane**. Enter jumps. The
snapshot does not resize the live window.

An optional file may rewrite what a row says; it cannot change what the
tree is. That file is yours, and the crate does not know any host's
product names.

[Install](install.md) · [Using](using.md)

An agent on a new host loads `AGENTS.md` and
`skills/factory-tui/SKILL.md` from the repository. Nix install is
`nix profile add github:lambdasistemi/factory-tui`.

## Today

- One row is one tmux **session**, **window**, or **pane**. A window
  holding a single pane is that pane, and carries no extra row.
- Right-hand view is `capture-pane`. Opening the browser does not
  resize live windows.
- `Enter` or a double-click jumps and closes the popup. Selecting a
  pane row jumps to that exact pane.
- `--dump` prints the same tree as text, with one `[window=@id]` marker
  per window and one `[pane=%id]` marker per pane.

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

## Research

Older notes under *Research* are an unvalidated experiment. They are
not the shipped default.
