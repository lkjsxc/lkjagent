# Batch Cursors

## Purpose

Define durable progress for long artifact repair writes.

## Contract

Large artifact repair is split into bounded semantic batches. Each batch cursor
names root, profile, path, section, missing requirements, retry count, and the
next write action shape.

## Inputs

The cursor reads audit weak paths, profile requirements, previous batch result,
parse faults, compaction snapshots, and line-limit constraints.

## Output

The output is next batch, re-audit, repair blocked with handoff, or complete
for the current weak-path list.

## Invariants

- A malformed giant write is not retried verbatim after a parse fault.
- Compaction resumes at the same path and section.
- Batch content is rendered through protocol schemas.
- Cursor progress is stored before the next endpoint turn can lose it.

## Fixture

`parse_fault_unclosed_content` proves batch retry changes shape.
`compaction_resume_missing` proves cursor fields survive compaction.

## Verification

Run `cargo test -p lkjagent-tools artifact_next` and
`cargo test -p lkjagent-runtime compaction_snapshot`.

## Status

design-only for durable store-backed cursors.
