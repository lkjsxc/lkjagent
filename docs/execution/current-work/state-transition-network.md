# State Transition Network

## Purpose

This task makes runtime authority, graph guidance, prompt rendering, dispatch, recovery, compaction, maintenance,
and completion use one persisted state-transition network.

## Contract

Implement the loop documented in
[../../architecture/state-graph/transition-network.md](../../architecture/state-graph/transition-network.md) and
[../../architecture/runtime/authority/decision-ledger.md](../../architecture/runtime/authority/decision-ledger.md):
`RuntimeSnapshot + RuntimeEvent -> RuntimeDecision`, then decision-derived admission before effects.

## Inputs

- current `mode` reducer and daemon authority code.
- graph state and transition code.
- dispatch policy and refusal code.
- store authority snapshot and graph state tables.
- uploaded failure fixtures.

## Outputs

- `RuntimeMission` as the reducer mission enum.
- data-first `RuntimeDecision` with decision kind and fingerprint.
- persisted events, decisions, and admissions.
- prompt authority card rendered from the persisted decision.
- stale-action refusal when queue, compaction, recovery, or case facts changed.

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
- `cargo test -p lkjagent-runtime --test authority_policy`
- `cargo test -p lkjagent-tools --test graph_control_dispatch`
- `cargo run -p lkjagent-xtask -- quiet verify`

## Status

open.
