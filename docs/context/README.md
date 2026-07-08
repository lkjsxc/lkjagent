# Context

## Purpose

Define how prompts are assembled from durable, source-tagged context items,
workspace evidence, and runtime state.

## Table of Contents

- [context-items.md](context-items.md): durable source-tagged facts and prompt
  admission.
- [contradictions.md](contradictions.md): conflict detection and unresolved
  conflict rendering.
- [contamination.md](contamination.md): prompt exclusion rules for risky
  material.
- [prompt-assembly.md](prompt-assembly.md): prompt regions, budgets, and frame
  fingerprints.
- [prompt-kernel.md](prompt-kernel.md): ordered prompt cards, profiles, and
  section fingerprints.
- [budgets.md](budgets.md): token caps and estimator ownership.
- [matter-brief.md](matter-brief.md): rolling summary contract for the current
  matter and workspace evidence.

## Model-Visible Shape

Context renders as compact XML-like cards with source handles, lane names, and
fingerprints. It does not dump JSON blobs, whole transcripts, raw failed model
output, duplicate objective text, or unbounded workspace files. Flat JSON may
exist only in data configuration or internal exchange storage; model context uses
source-linked prose and attribute-less XML-like cards.

## Failure This Prevents

Context remains a bounded projection instead of a transcript replay that
reinforces failed outputs or contradictory facts.
