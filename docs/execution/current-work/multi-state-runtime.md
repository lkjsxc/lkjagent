# Multi State Runtime

## Purpose

This file owns the shift from a single brittle task state to ranked neutral
state tracks.

## Contract

- Treat owner input as evidence for a task case, not as the task itself.
- Maintain multiple candidate state tracks with neutral postures.
- Rank active tracks by intensity, recency, evidence gap, priority, and confidence.
- Display the top tracks in graph notices, status, and console.

## Implementation Hooks

- Source: `crates/lkjagent-graph/src/state_track.rs`
- Tests: `crates/lkjagent-graph/tests/state_tracks.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- Raw owner wording is copied into the operational objective.
- A recovery track never rises after repeated parameter faults.
- A closed or stale track dominates active execution work.

## Status

partially implemented
