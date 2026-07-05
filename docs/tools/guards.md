# Guards

## Purpose

Define deterministic guards applied during tool admission and effect dispatch.

## Path Guard

All file paths resolve inside the workspace. Absolute paths and `..` segments
are denied by admission before any effect runs. The guard name is
`tools.guard.path-inside-workspace`.

## Budget Guard

A runtime decision carries the remaining tool, token, and retry budgets relevant
to the active state. Budget exhaustion suppresses tools in the `ToolSetView` or
selects a recovery decision. It does not render tools that admission will refuse.

## Repeat Guard

A byte-identical tool call to the previous admitted tool call for the same state
edge is not executed. Admission records diagnosis `repeated tool call; state the
next different tool call or finish` and emits a bounded recovery event.

## Recovery Guard

Recovery constraints may hide non-idempotent tools or require an observation
repair step. Hidden tools appear only in diagnostics and proof bundles, never in
normal prompt text.

## Failure This Prevents

Repeated or unsafe tool calls cannot form a fixed point, and the prompt does not
teach the model to call tools that guards will reject.
