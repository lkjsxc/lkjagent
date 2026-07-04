# lkjagent State Runtime Redesign Report

## Purpose

Provide a direct, implementation-ready handoff for the coding agent.

## Mission

Work on `lkjsxc/lkjagent`. Improve documentation first, then improve the
implementation to match the documentation. The target is a durable state-ledger
runtime for one owner, one local LLM, one workspace, and one SQLite store.

## Observed Repository State

Observed head commit: `99f13f0662d2dcd45624732fb6c8d490e01ca682`.
The repository already has a strong contract in `docs/` and a first pure
state-ledger domain slice in `lkjagent-core`. The current integrated daemon is
still plan-ledger shaped: `daemon.rs` hydrates `TaskSnapshot`, calls `next_work`,
renders prompts from task and step, calls the endpoint, dispatches effects, and
commits plan rows.

## High-Confidence Gaps

The store lacks the documented state-ledger table set. The daemon does not
persist `RuntimeDecision` before prompt rendering or endpoint calls. Prompt
rendering is keyed by `StepKind`, not decision context. The parser has a fixed
explore registry instead of a decision-specific `ToolSetView`. Context is not
yet durable source-tagged items with contamination and contradiction handling.
Proof bundles do not yet expose decisions, tool views, context conflicts,
contamination suppressions, and artifact fingerprints as first-class evidence.

## Design Direction

Use durable rows as the only runtime authority. Hydrate rows into a
`RuntimeSnapshot`. Use pure selectors to choose and persist one
`RuntimeDecision`. Render a `PromptFrame` from that decision. Parse model output
against the decision envelope and tool view. Persist admissions before effects.
Convert observations into events and patches. Close only with fresh checks tied
to current artifact fingerprints.

## 512-Token Policy

Adopt about 512 tokens as a target for project-authored files and as the default
model generation unit. Keep the existing 200-line hard cap. Do not force every
owner-requested final artifact file under 512 tokens. For long outputs, generate
small checked units, then deterministically assemble the requested final file
shape and run aggregate checks.

## Loose Timeout Policy

Set endpoint timeout defaults much higher than the current 60 seconds, around
900 seconds, while retaining environment and config overrides. Surface timeout,
request cap, retry delay, finish reason, closure mode, anomaly, exchange refs,
and decision id in exchange logs and status/proof output.

## Implementation Order

1. Update `docs/current-state.md` and add or refine docs for artifacts, decisions,
state events, tool catalog, context items, endpoint policy, recovery, and proof.
2. Add state-ledger store tables and row tests.
3. Add pure selectors and decision drafts.
4. Wire persisted decisions into the daemon loop.
5. Unify tool catalog, parser, admission, and dispatcher.
6. Persist context items and render context from decisions.
7. Add artifact units, assembly, and checks.
8. Update endpoint timeouts and outcome recording.
9. Update CLI status/proof surfaces.
10. Remove or rehome plan-only authority after state-ledger parity is proven.

## Critical Tests

Prove unknown state keys round-trip, selectors are deterministic, decision
fingerprints are stable, prompt-rendered tools match admission, unavailable tools
are not rendered, contaminated context is excluded, contradictions create
conflict cells, unfinished decisions resume before new work, endpoint timeouts
are configurable, and long artifacts close only after fresh checks.

## Required Gates

Run focused Cargo tests while working. Before final handoff run:
`cargo run -p lkjagent-xtask -- quiet verify` and
`docker compose run --rm verify`. Also run Docker `test`, `lint`, `bench`, and
`replay` when the touched behavior affects those surfaces. A gate that did not
run did not pass.

## Zip Package

The structured report package is `tmp/lkjagent-state-runtime-redesign-report.zip`.
If the repository contains `tmp/lkjagent-state-runtime-redesign-report.zip.base64`
instead of the zip, decode it with:
`base64 -d tmp/lkjagent-state-runtime-redesign-report.zip.base64 > tmp/lkjagent-state-runtime-redesign-report.zip`.

## Package Index

- `README.md`
- `architecture/artifact-units.md`
- `architecture/context-engine.md`
- `architecture/endpoint-policy.md`
- `architecture/proof-bundles.md`
- `architecture/runtime-loop.md`
- `architecture/state-ledger.md`
- `architecture/store-schema.md`
- `architecture/tool-catalog.md`
- `evidence/code-observations.md`
- `evidence/current-contract.md`
- `evidence/live-run-lessons.md`
- `evidence/repository-snapshot.md`
- `operations/commit-plan.md`
- `operations/handoff-template.md`
- `operations/subagent-briefs.md`
- `strategy/default-decisions.md`
- `strategy/desired-state.md`
- `strategy/question-defaults.md`
- `tasks/00-bootstrap-inventory.md`
- `tasks/01-doc-ledger.md`
- `tasks/02-store-state-schema.md`
- `tasks/03-runtime-selectors.md`
- `tasks/04-persisted-decision-loop.md`
- `tasks/05-tool-catalog-parser.md`
- `tasks/06-context-items.md`
- `tasks/07-artifact-generation.md`
- `tasks/08-endpoint-and-timeouts.md`
- `tasks/09-proof-status-cli.md`
- `tasks/10-cleanup-and-retirement.md`
- `verification/acceptance-rules.md`
- `verification/gate-commands.md`
- `verification/test-matrix.md`
