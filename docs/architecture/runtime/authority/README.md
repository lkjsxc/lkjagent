# Runtime Authority

## Purpose

Define the runtime-owned authority layer that decides active mission, tool
admission, compaction ownership, maintenance eligibility, recovery shape, and
completion closure.

## Contract

Runtime authority is the only source of action truth. Model output is intent.
Graph transitions are guidance. The old runtime `mode` tree is an adapter only,
not a parallel source of action truth. Context pressure, maintenance ticks,
verifier results, tool observations, parser faults, provider anomalies, and
completion requests are events. The pure reducer derives facts, obligations,
and a resolver plan before emitting one decision.

```text
RuntimeSnapshot + RuntimeEvent -> RuntimeFacts
RuntimeFacts -> Vec<Obligation>
Vec<Obligation> + RuntimeFacts -> ResolverPlan
ResolverPlan -> RuntimeDecision
RuntimeDecision + requested_tool -> ToolAdmission
```

## Decision Boundary

The reducer decides the active mission, admitted tools, blocked tools, missing
evidence, exact next action, recovery plan, compaction requirement, maintenance
eligibility, persistence writes, and prompt card. Effects may execute only the
admitted action named by that decision.

The graph may rank states, provide context packages, and suggest transitions.
It may not close a case, admit `agent.done`, start maintenance, or remove the
repair tools needed by the current mission.

## Mission Priority

```text
hard_runtime_compaction
owner_recovery
schema_repair
artifact_repair
verification_repair
owner_execution
owner_verification
owner_completion
idle_maintenance
closed_idle
```

Higher missions preempt lower missions. Maintenance is lower than every owner,
recovery, verification, and compaction mission.

## Invariants

- The reducer has no I/O, clock reads, filesystem reads, SQLite reads, or
  endpoint calls.
- Every decision names active mission, admitted tools, blocked tools, missing
  evidence, completion state, write contract state, and next valid action.
- Completion must pass the same gate on every close path.
- Turn-budget checkpoints are continuation decisions, not owner-permission
  prompts.
- Recovery must preserve the read, audit, repair, and batch tools needed by the
  mission that failed.
- Missing-root audit facts create root identity write obligations; they do not
  route to another same-root `doc.audit` before write progress.
- Compaction is runtime-owned and persists resume data without a model-authored
  memory action.
- Maintenance is strictly idle-only and must be preempted by owner work.

## Uploaded Failure Mapping

- Early `agent.done` maps to completion-policy and evidence invariants.
- Maintenance memory writes during owner work map to maintenance-policy
  invariants.
- Large raw writes map to compaction-policy and batch recovery invariants.
- Invalid `fs.batch_write` examples map to schema-repair invariants.
- Scaffold-only cookbook leaves map to artifact-readiness invariants.
- Recovery blocking artifact tools maps to tool-admission invariants.
- Turn-budget exhaustion maps to compaction and partial-handoff invariants.

## Verification

Authority is verified by pure reducer tests, dispatcher admission tests,
completion-refusal tests, compaction snapshot tests, maintenance-preemption
tests, schema-example tests, artifact-readiness tests, and uploaded run-log
benchmark fixtures.

## Table of Contents

- [reducer.md](reducer.md): pure snapshot, event, and decision contract.
- [transition-kernel.md](transition-kernel.md): durable turn sequence around the reducer.
- [kernel-driver.md](kernel-driver.md): effectful driver sequence and persistence order.
- [snapshot-ledger.md](snapshot-ledger.md): durable read-side snapshot fields.
- [event-catalog.md](event-catalog.md): closed event set that drives decisions.
- [decision-ledger.md](decision-ledger.md): persisted decision record and invariants.
- [effect-commands.md](effect-commands.md): runtime-owned effect commands.
- [admission-view.md](admission-view.md): immutable dispatch admission view.
- [admission-flow.md](admission-flow.md): accepted and refused dispatch path.
- [wiring-map.md](wiring-map.md): reducer and admission ownership for runtime paths.
- [missions.md](missions.md): owner, recovery, verification, maintenance, compaction, and idle rules.
- [turn-authority.md](turn-authority.md): single turn decision object.
- [mode-priority.md](mode-priority.md): deterministic mission priority order.
- [tool-admission.md](tool-admission.md): explainable admission and refusal data.
- [owner-questions.md](owner-questions.md): strict external-question admission gate.
- [tool-policy.md](tool-policy.md): admitted and blocked tool classes.
- [evidence-policy.md](evidence-policy.md): evidence ledger requirements.
- [completion.md](completion.md): close eligibility and refusal contract.
- [completion-policy.md](completion-policy.md): close gate inputs and refusal output.
- [recovery-policy.md](recovery-policy.md): fault recovery ownership.
- [maintenance.md](maintenance.md): idle-only maintenance policy.
- [maintenance-policy.md](maintenance-policy.md): maintenance eligibility and effect rules.
- [compaction.md](compaction.md): runtime-owned snapshot and resume contract.
- [compaction-policy.md](compaction-policy.md): hard and soft compaction authority.
- [exact-examples.md](exact-examples.md): schema-rendered valid action examples.

## Related Files

- [../obligation-network/README.md](../obligation-network/README.md)
- [../active-mode/README.md](../active-mode/README.md)
- [../../state-graph/README.md](../../state-graph/README.md)
- [../../artifacts/README.md](../../artifacts/README.md)
