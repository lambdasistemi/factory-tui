# Resume — M3 milestone owner (written at OMNIA PAUSA 2026-08-14T14:59:45Z)

You are the milestone owner for factory-tui M3 (no-bugs). **The machine
is PARKED.** Do not restart work. Release comes only from the machine
owner, scoped and in writing, through the project owner. Silence is not
release.

Load: orchestrator-contract → milestone-orchestrator → context-compiler →
worker-protocol → tmux-orchestrator → invariants.

## Seat

- Window `factory-tui:2:factory-tui-ms3-no-bugs`, pane `%6420`
- Launch (exact, replay including quotes):
  `claude --dangerously-skip-permissions --model 'claude-opus-5[1m]' --effort high`
- Runtime `/tmp/ms-3`; ledger `.milestones/3/` on the `milestones` branch
- Parent: project owner `0-projects:1:factory-tui` pane `%6361`

## State at the pause

`main` = `0ba974e22db10a7543d5fbbe31b6e6bf58bddc5c`. Four tickets merged
today: #24 build provenance, #22 pane titles, #26 raw tree, #19 preview-tag
guard. Nothing was mid-build or mid-slice; no candidate abandoned.

Operator F-keys, bound and verified (targets checked executable):

```
F1  released profile           /nix/store/rc3343...-factory-tui-0.0.1   (cannot self-identify)
F2  main @ 0ba974e2            /nix/store/iqj8y9nn...-factory-tui-0.1.0
F3  #21 @ 0177eb08             /nix/store/i904dzhg...-factory-tui-0.1.0
    env FACTORY_TUI_CONFIG=/home/paolino/.config/factory-tui/config.sampler-preview.toml
```

## Exact next action on RELEASE

1. **Do not merge #21 without an operator ruling.** It is verified and
   complete, but exits 2 on the operator's existing config (removed
   `[status]` table rejected). The break is theirs to accept, not this
   desk's to absorb. If they accept: re-verify the F-key proof against
   the then-current `main`, then authorize.
2. Confirm lanes resumed and re-verify F2/F3 still match their heads;
   repoint if `main` moved.
3. Chase the three decisions below if still open.

## Open decisions, all above this desk

| Id | Question | Holder | Recommendation |
|---|---|---|---|
| #21 break | accept a config-breaking change for the only user? | operator | accept — silent ignore is worse; hard failure names its replacement |
| Q-002 | may M3 push its milestone-scoped tag? | project owner / operator | yes — no publisher runs, no release object, cannot displace the product line |
| PR #29 | merge `chore(main): release 0.1.1`? | project owner / operator | merge — the only thing that makes F1 honest; `v0.1.0` is immutably wrong |
| Q-001 | host names in `docs/m1/flat-land.md` | project owner | scrub as a privacy fix, widen the C6 check to `docs/` |

## Children (parked, not torn down)

- `e-a` — epic #15. #21 verified, merge held. #17/#18 undispatched.
- `t-b` — #19 accepted/merged; root NOT archived because its remaining
  scope is the first milestone tag push, blocked on Q-002.
- `t-d`, `t-e`, `t-f` — complete (#22, #24, #26).

## Standing constraints carried through the pause

Floor subject is `df -B1 --output=avail /nix/store`, never cwd. `min-free`
is a correctness risk. A store-invalid/missing-path/untouched-broken-recipe
failure is a possible machine event — stop and report, never retry.
`git+https://` not `github:`. Families: claude, codex, grok; agy revoked,
qwen draft-only. F-key merge gate binds every feature PR. Five contracts
still read `enforced: NONE` and M3 cannot be accepted while any is silent.

## Carried in from the pause — realizing hooks (2026-08-19)

`git commit` can realize via a commit hook and must then take build
tokens like any build; `--no-verify` is not an escape. Verified across all
10 factory-tui worktrees on 2026-08-19: **0 live hooks**, no
`core.hooksPath` anywhere, no lefthook/pre-commit/husky configs (positive
control: 14 `.sample` files listed, so the check was working). That is
point-in-time — **re-verify before the next commit**. Ledger sweeps are
exempt by construction: `git commit-tree` runs no hooks.
