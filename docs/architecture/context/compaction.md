# Compaction

## Purpose

Specify the explicit event that keeps the single session alive forever:
distill the log into durable memory, rebuild the prefix, restart the log.
Compaction is the only moment the window shrinks.

## Trigger

The engine checks at every turn boundary: when the tracked window usage
reaches the trigger threshold ([budgets.md](budgets.md)), compaction runs
before anything else is delivered. The trigger is harness-owned; the model
neither requests nor refuses compaction.

## Procedure

1. Distillation turns. The harness injects a compaction notice directing the
   model to record what must survive: task state, open threads, fresh
   lessons. The model gets up to 4 turns of memory.save actions per
   [../memory/distillation.md](../memory/distillation.md). The final save
   must be a task summary entry when a task is open.
2. Digest rebuild. The harness rebuilds the memory digest from the memory
   store within its budget: top entries by rank, task summary first.
3. Prefix rebuild. A fresh prefix is assembled: identity, grammar and
   registry, skill index, workspace brief, new digest, per
   [../protocol/system-prompt.md](../protocol/system-prompt.md).
4. Log restart. The new log opens with one notice frame holding the task
   summary (or the maintenance state) so the model re-enters mid-stride.
5. Transcript record. One compaction event stores the before and after token
   counts and the ids of the memory rows written. The transcript itself is
   never compacted; only the window is.

The new window must land at or under the post-compaction target; if
distillation cannot fit the task summary inside the digest budget, the
compaction fails loudly as an error notice and the daemon pauses the task
for owner attention. Silent loss is forbidden by
[../../agent/honest-state.md](../../agent/honest-state.md).

## What Survives Where

| Content | Survives as |
| --- | --- |
| task state and open threads | task summary in digest plus log-head notice |
| lessons and discoveries | memory rows, retrievable by memory.find |
| loaded skill bodies | dropped; reload via skill.use if still needed |
| raw turn history | transcript only, never back into the window |

## Cost Model

Compaction restarts the endpoint prefix cache once, deliberately, and buys a
small window for thousands of cheap cached turns. Frequency falls out of the
budgets: bigger observations mean earlier triggers, which is the intended
pressure toward narrow tool use.

## Status

implemented.
