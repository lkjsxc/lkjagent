# Workflow

## Purpose

The docs-first change loop every contributor (human-directed or autonomous)
follows, and the gate each change class must pass before handoff.

## The Loop

1. Orient: read [../current-state.md](../current-state.md), then the README
   of the affected area.
2. Choose work: the user-named task, or the first open blocker in
   [../execution/current-blockers.md](../execution/current-blockers.md).
3. Build the case state: objective, constraints, assumptions, risks, evidence
   requirements, candidate files, and next action.
4. Update docs first: change the contract, the decision record if a
   decision moves, and [../current-state.md](../current-state.md).
5. Implement the narrowest change that satisfies the new contract.
6. Add the focused test that proves it, per
   [functional-style.md](functional-style.md).
7. Run the focused gate for the change class (table below).
8. Update execution state: the blocker row, the task file Status.
9. Run the pre-handoff gate for the change class.
10. Commit per [commit-protocol.md](commit-protocol.md) and hand off per
    [../agent/handoff.md](../agent/handoff.md).

## Gates by Change Class

| Change class | Focused gate | Pre-handoff gate |
| --- | --- | --- |
| docs only | check-docs, check-lines | same |
| one crate | cargo test -p that crate, fmt, clippy | quiet verify |
| cross-crate or runtime behavior | affected crate tests | quiet verify, then compose verify |
| Dockerfile, compose, CI | the touched service builds and runs | compose verify |
| xtask or checks | xtask self-tests | quiet verify |

Gate commands are defined in
[../operations/verification.md](../operations/verification.md); until the
xtask exists, its interim checks substitute for check-docs, check-lines, and
quiet verify on docs-only work.

## Rules of the Loop

- Never implement against an unwritten contract; if the doc is missing, the
  doc is the first deliverable.
- Never skip step 6: behavior without a test does not count as implemented
  in [../current-state.md](../current-state.md).
- A blocked step becomes an open question in the task file, not an
  improvised workaround.
