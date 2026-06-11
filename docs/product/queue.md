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

## Delivery Into an Open Task

If a message arrives while a task is open, it is injected at the next turn
boundary as owner guidance for that task. The agent decides whether it
amends the current task or is new work; new work is deferred by the agent
with an explicit acknowledgment in the transcript, and picked up when the
current task closes.

## Answers and Questions

- The agent closes a task with agent.done; the summary is the answer and is
  visible in `lkjagent log`.
- The agent may ask the owner a question with agent.ask. The task enters the
  waiting state. The next queue message is treated as the answer if it
  arrives while waiting.
- Waiting never blocks the daemon: with an empty queue it shifts to
  self-maintenance and returns the moment an answer arrives.

## Bounds

The queue has no size cap and no message expiry. Flooding the queue only
costs ordering delay; each message still costs context budget only when
delivered, per [../architecture/context/budgets.md](../architecture/context/budgets.md).

## Status

design-only.
