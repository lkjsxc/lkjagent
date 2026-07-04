# AGENTS.md

## Purpose

Entry instructions for automated coding agents working on lkjagent.

## What lkjagent Is

lkjagent is a durable state-ledger agent daemon in Rust for one owner and one
local LLM. Owner messages become durable cases, events, state cells, runtime
decisions, bounded model asks, deterministic effects, and harness-computed
completion checks.

## Non-Negotiable Rules

1. `docs/` is the implementation contract. Update docs and implementation in
   the same change, including [docs/current-state.md](docs/current-state.md)
   when behavior moves.
2. Every authored file stays at or below 200 lines.
3. Docs use ASCII prose, kebab-case filenames, one H1, then a Purpose section.
4. Every docs directory has one README table of contents and at least two
   children.
5. No release shorthand, release labels, or migration framing.
6. Functional style: pure core, effects at the edges, no panic paths in product
   crates.
7. Honest state only: no fake success, placeholders, mocks as product behavior,
   or unrun gate claims.
8. Durable state rows and persisted `RuntimeDecision` rows are the single
   control plane. Do not add a second graph authority, MCP, runtime sub-agents,
   prompt-only policy, or dispatcher-only tool registry.
9. Completion is engine-computed through checks. The model never decides that a
   task is done.
10. Commit small coherent slices with `Tested` and `Not-tested` trailers that
    match commands actually run.
11. Use Docker Compose for final verification when a stage or behavior claims
    completion.

## Read Order

1. [docs/current-state.md](docs/current-state.md)
2. [docs/vision/README.md](docs/vision/README.md)
3. [docs/state/README.md](docs/state/README.md)
4. [docs/runtime/README.md](docs/runtime/README.md)
5. [docs/product/README.md](docs/product/README.md)
6. [docs/context/README.md](docs/context/README.md)
7. [docs/tools/README.md](docs/tools/README.md)
8. [docs/agent/README.md](docs/agent/README.md)
9. [docs/operations/verification.md](docs/operations/verification.md)

## Task Routing

When the owner names a task, do that task. Otherwise read
[docs/current-state.md](docs/current-state.md), find the first open item, and
execute the smallest coherent slice that moves it.

Before changing files, construct the case state: objective, constraints,
assumptions, risks, evidence requirements, candidate files, and next action.

## Verification

A gate that did not run did not pass. Use quiet gates for deterministic checks
and record exact command output in commits and handoffs.

## Handoff

Name what changed and why, docs updated, commands run with actual results,
commands not run with reasons, and the next executable step.
