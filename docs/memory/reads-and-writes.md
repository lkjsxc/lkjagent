# Reads And Writes

## Purpose

Define the only paths that read or write memory rows.

## Writes

Memory rows are written by:

- `memory.save` during an explore step;
- substantial task closure, when the engine distills objective, outcome, key
  paths, and key facts into a row capped by `memory.distill.words=120`.

There is no idle distillation, merge pass, or hidden memory rewrite.

## Reads

Memory rows are read by:

- task intake, which queries owner-message terms and admits
  `memory.intake.max-hits=3` bounded facts into the task brief;
- `memory.find` inside explore steps, capped by `tools.memory-find.max-hits=10`;
- the owner CLI memory command.

## Failure Boundary

Memory cannot prove completion. It can supply facts, but checks over workspace
files and step results remain the only completion evidence.
