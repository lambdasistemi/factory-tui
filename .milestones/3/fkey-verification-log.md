# F-key verification log — what this desk ran, not what lanes reported

The operator's standing goal: every feature PR testable from an F key
before merging. This file records the desk's own re-runs, because a lane's
proof of its own work is a claim.

## Bound and live

```
F1  released v0.1.0 profile   /nix/store/rc3343...-factory-tui-0.0.1
    cannot answer --version; version defect baked into the tag; only a new
    release fixes it (PR #29, outside M3's hands)

F2  main @ 0ba974e2           /nix/store/iqj8y9nn...-factory-tui-0.1.0
    verified: reports "0.1.0 (revision 0ba974e22db1...)" == HEAD

F3  #21 @ 0177eb08            /nix/store/i904dzhg...-factory-tui-0.1.0
    verified: reports "0.1.0 (revision 0177eb0805f7...)" == pushed head
    launched with FACTORY_TUI_CONFIG=~/.config/factory-tui/config.sampler-preview.toml
    (the branch's shipped example; the operator's real config is untouched)
```

Each binding target was checked executable before being bound. Bindings
point at store paths rather than `nix run`: a `nix run` inside a popup can
fail after the popup opens, giving a flash with no readable error.

## #20 (merged) — behaviour-free PR

Touched CI, Nix, docs, a release script; **no `src/`**. So an F-key
observation would have been identical whether its guard worked or not.
Its real evidence was the guard demonstrated **both ways**:

```
milestone-3-preview.1   accepted, exit 0
v0.1.1-ms3.1            rejected, exit 1
```

## #21 — the milestone's headline claim, on live state

Same session, same moment, old build vs new:

```
window                                OLD(main)    NEW(#21)
factory-tui-e8-t5-raw-tree            [RUNNING]    (unmarked)   <- finished lane, idle prompt
factory-tui-ms3-no-bugs               [RUNNING]    [RUNNING]    <- actually working
factory-tui-e15-t16-status-samplers   [RUNNING]    [RUNNING]    <- actually working
```

`factory-tui-e8-t5-raw-tree` is the **day-one reproduction**: a finished
lane sitting at an idle prompt with unsent composer text, which the
shipped browser insisted was RUNNING.

Note what the second and third rows are for. A fix that unmarked
everything would look identical in a burn-down and be worthless — the
claim is only proved because working seats *stayed* RUNNING.

## What the gate caught that review would not have

`#21` **exits 2 on the operator's existing config**:

```
invalid config ~/.config/factory-tui/config.toml:
removed [status] table; replace it with ordered [[sampler]] entries
```

Merged unverified, the operator's F1 and F2 would both have started
erroring instead of showing a browser. The hard failure is judged
correct — silently ignoring a removed table leaves a user with no status
and no explanation — but taking a breaking change is the operator's call,
so merge authorization is held pending their ruling.

## Contracts closed by #21, all `enforced: NONE` since day one

- **C1** status semantics — demonstrated above.
- **C3** published schema ⇄ crate — the branch's schema documents
  `[[sampler]]`/`[[reinterpreter]]`, states the removed `[status]` table
  is rejected, and states occupancy is not work.
- **C4** shipped example ⇄ crate — `examples/config.toml` parses through
  the real crate, exit 0, 96 lines. The shipped file, not a copy.

Crate, schema, example and skill moved together. First slice in M3 to do
so, and the reason C3/C4 exist.
