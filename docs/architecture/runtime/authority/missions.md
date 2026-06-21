# Missions

## Purpose

Define the mission model used by runtime authority.

## Contract

The authority reducer selects exactly one mission: `OwnerWork`, `Recovery`,
`Verification`, `Maintenance`, `Compaction`, or `Idle`.

`OwnerWork` owns open owner tasks. It admits planning, observation, mutation,
artifact tools, verification, and recovery tools as required by evidence gaps.

`Recovery` owns parse, parameter, runtime, repeat, policy, payload,
verification, completion, compaction, and maintenance faults. It admits the
smallest exact tool set needed to escape the current fault.

`Verification` owns missing completion evidence. It admits read-only
inspection, artifact audit, document audit, focused verification, and repair
transition when evidence proves the artifact is incomplete.

`Maintenance` owns idle-only pruning and policy improvement. It yields when
owner work, recovery, pending artifacts, verification, or compaction exists.

`Compaction` owns hard context pressure and can interrupt any mission. It must
preserve the current mission and enough state to resume.

`Idle` waits for queue input or maintenance eligibility and emits no endpoint
action.

## Invariants

- Owner work preempts maintenance.
- Compaction can interrupt any mission only with a resumable snapshot.
- Verification can return to repair when inspected content is missing.
- Recovery cannot narrow the tool set to zero when a legal escape exists.

## Failure Cases

- Maintenance mutates or closes an owner task.
- Verification blocks repair after proving content is missing.
- Completion mode hides required audit tools.
- Repeat protection causes repeated `graph.state` loops.

## Verification

Mission tests cover preemption, legal transitions, admitted tool classes, and
evidence required before leaving each mission.

## Related Files

- [tool-admission.md](tool-admission.md)
- [maintenance.md](maintenance.md)
- [compaction.md](compaction.md)
