# Sampling And Budgets

## Purpose

Define generation budgets, sampling defaults, and endpoint retry limits.

## Sampling

- `llm.temperature=0.3`
- `llm.top-p=0.9`
- `llm.reasoning-effort=none` when the provider supports it

## Max Tokens By Step

| Step kind | Config key |
| --- | ---: |
| write and revise | `llm.max-tokens.write=2400` |
| plan | `llm.max-tokens.plan=900` |
| explore action | `llm.max-tokens.explore=500` |
| respond and ask | `llm.max-tokens.respond=700` |
| judged verify | `checks.judged.max-tokens=300` |

## Backoff

Endpoint retries use bounded exponential backoff. Consecutive endpoint failures
are capped by `engine.endpoint-patience=10`; exhaustion blocks the task with a
specific report.

## Failure Boundary

Generation caps are sized by step kind, so honest prose is not forced through a
small action budget.
