# lkjagent

## Purpose

lkjagent is a continuously running personal agent for one owner, one local LLM,
one visible workspace, and one SQLite ledger. Owner turns become matters,
records, decisions, state cells, events, artifacts, checks, and proof evidence.
Record-like owner requests write through to files under `data/workspace` by
default.

## Product Shape

- Durable rows and persisted runtime decisions own control flow, retries, paths,
  tool admission, context selection, and completion checks.
- The workspace is the owner-readable source for journals, notes, calendar-like
  entries, finance records, project notes, generated artifacts, transcripts,
  indexes, and proof logs.
- Owner turns are routed semantically: answer a pending question, append to a
  matter, write or update a record, request an artifact, inspect workspace
  evidence, or request a system operation.
- The model sees compact XML-like prompt cards with source refs. Model action
  output uses an attribute-less XML-like grammar, not JSON.
- The daemon runs inside Docker Compose; durable rows and workspace files remain
  the audit boundary.

## Common Commands

```sh
cargo run -p lkjagent-app -- run
cargo run -p lkjagent-app -- send "Record that I paid the train fare"
cargo run -p lkjagent-app -- status
cargo run -p lkjagent-app -- matter list
cargo run -p lkjagent-app -- record list
cargo run -p lkjagent-app -- log --limit 20
cargo run -p lkjagent-app -- watch
cargo run -p lkjagent-app -- workbench
cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current
docker compose run --rm verify
```

The commands above are the target owner and developer surfaces. Transitional
bridge commands may still read plan-family rows until the implementation no
longer needs them. [docs/current-state.md](docs/current-state.md) is the
behavior and evidence ledger.

## Read Order

1. [docs/current-state.md](docs/current-state.md)
2. [docs/vision/README.md](docs/vision/README.md)
3. [docs/state/README.md](docs/state/README.md)
4. [docs/runtime/README.md](docs/runtime/README.md)
5. [docs/product/README.md](docs/product/README.md)
6. [docs/workspace/README.md](docs/workspace/README.md)
7. [docs/context/README.md](docs/context/README.md)
8. [docs/protocol/README.md](docs/protocol/README.md)
9. [docs/tools/README.md](docs/tools/README.md)
10. [docs/agent/README.md](docs/agent/README.md)
11. [docs/operations/verification.md](docs/operations/verification.md)

## Repository Rules

- Documentation is the implementation contract.
- Every authored file stays at or below 200 lines.
- Completion is checks-gated and harness-computed.
- No fake success, placeholders, or unrun verification claims.
- Commit small slices with honest `Tested` and `Not-tested` trailers.
