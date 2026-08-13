# Data

Follows parent `data-model.md`, narrowed to the fields #5 must
serialize / deserialize / apply. All items below carry the parent's
"unknown top-level keys ignored" rule.

## `Config`

Top-level TOML tables owned by #5:

- `sessions.alias`: map `string → string`. Rewrites a session title
  in the tree output.
- `sessions.infra`: list of session-name patterns (literal or
  simple-glob; commit owner picks the shape and documents it in the
  slice). Matching sessions are tagged as infra.
- `sessions.machine`: optional list of session names (transport
  buckets; treated as normal sessions for the default tree; carried
  now so #6 can consume it without a schema bump).
- `status.running`: list of process command names classified as
  running.
- `status.idle`: list of process command names classified as idle.
- `status.parked_substring`: `string`, default empty
  (`""` ⇒ no parked detection).

Unknown top-level tables (for example `[[rule]]`, `[tree]` from #6)
are accepted and ignored. Missing tables deserialize to their
type-default (empty map / empty list / empty string).

## `Classified` window (aliased shape for #5)

A census `Win` augmented with:

- `session_alias`: `Option<String>` — the value of
  `sessions.alias[session]` when present.
- `is_infra`: `bool` — true when `sessions.infra` matches this
  session.
- `status`: enum { `Running`, `Idle`, `Parked`, `Unknown` } derived
  from `status.*`.

Additional fields listed by the parent (`role`, `project`,
`milestone`, `epic`, `ticket`, `goal`) are not populated in this
slice — they are #6's job.

## `Node`

Unchanged outward shape from the parent model (id, kind, title,
status, win, children). Kinds used by the raw tree in this slice:

- `SessionGroup`  — one per tmux session, ordered by census order.
- `Window`        — one per window under its session.

No folder kinds are introduced in this slice.

## Config load precedence (state invariant)

1. `$FACTORY_TUI_CONFIG` set and non-empty ⇒ load that path.
2. Else `~/.config/factory-tui/config.toml` (respecting XDG when
   `$XDG_CONFIG_HOME` is set).
3. Else `Config::empty()`.

A path resolved in step 1 or 2 that does not exist ⇒ `Config::empty()`
without error. A path that exists but fails to parse ⇒ error to
stderr and non-zero exit for the CLI entry points; `load_from_path`
returns `Err`.
