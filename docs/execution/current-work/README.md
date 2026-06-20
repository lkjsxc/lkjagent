# Current Work

## Purpose

This directory owns the reliability redesign work opened by the owner report:
semantic documentation generation, action recovery, state modeling,
observability, GPT handoff logging, and final verification.

## Table of Contents

- [owner-reported-failures.md](owner-reported-failures.md): confirmed user-visible failures.
- [action-fault-recovery.md](action-fault-recovery.md): parameter drift and recovery tasks.
- [document-structure-redesign.md](document-structure-redesign.md): scaffold and audit redesign.
- [context-accounting.md](context-accounting.md): context and token display work.
- [multi-state-runtime.md](multi-state-runtime.md): neutral task-track modeling work.
- [gpt-log.md](gpt-log.md): single handoff-log work.
- [verification-plan.md](verification-plan.md): focused tests, benchmarks, and compose gates.

## Reading Paths

- Implementation path: document-structure-redesign, action-fault-recovery, then context-accounting.
- Diagnosis path: owner-reported-failures, multi-state-runtime, then gpt-log.
- Verification path: verification-plan, then the gate commands it names.

## Cross-Links

- Related contract: [../../current-state.md](../../current-state.md).
- Owning queue: [../current-blockers.md](../current-blockers.md).
