# Graph Guidance

## Purpose

Define the compact graph state card rendered before endpoint actions.

## Card Shape

The card is rendered from structured state, not a model-written summary. It
includes:

- current state, case id, family, subroute, phase, and active node.
- objective, non-goals, constraints, assumptions, risks, and success criteria.
- active plan step, required evidence, and missing evidence.
- allowed tools, blocked tools, preferred next action, and legal transitions.
- selected packages with compression level.
- recent faults, recovery instruction, compaction instruction, and completion
  readiness.

Lists are bounded and stable. The card is a graph slice, not a dump of the
source graph.

## Weak Model Rules

The prefix reminds the model that the owner message is raw input, not the
plan. It tells the model to use graph state before mutation, prefer native
tools over shell, and treat completion by assertion as invalid.

## Status

implemented.
