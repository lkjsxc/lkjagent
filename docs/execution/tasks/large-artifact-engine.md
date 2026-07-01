# Large Artifact Engine

## Purpose

Implement the durable large-artifact engine: objective frames, profile library,
atom graph store rows, plan compiler, next-atom contracts, batch admission,
audit, assembly, readiness projection, prompts, CLI progress, benchmarks, and
smoke replay evidence.

## Status

done in this working tree with documentation, implementation, focused tests,
benchmark corpus, smoke replay, quiet verify, and final Docker verify evidence
recorded in the handoff.

## Depends On

- [../../current-state.md](../../current-state.md)
- [../../architecture/artifacts/large-artifact/README.md](../../architecture/artifacts/large-artifact/README.md)
- [../../operations/verification.md](../../operations/verification.md)

## Files To Read

- `AGENTS.md`
- `docs/current-state.md`
- `docs/architecture/artifacts/large-artifact/README.md`
- `tmp/lkjagent-redesign-report/README.md`

## Files To Touch

- `docs/architecture/artifacts/large-artifact/` (new)
- `crates/lkjagent-store/src/`
- `crates/lkjagent-tools/src/`
- `crates/lkjagent-runtime/src/`
- `crates/lkjagent-cli/src/`
- `crates/lkjagent-benchmark/src/`
- `crates/lkjagent-xtask/src/smoke.rs`

## Focused Gate

```sh
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
cargo test -p lkjagent-store artifact_graph
cargo test -p lkjagent-tools artifact_graph
cargo test -p lkjagent-runtime artifact_progress
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- smoke replay
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

The store holds artifact plans, atoms, edges, contracts, events, assembly runs,
and readiness projections. `artifact.plan`, `artifact.next`, `fs.batch_write`,
and `artifact.audit` use those rows. Status, watch, task show, benchmarks, and
smoke replay expose long-artifact progress from real files.

## Must Not

- Do not raise the global endpoint output cap.
- Do not count README, catalog, transcript, owner request, or manifest text as final content.
- Do not accept generic roots when the owner names a more specific root.
- Do not close manuscript work from story-bible-only or outline-only files.
