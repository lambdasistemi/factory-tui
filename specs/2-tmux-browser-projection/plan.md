# Implementation Plan: tmux browser and optional projection

**Branch**: `feat/2-tmux-browser-projection` | **Date**: 2026-08-13
**Spec**: [spec.md](./spec.md)
**Epic**: https://github.com/lambdasistemi/factory-tui/issues/8

## Status

- **Completed**: parent + children filed; this record
- **Current**: #5 ready (default tree + tables)
- **Blockers**: none

## Summary

Keep `tmux` census, preview, and jump in the crate. Move every
projection — host tables, name grammar, folder path — into an
optional TOML file. No file means session → window.

## Technical Context

**Language/Version**: Rust 1.90 (workspace pin)
**Primary Dependencies**: ratatui 0.29; add a TOML crate on #5
**Storage**: optional XDG/config path; no database
**Testing**: crate unit tests + dump goldens from a fake census
**Target Platform**: Linux and aarch64-darwin (existing flake)
**Project Type**: CLI / TUI
**Constraints**: observers must not resize; one gate `just ci`
**Scale/Scope**: three serial children on `factory-tui`

## Constitution Check

Today's constitution names factory-tui as an index over a factory
tree and says sessions are not rungs. #5 amends that: the shipped
default *is* the tmux census; a file may project it. Preview/jump
rules stay. Plans that keep host names in Rust fail this epic.

## Project Structure

```text
specs/2-tmux-browser-projection/   this record
src/tmux.rs                        census, capture, focus (stay)
src/tree.rs                        fold: raw or projected
src/parse.rs                       retire on #6
src/config.rs                      new on #5, grow on #6
examples/projection.toml           generic example (lands with #6)
```

## Children and owned surfaces

| Child | Run | Owned |
|---|---|---|
| #5 | `factory-tui --dump` is session → window; config tables work | `src/tree.rs` default, new `src/config.rs`, constitution, README default sentence |
| #6 | same binary folds from rules + path | `src/parse.rs` deleted, `src/config.rs` rules/fold, `examples/projection.toml`, goldens |
| #7 | docs match | README, `docs/`, example path only |

## Contract #5 publishes for #6

- Config struct deserializes unknown keys without failing.
- Tables: session aliases, infra patterns, running/idle/parked.
- Load order: env path, else XDG path, else empty config.

#6 adds `[[rule]]` and `[tree]` without renaming those tables.

## Release

Artifact is `factory-tui`. Tag after #5 (default flip) and at epic
close. Existing v* workflows.

## Research / models

See [research.md](./research.md), [modules-model.md](./modules-model.md),
[data-model.md](./data-model.md), [functions-model.md](./functions-model.md).
