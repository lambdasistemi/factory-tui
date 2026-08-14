# Functions model

Mandate v3 (recut). Only new and changed signatures. Argument names are contract.

## `src/config.rs`

```rust
pub const SUPPORTED_SAMPLER_FIELDS: &[&str]
```

The single supported-field declaration. Public because the schema check binds to
it and the gate reconciles it against the tmux query.

```rust
pub fn sampler_field_value<'a>(
    field: &str,
    win: &'a crate::tmux::Win,
    pane: &'a crate::tmux::Pane,
) -> Option<&'a str>
```

Resolve a declared field name to its observed value. Returns `None` for a field
outside the supported set. This is the single seam where a declared name meets
an observed value; every C2 proof binds here.

```rust
fn validate_samplers(config: &Config) -> Result<(), ConfigError>
```

Applies `Sampler` validation and the removed-`[status]`-key rejection. Called
from the existing load path alongside current validation.

Removed: `StatusConfig` and any helper reading `parked_substring`.

## `src/tree.rs`

```rust
fn status_of_pane(pane: &crate::tmux::Pane, win: &crate::tmux::Win, config: &Config) -> Status
```

New. Per-pane sampling: the first matching sampler's status, else
`Status::Unknown`. Replaces window-level occupancy as the primitive.

```rust
fn status_of(win: &Win, config: &Config) -> Status
```

Signature unchanged; semantics become the rollup of its panes' established
status rather than an "any pane matches" scan.

```rust
fn rollup(children: &[Node]) -> Status
fn worse(left: Status, right: Status) -> Status
pub fn status_label(status: Status) -> &'static str
```

Signatures unchanged. `rollup` must ignore `Unknown` children rather than
letting them lower an established status, and must yield `Unknown` when no child
is established.

## `tests/` — named contracts, not implementations

- shipped `examples/config.toml` parses through the real `Config` and yields its
  documented result, loading **that actual file** (C4);
- published schema reconciles with `SUPPORTED_SAMPLER_FIELDS` and real `Config`
  acceptance (C3);
- every supported field is present in the tmux query and resolvable through
  `sampler_field_value` (C2);
- per-pane sampling and rollup, including all-unmarked staying unmarked
  (C-rollup).

## `nix/crane.nix`

No signature change. Narrow source-filter extension only, per A-001.

## Not modified

`src/tmux.rs` `query_all` / `parse_pane`; `src/label.rs`; `src/build_info.rs`.

## Amendment v4 (ruling A-003 on Q-003) — `tree::build` boundary

```rust
pub fn build(wins: Vec<Win>, samplers: &[Sampler]) -> Node
```

Replaces `pub fn build(wins: Vec<Win>, status: &StatusConfig) -> Node`.

Deliberately narrowed to `&[Sampler]`, **not** `&Config`. #26 documents this
parameter as intentionally narrow (FN-26-BUILD): raw topology is built from the
census and the status table alone, and no reinterpreter reaches `build`.
Widening it to `&Config` would hand `build` the reinterpreter set it is
specifically designed not to receive, eroding a neighbouring ticket's invariant
to satisfy this one. The existing internal adapter keeps its shape, substituting
`sampler` for `status`.

Call sites, which change by substitution only:

| Site | Before | After |
|---|---|---|
| `src/main.rs:52` | `tree::build(wins, &config.status)` | `tree::build(wins, &config.sampler)` |
| `src/app.rs:61` | `tree::build(wins, &config.status)` | `tree::build(wins, &config.sampler)` |
| `src/app.rs:92` | `tree::build(tmux::query_all()?, &self.config.status)` | `tree::build(tmux::query_all()?, &self.config.sampler)` |
| `src/app.rs:455` | `&Config::empty().status` | `&Config::empty().sampler` |

No other change to `src/app.rs` or `src/main.rs` is authorized. No label,
rendering, event-loop, or refresh behavior may change.
