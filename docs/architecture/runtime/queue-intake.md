# Queue Intake

## Purpose

Specify how queue rows become context frames: when delivery happens, how
order is kept, and how answers to agent questions are matched. Owner-visible
behavior is [../../product/queue.md](../../product/queue.md).

## Delivery Rules

- Delivery happens only at a turn boundary, before the endpoint call.
- Pending messages are delivered oldest first, each as one owner frame:

```
<owner>
message text
</owner>
```

- Delivery marks the queue row delivered and writes an owner event to the
  transcript in the same transaction; a message can never be delivered twice
  or lost between queue and transcript.
- An owner frame costs context budget at delivery time; the cap and the
  truncation rule live in [../context/budgets.md](../context/budgets.md).

## Opening and Steering Tasks

- Delivery with no open task opens one; the message is the task statement.
- Delivery into an open task is steering: the agent must treat it as owner
  guidance for the current task, or explicitly defer it with an
  acknowledgment in its next think preamble. Deferred work is the next task
  the moment the current one closes.

## Answer Matching

When the task is waiting on agent.ask, the next delivered message is the
answer: the harness injects it as an owner frame directly after a notice
naming the question it answers. There is no parallel conversation; one
question may be outstanding at a time, enforced by
[../tools/control.md](../tools/control.md).

## Backpressure

There is none by design: senders never block and the queue never drops.
Cost appears only as delivery order delay and per-frame context budget. A
hundred queued messages still reach one loop, one at a time, in order.

## Status

design-only.
