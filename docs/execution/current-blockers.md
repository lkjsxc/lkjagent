# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Truth sweep and fixture root reconciliation | [tasks/deep-redesign-truth-sweep.md](tasks/deep-redesign-truth-sweep.md) | done |
| 2 | Compact context and no object-literal model context | [tasks/deep-redesign-compact-context.md](tasks/deep-redesign-compact-context.md) | done |
| 3 | Output budget contract and endpoint config | [tasks/deep-redesign-output-budget.md](tasks/deep-redesign-output-budget.md) | done |
| 4 | Short artifact path aliases and planner | [tasks/deep-redesign-short-paths.md](tasks/deep-redesign-short-paths.md) | done |
| 5 | Registry-derived exact action examples | [tasks/deep-redesign-exact-examples.md](tasks/deep-redesign-exact-examples.md) | done |
| 6 | Narrow runtime authority prompt cards | [tasks/deep-redesign-runtime-authority.md](tasks/deep-redesign-runtime-authority.md) | done |
| 7 | Artifact cursor micro-batches | [tasks/deep-redesign-artifact-batches.md](tasks/deep-redesign-artifact-batches.md) | done |
| 8 | Completion and maintenance reducers | [tasks/deep-redesign-completion-maintenance.md](tasks/deep-redesign-completion-maintenance.md) | done |
| 9 | Provider anomaly blocked handoff | [tasks/deep-redesign-provider-handoff.md](tasks/deep-redesign-provider-handoff.md) | done |
| 10 | Benchmark corpus and final gates | [tasks/deep-redesign-gates.md](tasks/deep-redesign-gates.md) | done |

## Active Data Fixture

`data/logs/current-model-run.md` and `data/logs/index.ndjson` are failure
evidence. `index.ndjson` paths are normalized from `/data/logs/...` to the
repository `data/logs/...` tree before integrity checks. The checked-in run
proves these facts:

- active case `1` is at node `document` in phase `execution`;
- owner task is `Create a long novel. with detailed structured settings.`;
- observed root is `stories/novel`;
- `graph.state` repeatedly reported `no active graph case` while authority and
  the log snapshot named active case `1`;
- authority refused a local `fs.mkdir` path that was not admitted;
- `fs.batch_write` wrote a small novel tree;
- `doc.audit` first failed and later passed structure;
- `artifact.audit` and `graph.state` repeated instead of changing shape;
- direct `graph.evidence` for audit-owned evidence was refused;
- reasoning-only provider responses were recorded as provider anomalies;
- final verification remained pending;
- no fresh successful live smoke run is checked in.

## Ordering Notes

- Documentation moves first, then code. Prompt guidance alone never closes a row.
- The transition kernel is the runtime authority. Graph policy is snapshot
  guidance, not fallback dispatch authority after runtime refusal.
- Runtime mission priority is hard compaction, owner recovery, schema repair,
  artifact repair, verification repair, owner execution, owner verification,
  owner completion, idle maintenance, then closed idle.
- Live model output uses one singular tag action. `fs.batch_write` uses only the
  line protocol inside `<files>`.
- Prompt-visible scaffold writers are removed from live registry, prompts,
  admission, recovery text, docs, and tests.
- `artifact.next` is non-mutating and returns write contracts rather than body
  content; the next persisted decision chooses whether to render a content write
  surface.
- Direct graph evidence, scaffold topology, README-only content, and
  owner-term-only pages do not satisfy artifact readiness.
- Audit-owned evidence comes from `doc.audit` and `artifact.audit` observations.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. No-op maintenance records cooldown instead of endpoint
  churn.
- Context compaction can run at state boundaries and must preserve cursor,
  mission, recovery route, blockers, and next action surface.

## Completion Evidence

The redesign is closed by focused crate tests, workspace tests, corpus checks,
`quiet verify`, and `docker compose run --rm verify` after the implementation
changes. The checked-in data log remains failure evidence until a fresh model
smoke run replaces it.
