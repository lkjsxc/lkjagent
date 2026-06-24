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
- case_ref
- queue_head
- queue_pending_count
- owner_objective
- normalized_objective
- task_family
- active_phase
- active_node
- active_plan_step
- constraints
- assumptions
- risks
- candidate_paths
- touched_paths
- required_evidence
- missing_evidence
- evidence_owners
- artifact_head
- artifact_weak_paths
- artifact_batch_cursor
- verification_state
- fault_head
- fault_retry_summary
- recovery_ladder_state
- last_action
- last_action_fingerprint
- last_observation
- last_successful_observation
- context_pressure
- compaction_head
- maintenance_state
- blocked_handoff
- allowed_graph_transitions
- source_graph_policy
- prompt_frame_head
```

## Event Shape

```text
RuntimeEvent
- owner_message_received
- queue_changed
- case_opened
- case_resumed
- context_frame_built
- prompt_frame_rendered
- endpoint_call_requested
- endpoint_response_received
- endpoint_fault
- model_action_parsed
- parse_fault
- schema_fault
- tool_admission_requested
- tool_admission_refused
- tool_started
- tool_succeeded
- tool_failed
- repeat_action_detected
- payload_overflow_detected
- evidence_added
- artifact_planned
- artifact_applied
- artifact_audited
- artifact_weak_path_found
- verification_requested
- verification_passed
- verification_failed
- completion_requested
- completion_blocked
- case_closed
- context_pressure_detected
- compaction_started
- compaction_completed
- maintenance_tick
- maintenance_started
- maintenance_noop
- maintenance_completed
- turn_budget_checkpoint
- turn_budget_exhausted
- owner_input_required
- blocked_handoff_recorded
```

## Decision Shape

```text
RuntimeDecision
- mission
- active_mode
- state_node
- decision_kind
- admitted_tools
- blocked_tools
- forced_next_action
- recommended_next_actions
- exact_valid_example
- missing_evidence
- existing_evidence
- completion_allowed
- completion_refusal
- recovery_plan
- compaction_plan
- maintenance_plan
- blocked_handoff_plan
- context_package_ids
- prompt_card
- runtime_effect_command
- persistence_plan
- authority_fingerprint
- staleness_fingerprint
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
- Missing evidence always yields one exact next executable action or one
  deterministic runtime effect command.

## Mechanical Tests Required

- Owner work blocks maintenance.
- Completion without artifact readiness is refused.
- Completion refusal admits artifact audit and repair tools.
- Parse recovery admits previous mission escape tools.
- Payload overflow admits `artifact.next` and `fs.batch_write`.
- Repeated schema faults change the next action class before another loop.
- Hard compaction preserves previous mission and exact next action.
- Turn-budget checkpoint continues autonomously when legal work remains.
- Turn-budget exhaustion produces a partial handoff only when no route remains.

## Failure Cases

- Reducer logic calls an effect or consults current time directly.
- Two decisions are emitted for one event.
- A decision omits missing evidence, next valid action, or runtime effect when
  no model-authored action is needed.
- Completion and dispatch use different admission gates.

## Verification

Unit tests cover every event class and assert that decisions are pure data.
Integration tests assert dispatch uses the emitted admission result unchanged.

## Related Files

- [missions.md](missions.md)
- [tool-admission.md](tool-admission.md)
- [completion-policy.md](completion-policy.md)
