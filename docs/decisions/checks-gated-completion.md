# Checks Gated Completion

## Purpose

Record that current deterministic checks satisfy obligations and close matters.

## Context

Owner-visible completion must be backed by fresh evidence over actual workspace
bytes, effect outcomes, runtime state, and requested verification.

## Decision

Each required obligation names a stable evidence predicate. It becomes satisfied
only from a passed current check for the same matter. The reducer creates a
completion candidate after every required predicate passes and no unresolved
operation blocks it. The model cannot create that candidate or settle it.

## Consequences

Evaluation, replay, proof, status, and closure read the same checks and source
fingerprints. A failed or stale check appends an invalidation or diagnosis event
and makes repair, evidence gathering, owner clarification, or blocked reporting
eligible.

## Rejected Alternatives

Model-invoked audits, readiness prose, or self-reported completion would place
the final state behind the least deterministic component.
