# Modules

## `src/tmux.rs` (unchanged owner)

Census, `capture-pane`, `switch-client` / `select-window`. No
projection types.

## `src/config.rs` (new, #5; grows in #6)

Load and validate the optional file. Owns the data types for
tables (#5) and rules/fold (#6). Downstream of nothing in the
crate except std / a TOML library.

## `src/tree.rs` (changed)

Builds a `Node` tree from `Vec<Win>` plus `Config`. Default path:
session → window. Projected path: apply rules, then fold.
Does not parse names itself after #6.

## `src/parse.rs` (retire on #6)

Today's grammar. #5 may still call it for names when no rules
exist, or #5 may already ignore it for the default tree. #6
deletes it.

## `src/app.rs` / `src/ui.rs` / `src/main.rs`

Load config once at start. No host names.

**Dependency direction**: `main` → `config` → `tree` → `tmux`.
`parse` has no successor after #6.
