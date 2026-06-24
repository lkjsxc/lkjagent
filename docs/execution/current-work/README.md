# Current Work

## Purpose

This directory owns reliability redesign work opened by owner reports:
semantic documentation generation, action recovery, state modeling,
observability, model handoff logging, loop recovery, maintenance, and final
verification.

## Table of Contents

- [owner-reported-failures.md](owner-reported-failures.md): confirmed user-visible failures.
- [action-fault-recovery.md](action-fault-recovery.md): parameter drift and recovery tasks.
- [document-structure-redesign.md](document-structure-redesign.md): scaffold and audit redesign.
- [artifact-address-controller.md](artifact-address-controller.md): root, path, and weak-path repair.
- [context-accounting.md](context-accounting.md): context and token display work.
- [multi-state-runtime.md](multi-state-runtime.md): neutral task-track modeling work.
- [runtime-recovery-controller.md](runtime-recovery-controller.md): deterministic recovery controller work.
- [recovery-shape-enforcement.md](recovery-shape-enforcement.md): per-fault shape-change enforcement.
- [runtime-authority-redesign.md](runtime-authority-redesign.md): authority reducer execution plan.
- [state-transition-network.md](state-transition-network.md): unified authority and graph decision network.
- [active-mode-controller.md](active-mode-controller.md): one owner of policy per turn.
- [recovery-and-maintenance-loop-redesign.md](recovery-and-maintenance-loop-redesign.md): active-mode loop redesign.
- [artifact-ledger-completion.md](artifact-ledger-completion.md): ledger-backed artifact readiness and close gates.
- [durable-compaction-history.md](durable-compaction-history.md): rich compaction snapshots and resume history.
- [workspace-structure-controller.md](workspace-structure-controller.md): recursive workspace and docs rebalancer.
- [model-log.md](model-log.md): single handoff-log work.
- [personal-records.md](personal-records.md): diary, schedule, and TODO record work.
- [verification-plan.md](verification-plan.md): focused tests, benchmarks, and compose gates.

## Reading Paths

- Implementation path: state-transition-network, runtime-authority-redesign,
  recovery-shape-enforcement, artifact-ledger-completion, durable-compaction-history,
  workspace-structure-controller, personal-records, then context-accounting.
- Diagnosis path: owner-reported-failures, multi-state-runtime,
  runtime-recovery-controller, then model-log.
- Verification path: verification-plan, then the gate commands it names.

## Cross-Links

- Related contract: [../../current-state.md](../../current-state.md).
- Owning queue: [../current-blockers.md](../current-blockers.md).
