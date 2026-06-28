# Deep Redesign Short Paths

## Purpose

Replace owner-sentence root slugs with short semantic artifact aliases.

## Status

open

## Depends On

[deep-redesign-truth-sweep.md](deep-redesign-truth-sweep.md)

## Files To Read

- [../../architecture/artifacts/path-aliases.md](../../architecture/artifacts/path-aliases.md)
- [../../architecture/document-structure/naming.md](../../architecture/document-structure/naming.md)

## Files To Touch

- `crates/lkjagent-graph/src/classify_artifact.rs`
- `crates/lkjagent-tools/src/artifact.rs`
- graph and tool tests

## Focused Gate

```sh
cargo test -p lkjagent-graph
cargo test -p lkjagent-tools --test artifact_short_paths
```

## Acceptance

- The long-novel objective maps to `stories/novel`.
- Path segments stay short.
- Artifact metadata preserves the full owner wording.

## Must Not

- Do not repeat the full owner sentence in path segments.
