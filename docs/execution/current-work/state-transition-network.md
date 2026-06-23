# State Transition Network

## Purpose

This task makes runtime authority, graph guidance, prompt rendering, dispatch, recovery, compaction, maintenance,
and completion use one persisted state-transition network.

## Contract

Implement the loop documented in
[../../architecture/state-graph/transition-network.md](../../architecture/state-graph/transition-network.md) and
[../../architecture/runtime/authority/decision-ledger.md](../../architecture/runtime/authority/decision-ledger.md):
`RuntimeSnapshot + RuntimeEvent -> RuntimeDecision`, then decision-derived
admission before effects.

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

`TurnAuthorityInput` may remain only as an adapter input while building the
full snapshot. It must not select a mission independently from
`RuntimeSnapshot`.

## Inputs

- current `mode` reducer and daemon authority code.
- graph state and transition code.
- dispatch policy and refusal code.
- store authority snapshot and graph state tables.
- uploaded failure fixtures.

## Outputs

- `RuntimeMission` selected from the snapshot by the priority table.
- data-first `RuntimeDecision` with decision kind and fingerprint.
- persisted snapshots, explicit events, decisions, transitions, and admissions.
- prompt authority card rendered from the persisted decision.
- immutable admission view keyed by decision id and staleness fingerprint.
- stale-action refusal when queue, compaction, recovery, artifact, or case facts
  changed.

## Invariants

- Graph policy is input, not a dispatch fallback authority.
- Prompt and dispatch share the same decision id.
- One event emits one decision.
- Every refusal includes one valid next action from the dispatcher registry.
- Completion reads the central completion reducer.

## Failure Cases

- Maintenance and owner graph policy render together.
- A cached maintenance action runs after owner work appears.
- A recovery escape tool is hidden by old graph policy.
- `agent.done` closes through a side path.

## Verification

- `cargo test -p lkjagent-runtime --test authority_reducer`
- `cargo test -p lkjagent-runtime --test turn_authority`
- `cargo test -p lkjagent-runtime --test turn_authority_runtime`
- `cargo test -p lkjagent-runtime --test authority_policy`
- `cargo test -p lkjagent-tools --test graph_control_dispatch`
- `cargo run -p lkjagent-xtask -- quiet verify`

## Status

partially implemented. Runtime mission selection, data-first decision records,
normalized authority event and decision persistence, admission persistence,
dispatch admission views, stale maintenance-action refusal, central completion
reducer use, and prompt-card decision id and fingerprint rendering exist. The
unified transition kernel, durable snapshot rows, explicit triggering events,
transition rows, recovery and compaction history, maintenance preemption proof,
and every close path remain open.
