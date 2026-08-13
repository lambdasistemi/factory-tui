# M1 decisions

Recorded so a later change has to contradict them on purpose.

## Accepted

1. **The index speaks factory, not multiplexer.** Session lists and
   encoded window titles are not the product.
2. **Window = seat.** Role isolation is a window, not a pane.
3. **Views are cameras.** Recipes live in the app. They do not mutate
   the land.
4. **Observers do not resize.** Snapshots and mosaics never
   `SIGWINCH` a live agent.
5. **tmux stays as process host.** Do not shop multiplexers for a
   compositor they cannot be.
6. **One tree, several renderers.** Keyboard TUI now; tablet attach
   by embedding this TUI; a seat server later if live mosaics must
   be honest.
7. **Materialize is optional and temporary.**

## Rejected

| Idea | Why not |
|---|---|
| Keep `prefix + s` as the factory chooser | Lists the wrong tree |
| One tmux session per milestone as identity | Encodes WHAT as WHERE; the index already crosses sessions |
| Flatten every seat into one session *before* the tablet has the tree | Makes tmux-ws's window list the old chooser |
| Ticket workshop as a permanent 2×2 of panes | Skip-level glass; unreadable on a tablet; layout law |
| View-sized live attach of the real windows | One window, one size; shrinks or crops |
| Switch to Zellij / WezTerm mux for views | Same ontology |
| Pixel-scale VNC of terminals | Works, looks bad, wrong layer |
| factory-tui as a `display-popup` that exits on go | Cannot be the standing embed |

## Open (not M1)

- Declared seat size (last interactive client, or a default such as
  160×50) once a seat server exists.
- A Home control in tmux-ws that returns the client to the factory
  session.
- Whether machine crew stays in its own session forever.
- Crane-rooted Nix package so the binary survives store GC.

## Observable test for M1

A stranger who has cloned this repository can:

1. Read [flat-land](flat-land.md) and [compositor](compositor.md) and
   state what a seat, a view, and an observer are.
2. Run `nix develop -c cargo test` and see the window-name parser and
   colour snapshot tests pass.
3. Run the binary inside tmux, walk a tree that is **not** a session
   list, and open a seat without the app resizing other seats.

The milestone artifact is this document set plus the browse-camera
prototype. It is marked temporary: a later milestone owns the
rooted package and the standing `--serve` embed.
