# lkjagent

## Purpose

lkjagent is a continuously running personal agent for one owner, one local LLM,
one workspace, and one SQLite store. The daemon turns owner messages into typed
plans, executes steps, verifies results with deterministic checks, and reports
honestly.

## Product Shape

- The harness owns control flow, retries, paths, and completion checks.
- The model authors bounded content inside a step-specific envelope.
- Long artifacts are first-class: manuscripts, document trees, reports, and
  other structured outputs.
- The daemon runs inside Docker Compose; the container boundary is the safety
  model.

## Common Commands

```sh
cargo run -p lkjagent-app -- run
cargo run -p lkjagent-app -- send "Create a document tree about ..."
cargo run -p lkjagent-app -- status
cargo run -p lkjagent-app -- task list
cargo run -p lkjagent-app -- log --limit 20
docker compose run --rm verify
```

Some commands are still implementation gaps. [docs/current-state.md](docs/current-state.md)
is the ledger for what is implemented now and what remains open.

## Read Order

1. [docs/current-state.md](docs/current-state.md)
2. [docs/vision/README.md](docs/vision/README.md)
3. [docs/product/README.md](docs/product/README.md)
4. [docs/agent/README.md](docs/agent/README.md)
5. [docs/operations/verification.md](docs/operations/verification.md)

## Repository Rules

- Documentation is the implementation contract.
- Every authored file stays at or below 200 lines.
- Completion is checks-gated and engine-computed.
- No fake success, placeholders, or unrun verification claims.
- Commit small slices with honest `Tested` and `Not-tested` trailers.
