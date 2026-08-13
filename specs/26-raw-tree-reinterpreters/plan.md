# Implementation plan

Artifact ceiling: 4,000 bytes / 110 lines.

## Strategy

Build raw identity and parentage without any reinterpreter input. Configuration supplies display strings only after the tree exists. This dependency direction makes structural interpolation unavailable to the renaming mechanism and gives INV-26-C8 one reusable structural fingerprint.

Promote pane display-label composition from the preview-only owner to a shared label owner because both the tree and preview consume the same untrusted pane title. The promotion must preserve #22's sanitation, visible fallback, and cell-width behavior without changing tmux acquisition or preview layout.

Use `[[reinterpreter]]` as the sole renaming schema. Entries are ordered and contain `scope`, `pattern`, and `label`. Regex replacement is generic configuration data; the crate contains no operator naming grammar. A changed display string includes the raw source label, so collisions remain distinguishable and the interpretation is auditable.

Keep `[sessions].infra` as a display annotation. Replace `[sessions].alias` with exact session-scoped reinterpreters, avoiding two independent renaming mechanisms and preventing aliases from merging raw session buckets.

## Boundaries

- `src/tree.rs` owns only raw tmux topology, node identity, jump targets, status roll-up, structural fingerprinting used by proof, and dump identity markers.
- `src/config.rs` owns deserialization and validation of `[[reinterpreter]]`, `[sessions].infra`, and the unchanged status table.
- A shared label module owns safe raw/reinterpreted row labels and the pane identity label used by both tree and preview.
- `src/app.rs` projects labels into rows after topology construction and binds pane-node selection to preview/jump.
- `src/ui.rs` consumes shared labels; it does not gain structural responsibility.
- Documentation, the shipped example, and published skill change in the same candidate.
- No edit is authorized in `src/tmux.rs`, `status_of`, `docs/m1/`, `.milestones/`, `.projects/`, `nix/`, release files, or workflows.

## Slice

One bisect-safe OWNER slice, **S-26**, delivers REQ-26-RAW through REQ-26-BOUNDARY and tasks T001–T008. Structure, configuration, example, and published schema are inseparable because landing any subset would publish a false contract.

## Verification

- Focused RED/GREEN tests named by the immutable gate.
- INV-26-C8 structural equality across generated states/configurations plus a splitting-mutant negative control.
- The exact six-window D14/D15 reproduction.
- Example parse plus a mechanically captured broken-copy failure.
- Removed-surface search with positive controls.
- `just ci` under the host disk-floor rule.
- One stable before/after direct tmux census compared with machine-readable identity markers in `--dump`.

## Resource and audit budget

OWNER mode, no draft tool. Commit owner: Claude, pinned `claude-opus-5[1m]`, high effort. Fresh auditor: Codex, pinned model/effort from its launch command. Default campaign budget: three building audits; all invariants are ADVISORY. Immediately before each Nix-realising command, require at least 3.5 GiB above `min-free` and journal exact bytes on deferral/resume. A local missing/invalid store path or untouched broken recipe is a machine event and is never retried.
