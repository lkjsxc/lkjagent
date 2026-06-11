# Runtime

## Purpose

This directory specifies the beating heart of lkjagent: the single agent
loop, the daemon process around it, how queued messages reach the loop, and
what the loop does when nobody is asking for anything. Owned by the
lkjagent-runtime crate per [../overview.md](../overview.md).

## Table of Contents

- [agent-loop.md](agent-loop.md): the turn and task lifecycle.
- [daemon-process.md](daemon-process.md): process model, startup, shutdown, restart.
- [queue-intake.md](queue-intake.md): turn-boundary delivery and answer matching.
- [self-maintenance.md](self-maintenance.md): the idle loop that improves the system.
