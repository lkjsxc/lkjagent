# Token Aggregate Ledger

## Purpose

Implement cumulative token usage for latest, task, session, and all-time scopes.

## Status

done: aggregate store APIs and status-console token rendering implemented;
focused tests, quiet verify, and Docker verify passed

## Depends On

- [cli-contract-redesign.md](cli-contract-redesign.md)

## Files To Read

1. [Token ledger](../../architecture/observability/token-ledger.md)
2. [Status format](../../architecture/observability/status-format.md)
3. [Product observability](../../product/observability.md)
4. `crates/lkjagent-store/src/token_usage.rs`
5. `crates/lkjagent-store/tests/token_usage.rs`
6. `crates/lkjagent-cli/src/accounting.rs`
7. `crates/lkjagent-cli/src/status.rs`
8. `crates/lkjagent-cli/tests/status.rs`

## Files To Touch

- `docs/architecture/observability/token-ledger.md`
- optional `docs/architecture/observability/token-aggregates.md`
- `crates/lkjagent-store/src/token_usage.rs`
- `crates/lkjagent-store/tests/token_usage.rs`
- `crates/lkjagent-cli/src/accounting.rs`
- `crates/lkjagent-cli/src/status*.rs`
- `crates/lkjagent-cli/tests/status.rs`
- `crates/lkjagent-cli/tests/console_render.rs`

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-store token_usage
cargo test -p lkjagent-cli status
cargo test -p lkjagent-cli console
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Aggregate structs return known sums and unknown counts for input, output,
  cached input, and total tokens.
- Unknown fields stay visibly unknown; they are not added as zero.
- Cache ratio is rendered only when the denominator is known and nonzero.
- Status and console render the same aggregate facts for the same store.
- Tests cover multiple usage rows, missing fields, and task filters.

## Must Not

- Do not fabricate token counts when the provider omits a field.
- Do not compute cache ratios from partial data.
- Do not make CLI output depend on wall-clock timing in tests.
