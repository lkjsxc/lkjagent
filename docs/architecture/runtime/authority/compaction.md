# Compaction

## Purpose

Define runtime-owned compaction snapshots for hard context pressure.

## Contract

The model may not be required to call `memory.save` to survive context
pressure. Runtime compaction snapshots preserve active case, objective, active
mission, graph node and phase, required evidence, missing evidence, active
artifact id, artifact root, write batch cursor, last successful observation,
last failed action, recovery ladder step, admitted next tools, exact next valid
action, and completion blocked reasons.

The snapshot is rendered as structured data for the next endpoint turn, status,
logs, and model handoff output.

## Invariants

- Compaction can interrupt any mission only after recording a resume card.
- The next model turn must not need stale transcript guesses to continue.
- Recovery and write batch cursors survive compaction.
- Snapshot writes are runtime effects after a pure authority decision.

## Failure Cases

- Compaction during recovery loses the current recovery ladder.
- The next turn repeats a stale invalid action after compaction.
- Snapshot lacks missing evidence or admitted next tools.
- Hard pressure blocks because graph policy refuses `memory.save`.

## Verification

Compaction tests assert snapshots contain case, artifact, evidence, recovery,
batch cursor, and next action fields. Runtime tests assert post-compaction
turns resume from the snapshot instead of repeating stale actions.

## Related Files

- [reducer.md](reducer.md)
- [missions.md](missions.md)
- [../../context/compaction.md](../../context/compaction.md)
