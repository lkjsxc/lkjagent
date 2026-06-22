# Prompt Frame

## Purpose

The prompt frame owns the compact state-derived context sent to the model
endpoint for each turn. The model proposes one action; the runtime validates,
authorizes, executes, observes, and reduces state.

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
documentation or artifact contract
growth stage
last successful action
last failed action
forbidden repeated action signatures
context slices
output grammar
completion blockers
next-action recommendation
```

## Rule

Owner text alone is never the prompt contract. The compiled frame must name the
current state, guards, missing evidence, and exact output grammar.

## Links

- State vector: [../state/state-vector.md](../state/state-vector.md).
- Generic wording: [generic-model-language.md](generic-model-language.md).

## Status

design-only
