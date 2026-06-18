# Distillation

## Purpose

When and how memory rows are written: the three moments the harness directs
memory.save, the quality rules every entry must meet, the digest rendering
rules, and pruning. Distillation is what makes a small window survivable:
the transcript remembers everything, memory remembers what matters.

## The Three Moments

Memory rows are written at three moments:

1. Task close. After agent.done, the harness saves the done summary as a
   task-summary row and records the memory id in the transcript.
2. Compaction. The compaction notice opens up to 4 turns of memory.save
   per [../context/compaction.md](../context/compaction.md); the final
   save must be a task-summary row when a task is open.
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

## Digest Rendering

The memory digest in the prefix renders selected entries in rank order,
task summary always first, within the 2,048-token budget from
[../context/budgets.md](../context/budgets.md). Selection rules are owned
by [retrieval.md](retrieval.md); the builder must fit whole entries inside
the cap.

## Pruning

The prune-memory maintenance directive
([../runtime/self-maintenance.md](../runtime/self-maintenance.md)) keeps
the memory table sharp:

- merge duplicate entries into one,
- rewrite vague entries until they stand alone,
- delete superseded rows.

memory rows may be updated and deleted; events rows may not. The
distinction is fixed in [store.md](store.md).

## Status

Implemented for memory storage APIs, digest selection, harness-written
task-summary rows on task close, compaction distillation prompts, and the
explicit maintenance distill directive. The model can write durable rows
through memory.save during prompted compaction or explicit maintenance.
