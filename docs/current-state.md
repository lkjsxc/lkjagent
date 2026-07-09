# Current State

## Purpose

Separate observed behavior in this checkout from the target contract and the
work still required to make them agree.

## Evidence Boundary

The supplied product source is commit
`ae5ff551457adce869dee6159200c85a63aab3de`. The immutable rebuild packet is
anchored at `1b615a76c03dfd58dfd2986f017563bd6789e832`. Baseline raw evidence and
its reviewed receipt live under
`tmp/lkjagent-progress/nodes/baseline-capture/`.

The supplied SQLite store has three tasks, thirteen steps, ten decisions, and
six provider exchanges. The task states are closed, blocked, and closed. The
blocked world-history task has an active write asking for at least 1,500 words
at `./history_narrative.md`. Two decisions repeat that operation with the same
768-token output cap and endpoint-limit diagnosis. The store has zero tool
admissions, observations, artifacts, and workspace records for those matters.

The four tracked 900-second summaries say `ran` and `closed`, but the matching
live databases were not committed. They are bounded historical fixtures, not
proof that the underlying matters completed. A preserved transcript records an
`fs.tree` prefix error, but its live database is also absent.

Local Docker lint, test, and verify passed with the ignored working-copy
`Cargo.lock`. The packet clean-checkout script fails in an isolated worktree
because that lockfile is not tracked. Those local passes are not clean-checkout
evidence.

The configuration loader rejects nested objects but still accepts unknown keys
and scalar arrays, implements only a subset of the anchored registry, and does
not enforce all type, range, or cross-key guards. The tracked complete example
is absent. Repository-determinism remains open until its failing regressions,
locked source gate, and clean archive Docker run pass.

A fresh model-free diary run writes a journal entry containing canned
missing-detail text after the owner asks for a diary entry. This is reproduced
failure evidence, not acceptable record behavior.

## Baseline Failures

| Failure | Status | Raw evidence |
| --- | --- | --- |
| repeated impossible history request | reproduced | `raw/12-sqlite-facts.tsv`, `raw/13-request-facts.tsv` |
| empty effect and workspace lineage | reproduced | `raw/12-sqlite-facts.tsv` |
| clean checkout lacks tracked lockfile | reproduced | `raw/31-clean-checkout.log` |
| diary writes canned missing details | reproduced | `raw/17-diary-run-once.log`, `raw/18-diary-after-run-manifest.tsv` |
| live and readiness closure | bounded | `raw/14-live-summary-facts.tsv` |
| relative-root tree failure | bounded | `raw/19-relative-root-historical.log` |

All paths are relative to
`tmp/lkjagent-progress/nodes/baseline-capture/`. Bounded means the preserved
summary or transcript exists but its matching live database does not.

## Current Implementation

The checkout contains useful state cells, state edges, runtime decisions,
context rows, prompt cards, tool admissions, observations, checks, artifacts,
workspace records, and TUI reducers. These rows and pure helpers are inputs to
the rebuild, not proof that the native authority loop is complete.

Production still hydrates `TaskSnapshot`, task rows, step rows, fixed templates,
and bridge projections. Context may be prepared before the final decision.
Generic parsed messages and finish actions can settle steps without objective
effect evidence. Recovery can repeat the same impossible model call. The live
runner can count idle polls and replace the last real state with a synthetic
closed snapshot.

Workspace behavior is split across record commands, template writes, direct
filesystem tools, and native text effects. They do not share one transaction,
idempotency, indexing, or recovery contract. Startup eagerly creates empty
navigation. Retrieval does not yet search and rank the full visible workspace.

The TUI derives ordinary conversation from queue and selected event rows rather
than one canonical conversation table. Historical reducer tests do not replace
a final PTY campaign against frozen source.

## Target Authority

The target pure core is:

```text
RuntimeSnapshot + RuntimeEvent + CurrentTime -> RuntimeState
RuntimeState + Policy + CurrentTime -> RuntimeDecision
```

The effects edge is:

```text
RuntimeDecision -> EffectResult
EffectResult -> RuntimeEvent
```

Durable events reduce to concurrent state cells. A selector persists one
decision before prompt compilation or a native effect. The same row owns
context, tool view, grammar, budgets, recovery, checks, and exit predicates.
The model may author bounded content or propose an admitted operation; it never
chooses completion.

The target store and constraints are defined in
[store/schema.md](store/schema.md) and
[store/schema-constraints.md](store/schema-constraints.md). The runtime and
effects transaction is defined in
[store/effect-journal.md](store/effect-journal.md). Completion requires fresh
checks for every required obligation and no unresolved blocking operation.

## Removal Map

| Current authority | Target replacement | Status |
| --- | --- | --- |
| `TaskSnapshot` hydration | event-reduced `RuntimeState` | open |
| task and step tables | matters, obligations, operations, and events | open |
| fixed template rows | reducer-created obligations and operations | open |
| bridge projections | direct native store writes | open |
| plan-line model protocol | admitted operation envelopes | open |
| generic finish shortcut | harness checks and exit predicates | open |
| synthetic idle task | daemon quiescence with wake conditions | open |
| split workspace writers | one effect journal and path service | open |
| synthesized TUI transcript | canonical conversation messages | open |

These surfaces may be read only by an offline fixture converter while focused
regressions are extracted. Production selection, rendering, admission, effects,
recovery, and completion must stop reading them.

## Open Program

The dependency order is repository determinism and evaluation fixtures, native
store identity, event reduction, workspace transactions, native selection,
context and protocol cutover, recovery continuity, retrieval, canonical TUI,
experiments, cleanup, source freeze, and final live and PTY evidence.

No downstream item is complete merely because a narrow test or historical log
passes. Each named workgraph node requires its locked Docker gate, raw output,
machine-readable result, separate review, and hash-bound receipt.

## Contract To Source Gaps

| Contract | Current source | First proving node |
| --- | --- | --- |
| native fresh store | `crates/lkjagent-store/src/plan_*` | store-foundation |
| event reducer | `crates/lkjagent-core/src/engine*` | event-reducer |
| decision-first loop | `crates/lkjagent-app/src/runtime_bridge.rs` | selector-runtime-cutover |
| workspace transaction | split app and effects writers | workspace-foundation |
| canonical transcript | queue and event projections | tui-core |
| final campaigns | summary-oriented xtask runners | live-acceptance |

## Honesty Rules

- Target prose is not an implementation claim.
- A historical summary is not current raw evidence.
- Missing raw data creates a bound, not a pass.
- A command that did not run did not pass.
- Completion belongs to checks computed from durable facts.
