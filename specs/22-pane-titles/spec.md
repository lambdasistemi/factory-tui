# Pane titles for seat identity

Issue: #22

Artifact ceiling: 3,500 bytes / 100 lines.

## User story

As a factory reader, I can distinguish panes that run the same command because every pane label uses the pane's real tmux title.

## Requirements

- **REQ-22-QUERY**: The live tmux census obtains `#{pane_title}` and preserves every pre-existing window and pane field.
- **REQ-22-LABEL**: Schematic boxes, preview metadata, and the preview block use one consistent pane identity label.
- **REQ-22-FALLBACK**: An empty sanitized title falls back to the existing non-blank `{index}:{command}` identity.
- **REQ-22-SAFETY**: Escape sequences and control characters in a title cannot style, erase, split, or otherwise corrupt the rendered frame.
- **REQ-22-WIDTH**: A label fits the cell width offered by its rendering site; a narrow box prioritizes the human tmux title and truncates it legibly.
- **REQ-22-SCOPE**: Status classification, tree shape, projection, pane selection, and preview composition remain unchanged.

## Invariants

| ID | Severity | Observable success | Observable failure |
|---|---|---|---|
| INV-22-PARSE | ADVISORY | An 18-field row populates every window/pane field, including title; a 17-field row is rejected without panic or shifted values. | A pane is dropped, a field shifts, or a short row reaches an out-of-range slice. |
| INV-22-FALLBACK | ADVISORY | Empty or control-only title renders the non-blank index/command fallback. | Any label site is blank. |
| INV-22-ESCAPE | ADVISORY | CSI/OSC and other controls are absent from label text and cannot affect adjacent frame cells. | Control bytes or injected styling reach a label widget. |
| INV-22-WIDTH | ADVISORY | Every returned label occupies no more terminal cells than requested and narrow labels remain identifiable. | A label exceeds its cell budget or splits the frame. |
| INV-22-CONSISTENT | ADVISORY | Schematic and preview label sites derive identity from the same composition rule. | One site reintroduces `{index}:{command}` while another uses title. |
| INV-22-SCOPE | ADVISORY | Existing status/tree/projection tests remain unchanged and green. | This labels-only change alters those behaviors. |

## Acceptance

The focused regression proof is shown red on the pre-change base and green on the candidate; full CI passes; and a real multi-pane window produces distinguishable sanitized title labels in a captured TUI frame.

