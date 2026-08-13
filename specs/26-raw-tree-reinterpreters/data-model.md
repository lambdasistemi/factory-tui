# Data model

Artifact ceiling: 3,000 bytes / 85 lines.

| ID | Data | Fields / relationship | Invariant |
|---|---|---|---|
| DATA-26-REINTERPRETER | `Reinterpreter` | `scope: Scope`, `pattern: String`, `label: String`; entries retain configuration order and validate their regex. | INV-26-C8, INV-26-RAW-NAME |
| DATA-26-SCOPE | `Scope` | Closed values `session`, `window`, `pane`; invalid values fail configuration parsing. | INV-26-C8, INV-26-CONFIG |
| DATA-26-NODE | raw tree `Node` | Stable raw identity, `Kind`, raw title, unchanged window status, optional window target, optional pane target, ordered children. Display labels are not identity fields. | INV-26-C8, INV-26-PANES |
| DATA-26-KIND | `Kind` | Root/session grouping, `Window`, and `Pane`; projected `Folder` is removed. | INV-26-RAW, INV-26-DELETION |
| DATA-26-TARGET | jump target | A single-pane Window targets its pane; a multi-pane Window targets its current/default pane and each Pane child targets exactly its own pane. | INV-26-PANES |
| DATA-26-DISPLAY | display label | One sanitized line. Unmatched equals raw. Matched output contains both replacement and sanitized raw source; blank replacement falls back to raw. | INV-26-C8, INV-26-RAW-NAME |
| DATA-26-SESSIONS | `SessionsConfig` | `infra: Vec<String>` only; annotation is computed from raw session name after grouping. `alias` and reserved `machine` are absent. | INV-26-C8, INV-26-DELETION |
| DATA-26-DUMP-ID | dump identity marker | Each Window row exposes exactly one stable window marker. Every pane exposes exactly one pane marker, on its single-pane Window or its multi-pane Pane child. | INV-26-REACH |

Relationship cardinality is fixed: root 1→N sessions; session 1→N windows; a one-pane window 1→0 Pane children; a window with N>1 panes 1→N Pane children. Empty-pane census rows do not exist because tmux reports one row per pane.

Status data and `Pane`/`Win` acquisition fields are unchanged.
