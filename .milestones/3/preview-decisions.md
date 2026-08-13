# Preview model — operator decisions, 2026-08-13

Source: the operator, direct, through the M3 desk channel. Recorded here
because a conversational bug report that is not written down is a
requirement that dies with the pane.

## The report

1. With multiple panes, the preview shows empty boxes and one box with
   text. Concern raised about pane sampling generally.
2. Pane identity is not legible — you cannot tell which tmux seat a box
   is.
3. "When I think about preview, I think about **windows**, not panes" —
   the window is the jump target, so the window is what should be
   previewed, as a quadrant.
4. A refresh button that genuinely rebuilds the composite from current
   content.

## What was proved before deciding

- The pane-scoped preview is the **design**, not a glitch. `ui.rs`
  `draw_right` splits into a *schematic* of empty bordered boxes (height
  capped at 5 rows) and a separate text block below; `app.rs
  refresh_preview` calls `tmux::capture_pane` once, for `preview_pane`
  only. Empty boxes plus one filled block is exactly what it builds.
- The composite is feasible: `geometry.rs::rects_for` already computes
  per-pane rects and `hit` already hit-tests them for clicks. Only the
  fill is missing.
- Real geometry supports real content: `cna-214` is 149x69 with four
  panes; scaled into a ~70-column preview that is roughly 49x30 per pane.
- `#{pane_title}` is **not queried at all** (zero occurrences in
  `src/tmux.rs`). Boxes are titled `{index}:{cmd}` — hence "2:claude".
  tmux already holds good titles (`✳ cna-215-ticket-owner`,
  `Read and acknowledge auditor brief`, `development`).
- `r` already calls `refresh()`, which re-queries `tmux::query_all()` for
  fresh pane membership and geometry. It only *feels* broken because the
  recapture step touches one pane. It becomes correct for free once the
  composite exists; no new key is needed.

## Decisions

- **D-PREVIEW-1 — the preview subject is the window.** A selected window
  renders as a composite quadrant: every pane drawn in its true relative
  geometry, each filled with that pane's own captured content.
- **D-PREVIEW-2 — quadrant default, zoom on demand.** Tab or click zooms
  one pane full-size, where scrollback still works. The per-pane
  *browser* as the only way to see content is removed; per-pane *history*
  is kept.
- **D-PREVIEW-3 — hybrid refresh.** Capture every pane on `r` and on
  window-selection change. The 800ms timer refreshes only the focused or
  zoomed pane. Bounded cost; `r` does what the operator asked.
- **D-PREVIEW-4 — content slice is the tail** of each pane's output,
  which is where activity is.
- **D-PREVIEW-5 — pane identity comes from `#{pane_title}`**, added to
  the tmux query and used in box titles.

## Sequencing

T-D (pane titles) lands first and T-C (composite) rebases on it. Both
rewrite the same region of `ui.rs`, so running them in parallel buys a
merge conflict for no gain. T-D is small and independently useful — it
improves even today's schematic — so the operator gets legible pane
names without waiting for the larger change.

## Scope

This is new product behavior beyond "only to stop a lie", which is the
limit set in the M3 brief. It proceeds on direct operator intent, which
outranks that limit; the scope change is recorded and routed to the
project owner, who owns milestone scope. Not blocked on that routing.
