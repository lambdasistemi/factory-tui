# M3 preview tag specification

Artifact ceiling: 110 lines / 6 KiB.

## User stories

- **US-001:** A stranger can install the current Milestone 3 preview through a named Nix flake ref without cloning the repository.
- **US-002:** A release operator can prove the preview ref is outside the product-release namespace before creating it.
- **US-003:** A reader can tell that the preview is temporary, Nix-only, and distinct from the latest product release.

## Requirements

- **RQ-001:** The supported tag family is `milestone-3-preview.<ordinal>`, where the ordinal is a positive decimal integer.
- **RQ-002:** The repository MUST expose a committed validation command that accepts every RQ-001 tag and rejects every tag outside that family.
- **RQ-003:** Permanent automated coverage MUST demonstrate both acceptance of `milestone-3-preview.1` and rejection of a `v`-prefixed candidate.
- **RQ-004:** Production release-please configuration, production version consistency, and production notes extraction MUST remain unchanged.
- **RQ-005:** Documentation MUST identify the preview as temporary, name the exact Nix install command, state that it has no GitHub release or prebuilt binary assets, and distinguish it from `v0.1.0`.
- **RQ-006:** Documentation MUST state that M3 graduation uses the next ordinary release-please `v<x.y.z>` release and retires the preview tag.
- **RQ-007:** The first preview tag MUST NOT be pushed without explicit Q-002 authorization.

## Invariants

- **INV-NS:** A tag accepted by the milestone validator can never begin with `v`. Failure means any `v`-prefixed value exits successfully; success means the negative control exits non-zero.
- **INV-PRODUCT:** M3 capability does not alter any production release gate or release-please owner. Failure means any forbidden production file differs from the slice base; success means all remain byte-identical.
- **INV-MARK:** The supported install reference visibly contains both `milestone-3` and `preview`. Failure means the documented supported ref omits either marker; success means it is exactly `milestone-3-preview.1` for the first artifact.
- **INV-CI:** The tag-policy assertion is reachable from the repository CI gate. Failure means the assertion can be removed or made permissive while CI stays green; success requires a seeded invalid input to make the relevant check fail.
- **INV-NO-RUN:** Pushing the authorized preview tag produces zero Actions workflow runs for the tag ref. This is verified after the tag push through the Actions API.
- **INV-INSTALL:** A clean, isolated Nix profile/store can install the GitHub tag ref and run the installed binary.
- **INV-LATEST:** After the authorized tag push, the GitHub latest-release endpoint still identifies `v0.1.0`, and no release object exists for the preview tag.

## Non-goals

- Publishing GitHub releases, Homebrew formulae, or binary bundles for the preview.
- Changing crate behavior or crate/release version numbers.
- Changing any M1 or milestone-branch artifact.
