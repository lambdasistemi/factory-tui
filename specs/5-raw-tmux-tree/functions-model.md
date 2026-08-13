# Functions

Only new / changed signatures listed. Argument and result names are
authoritative; bodies, helpers, and internal calls are the commit
owner's decision.

## `src/config.rs`

- `Config::empty() -> Config`
  - args: none
  - result: `Config` with all owned tables at type-default.
- `load() -> Config`
  - args: none
  - reads env then XDG default; missing file ⇒ `Config::empty()`;
    unreadable / invalid file ⇒ diagnostic + `Config::empty()` (see
    `load_from_path` for the error surface).
- `load_from_path(path: &Path) -> Result<Config, ConfigError>`
  - args: `path`
  - result: parsed config; `Err` on I/O failure or TOML parse error.
- `load_from_str(text: &str) -> Result<Config, ConfigError>`
  - args: `text`
  - result: parsed config (used by unit tests and goldens).

The `ConfigError` type is owned by `src/config.rs`. Its variants
carry the offending path (when known), the source TOML span, and a
short human message.

## `src/tree.rs`

- `build(wins: Vec<Win>, config: &Config) -> Node`
  - args: `wins` (census), `config` (may be `Config::empty()`)
  - result: a root `Node` whose children are `SessionGroup` nodes.
    With an empty config the tree is exactly session → window (no
    projection).

- `dump(node: &Node) -> String`
  - args: `node`
  - result: human-readable text tree used by `--dump` and by
    goldens. Format is stable within this slice; commit owner may
    choose the indent character but must document it in
    goldens.

## `src/main.rs`

Behavior addition, no exported new function names required:

- `--dump` and the default TUI startup both call `config::load()`
  once, then `tree::build(wins, &config)`. On `load()` error the
  binary prints the diagnostic to stderr and exits non-zero rather
  than silently falling back to an empty config.

No changes to preview / jump / popup / mouse function names or
signatures in this slice.
