# M3 preview tag implementation plan

Artifact ceiling: 100 lines / 6 KiB.

## Strategy

The preview line uses a Git tag namespace that does not match the existing `v*` release-workflow trigger. The repository owns a small validation boundary and permanent both-way coverage for that namespace. User documentation owns the temporary/install/graduation contract. The production release path remains untouched.

## Live boundaries

- GitHub tag resolution by Nix is verified only against the pushed tag, from an isolated store and profile.
- GitHub Actions non-triggering and Latest safety are verified only after the authorized tag push through GitHub APIs.
- Q-002 is the authority boundary immediately before the first tag push.

## Ordered slices

### SL-001 — repository capability

Deliver RQ-001 through RQ-006 and invariants INV-NS, INV-PRODUCT, INV-MARK, and INV-CI in one bisect-safe commit. This slice may add tag-policy validation, its permanent automated proof, CI wiring, and preview documentation. It may not alter the production release path or crate behavior.

### OP-001 — authorized publication evidence

After SL-001 is merged and Q-002 is answered, create `milestone-3-preview.1` at the accepted merged commit and establish INV-NO-RUN, INV-INSTALL, and INV-LATEST. This phase creates no repository commit.

## Rollback and retirement

Before public use, an incorrectly placed local tag is discarded without pushing. After publication, retirement is deletion of the temporary milestone tag only after the ordinary release-please product release graduates M3; that operation is outside this ticket.
