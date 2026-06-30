# Content Atom Graph

## Purpose

Introduce general content atoms for long and structured work products.

## Status

done: shared content atom profiles, atom audit facts, generic-root conflict
blocking, completion refusals, focused tests, benchmark corpus, quiet verify,
and Docker verify passed

## Depends On

- [resolver-table-totality.md](resolver-table-totality.md)

## Files To Read

1. [Artifacts](../../architecture/artifacts/README.md)
2. [Content artifacts](../../architecture/artifacts/content-artifacts.md)
3. [Story profile](../../architecture/artifacts/story-profile.md)
4. [Manuscript lifecycle](../../architecture/artifacts/manuscript-lifecycle.md)
5. `crates/lkjagent-tools/src/artifact_next_story.rs`
6. `crates/lkjagent-tools/src/artifact_story_manuscript.rs`
7. `crates/lkjagent-runtime/src/kernel/obligation_facts.rs`

## Files To Touch

- `docs/architecture/artifacts/content-atoms.md`
- `docs/architecture/artifacts/deterministic-assembly.md`
- `docs/architecture/artifacts/README.md`
- `docs/_meta/catalog/architecture.toml`
- artifact ledger, `artifact.next`, readiness, and runtime fact modules
- focused tool, runtime, and benchmark tests

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-tools artifact_next
cargo test -p lkjagent-runtime content_atom
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Long work progresses through exact atom contracts.
- Content atoms are shared across story, report, documentation, and generic
  work-product profiles.
- Generic roots are blocked when owner profile facts name a root or target path.
- Audits update atom status, missing atom count, next atom, and readiness facts.
- Completion refuses while required atoms remain missing or weak.

## Must Not

- Do not ask the model for a giant unbounded artifact in one turn.
- Do not make story-specific atoms the only atom model.
- Do not satisfy readiness with README-only or owner-term-only leaves.
