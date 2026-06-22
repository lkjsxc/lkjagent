# Runtime Authority Redesign

## Purpose

Own the current execution plan for making runtime authority the only layer that
selects active mission, admits tools, starts recovery, owns compaction, permits
maintenance, and closes cases.

## Contract

The runtime must treat model output and graph transitions as untrusted intent.
A pure authority reducer decides the next admissible action from durable state.
Effects execute only after that decision.

The dependency order is:

1. Refresh the documentation contract and current blocker queue.
2. Add uploaded run-log fixtures and authority reducer tests.
3. Introduce authority snapshot, event, decision, and admission types.
4. Implement the pure reducer and finite recovery ladders.
5. Route dispatcher and `agent.done` through the authority gate.
6. Add artifact ledger, profiles, readiness, adoption, repair, and batches.
7. Persist compaction snapshots with case, artifact, evidence, and recovery state.
8. Enforce idle-only maintenance and semantic memory pruning.
9. Repair protocol schemas and canonical examples.
10. Run local and Docker Compose gates.

## Invariants

- The graph guides; runtime authority decides.
- A completion node must not block read, audit, or repair tools needed to prove
  completion.
- Recovery must admit the smallest exact escape tool set.
- Maintenance must yield when owner work, recovery, artifact repair,
  verification, or hard compaction is active.
- Compaction must preserve resumability without model-authored `memory.save`.
- Content artifacts must pass content readiness before owner completion.

## Failure Cases

- `agent.done` succeeds after planning, scaffold, or file-existence evidence.
- `recover-repeat` loops through repeated `graph.state` or invalid parameters.
- A large raw `fs.write` retries instead of routing to bounded batches.
- Maintenance saves memory rows while owner work remains open.
- Compaction loses active artifact, recovery ladder, or next valid action.
- Dispatch refuses the exact example rendered by schema repair.

## Verification

Required evidence includes focused authority unit tests, uploaded benchmark
fixtures, artifact readiness tests, protocol repair tests, `quiet verify`, and
the Docker Compose final gate.

## Inventory

The 2026-06-21 orientation found 219 markdown files under 29 docs directories.
The expected authority, artifact, action-reliability, and uploaded-run fixture
contract files exist and are under the 200-line cap. Initial local doc gates
failed only because an untracked `tmp/prompt01.md` copy of the execution brief
was inside the repository and violated markdown shape and line limits.

## Related Files

- [../../architecture/runtime/active-mode/README.md](../../architecture/runtime/active-mode/README.md)
- [../../architecture/artifacts/README.md](../../architecture/artifacts/README.md)
- [../../architecture/action-reliability/README.md](../../architecture/action-reliability/README.md)
- [../../architecture/state-graph/README.md](../../architecture/state-graph/README.md)
- [../current-blockers.md](../current-blockers.md)

## Status

open. The documentation contract states that authority is computed before
endpoint calls and before dispatch, and that stale maintenance, compaction, or
idle actions must be refused when stronger runtime facts appear. Current code
has pure selection, endpoint authority cards, cached dispatch authority,
store-backed authority snapshot fields, stale maintenance-action refusal when
queued owner work appears before dispatch, and many completion refusals.
Stronger per-case authority history, broader stale-action refusal, and full
close-path proof remain open.
