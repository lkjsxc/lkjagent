# Dense Runtime State Network

## Purpose

Implement the active redesign slice that makes runtime authority dense, total,
and deterministic before asking the model for semantic content.

## Status

in progress: typed intent, dense rows, total resolver, runtime effects, and completion inputs pass; smoke pending

## Depends On

- [runtime-smoke-problem-sweep.md](runtime-smoke-problem-sweep.md)

## Files To Read

1. [../../current-state.md](../../current-state.md)
2. [../current-blockers.md](../current-blockers.md)
3. [../current-work/dense-runtime-state-network.md](../current-work/dense-runtime-state-network.md)
4. [Transition network](../../architecture/state-graph/transition-network.md)
5. [Transition kernel](../../architecture/runtime/authority/transition-kernel.md)
6. [Obligation network](../../architecture/runtime/obligation-network/README.md)
7. [Effect commands](../../architecture/runtime/authority/effect-commands.md)
8. [Completion policy](../../architecture/runtime/authority/completion-policy.md)
9. [Root identity](../../architecture/artifacts/root-identity.md)
10. [Story profile](../../architecture/artifacts/story-profile.md)

## Files To Touch

- `crates/lkjagent-graph/src/`
- `crates/lkjagent-store/src/runtime_authority/`
- `crates/lkjagent-runtime/src/kernel/`
- `crates/lkjagent-runtime/src/kernel_driver/`
- `crates/lkjagent-tools/src/artifact_*.rs`
- `crates/lkjagent-benchmark/src/`
- runtime, graph, store, tool, benchmark, and smoke tests
- docs named above

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-graph intent
cargo test -p lkjagent-store runtime_authority
cargo test -p lkjagent-runtime dense_runtime
cargo test -p lkjagent-runtime runtime_effect
cargo test -p lkjagent-runtime completion
cargo test -p lkjagent-tools artifact_readiness
cargo test -p lkjagent-tools artifact_next
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- `Compact Compass` routes as story artifact work.
- `iwanna` keeps an owner-title root.
- Missing-root facts cannot repeat same-root audit before write progress or a
  blocked handoff.
- Generic roots and examples are not used when a current root exists.
- A small story-bible seed cannot close large story work.
- Deterministic audits and inspections can bypass provider calls.
- Every close path uses one typed completion gate.
- Prompt and admission cite the same current decision and fingerprint.
- Docker Compose final verification passes.

## Must Not

- Do not add product MCP, runtime sub-agents, web UI, plan mode, heartbeat, or
  cron.
- Do not preserve fallback mission behavior after resolver planning.
- Do not let prompt text, graph evidence, or direct tool output bypass the
  persisted decision, audit-owned evidence, or typed completion gate.
- Do not claim runtime behavior without Docker Compose final verification.
