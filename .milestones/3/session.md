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

## Window `factory-tui-ms3-e<id>-status-samplers` — E-A epic (REQUESTED)

Not live. Requested from the machine owner; this desk does not create
windows or panes.

- Role skill: `epic-orchestrator` (loads `resolve-epic`)
- Quadrant: epic owner top-left, ticket owner top-right, commit owner
  and work slot below — built by the epic owner, not by this desk
- Runtime root: `/tmp/ms-3/e-a`
- Brief: `/tmp/ms-3/e-a/brief.md`
- Owner CLI: `codex` — alternate family from this claude desk

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

## Window `factory-tui-ms3-t<id>-prerelease-line` — T-B ticket (PLANNED)

Not live and not yet requested. Standalone ticket owned directly by this
desk: the milestone artifact blocker fits no epic's arc.

- Role skill: `ticket-orchestrator` (loads `resolve-ticket`)
- Runtime root: `/tmp/ms-3/t-b`
- Brief: `/tmp/ms-3/t-b/brief.md`
- Owner CLI: `codex` — alternate family from this desk

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
