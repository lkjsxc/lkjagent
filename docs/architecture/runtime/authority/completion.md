# Completion

## Purpose

Define close eligibility for owner tasks and the refusal shape for premature
`agent.done`.

## Contract

Owner work is close eligible only when these facts exist:

- Accepted normalized owner objective.
- Observed mutation or artifact adoption for artifact tasks.
- Focused verification evidence.
- Content readiness for content artifacts.
- No unresolved recovery ladder.
- No policy contradiction.
- Durable completion summary that references actual observed effects.

Content tasks also require an artifact ledger entry, observed files, structural
audit, profile-specific content readiness, and final verification.

## Invariants

- Scaffold, plan, file existence, or optimistic verification note is not enough.
- `agent.done` uses the same gate as every close path.
- Partial completion is legal only when the owner objective explicitly allows
  partial output and the summary states the partial scope.
- Refusal must include missing evidence and one admitted next valid action.

## Failure Cases

- Dictionary task closes after a shallow terminology list.
- Cookbook task closes after a well-shaped scaffold with weak leaves.
- Graph state says complete while artifact audit is missing.
- Completion summary claims pronunciation or examples that were not observed.

## Verification

Tests refuse `agent.done` when readiness, verification, or recovery evidence is
missing. Benchmark fixtures assert shallow dictionary and cookbook scaffold
cases remain open and admit repair.

## Related Files

- [tool-admission.md](tool-admission.md)
- [../../artifacts/content-readiness.md](../../artifacts/content-readiness.md)
- [../../structured-records/completion-evidence.md](../../structured-records/completion-evidence.md)
