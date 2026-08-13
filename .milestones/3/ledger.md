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
| T-D (#22) | ticket | Pane boxes carry the real tmux pane title, so a reader can tell which seat is which | **LIVE** pane `%6433`, PR #23 draft; `#{pane_title}` was never queried at all |
| T-E (#24) | ticket | Truthful version and build provenance: kill the hardcoded 0.0.1, make a build say which commit it is | **LIVE** pane `%6438`; ranked FIRST — operator-reported, and nothing else is distinguishable until it lands | `factory-tui-ms3-t-unknown-build-provenance` |
| T-F | ticket | Delete the projection; tree becomes raw session -> window -> pane; label-only reinterpreters replace folding | QUEUED — decisions recorded, milestone-owned (NOT inside E-A: it reverses M2 and spans structure/config/docs/skill) | not yet requested |
| T-C | ticket | Preview a WINDOW as a composite quadrant, each pane filled with its own content; zoom-on-demand; hybrid refresh | QUEUED — design settled, sequenced after T-D (same `ui.rs` region) | not yet requested |

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

0. **T-E (#24)** — promoted above everything on operator report. The
   artifact misreports its own version, so no build can be told from any
   other. Both the branch-key workflow and a distinguishable milestone
   artifact are blocked behind it, and a milestone audited by "a stranger
   installs the artifact" cannot be audited while the artifact lies about
   which build it is.
0b. **T-F** — the raw-tree reversal. Ordered ahead of the sampler work
   because building status semantics against a tree shape that is being
   deleted wastes the work.
1. **E-A** — the milestone's named defect, deliberately pushed behind
   T-E, T-D and T-F. The product's central claim
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

- **D10 — the preview previews the wrong thing.** `ui.rs` `draw_right`
  renders a schematic of empty bordered boxes (height capped at 5 rows)
  plus a separate text block, and `app.rs refresh_preview` captures one
  pane only. So a multi-pane window shows empty boxes and one filled
  block. This is the design, not a rendering fault: the preview is
  pane-scoped while the jump target is the window. Operator-reported and
  confirmed by reading the source. Fix: window composite quadrant. (T-C,
  see `preview-decisions.md`)
- **D11 — pane identity is illegible.** `#{pane_title}` is never queried
  (zero occurrences in the crate); boxes are titled `{index}:{cmd}`, so
  every claude seat reads `N:claude`. tmux already holds good titles.
  Operator-reported. (T-D)

- **D12 — the artifact lies about itself.** `nix/crane.nix:16` hardcodes
  `version = "0.0.1"` — the only `version` in `nix/` — so the `v0.1.0`
  release installs as `factory-tui-0.0.1` while `Cargo.toml` says
  `0.1.0` at that tag. release-please's bump has never reached an
  artifact. There is also **no runtime version surface at all** (zero
  `--version`/`CARGO_PKG` in `src/`), so two builds from different
  commits are indistinguishable except by store hash. Operator-reported
  as "the lie about F1". (T-E / #24)
- **D13 — nothing binds F1 to the release.** The popup binding is bare
  `factory-tui` off `$PATH`, so it is the released build only by
  convention; a profile change silently repoints it while it is still
  described as the release. Same class as false RUNNING: a claim with no
  enforcing mechanism. Folded into T-E's provenance surface — a reader
  must be able to see what they are running.

- **D14 — the shipped projection rules do not cover the standard
  `no-epic-t<id>` window convention.** Verified 2026-08-13:
  `factory-tui-no-epic-t19-m3-preview-tag` matches neither the `e<N>-t<N>`
  rule nor the `ms<N>` rule in `examples/projection.toml`. The
  `<repo>-no-epic-t<id>-<goal>` name is the documented convention for a
  standalone ticket lane, so every such window silently leaves the
  project tree. Operator-reported: "I see only 3 child" of 6 windows.
- **D15 — an unmatched window forks the project into two identical
  nodes.** Unmatched windows fall back to their tmux session bucket. When
  the session name equals a project name — the normal case — `--dump`
  renders **two sibling `• factory-tui` nodes**: the projected one, and a
  raw one holding the unmatched windows under their full raw names. A
  reader cannot tell which is authoritative, and the project appears
  twice. This is a crate defect independent of any config, and it is the
  same family as false RUNNING: the browser misrepresenting reality, here
  about structure rather than status.

Explicitly **not** M3 work: M1 validation and the M1 research pages
(D-2026-08-13-m1-unvalidated); tagging, packaging or announcing the
product release.

## Running an in-flight build — proven mechanism

Any branch runs without installing, without touching the profile, and
without the GitHub API:

```
nix run 'git+https://github.com/lambdasistemi/factory-tui?ref=<branch>'
```

Verified 2026-08-13: exit 0, zero disk when cached. **Do not use the
idiomatic `github:owner/repo/<ref>` form** — Nix resolves it through the
GitHub API, which is HTTP 403 rate-limited on this host; `git+https:`
uses the git protocol and never touches it.

This is how the operator tests in-flight work instead of only the release
(F1 = released profile build, another key = a branch). It is inert until
T-E lands, because every build currently reports `0.0.1`.

## Preview model

Operator-reported defects D10/D11 and the decisions that settle them are
in `preview-decisions.md` beside this file. Headline: **the preview
subject is the window, not a pane** — a window renders as a composite
quadrant with every pane filled from its own capture; Tab/click zooms one
pane for scrollback; `r` and window-selection recapture every pane while
the timer refreshes only the focused one.

Scope note: this is new behavior beyond the brief's "only to stop a lie"
limit. It proceeds on direct operator intent and the scope record is
routed to the project owner. Not blocked on that routing.

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
