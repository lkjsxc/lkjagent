# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Reconcile active data log and prompt anomaly contracts | [current-work/state-transition-network.md](current-work/state-transition-network.md) | implemented |
| 2 | Complete transition-kernel contract and data model | [current-work/state-transition-network.md](current-work/state-transition-network.md) | active |
| 3 | Store ledgers and snapshot adapter for kernel records | [current-work/state-transition-network.md](current-work/state-transition-network.md) | partially implemented |
| 4 | Prompt frame and dispatch admission through one decision id | [current-work/state-transition-network.md](current-work/state-transition-network.md) | partially implemented |
| 5 | Provider anomaly handling and endpoint recovery | [current-work/model-log.md](current-work/model-log.md) | partially implemented |
| 6 | Schema and batch-write recovery | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | partially implemented |
| 7 | Recovery shape enforcement for every fault class | [current-work/recovery-shape-enforcement.md](current-work/recovery-shape-enforcement.md) | partially implemented |
| 8 | Artifact address adoption and invalid-root durability | [current-work/artifact-address-controller.md](current-work/artifact-address-controller.md) | partially implemented |
| 9 | Artifact readiness and completion gate coverage | [current-work/artifact-ledger-completion.md](current-work/artifact-ledger-completion.md) | partially implemented |
| 10 | Compaction resume proof and status rendering | [current-work/durable-compaction-history.md](current-work/durable-compaction-history.md) | partially implemented |
| 11 | Idle-only maintenance and owner preemption proof | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | partially implemented |
| 12 | Provider exchange export and raw-case inspection | [current-work/model-log.md](current-work/model-log.md) | partially implemented |
| 13 | Replay benchmarks from current model run and owner failures | [current-work/verification-plan.md](current-work/verification-plan.md) | partially implemented |
| 14 | Final live Docker story run and compose verification | [current-work/verification-plan.md](current-work/verification-plan.md) | implemented |
| 15 | Personal diary, schedule, and TODO records | [current-work/personal-records.md](current-work/personal-records.md) | open |

## Current Evidence

`data/logs/current-model-run.md` is active evidence. The checked-in run proves
these facts:

- active case `1` is working at `recover-by-smaller-scope` in recovery;
- the owner task is the Chronos Fracture story bible rooted at
  `stories/chronos-fracture`;
- touched paths are `none`, so the run does not prove successful artifact
  creation or any file mutation;
- the evidence ledger has no useful evidence, including no
  `document-structure` or `artifact-readiness` evidence;
- the fault ledger repeats `MissingActionEnvelope` after earlier
  `bad envelope prose before action envelope` faults;
- the latest request still permits `<think>` tags and replays invalid assistant
  history shaped as prose or thinking before an `<action>` envelope;
- the latest response has empty `content`, `finish_reason=stop`,
  `closure_mode=Unclosed`, and nonzero completion tokens;
- the latest parse records `status=fault`, `content_bytes=0`, and
  `error=MissingActionEnvelope`;
- the latest export manifest lists `admission.json` and `observation.txt`, but
  both files are absent.

The active log does not contain the previously documented JSON-in-`files` or
oversized README mutation attempts. Those failure classes remain valid design
and benchmark targets only when supported by separate fixtures.

## Ordering Notes

- Documentation moves first, then code. Prompt guidance alone never closes an
  implementation row.
- The first implementation slice must prevent live prompts from teaching
  invalid `<think>` output and must sanitize invalid assistant history before it
  re-enters a provider request.
- Empty assistant content with nonzero completion tokens is a provider anomaly,
  not an ordinary parse fault. It must route through endpoint or provider
  recovery and must not loop forever as `MissingActionEnvelope`.
- The transition kernel remains the first authority target. Daemon wiring must
  use one snapshot, one explicit event, one persisted decision, one prompt
  frame, one admission view, one effect observation, and one next event.
- Graph policy is guidance for the snapshot. It is not fallback dispatch
  authority after runtime admission refuses a tool.
- Stale-action refusal must use the full staleness fingerprint: queue head,
  case id, graph node and phase, active mode, artifact root and cursor, latest
  fault, missing evidence, compaction pressure, maintenance state, and prompt
  frame head.
- Schema repair for `fs.batch_write` remains important, but the active checked-in
  log now points first at provider anomaly handling and prompt hygiene.
- Recovery routes must change repeated action shape. Repeated `graph.recover`
  refusal must force `graph.state`, `artifact.next`, deterministic inspection,
  or blocked handoff.
- Direct graph evidence, scaffold topology, README-only content, and
  owner-term-only pages do not satisfy artifact readiness.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. Owner work preempts maintenance before endpoint and
  before dispatch.
- Provider exchange manifests must list only files that exist or must record
  missing artifacts as explicit export errors.

## Current Narrowing Evidence

- Direct `graph.evidence` refuses audit-owned `artifact-readiness` and
  `document-structure` requirements.
- Hard compaction selects one `Compaction` active mode and does not render a
  model-authored `memory.save` action.
- Runtime authority examples for model-call modes come from the dispatcher
  registry renderer.
- Recovery has a closed `FaultClass` enum, route metadata, escalation route
  text, and blocked-handoff behavior on each pure recovery plan.
- Cached actions are refused before dispatch when queued owner work or
  runtime-only authority changes current decision fields.
- The daemon persists authority snapshot fields and CLI status prints active
  mode, evidence gaps, artifact root, recovery route, failed action, admitted
  tools, and next executable action.
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
  response, parse, admission, observation, timing, error, index, and export file
  writers, plus CLI list/show.
- Live compose smoke `/tmp/lkjagent-smoke-data-19` closed the Chronos story
  task after `artifact.audit` reported `readiness=story-semantic-content` and
  graph evidence recorded audit-owned `document-structure` and
  `artifact-readiness` rows.

## Remaining Proof Gaps

- The live prompt no longer permits `<think>` tags, and invalid assistant
  history is summarized instead of replayed as assistant output.
- Provider empty-content anomalies are separated from parse faults for new
  endpoint responses, but endpoint retry and blocked-handoff policy still needs
  full kernel ownership.
- Authority rows still need coverage for every dispatch, provider exchange,
  recovery, compaction, maintenance, and close path.
- Compaction snapshots need prompt-frame resume proof.
- Stale-action contradiction repair is not covered for every mode.
- Recovery shape-change enforcement is not proven for every live fault class.
- Rendered refusal examples need route-level proof across every policy path.
- Artifact adoption, ledger-root repair, invalid-root completion markers, and
  semantic readiness remain incomplete.
- Parser-level and runtime-level replay fixtures must cover the active current
  data-log failure.
