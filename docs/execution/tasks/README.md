# Tasks

## Purpose

One executable task file per implementation slice, written so a session can
start from the file alone. The queue order lives in
[../current-blockers.md](../current-blockers.md); this README owns the
shared template.

## Template

Every task file uses exactly these headings:

```
# <Task Title>

## Purpose
## Status
## Depends On
## Files To Read
## Files To Touch
## Focused Gate
## Acceptance
## Must Not
```

- Status is one line: open, in progress with the blocking question, or done
  with the closing commit.
- Files To Read lists contracts in reading order; Files To Touch lists
  expected paths, marking new ones.
- Focused Gate is the exact command list run during the work.
- Acceptance is the evidence that closes the task: commands plus the output
  facts that must hold.
- Must Not carries the task-specific traps.

## Table of Contents

### Active Redesign Queue

- [structural-truth-sweep.md](structural-truth-sweep.md): reconcile documentation status.
- [cli-contract-redesign.md](cli-contract-redesign.md): design the CLI contract.
- [token-aggregate-ledger.md](token-aggregate-ledger.md): implement cumulative token usage.
- [cli-core-redesign.md](cli-core-redesign.md): implement parser, help, and dispatch shape.
- [observability-render-redesign.md](observability-render-redesign.md): share status, log, and console decks.
- [resolver-table-totality.md](resolver-table-totality.md): remove fallback-shaped resolver logic.
- [content-atom-graph.md](content-atom-graph.md): add general work-product atoms.
- [manuscript-compose.md](manuscript-compose.md): implement staged manuscript assembly.
- [smoke-harness.md](smoke-harness.md): add replay and smoke harnesses.
- [final-redesign-gates.md](final-redesign-gates.md): run final gates and handoff.
- [large-artifact-engine.md](large-artifact-engine.md): durable atom graph engine for long artifacts.
- [live-manuscript-proof.md](live-manuscript-proof.md): live daemon proof boundary for a 10,000-word manuscript.

### Historical Task Contracts

- [bootstrap-workspace.md](bootstrap-workspace.md): create the cargo workspace and image skeleton.
- [xtask-checks.md](xtask-checks.md): build the gates that enforce the repository rules.
- [protocol-parser.md](protocol-parser.md): the action grammar parser and renderer.
- [context-engine.md](context-engine.md): budgets, admission, accounting, compaction decisions.
- [sqlite-store.md](sqlite-store.md): queue, events, memory, state, retrieval.
- [llm-client.md](llm-client.md): the chat-completions client and backoff.
- [state-graph-runtime.md](state-graph-runtime.md): graph model, routing, evidence gates.
- [tool-runtime.md](tool-runtime.md): dispatch and the registry-backed tool adapters.
- [agent-loop.md](agent-loop.md): the turn loop composing every crate.
- [queue-cli.md](queue-cli.md): the lkjagent binary and its commands.
- [self-maintenance.md](self-maintenance.md): explicit maintenance directives and
  their bounds.
- [compose-final-gate.md](compose-final-gate.md): Dockerfile, compose services, CI.
- [deep-redesign-truth-sweep.md](deep-redesign-truth-sweep.md): reconcile active
  failure evidence and stale roots.
- [deep-redesign-compact-context.md](deep-redesign-compact-context.md): compact
  prompt context and line-protocol-only batches.
- [deep-redesign-output-budget.md](deep-redesign-output-budget.md): compact max
  output budget and oversize recovery.
- [deep-redesign-short-paths.md](deep-redesign-short-paths.md): short semantic
  artifact aliases.
- [deep-redesign-exact-examples.md](deep-redesign-exact-examples.md):
  registry-derived exact action examples.
- [deep-redesign-runtime-authority.md](deep-redesign-runtime-authority.md):
  persisted decision prompt cards and admission.
- [deep-redesign-artifact-batches.md](deep-redesign-artifact-batches.md):
  path-specific artifact micro-batches.
- [deep-redesign-completion-maintenance.md](deep-redesign-completion-maintenance.md):
  completion and no-op maintenance reducers.
- [deep-redesign-provider-handoff.md](deep-redesign-provider-handoff.md): provider
  anomaly retry and blocked handoff.
- [deep-redesign-gates.md](deep-redesign-gates.md): benchmark corpus and final
  verification gates.
- [obligation-network-redesign.md](obligation-network-redesign.md): fact,
  obligation, resolver, and progress-key runtime redesign.
- [runtime-smoke-problem-sweep.md](runtime-smoke-problem-sweep.md): live-smoke
  false-close and noisy-loop repairs.
- [dense-runtime-state-network.md](dense-runtime-state-network.md): dense deterministic runtime authority network.
- [story-manuscript-generation-gap.md](story-manuscript-generation-gap.md): manuscript-scale story generation gap.
