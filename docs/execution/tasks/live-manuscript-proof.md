# Live Manuscript Proof

## Purpose

Prove the remaining live endpoint boundary for a complete 10,000-word
manuscript owned by the daemon, or record the exact observed failure without
claiming completion.

## Status

open: deterministic artifact planning, contracts, audit, readiness, replay,
quiet verify, and Docker verify are implemented. The Aurora Ledger live proof
at `tmp/live-proof-20260701T100958Z/` exhausted the observation loop in recovery
with 26 scene files, 6,230 scene words, zero assembled chapter files, zero final
manuscript words, no artifact readiness rows, and repeated `fs.batch_write`
recovery notices.

## Depends On

- [../../current-state.md](../../current-state.md)
- [../current-blockers.md](../current-blockers.md)
- [large-artifact-engine.md](large-artifact-engine.md)
- [../../operations/verification.md](../../operations/verification.md)

## Files To Read

- `tmp/lkjagent-redesign-report/02-runbooks/live-daemon-proof.md`
- `docs/current-state.md`
- `docs/execution/current-work/story-manuscript-generation-gap.md`
- `docs/architecture/artifacts/large-artifact/README.md`
- `docs/operations/running.md`

## Files To Touch

- `tmp/live-proof-*/` for captured command output and copied workspace evidence
- `docs/current-state.md`
- `docs/execution/current-blockers.md`
- `docs/execution/tasks/live-manuscript-proof.md`
- focused code and regression tests only when the run exposes a deterministic fault

## Focused Gate

```sh
cargo run -p lkjagent-xtask -- smoke live
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Endpoint config is checked before the live run.
- If endpoint config is absent, `smoke live` records the explicit skip.
- If endpoint config exists, the runbook captures status, logs, model logs,
  workspace files, and a proof bundle under `tmp/`.
- Success requires admitted `agent.done`, artifact readiness `ready`, no active
  artifact contract, root `stories/aurora-ledger`, ten requested chapter files,
  real manuscript prose, and at least 10,000 measured manuscript words.
- Any wait, loop, early close, generic root, story-bible-only close, endpoint
  failure, or word-count shortfall keeps this blocker open and adds focused
  regression coverage before the next completion claim.

## Must Not

- Do not use a direct endpoint fallback as daemon proof.
- Do not count story-bible, README, catalog, transcript, or owner request text
  as manuscript words.
- Do not commit secrets or local endpoint configuration.
- Do not claim live completion unless the success criteria are all captured in
  this checkout.
