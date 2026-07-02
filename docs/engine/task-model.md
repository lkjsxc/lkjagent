# Task Model

## Purpose

Define the durable task record and its state machine.

## Task Record

A delivered owner message opens a task unless it answers a waiting task. The
objective is stored verbatim and is never rewritten. The classifier records the
template name that shaped the initial plan.

Task fields:

| Field | Meaning |
| --- | --- |
| `id` | stable store id |
| `queue_id` | opening owner message |
| `objective` | owner text exactly as received |
| `template` | selected template name |
| `state` | `open`, `waiting`, `blocked`, or `closed` |
| `brief` | bounded engine-maintained working summary |
| `budget_used` | model calls consumed |
| `budget` | model call cap, `engine.task-budget.model-calls=200` |
| `summary` | terminal owner-facing report |

## State Machine

- `open` becomes `waiting` when an ask step delivers a question.
- `waiting` becomes `open` when the owner answers through the queue.
- `open` becomes `blocked` when the retry ladder or task budget exhausts.
- `open` becomes `closed` only when task checks pass.

`closed` and `blocked` are terminal. A later owner message starts a separate
task unless it is routed as the answer to `waiting`.

## Budget Rule

`engine.task-budget.model-calls=200` counts endpoint calls that reach the model.
Verify-only and engine-side work do not consume that budget. Budget exhaustion
jumps directly to the blocked report rung in
[retry-and-escalation.md](retry-and-escalation.md).

## Failure This Prevents

Task closure cannot be inferred from model prose or a tool name. It is a state
edge guarded by [completion checks](completion.md), which prevents false done
reports.
