# Deep Redesign Runtime Authority

## Purpose

Make one persisted runtime decision the authority for prompt rendering and dispatch.

## Status

open

## Depends On

[deep-redesign-exact-examples.md](deep-redesign-exact-examples.md)

## Files To Read

- [../../architecture/runtime/authority/reducer.md](../../architecture/runtime/authority/reducer.md)
- [../../architecture/protocol/compact-context.md](../../architecture/protocol/compact-context.md)

## Files To Touch

- `crates/lkjagent-runtime/src/kernel/reduce.rs`
- `crates/lkjagent-runtime/src/kernel/admission.rs`
- `crates/lkjagent-runtime/src/kernel/next_action.rs`
- `crates/lkjagent-runtime/src/kernel/render.rs`
- runtime authority tests

## Focused Gate

```sh
cargo test -p lkjagent-runtime --test kernel_mission_matrix
cargo test -p lkjagent-runtime --test kernel_prompt_render
cargo test -p lkjagent-runtime --test turn_authority_runtime
```

## Acceptance

- Owner recovery outranks schema repair.
- A stored decision id appears in the compact prompt card.
- Maintenance cannot preempt active owner work.

## Must Not

- Do not let a refused stale action reach an adapter.
