# Tasks: Flat land browse camera

**Input**: `specs/1-flat-land-compositor/`
**Prerequisites**: spec.md, plan.md
**Note**: Retroactive. All tasks were delivered on this branch; boxes
are checked against the tree, not as a future plan.

## Phase 1: Setup

- [x] T001 Add constitution at `.specify/memory/constitution.md`
- [x] T002 [P] Add Spec Kit scripts and templates under `.specify/`
- [x] T003 [P] Scaffold Rust crate (`Cargo.toml`, `src/main.rs`)

## Phase 2: Foundational

- [x] T004 Write M1 record in `docs/m1/flat-land.md`, `docs/m1/compositor.md`, `docs/m1/decisions.md`
- [x] T005 Crane package and checks in `nix/` and `flake.nix`
- [x] T006 Replace stub CI with `.github/workflows/ci.yml`

## Phase 3: User Story 1 — Walk the factory (P1)

- [x] T007 [US1] Census windows in `src/tmux.rs`
- [x] T008 [US1] Parse window names in `src/parse.rs`
- [x] T009 [US1] Build factory tree in `src/tree.rs`
- [x] T010 [US1] Draw the tree in `src/ui.rs` / `src/app.rs`

## Phase 4: User Story 2 — Glance without resize (P1)

- [x] T011 [US2] `capture_pane` without resize in `src/tmux.rs`
- [x] T012 [US2] SGR → text in `src/ansi.rs`
- [x] T013 [US2] Preview refresh ~800ms in `src/main.rs`
- [x] T014 [US2] Host-default background in `src/ui.rs`

## Phase 5: User Story 3 — Enter the intended seat (P1)

- [x] T015 [US3] `focus` jump in `src/tmux.rs`
- [x] T016 [US3] Enter and double-click in `src/app.rs`
- [x] T017 [US3] Light-readable labels and throttled wheel in `src/ui.rs` / `src/app.rs`

## Phase 6: User Story 4 — Record and gate (P2)

- [x] T018 [US4] MkDocs pages and `deploy-docs.yml`
- [x] T019 [US4] `nix build .#cli` and `just ci`
- [x] T020 [US4] Parser and ANSI tests in `src/parse.rs` / `src/ansi.rs`

## Phase 7: Polish

- [x] T021 Point issue #1 and PR #2 at this speckit tree
- [x] T022 [P] Wiki M1-State and `docs/m1/state.md`

## Dependencies

Setup → Foundational → US1 → US2 / US3 (after tree exists) → US4.
US2 and US3 do not depend on each other once T010 exists.

## MVP

US1 + US2 + US3. US4 is the public record and rooted build.
