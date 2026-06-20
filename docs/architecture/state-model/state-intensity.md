# State Intensity

## Purpose

This file owns ranked active-state scoring.

## Contract

- Rank score is a pure function of intensity, recency, evidence-gap urgency,
  owner priority, and confidence.
- Intensity contributes forty percent of the score.
- Recency contributes twenty-five percent of the score.
- Evidence-gap urgency contributes twenty percent of the score.
- Owner priority contributes ten percent of the score.
- Confidence contributes five percent of the score.

## Implementation Hooks

- Source: `crates/lkjagent-graph/src/state_track.rs`
- Tests: `crates/lkjagent-graph/tests/state_tracks.rs`
- Verification: `docker compose run --rm verify`

## Failure Modes

- A high-confidence but low-priority track dominates urgent work.
- Recovery never rises after a fresh fault.
- Ranking is embedded in rendering instead of pure code.

## Status

implemented
