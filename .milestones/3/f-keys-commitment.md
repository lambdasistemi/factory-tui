# Operator commitment — tell me when the F-keys are fixed and operative

Requested by the operator, 2026-08-13, through the M3 desk channel.
Recorded here so a successor honours it if this desk dies.

## What the operator asked for

Two things, in sequence:

1. Bind a key to what is **ready for the milestone** so it can be tested,
   instead of only ever testing what was released.
2. Fixed the lie about F1 — it was described as the released package and
   the artifact identifies itself as `0.0.1`.

And then: **let me know when the Fs are fixed and operative.**

## The bar — all four, or it is not "operative"

This is deliberately not "#24 merged". A merged ticket is a claim; the
operator asked for a working key.

1. **#24 merged.** Version derived from `Cargo.toml` — not restated in
   `nix/` — with a check that goes red if the two disagree, demonstrated
   red on purpose.
2. **A build can say which commit it is** at runtime, without a UI.
3. **A key runs an in-flight branch**, via
   `nix run 'git+https://github.com/lambdasistemi/factory-tui?ref=<branch>'`
   — never the `github:` form, which is HTTP 403 on this host.
4. **F1 and the branch key are visibly different builds when pressed.**
   Not inferred from store hashes: actually run both and see two
   different provenance strings.

Point 4 is the whole test. Points 1–3 can all be true while the operator
still cannot tell what is on screen, and "cannot tell what I am looking
at" is the complaint that started this.

## Reporting rule

Report **operative** only when 4 has been observed. If 1–3 land and 4 has
not been checked, report progress — do not report success. This desk owns
a milestone whose entire subject is a browser that lied about its own
state; announcing a fixed F-key on the strength of a merge commit would
be the same defect at the desk's altitude.

## Status

- #24 — https://github.com/lambdasistemi/factory-tui/issues/24 — live,
  PR #25 open, `build` check currently FAILURE (in-flight draft).
- Mechanism for 3 — proven working 2026-08-13, exit 0, zero disk cached.
- 4 — not yet possible; every build still reports `0.0.1`.
