# Modules model

Artifact ceiling: 2,000 bytes / 60 lines.

| ID | Module | Changed responsibility | Dependency direction |
|---|---|---|---|
| MOD-22-TMUX | `src/tmux.rs` | Obtain and retain pane title alongside the complete existing census record. | Supplies untrusted `Pane` data to existing consumers. |
| MOD-22-UI | `src/ui.rs` | Produce one safe, width-bounded pane identity label and use it at every current pane-label site. | Consumes `Pane`; may reuse the crate's ANSI/control stripping owner. |

No module is authorized to change status classification, tree construction, projection, pane selection, or preview layout.

