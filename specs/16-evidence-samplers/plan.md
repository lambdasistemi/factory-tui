# Plan

Issue: #16 · Base: `main@ea047e04` · Mandate v3 (recut)

## Verified state of the merged tree

Established by inspection of `ea047e04`, not assumed:

| Fact | Evidence |
|---|---|
| `pane_title` is queried | `QUERY_FORMAT` in `src/tmux.rs` ends with `#{pane_title}` |
| `Pane` carries `title` | `src/tmux.rs` `pub struct Pane` |
| Tree is session -> window -> pane | `Kind::Pane` in `src/tree.rs` |
| Status is still window-level occupancy | `status_of(win: &Win, …)` matches `pane.cmd` against `running`/`idle` and `parked_substring` |
| `Config` = `sessions`, `status`, `reinterpreter` | `src/config.rs` |
| Shipped example carries legacy `[status]` | `examples/config.toml` |
| `--version` exists | `build_info::display` -> `factory-tui {version} (revision {revision})` |

So the distinguishing evidence #16 needs is already in the tree. This ticket
changes *how status is decided*, not what tmux is asked.

## Strategy

1. `src/config.rs` owns the supported-field declaration, the `Sampler` model,
   load-time validation, and resolution of a declared field name to an observed
   value.
2. `src/tree.rs` evaluates ordered samplers **per pane** and rolls established
   child status up to windows and ancestors.

The supported-field set is declared once and is the single thing the validator,
the resolver, and the schema check bind to. `src/tmux.rs` is not modified, so
the fourth consumer — the query — is reconciled against the declaration by a
check rather than by an import. That reconciliation is stronger than an import:
it catches drift in either direction, including a field #22 later adds or
removes.

## Why `pane_title`

Sampled live across three CLI families on the development host,
`pane_current_command` is identical for working and waiting seats, while
`pane_title` carries a leading braille-block glyph (U+2800–U+28FF) exactly while
the tool is working. The distinguishing evidence is a rendering artifact of
"busy", not an agent name, so a sampler regex over it stays agent-agnostic.

That sample justifies the field choice and does not enter the crate. The crate
ships no regex naming any tool.

## Status is a pane property

Today `status_of` takes a `&Win` and asks "does *any* pane look busy". That is
the defect in miniature: it cannot distinguish a window holding one working pane
from a window holding one waiting pane, and it has nowhere to put "I don't
know".

The recut model:

- each pane is sampled independently; the first matching sampler wins;
- a pane with no match is `Unknown` and **contributes nothing** to rollup;
- a window/ancestor is the rank-worst *established* child status;
- all-unmarked rolls up to `Unknown`, which renders as the empty label — never
  `idle`.

`Unknown` and `Idle` must stay distinct. `Idle` is a positive reading ("I looked
and it is resting"); `Unknown` is the absence of a reading. Collapsing them
would present insufficient evidence as a conclusion, which is the whole class of
bug this ticket exists to remove.

## C3 reachability — the problem recurs

`nix/crane.nix` on `ea047e04` is still `craneLib.cleanCargoSource src`, which
admits only `.rs`, `.toml`, and cargo lockfiles. The sandbox source tree retains
empty `skills/` and `docs/` directories, which makes it *look* as though docs
are present; the files are not. Verified by listing the `nextest` derivation
source: `skills/factory-tui/references/config.md` is absent.

Ruling A-001 authorized a narrow `nix/crane.nix` source-filter extension for
exactly this, for exactly this reason. That fix is carried forward into this
recut. A C3 check that cannot see the published schema would be green
everywhere while observing nothing.

`examples/config.toml` is a `.toml` and **is** present in the sandbox, so C4
loads the real shipped file directly — no fixture copy.

## Slices

One bisect-safe slice. The schema and example must match the model landed in the
same commit (R6); an intermediate commit carrying the new evaluator with the old
schema is a state the repository should never bisect into.

**S1 — named evidence samplers with per-pane status (C1–C4, C-rollup).**

## Dependency sweep

`status_of`'s only inputs are the legacy `[status]` fields. Removing
`StatusConfig` orphans nothing else — confirm before deleting, and confirm no
consumer of `Status::Idle` depends on the old "any pane matches" semantics.

## Risks

- **Silent upgrade regression.** Operators carrying the old `[status]` table
  must fail loudly, not be quietly unmarked. Covered by I5.
- **Unknown collapsing into Idle.** Covered by I8; must be proven, since a
  rollup that returns `Idle` for an all-unmarked window would look reasonable.
- **Vacuous C1.** A fixture proving only "no sampler matches -> not RUNNING"
  would pass against a crate that never marks anything. C1 therefore requires
  the active pane to be RUNNING under the same config.
- **Three-way drift.** Declaration, resolver, and schema could diverge from the
  query. Covered by I3 and I6.
