# Tool Admission

## Purpose

Define the runtime decision that admits or refuses a requested tool.

## Contract

Tool admission is a runtime authority decision, not a graph-node side effect.
The graph may recommend a node, but authority decides whether the requested
tool can run in the current mission.

```text
admit_tool(decision, requested_tool) -> ToolAdmission
```

## Admission Shape

```text
ToolAdmission
- admitted
- reason
- active_mission
- required_evidence
- missing_evidence
- next_valid_tools
- exact_valid_example
- policy_contradiction
```

A refusal must name the failed gate and one precise repair path. Parameter
faults suppress unrelated graph context and render only the canonical example
for the failed tool.

## Mission Rules

- `CompletionGate` with missing artifact readiness admits `artifact.audit`,
  `doc.audit`, `artifact.next`, `fs.read`, `fs.tree`, `fs.write`, and
  `fs.batch_write`.
- `ArtifactContentRepair` admits `artifact.next`, `fs.batch_write`, `fs.write`,
  `fs.read`, `fs.read_many`, `artifact.audit`, and `doc.audit`.
- `BatchWriteRecovery` admits `artifact.next`, `fs.batch_write`, `fs.write`,
  `fs.read`, and `artifact.audit`.
- `ProtocolRecovery` preserves the previous mission escape tools.
- `IdleMaintenance` admits only maintenance effects and is preempted by any
  owner case, recovery fault, verification gap, or hard compaction.

## Invariants

- Completion mode must not block `fs.read`, `doc.audit`, `artifact.audit`, or
  repair tools when content evidence is missing.
- Recovery must not block all mutation tools when the recovery objective is to
  write missing content.
- Missing verification must not force only verification tools when the artifact
  needed for verification does not exist.
- Parameter faults must produce one exact valid action and suppress unrelated
  graph noise.
- Repeat faults must pick a different action, normalize to an accepted shape,
  switch to fallback, or create a partial handoff.

## Uploaded Expectations

- Early `agent.done` after planning is refused with missing structure and
  readiness evidence.
- Recovery from scaffold-only content admits batch and single-file writes.
- Invalid `fs.batch_write` syntax gets one canonical example, then fallback
  instead of the same invalid loop.
- Maintenance `memory.save` during owner work is refused and writes no row.

## Verification

Admission tests cover completion-node audit tools, recovery escape tools,
payload-too-large batch planning, parameter examples, repeated-action
suppression, and maintenance preemption.

## Related Files

- [reducer.md](reducer.md)
- [completion-policy.md](completion-policy.md)
- [../../action-reliability/README.md](../../action-reliability/README.md)
