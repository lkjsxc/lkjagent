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
No model-authored `memory.save` is required.

## Resume Shape

```text
CompactionResume
- case_id
- active_mission
- previous_mission
- active_node
- objective_summary
- artifact_id
- root
- batch_cursor
- missing_evidence
- last_failed_action
- last_failed_error
- last_successful_action
- last_successful_observation
- admitted_tools
- exact_next_action
- completion_blockers
- open_faults
```

## Invariants

- Hard pressure snapshots before owner intake, recovery prompts, maintenance,
  completion prompts, or checkpoint continuation prompts.
- Post-compaction state resumes the previous mission, not idle maintenance.
- Artifact repair resumes at the next unwritten weak path.
- Parse recovery resumes with previous mission escape tools preserved.
- Verification resumes with the failing command and next file if any.

## Prohibited States

- Hard pressure depends on a model-authored memory action.
- Snapshot loses artifact repair cursor or recovery class.
- Post-compaction authority is reconstructed from prose only.
- Compaction enters maintenance while owner evidence is missing.
- A turn-budget checkpoint asks the owner to continue because compaction ran.

## Fixture

`compaction_resume_missing` proves resume fields are mandatory. Uploaded
cookbook compaction fixtures prove batch cursor and readiness blockers survive.

## Verification

Run `cargo test -p lkjagent-runtime compaction_snapshot`.

## Status

partially implemented.
