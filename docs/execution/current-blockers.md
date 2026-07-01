# Current Blockers

## Purpose

This file is the dependency-ordered implementation queue. Rows move to done
only when the linked task contract names focused evidence and the actual gates
that ran.

## Active Queue

| # | Blocker | Task | Status |
| --- | --- | --- | --- |
| 1 | Live manuscript proof boundary | [tasks/live-manuscript-proof.md](tasks/live-manuscript-proof.md) | open: latest Aurora Ledger live proof exhausted the loop in recovery with 26 scene files, zero chapter files, zero final manuscript words, and no artifact readiness rows |

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
- story manuscript generation gap;
- large-artifact durable completion.

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
readiness refusals, and provider anomaly recovery. The large-artifact durable
completion row is closed for store-backed content atoms, write contracts,
deterministic assembly, readiness projection, replay, quiet verify, and Docker
verify evidence. Neither row proves that the daemon can finish a complete
10,000-word manuscript against a live endpoint. The active queue owns that
operator-run proof boundary.

Historical details live in
[current-work/story-manuscript-generation-gap.md](current-work/story-manuscript-generation-gap.md)
and [current-work/runtime-smoke-problems.md](current-work/runtime-smoke-problems.md).
