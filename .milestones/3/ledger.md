# M3 — No-bugs (release satisfaction)

Home: https://github.com/lambdasistemi/factory-tui
GH milestone: https://github.com/lambdasistemi/factory-tui/milestone/3
State page: https://github.com/lambdasistemi/factory-tui/wiki/M3-State
Project: factory-tui — `.projects/factory-tui/` on this branch
Desk: session `factory-tui`, window `factory-tui-ms3-no-bugs`, runtime `/tmp/ms-3`

## Outcome and its observable test

The published factory-tui can be called good enough to release; the
browser does not lie about work in progress.

Audited against the **published artifact**, obtained the way a stranger
obtains it — not a source build:

1. `nix profile add github:lambdasistemi/factory-tui/<pre-release tag>`
2. `factory-tui --dump` does not mark a waiting seat RUNNING
3. the configuration schema is published and describes what the crate
   actually accepts
4. the setup skill selects running-status samplers from the live box
5. every remaining finding is fixed or explicitly waived on this ledger

Accepting M3 is what makes the product satisfiable for a release. It is
not the release act. Tag/publish/announce of the product line is a later,
separately authorized act (D-2026-08-13-m3-no-bugs).

## Milestone artifact

- Line: a milestone-scoped git tag whose name does **not** match `v*`, so
  no GitHub release object is created and no publisher runs (A-003). Exact
  name is T-B's to choose; it must be obviously provisional.
- Graduates into: the next product release at M3 close (release-please
  owns the production line; the milestone tag never displaces it) — the
  milestone tag is retired at close.
- Merges into it: E-A, and every standalone ticket below.
- Status: **DESIGNED, not yet built.** The original blocker (the pinned
  publisher cannot mark a pre-release, so a `v*` tag could have taken
  Latest from v0.1.0) was removed rather than managed: see C7 and A-003.
  T-B is live and building it. Still gates the outcome audit.

## Units

| Id | Kind | Outcome | State | Lane |
|---|---|---|---|---|
| E-A (#15) | epic | Status is sampled, not assumed: samplers replace pane occupancy; published schema, example, census and default recipes move with it; agent-agnosticism gets an enforcing check | DISPATCHED — brief `/tmp/ms-3/e-a/brief.md` sha256 `c6fc721310faddea` (amended post-build with the host disk-lock constraint; the machine owner's build-time verification of `03039c79220307e7` predates that amendment); **LIVE** pane `%6422` (@4478), codex-raw/gpt-5.6-sol/high; acknowledged (tag format corrected by NOTE-001) | `factory-tui-ms3-e-unknown-status-samplers` |
| T-B (#19) | ticket | A clearly-marked milestone pre-release line a stranger can install, which cannot displace the product line | DISPATCHED — brief `/tmp/ms-3/t-b/brief.md` sha256 `c1ed3c92081ac9ca`; **LIVE** pane `%6423` (@4479), codex-raw/gpt-5.6-sol/high; `START` received; first tag push gated on Q-002 | `factory-tui-ms3-t-unknown-prerelease-line` |

Both children filed their own issues, as their contracts required — this
desk filed nothing.

- **#15** epic "Make work-in-progress status evidence-based and
  agent-agnostic", cut into ordered children by its owner:
  **#16** report RUNNING only from named evidence samplers (ready) ->
  **#17** census-driven status sampler recipes (blocked on 16) ->
  **#18** enforce agent-agnostic shipped artifacts in Nix and CI (blocked
  on 16, 17). Merge order 16, 17, 18. Baseline `just ci` green at
  `8a273de`; worktree `/code/factory-tui-epic-15`.
- **#19** "ci: publish an installable M3 preview tag without touching the
  product release line"; worktree `/code/factory-tui-issue-19`, branch
  `ci/19-m3-preview-tag`, baseline green.

## Priority order

1. **E-A** — the milestone's named defect. The product's central claim
   ("this seat is working") is false today for every claude/codex desk;
   everything else in M3 is cosmetic beside it. Brief directs it first
   and nothing worse appeared on the map.
2. **T-B** — day-0 artifact. Ranked second only because it blocks the
   *audit*, not the fixes; it must land well before acceptance, since an
   unobtainable artifact makes the outcome test unrunnable. If E-A's
   lane stalls, T-B is promoted rather than leaving the desk idle.

No inversions to date.

## Defect map (2026-08-13)

Release-blocking, in severity order. D1 is confirmed by reproduction;
the rest by direct source reading at `main`.

- **D1 — false RUNNING.** `src/tree.rs:303 status_of` returns Running
  when any pane's `pane_current_command` equals a configured name. That
  samples occupancy, not work. **Reproduced:**
  `factory-tui:factory-tui-e8-t5-raw-tree` (pane `%6385`) — a finished
  lane at an idle prompt with unsent composer text — is reported
  `RUNNING`. The same model fails open in the other direction: the
  grok-seated project-owner desk `0-projects:factory-tui` is unmarked.
  (C1)
- **D2 — no sampler model.** `[status]` is `running: Vec<String>` exact
  string equality on one field, plus a `parked_substring` special case.
  D-2026-08-13-status-samplers requires *named samplers, field + regex →
  status*. No field selector, no regex, no names. (C1, C2)
- **D3 — the sampler model cannot work over today's fields.** The tmux
  `-F` query at `src/tmux.rs:68` carries no field that separates a
  thinking agent from a waiting one. Extending the query is part of the
  fix, not an optimization; otherwise the new model is incapable of its
  purpose by construction. (C2)
- **D4 — census cannot select recipes.** `skills/factory-tui/scripts/census`
  emits `session\twindow` only. `SKILL.md` step 2 tells an agent to
  census and then pick recipes matching the box; the shipped instrument
  cannot answer that. The ruling is unimplementable as shipped. (C5)
- **D5 — no default sampler recipe data.** The ruling requires a default
  recipe set "as data the skill can copy". No such file exists; the only
  status data is `examples/projection.toml`'s wrong-shaped
  `running = ["claude", "codex", "codex-raw"]`. (C5)
- **D6 — the published schema publishes the defect.**
  `skills/factory-tui/references/config.md` documents the exact-name
  model, so an operator's agent filling a file from it reproduces the
  lie. The schema is published (#13) but nothing binds it to the crate's
  structs. (C3)
- **D7 — shipped example is untested.** No test loads
  `examples/projection.toml`; there is no `tests/` directory. It can rot
  through any schema change, including E-A's. (C4)
- **D8 — agent-agnosticism is unenforced.** `I5-NO-HOSTNAMES` is
  declared in `specs/5-raw-tmux-tree/{spec,plan}.md` but appears in
  neither `nix/checks.nix` nor `.github/workflows/ci.yml`. Nothing stops
  host names re-entering `src/`. Its declared scope (`src/`) is also
  narrower than the operator premise. (C6)
- **D9 — no installable milestone artifact.** See C7 and T-B. (C7)

Explicitly **not** M3 work: M1 validation and the M1 research pages
(D-2026-08-13-m1-unvalidated); tagging, packaging or announcing the
product release.

## Parked decisions

| Id | Question | Holder | What unblocks it |
|---|---|---|---|
| Q-001 | Host session names (`keri`, `0-machine`, `0-projects`) sit in `docs/m1/flat-land.md:9,45`. The privacy premise says scrub; D-2026-08-13-m1-unvalidated says leave M1 pages alone; this brief says no M1 work. Which ruling wins for these lines? | project owner | a ruling: scrub-in-place as a privacy fix, or waive with the leak recorded, or defer to an M1 milestone |
| Q-003 | RESOLVED 2026-08-13 by A-003: pinned publisher cannot mark a pre-release. Ruled to use a non-`v*` tag with no release object rather than change shared infrastructure. | this desk | closed |
| Q-002 | Does "no release pipeline act (tag/publish/announce)" forbid cutting the **milestone pre-release** tag, or only the product release? T-B's capability work proceeds either way; only the first tag push depends on the answer. | project owner | a ruling naming pre-release tags in or out of M3's hands |

## Registry mismatch reported upward

`/code/llm-settings/shared/milestones.md` has no line for this milestone.
Every other ACTIVE desk is listed there and a cold-start resurrector reads
it first. Registering is the project owner's duty, not this desk's;
reported, not edited. Proposed line:

```
ACTIVE | lambdasistemi/factory-tui | 3 | M3 — No-bugs (release satisfaction) | factory-tui-ms3-no-bugs (session factory-tui; runtime /tmp/ms-3)
```

## Escalations in flight

Q-001 and Q-002 to the project owner (`0-projects:1:factory-tui`, pane
`%6361`), delivered to its inbox per protocol. Neither blocks founding,
the map, or E-A dispatch.
