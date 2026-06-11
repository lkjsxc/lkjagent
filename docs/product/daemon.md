# Daemon

## Purpose

Describe the observable lifecycle of the lkjagent daemon: one long-lived
process that never needs an attending human.

## Behavior

The daemon starts with `lkjagent run` inside the container and keeps running
until stopped. It owns the single agent loop described in
[../architecture/runtime/agent-loop.md](../architecture/runtime/agent-loop.md).

While running, the daemon cycles through three observable states:

| State | Meaning |
| --- | --- |
| working | A task is open; the loop is taking turns toward agent.done |
| waiting | The agent asked the owner a question and no answer has arrived |
| maintaining | The queue is empty; the loop distills memory and refines skills |

State transitions are driven only by the queue and the loop, never by
timers visible to the owner. A new queue message always pulls the daemon out
of maintaining at the next turn boundary.

## Startup

On startup the daemon opens the store, replays nothing, and rebuilds its
context prefix from durable state: system prompt, memory digest, skill index,
and the workspace brief. If a task was open when the process stopped, the
task resumes from its transcript summary, not from a raw replay. Startup is
specified in [../architecture/runtime/daemon-process.md](../architecture/runtime/daemon-process.md).

## Shutdown

Stopping the container or sending SIGTERM ends the daemon between turns. A
turn in flight finishes its observation write before exit. Nothing is lost:
queue, transcripts, and memory are durable in the store, and skills are files.

## Failure

If the endpoint is unreachable, the daemon retries with backoff and reports
the condition through `lkjagent status`; it never fabricates a model reply.
If the loop itself fails, the daemon records the failure as a transcript
event and the container supervisor restarts the process.

## Status

design-only.
