# Compaction Snapshot Ledger

## Purpose

This file owns durable runtime compaction snapshots before and after hard compaction.

## Contract

Hard compaction is runtime-owned. Before truncating or rebuilding prompt context, the runtime persists enough state to
resume the original mission without asking the model to preserve memory. After rebuilding, it persists the new prompt
digest and resumed next action.

## Inputs

- active case, mission, node, and owner objective.
- required evidence and missing evidence.
- artifact ledger, weak paths, and batch cursor references.
- fault ledger, recovery route, and retry counts.
- last action, last observation, last successful observation, and admitted next tools.
- queue boundary, completion blockers, exact next action, and prompt digest.

## Outputs

- pre-compaction snapshot.
- compacted log record.
- post-rebuild snapshot linked to the preserved snapshot.
- status surface showing latest compaction resume fields.

## Invariants

- Hard pressure produces a runtime compaction decision before endpoint calls.
- The prompt never asks the model to run `memory.save` to preserve deterministic state.
- Snapshot history survives store reopen.
- Artifact weak paths and current cursor are present when an artifact mission is active.
- Post-compaction prompt cards resume the exact admitted action.

## Failure Cases

- Context pressure causes a model-authored memory action deadlock.
- Compaction loses the last successful observation.
- Batch repair restarts at the first path after compaction.
- Status shows no trace of the snapshot used to resume.

## Verification

- `cargo test -p lkjagent-runtime --test compaction_snapshot`
- store reopen tests for snapshot history.
- prompt-card tests for post-compaction exact action.

## Status

partially implemented. Current notices and graph compaction snapshot rows preserve
pre and post compaction fields, including artifact cursor and last observation
summary. Store reopen coverage and status rendering for the latest snapshot
remain open.
