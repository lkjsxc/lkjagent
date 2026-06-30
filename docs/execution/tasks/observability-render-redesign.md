# Observability Render Redesign

## Purpose

Make status, log, and console output share typed decks that expose operator
state clearly.

## Status

open

## Depends On

- [cli-core-redesign.md](cli-core-redesign.md)
- [token-aggregate-ledger.md](token-aggregate-ledger.md)

## Files To Read

1. [Product observability](../../product/observability.md)
2. [Status format](../../architecture/observability/status-format.md)
3. [Console deck](../../architecture/observability/console-deck.md)
4. [Provider exchange log](../../architecture/observability/provider-exchange-log.md)
5. `crates/lkjagent-cli/src/status.rs`
6. `crates/lkjagent-cli/src/log.rs`
7. `crates/lkjagent-cli/src/console/render.rs`

## Files To Touch

- `docs/product/observability.md`
- `docs/architecture/observability/status-format.md`
- `docs/architecture/observability/console-deck.md`
- `crates/lkjagent-cli/src/status*.rs`
- `crates/lkjagent-cli/src/log*.rs`
- `crates/lkjagent-cli/src/console/*.rs`
- CLI render tests

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-cli status
cargo test -p lkjagent-cli log
cargo test -p lkjagent-cli console
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Status shows task, authority, artifact, context, token, model-log, and next
  action sections with stable keys.
- Log output has bounded compact and full modes with exact ordering.
- Console uses the same deck facts as status and preserves owner input across
  redraws.
- Tests assert exact output for representative store states.

## Must Not

- Do not make observation mutate daemon state.
- Do not render missing token fields as zero.
- Do not add metrics endpoints or log shipping.
