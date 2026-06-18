# Queue

## Purpose

Describe how user messages enter lkjagent, wait, and get consumed. The queue
is the only way work reaches the agent and the only multiplexer in the
system.

## Behavior

- `lkjagent send` appends a message; nothing ever blocks the sender.
- Messages are durable immediately: a daemon crash after send loses nothing.
- The daemon consumes messages strictly in arrival order.
- Messages are delivered only at turn boundaries, never mid-turn. A running
  action is never interrupted; intake rules are
  [../architecture/runtime/queue-intake.md](../architecture/runtime/queue-intake.md).
- Consuming a message never deletes it; the queue row is marked delivered and
  the content becomes a transcript event.

## Mutation

Queue rows have status pending, delivered, or deleted. Pending rows may be
edited or tombstoned before delivery through queue tools; tombstoned rows
stay in the store and are skipped by delivery. Redelivery always inserts a
new pending row linked to source_queue_id. It never rewrites the source row
or delivered owner events. CLI send and queue tools record queue_mutation
events in the transcript.

Tool contracts live in [../architecture/tools/queue-ops.md](../architecture/tools/queue-ops.md);
storage rules live in [../architecture/memory/store.md](../architecture/memory/store.md).

## Delivery Into an Open Task

If a message arrives while a task is open, it is injected at the next turn
boundary as owner guidance for that task. The agent decides whether it
amends the current task or is new work; new work is deferred by the agent
with an explicit acknowledgment in the transcript, and picked up when the
current task closes.

## Done and Questions

- The agent closes a task with agent.done; the summary is visible in
  `lkjagent log`.
- The agent may ask the owner a question with agent.ask. The task enters the
  waiting state. The next queue message is delivered as a normal owner
  message and resumes the task.
- Waiting never blocks senders: with an empty queue the daemon remains
  waiting and returns to work when another message arrives through
  `lkjagent send`.

## Bounds

The queue has no size cap and no message expiry. Flooding the queue only
costs ordering delay; each message still costs context budget only when
delivered, per [../architecture/context/budgets.md](../architecture/context/budgets.md).

## Status

implemented.
