# Completion

## Purpose

Define evidence-gated closure for `agent.done`.

## Gate

`agent.done` closes a case only when the active graph state's completion gate
is satisfied. The gate checks:

- required evidence records for the task family and active phase.
- pending checks are empty or explicitly marked not run with a reason.
- completion summary is nonempty and matches observed evidence.
- task-specific guards, such as recursive document structure or exact file
  counts, are satisfied.

Missing evidence routes to a completion recovery notice. The model receives
the missing requirements and the next legal transitions. It does not close the
case by assertion.

## Status

implemented.
