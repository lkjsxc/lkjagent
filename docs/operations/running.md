# Running

## Purpose

Define day-to-day operation for the daemon and owner CLI.

## Environment

Configuration lives in flat `data/lkjagent.json` plus environment overrides.
Secrets are passed by environment variables and are not written to the config
file.

| Key | Meaning |
| --- | --- |
| `endpoint_url` | OpenAI-compatible chat-completions URL |
| `endpoint_model` | model name sent to the endpoint |
| `endpoint_api_key_env` | environment variable that holds the key |
| `endpoint_timeout_seconds` | finite endpoint timeout |
| `workspace_root` | workspace directory relative to data root |
| `prompt_max_context_tokens` | prompt render ceiling |
| `live_campaign_seconds` | default live campaign duration |

Older nested config keys may be read only to rewrite them into the flat file.
Model-visible context never includes the raw JSON config blob.

## Docker Operation

```sh
docker compose up -d agent
docker compose run --rm agent lkjagent status
docker compose run --rm agent lkjagent send "Record that hello.md should say hello."
docker compose run --rm agent lkjagent log --limit 20
docker compose run --rm agent lkjagent watch
```

## Direct Operation

```sh
cargo run -p lkjagent-app -- run
cargo run -p lkjagent-app -- send "Record that hello.md should say hello."
cargo run -p lkjagent-app -- status
```

During cutover, [../current-state.md](../current-state.md) states which binary
is active.

## Fresh Trial

For a fresh trial, stop the daemon, move or remove the data directory, start the
agent service, send one owner turn, and inspect `status`, `matter show`,
`record list`, and the workspace files. The store and workspace are ordinary
files under `data/`.
