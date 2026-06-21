# Mode Priority

## Purpose

Define the deterministic priority order that selects exactly one active mission.

## Decision Owner

`lkjagent-runtime` owns priority selection. Graph policy, endpoint output,
maintenance ticks, verification requests, and context pressure are inputs.

## Inputs

Priority reads queue state, active owner case, last fault, artifact readiness,
verification state, compaction pressure, and maintenance state.

## Output

The selector emits `ActiveModeDecision` with mission, reason, required resume
data, admitted tool classes, and the next valid action class.

## Priority

1. Hard compaction snapshot.
2. Owner task intake.
3. Fault recovery.
4. Artifact repair.
5. Verification.
6. Soft compaction.
7. Idle maintenance.
8. Closed idle.

## Prohibited States

- Maintenance continues after owner queue depth becomes non-zero.
- Verification runs before the artifact exists or readiness is plausible.
- Hard compaction asks the model to preserve state through `memory.save`.
- Recovery yields to normal progress while the previous fault is unresolved.

## Fixture

`maintenance_noop_claim` proves idle work cannot outrank owner work.
`compaction_resume_missing` proves hard pressure must snapshot first.

## Verification

Run `cargo test -p lkjagent-runtime active_mode`.

## Status

Implemented for hard compaction priority, owner-over-maintenance priority, and
recovery-over-maintenance priority. Artifact repair and verification priority
remain partial until artifact case state drives mode selection directly.
