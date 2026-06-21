# Batch Cursors

## Purpose

Define durable progress for long artifact repair writes.

## Contract

Large artifact repair is split into bounded semantic batches. A cursor records
planned paths, completed paths, current index, retry counts, and fallback mode.
The active implementation stores the current root cursor in the SQLite state
table and advances across current weak paths before asking for audit or
focused reads.

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
- `fs.batch_write` parser accepts the documented canonical format and normalizes
  allowed variants.
- Schema repair examples are generated from the same schema as dispatch.
- Repeated batch syntax failure switches to one-file `fs.write` or a native
  deterministic writer.

## Canonical Batch Format

```text
<act>
<tool>fs.batch_write</tool>
<files>
path: some/path.md
content:
# Title

Body.

-- lkjagent-next-file --
path: other/path.md
content:
# Title

Body.
</files>
</act>
```

Rendered examples always use this shape. Normalization may accept `path:foo`,
`<path>foo</path>`, XML-ish accidental path wrappers, and extra blank lines
before `path:` when the result is unambiguous.

## Invariants

- A malformed giant write is not retried verbatim after a parse fault.
- Compaction resumes at the same path and section.
- Batch content is rendered through protocol schemas.
- Cursor progress is stored before the next endpoint turn can lose it.
- No repeated invalid `fs.batch_write` example is emitted twice as the only
  recovery path.

## Fixture

`parse_fault_unclosed_content` proves batch retry changes shape.
`uploaded-cookbook-turn-budget-handoff` proves handoff stores the cursor.
`compaction_resume_missing` proves cursor fields survive compaction.

## Verification

Run `cargo test -p lkjagent-tools artifact_next` and
`cargo test -p lkjagent-runtime compaction_snapshot`.

## Status

partially implemented for root-scoped artifact.next cursors.
