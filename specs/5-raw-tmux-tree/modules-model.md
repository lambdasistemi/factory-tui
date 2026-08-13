# Modules

Follows the parent modules model
(`specs/2-tmux-browser-projection/modules-model.md` on PR #9), narrowed
to Phase 1.

## `src/tmux.rs` (unchanged owner)

Census (`list-windows`), `capture-pane`, `switch-client`,
`select-window`. This slice does not add or remove functions here and
does not rename `Win`.

## `src/config.rs` (new)

Owns:

- The `Config` data types for tables listed in
  [data-model.md](./data-model.md).
- The load pipeline: `$FACTORY_TUI_CONFIG` else XDG default else
  empty; missing file is not an error.
- Serde derivations that tolerate unknown top-level tables so #6 can
  extend without breaking parse.

Downstream of: `std`, `serde`, `toml`. No dependency on `tmux.rs` /
`tree.rs`.

## `src/tree.rs` (changed)

- Default path: `build(wins, &Config::empty()) → Node` groups every
  window under its session; empty sessions surface as empty groups.
- Config-driven path: aliases rewrite session titles; infra sessions
  are tagged; per-window status is derived from `status.*`. Neither
  path invokes the retired grammar for its default behaviour.
- `dump(node) → String` is unchanged in purpose.

## `src/parse.rs` (partial retirement — full delete on #6)

Parent contract: `parse.rs` is retired on #6. In this slice the
module may keep compiling but must contain no host identifier from
the retired list, and the default `--dump` path must not call it. If
retaining the file requires no host names to satisfy I5-NO-HOSTNAMES
and no dead-code warning, the commit owner may reduce it to an empty
shim; the decision is implementation-local.

## `src/app.rs`, `src/ui.rs`, `src/main.rs`

Load `Config` once at startup and pass it into `tree::build`. No
host names. No new commands. No changes to popup / jump / mouse.

**Dependency direction**: `main → config → tree → tmux`. `parse` has
no successor after #6 and no new dependents in this slice.
