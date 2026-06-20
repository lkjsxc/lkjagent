# Current Work

## Purpose

This directory owns reliability redesign work opened by owner reports:
semantic documentation generation, action recovery, state modeling,
observability, GPT handoff logging, loop recovery, maintenance, and final
verification.

## Table of Contents

- [owner-reported-failures.md](owner-reported-failures.md): confirmed user-visible failures.
- [action-fault-recovery.md](action-fault-recovery.md): parameter drift and recovery tasks.
- [document-structure-redesign.md](document-structure-redesign.md): scaffold and audit redesign.
- [context-accounting.md](context-accounting.md): context and token display work.
- [multi-state-runtime.md](multi-state-runtime.md): neutral task-track modeling work.
- [runtime-recovery-controller.md](runtime-recovery-controller.md): deterministic recovery controller work.
- [recovery-and-maintenance-loop-redesign.md](recovery-and-maintenance-loop-redesign.md): active-mode loop redesign.
- [gpt-log.md](gpt-log.md): single handoff-log work.
- [verification-plan.md](verification-plan.md): focused tests, benchmarks, and compose gates.

## Reading Paths

- Implementation path: document-structure-redesign, action-fault-recovery,
  runtime-recovery-controller, then context-accounting.
- Diagnosis path: owner-reported-failures, multi-state-runtime,
  runtime-recovery-controller, then gpt-log.
- Verification path: verification-plan, then the gate commands it names.

## Cross-Links

- Related contract: [../../current-state.md](../../current-state.md).
- Owning queue: [../current-blockers.md](../current-blockers.md).
