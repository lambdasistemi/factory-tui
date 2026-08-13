# Modules Model: Flat land browse camera

## `src/parse.rs`

- **Status**: new
- **Responsibility**: Decode a window name + session hint into factory
  fields (project, milestone, epic, ticket, role).
- **Owns abstractions**: Parsed, Role
- **Upstream dependencies**: None
- **Downstream consumers**: `src/tree.rs`
- **Promotions**: None
- **Forbidden dependencies**: tmux I/O, ratatui

## `src/tree.rs`

- **Status**: new
- **Responsibility**: Project a factory tree from a census of windows.
- **Owns abstractions**: Node, Kind, Status
- **Upstream dependencies**: `src/parse.rs`, `src/tmux.rs` (Win)
- **Downstream consumers**: `src/app.rs`
- **Promotions**: None
- **Forbidden dependencies**: ratatui, live tmux commands

## `src/tmux.rs`

- **Status**: new
- **Responsibility**: Live census and client jump; capture-pane snapshot.
- **Owns abstractions**: Win, Pane
- **Upstream dependencies**: None
- **Downstream consumers**: `src/tree.rs`, `src/app.rs`, `src/peek.rs`
- **Promotions**: None
- **Forbidden dependencies**: factory grouping logic

## `src/ansi.rs`

- **Status**: new
- **Responsibility**: SGR in a capture → styled lines. Must not resize.
- **Owns abstractions**: none beyond conversion
- **Upstream dependencies**: ratatui style/text
- **Downstream consumers**: `src/ui.rs`
- **Promotions**: None
- **Forbidden dependencies**: tmux

## `src/app.rs` / `src/ui.rs`

- **Status**: new
- **Responsibility**: Selection, preview refresh, jump, drawing.
- **Owns abstractions**: App, Row, ClickTarget
- **Upstream dependencies**: tree, tmux, ansi, peek, geometry
- **Downstream consumers**: `src/main.rs`
- **Promotions**: None
- **Forbidden dependencies**: joining or resizing foreign windows

## `nix/`

- **Status**: new
- **Responsibility**: Store-rooted CLI and sandboxed checks.
- **Owns abstractions**: `.#cli`, flake checks
- **Upstream dependencies**: crane, rust-overlay
- **Downstream consumers**: CI, operators
- **Promotions**: None
- **Forbidden dependencies**: relying on `target/release` as the artifact
