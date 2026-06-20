# Compaction

## Purpose

Specify the explicit graph transition that keeps the single session alive:
preserve structured case state, distill reusable memory, rebuild the prefix,
and restart the log. Compaction is the only moment the window shrinks.

## Trigger

The engine checks pressure at safe turn boundaries: before owner delivery,
before endpoint calls, and after an action/observation pair completes. The
trigger is harness-owned; the model neither requests nor refuses compaction.

Pressure states:

- green: normal.
- yellow: projected usage crosses the soft trigger; observations should stay
  narrow and retrieval-oriented.
- orange: current usage is above the soft trigger; the harness schedules
  compaction at the next safe boundary.
- red: projected usage reaches the hard trigger or reserve limit; the
  harness compacts before owner delivery or endpoint calls.
- black-invalid: current usage already violates the policy; the daemon
  pauses with an explicit diagnostic instead of looping.

Compaction never runs mid-action. If a model action is pending, the harness
executes the action, appends exactly one observation, and only then compacts
at the next boundary.

## Procedure

1. Distillation turns. The harness injects a compaction notice directing the
   model to preserve reusable lessons through `memory.save`. The graph state
   itself is preserved by the harness through a typed `CompactionPlan`.
2. Digest rebuild. The harness rebuilds the memory digest from the memory
   store within its budget: top entries by rank, task summary first.
3. Prefix rebuild. A fresh prefix is assembled: identity, grammar and
   registry, graph state, workspace brief, new digest, per
   [../protocol/system-prompt.md](../protocol/system-prompt.md).
4. Log restart. The new log opens with one notice frame holding the task
   summary (or the maintenance state) so the model re-enters mid-stride.
5. Transcript record. One compaction event stores the before and after token
   counts, memory row ids, and the policy values used. The transcript itself
   is never compacted; only the window is.

The new window must land at or under the post-compaction target; if
distillation cannot fit the task summary inside the digest budget, the
compaction fails loudly as an error notice and the daemon pauses the task
for owner attention. Silent loss is forbidden by
[../../agent/honest-state.md](../../agent/honest-state.md).

## What Survives Where

| Content | Survives as |
| --- | --- |
| task state and open threads | graph case, graph evidence, and log-head notice |
| lessons and discoveries | memory rows, retrievable by memory.find |
| selected context packages | package identities in graph state; text reselected after rebuild |
| legal next transitions | graph case plus compaction snapshot |
| recovery ladder | fault rows and graph recovery state |
| raw turn history | transcript only, never back into the window |

## Cost Model

Compaction restarts the endpoint prefix cache once, deliberately, and buys a
small window for thousands of cheap cached turns. Frequency falls out of the
budgets: bigger observations mean earlier triggers, which is the intended
pressure toward narrow tool use.

## Status

implemented.
