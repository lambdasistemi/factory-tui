# Tasks: tmux browser and optional projection

**Input**: spec.md, plan.md, models
**Children**: #5, #6, #7 (serial)

## Phase 1 — #5 default tree and tables

- [ ] T001 Census helper: `tree::build(wins, &Config::empty())` dumps
      session → window (failing golden first)
- [ ] T002 Config load: env path, XDG path, missing file = empty
- [ ] T003 Tables: alias, infra pattern, status lists applied in
      `build`
- [ ] T004 Remove host identifiers from `src/`
- [ ] T005 Amend constitution + README default sentence
- [ ] T006 `just ci`

## Phase 2 — #6 rules and fold

- [ ] T007 Config: `[[rule]]` + `[tree]` parse
- [ ] T008 `classify` first-match named captures
- [ ] T009 Fold along `tree.folders`; unmatched stay on session
- [ ] T010 Delete `src/parse.rs` grammar
- [ ] T011 Generic `examples/projection.toml` + goldens
- [ ] T012 `just ci`

## Phase 3 — #7 docs

- [ ] T013 README and docs home: tmux browser first
- [ ] T014 Config page points at the generic example
- [ ] T015 Research stays marked unvalidated, not the front door

## Dependencies

T001–T006 before T007. T007–T012 before T013.
