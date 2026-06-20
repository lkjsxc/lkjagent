# Multi State

## Purpose

This file defines neutral state tracks for concurrent task interpretations.

## Contract

- A task case keeps multiple `StateTrack` records.
- Postures are Exploring, Structuring, Implementing, Verifying, Recovering,
  Waiting, Maintaining, and Closing.
- Each track stores intensity, confidence, phase, active node, evidence gaps,
  next affordances, risks, and last update turn.
- Status and graph notices show the top ranked active tracks.

## Implementation Hooks

- Source: `crates/lkjagent-graph/src/state_track.rs`
- Tests: `crates/lkjagent-graph/tests/state_tracks.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- One stale track hides an urgent recovery track.
- The active display shows only a single brittle state.
- Closed tracks rank above live execution tracks.

## Status

partially implemented
