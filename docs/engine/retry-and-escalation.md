# Retry And Escalation

## Purpose

Define the finite ladder for faults, check failures, blocked steps, and blocked
tasks.

## Ladder

- Retry with diagnosis while `attempts_used < engine.attempts-per-step=3`.
  The retry frame states one diagnosis and the required change.
- Shrink or split after exhausted attempts when `engine.shrinks-per-lineage=1`
  still permits it. Write steps split; explore steps narrow; plan steps fall
  back to the template skeleton.
- Block and continue when the step cannot be repaired inside its lineage.
- Review the task once when blocked steps remain and no runnable steps exist.
  This is capped by `engine.reviews-per-task=1`.
- Ask or block. Owner-only missing information creates an ask step; all other
  terminal failures write a blocked report.

Task budget exhaustion at `engine.task-budget.model-calls=200` jumps to the
ask-or-block rung. Consecutive endpoint failures use
`engine.endpoint-patience=10` before the task blocks.

## Retry Frame

The faulty output is never quoted. The prompt contains only the one-line
diagnosis, such as `min_words chapter-02.md: 312 < 500`, and the concrete
change required.

## Termination

Attempts, split lineage, review count, endpoint patience, and task call budget
are all finite. The ladder ends in `done`, `waiting`, or `blocked`; it never
runs the same unchanged ask indefinitely.

## Failure This Prevents

Repetition cannot be reinforced by transcript replay. The next attempt changes
its prompt, and exhausted work changes shape or blocks with evidence.
