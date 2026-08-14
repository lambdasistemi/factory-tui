# Modules model

Mandate v3 (recut) · Base `main@ea047e04`

| Module | Responsibility after this slice | Depends on |
|---|---|---|
| `src/config.rs` | Own `SUPPORTED_SAMPLER_FIELDS`, the `Sampler` model, load-time validation, and resolution of a declared field name to an observed value. | `src/tmux.rs` types |
| `src/tree.rs` | Sample each pane through ordered samplers; roll established child status up to windows and ancestors. Owns no field names and no status vocabulary of its own. | `src/config.rs`, `src/tmux.rs` |
| `tests/` (new) | Integration surfaces binding the shipped `examples/config.toml` (C4) and the published schema (C3) to the real `Config`. | real `Config` |
| `nix/crane.nix` | Unchanged responsibility. Narrow source-filter extension only, per A-001, so the published schema reaches sandboxed derivations. | — |

## Not modified by this ticket

`src/tmux.rs` (#22), `src/label.rs` (#26), `src/build_info.rs` and
`tests/cli_version.rs` (#24), `src/app.rs`, `src/ui.rs`, `src/peek.rs`,
`src/ansi.rs`, `src/geometry.rs`, `src/main.rs`.

`Status` and `status_label` keep their current public shape so the UI is
untouched.

## Dependency direction

`tree` -> `config` -> `tmux` types. `config` owns the supported-field
declaration. `src/tmux.rs` is not modified to import it; the declaration and the
query are reconciled by a gate check instead, which catches drift in either
direction — including a field #22 later adds or removes.

## Boundaries

- `src/tmux.rs` is forbidden. Standing invariant: no two lanes edit the tmux
  format string, field count, or parser slice.
- Reinterpreter semantics are forbidden. Labels stay structurally inert.
- `src/tree.rs` must not name a tmux field or status literal outside what
  `src/config.rs` defines.
- No module gains knowledge of any agent, tool, or product name.

## Amendment v4 (ruling A-003 on Q-003)

`src/app.rs` and `src/main.rs` are **narrowly writable**, limited strictly to
the four `tree::build` call-site substitutions bound in `functions-model.md`.

The v3 text listing them as "not modified" was wrong: removing `StatusConfig`
necessarily breaks callers that select `config.status`, and Rust field access
cannot be redirected behind a method. Declaring the callers frozen while
mandating removal of the field they read was an incomplete dependency sweep, not
a constraint the implementation could honour.

Gate check **G3e** enforces the narrowness mechanically: every changed line in
those two files must be one of the authorized call-site substitutions.
