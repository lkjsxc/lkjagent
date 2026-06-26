# State Transition Network

## Purpose

This task makes runtime authority, graph guidance, prompt rendering, dispatch,
recovery, compaction, maintenance, and completion use one persisted
state-transition kernel.

## Contract

Implement the loop documented in
[../../architecture/state-graph/transition-network.md](../../architecture/state-graph/transition-network.md)
and [../../architecture/runtime/authority/decision-ledger.md](../../architecture/runtime/authority/decision-ledger.md):

```text
DurableReadModel -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> RuntimeEffectCommand
RuntimeEffectCommand -> EffectObservation
EffectObservation -> RuntimeEvent
RuntimeEvent -> next RuntimeDecision
```

The decision is persisted before prompt rendering, endpoint calls, dispatch,
compaction effects, maintenance effects, or close attempts. Model text supplies
intent or content. Runtime data supplies authority.

## Kernel Records

The kernel data model must include these records before broader daemon rewiring:

- `RuntimeSnapshot`: case, graph node, graph phase, task family, objective,
  constraints, queue facts, evidence facts, artifact head, weak paths, cursor,
  latest fault, retry counters, last observation, context pressure,
  compaction head, maintenance state, latest decision id, prompt frame head,
  authority fingerprint, and staleness fingerprint.
- `RuntimeEvent`: the closed event catalog for owner input, queue changes,
  prompt frames, endpoint responses, parse faults, schema faults, admissions,
  effects, observations, evidence, artifact operations, verification,
  completion, compaction, maintenance, turn budget, owner input wait, and
  blocked handoff.
- `RuntimeDecision`: mission, active mode, decision kind, admitted tools,
  blocked tools, exact next action or template, recommended actions, missing
  and existing evidence, completion state, recovery plan, compaction plan,
  maintenance plan, blocked-handoff plan, context package ids, prompt card data,
  persistence plan, authority fingerprint, and staleness fingerprint.
- `ToolAdmissionView`: immutable dispatch view derived from one persisted
  decision id.
- `RuntimeEffectCommand`: deterministic runtime-owned effects, including
  compaction, closed-idle wait, maintenance defer, blocked handoff recording,
  status refresh, and zero-content inspection tools when the decision says no
  model-authored semantic content is needed.

A model-call decision must not have an empty admitted tool set. Empty admitted
tool sets are allowed only for runtime compaction, closed idle, completed close,
or external owner wait.

## Implementation Task

Replace parallel authority paths with one transition kernel:

```text
build_runtime_snapshot(conn, daemon_state) -> RuntimeSnapshot
record_runtime_event(snapshot_id, event) -> RuntimeEventId
reduce(snapshot, event) -> RuntimeDecision
record_runtime_decision(snapshot_id, event_id, decision) -> RuntimeDecisionId
render_prompt_frame(decision_id) -> PromptFrame
admit_tool(decision_id, model_action, staleness_fingerprint) -> ToolAdmission
record_effect_observation(admission_id, observation) -> RuntimeEvent
```

`TurnAuthorityInput` may remain only as an adapter input while building the full
snapshot. It must not select a mission independently from `RuntimeSnapshot`.

## Inputs

- current `mode` reducer and daemon authority code.
- graph state and transition code.
- dispatch policy and refusal code.
- store authority snapshot, decision, admission, effect, prompt frame, graph,
  artifact, compaction, fault, and provider exchange tables.
- uploaded failure fixtures from `data/logs/current-model-run.md`.

## Outputs

- `RuntimeMission` selected by the priority table.
- data-first `RuntimeDecision` with decision kind and fingerprints.
- persisted snapshots, events, decisions, transitions, admissions, effects,
  observations, and prompt frames.
- prompt authority card rendered from the persisted decision.
- immutable admission view keyed by decision id and staleness fingerprint.
- stale-action refusal when queue, case, graph, active mode, compaction,
  recovery, artifact, evidence, maintenance, or prompt facts changed.

## Invariants

- One event emits one persisted decision.
- Graph policy is input, not a dispatch fallback authority.
- Prompt and dispatch share the same decision id.
- A tool-requiring prompt frame has a non-empty admitted tool set.
- Every refusal includes one valid next action from the dispatcher registry or a
  deterministic runtime effect command.
- Completion reads the central completion reducer on every close path.
- Compaction and maintenance are runtime-owned decisions, not prompt
  inventions.

## Failure Cases

- Maintenance and owner graph policy render together.
- A cached maintenance action runs after owner work appears.
- A cached write survives artifact cursor or compaction-pressure changes.
- A live prompt permits `<think>` output or replays invalid assistant history.
- Empty provider content with nonzero completion tokens loops as a parse fault.
- A recovery escape tool is hidden by old graph policy.
- `fs.batch_write` repeats a path-shaped schema fault without route change.
- A model-log export manifest lists files that are absent on disk.
- `agent.done` closes through a side path.

## Verification

- `cargo test -p lkjagent-runtime --test authority_reducer`
- `cargo test -p lkjagent-runtime --test turn_authority`
- `cargo test -p lkjagent-runtime --test turn_authority_runtime`
- `cargo test -p lkjagent-runtime --test authority_policy`
- `cargo test -p lkjagent-runtime --test authority_recovery_plan`
- `cargo test -p lkjagent-runtime --test recovery_controller`
- `cargo test -p lkjagent-runtime --test artifact_ledger_completion`
- `cargo test -p lkjagent-runtime --test compaction_snapshot`
- `cargo test -p lkjagent-tools --test graph_control_dispatch`
- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify` before final implementation claims.

## Status

partially implemented. Runtime mission selection, data-first decision records,
`RuntimeSnapshot.active_mode`, adapter-built turn snapshots, normalized
authority snapshot, event, decision, transition, effect, and admission store
APIs, dispatch admission views, stale maintenance-action refusal, central
completion reducer use, and prompt-card decision id and fingerprint rendering
exist. A standalone `kernel` module now defines pure snapshot, event, decision,
admission, effect, render, fault, and reducer records with tests for mission
priority and model-call admission invariants. Store ledgers include prompt
frame and observation rows, reopen lookup proof, and foreign-key proof that an
admission cannot exist without a decision. Adapter-valid authority decision rows
now store kernel-derived authority and staleness fingerprints, and authority
ledger events use the kernel event-kind taxonomy. The daemon records authority
prompt frames and effect observations for pending dispatches, writes kernel
mission and staleness fields into provider `authority.json`, and the kernel
admits `agent.done` only after evidence gaps are empty. Completion-ready active
graphs now shadow as kernel completion events. Authority rows record kernel
shadow mission fields from the persisted authority snapshot and event id, and
stale cached actions refuse when their prompt-frame head changes. The
snapshot adapter
computes staleness fingerprints from queue head and count, case, active mode,
graph, artifact cursor, fault, evidence, compaction, maintenance, and prompt facts;
rejects synthetic
active case ids; and ignores maintenance due state when owner work exists.
Kernel prompt rendering requires persisted event and decision ids, cites
authority and staleness fingerprints, preserves the admitted tool surface, and
renders concrete path-scoped `fs.batch_write` examples that parse. The old
turn-authority mission selector delegates shared facts to the kernel adapter.
Kernel
admission refuses stale cached maintenance actions, changed artifact-cursor
writes, compaction pressure writes, changed fault, evidence, maintenance, and
prompt-frame facts, blocked tools, completion requests, and retry-exhausted
repeated action fingerprints before dispatch. Daemon model-completion handling
can attach the persisted decision id, prompt frame id, and staleness fingerprint
to pending actions before dispatch, and persisted
pending actions refuse when the full staleness fingerprint changes. Prompt history
hygiene, provider empty-content anomaly handling, self-consistent export
manifests, and current-run replay fixtures now have focused coverage. Daemon
wiring through the new kernel, explicit triggering events on every path,
prompt-frame resume proof, maintenance preemption proof, route-wide admission
proof, and every close path remain open.
