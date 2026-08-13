# Tasks — Ticket 24

Artifact ceiling: 1,200 bytes / 50 lines.

## S-24-01 — Build identity end to end

- [ ] **T24-01** Prove version/package/runtime mismatch detection is executing and can fail.
- [ ] **T24-02** Derive all Nix package and release-artifact version values from `Cargo.toml`.
- [ ] **T24-03** Carry pure clean-source revision metadata into the binary with an honest fallback.
- [ ] **T24-04** Expose canonical build identity through `factory-tui --version` without tmux.
- [ ] **T24-05** Display the same canonical build identity in popup chrome.
- [ ] **T24-06** Prove two distinct clean revisions are distinguishable at runtime.
- [ ] **T24-07** Prove the corrected build path gives `v0.1.0` version `0.1.0`.
- [ ] **T24-08** Document the command, fields, and provenance/purity limits.

Acceptance requires all tasks checked only after fresh independent audit of the exact candidate.

