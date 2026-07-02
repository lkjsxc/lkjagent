# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what is specified, what
is implemented, what is partial, and what remains open. A behavior is complete
only when code, focused tests, quiet gates, and required Docker gates prove it.

## Summary

The documentation contract now specifies the plan-driven step engine. The pure
`lkjagent-core` crate exists with model, parser, renderer, checks, word
counting, classification, and the `next_work` / `apply_turn` seam. The
`lkjagent-store` crate now also has plan-store modules for the ten-table schema,
queue intake, task and step writes, turn transactions, config heartbeat values,
and orphan exchange detection. `lkjagent-llm` now accepts per-call max token
budgets and stop sequences while keeping endpoint anomaly handling. The effects
layer, rich templates, and app binary are not yet built. The existing runtime,
graph, tools, CLI, benchmark, and store modules remain until cutover.

The target product is one daemon, one owner queue, one SQLite store, one local
LLM endpoint, and one deterministic plan ledger. The harness owns control flow;
the model authors bounded content. Completion is an engine-computed check edge,
never a model assertion.

## Specified Contract

- The vision pages define honesty, smallness, engineered context, and the
  harness-directs principle.
- The product pages define daemon lifecycle, queue semantics, CLI commands,
  status output, and console surfaces.
- Engine, context, protocol, tools, checks, memory, store, LLM, operations,
  evaluation, repository, agent, and decision pages specify the target tree.
- The docs gate is retargeted to README topology, link resolution, format bans,
  `check-files`, and the docs file-count budget.

## Implemented Code Still Present

`crates/lkjagent-core/` tests cover envelope parsing, word counting, prompt
fingerprint variation, and a generic task flow that reaches `closed` after
injected parse faults. `crates/lkjagent-store/` tests cover the fresh schema,
FIFO delivery, waiting-answer routing, turn transaction rollback, and orphan
exchange detection. `crates/lkjagent-llm/` tests cover write-step wire requests
with `llm.max-tokens.write=1400`, configurable stop suffix repair, provider
anomalies, and backoff.

The current Rust workspace still includes the existing parser, protocol
registry, graph, context, store, LLM client, tools, runtime, CLI, benchmark, and
xtask crates. These crates remain the shipping code until the application
cutover and deletion tasks replace them.

## Proven Baseline

Stage 0 baseline evidence is recorded under
`tmp/baseline-20260702T144718Z/`:

- `cargo run -p lkjagent-xtask -- quiet verify` printed `ok verify`.
- `docker compose run --rm verify` printed `ok verify` after container
  creation lines.
- The anchor branch `pre-reforge-anchor` points at
  `0c5ba3d634bcc16cff5266850624ddf1044190d0`.
- Historical live-run fixtures were copied to `tmp/fixtures/`.

## Open Work

- Effects layer, rich templates, and app binary are not yet implemented.
- The benchmark corpus and replay smoke still exercise the existing runtime.
- The Aurora Ledger live proof remains open until the daemon produces ten
  requested chapter files totaling at least 10,000 measured manuscript words and
  closes the task through engine-computed checks, or records an honest blocked
  result with fixtures and a concrete fix task.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist.
- Checked-in run logs can be failure fixtures without proving current success.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
