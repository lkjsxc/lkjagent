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
| 12 | Runtime smoke false close and noisy repair | [tasks/runtime-smoke-problem-sweep.md](tasks/runtime-smoke-problem-sweep.md) | done |
| 13 | Dense deterministic runtime authority network | [tasks/dense-runtime-state-network.md](tasks/dense-runtime-state-network.md) | done |
| 14 | Story manuscript generation gap | [tasks/story-manuscript-generation-gap.md](tasks/story-manuscript-generation-gap.md) | done |

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

## Historical Smoke Findings

Fresh pre-change smoke evidence is committed under
`tmp/runtime-smoke-ground-truth-20260629T051817Z/` and summarized in
[current-work/runtime-smoke-ground-truth.md](current-work/runtime-smoke-ground-truth.md).
Those logs showed Compact-title false close, owner-title root drift,
same-root missing-root audit loops, placeholder roots, and weak story-scale
completion.

The implementation sweep now has focused tests, benchmark fixture updates, and
final Docker smoke evidence under `tmp/runtime-smoke-final-iwanna-20260629T131603Z/`
and `tmp/runtime-smoke-final-compact-20260629T134111Z/`. The final smoke closes
both named long-novel routes without the false close, generic root, or noisy
repair loop. The remaining active work is structural hardening, not reopening
those historical defects.

Historical details live in
[current-work/runtime-smoke-problems.md](current-work/runtime-smoke-problems.md).
The dense runtime authority slice is implemented and proved by the focused,
workspace, quiet verify, and Docker gates named in its task file.

## Story Manuscript Generation

The manuscript gap is implemented through typed manuscript classification,
count-guard vetoes, chapter write contracts, manuscript readiness, completion
refusals, provider anomaly recovery, and benchmark fixtures. Evidence is
summarized in
[current-work/story-manuscript-generation-gap.md](current-work/story-manuscript-generation-gap.md).
