# Working Here

## Purpose

Define the session loop for coding agents.

## Read Order

1. [../current-state.md](../current-state.md)
2. [../vision/README.md](../vision/README.md)
3. [../state/README.md](../state/README.md)
4. [../runtime/README.md](../runtime/README.md)
5. The docs page that owns the task being changed
6. [../operations/verification.md](../operations/verification.md)

## Case State

Before editing, name the objective, constraints, assumptions, risks, evidence
requirements, candidate files, and next action. Keep it in the working notes or
a task file when it must survive the session.

## Routing

If the owner names a task, do that task. Otherwise choose the first open item in
[../current-state.md](../current-state.md). Documentation changes precede code
changes. Implementation commits update their owning docs in the same slice.

## Completion

A task is complete only when its acceptance evidence exists. If a live endpoint
is unavailable, record the skip honestly and leave the live item open.
