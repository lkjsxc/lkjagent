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

When `agent.done` is attempted early, the refusal is a partial handoff, not a
success. It names the failed completion gate, missing evidence, current graph
state, and one copyable next executable action.

Code, bug, architecture, benchmark, and verification families require typed
verification evidence. Documentation and knowledge-base families require
document-structure evidence from a passed `doc.audit` or deterministic harness
audit. Scaffold creation alone is never completion evidence for content
artifacts.

Content artifacts require root, README, manifest, semantic children,
content-bearing files, audit evidence, graph plan evidence, observation
evidence, verification or audit evidence, and no active unrecovered fault over
threshold.

Architecture changes require matching code and docs evidence. Recovery tasks
require a fault record, a non-repeating alternate action, and improved state
or a specific blocker. Compaction tasks require a structured snapshot and a
rebuilt context notice before they can close.

## Status

partially implemented; graph completion checks exist. Artifact-aware readiness
and blocked handoff states remain open.
