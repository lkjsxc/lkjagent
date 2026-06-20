# State Graph Model

## Purpose

Define the typed source graph and the runtime state envelope that every
meaningful task enters before endpoint execution.

## Graph Definition

The source graph is deterministic Rust data, not loose prompt text. It holds:

- `GraphNodeId` and `GraphEdgeId` identifiers.
- `GraphNode` records with `NodeKind`, label, purpose, concise model
  instruction, context needs, evidence requirements, allowed tools, and
  typed node policy.
- `GraphEdge` records with `EdgeKind`, source node, target node, typed guard
  list, priority, auto-admission policy, state delta label, recovery target,
  and debug rationale.
- `GraphPolicy` values for context pressure, recovery limits, completion
  gates, shell demotion, document defaults, compaction preservation, and
  maintenance cadence.
- `ContextPackage` records selected by family, active node, pressure, missing
  evidence, and recovery state.

Node kinds cover intent classification, planning, state tracking, context
selection, execution, document construction, memory, compaction, recovery,
completion, and maintenance.

Node policy names what the node reads, what it may update, allowed tools,
preferred tools, blocked tools, selected packages, package compression,
recovery ladder, completion contribution, and maintenance contribution.

## Task Case

The runtime creates or resumes a `TaskGraphState` for each owner task. The
state records:

- objective state with raw and normalized owner text, an internal counter, and
  non-goals.
- task family, active phase, active node, status, confidence, and budgets.
- constraints, assumptions, open questions, risks, invariants, success
  criteria, and decisions.
- structured plan steps, active step, target paths, required evidence, and
  verification checks.
- selected context packages, candidate paths, touched paths, and stale-context
  state.
- evidence requirements, evidence records, pending checks, completion state,
  health pressure, recovery history, document topology state, and transition
  history.

The first owner message is input to classification and planning. It is not a
plan by itself.

## Source And Runtime Split

Source graph definitions are immutable at runtime. Runtime state, evidence,
events, plan steps, notes, context bindings, transitions, faults, artifacts,
and document state live in SQLite tables. The model sees a budgeted graph
slice, never the whole graph unless a graph node selects it.

## Status

implemented.
