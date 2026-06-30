# CLI Contract Redesign

## Purpose

Document a coherent command tree and operator UX before CLI implementation
changes.

## Status

open

## Depends On

- [structural-truth-sweep.md](structural-truth-sweep.md)

## Files To Read

1. [Product CLI](../../product/cli.md)
2. [Product observability](../../product/observability.md)
3. [Running](../../operations/running.md)
4. [Token ledger](../../architecture/observability/token-ledger.md)
5. `crates/lkjagent-cli/src/args.rs`
6. `crates/lkjagent-cli/src/status.rs`
7. `crates/lkjagent-cli/src/console/render.rs`

## Files To Touch

- `docs/product/cli.md` or new `docs/product/cli/` tree
- `docs/product/README.md`
- `docs/product/observability.md`
- `docs/architecture/observability/README.md`
- catalog entries for added or moved docs
- `README.md` only when command examples become implemented

## Focused Gate

```sh
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
```

## Acceptance

- A coding agent can infer every command, group, argument, and output surface
  from the docs.
- The docs name cumulative token fields and unknown-field rendering.
- The docs distinguish implemented CLI behavior from the design target.
- Product boundaries remain local CLI, terminal console, store, and no web UI.

## Must Not

- Do not imply hidden commands.
- Do not document runtime MCP, product sub-agents, cron, or heartbeat schedules.
- Do not update root command examples as implemented until code lands.
