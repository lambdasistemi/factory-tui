# Implementation Plan: Flat land browse camera

**Branch**: `feat/m1-flat-land-compositor` | **Date**: 2026-08-13 |
**Spec**: [spec.md](spec.md)
**Input**: lambdasistemi/factory-tui#1

## Status

- **Completed**: constitution, M1 docs, browse-camera prototype, crane
  package and CI gate, speckit scaffold + this contract (retroactive).
- **Current**: none.
- **Blockers**: none for this ticket. Wiki Pages deploy waits on merge.

## Summary

Ship a read-only factory index over live tmux windows: parse names into
a tree, snapshot the selected pane with SGR colour, jump the attached
client. Bind the product law in `.specify/memory/constitution.md` and
`docs/m1/`. Root the binary with crane so store GC cannot delete it.

## Technical Context

**Language/Version**: Rust 1.90 (MSRV 1.85)
**Primary Dependencies**: ratatui 0.29, tmux CLI
**Storage**: none (live census + optional STATUS/brief peek)
**Testing**: cargo test / cargo-nextest
**Target Platform**: Linux, inside tmux
**Project Type**: native TUI CLI
**Performance Goals**: preview refresh ~800ms; tree rebuild on `r`
**Constraints**: observers never `SIGWINCH`; no view-sized attach
**Scale/Scope**: one host, tens of windows

## Constitution Check

| Gate | Verdict |
|---|---|
| Record outranks implementation | Pass — docs/m1 + constitution in the same PR |
| WHAT is not WHERE | Pass — tree is factory-shaped |
| Seats are windows; views are cameras | Pass — no join-pane, no mosaic attach |
| Observers must not resize | Pass — `capture-pane` only |
| Nix-first, one gate | Pass — `.#cli` + crane checks |

## Project Structure

```text
specs/1-flat-land-compositor/
├── spec.md
├── plan.md
├── research.md
├── modules-model.md
├── data-model.md
├── functions-model.md
├── tasks.md
├── quickstart.md
└── checklists/requirements.md

src/           # browse-camera binary
nix/           # crane package + checks
docs/m1/       # user-facing M1 record
.specify/      # constitution, templates, scripts
```

## Slices (bisect-safe, already landed)

1. **Record** — constitution + `docs/m1/` + wiki pointer.
2. **Census + tree** — `tmux.rs`, `parse.rs`, `tree.rs`.
3. **Observer preview** — `ansi.rs`, `capture-pane -e`, no resize.
4. **Jump + input** — Enter, double-click, throttled wheel, light labels.
5. **Rooted build** — crane `.#cli`, `just ci`, real CI workflow.

## Live boundary

The live boundary is `tmux list-panes -a` and `capture-pane`. Unit
tests cover parse/ANSI. A clone without tmux can still run
`nix build .#cli` and parser tests. A full glass smoke needs a tmux
server (operator follow-up, not this ticket's gate).
