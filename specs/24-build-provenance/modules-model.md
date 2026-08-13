# Modules model — Ticket 24

Artifact ceiling: 1,600 bytes / 60 lines.

| ID | Component | Changed responsibility | Depends on | Must not own |
|---|---|---|---|---|
| MOD-24-MANIFEST | `Cargo.toml` | Sole maintained product-version declaration. | Release-please. | Nix or UI policy. |
| MOD-24-NIX | Existing flake/crane/check modules | Derive package/artifact version from MOD-24-MANIFEST, bind clean source revision, and reconcile manifest/package/executable identity. | MOD-24-MANIFEST and flake source metadata. | Runtime presentation or Git/time/network discovery. |
| MOD-24-IDENTITY | Dedicated Rust build-identity module | Own immutable build identity and its canonical display text. | Compiler package metadata and caller-supplied build revision. | CLI argument policy, tmux, preview, or release policy. |
| MOD-24-CLI | `src/main.rs` | Expose MOD-24-IDENTITY through an early non-interactive version command. | MOD-24-IDENTITY. | A duplicate formatter or version value. |
| MOD-24-UI | `src/ui.rs` | Display MOD-24-IDENTITY in the persistent popup chrome. | MOD-24-IDENTITY. | Build metadata discovery or preview/status changes. |
| MOD-24-DOCS | User documentation | Explain the command, identity fields, and fallback semantics. | Accepted runtime contract. | Internal orchestration details. |

Dependency direction is manifest → Nix/compiler metadata → identity → CLI/UI. Shared formatting is promoted to MOD-24-IDENTITY rather than duplicated by its consumers.

