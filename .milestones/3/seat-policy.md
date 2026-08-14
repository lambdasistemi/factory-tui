# Seat policy — commit owners

Operator directive, 2026-08-14, verbatim:

> make sure we use agy flash 3.7 or grok as commit-owners

## Resolved launch line

Do not rediscover this. The epic owner resolved "flash 3.7" against the
available models rather than guessing, and journalled it:

```sh
agy --dangerously-skip-permissions --model gemini-3.7-flash-high --effort high
```

Alternative authorized seat: `grok`.

## What still binds

The directive changes **who may hold the commit-owner seat**. It waives
nothing else:

- the commit-owner family must differ from the ticket-owner family
  (ticket owners here are `codex-raw`, so both `agy` and `grok` qualify);
- the auditor family must differ from the commit owner — with a Gemini
  commit owner the auditor must be non-Gemini;
- visible tmux dispatch and a post-cursor `START` naming pane and family
  remain the precondition for admitting any claim from the seat;
- acceptance evidence is unchanged, including the F-key merge gate and
  the both-directions proof rule.

## In-flight consequence, recorded rather than glossed

At the moment the directive landed, #16 had an **active `codex` commit
owner** mid-slice. The epic owner's reading is that it must policy-stop
and be replaced by a fresh visible `agy` owner. That is the correct
reading of a standing directive: a seat authorized under the old policy
does not grandfather itself by being busy.

The cost is real — a mid-slice replacement discards in-progress work and
the successor starts from the frozen mandate and gate, not from the
predecessor's context. That cost is the operator's to have chosen, and it
is smaller here than it looks because #16 was already being rebuilt from
a re-cut mandate on a new base.
