# Recovery Ladders

## Purpose

Define how typed faults move through graph recovery without turning generic
errors into owner waiting.

## Fault Ledger

Every parse, repeat, tool, shell, endpoint, verification, context, budget, or
completion fault records:

- kind, active node, action fingerprint, summary, count, and timestamp.
- recovery ladder position and selected strategy.
- recent-fault health state rendered in the graph card.

Repeated fingerprints are evidence that the next action class must change.
The runtime records the fault before choosing the next node.

## Ladder

Recovery follows this deterministic order:

1. correct the action format or parameters and retry once.
2. inspect graph state or workspace state and choose a different native tool.
3. reduce scope and create a smaller plan step.
4. route to the matching recovery node and record the decision.
5. use shell only from a shell-admitted recovery node.
6. mark the specific plan step blocked and continue an independent step.

Waiting is valid only for a concrete owner decision, context invalidity,
endpoint outage policy, or sandbox boundary.

## Status

implemented.
