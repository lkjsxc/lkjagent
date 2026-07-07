# Reads And Writes

## Purpose

Define the only paths that read or write memory rows.

## Writes

Memory rows are written by:

- admitted `memory.save` actions;
- substantial matter closure, when the engine distills objective, outcome, key
  paths, and key facts into a row capped by `memory.distill.words=120`.

There is no idle distillation, merge pass, or hidden memory rewrite.

## Reads

Memory rows are read by:

- owner-turn intake, which queries owner terms and admits
  `memory.intake.max-hits=3` bounded facts into the matter brief;
- admitted `memory.find` actions, capped by `tools.memory-find.max-hits=10`;
- the owner CLI memory command.

## Failure Boundary

Memory cannot prove completion. It can supply facts, but checks over workspace
files, records, artifacts, and observations remain the only completion evidence.
