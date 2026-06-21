# Current Blockers

## Purpose

The dependency-ordered implementation queue. Rows move to done only with the
evidence named by the task contract and the actual gates that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Runtime authority is not the single source of active mission and tool admission | [current-work/runtime-authority-redesign.md](current-work/runtime-authority-redesign.md) | open |
| 2 | Recovery can block the exact observation, repair, or batch tool needed to escape | [current-work/runtime-recovery-controller.md](current-work/runtime-recovery-controller.md) | open |
| 3 | Content artifacts can pass on scaffold or planning evidence instead of real content | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 4 | Completion can close while artifact readiness, verification, or recovery evidence is missing | [current-work/runtime-authority-redesign.md](current-work/runtime-authority-redesign.md) | open |
| 5 | Compaction snapshots are not rich enough to resume artifact repair and recovery | [current-work/context-accounting.md](current-work/context-accounting.md) | open |
| 6 | Maintenance is not strictly idle-only and preemptable by owner work | [current-work/active-mode-controller.md](current-work/active-mode-controller.md) | open |
| 7 | Uploaded run logs are not fully covered by benchmark regressions | [current-work/verification-plan.md](current-work/verification-plan.md) | open |
| 8 | Semantic maintenance pruning still allows repeated low-value memory rows | [current-work/recovery-and-maintenance-loop-redesign.md](current-work/recovery-and-maintenance-loop-redesign.md) | open |
| 9 | Protocol schema repair can render examples that dispatch later rejects | [current-work/action-fault-recovery.md](current-work/action-fault-recovery.md) | open |

## Owner Failure

The uploaded GPT-5.5-Pro logs are the active evidence. They show invalid
parameter loops, contradictory maintenance and graph policy layers, duplicate
memory writes, unsafe long-content writes, scaffold-only completion, and
maintenance restarts after no useful work.

## Ordering Notes

- Rows stay open until focused tests and Docker Compose verification prove the
  uploaded failure patterns cannot recur.
- Documentation moves first, then code. Do not mark prompt-only guidance done.
- Runtime authority, recovery, artifact readiness, completion, compaction,
  maintenance, fixtures, memory, and protocol repair move in that order unless
  repository inspection proves a stricter dependency.
- Stable active-mode and artifact docs are contracts, not proof that the
  blockers are closed.
- Docker Compose verification is required for any implemented claim.
- Historical fixes may exist, but this queue tracks the current controller and
  artifact hardening work until it passes the current final gate.
- The expanded authority, artifact, recovery, and evaluation docs are contract
  text only until focused tests and Docker Compose verification prove the
  corresponding runtime behavior.
- Current code refuses direct `graph.evidence` for `artifact-readiness` and
  `document-structure`, and the uploaded-run corpus covers the
  artifact-readiness graph-evidence bypass. This is a focused proof, not
  closure of rows 1 to 4.
- Current code selects one hard-compaction active mode before owner intake,
  recovery, or maintenance when context pressure requires a runtime snapshot,
  and runtime authority examples reuse the dispatcher registry renderer. This
  narrows rows 1 and 9 but does not close them.
- Current code admits `artifact.next` and stricter content audit for bounded
  cookbook/story recovery, and quiet plus Docker Compose verification passed
  for this slice on 2026-06-20. Rows 7 to 10 stay open until completion wiring
  and broader regression coverage prove the uploaded failure pattern cannot
  recur across every close path.
- Current code rejects scaffold phrases in `fs.write`, `fs.batch_write`, and
  content audit; `fs.batch_write` preflights paths, duplicates, sizes, and
  count before mutation; `artifact.next` uses content-bearing examples plus a
  root-scoped SQLite cursor; and `graph.recover` omits `graph.plan` unless it
  is both admitted and still needed. Rows 1 to 4 and 7 stay open pending the
  full controller and completion wiring proof.
- Current memory pruning deletes exact duplicates and merges same-title
  high-overlap rows with source IDs, but rewrite pruning remains open under
  row 5.
