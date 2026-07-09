# Running

## Purpose

Define day-to-day operation for the daemon and owner CLI.

## Configuration

Configuration lives in flat `data/lkjagent.json` plus environment overrides.
Secrets are passed by environment variables and are not written to the config
file.

The exact key, type, range, default, reload, and consumer contract is in
[../product/configuration-registry.md](../product/configuration-registry.md).
The tracked example contains every registry key. Missing required structure,
unknown keys, arrays, nested values, wrong scalar types, invalid ranges, and
cross-key conflicts fail startup. Model-visible context never includes the raw
JSON config blob or secret values.

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
cargo run -p lkjagent-app -- run --once
cargo run -p lkjagent-app -- send "Record that hello.md should say hello."
cargo run -p lkjagent-app -- status
```

`run` is long-running. `run --once` executes one bounded daemon cycle for smoke
checks and exits with a short state summary. Implementation gaps remain listed
in [../current-state.md](../current-state.md).

## Fresh Trial

For a fresh trial, stop the daemon, move or remove the data directory, start the
agent service, send one owner turn, and inspect `status`, `matter show`,
`record list`, and the workspace files. The store and workspace are ordinary
files under `data/`.
