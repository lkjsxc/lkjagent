# Prompt Frame

## Purpose

The prompt frame owns the model-facing rendering compiled from a context frame.
The model proposes one action; the runtime validates, authorizes, executes,
observes, and reduces state.

## Pipeline

```text
CaseState -> ContextFrame
RuntimeDecision + ContextFrame -> PromptFrame -> model turn
```

The context frame supplies state facts. The runtime decision supplies mission,
admission, completion, recovery, maintenance, and compaction authority. The
prompt frame selects the prompt mode and renders only the fields needed for the
next action.

## Required Fields

```text
case id
owner objective
normalized objective
hard state
active weighted tracks
dominant guards
allowed tools
blocked tools
required evidence
missing evidence
forbidden repeated action signatures
context slices
output grammar
completion blockers
next-action recommendation
prompt mode
```

## Prompt Modes

The compiler maps hard state and guard tracks to intake, semantic seed,
controlled expansion, audit, repair, recovery, maintenance, or verification
modes. Parser-recovery and repeated-action guards force recovery mode before
normal hard-state mapping.

## Rule

Owner text alone is never the prompt contract. The compiled frame must name the
current state, guards, missing evidence, selected context slices, and exact
output grammar.

## Links

- Context frame: [../architecture/context/context-frame.md](../architecture/context/context-frame.md).
- Runtime source: [runtime-source.md](runtime-source.md).
- State vector: [../state/state-vector.md](../state/state-vector.md).
- Generic wording: [generic-model-language.md](generic-model-language.md).

## Status

implemented
