# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Active Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Documentation truth sweep | [tasks/structural-truth-sweep.md](tasks/structural-truth-sweep.md) | done: docs reconciled; `check-docs` and `check-lines` passed |
| 2 | CLI contract rewrite | [tasks/cli-contract-redesign.md](tasks/cli-contract-redesign.md) | done: target CLI docs written; `check-docs` and `check-lines` passed |
| 3 | Token ledger contract and aggregate store API | [tasks/token-aggregate-ledger.md](tasks/token-aggregate-ledger.md) | done: aggregate APIs, status-console rendering, focused tests, quiet verify, and Docker verify passed |
| 4 | CLI parser and output core rewrite | [tasks/cli-core-redesign.md](tasks/cli-core-redesign.md) | done: metadata help, group parsing, watch, task and queue inspection, focused tests, quiet verify, and Docker verify passed |
| 5 | Status, log, and console render rewrite | [tasks/observability-render-redesign.md](tasks/observability-render-redesign.md) | open |
| 6 | Runtime resolver totality cleanup | [tasks/resolver-table-totality.md](tasks/resolver-table-totality.md) | open |
| 7 | Content atom graph and artifact profile generalization | [tasks/content-atom-graph.md](tasks/content-atom-graph.md) | open |
| 8 | Manuscript staged prose and deterministic assembly | [tasks/manuscript-compose.md](tasks/manuscript-compose.md) | open |
| 9 | Smoke harness and benchmark expansion | [tasks/smoke-harness.md](tasks/smoke-harness.md) | open |
| 10 | Final gates and handoff | [tasks/final-redesign-gates.md](tasks/final-redesign-gates.md) | open |

## Historical Closed Queue

The prior blocker sequence is closed. Its task files remain under
[tasks/](tasks/README.md) as evidence and regression context:

- deep redesign truth sweep;
- compact context and line-protocol batches;
- compact output budget and endpoint config;
- short artifact path aliases and planner;
- registry-derived exact action examples;
- narrow runtime authority prompt cards;
- artifact cursor micro-batches;
- completion and maintenance reducers;
- provider anomaly blocked handoff;
- benchmark corpus and gates;
- obligation network root repair;
- runtime smoke false close and noisy repair;
- dense deterministic runtime authority network;
- story manuscript generation gap.

## Active Data Fixture

`data/logs/current-model-run.md` and `data/logs/index.ndjson` are historical
failure evidence. `index.ndjson` paths are normalized from `/data/logs/...` to
the repository `data/logs/...` tree before integrity checks. The checked-in run
proves these facts:

- active case `1` is at node `evidence-plan` in phase `recovery`;
- owner task is `Create a long novel. named "iwanna". with detailed and
  structured settings.`;
- observed root is `stories/novel-named`;
- `doc.audit` repeatedly reported `missing_root` for that root;
- authority refused a local `fs.mkdir` path that was not admitted;
- duplicate `settings.md` `fs.batch_write` attempts did not create root
  identity;
- repeat recovery and `graph.recover` changed shape but routed back to
  same-root `doc.audit`;
- `graph.state` showed active case `1` while recovery remained open;
- reasoning-only provider responses were recorded as provider anomalies;
- document audit, artifact readiness audit, and final verification remained
  pending.

The checked-in generated log fixture remains historical failure evidence.

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
- Missing-root observations create root identity obligations; they do not route
  to another same-root `doc.audit` before write progress.
- Direct graph evidence, scaffold topology, README-only content, and
  owner-term-only pages do not satisfy artifact readiness.
- Audit-owned evidence comes from `doc.audit` and `artifact.audit` observations.
- Maintenance can start only from closed idle with an empty owner queue and no
  recoverable owner task. No-op maintenance records cooldown instead of endpoint
  churn.
- Context compaction can run at state boundaries and must preserve cursor,
  mission, recovery route, blockers, and next action surface.

## Manuscript Boundary

The story manuscript generation gap row in the historical queue is closed for
root identity, exact manuscript classification, chapter-priority contracts,
readiness refusals, and provider anomaly recovery. It is not proof that the
daemon can finish a complete 10,000-word manuscript. The active queue now
continues that work through content atoms, deterministic assembly, exact
remaining-path recovery, real manuscript word counts, and a reproducible smoke
harness.

Historical details live in
[current-work/story-manuscript-generation-gap.md](current-work/story-manuscript-generation-gap.md)
and [current-work/runtime-smoke-problems.md](current-work/runtime-smoke-problems.md).
