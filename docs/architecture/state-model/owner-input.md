# Owner Input

## Purpose

This file defines owner messages as evidence for a task case.

## Contract

- Preserve raw owner text inside an objective envelope.
- Derive a normalized objective that is not a copy of the owner text.
- Extract inferred intents, non-goals, constraints, preferences, risks, open
  questions, candidate tracks, and selected primary track.
- The envelope is built before endpoint execution.

## Implementation Hooks

- Source: `crates/lkjagent-graph/src/case_objective.rs`
- Tests: `crates/lkjagent-graph/tests/state_tracks.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Raw owner text is treated as the operational task.
- Constraints are lost before graph planning.
- The case has no candidate tracks for recovery or verification work.

## Status

partially implemented
