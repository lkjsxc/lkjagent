# Checks Gated Completion

## Purpose

Record the decision that checks close steps and tasks.

## Context

Owner-visible completion must be backed by evidence over real workspace state.

## Decision

A task reaches `closed` only when task-level checks pass and check rows are
recorded. The model may write summaries, but it cannot decide completion.

## Consequences

Benchmarks, replay, proof bundles, status, and task closure all read the same
check results. A failed check creates a diagnosis for the retry ladder.

## Rejected Alternatives

Model-invoked audits or self-reported done messages would place completion
behind the weakest and least deterministic component.
