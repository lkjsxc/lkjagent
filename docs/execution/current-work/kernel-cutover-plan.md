# Kernel Cutover Plan

## Purpose

Track the remaining work that removes split runtime authority and routes every
daemon path through one persisted transition kernel.

## Objective

The runtime reads durable facts, builds a `RuntimeSnapshot`, records a
`RuntimeEvent`, reduces to one `RuntimeDecision`, persists the decision, then
renders a prompt frame or executes a runtime effect. Provider calls, parse
faults, admissions, dispatch, observations, compaction, maintenance, artifact
repair, and completion all cite the same decision chain.

## Inputs

- `data/logs/current-model-run.md` and `data/logs/index.ndjson`.
- Runtime authority docs under `docs/architecture/runtime/authority/`.
- Artifact contracts under `docs/architecture/artifacts/`.
- Store ledgers under `crates/lkjagent-store/src/runtime_authority/`.
- Runtime kernel, daemon loop, dispatch, provider, artifact, and model-log code.

## Outputs

- Store APIs reconstruct snapshot, event, decision, prompt frame, admission,
  effect, observation, and provider exchange by case id and decision id.
- The daemon uses one effectful kernel driver for owner delivery, provider
  turns, pending actions, maintenance, compaction, recovery, artifact repair,
  and close attempts.
- Admission is decision-derived, stale-safe, repeat-safe, and records refusals
  before adapters run.
- Prompts and model-log exports render persisted decision ids, staleness
  fingerprints, admitted tools, blocked tools, missing evidence, artifact
  cursors, recovery route, and exact next actions.
- Long content artifacts proceed through semantic batches, audit-owned
  evidence, weak-path repair, and the central completion gate.

## Invariants

- The pure reducer performs no I/O and emits one decision for one event.
- A model-call decision has a non-empty admitted tool surface.
- Empty tool surfaces occur only for deterministic effects, close, owner wait,
  or closed idle.
- A refused or stale action never reaches a tool adapter.
- Maintenance starts only from closed idle and yields before provider or
  dispatch when owner work exists.
- `agent.done` closes only through the artifact-aware completion reducer.
- Direct graph notes cannot satisfy audit-owned `document-structure` or
  `artifact-readiness` evidence.

## Candidate Files

- `crates/lkjagent-runtime/src/kernel/`
- `crates/lkjagent-runtime/src/kernel_driver/`
- `crates/lkjagent-runtime/src/daemon/`
- `crates/lkjagent-store/src/runtime_authority/`
- `crates/lkjagent-tools/src/fs_batch/`
- `crates/lkjagent-tools/src/doc/`
- `crates/lkjagent-benchmark/`

## Verification

Focused slices name their crate tests in the blocker queue. The final proof is
`cargo fmt --check`, focused runtime, store, and tools tests,
`cargo run -p lkjagent-xtask -- quiet verify`, and
`docker compose run --rm verify`.

## Status

implemented.
