# Distillation

## Purpose

When and how memory rows are written, deduplicated, rendered, and pruned.
Distillation is what makes a small window survivable: the transcript remembers
everything, memory remembers what matters.

## Write Moments

Memory rows are written at controlled moments:

1. Task close. After agent.done, the harness saves the done summary as a
   task-summary row and records the memory id in the transcript.
2. Compaction. Hard compaction is runtime-owned and preserves structured case
   state without requiring model-authored `memory.save`.
3. Explicit maintenance. The distill directive
   ([../runtime/self-maintenance.md](../runtime/self-maintenance.md)) can
   read recent transcript spans and write durable lessons when invoked.

## Entry Quality Rules

- Each entry stands alone: it must make sense without its transcript.
- Lessons use imperative voice: what to do, not what happened.
- At most about 200 tokens per entry.
- Titles are searchable noun phrases, because recall is lexical per
  [retrieval.md](retrieval.md).
- Tags name the files, tools, and subsystems touched.
- Accepted kinds are lesson, fact, task-summary, and incident.
- Writes are idempotent. Equivalent rows are skipped, updated, or merged
  instead of inserted again.

## Digest Rendering

The memory digest in the prefix renders selected entries in rank order,
task summary always first, within the 2,048-token budget from
[../context/budgets.md](../context/budgets.md). Selection rules are owned
by [retrieval.md](retrieval.md); the builder must fit whole entries inside
the cap during normal operation and only truncates a first oversized entry
so daemon startup remains recoverable.

## Pruning

The prune-memory maintenance directive
([../runtime/self-maintenance.md](../runtime/self-maintenance.md)) keeps
the memory table sharp:

- merge duplicate entries into one,
- rewrite vague entries until they stand alone,
- delete superseded rows.

Memory rows may be updated and deleted; events rows may not. The distinction
is fixed in [store.md](store.md). Pruning records the actual merge, rewrite,
or delete effect.

## Status

Partially implemented. Storage APIs, digest selection, task-summary rows,
idempotent `memory.save`, punctuation-safe `memory.find`, exact duplicate
`memory.prune`, maintenance distillation, and runtime-owned hard compaction
exist. Richer structured compaction snapshots plus semantic merge/rewrite
pruning remain open.
