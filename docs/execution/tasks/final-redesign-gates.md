# Final Redesign Gates

## Purpose

Finish the redesign with repository-level proof and an honest handoff.

## Status

open

## Depends On

- [smoke-harness.md](smoke-harness.md)

## Files To Read

1. [Handoff](../../agent/handoff.md)
2. [Verification](../../operations/verification.md)
3. [Current state](../../current-state.md)
4. [Current blockers](../current-blockers.md)
5. Every task file completed in the redesign sequence

## Files To Touch

- `docs/current-state.md`
- `docs/execution/current-blockers.md`
- task docs whose statuses close in this slice
- final handoff notes requested by the owner
- source or docs only for final corrections found by gates

## Focused Gate

```sh
cargo fmt --check
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- All focused gates named by completed task docs have run with actual results.
- Quiet verify passes.
- Docker Compose final verify passes before repository implementation success is
  claimed.
- `docs/current-state.md` names only proved behavior.
- Current blocker statuses match evidence.
- The handoff names changed docs, changed source, run commands, skipped commands,
  remaining risks, and the next executable step.

## Must Not

- Do not claim long manuscript completion unless the daemon path creates the
  artifact, records ledger evidence, and closes through the central gate.
- Do not hide failing or skipped gates.
- Do not include unrelated working-tree changes in the final commit.
