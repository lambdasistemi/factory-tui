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

parties:   the tag-triggered `v*` workflows (`linux-release.yml`,
           `darwin-release.yml`), release-please
           (`.release-please-manifest.json` = `0.1.0`), the pinned
           publisher `paolino/dev-assets@v0.1.0`,
           `nix profile add github:...`
invariant: the milestone publishes something a stranger can install, and
           it can never be mistaken for — or displace — the product line.
enforced:  DESIGNED 2026-08-13 by A-003; not yet built. **Changed** from
           the original "GitHub pre-release" design after T-B inspected
           the pinned publisher's source and found it invokes
           `gh release create` with neither `--prerelease` nor
           `--latest=false`, so a `v0.1.1-ms3.1` tag would have published
           an ordinary release and taken Latest from `v0.1.0`.

           The artifact is now a milestone-scoped git tag whose name does
           **not** match `v*`. No publisher runs and no release object
           exists, so "cannot displace Latest" holds **by construction**
           rather than by a flag someone must remember to pass.

           Verified, not assumed: only `linux-release.yml` and
           `darwin-release.yml` are tag-triggered and both match `v*`
           exactly (`ci`, `release`, `deploy-docs` are branch/dispatch);
           Nix resolves arbitrary refs — `main` -> `8a273de`,
           `v0.1.0` -> `40c0c51` — so
           `nix profile add github:lambdasistemi/factory-tui/<tag>` works
           on any tag name.

           Rejected: (a) changing `paolino/dev-assets` — shared
           infrastructure across repositories, a cross-project change M3
           may not make and must not depend on; (b) pre-creating the
           release around the external action — splits one invariant
           across workflow coordination and the action's unverified
           create-or-view behavior, a seam with no check.

           T-B must ship a check that fails if the tag name is
           `v`-prefixed, prove no workflow run was produced (Actions API,
           after the fact), and prove the stranger install.

           Production gates `check-version-consistency` and
           `extract-notes` are **not to be touched** — the milestone line
           never reaches them, and loosening a production gate to serve a
           temporary artifact is the trade this ruling avoids.

           Accepted limitation, recorded not glossed: the milestone line
           carries no prebuilt binaries for non-Nix hosts, because no
           publisher runs. Nix is the ruled install path
           (D-2026-08-13-nix-first), binaries are an explicit fallback,
           and this line is temporary.
owner:     T-B

## C8 — tree structure ⇄ name reinterpretation

parties:   the raw tmux census (sessions, windows, panes), configured
           reinterpreters, every reader of the tree
invariant: **For any tmux state and any reinterpreter configuration, the
           set of nodes and the parent relation are identical with and
           without reinterpretation applied. Only display strings
           differ.** No interpolation of synthesized levels, no splitting
           of one node into several, no merging, re-parenting, hiding or
           reordering.
enforced:  NONE yet — the mechanism does not exist. This entry exists
           **before** the code so the check ships with it rather than
           after a defect teaches us to want it.

           Origin: the deleted projection violated exactly this. D14 —
           `no-epic-t<id>` matched no rule, so those windows left the
           project tree. D15 — unmatched windows fell into a session
           bucket that collided with a synthesized folder, rendering two
           identical `• factory-tui` nodes. Both are structural lies
           produced by a feature whose stated job was presentation.

           Required: a property check over arbitrary tmux states and
           reinterpreter configs asserting node-set and parent-relation
           equality, demonstrated red against a reinterpreter that tries
           to split a node. A reinterpreter that *can* alter structure has
           become a projection and is rejected in review regardless of
           what it currently does.
owner:     the tree ticket (queued behind T-E #24)
