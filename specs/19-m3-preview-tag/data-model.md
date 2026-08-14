# M3 preview tag data model

Artifact ceiling: 60 lines / 4 KiB.

## DATA-001 — MilestonePreviewTag

| Field | Type | Validation |
|---|---|---|
| milestone | fixed identifier | Exactly `milestone-3`. |
| channel | fixed identifier | Exactly `preview`. |
| ordinal | positive decimal integer | Greater than zero; decimal digits only. |

The serialized identity is the three fields joined in order with hyphen between milestone and channel and a dot before ordinal. It has no leading `v` and no product semantic-version identity.

## Relationships

- Each preview ordinal identifies one immutable Git commit.
- The first supported artifact is DATA-001 with ordinal 1.
- DATA-001 is retired after M3 graduates through an ordinary product release; it is never promoted or renamed into a product tag.
