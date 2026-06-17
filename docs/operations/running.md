# Running

## Purpose

The runtime configuration contract and day-to-day operation of the harness.
Owner-visible behavior is under [../product/](../product/README.md); this
file owns the knobs.

## Configuration

One file: /data/lkjagent.toml, read once at daemon startup. Changing it
requires a restart; the daemon never hot-reloads, per the cache rules in
[../architecture/context/caching.md](../architecture/context/caching.md).

| Key | Initial contract | Meaning |
| --- | --- | --- |
| endpoint.url | http://endpoint:8080 | base URL of the chat-completions server |
| endpoint.model | (required) | model name passed through to the server |
| endpoint.api-key-env | LKJAGENT_API_KEY | name of the env var holding the key, when one is needed |
| context.window | 32768 | total token window the budgets divide |
| context.reserve | 1024 | generation headroom, also max_tokens |
| context.trigger | 28672 | compaction trigger per [../architecture/context/budgets.md](../architecture/context/budgets.md) |
| sampling.temperature | 0.3 | per [../architecture/llm/sampling.md](../architecture/llm/sampling.md) |
| sampling.top-p | 0.9 | same |
| task.turn-budget | 64 | per [../architecture/runtime/agent-loop.md](../architecture/runtime/agent-loop.md) |
| maintenance.cycle-budget | 8 | per [../architecture/runtime/self-maintenance.md](../architecture/runtime/self-maintenance.md) |
| shell.timeout-seconds | 60 | default for shell.run, max 600 |

Environment variables override nothing except through the api-key-env
indirection and the compose-level variables in [compose.md](compose.md);
configuration has one home.

First start writes a commented default file to /data/lkjagent.toml when
none exists, then exits asking the owner to fill in endpoint.model; the
daemon never guesses a model name.

## Day to Day

```sh
docker compose up -d agent
docker compose exec agent lkjagent send "Survey the workspace and report."
docker compose exec agent lkjagent log --follow
docker compose exec agent lkjagent status
```

Stopping: `docker compose stop agent` delivers SIGTERM; the daemon finishes
the in-flight turn and exits per
[../architecture/runtime/daemon-process.md](../architecture/runtime/daemon-process.md).

## Backup and Reset

- Back up: snapshot the lkjagent-data volume (store plus skill library);
  both are ordinary files.
- Reset memory but keep skills: delete the store file while stopped; the
  queue and transcripts go with it, deliberately.
- Full reset: remove the volume; first start reseeds skills per
  [../architecture/skills/library.md](../architecture/skills/library.md).

## Status

implemented.
