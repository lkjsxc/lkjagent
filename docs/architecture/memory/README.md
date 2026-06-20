# Memory

## Purpose

This directory specifies persistence and recall: one SQLite store holding the
queue, graph cases, graph events, graph evidence, the append-only transcript,
distilled memory rows, and runtime state, with lexical retrieval over memory.
Decision:
[../../decisions/sqlite-store.md](../../decisions/sqlite-store.md).
Owned by the lkjagent-store crate.

## Table of Contents

- [store.md](store.md): schema, graph tables, the single file, and transaction rules.
- [transcripts.md](transcripts.md): event kinds, ordering, rendering, and reproducibility.
- [retrieval.md](retrieval.md): memory.find ranking and the digest builder.
- [distillation.md](distillation.md): when memory rows are written, their quality rules, and pruning.
- [maintenance-pruning.md](maintenance-pruning.md): idle pruning admission and no-op behavior.
- [semantic-merge.md](semantic-merge.md): non-exact merge evidence and source rows.
