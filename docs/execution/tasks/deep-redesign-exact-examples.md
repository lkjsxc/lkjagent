# Deep Redesign Exact Examples

## Purpose

Render every model-facing example from registry-backed data and prove it round-trips.

## Status

open

## Depends On

[deep-redesign-compact-context.md](deep-redesign-compact-context.md)

## Files To Read

- [../../architecture/protocol/action-format.md](../../architecture/protocol/action-format.md)
- [../../architecture/runtime/authority/tool-admission.md](../../architecture/runtime/authority/tool-admission.md)

## Files To Touch

- `crates/lkjagent-runtime/src/kernel/render.rs`
- `crates/lkjagent-tools/src/dispatch/examples.rs`
- `crates/lkjagent-protocol/src/registry_render.rs`
- example round-trip tests

## Focused Gate

```sh
cargo test -p lkjagent-protocol
cargo test -p lkjagent-tools --test semantic_examples
cargo test -p lkjagent-runtime --test kernel_prompt_render
```

## Acceptance

- `graph.plan` examples include `objective`, `steps`, `reason`, and `checks` or `paths`.
- Every rendered example parses, validates, and is admitted for the active decision.

## Must Not

- Do not handwrite invalid examples in runtime modules.
