# Functions model — Ticket 24

Artifact ceiling: 1,200 bytes / 45 lines.

| ID | Owner | Signature | Constraint / effect |
|---|---|---|---|
| FN-24-CURRENT | MOD-24-IDENTITY | `current() -> BuildIdentity` | Returns DATA-24-BUILD-IDENTITY from compile-time metadata; no I/O. |
| FN-24-DISPLAY | MOD-24-IDENTITY | `display(identity: BuildIdentity) -> String` | Produces the single canonical version-plus-revision text consumed by CLI and UI. |
| FN-24-VERSION-REQUEST | MOD-24-CLI | `is_version_request(args: &[String]) -> bool` | True only for the supported standalone version request; no tmux access. |

No changed signature is authorized in `src/tmux.rs`, preview handling, or status classification.

