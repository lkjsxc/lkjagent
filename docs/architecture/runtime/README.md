# Runtime

## Purpose

This directory specifies the beating heart of lkjagent: the single
graph-governed agent loop, the daemon process around it, how queued messages
become graph cases, and what the daemon records while waiting. Owned by the
lkjagent-runtime crate per [../overview.md](../overview.md).

## Table of Contents

- [agent-loop.md](agent-loop.md): the graph case, turn, and task lifecycle.
- [daemon-process.md](daemon-process.md): process model, startup, shutdown, restart.
- [queue-intake.md](queue-intake.md): turn-boundary delivery and waiting-task resume.
- [self-maintenance.md](self-maintenance.md): idle graph maintenance boundary and directives.
