# Context Frame

## Purpose

The context frame is the deterministic runtime object compiled before a model
turn. It is not a transcript slice; it is the compact decision frame that tells
the model what objective, state, guards, tools, evidence, and grammar govern
the next action.

## Required Inputs

The compiler reads case state only: objective, hard state, weighted tracks,
evidence gates, repeated action signatures, and fault-derived guards. Effects
such as filesystem, queue, model, memory, shell, and clock access stay outside
the pure compiler.

## Required Fields

The frame includes owner raw input, normalized objective, objective contract,
optional documentation or artifact contract, hard state, ranked weighted
tracks, dominant tracks, guard tracks, allowed and blocked tools, required and
missing evidence, evidence owners, forbidden repeated action signatures,
selected context slices, tool schema slice, output grammar, completion
blockers, and the recommended next action.

## Selection Rules

- Parser recovery selects action grammar, tool schemas, and last parser faults.
- Artifact drift selects owner objective, artifact contract, and drifted paths.
- Document structure, relation, and mock-content risk select topology,
  relation graph, and audit failure slices.
- Model-name risk selects sanitizer context and raw fixture pointers.
- Context pressure selects budget and post-compaction consistency slices.

## Prompt Boundary

Prompt frames are compiled from context frames. A prompt frame may render a
smaller view, but it must not invent fields that the context frame did not
own.

## Status

implemented
