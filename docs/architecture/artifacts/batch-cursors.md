# Batch Cursors

## Purpose

Define durable progress for long artifact repair writes.

## Contract

Large artifact repair is split into bounded semantic batches. A cursor records
planned paths, completed paths, current index, retry counts, and fallback mode.
The active implementation stores the current root cursor in the SQLite state
table and writes normalized artifact batch cursor rows for emitted
`artifact.next` batches. Successful `fs.write` and `fs.batch_write` calls mark
matching planned cursor paths complete. It advances across current weak paths
before asking for audit or focused reads.

## Cursor Shape

```text
BatchCursor
- artifact_id
- root
- batch_id
- planned_paths
- completed_paths
- failed_paths
- current_index
- last_valid_example
- retry_count_by_fault
- fallback_mode
```

## Rules

- Large raw write payloads are refused before endpoint exhaustion when the risk
  is predictable.
- Payload overflow moves to `BatchWriteRecovery`.
- `artifact.next` emits bounded batches and persists the cursor.
- `fs.batch_write` parser accepts the documented canonical line protocol and
  normalizes allowed variants.
- Child `<file>` tags inside `<files>` are a schema fault and do not advance the
  cursor.
- Schema repair examples are generated from the same schema as dispatch.
- Repeated child-tag or batch syntax failure switches to `artifact.next`,
  `graph.state`, deterministic inspection, one-file `fs.write`, or blocked
  handoff with exact weak paths.

## Canonical Batch Format

```text
<action>
<tool>fs.batch_write</tool>
<files>
path: stories/chronos-fracture/setting/timeline.md
content:
# Timeline

Chronos Fracture begins with a failed archive experiment.

-- lkjagent-next-file --
path: stories/chronos-fracture/setting/technology.md
content:
# Technology Rules

Time lenses expose causality debt instead of changing the past directly.
</files>
</action>
```

Rendered examples always use this shape. Prompt-facing examples never use
nested `<file>` child tags. Normalization may accept `path:foo`,
`<path>foo</path>`, tag-like accidental path wrappers, and extra blank lines
before `path:` when the result is unambiguous.

## Invariants

- A malformed giant write is not retried verbatim after a parse fault.
- Compaction resumes at the same path and section.
- Batch content is rendered through protocol schemas.
- Cursor progress is stored before the next endpoint turn can lose it.
- Failed `fs.batch_write` observations leave the cursor on the same weak path.
- No repeated invalid `fs.batch_write` example is emitted twice as the only
  recovery path.

## Fixture

`parse_fault_unclosed_content` proves batch retry changes shape.
`uploaded-cookbook-turn-budget-handoff` proves handoff stores the cursor.
`compaction_resume_missing` proves cursor fields survive compaction.

## Verification

Run `cargo test -p lkjagent-store --test artifact_cursor`,
`cargo test -p lkjagent-tools --test artifact_next_ledger`,
`cargo test -p lkjagent-tools --test artifact_write_ledger`,
`cargo test -p lkjagent-tools artifact_next`, and
`cargo test -p lkjagent-runtime compaction_snapshot`.

## Status

partially implemented for root-scoped `artifact.next` cursors, normalized
batch cursor rows, and successful write completion marking. Failed write paths
are not yet recorded as failed cursor paths.
