# Story Manuscript Generation Gap

## Purpose

Make lkjagent write requested story manuscript prose at chapter scale instead
of stopping at a small story-bible scaffold.

## Status

open: live novel smoke created story-bible files but no requested manuscript
chapter file and far below the 10,000 word target

## Depends On

- [dense-runtime-state-network.md](dense-runtime-state-network.md)

## Files To Read

1. [../../current-state.md](../../current-state.md)
2. [../current-blockers.md](../current-blockers.md)
3. [../current-work/story-manuscript-generation-gap.md](../current-work/story-manuscript-generation-gap.md)
4. [Artifact story profile](../../architecture/artifacts/story-profile.md)
5. [Root identity](../../architecture/artifacts/root-identity.md)
6. [Batch cursors](../../architecture/artifacts/batch-cursors.md)
7. [Obligation facts](../../architecture/runtime/obligation-network/facts.md)
8. [Obligation resolvers](../../architecture/runtime/obligation-network/resolvers.md)
9. [Completion policy](../../architecture/runtime/authority/completion-policy.md)

## Files To Touch

- `crates/lkjagent-graph/src/classify_profile.rs`
- `crates/lkjagent-graph/src/classify_artifact.rs`
- `crates/lkjagent-tools/src/artifact_next_story.rs`
- `crates/lkjagent-tools/src/artifact_readiness_story.rs`
- `crates/lkjagent-runtime/src/kernel/obligation_facts.rs`
- `crates/lkjagent-runtime/src/kernel/resolver.rs`
- `crates/lkjagent-benchmark/src/`
- focused graph, tool, runtime, and benchmark tests
- docs named above

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-graph story
cargo test -p lkjagent-tools artifact_next_story
cargo test -p lkjagent-tools artifact_readiness_story
cargo test -p lkjagent-runtime story
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- A direct request for
  `stories/the-bell-rings-twice/manuscript/chapter-01.md` writes that path and
  does not create `structured-output`.
- A 10,000 word high-school romance task records manuscript target words,
  chapter count, next manuscript path, and manuscript word progress as typed
  facts.
- `artifact.next` emits chapter write contracts before optional lore files when
  manuscript prose is the owner objective.
- Completion is blocked until actual chapter prose reaches the requested scale
  or a clean blocked handoff names the missing manuscript paths.
- Provider max-token and reasoning-only anomalies route to smaller manuscript
  write contracts rather than repeating story-bible repair.
- Fresh smoke evidence shows at least 8,500 English words across chapter files
  for `The Bell Rings Twice` or a clean blocked handoff that preserves the exact
  next chapter path.
- Docker Compose final verification passes.

## Must Not

- Do not claim a 10,000 word route from story-bible files alone.
- Do not satisfy manuscript readiness with README, cast, setting, or outline
  files.
- Do not route explicit story manuscript paths to generic counted-document
  scaffold output.
- Do not add product MCP, runtime sub-agents, web UI, heartbeat, or cron.
- Do not close the blocker from prompt wording alone; require code, tests, and
  live evidence.
