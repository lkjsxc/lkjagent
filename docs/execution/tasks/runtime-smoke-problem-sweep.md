# Runtime Smoke Problem Sweep

## Purpose

Fix the live-smoke problems recorded in
[../current-work/runtime-smoke-problems.md](../current-work/runtime-smoke-problems.md).

## Status

open

## Depends On

- [obligation-network-redesign.md](obligation-network-redesign.md)

## Files To Read

1. [../../current-state.md](../../current-state.md)
2. [../current-blockers.md](../current-blockers.md)
3. [../current-work/runtime-smoke-problems.md](../current-work/runtime-smoke-problems.md)
4. [../../architecture/runtime/obligation-network/README.md](../../architecture/runtime/obligation-network/README.md)
5. [../../architecture/artifacts/root-repair.md](../../architecture/artifacts/root-repair.md)
6. `crates/lkjagent-graph/src/classify.rs`
7. `crates/lkjagent-graph/src/classify_signals.rs`
8. `crates/lkjagent-graph/src/classify_artifact.rs`
9. `crates/lkjagent-runtime/src/kernel/next_action.rs`
10. `crates/lkjagent-runtime/src/kernel/resolver.rs`

## Files To Touch

- `crates/lkjagent-graph/src/classify.rs`
- `crates/lkjagent-graph/src/classify_signals.rs`
- `crates/lkjagent-graph/src/classify_artifact.rs`
- `crates/lkjagent-runtime/src/kernel/`
- `crates/lkjagent-runtime/tests/`
- benchmark or replay fixtures as needed
- docs/current-state.md
- docs/execution/current-blockers.md

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-graph compact
cargo test -p lkjagent-runtime obligation_network
cargo test -p lkjagent-runtime artifact_completion_gate
cargo run -p lkjagent-xtask -- benchmark check-corpus
```

## Acceptance

- A long-novel title containing `Compact` is classified as artifact work, not
  compaction work.
- A named novel root preserves the owner title instead of `stories/novel-named`.
- Missing-root facts force a root identity contract before another same-root
  audit unless a write or handoff changes the progress key.
- Recovery examples never use `stories/example-story` when a current artifact
  root exists.
- Long-novel completion requires scale-appropriate story readiness, not only a
  small seed artifact.
- Focused tests and a fresh clean-data smoke prove the route closes without the
  observed false-close or noisy-loop behavior.

## Must Not

- Do not claim the current smoke is sufficient proof of long-novel completion.
- Do not preserve title-word `compact` as a higher priority than long-content
  intent.
- Do not allow direct graph evidence to satisfy audit-owned requirements.
- Do not add placeholder roots or scaffold writers.
