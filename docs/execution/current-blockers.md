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
| 11 | Obligation network root repair | [tasks/obligation-network-redesign.md](tasks/obligation-network-redesign.md) | done |
| 12 | Runtime smoke false close and noisy repair | [tasks/runtime-smoke-problem-sweep.md](tasks/runtime-smoke-problem-sweep.md) | open |

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

## Open Smoke Findings

Fresh pre-change smoke evidence is committed under
`tmp/runtime-smoke-ground-truth-20260629T051817Z/` and summarized in
[current-work/runtime-smoke-ground-truth.md](current-work/runtime-smoke-ground-truth.md).
Live smoke runs after root repair found remaining problems:

- titles containing `Compact` can be classified as compaction work and close
  without artifact audit;
- named novel roots can degrade to `stories/novel-named`;
- missing-root repair is escapable but still spends noisy turns on repeated
  audit, refused mkdir, and placeholder-root examples;
- long-novel completion can pass with a small story-bible seed.

The implementation sweep now has focused tests, benchmark fixture updates, and
fresh post-change Docker smoke evidence under
`tmp/runtime-smoke-final-20260629T071918Z/`. The smoke improves routing and root
repair but still shows noisy recovery after story scale-readiness refusal.

Details live in [current-work/runtime-smoke-problems.md](current-work/runtime-smoke-problems.md).
