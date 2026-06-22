# Action Fault Recovery

## Purpose

This file owns the work to make malformed but recoverable action parameters
produce deterministic normalization or actionable refusal.

## Contract

- Normalize safe aliases before hard refusal.
- Never invent required semantic content such as summaries or questions.
- Report exact expected parameter names and one valid action example.
- Record normalization as an observable recovery event.
- Route repeated parser-level parameter faults through `recover-params`, not
  generic parse recovery.
- Keep `recover-params` limited to `graph.state`, `fs.list`,
  `workspace.summary`, and `agent.ask`.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Source: `crates/lkjagent-runtime/src/recovery.rs`
- Source: `crates/lkjagent-runtime/src/step/fault_wait.rs`
- Source: `crates/lkjagent-graph/src/source_recovery.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Tests: `crates/lkjagent-runtime/tests/fault_wait.rs`
- Tests: `crates/lkjagent-graph/tests/graph.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- `graph.state` with a harmless location parameter loops on a parse notice.
- `doc.scaffold` receives `path` and fails instead of using `root`.
- The same invalid action is retried without a new recovery strategy.

## Status

partially implemented. Safe alias normalization and canonical examples exist
for covered cases. Dispatchable registry examples now parse, validate, and
reach tool routes in focused tests, while recovery-rendered examples still need
a single canonical registry proof across every refusal path.
