# Data model

Artifact ceiling: 2,000 bytes / 60 lines.

| ID | Data | Change | Invariant |
|---|---|---|---|
| DATA-22-PANE-TITLE | `Pane` | Add the pane title exactly as received from tmux as a `String`. It remains untrusted until display-label composition. | INV-22-PARSE |
| DATA-22-LABEL | Rendered pane label | Non-empty sanitized title, otherwise `{index}:{command}`; terminal-cell width does not exceed the caller's budget. | INV-22-FALLBACK, INV-22-ESCAPE, INV-22-WIDTH, INV-22-CONSISTENT |

No relationship or classification field changes.

