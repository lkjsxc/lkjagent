# Compaction

## Goal

Reduce prompt cost without erasing durable evidence or creating a second truth.

## Matter Capsules

After a threshold, create a short capsule containing objective, current
obligations, decisions, verified outputs, unresolved questions, and source refs.
Original events remain durable. The capsule is a derived candidate with its own
fingerprint and provenance edges.

## Hierarchical Workspace Summaries

Directory README pages summarize only direct children. Topic and project
summaries point to deeper source files. Retrieval descends only into relevant
branches.

## Invalidation

Changing a referenced source marks the capsule stale. Selection either rebuilds
it deterministically, schedules a bounded model summary, or uses direct sources.

## Deadlock Prevention

Compaction is never required to save a memory before normal work can continue.
If compaction fails, the selector can use a smaller direct evidence set and
record maintenance debt.

## Quality Check

Compare capsule answers with direct-source answers on a recall suite. Adopt only
when key obligations and source refs remain intact.
