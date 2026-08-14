# Tasks

Mandate v3 (recut) · Base `main@ea047e04` · Slice **S1**, one behavior commit.

## Invariants

| ID | Contract | Must hold | Observable failure |
|---|---|---|---|
| I1 | C1 | A pane is RUNNING only when a sampler with status `running` matched it. Occupancy alone never yields RUNNING. | Waiting-pane fixture reports RUNNING |
| I2 | C1 | The active-evidence pane **is** RUNNING under the same config that leaves the waiting pane unmarked. | Active fixture unmarked — proves I1 is not vacuous |
| I3 | C2 | Every supported field is present in the tmux query and resolvable to its observed value. | A supported field is unqueried or unresolvable |
| I4 | C2 | A sampler naming an unsupported field fails config load, naming that field. | Load succeeds, or the sampler silently never matches |
| I5 | C2 | A config carrying a removed `[status]` key fails load. | Load succeeds and status marking is silently lost |
| I6 | C3 | The published schema's sampler shape and field vocabulary reconcile with `SUPPORTED_SAMPLER_FIELDS` and real `Config` acceptance. | Schema and parser disagree while the suite is green |
| I7 | C4 | The **actual shipped** `examples/config.toml` parses via the real `Config` and yields its documented result. | Shipped example unparseable or behaves differently |
| I8 | C-rollup | Status is per pane; `Unknown` panes contribute nothing; an all-unmarked window is `Unknown`, not `Idle`. | An all-unmarked window reports idle, or an unmarked pane lowers an established status |
| I9 | R4 | No agent, tool, or product name in Rust logic. | Any such literal in `src/` |
| I10 | brief | No host session name, alias, private skill name, or pane ID in tracked files or GitHub prose. | Any such string in the diff |
| I11 | R5 | Reinterpreter semantics unchanged; labels stay structurally inert. | `src/label.rs` semantics altered, or node set/parent relation changes |
| I12 | NOTE-010/011 | The pushed remote head is runnable and self-identifying: the printed revision equals the final local head and the pushed remote head. | Printed revision is stale, `unknown`, or disagrees with either head |

Every check must be permanent and reachable from `just ci`, except I12, which is
a post-push handback gate. A check that cannot be shown to fail does not close
its invariant.

## S1 tasks

- [ ] **T1** Add `SUPPORTED_SAMPLER_FIELDS` (all four already-queried fields) and
      `sampler_field_value`. (I3)
- [ ] **T2** Add the `Sampler` model and `[[sampler]]` parsing; remove
      `StatusConfig` after a dependency sweep. (I5)
- [ ] **T3** Validate samplers at load: field, status, regex, name uniqueness,
      and removed-`[status]`-key rejection. (I4, I5)
- [ ] **T4** Replace window-level occupancy with `status_of_pane`; make rollup
      ignore `Unknown` children and yield `Unknown` when none is established.
      (I1, I2, I8)
- [ ] **T5** Prove C1: waiting pane unmarked, active pane RUNNING, and a negative
      control restoring command-occupancy reddens the waiting check. (I1, I2)
- [ ] **T6** Prove C-rollup, including all-unmarked-is-not-idle under a
      controlled mutation that returns `Idle`. (I8)
- [ ] **T7** Prove C2: every supported field queried and resolvable; unsupported
      field rejected; removing a field from the query or resolver reddens the
      binding check. (I3, I4)
- [ ] **T8** Extend the crane source filter minimally per A-001 so the published
      schema reaches sandboxed derivations; prove the derivation source contains
      it. (I6)
- [ ] **T9** Update the published schema and `examples/config.toml` to the
      accepted model; bind each with a check proven red under a controlled
      mismatch. C4 loads the actual shipped file. (I6, I7)
- [ ] **T10** Confirm reinterpreters untouched and structurally inert. (I11)
- [ ] **T11** Record negative-control receipts for I1, I3, I6, I7, I8 and a full
      `just ci` receipt on the clean candidate. (all)
- [ ] **T12** Pre-handback runnability gate: rebase onto then-current `main`,
      re-run the complete gate on the rebased head, push the exact head, and
      prove the remote branch with the `git+https:` `nix run … -- --version`
      invocation. Journal headroom, literal command, complete output, exit
      status, and tested SHA. If `main` advances, rebase and re-run again.
      (I12)
