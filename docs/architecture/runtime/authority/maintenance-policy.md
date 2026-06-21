# Maintenance Policy

## Purpose

Define the runtime authority rules that make maintenance idle-only and bounded.

## Decision Owner

`lkjagent-runtime` owns maintenance eligibility. Store and memory helpers can
perform effects only after authority admits maintenance.

## Eligibility

Maintenance may run only when all conditions are true:

```text
queue empty
no active owner case
no pending verification
no active recovery
no incomplete compaction resume
maintenance budget available
```

An owner message, recovery fault, artifact gap, verification gap, or hard
compaction preempts maintenance immediately.

## Output

The output admits a bounded maintenance action, refuses maintenance, preempts
maintenance with owner or recovery work, or enters closed idle after a no-op.

## Digest Shape

```text
MaintenanceDigest
- directive
- result_kind
- normalized_query
- affected_records
- timestamp_bucket
```

Equivalent no-op digests are rejected before storage. A zero-change prune is a
no-op, not a real effect.

## Effect Rules

Maintenance effects are factual counts: rows pruned, rows merged, policy rows
changed, or no effect. Rows that only restate an empty queue are rejected
before storage. Maintenance cannot call owner completion tools.

## Prohibited States

- Maintenance runs while owner queue depth is non-zero.
- Maintenance runs while an active case is open.
- Maintenance claims an effect when no rows changed.
- Maintenance writes repeated low-value memory rows.
- Maintenance restarts immediately after a no-op.

## Mechanical Tests Required

- Owner task active returns a preempted decision.
- Repeated no-op writes no memory row.
- Maintenance completed moves to closed idle or cooldown.
- Maintenance cannot admit owner mutation or completion tools.

## Fixture

`maintenance_noop_claim` proves zero-change maintenance cannot claim success.
`uploaded-cookbook-maintenance-preemption` proves owner work preempts memory
maintenance.

## Verification

Run `cargo test -p lkjagent-runtime maintenance` and
`cargo test -p lkjagent-store memory_quality`.

## Status

partially implemented.
