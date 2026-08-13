# Functions

## `src/config.rs`

- `load() -> Config`
  - args: none (reads env and default path)
  - result: empty `Config` if no file
- `load_from_path(path: &Path) -> Result<Config, Error>`
  - args: `path`
- `load_from_str(text: &str) -> Result<Config, Error>`
  - args: `text` (tests)

## `src/tree.rs`

- `build(wins: Vec<Win>, config: &Config) -> Node`
  - args: `wins`, `config`
  - result: raw or projected tree
- `dump(node: &Node) -> String`
  - unchanged purpose; used by `--dump` and goldens

## `src/config.rs` (from #6)

- `classify(win: &Win, config: &Config) -> Classified`
  - args: `win`, `config`
  - first matching rule wins; no match ⇒ no extra fields
