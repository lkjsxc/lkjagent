# Daemon

## Purpose

Define how the lkjagent daemon lives, works, waits, and stops.

## Lifecycle

`lkjagent run` starts one foreground daemon for one data directory. The daemon
opens the SQLite store, claims the process lock through heartbeat config rows,
loads configuration, and enters the turn cycle.

Each cycle has one active concern:

1. deliver due owner messages;
2. select the next work item from the current task snapshot;
3. render at most one prompt;
4. call the endpoint when the selected work needs model output;
5. apply the deterministic effect;
6. evaluate checks;
7. persist the attempt, events, checks, and state update in one transaction.

At most one endpoint call happens in a cycle. Verify-only work consumes no
model call.

## Task States

- `open`: the task has runnable steps or pending completion checks.
- `waiting`: the task asked the owner one concrete question and waits for the
  answer.
- `blocked`: the task exhausted its ladder or budget and wrote an evidence
  report.
- `closed`: all task-level checks passed and the completion summary was
  recorded.

`blocked` and `closed` are terminal. A later owner message starts a separate
task unless it answers a waiting task.

## Idle

No open task and no pending queue item means idle. Idle updates heartbeat data
needed for lock reclaim and then sleeps on the queue poll interval. It does not
call the endpoint, rewrite memory, inspect files, or self-assign work.

## Waiting

A waiting task parks like idle, but status prints the pending question. The next
owner message is attached as the answer and returns the task to `open`.
`send --new` creates a separate task instead.

## Crash Resume

Every committed turn is durable in SQLite and exchange-log files. On boot the
daemon reclaims the lock if needed, reads the first non-terminal task, and
continues at selection. A crash mid-call can lose one uncommitted model call;
it cannot create a false completion.
