# Compose Final Gate

## Purpose

Land the docker compose file with the agent and verify services, finish
the runtime image, wire CI to the final gate, and prove the whole system
from a clean checkout.

## Status

open

## Depends On

[queue-cli.md](queue-cli.md); the Dockerfile from
[bootstrap-workspace.md](bootstrap-workspace.md).

## Files To Read

- [../../operations/compose.md](../../operations/compose.md)
- [../../operations/verification.md](../../operations/verification.md)
- [../../architecture/sandbox/container.md](../../architecture/sandbox/container.md)
- [../../architecture/sandbox/safety.md](../../architecture/sandbox/safety.md)

## Files To Touch

- docker-compose.yml (new): the two services and the named volume per the
  compose contract, including the disabled endpoint example.
- Dockerfile: finalize runtime stage contents (non-root agent user, core
  utilities, git, curl, ripgrep) and the seed skill copy step.
- .github/workflows/: point CI at the final gate if not already exact.

## Focused Gate

```sh
docker compose config
docker compose build agent verify
docker compose run --rm verify
```

## Acceptance

- The final gate passes from a clean checkout: image builds, quiet verify
  prints ok verify inside the container.
- The verify service has no mounts; the agent service mounts exactly the
  data volume and the workspace bind; asserted by inspecting
  `docker compose config` output in a gate check.
- The agent service starts, writes the default config on first start, and
  `docker compose exec agent lkjagent status` reports honestly against a
  reachable test endpoint.
- The runtime image runs as the non-root agent user with the documented
  utilities present.
- Blocker row 12 done; compose and sandbox statuses move in the ledger;
  the queue's Done section records the closing commits.

## Must Not

- Do not add bind mounts of the source tree to any service.
- Do not bake secrets, model names, or owner paths into the image or the
  committed compose file.
- Do not add services beyond agent and verify plus the disabled endpoint
  example.
