# Current State

## Purpose

This file is the honest ledger of lkjagent. Live proof status: blocked before
loop iteration one because `lkjagent-app` has no production endpoint adapter yet;
see `tmp/final-20260702T180819Z/handoff.md` for the final sweep evidence.

## Summary

The product binary is `lkjagent-app` built as `lkjagent`. The remaining
workspace crates are `lkjagent-core`, `lkjagent-store`, `lkjagent-llm`,
`lkjagent-effects`, `lkjagent-app`, and `lkjagent-xtask`. The old runtime,
graph, tools, CLI, context, protocol, benchmark crates, and old store modules
are deleted.

The deterministic plan engine is implemented and cut over. Benchmark corpus,
smoke replay, proof collection, Docker verify, and app container idle startup
have green evidence. The live Aurora Ledger endpoint proof has not run because
the app daemon still lacks the production endpoint adapter.

## Proven Surfaces

| Surface | Evidence |
| --- | --- |
| Docs topology, links, ASCII, and bans | `tmp/final-20260702T180819Z/check-docs.txt`: `ok check-docs` |
| Per-file line cap | `tmp/final-20260702T180819Z/check-lines.txt`: `ok check-lines` |
| Source and docs file budgets | `tmp/final-20260702T180819Z/check-files.txt`: `ok check-files` |
| Panic-path and dependency style | `tmp/final-20260702T180819Z/check-style.txt`: `ok check-style` |
| Benchmark corpus | `tmp/final-20260702T180819Z/bench-check-corpus.txt`: `ok bench check-corpus` |
| Deterministic replay | `tmp/final-20260702T180819Z/smoke-replay.txt`: `ok smoke replay` |
| Workspace fmt, clippy, and tests | `tmp/final-20260702T180819Z/quiet-verify.txt`: `ok verify` |
| Docker verify image | `tmp/final-20260702T180819Z/docker-verify.txt`: `ok verify` |
| App container idle status | `tmp/final-20260702T180819Z/docker-agent-status.txt`: status output exit 0 |
| Forbidden old control-plane markers absent | `tmp/final-20260702T180819Z/forbidden-grep.txt`: no output |

## Implemented Code

`lkjagent-core` owns the pure plan engine, parser, renderer, checks, word
counting, classifier, simple templates, manuscript template, docs-tree template,
and recovery helpers. `lkjagent-store` owns the plan-store schema and
transactions. `lkjagent-llm` owns the endpoint wire client. `lkjagent-effects`
owns filesystem, shell, check gathering, observations, and exchange logs.
`lkjagent-app` owns the daemon interpreter, queue-backed fake endpoint seam, CLI
parser, status and watch renderers, store-backed resume, and template
end-to-end coverage. `lkjagent-xtask` owns the repository gates, benchmark
corpus validation, deterministic replay, and proof bundle collection.

## Open Boundaries

- `lkjagent-app` does not yet call the configured endpoint in its daemon loop.
- The Aurora Ledger live proof is blocked before loop iteration one for that
  reason; no live success is claimed.
- Structure audit is not yet rebuilt for the plan engine.
- `bench run` writes the bounded report surface but does not yet drive a real
  endpoint scoring run.

## Proven Baseline

Stage 0 baseline evidence is recorded under
`tmp/baseline-20260702T144718Z/`:

- `cargo run -p lkjagent-xtask -- quiet verify` printed `ok verify`.
- `docker compose run --rm verify` printed `ok verify` after container
  creation lines.
- The anchor branch `pre-reforge-anchor` points at
  `0c5ba3d634bcc16cff5266850624ddf1044190d0`.
- Historical live-run fixtures were copied to `tmp/fixtures/`.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist.
- Checked-in run logs can be failure fixtures without proving current success.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
