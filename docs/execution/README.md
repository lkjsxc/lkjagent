# Execution

## Purpose

This directory is the work queue: the rules an autonomous session follows,
the dependency-ordered blocker map, current reliability work, and executable
task files. The mission is to keep docs and code aligned until
[../current-state.md](../current-state.md) can honestly mark each behavior.

## How Work Flows

A session takes the first open blocker from
[current-blockers.md](current-blockers.md), opens its current-work or task
contract, and follows [../agent/work-loop.md](../agent/work-loop.md). A task is
complete only when docs, implementation, focused tests, the blocker row, and
the status ledger all move together.

## Table of Contents

- [operating-rules.md](operating-rules.md): defaults for decisions no human is present to make.
- [current-blockers.md](current-blockers.md): the dependency-ordered implementation queue.
- [current-work/](current-work/README.md): reliability redesign tasks and gates.
- [tasks/](tasks/README.md): one executable task file per slice, with the shared template.
