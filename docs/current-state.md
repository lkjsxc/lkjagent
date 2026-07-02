# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what is specified, what
is implemented, what is partial, and what remains open. A behavior is complete
only when code, focused tests, quiet gates, and required Docker gates prove it.

## Summary

The product binary is now `lkjagent-app` built as `lkjagent`. The remaining
workspace crates are `lkjagent-core`, `lkjagent-store`, `lkjagent-llm`,
`lkjagent-effects`, `lkjagent-app`, and `lkjagent-xtask`. The old runtime,
graph, tools, CLI, context, protocol, benchmark crates, and old store modules
are deleted.

`lkjagent-core` owns the pure plan engine, parser, renderer, checks, word
counting, classifier, simple templates, manuscript template, and docs-tree
template. `lkjagent-store` owns the plan-store schema and transactions.
`lkjagent-llm` owns the endpoint wire client. `lkjagent-effects` owns filesystem,
shell, check gathering, observations, and exchange logs. `lkjagent-app` owns the
daemon interpreter, queue-backed fake endpoint seam, CLI parser, status and watch
renderers, store-backed resume, and template end-to-end coverage.

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

## Implemented Code

`crates/lkjagent-core/` tests cover envelope parsing, word counting, prompt
fingerprint variation, generic closure after injected parse faults, classifier
fixtures, simple template shapes, manuscript extraction and checks, docs-tree
plan validation, and recovery helpers.

`crates/lkjagent-store/` tests cover the fresh schema, FIFO delivery,
waiting-answer routing, turn transaction rollback, and orphan exchange
detection. `crates/lkjagent-llm/` tests cover per-call token budgets,
configurable stop suffix repair, provider anomalies, and backoff.
`crates/lkjagent-effects/` tests cover path escape and symlink guards, file
operations, shell output, observation truncation, exchange log writing, and all
deterministic catalog checks over a fixture tree.

`crates/lkjagent-app/` tests cover the documented help tree, queue-backed fake
endpoint execution, store-backed resume, simple-template end-to-end closure,
waiting-answer resume, expected workspace files, manuscript ten-chapter closure,
shortfall extension, write-fault splitting, docs-tree closure, docs-tree link
repair, and the status field set.

`crates/lkjagent-xtask/` now gates the cutover crate set, validates the tiny
benchmark corpus, runs deterministic smoke replay, and collects bounded proof
bundles from the new store and workspace. Structure audit intentionally fails
with a not-yet-rebuilt message until a later stage restores that surface.

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

- The production LLM adapter inside `lkjagent-app` is not yet implemented.
- Structure audit is not yet rebuilt for the plan engine.
- The Aurora Ledger live proof is blocked before loop iteration one: the app
  daemon can idle in the container and deterministic replay passes, but the
  app daemon is not yet wired to call the configured endpoint. The next fix
  task is to add the production endpoint adapter to `lkjagent-app`, then run
  the task-17 proof loop from a fresh data directory.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist.
- Checked-in run logs can be failure fixtures without proving current success.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
