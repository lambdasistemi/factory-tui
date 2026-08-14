# Report RUNNING only from named evidence samplers

Issue: #16 · Parent: #15 · Base: `main@ea047e04` (post-#26)

Mandate **v3 (recut)**. Every earlier mandate, gate, receipt, and falsification
for this ticket is superseded reference only and carries no standing evidence.

## Problem

`factory-tui` marks a window RUNNING when any pane's `pane_current_command`
appears in a configured list. An agent CLI keeps that command name while it sits
waiting for its operator, so occupancy is presented as work. An operator
scanning the popup for something to attend to is told the opposite of the truth.

## Outcome

Status is sampled per pane from named evidence samplers configured as
`field + regex -> status`. A pane is RUNNING only when queried evidence matches
a sampler that says so. Waiting panes and unknown agents stay unmarked.

## User stories

- As an operator, I see RUNNING only where an agent is actually working, so the
  popup ranks by attention needed rather than by which seats are occupied.
- As an operator, I express what "working" looks like for my own tools in
  configuration, because the crate knows no agent names.
- As an operator, a typo in a sampler field name fails loudly at config load
  rather than silently never matching.

## Requirements

- **R1** Replace the legacy `[status]` table — `running`, `idle`,
  `parked_substring` — with ordered named samplers.
- **R2** Every supported sampler field must be queried by the crate and
  resolvable to its observed value. A field the crate does not query may never
  be declared supported.
- **R3** Validate sampler `field`, `status`, `regex`, and `name` at config load.
  An unsupported field is an error naming the field, never a silent non-match.
  A config still carrying a removed `[status]` key is rejected.
- **R4** Hard-code no agent, tool, or product name in Rust. Unknown agents and
  insufficient evidence stay unmarked.
- **R5 (status is a pane property)** Sample each pane independently. A window or
  ancestor rolls up established child status. An unmarked pane contributes
  nothing. A window whose panes are all unmarked is **unmarked, not idle**.
- **R6** The published schema and the shipped `examples/config.toml` match the
  accepted crate model, in this PR, each bound by a check.

## Inherited surfaces this ticket must NOT change

- `src/tmux.rs` query and parsing (#22 owns it). `pane_title` is already
  queried and carried on `Pane`; #16 **consumes** it.
- `src/label.rs` reinterpreters (#26). They are label-only and structurally
  inert. #16 must not change their semantics.
- `src/build_info.rs` and `tests/cli_version.rs` (#24). `--version` already
  prints `factory-tui {version} (revision {revision})`.
- Projection/folding is gone. Do not reintroduce it in any form.
- The raw tree is session -> window -> pane, pane children only for multi-pane
  windows. #16 does not build it.

## Rejection behavior

A configuration is rejected at load, naming the offending sampler and field,
when it:

- names a `field` outside the supported set;
- names a `status` outside `running` / `idle` / `parked`;
- carries a `regex` that does not compile;
- has an empty or duplicate `name`;
- still uses a removed `[status]` key.

The last case matters as much as the others: silently ignoring removed keys
turns an operator's upgrade into total silent loss of status marking — the same
silent non-match R3 forbids, arriving by a different door.

## Observable success

- A waiting-pane fixture is not RUNNING; an active-evidence pane is RUNNING.
- Restoring the old command-occupancy decision makes the waiting check fail.
- An all-unmarked window is unmarked, not idle, and rollup is proven.
- Every supported field is proven queried and evaluated.
- An unsupported sampler field fails config load.
- The published schema and shipped `examples/config.toml` are each bound to the
  real `Config` by a check that goes red on a controlled mismatch.
- The live waiting reproduction is no longer RUNNING under a generic sampler
  config.
- `just ci` exits 0, and the pre-merge runnability gate passes on the pushed
  head.

## Non-goals

- Census, default recipes, skill procedure (#17).
- Nix/CI privacy enforcement (#18).
- Any change to the sampler shape `name + field + regex -> status`, a shared
  epic contract consumed by #17.
- Any generalization toward a shared `field + regex -> typed output` evaluator
  with reinterpreters. Consolidation is commissioned separately if duplication
  actually recurs.
