# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Complete transition-kernel contract and long-novel authority wiring | [current-work/state-transition-network.md](current-work/state-transition-network.md) | implemented |
| 2 | Store ledgers and snapshot adapter for kernel records | [current-work/state-transition-network.md](current-work/state-transition-network.md) | implemented |
| 3 | Prompt frame and dispatch admission through one decision id | [current-work/state-transition-network.md](current-work/state-transition-network.md) | implemented |
| 4 | Provider anomaly handling and endpoint recovery | [current-work/model-log.md](current-work/model-log.md) | implemented |
| 5 | Schema and batch-write recovery | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | implemented |
| 6 | Recovery shape enforcement for every fault class | [current-work/recovery-shape-enforcement.md](current-work/recovery-shape-enforcement.md) | implemented |
| 7 | Artifact address adoption and invalid-root durability | [current-work/artifact-address-controller.md](current-work/artifact-address-controller.md) | implemented |
| 8 | Artifact readiness and completion gate coverage | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | implemented |
| 9 | Compaction resume proof and status rendering | [current-work/durable-compaction-history.md](current-work/durable-compaction-history.md) | implemented |
| 10 | Idle-only maintenance and owner preemption proof | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | implemented |
| 11 | Provider exchange export and raw-case inspection | [current-work/model-log.md](current-work/model-log.md) | implemented |
| 12 | Replay benchmarks from current long-novel run and owner failures | [current-work/verification-plan.md](current-work/verification-plan.md) | implemented |
| 13 | Final live Docker story run and compose verification | [current-work/verification-plan.md](current-work/verification-plan.md) | implemented |
| 14 | Personal diary, schedule, and TODO records | [current-work/personal-records.md](current-work/personal-records.md) | open |

## Current Evidence

`data/logs/current-model-run.md` and `data/logs/index.ndjson` are active
evidence. The checked-in run proves these facts:

- active case `1` is at node `document` in phase `execution`;
- the owner task is `Create a long novel. with detailed settings`;
- before owner enqueue, case-none maintenance repeats empty memory searches,
  no-op pruning, and `agent.done` instead of staying closed idle;
- the artifact root is `stories/long-novel-with-detailed-settings`;
- active tracks are `document-structure`, `action-param-reliability`, and
  `observability-ledger`;
- the evidence ledger has `plan` and `observation` only;
- the top touched-paths section says `none`, but transcript evidence shows
  `artifact.apply` created the scaffold and `fs.list` observed the root;
- `doc.audit` failed content readiness with 28 weak structure-only pages;
- two `fs.batch_write` attempts used invalid `<file>` child tags inside
  `<files>` and both refused with `invalid parameter: each block must start
  with path:`;
- `graph.recover` did not force a changed action shape before the repeated
  invalid batch write;
- turns 59 and 62 record `provider_anomaly.reasoning_only_response`;
- document audit and artifact readiness audit remain pending.

Historical Chronos evidence remains useful but is not the active checked-in
run. Recorded Chronos smoke created story-bible structure and plan evidence,
then timed out during weak-content repair with `document-structure` and
`artifact-readiness` missing. The old empty-content-with-usage turn remains a
historical provider anomaly fixture.

## Ordering Notes

- Documentation moves first, then code. Prompt guidance alone never closes an
  implementation row.
- Prompt hygiene is implemented for the latest request history: live prompts do
  not teach `<think>` output and invalid assistant history is not replayed as an
  assistant exemplar.
- Provider anomalies are classified before parsing for new endpoint responses.
  Endpoint retry has a bounded provider-failure pause and blocked handoff is a
  kernel-owned route.
- The transition kernel is the runtime authority. Daemon wiring uses one
  snapshot, one explicit event, one persisted decision, one prompt frame, one
  admission view, one effect observation, and one next event.
- Graph policy is guidance for the snapshot. It is not fallback dispatch
  authority after runtime admission refuses a tool.
- Stale-action refusal must use the full staleness fingerprint: queue head,
  case id, graph node and phase, active mode, artifact root and cursor, latest
  fault, missing evidence, compaction pressure, maintenance state, and prompt
  frame head.
- Schema repair for `fs.batch_write` is active because the long-novel log
  repeats invalid `<file>` child blocks after a schema refusal.
- Recovery routes must change repeated action shape. Repeated `graph.recover`
  or child-tag batch faults must force `graph.state`, `artifact.next`,
  deterministic inspection, or blocked handoff.
- Direct graph evidence, scaffold topology, README-only content, and
  owner-term-only pages do not satisfy artifact readiness.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. Owner work preempts maintenance before endpoint and
  before dispatch.
- Provider exchange manifests list only files that exist and record previously
  listed missing files as explicit `missing_files` entries.

## Current Narrowing Evidence

- Direct `graph.evidence` refuses audit-owned `artifact-readiness` and
  `document-structure` requirements.
- Hard compaction selects one `Compaction` active mode and does not render a
  model-authored `memory.save` action.
- Runtime authority examples for model-call modes come from the dispatcher
  registry renderer.
- Recovery has a closed `FaultClass` enum, route metadata, escalation route
  text, blocked-handoff behavior on each pure recovery plan, and parse-fault
  counters that clear only after successful observation.
- Cached actions are refused before dispatch when queued owner work or
  runtime-only authority changes current decision fields. Pure kernel admission
  also refuses stale actions after fault, evidence, maintenance, prompt-frame,
  artifact cursor, or compaction facts change.
- The daemon persists authority snapshot fields, including pending queue head,
  and CLI status prints active mode, evidence gaps, artifact root, recovery
  route, failed action, admitted tools, and next executable action.
- Recovery retry counts persist in SQLite by case, node, tool, parameter shape,
  and fault class.
- Dispatchable registry examples parse, validate, and reach dispatcher routes
  for covered paths.
- `artifact.next` returns fact-only candidate actions with
  `next_decision_required=true`.
- `fs.write` and `fs.batch_write` reject known scaffold phrases before
  mutation.
- Pure completion admission refuses `agent.done` while owner evidence gaps are
  present.
- Repeat refusals name the active mode, forbidden tool, shape-change
  requirement, preferred alternate, and registry example.
- Provider exchange logging has implemented store, atomic request, authority,
  response, parse, admission, observation, timing, error, index, export file
  writers, missing-file records, provider-anomaly status, and CLI list/show. New
  authority files include persisted decision, prompt-frame, authority, and
  staleness identifiers.
- Historical Chronos compose evidence includes a closed story task after
  `artifact.audit` reported `readiness=story-semantic-content` and graph
  evidence recorded audit-owned `document-structure` and `artifact-readiness`
  rows; it is not the active checked-in run.

## Remaining Proof Gaps

No runtime-authority proof gaps remain after focused tests, `quiet verify`, and
the Docker Compose final gate. The open queue item is personal diary,
schedule, and TODO projection work, which is outside the runtime-authority
redesign.
