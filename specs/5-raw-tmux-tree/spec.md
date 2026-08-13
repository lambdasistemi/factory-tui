# Feature Specification: raw tmux tree + config-driven tables

**Ticket**: lambdasistemi/factory-tui#5
**Parent**: [../../specs is external] parent epic #8, PR #9
**Branch**: `feat/5-raw-tmux-tree`
**Created**: 2026-08-13
**Status**: Draft

Parent contract lives on `origin/main`'s
`specs/2-tmux-browser-projection/{spec,plan,modules-model,data-model,functions-model,tasks}.md`
(currently on PR #9). This ticket-level spec narrows Phase 1 (#5) and
does not restate parent invariants — reference by hash below.

## User Scenarios & Testing (Phase 1 only)

### US1 — no-config binary is a tmux browser (P1)

Operator runs `factory-tui` and `factory-tui --dump` with no config
file on disk and no `$FACTORY_TUI_CONFIG`. The dumped tree groups
every observed window under its tmux session name. Empty sessions
still appear as empty groups. Unknown windows are not silently
dropped.

**Independent test**: a fake census `Vec<Win>` with two sessions and
four windows produces a `--dump` text tree whose parents are exactly
those two session names.

### US2 — host tables load from the documented config path (P1)

Operator writes
`~/.config/factory-tui/config.toml` (or points
`$FACTORY_TUI_CONFIG` at another path) declaring:

- `sessions.alias` (map string → string)
- `sessions.infra` (list of patterns)
- `status.running` / `status.idle` / `status.parked_substring`

Loading applies aliases (a session named `shop` folds into `acme`
when `alias.shop = "acme"`); infra patterns mark matching sessions
as infra so a later ticket can hide them; status rules classify
windows as running / idle / parked from their pane command.

**Independent test**: a fake census plus a two-line config with one
alias and one infra rule dumps a tree whose renamed session appears
under the alias, whose infra session is tagged, and whose classified
windows carry the expected status.

### US3 — the crate is generic (P1)

`rg -n 'keri|csk|treasury-ms1|trenitalia|cip113|cna-214|cna|warden|grok-seat|project-role' src/`
returns nothing. Neither `wallet` nor `cw` appears in `src/` as a
session or project identifier (a `wallet` substring inside an
unrelated word such as `wallets_map` is not an identifier — the gate
uses word-boundary anchors).

Every retired name is either loaded from the local config file or
absent.

### US4 — record matches the binary (P1)

- Constitution amendment: the shipped default tree is sessions and
  windows; projection is optional and data-only.
- README default sentence: "With no config, `factory-tui` shows every
  window under its tmux session."

Both edits are in the same slice as the code change (constitution
rule §2.1).

## Requirements

- R5.1  With no config file and no `$FACTORY_TUI_CONFIG`,
        `factory-tui --dump` produces a session → window tree only.
- R5.2  Config load order: `$FACTORY_TUI_CONFIG` when set; else
        `~/.config/factory-tui/config.toml`; else empty config
        (missing file is not an error; unreadable / invalid file
        surfaces as a diagnostic and the binary refuses to lie).
- R5.3  The `Config` struct deserializes unknown top-level tables
        without failing so #6 can add `[[rule]]` / `[tree]` without a
        breaking change.
- R5.4  Tables shipped in #5: `sessions.alias`, `sessions.infra`,
        `sessions.machine`, `status.running`, `status.idle`,
        `status.parked_substring`.
- R5.5  `src/` contains no host product identifier from the retired
        list in US3.
- R5.6  Preview must not resize live windows (unchanged parent
        invariant); jump still uses `switch-client` / `select-window`
        (unchanged parent invariant); no code path in this slice
        touches those subsystems semantically.
- R5.7  Constitution and README default sentence match the shipped
        default in the same slice.
- R5.8  No `examples/projection.toml` in this slice — the generic
        example lands with #6 (parent contract).
- R5.9  No new name grammar and no changes to the popup/jump/mouse
        subsystems.

## Invariants (stable IDs)

- I5-DEFAULT-TREE   `tree::build(_, &Config::empty())` produces a
                    session-group node for every session in the
                    census and every window sits under its own
                    session; no orphan windows.
- I5-CONFIG-PATH    Load precedence env → XDG → empty is observed;
                    missing files are silent; the empty-config path
                    is byte-identical to the no-file path.
- I5-CONFIG-FORWARD An unknown top-level table in the TOML file is
                    ignored (forward-compatible with #6).
- I5-TABLES-APPLIED `sessions.alias`, `sessions.infra`, and
                    `status.{running,idle,parked_substring}` change
                    observable tree output when set; do nothing when
                    unset.
- I5-NO-HOSTNAMES   `rg -nE '\b(keri|csk|treasury-ms1|trenitalia|cip113|cna-214|cna|warden|grok-seat|project-role|wallet|cw)\b' src/`
                    returns exit 1 (no matches). Applies to the
                    committed final tree, not intermediate work.
- I5-PREVIEW-JUMP   No semantic change to preview / jump / popup /
                    mouse behaviour; smoke unit or golden proves
                    preview does not send a resize command.
- I5-RECORD-MATCH   Constitution amendment names the tmux census as
                    the default; README first paragraph names
                    sessions and windows as the default tree.
- I5-GATE-GREEN     `./gate.sh` and `nix develop --quiet -c just ci`
                    pass on the final commit tree from a clean
                    detached worktree.

Invariants above are the acceptance surface. The auditor's report
maps one verdict to each.

## Success Criteria (aligned to issue #5 checkboxes)

- SC5.1  `factory-tui` and `factory-tui --dump` with no config group
         windows under their tmux session names — proved by US1
         golden.
- SC5.2  Compiled `src/` contains none of the identifiers listed on
         #5 — proved by I5-NO-HOSTNAMES.
- SC5.3  Documented config path can declare tables — proved by US2
         golden.
- SC5.4  Unit / golden with no config dumps a session → window tree
         from a fake census.
- SC5.5  Unit / golden with a config applies one alias and one infra
         rule.
- SC5.6  Constitution text matches shipped default.
- SC5.7  README says the default tree is sessions and windows.

## Assumptions

- The existing `Vec<Win>` census shape from `src/tmux.rs` is reused
  and not renamed in this slice.
- A TOML crate is added under the workspace's dependency policy
  (`toml`, current stable) — commit owner picks the pin.
- The parked-substring default is empty string (no parked detection
  unless configured).

## Non-goals

- Name grammar and folder path (#6).
- Documentation restructure (#7).
- Any refactor of `src/parse.rs` internals beyond making the default
  tree not need it (delete lands with #6).
- Shipping a host alias table.
