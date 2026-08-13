# M3 session — how to reopen and resume every seat

Session `factory-tui` on the development host. A stranger with tmux and
git rebuilds M3 from this file alone. Runtime roots under `/tmp` die
with the host; this file does not, which is why it lives here.

Window naming: `factory-tui-ms3-<goal>` for the desk,
`factory-tui-ms3-e<id>-<goal>` / `factory-tui-ms3-t<id>-<goal>` for
children.

---

## Window `factory-tui-ms3-no-bugs` — the milestone desk (LIVE)

Singleton, one pane. Why it exists: M3 has no code, no pairs and no
slices; its workers live in their own windows. A quadrant here would be
three dead seats inviting work that must never happen at this altitude.

- Session/window: `factory-tui:2:factory-tui-ms3-no-bugs`
- Pane: `%6420`
- Role skill: `milestone-orchestrator`
- Load chain: orchestrator-contract → milestone-orchestrator →
  context-compiler → worker-protocol → tmux-orchestrator → invariants
- cwd: `/home/paolino`  ·  Worktree: none at this altitude
- Runtime root: `/tmp/ms-3`
- Parent: project owner `0-projects:1:factory-tui` pane `%6361` (grok,
  operator-authorized seat)

Launch (exact — replay including the quotes):

```sh
claude --dangerously-skip-permissions --model 'claude-opus-5[1m]' --effort high
```

Resume paste:

```
Read /tmp/ms-3/resume/ms.md in full and continue as the M3 milestone owner.
```

If `/tmp` was lost, first re-pull the ledger, which is the real record:

```sh
/code/llm-settings/shared/skills/milestone-orchestrator/scripts/ledger-sweep.sh \
  https://github.com/lambdasistemi/factory-tui 3 pull
```

then read `.milestones/3/resume/ms.md` from the checkout.

---

## Window `factory-tui-ms3-e-unknown-status-samplers` — E-A epic (LIVE)

Built by the machine owner 2026-08-13 on request; this desk does not
create windows or panes. Brief hash was verified at build time.
Acknowledged in substance; its first journal line used a hand-written
`ACK` rather than `status-event` `START` — corrected by NOTE-001, since a
hand-written tag column is invisible to every wait a supervisor arms.

- Role skill: `epic-orchestrator` (loads `resolve-epic`)
- Quadrant: epic owner top-left, ticket owner top-right, commit owner
  and work slot below — built by the epic owner, not by this desk
- Window/pane: `factory-tui:3` (@4478) pane `%6422`
- Runtime root: `/tmp/ms-3/e-a`
- Brief: `/tmp/ms-3/e-a/brief.md`
- Owner CLI: `codex-raw`, `gpt-5.6-sol`, effort high — alternate family from this claude desk

Launch (exact):

```sh
codex-raw --dangerously-bypass-approvals-and-sandbox \
  -C /code/factory-tui -c model_reasoning_effort=high
```

Resume paste:

```
Read /tmp/ms-3/e-a/brief.md in full and continue as the E-A epic owner.
```

---

## Window `factory-tui-ms3-t-unknown-prerelease-line` — T-B ticket (LIVE)

Built by the machine owner 2026-08-13 on request. Standalone ticket owned
directly by this desk: the milestone artifact blocker fits no epic's arc.
Canonical `START` received. Its first pre-release tag push is held pending
Q-002; everything upstream of that push proceeds.

- Role skill: `ticket-orchestrator` (loads `resolve-ticket`)
- Window/pane: `factory-tui:4` (@4479) pane `%6423`
- Runtime root: `/tmp/ms-3/t-b`
- Brief: `/tmp/ms-3/t-b/brief.md`
- Owner CLI: `codex-raw`, `gpt-5.6-sol`, effort high — alternate family from this claude desk

Launch (exact):

```sh
codex-raw --dangerously-bypass-approvals-and-sandbox \
  -C /code/factory-tui -c model_reasoning_effort=high
```

---

## Not this milestone's seats

- `factory-tui:1:factory-tui-e8-t5-raw-tree` pane `%6385` — leftover M2
  ticket lane. Not the M3 desk, not directed from here, not killed by
  here. It is also M3's live reproduction of the false-RUNNING defect.
- `0-projects:1:factory-tui` pane `%6361` — the project owner. Parent,
  not a child. Upward traffic goes to its inbox as a file; never a
  pointer injected into its pane.

---

## Host condition every lane on this box inherits

Reported by the machine owner (`%5234`) 2026-08-13, measured not assumed:

- Headroom above `min-free` was ~10 GiB, and it existed because of a
  hand-run garbage collection, not because the host is fixed.
- `min-free` (48 GiB) sits above where this machine actually operates and
  `max-free` (80 GiB) is unreachable, so allocations oscillate across the
  trigger — ~2.8 GiB of swing inside one minute, trigger crossed twice in
  fifteen.
- Crossing it seizes the exclusive collector lock and hangs **every**
  realisation on the host, not only the one that crossed.

Standing instruction passed to both lanes: re-measure in exact bytes
immediately before any Nix-realising command, and **if one hangs with no
visible collector, stop and report a disk event — never retry into the
lock.** The permanent threshold correction sits with the operator.

A resurrector should assume this is still true until the operator says
otherwise.
