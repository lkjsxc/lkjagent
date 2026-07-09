# Sampling And Budgets

## Purpose

Define endpoint sampling, semantic output units, and causal retry budgets.

## Sampling

Flat configuration supplies temperature, top-p, and optional provider reasoning
effort. Effective values and provider capability fingerprints are stored with
each exchange. Evaluation controls them within a comparative cell.

## Output Reserve

The decision reserves parser and close-tag headroom before context selection.
Semantic artifact units normally target about 512 output tokens. The operation
may choose a larger cap only when its output shape, remaining budget, and reason
are persisted.

Requested size is compared with output cap, context cap, operation count, and
file budget before a model call. Work that cannot fit is split into named
semantic units or narrowed; it is not sent unchanged through an impossible cap.

## Input Lanes

The decision allocates bounded input lanes for kernel, objective, state,
workspace, conversation, evidence, conflict, recovery, and tools. Provider
usage calibrates later estimates without changing the current decision.

## Retry

Endpoint failures use bounded backoff and a failure lineage. Each retry changes
an external condition, prompt, tool view, budget, or strategy. Exhaustion writes
a visible wait or blocker with remaining owner actions; it never settles as
success.
