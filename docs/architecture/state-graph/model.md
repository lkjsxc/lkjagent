# State Graph Model

## Purpose

Define the typed source graph and the runtime state envelope that every
meaningful task enters before endpoint execution.

## Graph Definition

The source graph is deterministic Rust data, not loose prompt text. It holds:

- `GraphNodeId` and `GraphEdgeId` identifiers.
- `GraphNode` records with `NodeKind`, label, instructions, context needs,
  evidence requirements, allowed actions, and policy.
- `GraphEdge` records with `EdgeKind`, source node, target node, guard, and
  rationale.
- `GraphPolicy` values for context pressure, recovery limits, completion
  gates, and maintenance cadence.

Node kinds cover intent classification, planning, state tracking, context
selection, execution, document construction, memory, compaction, recovery,
completion, and maintenance.

## Task Case

The runtime creates or resumes a `TaskGraphState` for each owner task. The
state records:

- objective, task family, active phase, active node, and confidence.
- constraints, assumptions, open questions, risks, and invariants.
- candidate paths, touched paths, context packages, and pending checks.
- evidence requirements, observed evidence, and missing evidence.
- completion guard state and recovery history.

The first owner message is input to classification and planning. It is not a
plan by itself.

## Source And Runtime Split

Source graph definitions are immutable at runtime. Runtime state, evidence,
events, and learned ranking signals live in SQLite tables. The model sees a
budgeted graph slice, never the whole graph unless a graph node selects it.

## Status

implemented.
