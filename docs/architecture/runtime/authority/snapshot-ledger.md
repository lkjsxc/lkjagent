# Snapshot Ledger

## Purpose

This file owns the durable snapshot shape consumed by the runtime authority reducer.

## Contract

A snapshot is the complete read-side view for one decision. It joins daemon, queue, case, graph, artifact, evidence,
fault, context, verification, maintenance, and observation facts before the reducer runs. Effects do not add hidden
policy facts after snapshot construction.

## Inputs

- queue state and active case envelope.
- active mission, active node, and owner objective.
- constraints, assumptions, risks, and success criteria.
- artifact, evidence, fault, verification, maintenance, and compaction ledgers.
- tool policy and context budget.
- last action, last observation, last successful observation, and blocked handoff.

## Outputs

- `RuntimeSnapshot` record passed to `RuntimeEvent` reduction.
- prompt-card fields copied from the resulting decision.
- fingerprint input for stale-action checks.

## Invariants

- Snapshot building is effectful, but the reducer receives immutable data.
- The snapshot records the current queue boundary and active case identity.
- Artifact facts name the current artifact id rather than a raw path only.
- Compaction and recovery facts are present before tool admission.
- Missing facts are represented as unknown or absent fields, not invented success.

## Failure Cases

- Prompt rendering uses generic artifact text because the snapshot lacks ledger fields.
- Dispatch accepts an action after the queue head changed.
- Completion infers success from missing-evidence length without artifact and verification facts.
- Hard compaction loses the last successful observation or batch cursor.

## Verification

- pure reducer snapshot tests for every mission.
- status rendering tests for latest snapshot fields.
- stale-action dispatch tests that rebuild and compare fingerprints.

## Status

design-only for the normalized ledger; current authority snapshots are partial.
