# Running

## Purpose

The runtime configuration contract and day-to-day operation of the harness.
Owner-visible behavior is under [../product/](../product/README.md); this
file owns the knobs.

## Environment

Local deployment values live in the repository-root .env file. It is read by
docker compose before containers start and by the lkjagent binary when run
directly from the repository root. .env is not committed; copy
[../../.env.example](../../.env.example) and fill the local values.

| Variable | Meaning |
| --- | --- |
| LKJAGENT_ENDPOINT_URL | base URL of the chat-completions server |
| LKJAGENT_MODEL | model name passed through to the server |
| LKJAGENT_API_KEY | optional bearer token for the endpoint |
| LKJAGENT_ENDPOINT_TIMEOUT_SECONDS | endpoint request timeout; default 180 |
| LKJAGENT_DATA_DIR | host path mounted as /data; default ./data |
| LKJAGENT_MODEL_DIR | host path for the disabled endpoint example |
| LKJAGENT_CONTEXT_LENGTH | runtime context.window override |

Host environment variables override values in .env. Changing deployment
values requires a restart; the daemon never hot-reloads, per the cache rules
in [../architecture/context/caching.md](../architecture/context/caching.md).
The agent workspace is /data/workspace inside the container and
LKJAGENT_DATA_DIR/workspace on the host. The default host path is
./data/workspace.

## Fresh Docker Trial

```sh
rm -rf data
mkdir -p data
docker compose up -d --build agent
docker compose run --rm agent status
docker compose run --rm agent graph
docker compose run --rm agent send "Create hello.md with a short hello."
docker compose run --rm agent log
find data/workspace -maxdepth 2 -type f -print
```

This leaves the host data directory empty before first start. The daemon then
creates only clean runtime files under /data: config, store, and workspace. A
fresh workspace is intentionally empty until an owner message is sent and the
agent writes files for that task. Source graph definitions live in the image
or local build, while runtime graph cases live in /data.

## Runtime Config

The runtime config file is /data/lkjagent.json, read once at daemon startup.
It records resolved defaults and non-secret runtime knobs.

| Key | Initial contract | Meaning |
| --- | --- | --- |
| endpoint.url | http://endpoint:8080 | fallback when LKJAGENT_ENDPOINT_URL is unset |
| endpoint.model | LKJAGENT_MODEL or required | fallback when LKJAGENT_MODEL is unset |
| endpoint.api-key-env | LKJAGENT_API_KEY | name of the env var holding the key, when one is needed |
| endpoint.timeout-seconds | 180 | request timeout; LKJAGENT_ENDPOINT_TIMEOUT_SECONDS overrides |
| context.window | 24576 | total token window the budgets divide; 16384 is the supported lower bound |
| context.reserve | 2048 | generation headroom, also max_tokens |
| context.trigger | 21504 | optional hard trigger; stale or unsafe values are derived from the selected window |
| sampling.temperature | 0.3 | per [../architecture/llm/sampling.md](../architecture/llm/sampling.md) |
| sampling.top-p | 0.9 | same |
| task.turn-budget | 64 | endpoint turns before an autonomous runtime checkpoint; not an owner permission boundary |
| daemon.lock-stale-seconds | 300 | daemon lock reclaim window; never below endpoint timeout plus 60 |
| shell.timeout-seconds | 60 | default for shell.run, max 600 |

LKJAGENT_ENDPOINT_URL, LKJAGENT_MODEL,
LKJAGENT_ENDPOINT_TIMEOUT_SECONDS, and LKJAGENT_CONTEXT_LENGTH override the
runtime table at startup. `task.turn-budget` is interpreted as checkpoint
turns: reaching it records a continuation epoch and resumes automatically when
runtime authority still admits useful owner work.
The API key value is never written to /data/lkjagent.json; only the
environment variable name is stored.

`LKJAGENT_CONTEXT_LENGTH=16384` is valid and causes earlier compaction with
a smaller live log. Values below 16384 fail with a config error. Changing
any context budget value requires a daemon restart because the prefix cache
and pressure policy are session state.

`task.turn-budget` is read at daemon startup. Larger values let one owner
message drive longer structured work before the harness asks for continuation
guidance; smaller values force earlier owner checkpoints.

First start writes /data/lkjagent.json from .env when LKJAGENT_MODEL is set.
When no model exists in either .env, the host environment, or the config file,
startup writes a default JSON file and exits asking the owner to fill in
endpoint.model.

## Day to Day

```sh
docker compose up -d agent
docker compose run --rm agent console
docker compose run --rm agent send "Survey the workspace and report."
docker compose run --rm agent log --follow
docker compose run --rm agent status
```

The agent service is the resident daemon. Console, send, log, status,
memory, and graph commands run as short-lived containers that use the same
/data bind mount for store, config, and workspace. Stopping the service relies on durable store state
and stale lock reclaim; there is no custom signal-drain path.

## Backup and Reset

- Back up: snapshot LKJAGENT_DATA_DIR (store, workspace, and config); all
  are ordinary files.
- Reset memory: delete the store file while stopped; the queue and
  transcripts go with it, deliberately.
- Full reset: remove the host data directory; first start recreates config,
  workspace, and runtime graph state from source graph definitions.

## Status

implemented.
