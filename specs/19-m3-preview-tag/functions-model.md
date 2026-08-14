# M3 preview tag functions model

Artifact ceiling: 60 lines / 4 KiB.

## FN-001 — milestone tag validation command

- **Entrypoint:** `scripts/release/check-milestone-tag`
- **Argument:** `tag: String`, exactly one required positional value.
- **Result:** no data result; a human-readable confirmation may be written to standard output.
- **Success effect:** exits zero only when `tag` is a valid DATA-001 serialization.
- **Failure effect:** exits non-zero with a diagnostic when the argument is missing or invalid, including every `v`-prefixed value.
- **Side effects:** none beyond standard output/error.

No production release function changes in this ticket.
