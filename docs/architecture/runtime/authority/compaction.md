# Compaction

## Purpose

Define runtime-owned compaction snapshots for hard context pressure and safe
resume after interruption.

## Contract

The model is not required to call `memory.save` to survive context pressure.
Runtime compaction snapshots preserve enough state for the next turn to resume
from durable authority data rather than stale transcript guesses.

## Snapshot Fields

A compaction snapshot records:

- snapshot id, case id, and creation timestamp.
- active mission, active mode, graph node, and graph phase.
- authority decision id and prompt frame fingerprint.
- required evidence, missing evidence, and completion gate state.
- active artifact id, artifact root, artifact cursor, and weak paths.
- write batch cursor and next planned write paths.
- last valid action and last valid observation.
- latest successful observation.
- pending fault class, failed action fingerprint, recovery route, and retry counters.
- admitted tools, blocked tools, and exact next valid action.
- compaction reason and context pressure summary.

## Resume Rule

After compaction, the prompt frame renders from the snapshot plus the next
runtime decision. The model never infers active mode, recovery route, missing
evidence, batch cursor, or completion state from old transcript text.

## Invariants

- Compaction can interrupt any mission only after recording a resume card.
- The next model turn has the same or narrower admitted tool surface.
- Recovery and write batch cursors survive compaction.
- Snapshot writes are runtime effects after a pure authority decision.
- Hard pressure never depends on a model-authored memory action.

## Failure Cases

- Compaction during recovery loses the current recovery ladder.
- The next turn repeats a stale invalid action after compaction.
- Snapshot lacks missing evidence or admitted next tools.
- Snapshot lacks artifact cursor or batch cursor data.
- Hard pressure blocks because graph policy refuses `memory.save`.

## Verification

Compaction tests assert snapshots contain case, artifact, evidence, recovery,
batch cursor, latest observation, and next action fields. Runtime tests assert
post-compaction turns resume from the snapshot instead of repeating stale
actions.

## Related Files

- [reducer.md](reducer.md)
- [missions.md](missions.md)
- [../../context/compaction.md](../../context/compaction.md)

## Status

partially implemented. Runtime notices and graph compaction rows preserve core
case, recovery, artifact, batch cursor, observation, and next-action fields.
Durable history beyond the latest notice and full status rendering remain open.
