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

## Interpretation

The fresh ground truth confirms that the open blocker is current. Compact title
work can still close without artifact audit, named novel roots still degrade to
generic roots, missing-root repair is still noisy, and the live route has not
proven scale-appropriate long-novel completion.
