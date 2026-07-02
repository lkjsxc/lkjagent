# Completion

## Purpose

Define how steps and tasks finish through deterministic checks.

## Completion Authority

Completion belongs to the engine. Model output can provide content or a sparse
judgment block, but `closed` is reachable only when every task-level check
passes and the engine records the results.

## Step Checks

A step may attach checks from `checks.catalog.names` in
[../checks/catalog.md](../checks/catalog.md). Checks run immediately after the
step effect and before the step can become `done`.

A failed check produces a bounded diagnosis with the measured value. The retry
ladder in [retry-and-escalation.md](retry-and-escalation.md) decides whether to
retry, split, extend, block, or review.

## Task Checks

When no runnable steps remain, task checks run over the real workspace. A
manuscript task checks chapter file count, total manuscript words, and any
objective-specific absence or structure checks. A docs-tree task checks README
coverage and relative links.

## Evidence Rows

Each check result stores the check name, parameters, pass flag, measured value,
and timestamp. Status, task display, benchmarks, replay, and proof bundles read
those rows rather than asking the model what happened.

## Failure This Prevents

False completion is structurally blocked. A task cannot close because the model
says it is done; it closes only when measured evidence passes.
