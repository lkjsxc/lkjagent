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
| 6 | Runtime transition controller does not consume transition-quality scoring | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 7 | Recovery routes can admit unproductive repeated inspection loops | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 8 | `graph.note` examples and accepted kinds disagree | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | open |
| 9 | Compaction-only mode can conflict with active graph policy | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 10 | Large document tasks can attempt unsafe giant writes | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 11 | Completion and waiting gates need stronger progress proof | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 12 | Status and console omit compact context and token accounting | [current-work/context-accounting.md](current-work/context-accounting.md) | done |
| 13 | No single current GPT handoff log exists | [current-work/gpt-log.md](current-work/gpt-log.md) | done |

## Owner Failure

The harness can enter repeated parse, invalid recovery, compaction, mutation,
or owner-question loops even when docs and pure graph code know a better
route. Earlier failures also include semantically poor files such as
part-001.md and parameter faults such as unknown params [path].

## Ordering Notes

- Rows 1 and 2 must move before Rust behavior is claimed complete.
- Rows 3 and 4 are the first implementation slice because they have a direct
  confirmed code cause.
- Rows 5 through 11 may proceed after the document scaffold tests establish the
  shape contract.
- Docker Compose verification remains the final integration gate and waits for
  the preceding open rows.

## Done

| # | Blocker | Task | Closing commit |
| --- | --- | --- | --- |
| 0 | Initial bootstrap queue | [tasks/](tasks/README.md) | completed before this reliability queue |
