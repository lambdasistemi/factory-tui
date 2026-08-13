# Functions model

Artifact ceiling: 2,500 bytes / 75 lines.

| ID | Function | Signature contract | Constraint |
|---|---|---|---|
| FN-26-BUILD | `build` | `(wins: Vec<Win>, status: &StatusConfig) -> Node` | Returns only raw tmux topology; accepts no reinterpreter input. |
| FN-26-REINTERPRET | `reinterpret` | `(scope: Scope, raw_name: &str, config: &Config) -> String` | Returns one safe display string; cannot receive or return structural data. |
| FN-26-NODE-LABEL | `node_label` | `(node: &Node, config: &Config) -> String` | Selects the raw/pane label then delegates one-string reinterpretation. |
| FN-26-PANE-LABEL | `pane_label` | `(pane: &Pane, max_width: u16) -> String` | Preserves #22's safe, visible, width-bounded label contract for tree and preview consumers. |
| FN-26-FINGERPRINT | `structural_fingerprint` | `(root: &Node) -> Vec<StructuralEntry>` | Encodes ordered node ID, parent ID, kind, and target while excluding all display strings. |
| FN-26-DUMP | `dump` | `(node: &Node, config: &Config) -> String` | Renders labels after construction and emits the stable Window/Pane identity markers required by live reachability evidence. |
| FN-26-SELECT | `selected_pane_id` | `(node: &Node) -> Option<&str>` | Returns a Pane node's exact pane target or a single-pane Window target without changing multi-pane preview fallback. |

Existing `status_of`, `query_all`, `capture_pane`, `focus`, and preview-layout signatures/effects remain unchanged.
