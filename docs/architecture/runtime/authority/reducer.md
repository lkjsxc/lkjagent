# Reducer

## Purpose

Define the pure reducer that converts durable runtime state and one event into
one explainable runtime decision.

## Contract

The reducer owns mission selection and returns data only:

```text
RuntimeSnapshot + RuntimeEvent -> RuntimeFacts
RuntimeFacts -> Vec<Obligation>
Vec<Obligation> + RuntimeFacts -> ResolverPlan
ResolverPlan -> RuntimeDecision
```

It must not read files, SQLite, endpoint state, shell state, wall-clock time,
or environment. Dispatch, endpoint calls, compaction writes, maintenance
writes, and case closure happen after the decision.

## Snapshot Shape

A `RuntimeSnapshot` contains only durable read-model facts:

- case id, queue head, queue count, owner objective, normalized objective,
  task family, constraints, assumptions, risks, and active plan step;
- graph node, graph phase, legal transitions, selected context packages, and
  source graph policy facts;
- required evidence, missing evidence, existing evidence, evidence owners, and
  touched paths;
- artifact id, root, kind, profile, root status, weak paths, weak cursor,
  batch cursor, latest audit, and drift state;
- verification state, completion blockers, latest observation, latest
  successful observation, last action, and action fingerprints;
- fault class, retry count, recovery route, blocked handoff, provider anomaly
  class, provider anomaly budget, and provider exchange id;
- context pressure, compaction policy, compaction head, maintenance state,
  prompt frame head, authority fingerprint, and staleness fingerprint.

## Event Shape

`RuntimeEvent` is one observed fact such as owner intake, queue change, case
open or resume, prompt render, endpoint response, provider anomaly, parsed
model action, parse fault, schema fault, admission result, stale action,
repeated action, tool result, payload overflow, evidence addition, artifact
plan, write-contract readiness, audit result, weak-path discovery, verification
result, completion request, compaction boundary, maintenance result, turn budget
checkpoint, owner-input requirement, blocked handoff, or closed-idle selection.

## Decision Shape

Every `RuntimeDecision` records:

- case id, graph node, graph phase, mission, active mode, resolver plan, and
  decision kind;
- admitted tools, blocked tools, forced tool or runtime effect, and
  non-content exact action when applicable;
- content write contract when the model must author `fs.batch_write`;
- missing evidence, existing evidence, completion blockers, close allowance,
  and refusal text;
- artifact id, root, weak paths, cursor, fault class, retry count, provider
  anomaly budget, recovery route, and blocked handoff plan;
- compaction policy, compaction plan, maintenance plan, context packages, and
  next action surface;
- authority fingerprint, staleness fingerprint, persistence plan, and the rule
  explanation for the first matching mission-priority rule.

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

The first matching mission wins and records its explanation. Hard compaction
never waits for a model memory action. Owner recovery outranks schema repair
and normal execution. Maintenance can run only after all owner, recovery,
verification, and compaction missions are absent.

## Evidence Invariants

- Planning evidence cannot satisfy `document-structure`.
- Direct graph evidence cannot satisfy `artifact-readiness` or
  `document-structure`; audit-owned evidence comes from audits.
- Artifact readiness must name the current artifact id.
- Recovery evidence is required when any unresolved recovery fault exists.
- Missing evidence always yields one exact next executable action, content
  write contract, or deterministic runtime effect command.
- `missing_root` and `root_missing` facts yield a root identity write contract,
  not another same-root audit.

## Mechanical Tests Required

- Owner work blocks maintenance.
- Completion without artifact readiness is refused.
- Completion refusal admits artifact audit and repair tools.
- Parse recovery admits previous mission escape tools.
- Payload overflow admits `artifact.next` and `fs.batch_write`.
- Prompt-visible scaffold writers are never admitted.
- Repeated schema faults change the next action class before another loop.
- Hard compaction preserves previous mission and exact next action.
- Turn-budget checkpoint continues autonomously when legal work remains.
- Turn-budget exhaustion produces a partial handoff only when no route remains.

## Failure Cases

- Reducer logic calls an effect or consults current time directly.
- Two decisions are emitted for one event.
- A decision omits missing evidence, progress key, next valid action, write
  contract, or runtime effect when one is required.
- Completion and dispatch use different admission gates.

## Verification

Unit tests cover every event class and assert that decisions are pure data.
Integration tests assert dispatch uses the emitted admission result unchanged.

## Related Files

- [missions.md](missions.md)
- [tool-admission.md](tool-admission.md)
- [completion-policy.md](completion-policy.md)
