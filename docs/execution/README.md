# Execution

## Purpose

This directory is the work queue: the rules an autonomous session follows,
the dependency-ordered blocker map, and one executable task file per slice.
The mission is to implement the contracts under
[../architecture/](../architecture/README.md) until
[../current-state.md](../current-state.md) says implemented everywhere.

## How Work Flows

A session takes the first open blocker from
[current-blockers.md](current-blockers.md), opens its task file under
[tasks/](tasks/README.md), and follows
[../agent/work-loop.md](../agent/work-loop.md). A task is complete only
when docs, implementation, focused tests, the task Status, the blocker row,
and the status ledger all moved together.

## Table of Contents

- [operating-rules.md](operating-rules.md): defaults for decisions no human is present to make.
- [current-blockers.md](current-blockers.md): the dependency-ordered implementation queue.
- [tasks/](tasks/README.md): one executable task file per slice, with the shared template.
