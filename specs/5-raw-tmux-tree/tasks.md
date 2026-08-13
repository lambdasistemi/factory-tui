# Tasks: raw tmux tree + config-driven tables

**Input**: spec.md, plan.md, modules-model.md, data-model.md,
functions-model.md
**Ticket**: lambdasistemi/factory-tui#5

## Slice S5.1 — raw-tmux-tree (single OWNER slice)

- [x] T5-001 RED (golden): with no config file and no
      `$FACTORY_TUI_CONFIG`, `tree::build(fake_census,
      &Config::empty())` dumps a session → window text tree; every
      window sits under its own session. (I5-DEFAULT-TREE)
- [x] T5-002 RED (unit): `config::load()` respects
      `$FACTORY_TUI_CONFIG` first, then XDG default, then falls back
      to `Config::empty()` on missing file. (I5-CONFIG-PATH)
- [x] T5-003 RED (unit): `config::load_from_str` deserializes a
      config with an unknown top-level table (`[fabricated]`) into
      the current-schema `Config` without error. (I5-CONFIG-FORWARD)
- [x] T5-004 RED (golden): with a config declaring one alias and one
      infra pattern, `tree::build` renames the aliased session in the
      dumped tree and tags the infra session. (I5-TABLES-APPLIED)
- [x] T5-005 RED (gate integration): the frozen `./gate.sh` step
      that greps `src/` for the retired host identifiers is red on
      the branch base and green on the accepted candidate.
      (I5-NO-HOSTNAMES)
- [x] T5-006 GREEN: implement `src/config.rs` per
      `functions-model.md`; add `toml` + `serde` deps.
- [x] T5-007 GREEN: implement the default and table-aware paths in
      `src/tree.rs::build` per `data-model.md`; wire `--dump` in
      `src/main.rs` to `config::load()`.
- [x] T5-008 GREEN: remove every host identifier from `src/` per
      I5-NO-HOSTNAMES; do not rely on `src/parse.rs` for the default
      tree (retire fully on #6).
- [x] T5-009 GREEN: amend `.specify/memory/constitution.md`:
      shipped default is sessions and windows; projection is
      optional and data-only; host names live in a local file.
      (I5-RECORD-MATCH)
- [x] T5-010 GREEN: amend README first paragraph so the default
      sentence matches the shipped default. (I5-RECORD-MATCH)
- [x] T5-011 GREEN: `nix run .#ci` and `./gate.sh` pass on the final
      accepted commit tree from a clean detached worktree.
      (I5-GATE-GREEN)

## Dependencies

RED bundle (T5-001..T5-005) is committed by the commit owner before
GREEN implementation (T5-006..T5-011). All RED tests must fail for
the intended reason; the frozen slice-gate falsification receipt
under the runtime root witnesses this.

## Out of scope (deferred to #6 / #7)

- `[[rule]]` and `[tree]` parsing.
- `classify` name-grammar function.
- `examples/projection.toml` and its goldens.
- Deletion of `src/parse.rs` (only host names must be gone in #5).
- README restructure and docs page rewrite (#7).
