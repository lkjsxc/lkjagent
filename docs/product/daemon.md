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
| idle | No task or maintenance cycle is open between loop ticks |
| working | A task or maintenance cycle is open; the loop is taking turns |
| waiting | The agent asked the owner a question and no later send has arrived |
| error | The endpoint or loop failed; details are visible in status and log |

State transitions are driven only by the queue and the loop, never by
timers visible to the owner. With an empty queue, the next idle boundary
opens bounded self-maintenance. A new queue message preempts maintenance or
pulls the daemon out of idle or waiting at the next turn boundary.

## Startup

On startup the daemon opens the store, replays nothing, and rebuilds its
context prefix from durable state: system prompt, graph state, memory digest,
and the workspace brief. If a task was open when the process stopped, the
task resumes from graph case state and evidence, not from a raw replay.
Startup is specified in
[../architecture/runtime/daemon-process.md](../architecture/runtime/daemon-process.md).

## Shutdown

Stopping the container ends the process. In-flight endpoint calls are not
drained by a custom signal handler. Queue rows, transcript events, memory,
graph cases, evidence, memory, and workspace state are durable in data. A
restarted daemon reclaims a stale lock after the heartbeat
exceeds the configured stale window.

## Failure

If the endpoint is unreachable, the daemon records an error event, sets
`daemon_state=error`, and tries again only after the capped retry deadline;
polls before that deadline do not hit the endpoint or add more error events.
It never fabricates a model reply. Parser, repeat-action, and tool failures
stay inside the task: the daemon records the failure, adds a recovery notice
for the next model turn, and keeps working until agent.done, agent.ask, or the
task budget.
Task budget exhaustion becomes an observable waiting state with a concrete
question, and the next owner send resumes the task with a fresh budget.
When a user task closes, the daemon returns to maintenance on the next empty
queue boundary instead of stopping permanently. If the loop itself fails,
stale lock reclaim lets a restarted process continue from durable state.

## Status

implemented.
