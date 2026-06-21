# Schema Repair

## Purpose

Define deterministic repair for action parse faults, parameter faults, invalid
kinds, duplicate reads, and repeated invalid actions.

## Contract

Fault classes are `ParseFault`, `ParameterFault`, `ToolRuntimeFault`,
`RepeatFault`, `PolicyContradiction`, `PayloadTooLarge`,
`VerificationMismatch`, `CompletionRefused`, `CompactionPressure`, and
`MaintenanceConflict`.

Each fault class has a finite ladder. A parameter fault renders the exact
schema for the failed tool, retries once through canonical parameters when the
tool is still needed, then chooses an alternate observation or emits a policy
contradiction. A payload fault never retries raw `fs.write`; it routes to
artifact planning, `artifact.next`, or bounded `fs.batch_write`.

## Invariants

- A refusal shows exactly one canonical valid action example.
- The example must parse, validate, and pass the same dispatcher admission.
- Invalid `graph.evidence` kinds list allowed evidence requirements.
- Unknown `scale` in `artifact.apply` names accepted parameters.
- Repeated invalid `graph.state` must select a different next action.
- Nested `<path>` child parameters are valid for `fs.read` and `fs.stat`.

## Failure Cases

- Invalid `graph.evidence kind=evidence` loops into generic graph state.
- Unknown `scale` in `artifact.apply` renders multiple conflicting examples.
- Repeated `graph.state` survives repeat protection unchanged.
- `fs.read` or `fs.stat` rejects a child `<path>` parameter.
- Large payload recovery suggests another raw large write.

## Verification

Focused tests cover child tags for `fs.read`, `fs.stat`, and `fs.list`,
allowed evidence kinds, unknown parameter rejection, exact example rendering,
no repeated invalid example loop, and payload-to-batch recovery.

## Related Files

- [parameter-contract.md](parameter-contract.md)
- [recovery.md](recovery.md)
- [error-messages.md](error-messages.md)
