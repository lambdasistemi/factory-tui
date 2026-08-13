# Research: tmux browser and optional projection

## Current split

- `src/tmux.rs` — host-agnostic census, snapshot, jump.
- `src/parse.rs` — host tables + a hand-written name grammar.
- `src/tree.rs` — fold policy (machine group, infra, inherit
  milestone from a desk in the same session).

Only the first file belongs in the crate as product logic.

## Alternatives

1. **Keep grammar in Rust, move tables only.** Removes the nickname
   leak. Leaves the factory parser as the default. Rejected as the
   end state; allowed as the #5 → #6 gap.
2. **Lua/Rhai in the config.** Too much language. Rejected.
3. **Ordered regex + named captures + folder path.** Covers every
   current `parse_*` branch. Accepted.
4. **Hard-code a built-in “factory” projection.** Would re-smuggle
   convention as product. Rejected. Example file is opt-in.

## Unmatched windows

`unscoped` is itself a projection. Spec R4: keep them under the
tmux session.

## Config location

`$FACTORY_TUI_CONFIG` then `~/.config/factory-tui/config.toml`.
Missing file = empty config, not an error. Do not read the
working directory (a clone must not surprise-load a repo file).
The generic example is loaded only by tests, or by the operator
copying it.

## Tests

Do not talk to a live tmux in unit tests. Build `Win` values in
Rust and assert `tree::dump`. Goldens: no config, tables only,
full example.
