# Deep Redesign Truth Sweep

## Purpose

Reconcile the contract with checked-in failure evidence before code changes.

## Status

open

## Depends On

None.

## Files To Read

- [../../current-state.md](../../current-state.md)
- [../current-blockers.md](../current-blockers.md)
- [../../evaluation/uploaded-run-fixtures.md](../../evaluation/uploaded-run-fixtures.md)
- [../../../data/logs/current-model-run.md](../../../data/logs/current-model-run.md)

## Files To Touch

- `docs/current-state.md`
- `docs/execution/current-blockers.md`
- `docs/evaluation/uploaded-run-fixtures.md`
- `crates/lkjagent-benchmark/src/tasks/owner_long_novel.rs`
- `crates/lkjagent-benchmark/src/judges/long_novel.rs`
- checked-in fixture tests as needed

## Focused Gate

```sh
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- benchmark check-corpus
```

## Acceptance

- The checked-in active run is named as failure evidence.
- Long-novel fixtures use structured-settings wording or root-independent behavior checks.
- Corpus checks catch stale long-novel root assertions.

## Must Not

- Do not claim a fresh smoke run without running one.
