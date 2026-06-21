# State Graph

## Purpose

This directory specifies the typed workflow graph that governs lkjagent
cognition, task execution, context selection, evidence, compaction, recovery,
completion, and maintenance. The graph is the central runtime model. It is
owned by the lkjagent-graph crate and interpreted by the runtime, store,
context engine, and tools.

## Table of Contents

- [model.md](model.md): node, edge, policy, context, evidence, and case data types.
- [transitions.md](transitions.md): legal state movement and admission rules.
- [context-packages.md](context-packages.md): graph-selected context package rendering.
- [task-state.md](task-state.md): active case state, planning fields, and progress ledger.
- [compaction.md](compaction.md): graph-aware pressure and preservation rules.
- [completion.md](completion.md): evidence-gated closure.
- [policy.md](policy.md): graph policy fields and enforcement boundary.
- [escape-hatches.md](escape-hatches.md): deterministic runtime correction for policy contradictions.
- [tool-affordances.md](tool-affordances.md): allowed tools by phase and node.
- [maintenance.md](maintenance.md): idle graph evolution and policy cleanup.
- [recovery-ladders.md](recovery-ladders.md): fault routing and alternate actions.
- [graph-guidance.md](graph-guidance.md): endpoint-visible graph state card.
- [document-construction.md](document-construction.md): document topology nodes and gates.
