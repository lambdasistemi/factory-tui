# Functions model

Artifact ceiling: 2,000 bytes / 60 lines.

| ID | Function | Signature contract | Constraint |
|---|---|---|---|
| FN-22-QUERY | `query_all` | `() -> io::Result<Vec<Win>>` | Existing signature and effects remain; returned panes additionally carry title. |
| FN-22-LABEL | `pane_label` | `(pane: &Pane, max_width: u16) -> String` | Returns safe non-blank identity within `max_width` terminal cells when a visible cell is available. |

Private parser helpers may be extracted for proofability without changing the public surface.

