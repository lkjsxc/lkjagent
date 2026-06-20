# Transition Quality

## Purpose

This file defines scoring and records for task-state transitions.

## Contract

- Every transition records legality, reason, evidence delta, context delta,
  risk delta, repetition penalty, and expected next observation.
- Transitions that increase evidence or reduce uncertainty outrank no-op loops.
- Completion is illegal while required evidence gaps remain.
- Asking the owner is legal only when local context cannot safely decide.

## Implementation Hooks

- Source: `crates/lkjagent-graph/src/transition.rs`
- Tests: `crates/lkjagent-graph/tests/graph.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- A transition repeats without evidence gain.
- Context pressure is ignored until endpoint failure.
- Waiting is selected while a safe local inspection remains.

## Status

partially implemented
