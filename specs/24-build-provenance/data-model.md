# Data model — Ticket 24

Artifact ceiling: 1,200 bytes / 45 lines.

## DATA-24-BUILD-IDENTITY

Immutable process-wide value with:

| Field | Type | Validation |
|---|---|---|
| `version` | static string | Equals compiler package version sourced from `Cargo.toml`; non-empty. |
| `revision` | static string | Exact clean flake revision when supplied; otherwise an explicit non-exact fallback; non-empty. |

Relationships:

- One DATA-24-BUILD-IDENTITY is compiled into each binary.
- CLI and UI render the same value.
- Nix package metadata and DATA-24-BUILD-IDENTITY.version reconcile to MOD-24-MANIFEST.

State invariants:

- Neither field changes during process execution.
- An unavailable/dirty revision is labeled as such and is never accepted as an exact commit.
- Version formatting does not create a new maintained semantic-version value.

