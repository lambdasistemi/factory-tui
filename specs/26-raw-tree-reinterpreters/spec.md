# Raw tmux tree and label-only reinterpreters

Issue: #26

Artifact ceiling: 4,500 bytes / 120 lines.

## User stories

As a factory reader, I see tmux as it exists—session → window → pane—so no naming rule can hide or relocate a live seat.

As an operator, I can make cryptic tmux names readable without granting configuration any power over tree structure, and I can always recover the original name from the displayed row.

## Requirements

- **REQ-26-RAW**: Tree structure is exactly root → raw tmux session → raw tmux window → pane. No project, milestone, epic, role, rule-match, or fallback bucket is synthesized.
- **REQ-26-PANES**: A multi-pane window has one child node per pane. A single-pane window has no redundant pane child and targets its sole pane directly.
- **REQ-26-REINTERPRET**: Ordered `[[reinterpreter]]` entries use `scope`, `pattern`, and `label`; the first matching entry may replace one row's display label and nothing else. Unmatched input renders raw.
- **REQ-26-RAW-NAME**: A rewritten label visibly retains the sanitized original raw name. Invalid or visually empty output falls back to the raw label.
- **REQ-26-DELETE**: Projection schema, classifiers, folder accumulation/inheritance, `examples/projection.toml`, `[sessions].machine`, and `[sessions].alias` are removed. Session aliasing remains expressible as a session-scoped reinterpreter; `[sessions].infra` remains label-only.
- **REQ-26-CONFIG**: `examples/config.toml` is host-neutral, includes a minimal honest `[status]`, and parses through the real `Config` type.
- **REQ-26-DOCS**: `README.md`, `AGENTS.md`, `docs/`, and `skills/factory-tui/` describe only the raw topology and reinterpreter schema.
- **REQ-26-BOUNDARY**: `src/tmux.rs` query format/field slice, status classification, preview composition beyond pane-node selection, M1, milestone/project records, and release machinery do not change.

## Invariants

| ID | Severity | Observable success | Observable failure |
|---|---|---|---|
| INV-26-C8 | ADVISORY | For generated tmux states and reinterpreter configurations, the ordered `(node id, parent id, kind, jump target)` set is byte-identical with and without reinterpretation; only display strings differ. A deliberately splitting mutant is rejected by the same comparator. | A rule inserts, removes, merges, splits, reorders, or reparents a node, or changes its jump target. |
| INV-26-D14-D15 | ADVISORY | The six reproduced window names appear exactly once beneath one `factory-tui` session, with no duplicate sibling title. | Any reproduced window is missing/duplicated, or projection and fallback create identically titled siblings. |
| INV-26-PANES | ADVISORY | Each direct tmux pane is represented exactly once: at its one-pane window or as a child of its multi-pane window; selecting it previews and jumps to that pane. | A pane is hidden, duplicated, attached to the wrong parent, or selection targets a different pane. |
| INV-26-RAW-NAME | ADVISORY | Unmatched rows render raw; matched rows render one sanitized label containing the sanitized raw source name. | A rewrite conceals its source, renders blank/control text, or visually creates extra rows. |
| INV-26-CONFIG | ADVISORY | The shipped example parses through `Config`; an intentionally broken copy makes the same check fail. | The example drifts from the executable schema or the check accepts its seeded break. |
| INV-26-DELETION | ADVISORY | Executing checks prove the projection types/functions/example and all published projection guidance are absent while positive controls show the searches run. | Folding remains callable, legacy schema is published, or the absence check is vacuous. |
| INV-26-REACH | ADVISORY | A stable direct `tmux list-panes -a` census and `factory-tui --dump` report identical unique window and pane counts, including all 43 reproduced windows when that census remains current. | The dump count derives from itself, differs from tmux, or a live pane is unreachable. |

## Acceptance

The complete RED bundle fails on the D14/D15 reproduction before production changes. Permanent checks include the C8 split-mutant negative control and the broken-example negative control. Focused checks, the frozen gate, `just ci`, and the stable live tmux reachability comparison all pass without touching forbidden scope.
