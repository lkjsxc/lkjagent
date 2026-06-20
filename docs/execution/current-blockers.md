# Current Blockers

## Purpose

The dependency-ordered implementation queue. Rows move to done only with the
evidence named by the task contract and the actual gates that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Owner-reported reliability failures are not recorded as active work | [current-work/owner-reported-failures.md](current-work/owner-reported-failures.md) | done |
| 2 | Documentation lacks a semantic tree and graph contract | [current-work/document-structure-redesign.md](current-work/document-structure-redesign.md) | done |
| 3 | `doc.scaffold` can emit sequence-named child files | [current-work/document-structure-redesign.md](current-work/document-structure-redesign.md) | done |
| 4 | `doc.audit` does not enforce the desired topology contract | [current-work/document-structure-redesign.md](current-work/document-structure-redesign.md) | done |
| 5 | Action parameter drift can produce weak unknown-param loops | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | done |
| 6 | Owner messages need stronger objective envelopes and candidate tracks | [current-work/multi-state-runtime.md](current-work/multi-state-runtime.md) | focused tests pass; controller use pending |
| 7 | Status and console omit compact context and token accounting | [current-work/context-accounting.md](current-work/context-accounting.md) | done |
| 8 | No single current GPT handoff log exists | [current-work/gpt-log.md](current-work/gpt-log.md) | done |
| 9 | Recovery nodes are not specific enough for parameter faults | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | done |
| 10 | Benchmarks do not cover the owner-reported failures | [current-work/verification-plan.md](current-work/verification-plan.md) | done |
| 11 | Compose smoke evidence is missing after the reliability redesign | [current-work/verification-plan.md](current-work/verification-plan.md) | done |

## Owner Failure

The harness can generate semantically poor documentation files such as
part-001.md and can loop on action parameter faults such as unknown params
[path].

## Ordering Notes

- Rows 1 and 2 must move before Rust behavior is claimed complete.
- Rows 3 and 4 are the first implementation slice because they have a direct
  confirmed code cause.
- Rows 5 through 9 may proceed after the document scaffold tests establish the
  shape contract.
- Row 11 is the final integration gate and waits for the preceding rows.

## Done

| # | Blocker | Task | Closing commit |
| --- | --- | --- | --- |
| 0 | Initial bootstrap queue | [tasks/](tasks/README.md) | completed before this reliability queue |
