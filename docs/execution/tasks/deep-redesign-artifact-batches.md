# Deep Redesign Artifact Batches

## Purpose

Repair large artifacts through small path-specific cursor batches.

## Status

open

## Depends On

[deep-redesign-short-paths.md](deep-redesign-short-paths.md)
[deep-redesign-output-budget.md](deep-redesign-output-budget.md)

## Files To Read

- [../../architecture/artifacts/write-batches.md](../../architecture/artifacts/write-batches.md)
- [../../architecture/artifacts/content-readiness.md](../../architecture/artifacts/content-readiness.md)

## Files To Touch

- `crates/lkjagent-tools/src/artifact_next.rs`
- `crates/lkjagent-tools/src/artifact_next_example.rs`
- `crates/lkjagent-tools/src/artifact_cursor_support.rs`
- artifact-next tests

## Focused Gate

```sh
cargo test -p lkjagent-tools --test artifact_next_long_novel
cargo test -p lkjagent-tools --test artifact_next_quality
cargo test -p lkjagent-tools --test artifact_next_ledger
```

## Acceptance

- Weak paths are repaired across multiple small turns.
- Generic scaffold phrases are refused before mutation.
- Cursor state resumes after compaction.

## Must Not

- Do not overwrite richer content with weaker generated content.
