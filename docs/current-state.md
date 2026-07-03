# Current State

## Purpose

This file is the honest ledger of lkjagent. Live proof status: the endpoint
adapter is implemented and the Aurora Ledger unattended run has a captured proof
bundle from the current product code.

## Summary

The product binary is `lkjagent-app` built as `lkjagent`. The remaining
workspace crates are `lkjagent-core`, `lkjagent-store`, `lkjagent-llm`,
`lkjagent-effects`, `lkjagent-app`, and `lkjagent-xtask`. The old runtime,
graph, tools, CLI, context, protocol, benchmark crates, and old store modules
are deleted.

The deterministic plan engine is implemented and cut over. Benchmark corpus,
benchmark live-suite command, smoke replay, proof collection, structure audit,
Docker verify, app container idle startup, endpoint-backed daemon calls, and the
Aurora Ledger live proof have green evidence. The live proof closed one
manuscript task after ten chapter files, passing file-count, word-count, and
absence checks without operator intervention after queueing the task.

## Proven Surfaces

| Surface | Evidence |
| --- | --- |
| Docs topology, links, ASCII, and bans | `tmp/final-20260703T061546Z/check-docs.txt`: `ok check-docs` |
| Per-file line cap | `tmp/final-20260703T061546Z/check-lines.txt`: `ok check-lines` |
| Source and docs file budgets | `tmp/final-20260703T061546Z/check-files.txt`: `ok check-files` |
| Panic-path and dependency style | `tmp/final-20260703T061546Z/check-style.txt`: `ok check-style` |
| Structure audit | `tmp/final-20260703T061546Z/structure-audit.txt`: `ok structure audit` |
| Benchmark corpus | `tmp/final-20260703T061546Z/bench-check-corpus.txt`: `ok bench check-corpus` |
| Benchmark live-suite command | `tmp/bench-live-20260703T054104Z/benchmark-report.md`: score `6/7` |
| Deterministic replay | `tmp/final-20260703T061546Z/smoke-replay.txt`: `ok smoke replay` |
| Live smoke status | `tmp/final-20260703T061546Z/smoke-live.txt`: `ok smoke live` |
| Workspace fmt, clippy, and tests | `tmp/final-20260703T061546Z/quiet-verify.txt`: `ok verify` |
| Docker verify image | `tmp/final-20260703T061546Z/docker-verify.txt`: `ok verify` |
| App container idle status | `tmp/final-20260703T061546Z/docker-agent-status.txt`: status output exit 0 |
| Forbidden old control-plane markers absent | `tmp/final-20260703T061546Z/forbidden-grep.txt`: no output |
| Aurora Ledger live proof | `tmp/live-proof-20260703T055604Z/proof/summary.md`: closed manuscript, ten chapters, checks passed |

## Implemented Code

`lkjagent-core` owns the pure plan engine, parser, renderer, checks, word
counting, classifier, simple templates, manuscript template, docs-tree template,
and recovery helpers. `lkjagent-store` owns the plan-store schema and terminal
state transactions. `lkjagent-llm` owns the endpoint wire client.
`lkjagent-effects` owns filesystem, shell, check gathering, observations, and
exchange logs. `lkjagent-app` owns the daemon interpreter, queue-backed fake
endpoint seam, CLI parser, status and watch renderers, store-backed resume,
endpoint adapter, and template end-to-end coverage. `lkjagent-xtask` owns the
repository gates, structure audit, benchmark corpus validation, live benchmark
suite runner, deterministic replay, and proof bundle collection.

## Open Boundaries

- No executable reforge task is known to be open in the current repository
  state.
- The endpoint benchmark command reports the observed live score; it does not
  treat a partial score as a release gate.

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
