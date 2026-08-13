# Config schema

Path: `$FACTORY_TUI_CONFIG`, else
`~/.config/factory-tui/config.toml`. Missing file = empty config.

Unknown top-level tables are ignored.

## `[sessions]`

- `alias`: map from tmux session name to displayed project/session
  name. Example: `shop = "acme"`.
- `infra`: list of whole-name globs (`*`, `?`). Matching sessions are
  tagged `[infra]` on the raw tree.
- `machine`: reserved list of session names.

## `[status]`

- `running`: pane command names (exact) → RUNNING
- `idle`: pane command names → idle
- `parked_substring`: if this string appears in the **window name**
  or a pane command → PARKED

## `[[rule]]` (ordered, first match wins)

- `window` (required): regex against the window name. Named captures
  become fields: `project`, `milestone`, `epic`, `ticket`, `goal`,
  `role`.
- `session` (optional): regex the session name must also match.
- `role` (optional): assigned if the capture does not set `role`.

If `project` is missing after a match, the session alias or the raw
session name is used. If `milestone` is missing, `-ms<digits>` in the
session name is used.

A window that matches no rule stays under its tmux session.

## `[tree]`

- `folders`: ordered field names to nest (`project`, `milestone`,
  `epic`, `ticket`, `role`, `goal`)
- `desk_roles`: roles whose window is the folder's jump target
- `inherit_milestone_from_desk`: if true, a desk's milestone is
  copied to other classified windows in the same session

Projection runs only when `rule` is non-empty **and** `folders` is
non-empty.
