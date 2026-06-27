# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
partial, and what is open. A behavior is implemented only when code, focused
tests, quiet gates, and required Docker gates prove it.

## Summary

lkjagent has a working Rust workspace with local gates, Docker Compose gates,
a strict tag action parser, dispatcher registry, typed graph model, context
budgeting, SQLite persistence, endpoint client, daemon loop, owner queue intake,
status and log commands, memory commands, model-run logging, artifact ledgers,
compaction records, provider-exchange logging, runtime authority ledgers,
personal-record projections, and benchmark fixtures.

The runtime-authority kernel cutover is implemented and verified. Product paths
use one persisted transition kernel for prompt rendering, provider calls, parse
and schema faults, admission, dispatch, observations, recovery routes,
compaction, maintenance, model-log handoff, and completion attempts. Model
output and graph policy are intent inputs; the runtime decision is the
executable authority.

The checked-in `data/logs/current-model-run.md` remains the active fixture for
long-novel failure evidence. It is not a fresh proof after the redesign. The
fixture proves the failures that the cutover must cover: weak-content repair,
child-tag `fs.batch_write` schema faults, reasoning-only provider anomalies,
no-op maintenance churn, stale touched-path reporting, and false completion.

## Implemented Behavior

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Parser | `lkjagent-protocol` parses singular `<action>` turns, rejects invalid envelopes, and emits structured faults. |
| Dispatcher registry | `lkjagent-tools` validates tools, required-any groups, personal tools, and registry-rendered examples. |
| Graph model | `lkjagent-graph` stores typed cases, evidence requirements, ranked tracks, transitions, and completion decisions. |
| Store | Queue, state, event, memory, personal record, summary, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist. |
| Runtime kernel core | Snapshot, event, decision, admission, effect, render, fault, provider, adapter, reducer, and driver records exist. |
| Authority persistence | Store rows carry snapshot, event, decision, companion details, prompt frames, observations, fingerprints, and staleness facts. |
| Endpoint loop | Provider calls record model-log files, token usage when present, anomalies, retry state, and bounded recovery facts. |
| CLI entrypoint | `lkjagent --help` and `lkjagent help` print usage before config loading, and `--data` is accepted before or after the command. |
| Model log | Status, console, `model-log`, exchange exports, raw-case inspection, replay export, and touched paths have durable readers. |
| Artifact lifecycle | Artifact plan, apply, audit, next, cursors, weak paths, invalid roots, story readiness, and completion refusals are ledger-backed. |
| Recovery | Fault classes, route metadata, retry counters, changed-shape repeat routing, blocked handoff, and canonical examples are covered. |
| Graph evidence | Direct graph evidence refuses audit-owned requirements and immediate claims after refused or failed tool output. |
| Compaction | Hard compaction writes resumable pre and post snapshots and resumes through prompt-frame authority records. |
| Maintenance | Maintenance gates, owner preemption checks, no-op cooldown facts, and closed-idle rules have focused coverage. |
| Completion | `agent.done` has artifact-aware completion refusal coverage with evidence and readiness gates. |
| Personal records | Diary, schedule, and TODO records are store-backed; CLI inspection and bounded Markdown projections are implemented. |
| Benchmarks | Owner-reported recovery, artifact, memory, accounting, model-log, batch-schema, compaction, repeated-recovery, and long-novel signatures are in the corpus. |

## Active Data Log Fixture

`data/logs/current-model-run.md`, `data/logs/index.ndjson`, and latest turn
directories prove these fixture facts:

- active case `1` is at node `document` in phase `execution`;
- owner task is to create a long novel with detailed settings;
- case-none maintenance repeats empty memory searches, no-op pruning, and
  `agent.done` before owner work arrives;
- artifact root is `stories/long-novel-with-detailed-settings`;
- active tracks are `document-structure`, `action-param-reliability`, and
  `observability-ledger`;
- old touched-path summary says `none`, despite scaffold creation and later
  workspace observation of the root;
- evidence ledger contains `plan` and `observation`; audit-owned
  `document-structure` and `artifact-readiness` are missing in the fixture;
- `artifact.apply` created a 39-file `NarrativeManuscript` scaffold;
- `doc.audit` failed content readiness with 28 structure-only pages;
- two `fs.batch_write` attempts used invalid child `<file>` tags and were
  refused before mutation;
- after the first schema fault, the old run repeated the same invalid shape;
- turns 59 and 62 record `provider_anomaly.reasoning_only_response`;
- document audit and artifact readiness audit remain pending in the fixture.

## Runtime Authority Target Flow

```text
DurableReadModel -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> RuntimeEffectCommand
RuntimeEffectCommand -> EffectObservation
EffectObservation -> RuntimeEvent
```

The decision is persisted before prompt rendering, endpoint calls, dispatch,
recovery, compaction, maintenance, or close attempts. Prompt frames, provider
exchange rows, pending actions, admissions, observations, model-log exports,
and status expose the same authority ids and staleness fingerprints.

## Historical Live Smoke Evidence

Chronos evidence remains historical, not active checked-in data. The recorded
Chronos smoke created story-bible structure and plan evidence, then timed out
during weak-content repair with `document-structure` and `artifact-readiness`
still missing. The older empty-content-with-usage turn remains a provider
anomaly replay fixture, not the latest exchange.

## Latest Recorded Verification Evidence

This kernel cutover has verification evidence:

- `cargo test -p lkjagent-runtime`: `RUNTIME_ALL_EXIT=0`.
- `cargo test -p lkjagent-store --test runtime_authority --test runtime_kernel_ledger`:
  `STORE_AUTH_TESTS_EXIT=0`.
- `cargo test -p lkjagent-tools --test semantic_examples --test artifact_next_long_novel --test artifact_next_quality`:
  `TOOLS_ARTIFACT_TESTS_EXIT=0`.
- `cargo fmt --check`: `FMT_CHECK_EXIT=0`.
- `cargo run -p lkjagent-xtask -- check-docs`: `CHECK_DOCS_EXIT=0`,
  `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `CHECK_LINES_EXIT=0`,
  `ok check-lines`.
- `cargo run -p lkjagent-xtask -- check-style`: `CHECK_STYLE_EXIT=0`,
  `ok check-style`.
- `cargo run -p lkjagent-xtask -- benchmark check-corpus`:
  `BENCHMARK_CHECK_CORPUS_EXIT=0`, `ok benchmark-corpus`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`,
  `ok verify`.
- `docker compose run --rm verify`: `DOCKER_VERIFY_EXIT=0`, `ok verify`.

## Active Target

The dependency queue is [execution/current-blockers.md](execution/current-blockers.md).
No blocker remains open after the verified kernel cutover.

## Remaining Proof Gaps

No current blocker proof gaps remain after focused tests, `quiet verify`, and
Docker Compose verify.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
