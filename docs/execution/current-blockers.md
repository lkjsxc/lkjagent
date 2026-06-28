# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Truth sweep and fixture root reconciliation | [tasks/deep-redesign-truth-sweep.md](tasks/deep-redesign-truth-sweep.md) | open |
| 2 | Compact context and no object-literal model context | [tasks/deep-redesign-compact-context.md](tasks/deep-redesign-compact-context.md) | open |
| 3 | Output budget contract and endpoint config | [tasks/deep-redesign-output-budget.md](tasks/deep-redesign-output-budget.md) | open |
| 4 | Short artifact path aliases and planner | [tasks/deep-redesign-short-paths.md](tasks/deep-redesign-short-paths.md) | open |
| 5 | Registry-derived exact action examples | [tasks/deep-redesign-exact-examples.md](tasks/deep-redesign-exact-examples.md) | open |
| 6 | Narrow runtime authority prompt cards | [tasks/deep-redesign-runtime-authority.md](tasks/deep-redesign-runtime-authority.md) | open |
| 7 | Artifact cursor micro-batches | [tasks/deep-redesign-artifact-batches.md](tasks/deep-redesign-artifact-batches.md) | open |
| 8 | Completion and maintenance reducers | [tasks/deep-redesign-completion-maintenance.md](tasks/deep-redesign-completion-maintenance.md) | open |
| 9 | Provider anomaly blocked handoff | [tasks/deep-redesign-provider-handoff.md](tasks/deep-redesign-provider-handoff.md) | open |
| 10 | Benchmark corpus and final gates | [tasks/deep-redesign-gates.md](tasks/deep-redesign-gates.md) | open |

## Active Data Fixture

`data/logs/current-model-run.md` and `data/logs/index.ndjson` are failure
evidence. The checked-in run proves these facts:

- active case `1` is at node `document` in phase `execution`;
- owner task is `Create a long novel. with structured settings.`;
- pre-owner maintenance repeats empty memory searches, no-op pruning, and
  close attempts instead of staying closed idle;
- the active run uses `stories/long-novel-with-structured-settings`, while the
  target path planner must choose a short semantic alias such as
  `stories/novel` for the same objective;
- active tracks are `document-structure`, `action-param-reliability`, and
  `observability-ledger`;
- evidence ledger has `plan` and `observation` only;
- `doc.audit` failed readiness with weak structure-only story pages;
- an attempted batch exceeded the file-count limit and was refused before
  mutation;
- recovery needed to change shape instead of repeating an oversized batch;
- reasoning-only provider responses were recorded as provider anomalies;
- document audit and artifact readiness audit remain pending.

## Ordering Notes

- Documentation moves first, then code. Prompt guidance alone never closes a row.
- The transition kernel is the target runtime authority. Graph policy is
  snapshot guidance, not fallback dispatch authority after runtime refusal.
- Runtime mission priority is hard compaction, owner recovery, schema repair,
  artifact repair, verification repair, owner execution, owner verification,
  owner completion, idle maintenance, then closed idle.
- Live model output uses one singular tag action. `fs.batch_write` uses only
  the line protocol inside `<files>`.
- `artifact.apply` creates structure once. If a root exists and weak content is
  the problem, the next action is audit, `artifact.next`, or a path-specific
  `fs.batch_write`.
- Direct graph evidence, scaffold topology, README-only content, and
  owner-term-only pages do not satisfy artifact readiness.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. No-op maintenance records cooldown instead of
  endpoint churn.

## Remaining Proof Gaps

Every blocker in the queue remains open until focused tests, corpus checks,
`quiet verify`, and Docker Compose verify run after the implementation changes.
