# Implementation plan

Artifact ceiling: 3,500 bytes / 100 lines.

## Strategy

Extend the existing tab-separated tmux census by one terminal pane field, keeping the field-count guard and pane slice aligned. Carry the title as untrusted data on `Pane`. Centralize label composition so every label site sanitizes, falls back, and truncates by terminal-cell width under one contract.

The chosen composition is **sanitized title first; `{index}:{command}` only as fallback**. In narrow boxes the title wins because it is the operator-authored seat identity and distinguishes otherwise identical commands. Over-long text is visibly truncated within the caller's cell budget.

## Boundaries

- `src/tmux.rs` owns acquisition and carriage of the raw tmux title.
- `src/ui.rs` owns safe display-label composition and all current pane-label call sites.
- Existing ANSI stripping may be reused or promoted, but no new dependency is authorized.
- Parser and rendering proof may live beside the owning modules.
- No status, tree, projection, preview-layout, M1, milestone, or release edits.

## Slice

One bisect-safe OWNER slice implements REQ-22-QUERY through REQ-22-SCOPE and tasks T001-T005.

## Verification

- Focused parser and label tests named by the frozen gate.
- RED proof against base `8a273defe92fafc93498189c80b230653ea235b9`.
- Frozen slice gate plus `just ci`.
- Captured ratatui test-backend frame and a live multi-pane tmux title fixture created only at runtime.

## Resource constraint

Before every Nix-realising command, record exact free bytes and visible collector state. Never retry a silent hang without a visible collector; report it as a disk event.

