# factory-tui

A popup browser over the tmux windows on this host.

It does not replace tmux and it does not create windows. You keep
using sessions and windows as usual. factory-tui reads their names,
draws a tree, shows a coloured snapshot of the selected window, and
jumps the attached client there.

```text
machine
infra
project
  M1 ship
    desk
    e12 parser
      t34 rename
```

That tree is decoded from **window names** (and, when the window name
is incomplete, from the **session name**). A window that does not
match the grammar still appears, under *unscoped*. Nothing is hidden.

[Install](install.md) · [Lay out tmux](using.md)

## Today

- One row in the tree is one tmux **window**. Panes stay inside that
  window; they are not extra rungs.
- The right-hand pane is a snapshot (`capture-pane`). Opening the
  browser does not resize live windows.
- `Enter` or a double-click jumps to the selected window and closes
  the popup.
- Status is a guess from the window name (`PARKED`) and from the
  process in a pane (`claude`, `codex`, a shell, …).

Bind it as a tmux popup, then open it from any session:

```tmux
bind-key -n F1 display-popup -E -w 90% -h 90% factory-tui
bind-key S     display-popup -E -w 90% -h 90% factory-tui
```

`factory-tui --dump` prints the same tree with no UI.

## Not yet decided

The pages under *Research* sketch a flatter world where sessions and
panes stop mattering. That is an **unvalidated experiment**. Do not
treat it as the product. Today's useful setup is ordinary tmux plus
the naming rules in [Using](using.md).
