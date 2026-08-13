# Research: Flat land browse camera

The M1 user-facing research is `docs/m1/`. This file is the planning
extract only.

## Decision: index is a TUI over live tmux, not a new mux

- **Rationale**: The host already has seats as tmux windows. The
  failure is the chooser, not process persistence.
- **Alternatives**: Zellij / WezTerm mux (same one-size ontology);
  seat server (right long-term compositor, out of scope).

## Decision: snapshot observer, not view-sized attach

- **Rationale**: A tmux window has one size. Attach-at-view-size
  shrinks or crops the agent.
- **Alternatives**: `window-size largest` crop; materialize via
  `join-pane` (temporary workshop, not browse).

## Decision: window-name grammar is the first census

- **Rationale**: Seats already advertise role in the window name.
- **Alternatives**: require `.orch/window.toml` first (stricter, drops
  unnamed seats). Parser keeps unknowns visible.

## Decision: crane `.#cli` is the rooted artifact

- **Rationale**: `target/release` linked from `nix develop` died after
  store GC (`required file not found` on the interpreter).
- **Alternatives**: musl static binary (later); patchelf wrapper (hack).
