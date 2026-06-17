# Queue CLI

## Purpose

Implement lkjagent-cli: the single binary with run, send, status, log, and
the read-only memory and skills commands, all speaking to the store.

## Status

done

## Depends On

[agent-loop.md](agent-loop.md) for run; store APIs for everything else.

## Files To Read

- [../../product/cli.md](../../product/cli.md)
- [../../product/queue.md](../../product/queue.md)
- [../../product/observability.md](../../product/observability.md)
- [../../operations/running.md](../../operations/running.md)

## Files To Touch

- crates/lkjagent-cli/src/: main.rs (argument dispatch, no logic), one
  module per command, config.rs (lkjagent.toml reading, first-start
  default writing per the running contract).
- crates/lkjagent-cli/tests/: command tests against a temp store; config
  first-start behavior.

## Focused Gate

```sh
cargo test -p lkjagent-cli
cargo clippy -p lkjagent-cli -- -D warnings
cargo run -p lkjagent-xtask -- quiet verify
```

## Acceptance

- send appends through lkjagent-store, writes a queue_mutation event with
  reason owner-send, and prints the queue id; a daemonless send still
  persists.
- status prints daemon state, queue depth, open task, turns, context
  ledger numbers, and last compaction, one fact per line.
- log renders the documented compact form; --follow tails; --full prints
  whole payloads.
- run starts the daemon per the startup order, refuses a second instance,
  and exits cleanly on SIGTERM mid-task, all integration-tested.
- First start without a config writes the commented default and exits
  asking for endpoint.model, exactly as the running contract states.
- Blocker row 10 done; product area statuses move in the ledger.

## Must Not

- Do not add commands beyond the documented six.
- Do not open any network connection from CLI commands; the store is the
  only interface.
- Do not decorate output; plain text, one fact per line, exit codes per
  the product contract.
