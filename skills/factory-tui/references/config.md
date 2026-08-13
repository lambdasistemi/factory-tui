# Config schema

Path: `$FACTORY_TUI_CONFIG`, else
`~/.config/factory-tui/config.toml`. Missing or empty file = empty
config = the raw tmux tree.

Unknown top-level tables are ignored.

Nothing in this file can change the shape of the tree. The tree is
session → window → pane, built from the tmux census before any of this
is read.

## `[sessions]`

- `infra`: list of whole-name globs (`*`, `?`). Matching sessions are
  tagged `[infra]`.

## `[status]`

- `running`: pane command names (exact) → RUNNING
- `idle`: pane command names → idle
- `parked_substring`: if this string appears in the **window name**
  or a pane command → PARKED

## `[[reinterpreter]]` (ordered, first match wins)

Each entry needs all three fields:

- `scope` (required): `session`, `window`, or `pane`. Any other value
  is rejected at load.
- `pattern` (required): regex matched against the raw tmux name. Must
  compile.
- `label` (required): replacement for the span the pattern matched.
  `$name` refers to a named capture. Must not be blank.

```toml
[[reinterpreter]]
scope = "session"
pattern = "^ops-(?P<rest>.+)$"
label = "operations $rest"
```

Behaviour:

- Only entries whose `scope` matches the row are considered, in file
  order; the first whose pattern matches wins and the rest are skipped.
- The row displays the replacement with its raw tmux name kept beside
  it, so the original is always readable and two rows that rewrite to
  the same text stay distinct.
- A row nothing matches renders raw.
- A replacement that would display nothing — an empty expansion, a
  capture that does not exist — falls back to the raw name.
- Pane rows are reinterpreted from the pane's own identity: its tmux
  title when it has one, otherwise `{index}:{command}`.

To show a session under a different name, use a `session`-scoped entry.
There is no alias table and no separate renaming mechanism.

## Verifying

`factory-tui --dump` carries one `[window=@id]` marker per window row
and one `[pane=%id]` marker per pane — on its single-pane window, or on
its own row. Step 4 of the skill compares those counts against tmux.

Those markers, the status field (`[RUNNING]`, `[PARKED]`, `[idle]`) and
`[infra]` are factory-tui's to write. A name or label containing one is
shown with its bracket turned into a parenthesis, so no row can claim an
id, a liveness or a role it does not have; a name that merely contains
the word `idle` is left exactly as tmux reports it.
