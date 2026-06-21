# Maintenance

## Purpose

Define maintenance as an idle-only, preemptable mission.

## Contract

Maintenance is eligible only when the queue is empty, no owner case is open,
no recovery ladder exists, no artifact is incomplete, no hard compaction is
pending, and no verification gate is pending.

Maintenance may prune memory, merge repeated lessons, and improve graph policy
inside bounded work units. It must never complete or mutate an owner task.

## Invariants

- Owner work yields maintenance before the next endpoint turn.
- Maintenance writes no duplicate no-op memory rows.
- Repeated maintenance actions stop as bounded no-op outcomes.
- Maintenance cannot call `agent.done` for owner cases.

## Failure Cases

- Maintenance runs while a bread dictionary case is in recovery.
- Maintenance saves repeated empty-cycle memories.
- Maintenance closes or changes an owner artifact.
- Graph policy and maintenance policy both reject each other's next action.

## Verification

Tests assert maintenance yields to queue rows, open cases, recovery ladders,
pending artifacts, verification gates, and hard compaction. Memory tests assert
duplicate no-op lessons merge or are skipped.

## Related Files

- [missions.md](missions.md)
- [compaction.md](compaction.md)
- [../../memory/maintenance-pruning.md](../../memory/maintenance-pruning.md)
