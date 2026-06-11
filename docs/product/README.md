# Product

## Purpose

This directory states the observable behavior of lkjagent from its owner's
point of view: what the daemon does, what the CLI offers, how the queue
behaves, and what can be observed while the agent runs. Everything here is
design-only until [../current-state.md](../current-state.md) says otherwise.
Internal mechanics live under [../architecture/](../architecture/README.md).

## Table of Contents

- [daemon.md](daemon.md): the continuously running process and its lifecycle.
- [cli.md](cli.md): the thin command-line client and its commands.
- [queue.md](queue.md): how user messages enter, wait, and get answered.
- [observability.md](observability.md): status, transcript, and memory views.
