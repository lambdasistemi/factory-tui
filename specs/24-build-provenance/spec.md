# Ticket 24 — Truthful version and build provenance

Artifact ceiling: 3,200 bytes / 100 lines.

## Paramount user story

As an operator running release and branch builds side by side, I ask factory-tui what it is and observe the version and exact source provenance of the build on screen.

## Requirements

- **R-24-01 — One version authority.** `Cargo.toml` remains the only maintained product-version value. Nix package metadata and release artifact versions derive from it.
- **R-24-02 — Non-interactive identity.** `factory-tui --version` exits successfully outside tmux and reports the Cargo version plus build revision.
- **R-24-03 — On-screen identity.** The popup visibly reports the same identity as `--version` without changing preview or status behavior.
- **R-24-04 — Commit distinction.** Clean Nix builds from distinct revisions at the same Cargo version report distinct revisions at runtime.
- **R-24-05 — Historical release truth.** The corrected derivation gives the `v0.1.0` source version `0.1.0`.
- **R-24-06 — Permanent guard.** CI contains an executing check across the manifest, Nix metadata, and binary. A deliberate version divergence makes it fail.
- **R-24-07 — Honest fallback.** A build without exact revision metadata reports that provenance as unavailable or dirty; it never invents a commit.

## Invariants

| ID | Severity | Observable truth | Failure signal | Success signal |
|---|---|---|---|---|
| INV-24-VERSION | ADVISORY | Nix package and artifact versions equal `Cargo.toml`. | Version check exits non-zero. | Check reports the shared version and exits 0. |
| INV-24-GUARD | ADVISORY | The version check executes and detects a deliberately pinned mismatch. | Negative control unexpectedly exits 0 or is not selected. | Baseline is green; pinned mismatch is red for the named comparison. |
| INV-24-CLI | ADVISORY | `--version` needs neither tmux nor UI and reports version plus provenance. | Non-zero exit, tmux access, or missing identity field. | Exact executable output passes its assertion. |
| INV-24-UI | ADVISORY | The popup visibly uses the same build identity as the CLI. | Rendering proof cannot find the shared identity. | Rendering proof observes both identity fields. |
| INV-24-REV | ADVISORY | Distinct clean source revisions remain distinguishable at runtime. | Two revision builds report the same provenance. | Runtime outputs differ by their exact revisions. |
| INV-24-PURE | ADVISORY | Provenance uses flake source metadata without time, network, or build-time Git. | Build needs ambient state beyond the flake source. | Rebuilding one revision preserves identity. |
| INV-24-TAG | ADVISORY | The corrected build path applied to `v0.1.0` names version `0.1.0`. | Result retains `0.0.1` or another value. | Package/runtime evidence reports `0.1.0`. |

## Non-goals

- Release-please configuration, tagging, publishing, or release execution.
- Preview/status restructuring, `src/tmux.rs`, or M1 behavior.
- The `milestones` branch, `.milestones/**`, or `.projects/**`.
