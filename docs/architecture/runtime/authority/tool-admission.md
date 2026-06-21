# Tool Admission

## Purpose

Define the runtime decision that admits or refuses a requested tool.

## Contract

```text
admit_tool(snapshot, requested_tool) -> ToolAdmission
```

`ToolAdmission` includes admitted flag, reason, active mission, required
evidence, missing evidence, next valid tools, exact valid example, and any
policy contradiction.

Tool admission is a runtime authority decision, not a graph-node side effect.
The graph may recommend a node, but authority decides whether the requested
tool can run in the current mission.

## Invariants

- Completion mode must not block `fs.read`, `doc.audit`, `artifact.audit`, or
  repair tools when content evidence is missing.
- Recovery must not block all mutation tools when the recovery objective is to
  write missing content.
- Missing verification must not force only verification tools when the artifact
  needed for verification does not exist.
- Parameter faults must produce one exact valid action and suppress unrelated
  graph noise.
- Repeat faults must pick a different action or create a partial handoff.

## Failure Cases

- Uploaded log case: `agent.done` is allowed while dictionary content is absent.
- Uploaded log case: recovery refuses `doc.scaffold` or `fs.write` needed for
  artifact repair.
- Uploaded log case: unknown `scale` in `artifact.apply` does not render one
  canonical valid example.
- Uploaded log case: nested `<path>` for `fs.read` or `fs.stat` is refused.

## Verification

Admission tests cover completion-node audit tools, recovery escape tools,
payload-too-large batch planning, parameter examples, and repeated-action
suppression.

## Related Files

- [reducer.md](reducer.md)
- [completion.md](completion.md)
- [../../action-reliability/README.md](../../action-reliability/README.md)
