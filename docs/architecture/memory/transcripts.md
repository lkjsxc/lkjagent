# Transcripts

## Purpose

The transcript is the append-only event log in the store and the complete
account of agent behavior: if something is not in the transcript, it did
not happen. This file fixes the event kinds, their linkage to turns, and
the ordering guarantees every reader relies on.

## Event Kinds

| Kind | Written when |
| --- | --- |
| owner | a queue message is delivered to the loop |
| action | the model produces a parsed action for a turn |
| observation | a tool returns its bounded result |
| notice | the harness injects a frame: delivery, truncation, budget, distillation direction |
| queue_mutation | a queue row is enqueued, edited, tombstoned, or redelivered |
| compaction | a compaction completes: before and after token counts, memory row ids written |
| error | the harness records a fault: parse fault, tool failure, endpoint outage, internal error |

Each event stores its payload verbatim in content and its windowed token
count in tokens, per the schema in [store.md](store.md).

## Turn Linkage

Events written during a turn carry that turn. A normal turn produces one
action event and one observation event with the same turn number; owner and
notice events carry the turn at whose boundary they were injected. An
out-of-turn queue_mutation event has no turn. The
delivered_turn column of a queue row matches the turn of its owner event,
written in the same transaction.

## Ordering

- id is the total order of the transcript; readers sort by id alone.
- non-null turn numbers are nondecreasing along id.
- Rows are append-only: never updated, never deleted, never compacted.
  Compaction shrinks the window, not the transcript, per
  [../context/compaction.md](../context/compaction.md).

## Rendering

Store readers receive events in id order with kind, turn when present,
content, tokens, and created_at. The `lkjagent log` CLI surface is owned by
[../../product/observability.md](../../product/observability.md).

## Reproducibility

Recorded completions from action events become parser test fixtures: every
shape the endpoint ever produced can be replayed against the rules in
[../protocol/parsing.md](../protocol/parsing.md) as a table-driven test.
Everything observable is reconstructible from the store plus the skill
files.

## Status

implemented.
