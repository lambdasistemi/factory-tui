# Resume — M3 milestone owner

You are the milestone owner for factory-tui M3 (no-bugs). Continue; do
not restart. Load: orchestrator-contract → milestone-orchestrator →
context-compiler → worker-protocol → tmux-orchestrator → invariants.

Read first, in this order:
1. `.milestones/3/ledger.md` — outcome test, units, priority, defect map,
   parked decisions
2. `.milestones/3/registry.md` — seven contracts, all `enforced: NONE`
3. `.milestones/3/session.md` — how to reopen every seat
4. `/tmp/ms-3/STATUS.md` if `/tmp` survived — the journal

## Where things stand

Milestone founded 2026-08-13. Ledger, registry, and state page published.
GitHub description verified against the live milestone — no drift, so it
was deliberately **not** republished. Wiki `M3-State` published.

Defect map complete and evidence-backed: D1 (false RUNNING) reproduced on
the live box, the rest read from source at `main`. No child had been
accepted at the time of writing.

## Exact next action

1. Confirm the machine owner seated `factory-tui-ms3-e<id>-status-samplers`.
   If seated, deliver `/tmp/ms-3/e-a/brief.md` with `send-pointer` and
   require a post-cursor `START`. If not, chase the lane request.
2. Request the T-B lane and write `/tmp/ms-3/t-b/brief.md`.
3. Answer Q-001/Q-002 when the project owner rules; both are recorded in
   `ledger.md` and neither blocks E-A.

## Standing constraints

- Hands are asks, answers, sweeps. Nothing else — no product code, no
  `guard-merge`, no filing an issue a child can file, no directing a
  grandchild.
- Authoritative children: Claude and Codex. Grok only on a cited operator
  order for that exact seat. `agy`/`qwen` are draft-only.
- No M1 work. No release act (tag/publish/announce of the product line).
- Never leak this host's private factory skills or aliases into the
  public repo.
- Escalate product policy beyond this milestone to the project owner
  `%6361`, by file in its inbox — never the machine owner.
- The outcome audit runs against the **published artifact**, obtained as
  a stranger obtains it. A source build does not satisfy it, and
  `MILESTONE-COMPLETE` is never reached by counting closed children.
