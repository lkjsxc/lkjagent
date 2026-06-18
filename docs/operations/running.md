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
| LKJAGENT_CONTEXT_LENGTH | endpoint example context length |

Host environment variables override values in .env. Changing deployment
values requires a restart; the daemon never hot-reloads, per the cache rules
in [../architecture/context/caching.md](../architecture/context/caching.md).
The agent workspace is /data/workspace inside the container and
LKJAGENT_DATA_DIR/workspace on the host. The default host path is
./data/workspace.

## Runtime Config

The runtime config file is /data/lkjagent.json, read once at daemon startup.
It records resolved defaults and non-secret runtime knobs.

| Key | Initial contract | Meaning |
| --- | --- | --- |
| endpoint.url | http://endpoint:8080 | fallback when LKJAGENT_ENDPOINT_URL is unset |
| endpoint.model | LKJAGENT_MODEL or required | fallback when LKJAGENT_MODEL is unset |
| endpoint.api-key-env | LKJAGENT_API_KEY | name of the env var holding the key, when one is needed |
| endpoint.timeout-seconds | 180 | request timeout; LKJAGENT_ENDPOINT_TIMEOUT_SECONDS overrides |
| context.window | 32768 | total token window the budgets divide |
| context.reserve | 1024 | generation headroom, also max_tokens |
| context.trigger | 28672 | compaction trigger per [../architecture/context/budgets.md](../architecture/context/budgets.md) |
| sampling.temperature | 0.3 | per [../architecture/llm/sampling.md](../architecture/llm/sampling.md) |
| sampling.top-p | 0.9 | same |
| task.turn-budget | 64 | per [../architecture/runtime/agent-loop.md](../architecture/runtime/agent-loop.md) |
| daemon.lock-stale-seconds | 300 | daemon lock reclaim window; never below endpoint timeout plus 60 |
| shell.timeout-seconds | 60 | default for shell.run, max 600 |

LKJAGENT_ENDPOINT_URL, LKJAGENT_MODEL, and
LKJAGENT_ENDPOINT_TIMEOUT_SECONDS override the endpoint table at startup.
The API key value is never written to /data/lkjagent.json; only the
environment variable name is stored.

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
memory, and skills commands run as short-lived containers that use the same
/data bind mount. Stopping the service relies on durable store state and
stale lock reclaim; there is no custom signal-drain path.

## Backup and Reset

- Back up: snapshot LKJAGENT_DATA_DIR (store, skill library, workspace, and
  config); all are ordinary files.
- Reset memory but keep skills: delete the store file while stopped; the
  queue and transcripts go with it, deliberately.
- Full reset: remove the host data directory; first start reseeds skills per
  [../architecture/skills/library.md](../architecture/skills/library.md).

## Status

implemented.
