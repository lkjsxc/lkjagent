# Current Blockers

## Purpose

The dependency-ordered implementation queue. Rows move to done only with the
evidence named by the task contract and the actual gates that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Active mode is not authoritative across owner work, recovery, maintenance, and compaction | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 2 | Maintenance and graph policy can both reject each other's allowed actions | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 3 | Compaction can require memory.save while graph policy refuses memory.save | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 4 | Generated action examples can be rejected by semantic dispatch | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 5 | memory.save is not idempotent enough and maintenance creates duplicate rows | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 6 | memory.find can pass unsafe FTS queries | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 7 | Long content tasks can attempt giant fs.write and enter parse loops | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 8 | Content artifacts can be scaffolded as generic project docs instead of semantic artifact trees | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 9 | Recovery nodes can block doc.scaffold, fs.write, graph.plan, or graph.evidence when needed | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 10 | Completion can close scaffold-only or planning-only tasks | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 11 | Owner questions can be used for internal runtime or tool uncertainty | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 12 | Uploaded run logs are not fully covered by benchmark regressions | [current-work/verification-plan.md](current-work/verification-plan.md) | open |

## Owner Failure

The uploaded GPT-5.5-Pro logs are the active evidence. They show invalid
parameter loops, contradictory maintenance and graph policy layers, duplicate
memory writes, unsafe long-content writes, scaffold-only completion, and
maintenance restarts after no useful work.

## Ordering Notes

- Rows stay open until focused tests and Docker Compose verification prove the
  uploaded failure patterns cannot recur.
- Documentation moves first, then code. Do not mark prompt-only guidance done.
- Stable active-mode and artifact docs are contracts, not proof that the
  blockers are closed.
- Docker Compose verification is required for any implemented claim.
- Historical fixes may exist, but this queue tracks the current controller and
  artifact hardening work until it passes the current final gate.
- Current code admits `artifact.next` and stricter content audit for bounded
  cookbook/story recovery, but rows 7 to 10 stay open until completion wiring,
  benchmark corpus, quiet verify, and Docker Compose verify prove the uploaded
  failure pattern cannot recur.
