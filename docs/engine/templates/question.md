# Question Template

## Purpose

Define direct answer tasks.

## Selection

The classifier selects this template when the objective asks for an answer and
does not require creating or revising files.

## Plan

- Workspace-dependent questions begin with an explore step capped by
  `template.question.explore-budget=6`.
- Self-contained questions go directly to a respond step.
- The respond step emits `<message>` and stores the answer as the task summary.

## Checks

Question tasks usually have no task-level file checks. When the question names a
command or file fact, a verify step may run the relevant check from
[../completion.md](../completion.md) before the response.

## Failure This Prevents

Small questions do not enter the long-artifact machinery, so the owner receives
a bounded answer instead of an unnecessary plan.
