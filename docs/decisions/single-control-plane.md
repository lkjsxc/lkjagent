# Single Control Plane

## Purpose

Record the decision that the plan ledger is the only runtime authority.

## Context

The harness must direct a weak model without asking it to navigate competing
policy surfaces.

## Decision

Tasks, steps, attempts, checks, and the retry ladder are the single control
plane. The model authors bounded content or one explore action for the active
step.

## Consequences

There are no graph authority decisions, admission matrices, runtime modes, or
model-selected completion tools. Status and prompts read the same plan digest.

## Rejected Alternatives

A separate graph or policy layer could disagree with the selected step and make
legal output impossible to identify from the prompt.
