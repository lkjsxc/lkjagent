# Reducer

## Purpose

Define the pure reducer that converts durable runtime state and one event into
one explainable runtime decision.

## Contract

The reducer owns mission selection and returns data only:

```text
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
```

It must not read files, SQLite, endpoint state, shell state, wall-clock time,
or environment. Dispatch, endpoint calls, compaction writes, maintenance
writes, and case closure happen after the decision.

## Snapshot Shape

```text
RuntimeSnapshot
- daemon_state
- queue_state
- active_case
- active_mission
- active_node
- owner_objective
- normalized_objective
- task_kind
- constraints
- assumptions
- risks
- artifact_ledger
- evidence_ledger
- fault_ledger
- tool_policy
- context_budget
- compaction_snapshot
- maintenance_state
- verification_state
- last_action
- last_observation
- last_successful_observation
- blocked_handoff
```

## Event Shape

```text
RuntimeEvent
- owner_message_received
- model_action_parsed
- parse_fault
- schema_fault
- tool_admission_request
- tool_observation
- tool_error
- repeat_action_detected
- payload_overflow_detected
- evidence_added
- artifact_audit_failed
- artifact_audit_passed
- verification_requested
- verification_passed
- verification_failed
- completion_requested
- context_pressure_detected
- compaction_completed
- maintenance_tick
- turn_budget_checkpoint
- turn_budget_exhausted
```

## Decision Shape

```text
RuntimeDecision
- mission
- state_node
- admitted_tools
- blocked_tools
- forced_next_action
- recommended_next_actions
- exact_valid_example
- missing_evidence
- completion_allowed
- completion_refusal
- recovery_plan
- compaction_required
- maintenance_allowed
- prompt_card
- persistence_writes
```

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

The first matching mission wins. Hard compaction never waits for a model
memory action. Owner recovery outranks normal execution. Maintenance can run
only after all owner, recovery, verification, and compaction missions are
absent.

## Evidence Invariants

- Planning evidence cannot satisfy `document-structure`.
- Graph evidence cannot satisfy `artifact-readiness`.
- Artifact readiness must name the current artifact id.
- Recovery evidence is required when any unresolved recovery fault exists.
- Missing evidence always yields one exact next executable action.

## Mechanical Tests Required

- Owner work blocks maintenance.
- Completion without artifact readiness is refused.
- Completion refusal admits artifact audit and repair tools.
- Parse recovery admits previous mission escape tools.
- Payload overflow admits `artifact.next` and `fs.batch_write`.
- Hard compaction preserves previous mission and exact next action.
- Turn-budget checkpoint continues autonomously when legal work remains.
- Turn-budget exhaustion produces a partial handoff only when no route remains.

## Failure Cases

- Reducer logic calls an effect or consults current time directly.
- Two decisions are emitted for one event.
- A decision omits missing evidence or next valid action.
- Completion and dispatch use different admission gates.

## Verification

Unit tests cover every event class and assert that decisions are pure data.
Integration tests assert dispatch uses the emitted admission result unchanged.

## Related Files

- [missions.md](missions.md)
- [tool-admission.md](tool-admission.md)
- [completion-policy.md](completion-policy.md)
