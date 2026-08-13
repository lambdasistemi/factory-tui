# factory-tui

Public tmux browser. Default tree is session → window. Optional
local projection. The crate does not know any host's factory map.

Home: https://github.com/lambdasistemi/factory-tui

## Milestone map

| Id | Outcome | State | Owner / session |
|---|---|---|---|
| 1 | Flat-land / compositor record and browse-camera prototype | RETAINED — unvalidated experiment; not the shipped default; do not reopen from this desk | record on `milestones` at `.milestones/1/`; GH still open on purpose |
| 2 | Tmux browser + optional projection | COMPLETE — children merged; remaining defects moved to 3 | GH closed |
| 3 | No-bugs: published product can be called good enough to release | ACTIVE | `factory-tui:2:factory-tui-ms3-no-bugs` `%6420`; runtime `/tmp/ms-3`; owner STARTed |

Priority: 3 is the only ACTIVE execution milestone.

## Cross-milestone notes

M3 is the milestone whose acceptance satisfies the product for a
release. Release itself (tag, announce) is a later, separately
authorized act after M3's outcome audit.

M3 inherits D-2026-08-13-status-samplers: false RUNNING is a
release-blocking bug, not a later cleanup.
