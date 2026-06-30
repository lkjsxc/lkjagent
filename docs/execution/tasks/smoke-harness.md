# Smoke Harness

## Purpose

Add reproducible smoke and replay harnesses for runtime claims.

## Status

done: deterministic smoke replay, explicit live smoke skip status, bounded
`tmp/` summaries, Docker replay wiring, focused tests, benchmark corpus, quiet
verify, and Docker verify passed

## Depends On

- [manuscript-compose.md](manuscript-compose.md)

## Files To Read

1. [Verification](../../operations/verification.md)
2. [Evaluation](../../evaluation/README.md)
3. [Runtime smoke ground truth](../current-work/runtime-smoke-ground-truth.md)
4. [Story manuscript generation gap](../current-work/story-manuscript-generation-gap.md)
5. `crates/lkjagent-xtask/src/`
6. `crates/lkjagent-benchmark/src/`
7. `docker-compose.yml`

## Files To Touch

- `docs/operations/verification.md`
- `docs/evaluation/README.md`
- current-work smoke docs only when evidence is actually produced
- `crates/lkjagent-xtask/src/`
- `crates/lkjagent-benchmark/src/`
- deterministic smoke fixtures

## Focused Gate

```sh
cargo fmt --check
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Deterministic smoke runs without an external endpoint.
- Live endpoint smoke is explicit and skipped honestly when endpoint config is
  absent.
- Smoke summaries include token aggregates, decision ids, root, paths, word
  counts, and completion gate result.
- Historical missing-root loops, generic roots, false closes, provider
  anomalies, and manuscript incomplete paths have replay coverage.
- Smoke artifacts are bounded and archived under `tmp/`.

## Must Not

- Do not make live endpoint smoke part of quiet verify unless it runs without
  secret config.
- Do not check in large generated artifacts without a task-specific reason.
- Do not report smoke success without inspecting real workspace evidence.
