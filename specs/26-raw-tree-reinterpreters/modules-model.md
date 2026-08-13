# Modules model

Artifact ceiling: 2,500 bytes / 70 lines.

| ID | Module | Changed responsibility | Dependency direction |
|---|---|---|---|
| MOD-26-CONFIG | `src/config.rs` | Own and validate the generic reinterpreter schema; retain infra/status configuration; remove projection and duplicate alias schema. | Supplies label data to MOD-26-LABEL and unchanged status data to MOD-26-TREE. Never receives or mutates tree nodes. |
| MOD-26-LABEL | shared label module | Own safe pane identity composition and one-node-to-one-string reinterpretation, including raw-name reachability. | Consumes raw node/pane labels plus MOD-26-CONFIG; supplies strings to MOD-26-APP and MOD-26-UI. Has no structural output. |
| MOD-26-TREE | `src/tree.rs` | Build only raw session/window/pane topology and stable jump targets; remove projection/folder code; expose proofable structural identity and dump markers. | Consumes tmux census and unchanged status inputs. Does not depend on reinterpreters. |
| MOD-26-APP | `src/app.rs` | Flatten raw nodes into display rows using MOD-26-LABEL and bind pane-node selection to the existing preview/jump path. | Consumes MOD-26-TREE structure and MOD-26-LABEL strings; never feeds labels back into identity or parentage. |
| MOD-26-UI | `src/ui.rs` | Reuse MOD-26-LABEL for existing pane label sites and render the new Pane kind. | Consumes app rows and shared labels; no tree construction. |
| MOD-26-PUBLISHED | `README.md`, `AGENTS.md`, `docs/`, `skills/factory-tui/`, `examples/config.toml` | Publish the executable raw-tree/reinterpreter contract and no deleted projection schema. | Mirrors MOD-26-CONFIG and MOD-26-TREE; tested against the real parser and removed-surface scan. |

`src/tmux.rs` remains the upstream source of pane titles from #22 and is read-only for this ticket.
