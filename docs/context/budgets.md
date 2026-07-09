# Budgets

## Purpose

Define the token and content budgets that keep each turn bounded.

## Request Budgets

- `context.request.typical-tokens=6000` is the normal target for scripted work.
- `context.request.hard-cap-tokens=8000` is the render ceiling.
- `context.truncation.marker=[...]` marks omitted middle content.

## Lane Budgets

- kernel carries stable product and safety invariants;
- objective carries the owner goal and explicit constraints;
- state carries the operation, obligations, and exit rule;
- workspace and conversation carry source-linked excerpts;
- evidence, conflict, recovery, and tools carry decision-specific facts.

Lane caps vary by prompt state while the total request stays below the hard cap.

## Generation Budgets

The LLM page owns generation caps. Context rendering uses those caps to leave
headroom. Ordinary artifact units target about 512 output tokens. Larger
response or review caps are decision-specific and must be recorded with the
persisted `RuntimeDecision`.

## Estimator

`context.estimator.chars-per-token=4` is the conservative fallback when endpoint
usage is unavailable. Endpoint usage rows replace estimates after a call.

## Failure This Prevents

Honest prose attempts fit the output budget chosen for the operation instead of being
converted into length faults by shrinking caps.
