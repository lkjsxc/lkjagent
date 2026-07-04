# Context

## Purpose

Define how prompts are assembled from durable, source-tagged context items and
runtime state.

## Table of Contents

- [context-items.md](context-items.md): durable source-tagged facts and prompt
  admission.
- [contradictions.md](contradictions.md): conflict detection and unresolved
  conflict rendering.
- [contamination.md](contamination.md): prompt exclusion rules for risky
  material.
- [prompt-assembly.md](prompt-assembly.md): prompt regions, budgets, and frame
  fingerprints.
- [budgets.md](budgets.md): token caps and estimator ownership.
- [task-brief.md](task-brief.md): rolling summary contract for the current plan
  family.

## Failure This Prevents

Context remains a bounded projection instead of a transcript replay that
reinforces failed outputs or contradictory facts.
