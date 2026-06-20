# Current Blockers

## Purpose

The dependency-ordered implementation queue. Rows move to done only with the
evidence named by the task contract and the actual gates that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Recovery, maintenance, compaction, and graph policy can contradict each other | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 2 | Owner work does not reliably preempt maintenance | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 3 | Maintenance can save duplicate memory rows and claim pruning without delete or update | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 4 | Long-form content tasks can attempt giant writes and fail into parse loops | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 5 | Document and content tasks can complete after planning only | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 6 | Graph action examples can be syntactically valid but semantically rejected | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 7 | Transition recommendations can point to illegal or impossible targets | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 8 | Structured record identity and deduplication are missing | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 9 | The runtime lacks a deterministic active-mode controller | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 10 | Uploaded-failure benchmark coverage needed final verification | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | done |
| 11 | Status and console omit compact context and token accounting | [current-work/context-accounting.md](current-work/context-accounting.md) | done |
| 12 | No single current GPT handoff log exists | [current-work/gpt-log.md](current-work/gpt-log.md) | done |

## Owner Failure

The uploaded GPT-5.5-Pro logs are the active evidence. They show invalid
parameter loops, contradictory maintenance and graph policy layers, duplicate
memory writes, unsafe long-content writes, scaffold-only completion, and
maintenance restarts after no useful work.

## Ordering Notes

- Rows 1 through 9 remain open until focused tests and Docker Compose
  verification prove the uploaded failure patterns cannot recur.
- Row 10 is done for this slice: uploaded-loop fixtures exist and final quiet
  plus Docker Compose verification passed on 2026-06-20.
- Documentation moves first, then code. Do not mark prompt-only guidance done.
- Docker Compose verification is required for any implemented claim.

## Done

| # | Blocker | Task | Closing commit |
| --- | --- | --- | --- |
| 0 | Initial bootstrap queue | [tasks/](tasks/README.md) | completed before this reliability queue |
| 6 | Transition selector integration | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `a544a1c` |
| 7 | Repeated inspection loops | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `701a0bd` |
| 8 | `graph.note` examples | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | `35a422d` |
| 9 | Compaction policy contradiction | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `a73e162` |
| 10 | Large content unsafe writes | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `39bed37`, `a9764de` |
| 11 | Completion and waiting progress proof | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | `a544a1c` |
