# Task State

## Purpose

Specify the durable case state that replaces a bare open-or-closed task flag.

## Fields

Each active case stores:

- case id, status, objective, task family, active node, phase, and confidence.
- plan text with constraints, assumptions, candidate files, risks, evidence
  requirements, and next actions.
- touched paths, pending checks, open questions, selected context packages,
  recovery strategy, and completion guard.
- created, updated, and closed timestamps.

The store records every phase change and evidence addition in `graph_events`
and `graph_evidence`. Restart and compaction reconstruct the case from these
tables instead of a lossy prose summary.

## Phases

The supported phases are intake, planning, context, execution, verification,
recovery, compaction, completion, maintenance, waiting, and closed.

## Status

implemented.
