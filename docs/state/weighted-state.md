# Weighted State

## Purpose

Weighted state owns concurrent pressures that a single hard node cannot show.
Weights are independent values from zero through one; they are not normalized.

## Track Shape

Each track stores:

```text
id
label
posture
weight
confidence
source event
evidence gap
guard policy
decay policy
last updated event
next recommended action
```

## Required Labels

The initial vector includes objective normalization, context-frame quality,
documentation contract, semantic seed, structure seed, structure expansion,
structure connectivity, semantic coverage, duplication risk, mock content risk,
topic drift risk, model-specific naming, prompt contract, document structure,
artifact readiness, artifact drift, parse recovery, action parameter
reliability, tool execution recovery, evidence gap, context pressure, context
snapshot mismatch, queue interruption, completion readiness, observability
ledger, repeated action risk, maintenance opportunity, maintenance no-op risk,
and workspace evidence risk.

## Interpretation

- `0.00..0.19`: inactive.
- `0.20..0.39`: weak.
- `0.40..0.59`: active.
- `0.60..0.79`: strong.
- `0.80..1.00`: dominant or guard-triggering.

## Status

implemented
