# Data model

Mandate v3 (recut) · Base `main@ea047e04`

## Supported evidence fields

One declaration in `src/config.rs`, bound to by the validator, the resolver, and
the schema check, and reconciled against the tmux query by the gate.

| Field name | Source | Scope | Queried on `ea047e04`? |
|---|---|---|---|
| `pane_current_command` | `#{pane_current_command}` | pane | yes |
| `pane_current_path` | `#{pane_current_path}` | pane | yes |
| `pane_title` | `#{pane_title}` | pane | yes (#22) |
| `window_name` | `#{window_name}` | window | yes |

All four are already queried, so the full vocabulary ships in one slice.
`pane_title` is the field that makes C1 satisfiable.

The supported set must never name a field the crate does not query. Such a field
would load without error and then never match — the silent non-match R3 forbids.

## `Sampler` — one `[[sampler]]` table

| Field | Type | Required | Validation |
|---|---|---|---|
| `name` | `String` | yes | non-empty; unique; operator-facing label only, never matched against |
| `field` | `String` | yes | must be in the supported set |
| `regex` | `String` | yes | must compile |
| `status` | `String` | yes | one of `running`, `idle`, `parked` |

Ordered; first match wins per pane.

## `Config`

- **Removed:** `status: StatusConfig` and the `StatusConfig` type
  (`running`, `idle`, `parked_substring`).
- **Added:** `sampler: Vec<Sampler>`, ordered `[[sampler]]` tables.
- **Unchanged:** `sessions`, `reinterpreter`.

A configuration still carrying any removed `[status]` key is rejected at load.

## `Status`

Unchanged: `Running`, `Idle`, `Parked`, `Unknown`. `Unknown` remains the default
for no match and renders as the empty label.

`Unknown` and `Idle` are distinct and must stay distinct. `Idle` is a positive
reading; `Unknown` is the absence of one.

## Evaluation invariants

- A pane's status is the status of the **first** sampler matching it; no match
  yields `Unknown`.
- An `Unknown` pane contributes nothing to rollup.
- A window/ancestor status is the rank-worst **established** child status,
  preserving `Parked` > `Running` > `Idle` > `Unknown`.
- A window whose panes are all `Unknown` is `Unknown` — never `Idle`.
- With no samplers configured, everything is `Unknown`. The crate ships no
  default samplers: a default marking anything RUNNING would be the crate
  asserting agent knowledge it must not have.

## Not modified

`tmux::Pane` (already carries `title`), `tmux::Win`, `Reinterpreter`, `Scope`,
`BuildIdentity`.
