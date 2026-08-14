# Config schema

Path: `$FACTORY_TUI_CONFIG`, else
`~/.config/factory-tui/config.toml`. Missing or empty file = empty
config = the raw tmux tree.

Unknown top-level tables are ignored.
The removed `[status]` table is rejected so an old configuration cannot
silently lose all status marking during an upgrade.

Nothing in this file can change the shape of the tree. The tree is
session → window → pane, built from the tmux census before any of this
is read.

## `[sessions]`

- `infra`: list of whole-name globs (`*`, `?`). Matching sessions are
  tagged `[infra]`.

## `[[sampler]]` (ordered, first match wins per pane)

Each entry needs all four fields:

- `name`: non-blank label used in validation diagnostics. Names must be
  unique.
- `field`: one of `pane_current_command`, `pane_current_path`,
  `pane_title`, or `window_name`.
- `regex`: expression matched against the observed field value. It must
  compile.
- `status`: `running`, `idle`, or `parked`.

```toml
[[sampler]]
name = "busy-title"
field = "pane_title"
regex = "^[\\x{2800}-\\x{28ff}]"
status = "running"
```

Each pane is sampled independently. The first matching entry supplies its
status. If nothing matches, the pane is unmarked; an entirely unmarked window
also stays unmarked rather than being reported idle. Established child status
rolls up as PARKED over RUNNING over idle.

No sampler is configured by default. In particular, merely occupying a pane
with a command does not imply that command is working.

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
