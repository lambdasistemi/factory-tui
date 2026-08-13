# Tasks: M3 preview tag

Artifact ceiling: 80 lines / 4 KiB.

## SL-001 — repository capability

- [x] **T019-01** Add FN-001 with exact DATA-001 validation and diagnostics.
- [x] **T019-02** Add permanent both-way automated coverage and wire it into the repository CI gate.
- [x] **T019-03** Add M3 preview user/operator documentation satisfying RQ-005 and RQ-006.
- [x] **T019-04** Prove forbidden production release files remain byte-identical to the slice base.

## SL-002 — audit wording correction

- [x] **T019-04A** Align the Nix check comment with the independently audited mutation coverage, without changing behavior.

## OP-001 — publication evidence

- [ ] **T019-05** After Q-002 authorization and merge, push `milestone-3-preview.1` at the accepted commit.
- [ ] **T019-06** Verify zero Actions runs, isolated stranger install/binary execution, no milestone release object, and unchanged Latest.
