# Budgets

## Purpose

Define the token and content budgets that keep each turn bounded.

## Request Budgets

- `context.request.typical-tokens=6000` is the normal target for scripted work.
- `context.request.hard-cap-tokens=8000` is the render ceiling.
- `context.truncation.marker=[...]` marks omitted middle content.

## Step Input Budgets

- `context.write.continuity-tail-words=150` supplies prose continuity.
- `context.explore.observation-tokens=1500` bounds the latest tool observation.
- `context.revise.input-tokens=4000` bounds revise input excerpts.
- `context.memory.fact-tokens=100` bounds memory facts in a task brief.

## Generation Budgets

The LLM page owns generation caps. Context rendering uses those caps to leave
headroom. The step defaults are `llm.max-tokens.write=1400`,
`llm.max-tokens.plan=900`, `llm.max-tokens.explore=500`, and
`llm.max-tokens.respond=700`.

## Estimator

`context.estimator.chars-per-token=4` is the conservative fallback when endpoint
usage is unavailable. Endpoint usage rows replace estimates after a call.

## Failure This Prevents

Honest prose attempts fit the output budget chosen for the step instead of being
converted into length faults by shrinking caps.
