# Compose

## Purpose

The docker compose service design: what each service does, what it may
mount, and the guardrails that keep verification honest. The compose file
itself lands with [../execution/tasks/compose-final-gate.md](../execution/tasks/compose-final-gate.md).

## Services

| Service | Profile | Runs | Mounts |
| --- | --- | --- | --- |
| agent | default | lkjagent run and CLI commands | host data directory at /data |
| verify | verify | quiet verify inside the build image | none |
| endpoint-example | endpoint | optional llama.cpp-class endpoint | model bind read-only |

The endpoint is not a service this file owns: it is whatever
OpenAI-compatible server the owner runs, reachable from the agent service
by URL per [../architecture/llm/endpoint.md](../architecture/llm/endpoint.md).
A profiled example for a llama.cpp-class server container ships in the
compose file for convenience, disabled by default.

Compose reads the repository-root .env file automatically. That file holds
local deployment values and stays uncommitted; [.env.example](../../.env.example)
names the expected variables.

The agent workspace is /data/workspace inside the mounted data directory.
By default the host path is ./data; LKJAGENT_DATA_DIR changes it. The stock
image creates it for the non-root agent user.

## Profiles

The default compose profile is production-shaped: it starts only the
resident agent daemon with the mounted data directory.

The `verify` profile holds the final gate service. It may be run directly
with `docker compose run --rm verify`, or explicitly with the profile when
checking service sets.

The `endpoint` profile holds the local endpoint example. It is disabled
unless the owner asks for it with `docker compose --profile endpoint`.

## Shape

```yaml
services:
  agent:
    build:
      context: .
      target: runtime
    command: ["run"]
    init: true
    restart: unless-stopped
    stop_grace_period: 45s
    working_dir: /
    read_only: true
    security_opt:
      - no-new-privileges:true
    cap_drop:
      - NET_RAW
    tmpfs:
      - /tmp:size=64m,mode=1777
      - /home/agent:size=16m,uid=1000,gid=1000,mode=700
    volumes:
      - type: bind
        source: ${LKJAGENT_DATA_DIR:-./data}
        target: /data
        bind:
          create_host_path: true
    environment:
      - LKJAGENT_ENDPOINT_URL
      - LKJAGENT_ENDPOINT_TIMEOUT_SECONDS
      - LKJAGENT_MODEL
      - LKJAGENT_API_KEY
  verify:
    profiles: ["verify"]
    build:
      context: .
      target: build
    command: cargo run -p lkjagent-xtask -- quiet verify
  endpoint-example:
    profiles: ["endpoint"]
    image: ghcr.io/ggerganov/llama.cpp:server
```

The committed file is the contract; this sketch shows the intended shape
and the guardrails below bind it.

## Guardrails

- The verify service never mounts the source tree: it proves the committed
  repository, per [verification.md](verification.md).
- The agent service mounts exactly one durable host directory:
  LKJAGENT_DATA_DIR at /data. Mounting more enlarges the blast radius
  described in
  [../architecture/sandbox/safety.md](../architecture/sandbox/safety.md)
  and is an owner decision made in an override file, never in the committed
  compose file.
- /data/workspace is writable by the container user before real work is
  queued; otherwise fs.write and fs.edit report permission errors.
- The agent root filesystem is read-only. Ephemeral writes belong in tmpfs
  at /tmp or /home/agent; durable writes belong under /data.
- The daemon has a process and workspace healthcheck, bounded json-file
  logs, `restart: unless-stopped`, `init: true`, and a 45-second stop grace
  window.
- The default profile starts only the agent. Verification and local endpoint
  containers are opt-in services behind their profiles.
- The agent drops NET_RAW and runs with no-new-privileges. It still starts
  through the root entrypoint long enough to repair /data ownership before
  dropping to the agent user.
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
docker compose --profile endpoint up -d endpoint-example
```

## Status

implemented.
