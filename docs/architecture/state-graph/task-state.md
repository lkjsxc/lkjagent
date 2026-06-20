# Task State

## Purpose

Specify the durable case state that replaces a bare open-or-closed task flag.

## Fields

Each active case stores structured state:

- objective, family, phase, status, active node, confidence, and budgets.
- constraints, assumptions, questions, risks, invariants, success criteria,
  and decisions.
- plan steps with status, node, target paths, evidence requirements, and
  verification checks.
- context bindings, workspace paths, evidence requirements, evidence records,
  completion state, recovery history, document state, and transition history.

The store keeps `graph_cases` as the header and normalized child tables for
constraints, assumptions, questions, risks, success criteria, plan steps,
decisions, context bindings, transitions, faults, artifacts, and document
state. Restart and compaction reconstruct from structured rows instead of a
lossy prose summary.

## Phases

The supported phases are intake, planning, context, execution, verification,
recovery, compaction, completion, maintenance, waiting, and closed.

## Status

implemented.
