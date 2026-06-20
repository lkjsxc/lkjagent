# Action Fault Recovery

## Purpose

This file owns the work to make malformed but recoverable action parameters
produce deterministic normalization or actionable refusal.

## Contract

- Normalize safe aliases before hard refusal.
- Never invent required semantic content such as summaries or questions.
- Report exact expected parameter names and one valid action example.
- Record normalization as an observable recovery event.

## Implementation Hooks

- Source: `crates/lkjagent-tools/src/dispatch/validate.rs`
- Tests: `crates/lkjagent-tools/tests/graph_control_dispatch.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- `graph.state` with a harmless location parameter loops on a parse notice.
- `doc.scaffold` receives `path` and fails instead of using `root`.
- The same invalid action is retried without a new recovery strategy.

## Status

not implemented
