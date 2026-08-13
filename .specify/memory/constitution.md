# factory-tui Constitution

This document is project law. Plans that contradict it are rejected.
Code is regenerable from a good record; the record is not regenerable
from code. Specs, vision, and acceptance outrank implementation. When
time or budget forces a cut, cut implementation scope — never the
record.

## 1. What this software is

factory-tui is the **index and compositor** for a factory of visible
agent seats. The operator walks a tree of work (machine, project,
milestone, epic, ticket, role) and opens a seat. Terminal multiplexers
are a process host, not the factory's org chart.

## 2. Core principles

### 2.1 The record outranks the implementation

Every story ships its user-facing docs in the same change. Acceptance
is written in user-visible terms (a tree a person can walk, a view they
can name, a seat they can enter) before any code. A green binary with a
stale document is a failed change.

The shipped default tree is tmux sessions and windows. Projection is
optional and data-only; host aliases live in the operator's local
configuration file, never in the crate.

### 2.2 WHAT is not WHERE

Factory authority (who owns what work) and placement (which process
sits on which terminal object) are independent. The UI and the durable
index speak WHAT. Sessions, windows, and panes are WHERE. Naming a
tmux session after a milestone does not make it a milestone.

### 2.3 Seats are windows; views are cameras

A **seat** is one visible agent process with its own identity and
start acknowledgement. In the current host, a seat is one tmux window.

A **view** is a named camera over a set of seats (for example a ticket
workshop as a 2×2). Views do not own processes. They do not steal
panes. They do not change a seat's terminal size.

### 2.4 Observers must not resize

Browse and mosaic are observers: they snapshot or crop. They never
send a resize to a live agent. Interactive attach (Enter / go) is the
one client that may size a seat to the glass.

A multiplexer window has one size. Attaching a view-sized client
either shrinks the agent for everyone or shows a crop. factory-tui
must not pretend otherwise.

### 2.5 No factory panes, no factory sessions

Panes are optional human chrome (a disposable draft beside its owner,
a personal scratch split). They are not how the factory stores
structure.

Sessions are optional transport buckets (machine crew, a home session
for the browser). They are not rungs on the owner ladder. The index
must still work if every product seat lives in one session.

### 2.6 One index, several eyes

The factory tree is one. A keyboard TUI, a tablet attach, and a later
web mosaic are renderers of that tree. They share seat identity
(stable window id today; a seat-server id later). They do not each
invent a hierarchy.

### 2.7 Nix-first, one gate

`nix flake check` is the gate. Local and CI run the same shell. A
`target/release` binary linked against an unrooted Nix store path is
not a release; it will vanish on garbage collection.

## 3. Domain constraints

- **tmux** is the current process babysitter. It is not the compositor.
- **tmux-ws** is the live-attach camera (browser xterm). It attaches
  to a session. The factory home session is how a tablet enters the
  index, not a second factory model.
- A future **seat server** (PTY + retained cell buffer, interactive
  vs observer subscribers) is the primitive that would make view-sized
  live mosaics honest. Until it exists, snapshots are the browse
  camera.
- **Materialize** (join seats into a temporary split window) is an
  optional human workshop. It is never the source of truth.

## 4. Development workflow

- Public repository. Issue bodies and PR descriptions are written for
  a reader who has only this repository.
- Conventional Commits. Merge commits on `main`. Release-please
  cuts versions for the Rust crate.
- Tests prove something that can fail: a parser against live window
  names, a view recipe that refuses to resize, a go-target that is a
  window not a pane.
- Branch protection requires the named CI gate once the stub is
  replaced.

## 5. Governance

Amendments to this constitution are their own change, reviewed as
law, not as a drive-by in a feature diff. Milestone ledgers may
refine how a release line is named; they may not quietly invert
§2.
