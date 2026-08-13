# Data

## Config

- `sessions.alias`: map string → string
- `sessions.infra`: list of glob/string patterns
- `sessions.machine`: optional list of session names
- `status.running` / `status.idle`: process command names
- `status.parked_substring`: string, default empty (no parked
  detection)
- `rule[]`: `window` regex, optional `session` regex, `role`,
  other fields via named captures
- `tree.folders`: ordered field names
- `tree.desk_roles`: roles whose window is the folder jump target
- `tree.inherit_milestone_from_desk`: bool

Unknown keys are ignored so #5 files still load after #6.

## Classified window

A census `Win` plus optional fields: `role`, `project`,
`milestone`, `epic`, `ticket`, `goal`, `parked`.

## Node

Unchanged outward shape (id, kind, title, status, win, children).
Kinds used by the raw tree: session group, window. Projected tree
may add folder kinds named by the fold path.
