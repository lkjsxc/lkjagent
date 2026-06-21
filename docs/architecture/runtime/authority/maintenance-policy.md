# Maintenance Policy

## Purpose

Define the runtime authority rules that make maintenance idle-only and bounded.

## Decision Owner

`lkjagent-runtime` owns maintenance eligibility. Store and memory helpers can
perform effects only after authority admits maintenance.

## Inputs

Maintenance policy reads queue depth, active case state, recovery state,
artifact gaps, verification state, compaction pressure, maintenance budget,
and last maintenance effect.

## Output

The output admits a bounded maintenance action, refuses maintenance, or
preempts maintenance with the owner or recovery mission.

## Effect Rules

Maintenance effects are factual counts: rows pruned, rows merged, policy rows
changed, or no effect. A zero-change prune is a no-op, not a real effect.
Rows that only restate an empty queue are rejected before storage.

## Prohibited States

- Maintenance runs while owner queue depth is non-zero.
- Maintenance claims an effect when no rows changed.
- Maintenance writes repeated low-value memory rows.
- Maintenance calls owner completion tools.

## Fixture

`maintenance_noop_claim` proves zero-change maintenance cannot claim success.

## Verification

Run `cargo test -p lkjagent-runtime maintenance` and
`cargo test -p lkjagent-store memory_quality`.

## Status

partially implemented.
