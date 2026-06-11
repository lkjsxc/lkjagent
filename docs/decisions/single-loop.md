# Single Loop

## Purpose

Fix the concurrency and session model of the daemon.

## Decision

The daemon runs exactly one agent loop over exactly one continuous session.
All queued user messages feed that loop in order. Compaction keeps the
session alive indefinitely; there is no session list, no session switching,
and no parallel inference. The loop contract is
[../architecture/runtime/agent-loop.md](../architecture/runtime/agent-loop.md).

## Consequences

- The local endpoint serves one request at a time, matching what a 16 GB
  machine can actually do; no queueing theory, no fairness code.
- One brain accumulates one memory; distillation compounds instead of
  fragmenting across sessions.
- The queue is the only multiplexer: many messages, one consumer, strict
  order. See [../product/queue.md](../product/queue.md).
- A wedged loop is the failure domain; recovery and restart behavior live in
  [../architecture/runtime/daemon-process.md](../architecture/runtime/daemon-process.md).

## Rejected Directions

- Named sessions with one active at a time: adds a session registry, switch
  semantics, and per-session memory, all to serve a machine that cannot run
  two anyway.
- Parallel sessions with request serialization: concurrency complexity with
  zero added throughput on one local model.
