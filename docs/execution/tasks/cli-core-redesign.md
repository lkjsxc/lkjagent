# CLI Core Redesign

## Purpose

Implement the documented command tree, help text, parsing, and dispatch shape.

## Status

open

## Depends On

- [cli-contract-redesign.md](cli-contract-redesign.md)
- [token-aggregate-ledger.md](token-aggregate-ledger.md)

## Files To Read

1. CLI contract docs created by [cli-contract-redesign.md](cli-contract-redesign.md)
2. `crates/lkjagent-cli/src/args.rs`
3. `crates/lkjagent-cli/src/args_help.rs`
4. `crates/lkjagent-cli/src/main.rs`
5. `crates/lkjagent-cli/src/lib.rs`
6. CLI tests under `crates/lkjagent-cli/tests/`

## Files To Touch

- `crates/lkjagent-cli/src/args.rs`
- `crates/lkjagent-cli/src/args_help.rs`
- `crates/lkjagent-cli/src/lib.rs`
- split command modules under `crates/lkjagent-cli/src/`
- CLI tests under `crates/lkjagent-cli/tests/`
- CLI docs changed by the contract task

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-cli
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Command metadata is plain data and renders all help text.
- `lkjagent --help`, `lkjagent help`, and group help work without loading
  runtime config.
- Missing and unknown commands print useful usage and nonzero exit codes.
- `--data DIR` works before or after the command.
- Parsing is pure and dispatch stays effectful at the edge.
- Snapshot outputs are stable in tests.

## Must Not

- Do not open network connections from read-only CLI commands.
- Do not add a socket, HTTP API, web UI, or product sub-agent.
- Do not hide parse errors behind generic failure text.
