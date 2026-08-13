# M3 contract registry

Cross-boundary agreements. `enforced: NONE` is a scheduled incident:
it gets a commissioned check or a recorded waiver, never silence.

## C1 — status semantics: crate ⇄ operator config ⇄ reader

parties:   `src/tree.rs::status_of`, `[status]` in a host `config.toml`,
           every human/agent reading `--dump` or the popup
invariant: a seat is reported RUNNING only when the sampled evidence
           distinguishes *working* from *occupying a pane*. A seat whose
           work state cannot be established is left unmarked.
enforced:  NONE — and actively violated. `status_of` returns Running when
           any pane's `pane_current_command` string-equals a configured
           name. Reproduced 2026-08-13: window
           `factory-tui:factory-tui-e8-t5-raw-tree` (pane %6385), a
           finished lane sitting at an idle prompt with unsent composer
           text, is reported `RUNNING` by `factory-tui --dump`.
owner:     T-A (first child)

## C2 — sampler field set: crate tmux query ⇄ configured samplers

parties:   `src/tmux.rs:68` (the `-F` format string that fixes which tmux
           fields exist), `Pane`/`Win` structs, configured samplers
invariant: every field a sampler may name is actually queried and reaches
           the evaluator; a sampler naming an unqueried field fails loudly
           at config-load, never silently never-matches.
enforced:  NONE. Today the only pane field carrying any status signal is
           `cmd`. **No currently queried field can distinguish a thinking
           agent from a waiting one**, so a field+regex sampler model built
           over today's query is architecturally incapable of satisfying
           C1 however good its regexes. Extending the query is therefore
           part of C1's fix, not a later optimization.
owner:     T-A

## C3 — published schema ⇄ crate serde structs

parties:   `skills/factory-tui/references/config.md` (published schema),
           `src/config.rs` (`Config`, `StatusConfig`, `Rule`, `TreeConfig`)
invariant: the published schema describes the config the crate actually
           accepts. An operator's agent filling a file from the doc alone
           produces a file the crate parses and honours.
enforced:  NONE. The doc is published (#13) but is hand-maintained beside
           the structs with no check binding them. It currently documents
           the C1-violating model (`running`: exact pane-command names),
           so the published schema propagates the defect.
owner:     T-A

## C4 — shipped example ⇄ crate

parties:   `examples/projection.toml`, `src/config.rs`
invariant: the shipped example parses under the real `Config` and produces
           the documented tree.
enforced:  NONE. No test or check loads `examples/projection.toml`; there
           is no `tests/` directory. The example can rot silently through
           any schema change, including T-A's.
owner:     T-A

## C5 — census output ⇄ sampler recipe selection

parties:   `skills/factory-tui/scripts/census`, `skills/factory-tui/SKILL.md`
           step 2, the default sampler recipe data
invariant: the census emits the fields a recipe is keyed on, so the setup
           skill can include only recipes that match the live box.
enforced:  NONE. `census` prints `session\twindow` only — nothing about
           pane commands or any status-bearing field. The skill's own
           procedure instructs an agent to census and then select, and the
           shipped instrument cannot answer the question. Ruling
           D-2026-08-13-status-samplers is unimplementable as shipped.
owner:     T-A

## C6 — host privacy: this box ⇄ the public repo

parties:   operator premise (2026-08-13, no private factory skills or host
           aliases in the public repo), `src/`, `docs/`, `skills/`,
           `examples/`, issues
invariant: no host session name, alias, or private skill name appears in a
           shipped artifact.
enforced:  NONE. `I5-NO-HOSTNAMES` is declared in
           `specs/5-raw-tmux-tree/spec.md:111` and `plan.md:100` as an `rg`
           over `src/`, but there is no such derivation in
           `nix/checks.nix` and no such step in `.github/workflows/ci.yml`.
           A declared invariant with no runner is the canonical check that
           cannot fail. Its declared scope is also narrower than the
           premise: host session names already sit in
           `docs/m1/flat-land.md:9` (`keri`) and `:45`
           (`0-machine`, `0-projects`), outside `src/`.
           Wiring it is folded into T-A (it is the mechanism that makes
           "the crate stays agent-agnostic" permanent). The pre-existing
           `docs/m1/` occurrences are M1 pages this milestone may not edit
           — see Q-001, escalated.
owner:     T-A (wiring) + project owner (Q-001, the M1-page occurrences)

## C7 — milestone artifact ⇄ product release line

parties:   `scripts/release/check-version-consistency`,
           `scripts/release/extract-notes`, release-please
           (`.release-please-manifest.json` = `0.1.0`), the tag-triggered
           `v*` workflows, `nix profile add github:...`
invariant: the milestone can publish a clearly-marked pre-release a
           stranger can install, and that pre-release can never be mistaken
           for — or displace — the product line.
enforced:  NONE, and the pipeline currently rejects it. A tag such as
           `v0.1.1-ms3.1` fails `check-version-consistency` (it requires
           tag == `Cargo.toml` version exactly, today `0.1.0`) and yields
           empty `extract-notes` (no matching `## ` heading). Meanwhile
           both release workflows fire on `tags: ["v*"]`, so an
           unmarked pre-release tag would publish and could take "Latest"
           from `v0.1.0` — worse than having no artifact at all.
owner:     T-B
