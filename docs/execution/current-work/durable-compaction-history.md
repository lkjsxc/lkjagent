# Durable Compaction History

## Purpose

This task persists rich compaction snapshots that let the runtime resume owner, artifact, and recovery work.

## Contract

Hard compaction emits a runtime decision, writes a pre-compaction snapshot, compacts the log, rebuilds prompt context,
writes a post-rebuild snapshot, and resumes the original mission with the same next admitted action. The model is not
asked to preserve deterministic state.

## Inputs

- context compaction ledger contract.
- daemon compaction support code.
- authority compaction policy.
- store schema for existing compaction notices.
- CLI status and model-log surfaces.

## Outputs

- runtime compaction snapshot table or enriched existing table.
- pre and post snapshot records.
- artifact weak path, batch cursor, recovery route, missing evidence, and observation fields.
- status rendering for latest snapshot.

## Invariants

- Hard pressure produces compaction before endpoint work.
- Snapshot history survives store reopen.
- Last action, last observation, and last successful observation are preserved.
- Batch cursor id and current path are preserved.
- Post-compaction prompt card resumes the exact admitted action.

## Failure Cases

- Compaction deadlocks on `memory.save`.
- Artifact repair restarts or loses weak paths after compaction.
- Recovery loses the last failed action fingerprint.
- Status cannot show the snapshot used to resume.

## Verification

- `cargo test -p lkjagent-runtime --test compaction_snapshot`
- store reopen tests for snapshot history.
- `cargo test -p lkjagent-runtime --test recursive_guard`

## Status

implemented. Runtime hard compaction writes pre and post rows with recovery,
artifact cursor, missing evidence, exact next action, and last successful
observation summaries. Store reopen coverage, CLI status rendering, kernel
effect coverage, and prompt-frame resume proof show that post-compaction work
resumes the same artifact repair action when no stronger facts changed.
