# lkjagent

## Purpose

lkjagent is a continuously running personal agent for one owner, one local LLM,
one workspace, and one SQLite store. The daemon is moving toward a durable
state-ledger runtime where owner messages become cases, events, state cells,
runtime decisions, prompt frames, tool admissions, effects, observations, checks,
and honest reports.

## Product Shape

- Durable rows and persisted runtime decisions own control flow, retries, paths,
  tool admission, context selection, and completion checks.
- The active state vector determines output grammar and model-visible tools.
- The model authors bounded content or requests an operation exposed by the
  current decision.
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
cargo run -p lkjagent-app -- watch
docker compose run --rm verify
```

Some commands are still implementation gaps. [docs/current-state.md](docs/current-state.md)
is the ledger for what is implemented now and what remains open.

## Read Order

1. [docs/current-state.md](docs/current-state.md)
2. [docs/vision/README.md](docs/vision/README.md)
3. [docs/state/README.md](docs/state/README.md)
4. [docs/runtime/README.md](docs/runtime/README.md)
5. [docs/product/README.md](docs/product/README.md)
6. [docs/workspace/README.md](docs/workspace/README.md)
7. [docs/context/README.md](docs/context/README.md)
8. [docs/tools/README.md](docs/tools/README.md)
9. [docs/agent/README.md](docs/agent/README.md)
10. [docs/operations/verification.md](docs/operations/verification.md)

## Repository Rules

- Documentation is the implementation contract.
- Every authored file stays at or below 200 lines.
- Completion is checks-gated and harness-computed.
- No fake success, placeholders, or unrun verification claims.
- Commit small slices with honest `Tested` and `Not-tested` trailers.
