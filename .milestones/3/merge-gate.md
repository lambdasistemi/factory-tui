# Merge gate — F-key testable before merge

Operator goal, 2026-08-14: *get all feature PRs testable via F keys
before merging.* This is an acceptance rule, not a preference. The
milestone owner authorizes merges, so it is enforceable at that point.

## The rule

No feature PR is authorized to merge until **the operator can run it from
an F key and tell it apart from every other build.** Three conditions,
all the lane's to satisfy before requesting authorization:

1. **Rebased onto current `main`** — actually rebased, not merely
   conflict-free. A branch cut before #24 produces a binary that cannot
   report which build it is, and a key bound to an anonymous build is not
   a test.
2. **Proven to run**, by the lane, with the real output:
   ```
   nix run 'git+https://github.com/lambdasistemi/factory-tui?ref=<branch>' -- --version
   ```
   The printed revision must equal the pushed head. Use `git+https:` —
   `github:` is HTTP 403 on this host. Tags need `?ref=refs/tags/<tag>`;
   `?ref=<tag>` and `?tag=` both fail with misleading errors.
3. **Exact command and exact output journalled** in the handback, so the
   operator binds a key from the record rather than reconstructing it.

## Why

The operator is the only user. Until 2026-08-14 they were testing the
*released* build — which predates this entire milestone. #22, #24 and #26
all merged without the operator ever running them.

That is the milestone's own subject turned back on the team: we asked
them to trust claims about software instead of letting them run it. A PR
the person it is for cannot run is not finished, however green its checks.

## Standing exception, stated so it is not silently assumed

`F1` is the released build and **cannot satisfy this rule**. `v0.1.0` has
the version defect baked in; the pristine tag builds to the identical
store path as the installed profile
(`rc3343awy1rkvlwya9gngyr1lbvdi1h1-factory-tui-0.0.1`), so reinstalling
cannot fix it. Only a new release can, and that is a release act outside
M3's hands — escalated as PR #29.

Until then the comparison available to the operator is "a current build
that identifies itself" against "a released build that cannot answer".
That is distinguishable, but it is not the clean before/after asked for,
and it should not be reported as if it were.

## Applied

- `main` @ `ea047e04` — verified runnable and self-identifying:
  `factory-tui 0.1.0 (revision ea047e04edbcb1ab2ecad2f91b5442d6494903e4)`,
  raw tree renders all six windows of one session with pane children and
  real tmux titles.
- `#20` / `ci/19-m3-preview-tag` — was 12 commits behind; rebase ordered
  and in progress.
- `#21` / `fix/16-evidence-samplers` — was 12 commits behind; rebuild on
  `ea047e04` ordered through the epic owner, since A-002 already moved
  #16's subject to pane-level sampling and panes only became nodes in #26.

## Lane incident recorded

T-B was **stale 13 hours** — idle at its composer after a note-only
acknowledgement, with no terminal event, while not actually blocked. It
self-diagnosed on being challenged: it had ended a turn after
acknowledging a note while its dependency PR #25 was unaccepted. A lane
that goes quiet without a terminal event is indistinguishable from a lane
that died. The `COMPLETE`-on-every-stop rule exists for exactly this and
was not applied.
