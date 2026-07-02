# Journal Template

## Purpose

Define personal record tasks as plain workspace file writes.

## Selection

The classifier selects this template for journal, schedule, todo, and similar
personal-record objectives.

## Plan

- Journal entries append to `journal/<date>.md` using
  `template.journal.date-source=local-clock`.
- Schedule entries write under `schedule/` when the objective names dates.
- Todo entries write under `todos/` as Markdown.
- Substantial entries may create a memory row capped by
  `memory.distill.words=120` after the respond step.

## Checks

The template attaches `file_exists` and objective-specific `contains` checks
from [../completion.md](../completion.md). It does not create dedicated
personal tables.

## Failure This Prevents

Personal records use the same plan, file, and check path as other work, so they
cannot become a separate feature stack with untested persistence.
