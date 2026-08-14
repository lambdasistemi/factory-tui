# M3 preview tag modules model

Artifact ceiling: 70 lines / 4 KiB.

| ID | Component | Responsibility | Dependencies | Constraints |
|---|---|---|---|---|
| MOD-001 | Milestone tag policy | Own acceptance and rejection of milestone preview tag names. | DATA-001; FN-001 | Must not participate in product tag validation. |
| MOD-002 | Repository verification | Prove MOD-001 accepts the supported namespace, rejects the product namespace, and remains reachable from CI. | MOD-001; DATA-001 | Both directions are permanent assertions. |
| MOD-003 | M3 preview documentation | Own operator/user meaning, install surface, limitation, distinction, and graduation contract. | DATA-001 | Must not describe a GitHub release or prebuilt artifact. |
| MOD-004 | Existing production release pipeline | Continue owning `v<x.y.z>` version checks, notes, release-please, and binary publication without modification. | Existing release configuration | Byte-identical to slice base. |

Dependency direction is MOD-002 to MOD-001 and MOD-003 to DATA-001. MOD-001 has no dependency on MOD-004, preserving separate namespaces.
