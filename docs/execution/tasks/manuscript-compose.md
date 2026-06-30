# Manuscript Compose

## Purpose

Make long manuscript tasks finish through daemon-owned content atoms and
deterministic assembly.

## Status

open

## Depends On

- [content-atom-graph.md](content-atom-graph.md)

## Files To Read

1. [Manuscript lifecycle](../../architecture/artifacts/manuscript-lifecycle.md)
2. [Story profile](../../architecture/artifacts/story-profile.md)
3. [Story manuscript generation gap](../current-work/story-manuscript-generation-gap.md)
4. `crates/lkjagent-tools/src/artifact_story_manuscript.rs`
5. `crates/lkjagent-tools/src/artifact_next_story.rs`
6. runtime story manuscript tests

## Files To Touch

- `docs/architecture/artifacts/manuscript-lifecycle.md`
- `docs/architecture/artifacts/content-atoms.md`
- `docs/architecture/artifacts/deterministic-assembly.md`
- manuscript planning, assembly, readiness, and runtime fact modules
- story manuscript benchmark fixtures
- focused tool and runtime tests

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-tools manuscript
cargo test -p lkjagent-runtime story_manuscript
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Exact chapter path requests create the exact final path through the daemon.
- Long manuscript tasks write prose atoms before optional lore after identity.
- Deterministic assembly creates chapter files from approved scene atoms when
  needed.
- Word counts come from real manuscript files only.
- Completion closes only after final artifact readiness and central gate pass.
- Recovery shrinks to smaller scene atoms or blocks with exact remaining paths.

## Must Not

- Do not claim direct endpoint fallback output as daemon completion.
- Do not satisfy manuscript readiness with story-bible files.
- Do not close scene-only output until required chapter assembly exists.
