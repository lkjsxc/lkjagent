# Transition Network

## Purpose

This file owns the state-transition network that connects runtime authority,
graph guidance, recovery, compaction, maintenance, artifacts, and completion.

## Contract

The network is event driven. The graph ranks nodes and proposes edges, but
runtime authority decides the active mission and the admitted next action.
Every prompt, dispatch request, recovery route, compaction snapshot,
maintenance tick, and close path reads the same decision stream.

## Kernel Boundary

```text
DurableReadModel -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeFacts
RuntimeFacts -> Vec<Obligation>
Vec<Obligation> + RuntimeFacts -> TotalResolverPlan
TotalResolverPlan -> RuntimeDecision
RuntimeDecision -> PromptFrame or RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> EffectCommand
EffectObservation -> RuntimeEvent
RuntimeEvent -> next RuntimeDecision
```

The kernel reducer is pure. Store, prompt, endpoint, dispatcher, compaction,
maintenance, and completion adapters perform effects only after a persisted
decision exists. Zero-content inspections and runtime bookkeeping can be
runtime-owned effect commands when the decision says no model-authored semantic
content is needed.

## Inputs

- case envelope and active graph state.
- source graph node and edge guidance.
- runtime events, facts, obligations, resolver plans, and decisions.
- artifact, evidence, fault, verification, maintenance, and compaction ledgers.
- prompt frame and context snapshot identifiers.

## Outputs

- state transition record for each accepted move.
- authority head id for the active case.
- next prompt frame and dispatch admission view.
- blocked handoff when recovery routes are exhausted.

## Invariants

- Owner work follows intake, contract, plan, context, execute, observe, evidence, verify, audit, and complete states.
- Recovery records fault, classification, route, shrunk action surface, escape action, observation, and resume state.
- Compaction records pressure, pre-snapshot, log compaction, prompt rebuild, post-snapshot, and resume state.
- Maintenance begins only from closed idle and returns to closed idle after a bounded audit.
- Blocked handoff records exact missing evidence before waiting for new owner input.

## Failure Cases

- Graph transition and runtime mission disagree about the legal tool.
- Recovery repeats the same invalid action class.
- Graph guidance routes a `missing_root` fact back to same-root `doc.audit`.
- Maintenance renders owner graph policy.
- Completion closes after planning or scaffold-only evidence.

## Verification

- reducer tests for mission priority and event classes.
- graph transition tests for legal movement.
- dispatch tests proving graph policy is guidance, not a fallback authority.
- replay tests proving story root repair and schema loops change route instead
  of repeating the same invalid action class.

## Status

implemented for the persisted daemon driver described by
[../runtime/authority/transition-kernel.md](../runtime/authority/transition-kernel.md).
Runtime mission selection, decision records, event and admission persistence,
prompt-card decision identifiers, authority fingerprints, prompt rendering,
provider exchange, dispatch observation, maintenance, compaction, and completion
all run through the persisted decision id on the daemon path.

Active work is density and totality, not a second driver. The current slice
stores facts, obligations, resolver plans, progress edges, deterministic effect
rows, and completion gate inputs as first-class records; removes the mission
fallback branch after resolver planning; and proves prompt/admission staleness
with a shared fingerprint.
