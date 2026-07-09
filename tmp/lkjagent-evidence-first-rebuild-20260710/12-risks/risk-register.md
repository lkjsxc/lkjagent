# Risk Register

## High

- Native-state rewrite temporarily creates two authorities.
  Mitigation: switch one vertical flow at a time, then delete old reads before
  releasing the next flow.
- Filesystem and SQLite cannot share one atomic transaction.
  Mitigation: prepared effect journal, atomic rename, idempotency, recovery
  matrix, and crash injection.
- Real endpoint experiments are noisy and costly.
  Mitigation: deterministic corpus first, controlled repeats, bounded cells,
  and task-level adoption.
- Context sophistication can add latency and stale summaries.
  Mitigation: incremental indexes, source fingerprints, cost telemetry, and
  direct-source fallback.

## Medium

- Strict XML may reduce semantic freedom.
  Mitigation: state-specific envelopes and measured grammar comparison.
- 512-token pages can create excessive fragmentation.
  Mitigation: semantic outlines, bounded README hierarchy, and scope only to
  managed memory documents.
- TUI canonical schema may require store changes across app and tests.
  Mitigation: land schema and read projection before producer switch.
- Always-on expectations may cause wasteful model calls.
  Mitigation: distinguish useful active work from true quiescence and schedule
  deterministic maintenance only when due.

## Low

- Fresh store reset may strand old runtime logs.
  Mitigation: preserve workspace, keep redacted fixtures, and rebuild indexes.
