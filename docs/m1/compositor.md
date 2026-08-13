# User-controlled compositor

!!! warning "Unvalidated experiment"
    This page is research, not operating advice. Today's binary is a
    browser over ordinary tmux windows. See [Using](../using.md).

**Milestone 1 outcome:** the operator owns **views**. A view is a
named camera over seats. The app decides the recipe (one desk, a
ticket workshop, a crew row). The multiplexer does not.

## Views are not windows

tmux will not tile four windows for one client. It will not show the
same pane in two places. `join-pane` *moves* a seat into a split and
undoes window-per-role.

So a live tmux quadrant is not a view. It is a temporary workshop.

A view is data on a tree node:

```text
tree (source of truth)          view (composed)
ticket 271                      OWNER workshop
  ● ticket-owner      ─┐        ┌ T.O. snapshot  ┬ implementer ┐
  ● commit-owner      ─┼──────► │                ├ auditor     │
  ● auditor           ─┘        └────────────────┴─────────────┘
```

Click a cell selects that seat. Enter / double-click opens it full
screen. The camera refreshes; the agents do not change size.

## Three layers

| Layer | What it is | When |
|---|---|---|
| Snapshot mosaic | `capture-pane` (with colours) laid out by the recipe | browse, glance, tablet via the embedded TUI |
| Interactive attach | one client, one window, client-sized | Enter / go |
| Materialize | `join-pane` into a scratch window, sized to the recipe | a human who wants a real split for an hour, then break apart |

Materialize is never the source of truth.

## Attach cannot match view sizes

A tmux window has **one** size.

If the app attaches a client into a view cell of 90×20:

- `window-size smallest` / `latest` — the real agent shrinks. Every
  other viewer sees the postage stamp. Claude and Codex reflow.
- `resize-window -x 90 -y 20` — same shrink, sticky until undone.
- `window-size largest` — the agent stays big; the view shows a
  **crop**, not a scaled workshop.

There is no scale-to-fit. TUIs are a cell grid, not pixels.

Rule:

```text
browse  = snapshots, view-sized, agents untouched
enter   = attach one window, client-sized
workshop = optional materialize, then tear down
```

Do not attach four live windows into a view and push those sizes onto
them.

## Nothing better as a drop-in multiplexer

Zellij, WezTerm mux, and Screen share the same ontology: one pane, one
parent, one size. Zellij still sizes everyone to the smallest
client.

Zellij's read-only watch is the only interesting step: an observer
that must not type or resize. It still does not give a view-sized
*live* workshop.

A TUI cannot be two grids at once. “Look small here, run large there”
is a display problem.

The primitive that would be right is a **seat server**, not another
mux:

```text
agent  →  PTY + retained cell buffer  (the seat owns its size)
                │
                ├─ interactive client   1:1 cells, may resize
                └─ observers            snapshot / crop / pixel-scale
                                        cannot resize
```

Browse and mosaics are observers. Enter is the one interactive
client. Mosaic cells never `SIGWINCH` the agent.

Until that server exists, this app's snapshots *are* the observer.
tmux-ws is the interactive attach. Using tmux as a compositor is the
mistake; using it as a process host is fine.

## Embedding in a home session

tmux-ws is already a real tmux client (`tmux attach -t <session>` over
a PTY). It does not need its own factory tree if this TUI is a
**standing window** in a home session (for example `0-factory`):

- Tablet: attach to that session, walk the tree, Enter moves *that*
  client to the seat. The TUI process stays up (`--serve`). Back =
  select the home session again.
- Laptop: `F1` switches to the same window, not a second popup.
- Go must name the interacting client (`switch-client -c …`).
- A popup that quits after go is the wrong embed.

Touch must reach the TUI as mouse events or the tablet can see the
tree and not drive it. That is a camera concern, not a reason to
invent a second factory model.

## What the current prototype already is

The binary in this repository is the browse camera:

- Tree projected from live window names (best-effort parser).
- Coloured snapshot of the selected pane, about once a second.
- Enter / double-click jumps the attached client.
- It does not resize agents. It does not join panes. It does not
  claim to be a live mosaic.
