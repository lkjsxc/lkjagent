# Compose

## Purpose

The docker compose service design: what each service does, what it may
mount, and the guardrails that keep verification honest. The compose file
itself lands with [../execution/tasks/compose-final-gate.md](../execution/tasks/compose-final-gate.md).

## Services

| Service | Runs | Mounts |
| --- | --- | --- |
| agent | lkjagent run and the other lkjagent commands | named volume at /data |
| verify | quiet verify inside the build image | none |

The endpoint is not a service this file owns: it is whatever
OpenAI-compatible server the owner runs, reachable from the agent service
by URL per [../architecture/llm/endpoint.md](../architecture/llm/endpoint.md).
A profiled example for a llama.cpp-class server container ships in the
compose file for convenience, disabled by default.

Compose reads the repository-root .env file automatically. That file holds
local deployment values and stays uncommitted; [.env.example](../../.env.example)
names the expected variables.

The agent workspace is /data/workspace inside the named data volume. The
stock image creates it for the non-root agent user.

## Shape

```yaml
services:
  agent:
    build:
      context: .
      target: runtime
    command: ["run"]
    volumes:
      - lkjagent-data:/data
    environment:
      - LKJAGENT_ENDPOINT_URL
      - LKJAGENT_ENDPOINT_TIMEOUT_SECONDS
      - LKJAGENT_MODEL
      - LKJAGENT_API_KEY
  verify:
    build:
      context: .
      target: build
    command: cargo run -p lkjagent-xtask -- quiet verify
volumes:
  lkjagent-data:
```

The committed file is the contract; this sketch shows the intended shape
and the guardrails below bind it.

## Guardrails

- The verify service never mounts the source tree: it proves the committed
  repository, per [verification.md](verification.md).
- The agent service mounts exactly one thing: the data volume. Mounting more
  enlarges the blast radius described in
  [../architecture/sandbox/safety.md](../architecture/sandbox/safety.md)
  and is an owner decision made in an override file, never in the committed
  compose file.
- /data/workspace is writable by the container user before real work is
  queued; otherwise fs.write and fs.edit report permission errors.
- No develop or watch sections, no source bind for hot reload: the harness
  is rebuilt, not reloaded.
- Secrets and deployment values travel as environment variables from .env or
  the host shell, never as committed values; the compose file references
  names only.
- One compose file; environment differences live in the documented
  variables, not in file forks.

## Daily Commands

```sh
docker compose up -d agent                 # start the resident daemon
docker compose run --rm agent console      # open the owner console
docker compose run --rm agent send "..."   # queue owner work
docker compose run --rm agent log --follow # read transcript events
docker compose run --rm agent status       # observe daemon and queue state
docker compose run --rm verify             # final gate
```

## Status

implemented.
