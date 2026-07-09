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
State payloads use flat key `tool_budget_remaining`. When it is `0`, selector
builds an empty `ToolSetView` and records `tool-budget:suppressed` in the
decision evidence requirements.

## Repeat Guard

An identical action for the same operation, state, prompt, tool view, budget,
and failure signature is not executed without a changed external condition.
Admission records a bounded repeat diagnosis and recovery event. The comparison
uses canonical typed fields persisted for the decision, and every repeat attempt
receives its own rejected admission row.

## Recovery Guard

Recovery constraints may hide non-idempotent tools or require an observation
repair operation. Hidden tools appear only in diagnostics and proof, never in
normal prompt text.

## Failure This Prevents

Repeated or unsafe tool calls cannot form a fixed point, and the prompt does not
teach the model to call tools that guards will reject.
