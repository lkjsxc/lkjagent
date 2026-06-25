# Turn Authority

## Purpose

Define the pure decision object that owns one turn before prompt rendering and
before tool dispatch.

## Decision Owner

`lkjagent-runtime` owns `TurnAuthority`. The graph proposes phase and evidence.
The model proposes an action. The dispatcher executes only after this object
admits the action. A pending action generated before runtime compaction remains
admissible when only compaction pressure and prompt-frame identity changed.

## Inputs

The reducer reads the durable case id, active objective, queue depth, graph
node, required evidence, artifact ledger, audit ledger, recovery state,
compaction pressure, maintenance state, verification state, and last action
fault.

## Output

The output names active mission, phase, node, missing evidence, admitted tools,
blocked tools, next valid actions, completion status, recovery status,
compaction status, maintenance status, and policy contradictions.

## Prohibited States

- Two missions are active for the same turn.
- Prompt text admits a tool that dispatch refuses.
- Dispatch recomputes a different authority than the prompt displayed.
- Completion closes through a path that skips this object.

## Fixture

`tool_admission_graph_plan_contradiction` proves graph suggestions cannot name
a blocked preferred action. `cookbook_missing_evidence` proves a scaffolded
artifact remains open when evidence is missing.

## Verification

Run `cargo test -p lkjagent-runtime authority_reducer` and
`cargo test -p lkjagent-tools effective_policy`.

## Status

design-only for fields not yet represented in the runtime snapshot.
