# Current Blockers

## Purpose

The dependency-ordered implementation queue. Rows move to done only when the
linked task contract names focused evidence and the actual gates that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Generated documentation can repeat universal scaffold boilerplate | [current-work/document-structure-redesign.md](current-work/document-structure-redesign.md) | partially implemented |
| 2 | Generated path names can concatenate unrelated owner topics | [current-work/document-structure-redesign.md](current-work/document-structure-redesign.md) | partially implemented |
| 3 | Document and artifact tools confuse root directories with Markdown leaf paths | [current-work/artifact-address-controller.md](current-work/artifact-address-controller.md) | partially implemented |
| 4 | Runtime authority is not the single source of active mission and tool admission | [current-work/state-transition-network.md](current-work/state-transition-network.md) | partially implemented |
| 5 | Recovery can block the exact observation, repair, or batch tool needed to escape | [current-work/recovery-shape-enforcement.md](current-work/recovery-shape-enforcement.md) | partially implemented |
| 6 | Content artifacts can pass on scaffold or planning evidence instead of real content | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | partially implemented |
| 7 | Completion can close while artifact readiness, verification, or recovery evidence is missing | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | partially implemented |
| 8 | Compaction snapshots are not rich enough to resume artifact repair and recovery | [current-work/durable-compaction-history.md](current-work/durable-compaction-history.md) | partially implemented |
| 9 | Maintenance is not strictly idle-only and preemptable by owner work | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 10 | Uploaded run logs are not fully covered by benchmark regressions | [current-work/verification-plan.md](current-work/verification-plan.md) | open |
| 11 | Raw provider exchanges are not logged as replayable JSON under `data/logs` | [current-work/model-log.md](current-work/model-log.md) | partially implemented |
| 12 | Semantic maintenance pruning still allows repeated low-value memory rows | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | partially implemented |
| 13 | Protocol schema repair can render examples that dispatch later rejects | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | partially implemented |

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
- The first implementation slice removes the repeated scaffold generator and
  adds generated path hygiene in
  [current-work/document-structure-redesign.md](current-work/document-structure-redesign.md).
- Root/path address repair still precedes runtime authority because focused
  reducer refusals exist, but old `.md` directory adoption, ledger-root
  adoption, and full policy-path dispatch proof remain open.
- The first runtime authority code slice remains the transition kernel in
  [current-work/state-transition-network.md](current-work/state-transition-network.md):
  snapshot, explicit event, decision, prompt frame, admission, effect, and next
  event.
- Documentation generator repair, path hygiene, runtime authority, recovery,
  artifact readiness, completion, compaction, maintenance, fixtures, provider
  exchange logging, memory, and protocol repair move in that order unless
  repository inspection proves a stricter dependency.
- Stable YOLO-mode, active-mode, recovery, prompt-source, and artifact docs are
  contracts. They are not proof that the runtime implements the behavior.
- Docker Compose verification is required for any implemented runtime claim.

## Current Narrowing Evidence

- `doc.scaffold` no longer emits the old universal leaf body; generated leaves
  use explicit structure-only or owner-term-only state instead.
- `artifact.next` generic documentation examples now name the root, kind, target
  path, source boundary, and required audit evidence.
- `artifact.next` accepts a focused weak path and no longer renders
  artifact.audit for a Markdown file root in focused tests.
- `doc.scaffold`, `artifact.apply`, `doc.audit`, and `artifact.audit` now have
  semantic Markdown-root refusals covered by focused tests.
- `fs.batch_write` accepts JSON-in-files recovery payloads while keeping line
  protocol canonical.
- Multi-topic requests such as model endpoint, Minecraft, Windows, Japan, and
  the United States select a relation-first seed and topic pages instead of one
  combined filename.
- `doc.audit` reports topology, links, path hygiene, content readiness, artifact
  readiness, and exact path-hygiene failures.
- `check-docs` rejects the old generated boilerplate outside a small allowlist.
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
- Owner-task prompt authority no longer renders an empty tool surface while
  recommending a graph-admitted action.
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
- The benchmark matrix includes missing action blocks, interrupted output,
  stop-closure repair, contradictory authority, provider exchange logging,
  Japanese-cookbook drift, document-structure evidence ownership, batch-write
  schema faults, artifact address loops, shell parameter faults, queue
  interruption, compaction resume, and repeated recovery signatures.
- Memory pruning deletes exact duplicates and merges same-title high-overlap
  rows with source IDs.
- Provider exchange logging has a design contract, implemented store, atomic
  request, authority, response, parse, admission, observation, timing, error,
  index, and export file writer, and CLI list/show.

## Remaining Proof Gaps

- Authority snapshots and transition rows exist for turn authority refresh with
  case, node, phase, artifact, and evidence fields, but still need coverage for
  every dispatch and completion path.
- Compaction snapshots have pre/post durable rows and latest reopen lookup, but
  status rendering for the latest snapshot remains open.
- Broader stale-action contradiction repair is not yet covered for every mode.
- Recovery shape-change enforcement is not yet proven for every live fault class.
- Refusal-rendered examples now dispatch for covered address paths, but still
  need route-level proof for every policy path.
- Address-controller Docker Compose evidence is recorded for the focused slice;
  a fresh live SF-novel rerun and adoption proof remain open.
- Artifact adoption and semantic readiness remain incomplete.
- Compaction snapshots still need prompt-frame resume proof after rebuild.
- Maintenance rewrite pruning and pre-dispatch owner preemption remain open.
- Parser-level replay fixtures still need direct runtime execution beyond the
  uploaded-run benchmark text signatures.
- Rendered recovery examples still need a registry-wide parse, validation, and
  dispatch proof.
