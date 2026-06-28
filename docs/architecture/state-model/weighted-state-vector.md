# Weighted State Vector

## Purpose

Define the concurrent state tracks that add decision pressure without replacing
the hard lifecycle state machine.

## Hard And Weighted Layers

- Hard state owns lifecycle, phase, legal transitions, tool admission,
  blocked tools, and completion gates.
- Weighted tracks own concurrent pressure: risks, gaps, recovery modes,
  scheduling needs, and context needs.
- Track weights are independent decision pressures from `0.00` to `1.00`.
  They are never normalized to sum to one.
- Confidence is separate from weight and records how reliable the source is.
- Guards from dominant tracks can restrict tools, require audits, or promote a
  hard transition.

## Track Fields

Each track stores label, posture, weight, confidence, source, optional evidence
gap, optional guard, decay policy, update policy, and last update event.

Initial labels are objective-normalization, planning, document-structure,
artifact-contract, artifact-readiness, artifact-drift, parse-recovery,
action-param-reliability, tool-execution-recovery, evidence-gap,
context-pressure, context-snapshot-mismatch, queue-interruption,
completion-readiness, observability-ledger, and repeated-action-risk.

Initial postures are Planning, Structuring, Executing, Auditing, Repairing,
Recovering, Verifying, Observing, Compacting, Scheduling, and Blocking.

## Weight Bands

- `0.00` to `0.19`: inactive.
- `0.20` to `0.39`: weak.
- `0.40` to `0.59`: active.
- `0.60` to `0.79`: strong.
- `0.80` to `1.00`: dominant or guard-triggering.

## Deterministic Updates

Track updates are pure event reducers. The model never sets weights directly.

- ParseFault raises parse-recovery, raises action-param-reliability slightly,
  and lowers completion-readiness.
- Three consecutive ParseFault events raise parse-recovery to at least `0.90`,
  disable large payload actions, and prefer one small valid action.
- A valid parsed action lowers parse-recovery.
- ToolParameterFault raises action-param-reliability and records expected and
  received schema fields.
- Repeating the same invalid action raises repeated-action-risk and blocks that
  action signature during the recovery window.
- A doc.audit topology failure raises document-structure, lowers
  artifact-readiness, and requires topology repair before content continuation.
- A doc.audit pass lowers document-structure and may create audit-owned
  document-structure evidence.
- Artifact objective mismatch raises artifact-drift sharply, lowers
  artifact-readiness sharply, and blocks artifact.next until repair changes shape.
- Artifact audit pass lowers artifact-drift and raises artifact-readiness.
- Context soft pressure raises context-pressure; hard pressure promotes to
  compaction and blocks mutation until snapshot checks pass.
- Post-compaction mismatch raises context-snapshot-mismatch and blocks
  mutation.
- A new owner task raises queue-interruption until the scheduler classifies it.
- Complete evidence and passing audits raise completion-readiness.

## Guard Effects

- parse-recovery at or above `0.80` blocks large fs.batch_write and large
  large mutation batches, prefers graph.state, doc.audit, fs.list, fs.tree, and
  one small fs.write, and renders canonical grammar examples.
- artifact-drift at or above `0.75` blocks artifact.next
  until objective-match audit and repair pass.
- context-pressure at or above `0.85` blocks mutation and requires snapshot,
  compaction, and post-compaction consistency checks.
- queue-interruption at or above `0.70` requires queue or case classification
  before mutation and must not overwrite the active objective.
- completion-readiness never admits agent.done by itself; hard gates decide.
- repeated-action-risk blocks the same action signature in the current recovery
  window.

## Context Slice Selection

Dominant tracks select context slices:

- parse-recovery plus action-param-reliability selects action grammar, schema
  examples, last parser faults, invalid actions, and legal small actions.
- artifact-drift plus artifact-readiness selects owner objective, artifact
  contract, forbidden drift terms, drifted paths, objective-match audit, and
  repair plan.
- document-structure selects topology rules, doc.audit failures, missing
  README or link failures, and tree repair instructions.
- queue-interruption selects queue status, active case snapshot, and incoming
  owner-message classification rules.
- context-pressure selects budget, durable snapshot, and post-compaction
  checklist.

## Implementation Hooks

- Types live in `crates/lkjagent-graph/src/kernel_types.rs` and
  `crates/lkjagent-graph/src/kernel_events.rs`.
- Reducers live in `crates/lkjagent-graph/src/kernel_vector.rs`.
- Tool authorization lives in `crates/lkjagent-graph/src/kernel_authority.rs`.
- Focused regressions live in `crates/lkjagent-graph/tests/weighted_kernel.rs`.

## Completion Check

Every weighted track has update, guard, decay, display, and test policy. A
high guard track blocks agent.done even when completion-readiness is high.
