# Memory

## Purpose

This directory specifies persistence and recall: one SQLite store holding
the queue, the append-only transcript, distilled memory rows, and runtime
state, with lexical retrieval over the memory rows. Decision:
[../../decisions/sqlite-store.md](../../decisions/sqlite-store.md).
Owned by the lkjagent-store crate.

## Table of Contents

- [store.md](store.md): the schema, the single file, and the transaction rules.
- [transcripts.md](transcripts.md): event kinds, ordering, rendering, and reproducibility.
- [retrieval.md](retrieval.md): memory.find ranking and the digest builder.
- [distillation.md](distillation.md): when memory rows are written, their quality rules, and pruning.
