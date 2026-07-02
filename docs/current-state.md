# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what is specified, what
is implemented, what is partial, and what remains open. A behavior is complete
only when code, focused tests, quiet gates, and required Docker gates prove it.

## Summary

The documentation contract is being rewritten to a plan-driven step engine. The
engine is not yet built. The repository still contains the existing runtime,
graph, tools, CLI, benchmark, and store implementation until the cutover task
removes them.

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
  evaluation, repository, agent, and decision pages land during the current
  documentation stage.

## Implemented Code Still Present

The current Rust workspace still includes the pre-cutover crates and gates:
parser, protocol registry, graph, context, store, LLM client, tools, runtime,
CLI, benchmark, and xtask. These crates remain the shipping code until the
application cutover and deletion tasks replace them.

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

- The plan-driven engine, store schema, effects layer, templates, and app binary
  are not yet implemented.
- The documentation gate still accepts the pre-cutover tree until the docs gate
  retarget task lands.
- The benchmark corpus and replay smoke still exercise the pre-cutover runtime.
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
