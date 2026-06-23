# Missions

## Purpose

Define the mission model used by runtime authority.

## Contract

The authority reducer selects exactly one `RuntimeMission` for each
`RuntimeSnapshot` and `RuntimeEvent`. The persisted mission string is the
stable contract consumed by prompt cards, dispatch admission, status, tests,
and replay.

## Mission Set

- `hard_runtime_compaction`: runtime-owned snapshot and prompt rebuild under
  hard context pressure.
- `owner_recovery`: unresolved owner-case fault recovery when the fault is not
  a parse or schema fault.
- `schema_repair`: parser or schema recovery that produces one valid model
  action example.
- `artifact_repair`: artifact ledger adoption, audit, weak-path repair, or
  batch cursor work.
- `verification_repair`: failed verification or missing verification evidence
  repair.
- `owner_execution`: normal owner-case planning, context, execution,
  observation, and evidence collection.
- `owner_verification`: requested verification gates when repair is not the
  active need.
- `owner_completion`: close gating and refusal when completion is requested or
  all close inputs are ready.
- `idle_maintenance`: bounded maintenance when no owner, recovery,
  verification, artifact, or compaction work exists.
- `closed_idle`: no endpoint call and no effect besides waiting for new input.

## Invariants

- Mission priority is the table in [mode-priority.md](mode-priority.md).
- Active mode is derived from the mission and is not a second selector.
- Owner work preempts maintenance.
- Hard compaction can interrupt any mission only with a resumable snapshot.
- Verification can return to repair when inspected content is missing.
- Recovery cannot narrow the tool set to zero when a legal escape exists.

## Failure Cases

- Maintenance mutates or closes an owner task.
- Verification blocks repair after proving content is missing.
- Completion mode hides required audit tools.
- Repeat protection causes repeated `graph.state` loops.
- Schema repair renders an example that parser or dispatcher later rejects.

## Verification

Mission tests cover priority, legal transitions, admitted tool classes, and
evidence required before leaving each mission. The tests assert persisted
mission strings, not only rendered prose.

## Related Files

- [mode-priority.md](mode-priority.md)
- [tool-admission.md](tool-admission.md)
- [maintenance.md](maintenance.md)
- [compaction.md](compaction.md)
