# Distillation

## Purpose

When and how memory rows are written: the three moments the harness directs
memory.save, the quality rules every entry must meet, the digest rendering
rules, and pruning. Distillation is what makes a small window survivable:
the transcript remembers everything, memory remembers what matters.

## The Three Moments

Memory rows are written by the model via memory.save at three moments:

1. Task close. After agent.done, the harness injects a distillation notice
   and allows up to 2 turns of memory.save actions: lessons, facts, and
   incidents worth keeping from the closed task.
2. Compaction. The compaction notice opens up to 4 turns of memory.save
   per [../context/compaction.md](../context/compaction.md); the final
   save must be a task-summary row when a task is open.
3. Maintenance. The distill directive of a maintenance cycle
   ([../runtime/self-maintenance.md](../runtime/self-maintenance.md))
   reads recent transcript spans and writes durable lessons.

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

design-only. Store APIs for memory writes, edits, deletes, and digest
selection are implemented; runtime distillation turns are not.
