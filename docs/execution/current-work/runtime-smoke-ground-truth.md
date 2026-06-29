# Runtime Smoke Ground Truth

## Purpose

Record the fresh repository baseline and live Docker smoke evidence captured
before changing the runtime smoke problem sweep implementation.

## Case State

- Objective: fix `Runtime smoke false close and noisy repair` while preserving
  honest evidence for the current behavior first.
- Constraints: docs and code must move together, files stay under 200 lines,
  Docker Compose is the live-smoke surface, and no fake evidence may be used.
- Assumptions: endpoint variables are present in this shell, but smoke output is
  still treated as observed behavior rather than proof of desired behavior.
- Risks: long model runs may time out, ignored `tmp/` evidence must be
  force-added, and checked-in failure logs can be confused with fresh evidence.
- Evidence requirements: command outputs, smoke model logs, index files, final
  status output, and a short analysis must all name exact paths.
- Candidate files: graph classification, artifact root extraction, runtime
  next-action and resolver code, runtime tests, benchmark corpus, and these
  execution docs.
- Next action: commit this ground truth before implementation changes.

## Baseline Commands

Exact command outputs are preserved under
`tmp/runtime-smoke-ground-truth-20260629T051817Z/commands/`.

| Command | Result | Exact output |
| --- | --- | --- |
| `git status --short` | exit 0, empty output | `01-git-status-short.log` |
| `git rev-parse HEAD` | exit 0, `dc390806736183c503fa16493ace17aa573eca7f` | `02-git-rev-parse-head.log` |
| `docker compose build verify` | exit 0 | `03-docker-compose-build-verify.log` |
| `docker compose run --rm verify` | exit 1 | `04-docker-compose-run-verify.log` |
| `cargo run -p lkjagent-xtask -- check-docs` | exit 0, `ok check-docs` | `05-xtask-check-docs.log` |
| `cargo run -p lkjagent-xtask -- check-lines` | exit 0, `ok check-lines` | `06-xtask-check-lines.log` |
| `cargo run -p lkjagent-xtask -- check-style` | exit 0, `ok check-style` | `07-xtask-check-style.log` |
| `cargo run -p lkjagent-xtask -- benchmark check-corpus` | exit 0, `ok benchmark-corpus` | `08-xtask-benchmark-check-corpus.log` |

The Docker verify failure cause is exact:

```text
check-docs failed
exit status: 1
docs/execution/tasks/deep-redesign-truth-sweep.md: doc link: broken link '../../../data/logs/current-model-run.md' resolves to 'data/logs/current-model-run.md'
```

## Live Smoke: Compact Compass

- Owner task: `Create a long novel named "Compact Compass" with detailed and
  structured settings.`
- Fresh data directory:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/compact-compass-data/`.
- Complete model log:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/compact-compass-data/logs/current-model-run.md`.
- Complete index:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/compact-compass-data/logs/index.ndjson`.
- Status and command evidence:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/compact-compass-evidence/`.

Observed facts:

```text
final_reason=open_task_none
current_model_run_lines=87
index_lines=7
fs.batch_write=0
doc.audit=0
artifact.next=0
artifact.audit=0
graph.evidence=1
agent.done=1
fs.write=1
final open_task=none
active_artifact_root=none
```

The action route was `memory.find -> graph.plan -> fs.write ->
graph.evidence -> agent.done`. This is a false close for a long-novel owner
request because no artifact identity, root audit, artifact audit, or contracted
batch write occurred.

## Live Smoke: iwanna

- Owner task: `Create a long novel named "iwanna" with detailed and structured
  settings.`
- Fresh data directory:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/iwanna-data/`.
- Complete model log:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/iwanna-data/logs/current-model-run.md`.
- Complete index:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/iwanna-data/logs/index.ndjson`.
- Status and command evidence:
  `tmp/runtime-smoke-ground-truth-20260629T051817Z/smokes/iwanna-evidence/`.

Observed facts:

```text
final_reason=manual_stop_after_poll_timeout
current_model_run_lines=431
index_lines=144
open_task=Create a long novel named "iwanna" with detailed and structured settings.
active_mode=recovery
active_artifact_root=stories/novel-named
missing_root=3
fs.batch_write=91
doc.audit=119
artifact.next=88
artifact.audit=56
stories/iwanna=0
stories/example-story=1
repeat action refused=43
provider anomaly=5
```

The run preserved the historical defects: the root lost the owner title, the
runtime stayed in a noisy recovery loop, and a generic example root appeared
while a current artifact root existed.

## Intermediate Post-Change Smoke Evidence

Intermediate post-change smoke evidence is preserved under
`tmp/runtime-smoke-final-20260629T071918Z/`. It is chronology for the completed
sweep, not the current active state.

Compact Compass:

```text
final_reason=route_evidence_observed
current_model_run_lines=105
index_lines=11
stories/compact-compass=24
stories/novel-named=0
stories/example-story=0
fs.batch_write=2
files_written=5=1
doc.audit=2
artifact.audit=6
graph.evidence=0
agent.done=0
fs.mkdir=0
fs.write=0
```

The action route was `memory.find -> graph.plan -> doc.audit ->
fs.batch_write -> doc.audit -> artifact.audit -> artifact.next ->
artifact.audit`. The task no longer false-closed as compaction work.

`iwanna`:

```text
final_reason=route_evidence_observed
current_model_run_lines=123
index_lines=17
stories/iwanna=34
stories/novel-named=0
stories/example-story=0
fs.batch_write=4
files_written=5=1
doc.audit=2
artifact.audit=15
graph.evidence=0
agent.done=0
fs.mkdir=0
fs.write=0
missing_root=1
story_scale_missing=3
story_semantic_missing=3
```

The root preserved `iwanna`, missing-root repair reached a contracted
`fs.batch_write` before another same-root audit, and completion did not close
the small seed artifact. The run still shows repeated artifact-audit recovery
after scale-readiness refusal, so follow-up work should improve post-readiness
repair planning.

## Final Closure Smoke Evidence

Final clean-data smoke evidence is preserved under
`tmp/runtime-smoke-final-iwanna-20260629T131603Z/` and
`tmp/runtime-smoke-final-compact-20260629T134111Z/`.

`iwanna` reached `open_task=none`, wrote 23 unique story paths, kept
`stories/iwanna`, avoided generic roots, recorded
`readiness=story-semantic-content`, emitted `agent.done`, and had no authority
refusals or repeat-action refusals. Compact Compass wrote 22 unique story paths,
kept `stories/compact-compass`, avoided generic roots, recorded
`readiness=story-semantic-content`, emitted `agent.done`, and reached
`open_task=none` on continuation before maintenance opened.

## Interpretation

The pre-change ground truth confirms that the open blocker was current. The
final smoke proves compact-title routing, owner-title roots, root identity
writes, generic-root suppression, story-scale readiness, and owner-task closure
without the observed false close or noisy recovery loop.
