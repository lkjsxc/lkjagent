# Sampling And Budgets

## Purpose

Define generation budgets, sampling defaults, and endpoint retry limits.

## Sampling

- `llm.temperature=0.3`
- `llm.top-p=0.9`
- `llm.reasoning-effort=none` when the provider supports it

## Generation Units

Ordinary model-authored artifact units target about 512 output tokens through
`llm.max-tokens.unit=512`. Write decisions include close-tag headroom above the
unit target so providers can finish the required envelope without turning a good
bounded unit into an endpoint-length fault. Decisions may select larger caps for
planning, response, or review work only when the decision row records the budget
and the reason.

The current plan-family bridge may still pass step-kind caps into endpoint
calls. The state-ledger budget policy replaces those caps as decisions are wired
through prompt frames.

## Backoff

Endpoint retries use bounded exponential backoff. Consecutive endpoint failures
are capped by `engine.endpoint-patience=10`; exhaustion blocks the matter with a
specific report.

## Failure Boundary

Generation caps are sized by step kind, so honest prose is not forced through a
small tool-call budget.
