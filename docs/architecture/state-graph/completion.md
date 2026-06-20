# Completion

## Purpose

Define evidence-gated closure for `agent.done`.

## Gate

`agent.done` closes a case only when the active graph state's completion gate
is satisfied. The gate checks:

- required evidence records for the task family and active phase.
- pending checks are empty or explicitly marked not run with a reason.
- task-specific guards, such as recursive document structure or file-count
  requirements, are satisfied.

Missing evidence routes to a completion recovery notice. The model receives
the missing requirements and the next legal transitions. It does not close the
case by assertion.

Code, bug, architecture, benchmark, and verification families require typed
verification evidence. Documentation and knowledge-base families require
document-structure evidence from doc.audit, doc.scaffold, or a deterministic
harness scaffold.

Architecture changes require matching code and docs evidence. Recovery tasks
require a fault record, a non-repeating alternate action, and improved state
or a specific blocker. Compaction tasks require a structured snapshot and a
rebuilt context notice before they can close.

## Status

implemented.
