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
| 6 | Runtime transition controller does not consume transition-quality scoring after every state-changing event | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 7 | Recovery routes can admit unproductive repeated inspection loops | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | done |
| 8 | `graph.note` examples and accepted kinds disagree | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | done |
| 9 | Compaction-only mode can conflict with active graph policy | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | done |
| 10 | Large document tasks can attempt unsafe giant writes | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | done |
| 11 | Completion and waiting gates need stronger progress proof | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 12 | Status and console omit compact context and token accounting | [current-work/context-accounting.md](current-work/context-accounting.md) | done |
| 13 | No single current GPT handoff log exists | [current-work/gpt-log.md](current-work/gpt-log.md) | done |

## Owner Failure

The main remaining risk is incomplete use of transition selection after every
state-changing runtime event and partial-completion handoff proof. Earlier
failures also include semantically poor files such as part-001.md and
parameter faults such as unknown params [path].

## Ordering Notes

- Rows 1 and 2 must move before Rust behavior is claimed complete.
- Rows 3 and 4 are the first implementation slice because they have a direct
  confirmed code cause.
- Rows 5 through 11 may proceed after the document scaffold tests establish the
  shape contract.
- Docker Compose verification remains the final integration gate.

## Done

| # | Blocker | Task | Closing commit |
| --- | --- | --- | --- |
| 0 | Initial bootstrap queue | [tasks/](tasks/README.md) | completed before this reliability queue |
| 7 | Repeated inspection loops | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `701a0bd` |
| 8 | `graph.note` examples | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | `35a422d` |
| 9 | Compaction policy contradiction | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `a73e162` |
| 10 | Large content unsafe writes | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `39bed37`, `a9764de` |
