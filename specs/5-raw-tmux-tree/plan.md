# Implementation Plan: raw tmux tree + config-driven tables

**Ticket**: lambdasistemi/factory-tui#5
**Parent**: epic #8, PR #9 (`feat/2-tmux-browser-projection`)
**Branch**: `feat/5-raw-tmux-tree`
**Date**: 2026-08-13

## Status

- **Completed**: parent spec on PR #9; this ticket contract
- **Current**: single behavior slice, OWNER topology
- **Blockers**: none

## Summary

Move host-specific projection out of the crate. The compiled binary
becomes a tmux browser whose default tree is the census, groupable
by session. Every host identifier moves into an optional TOML file at
`$FACTORY_TUI_CONFIG` else `~/.config/factory-tui/config.toml`.
Missing / empty file is a first-class outcome. Constitution and
README are amended in the same slice.

## Technical Context

- **Language**: Rust (workspace pin `1.85`, toolchain file present)
- **Primary deps**: `ratatui` (existing); add `toml` and `serde`
  (with `derive`) for config load
- **Storage**: optional XDG / env-selected config path; no database
- **Testing**: crate unit tests, dump goldens seeded from an
  in-memory fake census (no live tmux)
- **Target**: Linux + aarch64-darwin (existing flake matrix)
- **Constraints**: preview must not resize (parent invariant); one
  gate `just ci`
- **Scope**: single ticket slice

## Constitution amendment

Current constitution §2 does not name the tmux census as the shipped
default. Add a short amendment (own commit-message-visible line in
the record) declaring:

- The shipped default tree is sessions and windows.
- Projection is optional and data-only.
- Host aliases live in the operator's local config file, never in
  the crate.

The constitution's existing rules on observers-must-not-resize and
seats-are-windows are unchanged.

## README amendment

Add or replace the first "what it does" sentence with:
"With no config, `factory-tui` shows every window under its tmux
session; an optional file at `$FACTORY_TUI_CONFIG` (or
`~/.config/factory-tui/config.toml`) may project that tree."

Existing install / release / license paragraphs are unchanged.

## Slices (bisect-safe)

One OWNER slice: `S5.1 raw-tmux-tree`. The whole acceptance surface
is coupled (I5-NO-HOSTNAMES requires deleting host names from
`src/`, which requires the new `src/config.rs` and the `build`
default path to exist first). Splitting further would either leave
main red between slices or leave host names in the crate at commit N
+ 1. The commit owner may organize its RED bundle into multiple
local prep commits and squash on acceptance (see `resolve-ticket`
task-stamp flow).

## Contract published to #6

Same as parent plan.md:

- `Config` struct deserializes unknown top-level tables without
  failure so #6 may add `[[rule]]` and `[tree]`.
- Tables `sessions.alias`, `sessions.infra`, `sessions.machine`,
  `status.running`, `status.idle`, `status.parked_substring` keep
  the names #6 will read.
- Load order fixed: env → XDG → empty.

## Live boundaries

- No live tmux boundary is exercised by tests in this slice; the
  crate keeps its existing `capture-pane` / `switch-client` shells
  and this slice does not touch them semantically.
- The one operator-facing binary invocation
  (`factory-tui --dump` on a real tmux) is a manual verification
  step for the ticket owner, not part of the gate; the gate proves
  the observable tree from a fake census.

## Verification (frozen gate)

`./gate.sh` (untracked, backed up under
`/tmp/factory-tui/epic-8/ticket-5/gate.sh.bak`) runs, in order:

1. `git diff --check`
2. `nix develop --quiet -c cargo fmt --all -- --check`
3. `nix develop --quiet -c cargo test --workspace`
4. `nix develop --quiet -c cargo clippy --workspace --all-targets -- -D warnings`
5. `rg -nE '\b(keri|csk|treasury-ms1|trenitalia|cip113|cna-214|cna|warden|grok-seat|project-role|wallet|cw)\b' src/ && exit 1 || true`
6. `nix run .#ci`

Falsification: on branch base (before implementation), step 5 fails
(host identifiers are still present), and the empty-config dump
still calls the retired grammar. Gate hash and falsification receipt
are frozen under the runtime root.

## Risk register

- The `toml` / `serde` addition changes `Cargo.lock` and `nix`
  fetcher hashes. Commit owner refreshes the workspace lock and any
  Nix pin the flake requires.
- Naive removal of `src/parse.rs` would break existing UI paths that
  call it. Parent plan says `parse.rs` retires on #6; #5 may leave
  the module compiled but must ensure the default tree does not call
  it (otherwise host identifiers survive through function bodies).
  Commit owner decides between "keep behind a feature flag" and
  "shrink to a compat shim with no host names"; the invariant is
  I5-NO-HOSTNAMES, not "delete the file".
- The `wallet` / `cw` bare-word ban risks false hits inside doc
  strings or comments; gate anchors on `\b…\b`. If a legitimate use
  of `wallet` inside an unrelated word (e.g. `wallets_map`) exists
  the gate stays green because of the anchor.

## Research / models

See [modules-model.md](./modules-model.md),
[data-model.md](./data-model.md),
[functions-model.md](./functions-model.md).
