# M1 state

Updated: 2026-08-13

Legend: done · active/next · queued · blocked · unknown

## Picture

```text
Record
  ✅ constitution
  ✅ flat-land / compositor / decisions
  🟡 GitHub milestone #1 + issue #1 + PR
  ✅ wiki M1-State
  ⏳ Pages deploy of docs/ on main

Prototype (browse camera)
  ✅ tree of seats from live window names
  ✅ coloured snapshot observer (does not resize)
  ✅ Enter / double-click jump
  🟡 Nix-rooted `.#cli` + crane checks on this PR
  ⏳ standing --serve embed (not M1)

Release
  🟡 release-please rust + Linux/Darwin artifact workflows on this PR
  ⏳ first `v*` tag after merge (no GitHub release yet)

Next after M1
  ⏳ home session + --serve for tablet attach
```

## Blockers

None for publishing the record. The first installable GitHub release
waits on this PR merging and a later `v*` tag.
