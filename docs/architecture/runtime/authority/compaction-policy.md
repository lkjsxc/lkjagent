# Compaction Policy

## Purpose

Define runtime-owned compaction authority for hard and soft context pressure.

## Decision Owner

`lkjagent-runtime` owns compaction policy. The context crate may compute
pressure, but runtime authority decides snapshot timing and resume state.

## Inputs

Compaction reads token pressure, active case, active mission, artifact state,
weak paths, missing evidence, last audit, last failed action, recovery class,
batch cursor, admitted tools, and verification state.

## Output

The output is snapshot now, continue without snapshot, or soft compaction
eligible. Snapshot output includes the resume card rendered to the next turn.

## Required Snapshot Fields

Snapshots include case id, objective, active mode, graph node, artifact root,
profile, evidence gaps, audit failures, weak paths, missing links,
scaffold-only paths, batch cursor, last observation, last failed action, fault
class, retry count, admitted tools, blocked tools, next action, verification
state, and queue depth.

## Prohibited States

- Hard pressure depends on a model-authored memory action.
- Snapshot loses artifact repair cursor or recovery class.
- Post-compaction authority is reconstructed from prose only.

## Fixture

`compaction_resume_missing` proves resume fields are mandatory.

## Verification

Run `cargo test -p lkjagent-runtime compaction_snapshot`.

## Status

partially implemented.
