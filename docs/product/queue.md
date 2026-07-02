# Queue

## Purpose

Define how owner messages enter the system and how waiting tasks receive
answers.

## Message Intake

`lkjagent send TEXT` inserts one pending row into the queue table. It does not
require a running daemon. The daemon delivers queue rows only at cycle start, so
an owner message never interrupts a turn halfway through an endpoint call or
state transaction.

## Task Creation

If no task is waiting, the first pending message opens a task. The objective is
the owner text verbatim. A deterministic classifier picks the initial template,
which creates the first steps and task-level checks.

## Answer Routing

If the active task is `waiting`, the next owner message is treated as the
answer to that task. The queue row is linked to the task, an answer event is
recorded, and the task becomes `open`.

`lkjagent send --new TEXT` bypasses answer routing and creates a separate queued
message. The existing waiting task remains waiting.

## Ordering

Tasks run strictly FIFO. The daemon does not interleave model calls from two
open tasks. This keeps the store, exchange logs, prompt state, and owner
transcript easy to reconstruct.

## Visibility

`lkjagent queue list` shows pending, delivered, and answered rows. `task show`
links the task back to its opening queue row and any answer rows.
