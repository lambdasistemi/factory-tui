# Tree model — operator decisions, 2026-08-13

## The report

Six windows in session `factory-tui`; only three appeared under the
project. Reproduced: the other three rendered under a **second, identically
titled** `• factory-tui` node, under their raw names.

Operator verdict: *"this is brittle, we should simply have session →
window → pane"*, then: **delete** the projection, and *"what would be
great is reinterpreters for the cryptic names."*

## D-TREE-1 — the tree is session → window → pane. Raw.

No synthesized folders, no matching, no fallback bucket. Every session,
window and pane appears exactly once, where tmux actually has it.

Measured 2026-08-13: 16 sessions, 43 windows, 72 panes — 131 nodes fully
expanded against a 73-line projected dump, but **16 lines collapsed at
session level**, and **26 of 43 windows have exactly one pane**.

## D-TREE-2 — pane nodes only where a window has more than one pane.

For a single-pane window the pane *is* the window; a child node there is
noise. This is a display rule, not a filter: nothing is hidden, because
the pane and the window are the same thing at that point.

## D-TREE-3 — the projection is deleted, not disabled.

`[[rule]]`, `[tree]`, folders, `desk_roles`,
`inherit_milestone_from_desk` and the folding code go. "Optional" did not
protect the operator: they enabled it for a real reason — 43 windows
across 16 sessions — and it silently dropped half their project. A
documented schema in a public repo that mis-folds anyone following the
naming conventions we ourselves publish is a shipped defect with an off
switch, not a feature.

This partially undoes what M2 shipped ("tmux browser + optional
projection"). Proceeding on direct operator authority; the scope record
goes to the project owner.

## D-TREE-4 — reinterpreters replace projection, and are label-only.

Cryptic window names become readable **without touching structure**.

The distinction is the whole point, and it is what makes this safe where
projection was not:

| | projection (deleted) | reinterpreter |
|---|---|---|
| changes | the **structure** of the tree | the **label** of a node |
| failure to match | window silently leaves the project tree, or forks it into a duplicate node | the raw name is shown |
| worst case | the browser lies about what exists | the browser is ugly |

A reinterpreter is **total**: every node renders, matched or not. It
cannot drop a window, cannot invent a folder, cannot fork a node. That
property is a requirement, not an implementation detail — a reinterpreter
that can hide a node has become a projection and must be rejected in
review.

Requirements:

- Reinterpreters live in **configuration as data**, not in the crate —
  consistent with D-2026-08-13-status-samplers. The crate evaluates
  configured reinterpreters and knows no operator's naming grammar.
- Unmatched name ⇒ raw name. Always. No exceptions.
- The raw name must stay **reachable** — a reader has to be able to
  recover what tmux actually calls the thing, or the browser has replaced
  one cryptic label with an unverifiable one.

## Consolidation note — same shape as the status samplers

Status samplers are *field + regex → status*. Reinterpreters are
*field + regex → label*. That is one mechanism with two output types, and
the recurrence is worth naming before both are built: two bespoke
frameworks with one shape is exactly the duplication a consolidation
ticket exists to prevent.

Flagged to E-A while #16 is still pre-implementation. Not mandated —
whether #16 designs for the second output now or is refactored onto a
shared evaluator later is its owner's judgement, and speculative
generality is its own defect. Recorded here so the choice is deliberate.

## Sequencing

Queued behind T-E (#24). The operator can still not tell which build they
are looking at, and that outranks tree shape. D14/D15 are **not** fixed by
patching the regexes — they are deleted along with the machinery that
produced them.

## D-TREE-5 — a reinterpreter is structurally inert. Enforced, not intended.

Operator, sharpening D-TREE-4: **"no structural interpolation or worse
splitting."**

So a reinterpreter is a pure function from one node to one display
string. It may not:

- **interpolate** — insert a synthesized intermediate level (a
  "project"/"milestone"/"epic" folder). That is projection wearing a new
  name, and it is what was just deleted.
- **split** — turn one window or pane into several nodes, or partition
  the node set. The operator named this as the worse failure, and it is:
  interpolation adds something a reader can see is invented, while
  splitting makes one real thing look like two, which is the D15 duplicate
  `• factory-tui` defect in a new costume.
- **merge, re-parent, hide, or reorder** — all the same class.

### The enforceable form

> For any tmux state and any reinterpreter configuration, the set of
> nodes and the parent relation are **identical** with and without
> reinterpretation applied. Only display strings differ.

That is a property, not a code-review sentiment, and it must ship as a
check that has been **shown able to fail** — write a reinterpreter that
attempts to split a node and require the check to go red. Any design in
which that property cannot be stated is the wrong design: it means
labelling and structure are not separated in the code, and the separation
is the entire safety argument.

Structure comes from tmux. Reinterpreters are cosmetics over it. If a
reinterpreter can change what exists, it has become a projection and is
rejected.
