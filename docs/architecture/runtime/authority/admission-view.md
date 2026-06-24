# Admission View

## Purpose

This file owns the immutable tool-admission view derived from a runtime decision.

## Contract

Dispatch receives `AuthorityAdmissionView` and never recomputes policy from graph state alone. The view is derived
from the persisted decision and contains the exact data needed to accept an effect or return a structured refusal.

## Inputs

- decision id and case id.
- authority fingerprint and staleness fingerprint.
- active mission and active node.
- admitted and blocked tools.
- shell allowance and completion allowance.
- missing evidence, recovery route, and valid examples by tool.

## Outputs

- admitted dispatch execution.
- structured refusal with reason, active mode, previous mode when known,
  missing evidence, failed gate, admitted tools, blocked tools, decision id, and
  exact next executable action.
- `runtime_tool_admissions` record for accepted and refused requests.

## Invariants

- A requested tool executes only when it appears in the view's admitted set.
- Old graph policy cannot admit a tool blocked by authority.
- Recovery escape tools can execute when authority admits them even if the old node policy is narrower.
- Fingerprint mismatch refuses the stale action and emits the refreshed next action.
- The staleness fingerprint covers queue head, case id, graph node and phase,
  active mode, artifact root and cursor, latest fault, missing evidence,
  compaction pressure, maintenance state, and prompt frame head.
- Shell routes require explicit shell admission in the view.

## Failure Cases

- A graph-policy fallback executes after authority blocked the action.
- A cached action survives compaction, queue, artifact cursor, evidence, prompt,
  or case changes.
- Recovery blocks the read, audit, repair, or batch tool needed to escape.
- Refusal text lacks a route that parses and reaches dispatch.

## Verification

- dispatch tests for admitted, blocked, stale, and recovery-escape actions.
- route-level tests for rendered examples.
- admission history store tests.

## Status

partially implemented. Store APIs record tool admission outcomes. Pending-action dispatch installs a decision-derived
immutable admission view, consults it before effective-policy or graph fallback refusal, and writes admission or
stale-refusal rows against the latest authority decision. Full-fingerprint stale refusal and broader route coverage
remain open.
