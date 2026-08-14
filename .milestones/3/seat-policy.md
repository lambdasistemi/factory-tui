# Seat policy — commit owners

## CURRENT (D-2026-08-14-agy-revoked, 2026-08-14 ~12:00Z)

**Standing authorized families: `claude`, `codex`, `grok`.**

- Launch **no** `agy` / Gemini Flash seats for any role — not commit
  owner, researcher, driver, or auditor.
- Alternation still binds: commit owner ≠ ticket owner; auditor ≠ commit
  owner.
- An Opus audit is **not** a licence to keep `agy`. The removal rests on
  sealed reliability evidence and is not a retry.

A resurrector reading this file must not seat `agy` under any
circumstances. The section below is retained only so the historical
record of #16 is intelligible.

## SUPERSEDED — the morning's allowance (dead, do not act on)

Between ~09:20Z and ~12:00Z on 2026-08-14, the operator directed:

> make sure we use agy flash 3.7 or grok as commit-owners

resolved by the epic owner to:

```sh
agy --dangerously-skip-permissions --model gemini-3.7-flash-high --effort high
```

under a scrutiny condition: the ticket owner authors the gate and design,
and every `agy` candidate is audited by Claude Opus 5 at high effort.

That allowance was withdrawn the same day. It is recorded because #16's
accepted-on-merit candidate was produced under it.

## What happened to the one seat that ran

`%6511` in the #16 lane. Retired at a safe boundary after capture, not
killed mid-work. Verified by this desk rather than relayed:

```
pane %6511                            gone
gemini-3.7-flash processes host-wide  0
terminal owner event                  COMPLETE, provider-retired,
                                      no-edits / no-commits / no-pushes
```

Its candidate `cafc64838e8b0fc1efc01fefc27ef731f5285240` was **not**
discarded. The ruling forbids discarding finished work to look compliant,
so it stands or falls on merit. Evidence, hash-verified by this desk
against real files:

```
audit-report.md        836f8e3dd70a46e8...   verdict pass, 0 blocking, 5 advisories
gate-v8-candidate.log  9dfec8ce...           gate-v8 GREEN receipt
capture                /tmp/ms-3/e-a/t16/handoffs/agy-retirement-capture.md
archived auditor root  /tmp/ms-3/e-a/t16/.archived/auditor-2-audit-pass/
```

The auditor left frozen mutation harnesses (`mutate.sh`, `mutate2.sh`,
`mutate3.sh`) and a `gate-falsify.sh` with falsification logs — a real
audit, not a rubber stamp.

## The interpretation trap, recorded because it will recur

The epic owner first applied the revocation **prospectively only** — "no
new agy seats" — leaving the live seat running. That is the natural
reading and it is wrong: a revocation has two clauses, and a seat
authorized under a withdrawn policy does not grandfather itself by
already being seated. Corrected in one exchange; flagged upward so other
desks landing on the same reading are caught.
