# Recovery

## Purpose

This file owns bounded recovery after action parameter faults.

## Contract

- Parameter faults route to a specific recovery path.
- The next model-visible notice names valid tools and exact parameter names.
- `recover-params` admits only graph inspection, bounded path inspection, and
  owner ask tools.
- The same action text is never executed twice.
- The same failing tool class cannot loop without a strategy change.
- Recovery prefers graph inspection or bounded read-only tools before waiting.
- Payload faults prefer artifact planning, `artifact.next`, or bounded writes
  before another raw write.
- Rendered recovery examples must be admitted by the active mode and
  dispatcher.
- Recovery ladders are finite for parse, parameter, runtime, repeat, policy,
  payload, verification, completion, compaction, and maintenance faults.

## Implementation Hooks

- Source: `crates/lkjagent-runtime/src/recovery.rs`
- Source: `crates/lkjagent-runtime/src/step/fault_wait.rs`
- Source: `crates/lkjagent-graph/src/source_recovery.rs`
- Tests: `crates/lkjagent-runtime/tests/fault_wait.rs`
- Tests: `crates/lkjagent-graph/tests/graph.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- The daemon repeats the same invalid action three times.
- A parameter fault falls into a generic parse recovery state with no recipe.
- The owner is asked a question that the registry could answer.
- `recover-params` exposes mutation tools instead of schema and state tools.
- Payload-too-large retries one raw `fs.write`.
- Repeat protection emits the same `graph.state` action again.

## Status

partially implemented. Parameter recovery and bounded artifact next-batch
planning exist; full productive artifact escape coverage remains open.
