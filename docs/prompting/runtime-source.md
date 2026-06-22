# Runtime Source

## Purpose

Define the data source for model-facing prompt frames when runtime authority is
active.

## Contract

Prompt text is a rendering of state. It is not a policy layer. The renderer
uses only facts owned by `ContextFrame` and the current `RuntimeDecision`.

```text
RuntimeDecision + ContextFrame -> PromptFrame
```

The decision contributes mission, admitted tools, blocked tools, missing
evidence, completion state, recovery plan, compaction requirement, maintenance
eligibility, exact valid example, and next executable action. The context frame
contributes objective, hard state, weighted tracks, guard tracks, selected
context slices, evidence owners, output grammar, artifact identifiers, weak
paths, repeated action signatures, and completion blockers.

## Required Rendering

Every endpoint prompt card names one active mission, the normalized owner
objective, the hard state, dominant guards, missing evidence, allowed tools,
blocked tools with reasons, the last failure when present, completion blockers,
and either one forced next action or a narrow normal-execution action set.

Recovery prompt cards render one copyable valid action example from the same
registry used by validation and dispatch. Completion-refusal prompt cards render
one next executable action that parses and is admitted by the current decision.
Compaction-resume prompt cards render the preserved mission and next action.

## Prohibited Sources

- raw owner text prefixes as policy.
- transcript position as permission.
- graph guidance as closure authority.
- hand-written examples that the dispatcher can reject.
- maintenance restrictions while an owner mission is active.
- owner completion blockers omitted from the model-facing card.

## Cache Shape

Stable identity, protocol, registry, and selected context package text stay
stable across turns. Volatile runtime facts live in the compact prompt card.

## Status

partially implemented.
