# Runtime

## Purpose

Define the durable decision loop that projects state into prompts, effects,
recovery, and completion.

## Table of Contents

- [authority-model.md](authority-model.md): canonical reducer, selector, and
  decision authority.
- [matters-and-obligations.md](matters-and-obligations.md): owner goals,
  predicates, invalidation, and completion eligibility.
- [operation-graph.md](operation-graph.md): semantic operations, typed edges,
  feasibility, and replanning.
- [loop-and-decisions.md](loop-and-decisions.md): snapshot hydration,
  selector candidates, `RuntimeDecision` fields, and prompt or effect selection.
- [recovery-and-completion.md](recovery-and-completion.md): crash recovery,
  evidence-gated closure, observability, and proof requirements.
- [artifact-units.md](artifact-units.md): small generation units,
  deterministic assembly, artifact manifests, and artifact-fingerprint checks.
- [waiting-and-quiescence.md](waiting-and-quiescence.md): visible wait states,
  wake sources, and daemon quiescence.

## Runtime Contract

Each cycle reduces fresh durable events before choosing work. The selected
`RuntimeDecision` stores the operation, state, prompt, tool, budget, recovery,
check, and wake fingerprints. Earlier blocked, active, failed, pending, or
unsuperseded operations prevent completion until recovery or supersession
evidence settles them.

## Failure This Prevents

Prompt rendering, tool admission, effect dispatch, recovery, resume, and
completion all use the same persisted authority row for a turn.
