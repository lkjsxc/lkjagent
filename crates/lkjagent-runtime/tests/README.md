# lkjagent-runtime Tests

## Purpose

This directory holds pure runtime-step and thin daemon adapter tests.

## Table of Contents

- [maintenance.rs](maintenance.rs): idle cycle rotation, preemption, and early-close fixtures.
- [maintenance_authority.rs](maintenance_authority.rs): maintenance tool authority and local remote fixtures.
- [automatic_maintenance.rs](automatic_maintenance.rs): daemon idle maintenance and owner preemption fixtures.
- [budget_recovery.rs](budget_recovery.rs): exhausted task budget waiting and owner-send resume fixtures.
- [daemon_loop.rs](daemon_loop.rs): resident queue, endpoint, tool, ask, and error fixtures.
- [endpoint_retry.rs](endpoint_retry.rs): endpoint backoff deadline fixture.
- [file_count_aggregate_daemon.rs](file_count_aggregate_daemon.rs): aggregate count auto-scaffold fixture.
- [file_count_daemon.rs](file_count_daemon.rs): counted file daemon fixtures.
- [owner_guidance.rs](owner_guidance.rs): queued owner guidance guard persistence fixture.
- [prompt_daemon.rs](prompt_daemon.rs): prompt, startup, lock, and shutdown fixtures.
- [recursive_guard.rs](recursive_guard.rs): guarded recursive-structure daemon fixture.
- [recursive_scaffold.rs](recursive_scaffold.rs): docs auto-scaffold daemon fixture.
- [recursive_structure.rs](recursive_structure.rs): recursive structure seed integration fixture.
- [recovery_loop.rs](recovery_loop.rs): repeated recoverable-error daemon fixture.
- [step.rs](step.rs): task lifecycle, recovery, and compaction fixtures.
- [support/](support/README.md): shared state and store helpers.
- [task_budget.rs](task_budget.rs): configured task turn budget fixtures.
