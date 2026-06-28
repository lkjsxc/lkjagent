# Deep Redesign Output Budget

## Purpose

Set the default model output budget around 512 tokens and route oversize recovery to smaller actions.

## Status

done

## Depends On

[deep-redesign-compact-context.md](deep-redesign-compact-context.md)

## Files To Read

- [../../architecture/llm/output-budget.md](../../architecture/llm/output-budget.md)
- [../../architecture/llm/endpoint.md](../../architecture/llm/endpoint.md)
- [../../architecture/context/budgets.md](../../architecture/context/budgets.md)

## Files To Touch

- `crates/lkjagent-llm/src/wire.rs`
- `crates/lkjagent-llm/src/client.rs`
- `crates/lkjagent-context/src/budget.rs`
- runtime recovery tests as needed

## Focused Gate

```sh
cargo test -p lkjagent-llm
cargo test -p lkjagent-context
cargo test -p lkjagent-runtime --test budget_recovery
```

## Acceptance

- Default request `max_tokens` is compact.
- The prompt card names the output budget.
- `finish_reason=length` causes a smaller next action, not a same-shape retry.

## Must Not

- Do not solve long artifacts by raising the default output cap.
