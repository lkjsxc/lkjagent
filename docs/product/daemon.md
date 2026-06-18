# Daemon

## Purpose

Describe the observable lifecycle of the lkjagent daemon: one long-lived
process that never needs an attending human.

## Behavior

The daemon starts with `lkjagent run` inside the container and keeps running
until stopped. It owns the single agent loop described in
[../architecture/runtime/agent-loop.md](../architecture/runtime/agent-loop.md).

While running, the daemon cycles through four observable states:

| State | Meaning |
| --- | --- |
| idle | No task is open; the queue is empty; the daemon is waiting |
| working | A task is open; the loop is taking turns toward agent.done |
| waiting | The agent asked the owner a question and no later send has arrived |
| error | The endpoint or loop failed; details are visible in status and log |

State transitions are driven only by the queue and the loop, never by
timers visible to the owner. A new queue message pulls the daemon out of
idle or waiting at the next turn boundary.

## Startup

On startup the daemon opens the store, replays nothing, and rebuilds its
context prefix from durable state: system prompt, memory digest, skill index,
and the workspace brief. If a task was open when the process stopped, the
task resumes from stored task state and summaries, not from a raw replay. Startup is
specified in [../architecture/runtime/daemon-process.md](../architecture/runtime/daemon-process.md).

## Shutdown

Stopping the container ends the process. In-flight endpoint calls are not
drained by a custom signal handler. Queue rows, transcript events, memory,
and skills are durable, and a restarted daemon reclaims a stale lock after
the heartbeat exceeds the configured stale window.

## Failure

If the endpoint is unreachable, the daemon records an error event, sets
`daemon_state=error`, and tries again on later polls; it never fabricates a
model reply. If the loop itself fails, stale lock reclaim lets a restarted
process continue from durable state.

## Status

implemented.
