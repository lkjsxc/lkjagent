# Deep Redesign Compact Context

## Purpose

Make model-facing context small, authority-owned, and free of object-literal batch examples.

## Status

open

## Depends On

[deep-redesign-truth-sweep.md](deep-redesign-truth-sweep.md)

## Files To Read

- [../../architecture/protocol/action-format.md](../../architecture/protocol/action-format.md)
- [../../architecture/protocol/batch-write.md](../../architecture/protocol/batch-write.md)
- [../../architecture/protocol/compact-context.md](../../architecture/protocol/compact-context.md)

## Files To Touch

- `crates/lkjagent-runtime/src/prompt.rs`
- `crates/lkjagent-runtime/src/kernel/render.rs`
- `crates/lkjagent-protocol/src/parse.rs`
- `crates/lkjagent-tools/src/fs_batch/parse.rs`
- protocol, runtime, and tools tests

## Focused Gate

```sh
cargo test -p lkjagent-protocol
cargo test -p lkjagent-tools --test batch_write_formats
cargo test -p lkjagent-runtime --test kernel_prompt_render
```

## Acceptance

- Live prompts show only singular tag actions and line-protocol batch writes.
- Top-level JSON, line-action bodies, and JSON batch payloads are refused as live output.
- Prompt history never renders object-literal file batches as examples.

## Must Not

- Do not preserve object-literal prompt affordances.
