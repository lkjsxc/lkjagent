# Checks Gated Completion

## Purpose

Record the decision that evidence checks close cases.

## Context

Owner-visible completion must be backed by fresh evidence over real workspace
state and active state cells.

## Decision

A case reaches terminal closure only when completion predicates from the active
`RuntimeDecision` read current passing check rows and artifact fingerprints. The
model may write summaries or request a finish-like operation only when the
current decision exposes that grammar, but it cannot decide completion.

## Consequences

Benchmarks, replay, proof bundles, status, and closure all read the same check
results and state cells. A failed or stale check creates a diagnosis and a new
state edge for retry, evidence gathering, owner ask, or blocked reporting.

## Rejected Alternatives

Model-invoked audits or self-reported done messages would place completion
behind the weakest and least deterministic component.
