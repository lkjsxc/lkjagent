# lkjagent

## Purpose

lkjagent is a minimal, continuously running agent harness for a local LLM.
One daemon owns one agent loop. A persistent queue feeds it user messages.
It talks to an OpenAI-compatible endpoint, acts through a small fixed
toolset, and refines its own skills and memory while idle.

This repository is read and written by LLM agents. The documentation tree
under [docs/](docs/README.md) is the implementation contract: code follows docs.

## Shape

- Rust cargo workspace of small focused crates; functional core, effects at the edges.
- Runs entirely inside a container; the host only runs docker compose.
- YOLO only: no permission prompts; the sandbox boundary is the safety model.
- Local model budget: about 16 GB of memory and a 32k token context window.
- Append-only context with explicit compaction keeps the endpoint prefix cache hot.
- The model speaks a tag-based action protocol, never JSON.
- Skills are markdown capability files; one format serves the harness and its builders.
- No MCP, no sub-agents, no plan mode, no web UI.

## Status

The Cargo workspace, local verification gates, action parser, context engine,
container wiring, resident daemon, tool loop, and queue intake are implemented.

- [docs/current-state.md](docs/current-state.md) is the honest status ledger.
- [docs/execution/current-blockers.md](docs/execution/current-blockers.md) is the implementation queue.

## Read Order

1. [docs/current-state.md](docs/current-state.md)
2. [docs/README.md](docs/README.md)
3. [AGENTS.md](AGENTS.md) for automated coding agents
4. The README of the area being changed

## Repository Map

| Path | Role |
| --- | --- |
| [docs/](docs/README.md) | Implementation contract (canonical) |
| [AGENTS.md](AGENTS.md) | Entry point for automated coding agents |
| crates/ | Rust workspace; see [docs/repository/layout.md](docs/repository/layout.md) |
| [LICENSE](LICENSE) | Apache License 2.0 |

## Operation Design

The harness is operated through one binary inside the container:

```sh
lkjagent run            # start the daemon (single agent loop)
lkjagent send "text"    # enqueue a user message
lkjagent status         # daemon state, queue depth, context usage
lkjagent log            # tail recent transcript events
```

For a fresh Docker trial, start the daemon and then send work. The workspace is
empty until the first task writes files.

```sh
rm -rf data
mkdir -p data
docker compose up -d --build agent
docker compose run --rm agent status
docker compose run --rm agent send "Create hello.md with a short hello."
docker compose run --rm agent log
find data/workspace -maxdepth 2 -type f -print
```

The full contract lives in [docs/product/cli.md](docs/product/cli.md) and
[docs/operations/running.md](docs/operations/running.md).

## Verification Design

Local gates are implemented in
[docs/operations/verification.md](docs/operations/verification.md) and
[crates/lkjagent-xtask/](crates/lkjagent-xtask/):

```sh
cargo run -p lkjagent-xtask -- check-docs    # doc shape, topology, links, banned tokens
cargo run -p lkjagent-xtask -- check-lines   # 200-line cap on every file
cargo run -p lkjagent-xtask -- check-style   # forbid panic paths in product crates
cargo run -p lkjagent-xtask -- quiet verify  # all checks plus tests, prints: ok verify
docker compose run --rm verify               # final gate, image build, no source mounts
```

Until the xtask exists, the shell checks listed in
[docs/operations/verification.md](docs/operations/verification.md) are the gate.
