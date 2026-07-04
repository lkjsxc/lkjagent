# Retry And Escalation

## Purpose

Define the finite ladder for faults, check failures, blocked state, and owner
asks.

## Ladder

- Retry with bounded diagnosis while the active state lineage has attempts
  remaining. The retry frame states one diagnosis and the required change.
- Suppress or hide tools when policy, repeat guards, or recovery constraints say
  admission would reject them.
- Shrink, split, or narrow work after exhausted attempts when lineage policy
  permits it.
- Gather fresh evidence when completion, contradiction, or artifact state
  requires proof.
- Ask the owner when owner-only information is missing.
- Block with an evidence-backed report when no safe transition remains.

Endpoint failures use finite patience before a recovery or blocked state is
selected. Model-call budgets are state cells and usage rows, not prompt text.

## Retry Frame

The faulty output is never quoted. The prompt contains only a bounded diagnosis,
such as `min_words chapter-02.md: 312 < 500`, and the concrete change required.
Raw failed output is contaminated context.

## Termination

Attempts, split lineage, review count, endpoint patience, and case call budget
are finite. The ladder ends in done, waiting, blocked, closed, or another
durable recovery state; it never runs the same unchanged ask indefinitely.

## Failure This Prevents

Repetition cannot be reinforced by transcript replay. The next turn changes
state, prompt, available tools, budget, or terminal outcome.
