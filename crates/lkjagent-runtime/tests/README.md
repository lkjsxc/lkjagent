# lkjagent-runtime Tests

## Purpose

This directory holds pure runtime-step and thin daemon adapter tests.

## Table of Contents

- [maintenance.rs](maintenance.rs): idle cycle rotation, preemption, and early-close fixtures.
- [maintenance_authority.rs](maintenance_authority.rs): maintenance tool authority and local remote fixtures.
- [active_mode.rs](active_mode.rs): active-mode selection and policy rendering fixtures.
- [automatic_maintenance.rs](automatic_maintenance.rs): daemon idle maintenance and owner preemption fixtures.
- [authority_reducer.rs](authority_reducer.rs): pure runtime decision and admission fixtures.
- [authority_examples.rs](authority_examples.rs): runtime authority valid-example renderer fixtures.
- [authority_redesign.rs](authority_redesign.rs): uploaded-log authority redesign fixtures.
- [artifact_completion_gate.rs](artifact_completion_gate.rs): content artifact readiness completion fixtures.
- [artifact_ledger_completion.rs](artifact_ledger_completion.rs): artifact ledger completion gate fixtures.
- [audit_owned_evidence.rs](audit_owned_evidence.rs): audit-owned evidence cannot be graph-authored.
- [budget_recovery.rs](budget_recovery.rs): exhausted task budget waiting and owner-send resume fixtures.
- [compaction_snapshot.rs](compaction_snapshot.rs): structured runtime compaction resume fields.
- [current_model_run_fixture.rs](current_model_run_fixture.rs): checked-in current model-run failure fixture.
- [daemon_loop.rs](daemon_loop.rs): resident queue, endpoint, tool, ask, and error fixtures.
- [document_completion.rs](document_completion.rs): document audit evidence completion fixtures.
- [endpoint_retry.rs](endpoint_retry.rs): endpoint backoff deadline fixture.
- [fault_wait.rs](fault_wait.rs): repeated fault graph recovery routing fixtures.
- [file_count_aggregate_daemon.rs](file_count_aggregate_daemon.rs): aggregate count auto-scaffold fixture.
- [file_count_daemon.rs](file_count_daemon.rs): counted file daemon fixtures.
- [graph_memory_links.rs](graph_memory_links.rs): graph case to task-summary memory links.
- [graph_prefix_budget.rs](graph_prefix_budget.rs): startup graph guard prefix budget fixture.
- [kernel_completion.rs](kernel_completion.rs): kernel close-case and completion-block fixtures.
- [kernel_driver_wiring.rs](kernel_driver_wiring.rs): daemon authority driver completion-event fixture.
- [owner_guidance.rs](owner_guidance.rs): queued owner guidance guard persistence fixture.
- [payload_risk.rs](payload_risk.rs): max-token write payload recovery fixture.
- [pending_action_authority.rs](pending_action_authority.rs): pending action authority id propagation fixture.
- [prompt_daemon.rs](prompt_daemon.rs): prompt, startup, lock, and shutdown fixtures.
- [prompt_hygiene.rs](prompt_hygiene.rs): live prompt hidden-reasoning hygiene fixture.
- [provider_anomaly.rs](provider_anomaly.rs): provider anomaly recovery without parse-fault fixtures.
- [recursive_guard.rs](recursive_guard.rs): guarded recursive-structure daemon fixture.
- [recursive_scaffold.rs](recursive_scaffold.rs): docs auto-scaffold daemon fixture.
- [recursive_structure.rs](recursive_structure.rs): recursive structure seed integration fixture.
- [recovery_controller.rs](recovery_controller.rs): graph selector recovery routing fixtures.
- [recovery_loop.rs](recovery_loop.rs): repeated recoverable-error daemon fixture.
- [step.rs](step.rs): task lifecycle, recovery, and compaction fixtures.
- [support/](support/README.md): shared state and store helpers.
- [task_budget.rs](task_budget.rs): configured task turn budget fixtures.
- [turn_authority.rs](turn_authority.rs): pure active-mode authority matrix.
- [turn_authority_runtime.rs](turn_authority_runtime.rs): endpoint card and closed-idle authority fixtures.
