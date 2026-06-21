# Retry Budget

## Purpose

Define retry limits for recovery so invalid loops terminate deterministically.

## Contract

Each fault class stores same-class retry count, same-action retry count, last
invalid action, and last valid observation. Budget exhaustion changes action
class or emits a structured handoff.

## Budgets

Parse and parameter faults get one same-action retry after a schema-rendered
example. Repeat-action faults get zero same-action retries. Audit and
weak-content faults continue by path cursor, not by repeating the same failed
completion action.

## Invariants

- The same malformed payload is never sent more than once after diagnosis.
- The same refused graph inspection is not repeated past budget.
- Budget is preserved across compaction snapshots.

## Fixture

`repeat_action_refused` proves the next action class changes after refusal.

## Verification

Run `cargo test -p lkjagent-runtime recovery_loop`.

## Status

design-only for class-specific budgets.
