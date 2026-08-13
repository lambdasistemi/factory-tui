# Plan — Truthful version and build provenance

Artifact ceiling: 2,600 bytes / 90 lines.

## Strategy

Deliver one bisect-safe OWNER slice. The manifest version is promoted through the existing Nix build owner; clean flake revision metadata is injected at build time; one Rust identity component serves both command and popup consumers; a flake check reconciles the manifest, package metadata, and executable output.

## Purity decision

Use the flake source revision already bound by Nix evaluation. This does not invoke Git, read wall-clock time, use the network, or make a clean revision build impure. Non-flake or dirty builds must use an explicit honest fallback. The tradeoff is that an exact commit is guaranteed for clean flake inputs, while ad-hoc Cargo builds cannot claim one unless their caller supplies trustworthy metadata.

## Boundaries

- Manifest authority: `Cargo.toml`.
- Nix build identity: `flake.nix`, `nix/crane.nix`, `nix/checks.nix`, and minimal adjacent wiring only.
- Runtime identity: a dedicated Rust build-identity module consumed by `src/main.rs` and `src/ui.rs`.
- User record: `README.md` and/or `docs/using.md`.
- Proof: Rust tests, executing Nix check, frozen ticket gate, and out-of-tree historical/two-revision evidence.

## Ordered slice

**S-24-01 — Build identity end to end**

1. Establish RED for missing/mismatched identity surfaces.
2. Derive package version and inject pure revision metadata.
3. Expose one runtime identity through CLI and popup.
4. Prove the permanent mismatch guard can fail.
5. Produce two-revision and `v0.1.0` evidence without moving a tag.
6. Document the command and provenance limits.

## Verification layers

- Focused Rust proof for identity formatting and argument behavior.
- Nix version-consistency check for manifest/package/runtime reconciliation.
- Complete flake gate for build, tests, lint, formatting, deny, and docs.
- Clean-revision executable probes for commit distinction.
- Synthetic, detached `v0.1.0` source probe using only the corrected build-path delta.

## Constraints

- No release-please ownership change and no second maintained version literal.
- No changes to preview/status classification or `src/tmux.rs`.
- Re-measure exact disk headroom immediately before every realizing Nix command; journal deferral and resumption.
- No release act.

