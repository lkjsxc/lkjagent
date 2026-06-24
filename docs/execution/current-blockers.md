# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Documentation and current-state reconciliation | [current-work/state-transition-network.md](current-work/state-transition-network.md) | active |
| 2 | Complete transition-kernel contract and data model | [current-work/state-transition-network.md](current-work/state-transition-network.md) | partially implemented |
| 3 | Store ledgers and snapshot adapter for kernel records | [current-work/state-transition-network.md](current-work/state-transition-network.md) | partially implemented |
| 4 | Prompt frame and dispatch admission through one decision id | [current-work/state-transition-network.md](current-work/state-transition-network.md) | partially implemented |
| 5 | Schema and batch-write recovery | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | partially implemented |
| 6 | Recovery shape enforcement for every fault class | [current-work/recovery-shape-enforcement.md](current-work/recovery-shape-enforcement.md) | partially implemented |
| 7 | Artifact address adoption and invalid-root durability | [current-work/artifact-address-controller.md](current-work/artifact-address-controller.md) | partially implemented |
| 8 | Artifact readiness and completion gate coverage | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | partially implemented |
| 9 | Compaction resume proof and status rendering | [current-work/durable-compaction-history.md](current-work/durable-compaction-history.md) | partially implemented |
| 10 | Idle-only maintenance and owner preemption proof | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 11 | Provider exchange export and raw-case inspection | [current-work/model-log.md](current-work/model-log.md) | partially implemented |
| 12 | Replay benchmarks from current model run and owner failures | [current-work/verification-plan.md](current-work/verification-plan.md) | open |
| 13 | Final live Docker story run and compose verification | [current-work/verification-plan.md](current-work/verification-plan.md) | open |

## Current Evidence

`data/logs/current-model-run.md` is active evidence. The latest Chronos
Fracture run proves both progress and failure:

- stale maintenance action refusal worked when owner work appeared;
- the owner task stayed active at `document-completion-check` in recovery;
- the run created directories but did not prove document-structure or
  artifact-readiness;
- repeated missing action blocks and parse faults reached at least count 5;
- `fs.batch_write` repeatedly missed `files` and used a path-shaped unknown
  parameter;
- recovery rendered examples but did not force a productive write, audit, or
  deterministic inspection route;
- no successful close claim is valid until a replayed or live story run and
  Docker Compose verification prove this failure class is gone.

## Ordering Notes

- Documentation moves first, then code. Prompt guidance alone never closes an
  implementation row.
- Rows stay open until focused tests, quiet verify, and any required Docker
  Compose verification prove the uploaded failure patterns cannot recur.
- The transition kernel remains the first implementation target. A pure kernel
  data module and invariant tests now exist. The snapshot adapter computes
  authority and staleness fingerprints, rejects synthetic active case ids, and
  ignores maintenance due state while owner work exists. Store ledgers now
  include prompt frames and observations, plus foreign-key proof that admissions
  require a decision. Daemon wiring still needs one snapshot, one explicit event, one
  persisted decision, one prompt frame, one admission view, one effect
  observation, and one next event.
- Graph policy is guidance for the snapshot. It is not fallback dispatch
  authority after runtime admission refuses a tool.
- Stale-action refusal must use the full staleness fingerprint: queue head,
  case id, graph node and phase, active mode, artifact root and cursor, latest
  fault, missing evidence, compaction pressure, maintenance state, and prompt
  frame head.
- Schema repair for `fs.batch_write` must either safely normalize path-shaped
  unknown parameters or refuse with a concrete path-scoped canonical example.
- Recovery routes must change action shape after repeated faults and keep the
  observation, audit, repair, and batch tools needed to escape.
- Artifact readiness must be tied to the current artifact id. Direct graph
  evidence, scaffold topology, README-only content, and owner-term-only pages
  do not satisfy readiness.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. Owner work preempts maintenance before endpoint and
  before dispatch.
- Provider exchanges must be exportable as sanitized replay records before raw
  model-run failures can be fully reproduced.
- The live Chronos story run is required before closing the reliability
  redesign.

## Current Narrowing Evidence

- Direct `graph.evidence` refuses audit-owned `artifact-readiness` and
  `document-structure` requirements.
- Hard compaction selects one `Compaction` active mode and does not render a
  model-authored `memory.save` action.
- Runtime authority examples for model-call modes come from the dispatcher
  registry renderer.
- Recovery has a closed `FaultClass` enum, route metadata, escalation route
  text, and blocked-handoff behavior on each pure recovery plan.
- Cached maintenance actions are refused before dispatch when queued owner work
  changes the current authority.
- The daemon persists authority snapshot fields and CLI status prints active
  mode, evidence gaps, artifact root, recovery route, failed action, admitted
  tools, and next executable action.
- Owner-task prompt authority no longer renders an empty tool surface while
  recommending a graph-admitted action.
- Recovery retry counts persist in SQLite by case, node, tool, parameter shape,
  and fault class.
- Compaction notices preserve latest observation summary and `artifact.next`
  batch cursor in addition to graph, recovery, artifact, and next-action fields.
- Dispatchable registry examples parse, validate, and reach dispatcher routes
  for covered paths.
- `artifact.next` and stricter content audit support bounded cookbook and story
  recovery examples.
- `fs.write` and `fs.batch_write` reject known scaffold phrases before
  mutation.
- Pure completion admission refuses `agent.done` while owner evidence gaps are
  present and keeps audit, repair, and batch routes visible.
- Repeat refusals name the active mode, forbidden tool, shape-change
  requirement, preferred alternate, and registry example.
- Memory pruning deletes exact duplicates and merges same-title high-overlap
  rows with source IDs.
- Provider exchange logging has implemented store, atomic request, authority,
  response, parse, admission, observation, timing, error, index, and export file
  writers, plus CLI list/show.

## Remaining Proof Gaps

- Prompt frame rendering from persisted decisions is the next code slice.
- Authority rows still need coverage for every dispatch, provider exchange,
  recovery, compaction, maintenance, and close path.
- Compaction snapshots need status rendering and prompt-frame resume proof.
- Stale-action contradiction repair is not covered for every mode.
- Recovery shape-change enforcement is not proven for every live fault class.
- Rendered refusal examples need route-level proof across every policy path.
- Artifact adoption, ledger-root repair, invalid-root completion markers, and
  semantic readiness remain incomplete.
- Maintenance owner preemption remains open before endpoint and every dispatch
  path.
- Parser-level and runtime-level replay fixtures must assert productive next
  actions, not only error recognition.
