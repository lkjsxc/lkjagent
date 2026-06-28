# Deep Redesign Completion Maintenance

## Purpose

Centralize completion and no-op maintenance in the kernel.

## Status

open

## Depends On

[deep-redesign-runtime-authority.md](deep-redesign-runtime-authority.md)
[deep-redesign-artifact-batches.md](deep-redesign-artifact-batches.md)

## Files To Read

- [../../architecture/runtime/authority/completion-policy.md](../../architecture/runtime/authority/completion-policy.md)
- [../../architecture/artifacts/completion-gates.md](../../architecture/artifacts/completion-gates.md)

## Files To Touch

- `crates/lkjagent-runtime/src/kernel/reduce.rs`
- `crates/lkjagent-runtime/src/kernel/effect.rs`
- `crates/lkjagent-tools/src/control.rs`
- `crates/lkjagent-store/src/artifact_ledger.rs`
- completion and maintenance tests

## Focused Gate

```sh
cargo test -p lkjagent-runtime --test kernel_completion
cargo test -p lkjagent-runtime --test maintenance_gate
cargo test -p lkjagent-tools --test completion_guard
```

## Acceptance

- `agent.done` cannot close scaffold-only artifacts.
- No-op maintenance records cooldown and does not churn the endpoint.

## Must Not

- Do not ask the owner during idle maintenance without external missing input.
