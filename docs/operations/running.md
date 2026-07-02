# Running

## Purpose

Define day-to-day operation for the daemon and owner CLI.

## Environment

Configuration lives in `data/lkjagent.json` plus environment overrides. Secrets
are passed by environment variables and are not written to the config file.

| Key | Meaning |
| --- | --- |
| `endpoint.url` | OpenAI-compatible base URL |
| `endpoint.model` | model name sent to the endpoint |
| `endpoint.api-key-env` | environment variable that holds the key |
| `context.window` | configured context length |
| `sampling.temperature` | default `llm.temperature=0.3` |
| `sampling.top-p` | default `llm.top-p=0.9` |

## Docker Operation

```sh
docker compose up -d agent
docker compose run --rm agent lkjagent status
docker compose run --rm agent lkjagent send "Create hello.md with a hello."
docker compose run --rm agent lkjagent log --limit 20
docker compose run --rm agent lkjagent watch
```

## Direct Operation

```sh
cargo run -p lkjagent-app -- run
cargo run -p lkjagent-app -- send "Create hello.md with a hello."
cargo run -p lkjagent-app -- status
```

During cutover, [../current-state.md](../current-state.md) states which binary
is active.

## Fresh Trial

For a fresh trial, stop the daemon, move or remove the data directory, start the
agent service, send one owner task, and inspect `status`, `task show`, and the
workspace files. The store and workspace are ordinary files under `data/`.
