# Current Blockers

## Purpose

The dependency-ordered implementation queue. Rows move to done only when the
linked task contract names focused evidence and the actual gates that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Runtime authority is not the single source of active mission and tool admission | [current-work/state-transition-network.md](current-work/state-transition-network.md) | open |
| 2 | Recovery can block the exact observation, repair, or batch tool needed to escape | [current-work/recovery-shape-enforcement.md](current-work/recovery-shape-enforcement.md) | open |
| 3 | Content artifacts can pass on scaffold or planning evidence instead of real content | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | open |
| 4 | Completion can close while artifact readiness, verification, or recovery evidence is missing | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | open |
| 5 | Compaction snapshots are not rich enough to resume artifact repair and recovery | [current-work/durable-compaction-history.md](current-work/durable-compaction-history.md) | open |
| 6 | Maintenance is not strictly idle-only and preemptable by owner work | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 7 | Uploaded run logs are not fully covered by benchmark regressions | [current-work/verification-plan.md](current-work/verification-plan.md) | open |
| 8 | Semantic maintenance pruning still allows repeated low-value memory rows | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 9 | Protocol schema repair can render examples that dispatch later rejects | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | open |

## Owner Failure Evidence

The uploaded model run logs are active evidence. They show invalid parameter
loops, contradictory maintenance and graph policy layers, duplicate memory
writes, unsafe long-content writes, scaffold-only completion, compaction
memory-action deadlocks, and maintenance restarts after no useful work.

## Ordering Notes

- Rows stay open until focused tests and Docker Compose verification prove the
  uploaded failure patterns cannot recur.
- Documentation moves first, then code. Prompt guidance and scaffold output do
  not close implementation rows.
- The first runtime authority code slice is the transition kernel in
  [current-work/state-transition-network.md](current-work/state-transition-network.md):
  snapshot, explicit event, decision, prompt frame, admission, effect, and next
  event.
- Runtime authority, recovery, artifact readiness, completion, compaction,
  maintenance, fixtures, memory, and protocol repair move in that order unless
  repository inspection proves a stricter dependency.
- Stable YOLO-mode, active-mode, recovery, prompt-source, and artifact docs are
  contracts. They are not proof that the runtime implements the behavior.
- Docker Compose verification is required for any implemented runtime claim.

## Current Narrowing Evidence

- Direct `graph.evidence` refuses audit-owned `artifact-readiness` and
  `document-structure` requirements.
- Hard compaction selects one `Compaction` active mode and does not render a
  model-authored `memory.save` action.
- Runtime authority examples for model-call modes come from the dispatcher
  registry renderer.
- Runtime recovery has a closed `FaultClass` enum, route metadata, escalation
  route text, and blocked-handoff behavior on each pure recovery plan.
- Cached maintenance actions are refused before dispatch when queued owner work
  changes the current authority.
- The daemon persists authority snapshot fields and CLI status prints active
  mode, evidence gaps, artifact root, recovery route, failed action, admitted
  tools, and next executable action.
- Recovery retry counts persist in SQLite by case, node, tool, parameter shape,
  and fault class.
- Compaction notices now preserve the latest observation summary and
  artifact.next batch cursor in addition to graph, recovery, artifact, and
  next-action fields.
- Dispatchable registry examples parse, validate, and reach the dispatcher
  route instead of failing schema repair before routing.
- Recovery-plan examples parse, validate, and are admitted by recovery policy
  when they are model-authored tools.
- `artifact.next` and stricter content audit support bounded cookbook and story
  recovery examples.
- `fs.write` and `fs.batch_write` reject known scaffold phrases before mutation.
- `graph.recover` omits `graph.plan` unless it is admitted and still needed.
- Pure completion admission refuses `agent.done` while owner evidence gaps are
  present and keeps audit, repair, and batch routes visible.
- Repeat refusals name the active mode, forbidden tool, shape-change
  requirement, preferred alternate, and registry example.
- The benchmark matrix includes Japanese-cookbook drift,
  document-structure evidence ownership, batch-write schema faults, shell
  parameter faults, queue interruption, compaction resume, and repeated
  recovery signatures.
- Memory pruning deletes exact duplicates and merges same-title high-overlap
  rows with source IDs.

## Remaining Proof Gaps

- Authority snapshots and transition rows exist for turn authority refresh, but
  still need full case-specific fields and coverage for every dispatch and
  completion path.
- Compaction snapshots still need durable history beyond the latest notice.
- Broader stale-action contradiction repair is not yet covered for every mode.
- Recovery shape-change enforcement is not yet proven for every live fault class.
- Refusal-rendered examples still need route-level proof for every policy path.
- Artifact adoption and semantic readiness remain incomplete.
- Compaction snapshots still need richer last-observation and batch-cursor
  fields.
- Maintenance rewrite pruning and pre-dispatch owner preemption remain open.
- Rendered recovery examples still need a registry-wide parse, validation, and
  dispatch proof.
