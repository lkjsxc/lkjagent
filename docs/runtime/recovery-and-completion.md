# Recovery And Completion

## Purpose

Define crash recovery, evidence-gated closure, and runtime observability.

## Crash Recovery

On boot, recovery checks unfinished decisions before selecting new work. If a
decision was persisted before an endpoint call but no exchange, admission, or
observation was committed, the runtime retries the same decision when safe or
commits a bounded recovery event. It does not recompute a new tool view and
pretend it rendered the old prompt.

If an admission was persisted before a tool effect, resume either reruns the
idempotent effect or records a recovery event for non-idempotent work. Recovery
facts are durable events and may produce recovery state cells.

## Completion

Completion is represented by state cells such as `completion:check-pending/*`,
`completion:check-passed`, `completion:check-failed`, `completion:blocked`, and
`completion:close-candidate`. A pending deterministic verify cell carries the
native `check.run/<step>` operation. Each recorded check row emits a native
`completion:check-passed/*` or `completion:check-failed/*` cell with evidence
refs. Hydration suppresses passed check rows that lack matching active native
completion cells. Closing a case requires no pending, active, blocked, failed,
or unsuperseded skipped operation and current passing check results tied to
artifact fingerprints and the active decision. Model prose is not completion
evidence.

While plan-family rows remain as bridge storage, they are blocking evidence.
Any blocked, active, pending, failed, or unsuperseded skipped bridge step keeps
the matter open or blocked even if task-level checks are empty or model prose
claims success.

If an artifact changes after a passing check, the reducer suppresses dependent
`completion:check-passed` cells or creates a fresh check requirement.

## Observability

Status, logs, and proof bundles render case state, active state cells, decision
ids and fingerprints, prompt-frame refs, tool-view summaries, admissions,
observations, context conflicts, contamination suppressions, artifact
fingerprints, check results, exchange refs, token usage, and recovery events.

## Failure This Prevents

A crash cannot create false completion, stale prompt authority, or unexplained
refused-tool behavior.
