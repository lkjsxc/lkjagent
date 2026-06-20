# State Graph Model

## Purpose

Define the typed source graph and the runtime state envelope that every
meaningful task enters before endpoint execution.

## Graph Definition

The source graph is deterministic Rust data, not loose prompt text. It holds:

- `GraphNodeId` and `GraphEdgeId` identifiers.
- `GraphNode` records with `NodeKind`, label, instructions, context needs,
  evidence requirements, allowed actions, and policy.
- `GraphEdge` records with `EdgeKind`, source node, target node, typed guard
  list, and rationale.
- `GraphPolicy` values for context pressure, recovery limits, completion
  gates, shell demotion, document defaults, compaction preservation, and
  maintenance cadence.

Node kinds cover intent classification, planning, state tracking, context
selection, execution, document construction, memory, compaction, recovery,
completion, and maintenance.

## Task Case

The runtime creates or resumes a `TaskGraphState` for each owner task. The
state records:

- objective state with raw and normalized owner text plus non-goals.
- task family, active phase, active node, status, confidence, and budgets.
- constraints, assumptions, open questions, risks, invariants, success
  criteria, and decisions.
- structured plan steps, active step, target paths, required evidence, and
  verification checks.
- selected context packages, candidate paths, touched paths, and stale-context
  state.
- evidence requirements, evidence records, pending checks, completion state,
  recovery history, document topology state, and transition history.

The first owner message is input to classification and planning. It is not a
plan by itself.

## Source And Runtime Split

Source graph definitions are immutable at runtime. Runtime state, evidence,
events, plan steps, notes, context bindings, transitions, faults, artifacts,
and document state live in SQLite tables. The model sees a budgeted graph
slice, never the whole graph unless a graph node selects it.

## Status

implemented.
