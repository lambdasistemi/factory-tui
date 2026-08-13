# Functions Model: Flat land browse camera

## `src/parse.rs`

### `parse_window`

- **Requirement / slice**: FR-001
- **Signature**: `pub fn parse_window(name: &str, session: &str) -> Parsed`
- **Arguments**:
  - `name`: `&str`
  - `session`: `&str`
- **Result**: `Parsed`
- **Signature constraints / effects**: pure

## `src/tree.rs`

### `build`

- **Requirement / slice**: FR-001
- **Signature**: `pub fn build(wins: Vec<Win>) -> Node`
- **Arguments**:
  - `wins`: `Vec<Win>`
- **Result**: `Node`
- **Signature constraints / effects**: pure grouping

## `src/tmux.rs`

### `query_all`

- **Requirement / slice**: FR-001
- **Signature**: `pub fn query_all() -> io::Result<Vec<Win>>`
- **Arguments**: none
- **Result**: `io::Result<Vec<Win>>`
- **Signature constraints / effects**: reads tmux; no resize

### `capture_pane`

- **Requirement / slice**: FR-002, FR-003
- **Signature**: `pub fn capture_pane(id: &str) -> io::Result<String>`
- **Arguments**:
  - `id`: `&str`
- **Result**: `io::Result<String>`
- **Signature constraints / effects**: `capture-pane` only; must not
  resize the target

### `focus`

- **Requirement / slice**: FR-004
- **Signature**: `pub fn focus(session: &str, window_id: &str, pane: Option<&str>) -> io::Result<()>`
- **Arguments**:
  - `session`: `&str`
  - `window_id`: `&str`
  - `pane`: `Option<&str>`
- **Result**: `io::Result<()>`
- **Signature constraints / effects**: switch-client + select-window;
  no `resize-window`

## `src/ansi.rs`

### `to_text`

- **Requirement / slice**: FR-002
- **Signature**: `pub fn to_text(input: &str) -> Text<'static>`
- **Arguments**:
  - `input`: `&str`
- **Result**: `Text<'static>`
- **Signature constraints / effects**: pure SGR parse
