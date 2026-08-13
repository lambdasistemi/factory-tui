# Decisions

Superseded rulings stay; name the successor.

## D-2026-08-13-raw-default

Default tree is tmux sessions and windows. Projection is optional and
data-only. Host aliases live in the operator's local file, never in
the crate.

Premise: public product used by more than this host.
Status: in force.

## D-2026-08-13-m1-unvalidated

M1 is an unvalidated experiment. Docs must not present it as the
shipped default. Leave M1 research pages alone unless a later ruling
reopens M1.

Premise: operator. Status: in force.

## D-2026-08-13-nix-first

Nix is the supported install path. Homebrew and GitHub binaries are
fallbacks for hosts without Nix.

Premise: the people this is for already have Nix. Status: in force.

## D-2026-08-13-status-samplers

Running is not “a pane's current command is an agent binary.” That
samples occupancy, not work. Status sampling belongs in configuration
as named samplers (field + regex → status). The crate evaluates
configured samplers only; it does not hard-code agent names. A default
set of sampler recipes lives as data the skill can copy. The skill
censuses live pane commands and includes only the recipes that match
this box. Unknown agents stay unmarked. The configuration schema is
published so another agent can fill a file without guessing.

Premise: operator, after false RUNNING on waiting desks.
Status: in force. Execution: not founded as a milestone yet.

## D-2026-08-13-owner-hands

This desk is the project owner. Hands: ruling, ask to a milestone
owner (or the right technical owner), ledger sweep. No product code,
review, merge, or ticket work from this window.

Premise: project-orchestrator contract + operator correction.
Status: in force.
