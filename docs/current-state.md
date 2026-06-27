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
compaction records, and benchmark fixtures.

The central transition kernel is still not complete. Runtime authority is not
yet the single durable source for every prompt, endpoint call, parse fault,
admission, dispatch, observation, recovery route, compaction interrupt,
maintenance boundary, and close path. Model text is intent or content. Runtime
data must be authority.

The active `data/logs/current-model-run.md` shows a long-novel artifact run at
`stories/long-novel-with-detailed-settings`. The scaffold was created, but the
run is stuck in weak-content repair after `doc.audit` found structure-only
pages. It also proves repeated invalid child-tag `fs.batch_write` payloads,
reasoning-only provider anomalies, and stale touched-path reporting that says
`none` even though artifact operations touched the workspace.

## Implemented Behavior

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Parser | `lkjagent-protocol` parses singular `<action>` turns, rejects top-level JSON and prose outside the envelope, records implicit-envelope outcomes, and emits structured parse faults. |
| Dispatcher registry | `lkjagent-tools` validates registered tools, required-any groups, personal record tools, and registry-rendered examples for covered action families. |
| Graph model | `lkjagent-graph` stores typed cases, evidence requirements, ranked tracks, transitions, and completion decisions. |
| SQLite store | Queue, state, event, memory, personal record, task summary, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist in `lkjagent-store`. |
| Endpoint loop | The daemon calls a local endpoint, records token usage when present, and preserves unknown usage as unknown. |
| Model log | Status, console, and `lkjagent model-log` expose a provider-neutral current run snapshot and per-turn request, authority, response, parse, admission, observation, timing, error, index, and export files when present. |
| Document scaffold seed | Deterministic scaffold paths, relation-first generic seeds, bounded slugs, compact `catalog.toml`, and creative writing profiles exist for covered artifact roots. |
| Document audit basics | Audit checks README topology, catalog coverage, path hygiene, line limits, workspace briefs, structure-only pages, owner-term pages, and old generated boilerplate leaves. Documentation roots require README `Purpose` sections and links; `stories/` artifact roots do not. Failed audits return bounded representative failure lists with omitted counts. |
| Placeholder and payload refusal | `fs.write`, `fs.batch_write`, content audit, and check-docs reject common scaffold phrases and oversized payloads before mutation. |
| Audit-owned evidence guard | Direct `graph.evidence` cannot satisfy `artifact-readiness` or `document-structure`. |
| Hard compaction mode | A runtime-owned `Compaction` active mode exists and does not render `memory.save` as a model action. |
| Stale-action preemption seed | Cached and pending actions are refused before dispatch when selected authority fields change, except compaction-only prompt rotation with unchanged queue, graph, artifact, fault, and evidence authority. |
| Baseline benchmarks | The corpus includes owner-reported recovery, artifact, memory, accounting, model-log, batch-schema, compaction, and repeated-recovery signatures. |

## Partially Implemented Behavior

| Area | Current truth |
| --- | --- |
| Runtime authority | Pure active-mode selection through the kernel adapter, normalized authority snapshots with queue-head and active-mode facts, event and decision rows, adapter-valid kernel decision fingerprints, prompt-card decision ids, plan-first, structure-apply, and audit-gap prompt examples, owner-task `artifact.apply` admission, artifact-readiness graph-tool intersection, graph-state authority overlays, pending-action admission rows, immutable admission-view refusal for pending actions, owner-queue stale-action refusal, full-fact kernel stale refusal, kernel `agent.done` admission/refusal, and daemon kernel completion-event shadowing exist. A standalone `kernel` module defines pure snapshot, event, decision, admission, effect, render, fault, adapter, and reducer records with invariant tests. The daemon still has parallel authority paths that can disagree. |
| State-transition contracts | Snapshot, event, decision, admission, transition, artifact ledger, compaction history, fan-out, and index-network contracts are documented. Authority ledger events use canonical kernel event-kind strings. Full unified runtime wiring remains open. |
| Recovery controller | Fault notices, recovery graph routes, escape-tool visibility, repeat refusal, route metadata, pure recovery plans, SQLite retry counts, repeated batch-schema shape change to `artifact.next`, payload-overflow routing to `artifact.next` for known artifacts, provider-anomaly retry-budget pause, and parse-fault retention until successful observation exist. Every-route shape-change proof remains open. |
| Schema repair | Safe alias normalization and registry examples exist for covered cases. `fs.batch_write` normalizes selected safe payloads and refuses unsafe shapes before mutation. Runtime route changes after repeated schema faults remain open beyond covered classes. |
| Artifact lifecycle | Scaffold, audit, six-path `artifact.next` batches, empty-root identity batches, story semantic readiness checks, bounded write examples, root-scoped cursors, root/path address refusals, normalized artifact ledger and cursor APIs, invalid-root markers, and daemon `agent.done` refusal for unresolved ledger weak paths exist. Adoption repair and close-path proof remain incomplete. |
| Completion gates | A pure completion reducer returns completion kind, failed gates, missing evidence, existing evidence, current artifact, next action, valid example, blocked-handoff allowance, and status text. Every close path is not yet proven to call the same artifact-aware gate. |
| Compaction resumability | Compaction records graph, recovery, artifact, batch cursor, last-observation, and next-action fields in notices and writes pre/post graph compaction snapshot rows. Prompt-frame resume proof remains open. |
| Maintenance | Idle maintenance, owner queue preemption at turn boundaries, no-op cooldown, exact duplicate deletion, high-overlap merge, and low-signal rewrite pruning exist. Every dispatch and close path still needs unified kernel authority proof. |
| Provider exchange logging | Store schema, APIs, atomic file writer, per-turn export files, kernel authority fields in `authority.json`, CLI list/show, raw-case inspection, sanitized replay export, raw turn-file copying, self-consistent manifests, explicit missing-file records, and provider-anomaly store plus manifest status exist. |
| Benchmarks | Uploaded-run text signatures cover provider artifacts and repeated recovery. Historical Chronos smoke evidence exists. The active checked-in long-novel log is now the primary replay target for weak-content repair, child-tag batch-write faults, provider anomalies, and touched-path mismatch. |

## Active Data Log Evidence

`data/logs/current-model-run.md`, `data/logs/index.ndjson`, and the latest turn
directories prove these facts:

- snapshot: `daemon_state=working`, `active_case=1`, `active_node=document`,
  and `active_phase=execution`;
- context: `13.46K/24.58K`, about 55 percent used;
- owner task: create a long novel with detailed settings;
- maintenance prelude: many case-none idle maintenance cycles repeat no-result
  memory searches, no-op pruning, and `agent.done` before owner work arrives;
- artifact root: `stories/long-novel-with-detailed-settings`;
- state tracks: `document-structure`, `action-param-reliability`, and
  `observability-ledger`;
- touched paths: the synthesized top section says `none`, but transcript
  evidence shows `artifact.apply` created the scaffold and `fs.list` later
  observed the root, so touched-path synthesis is stale;
- evidence ledger: `plan` and `observation` only; audit-owned
  `document-structure` and `artifact-readiness` remain missing;
- artifact lifecycle: `artifact.apply` created a 39-file scaffold with
  `profile=NarrativeManuscript` and `kind=novel`;
- weak-content repair: `doc.audit` failed content readiness with 28
  structure-only pages and requested `fs.batch_write` or `artifact.next`;
- schema fault: `fs.batch_write` was attempted twice with `<file>` child tags
  inside `<files>` and refused with `invalid parameter: each block must start
  with path:`;
- recovery defect: after the first schema fault, `graph.recover` allowed the
  same invalid batch-write shape to repeat instead of forcing `artifact.next` or
  a canonical line-protocol batch example;
- provider anomalies: turns 59 and 62 record `reasoning_only_response` and keep
  parser retry counts unchanged;
- verification: document audit and artifact readiness audit remain pending.

## Historical Live Smoke Evidence

Chronos evidence remains historical, not active checked-in data. The recorded
Chronos smoke at `/tmp/lkjagent-smoke-data-1782483148` created the story-bible
directory shape and plan evidence, then timed out during weak-content repair
with `document-structure` and `artifact-readiness` still missing. The older
empty-content-with-usage turn at
`data/logs/model/epoch-1782344195/case-1/turn-000019` remains a provider anomaly
replay fixture, not the latest exchange.

## Active Target

The dependency queue is [execution/current-blockers.md](execution/current-blockers.md).
The active implementation target remains the persisted kernel:

```text
DurableReadModel -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> RuntimeEffectCommand
RuntimeEffectCommand -> EffectObservation
EffectObservation -> RuntimeEvent
```

The decision must be persisted before prompt rendering, endpoint calls,
dispatch, recovery, compaction, maintenance, or close attempts. The next narrow
proof is that the long-novel weak-content repair, provider anomaly, touched-path
summary, and dispatch admission all expose and use the same persisted decision,
prompt-frame, authority, and staleness identifiers.

## Latest Recorded Verification Evidence

This snapshot-adapter slice has focused evidence:

- `cargo test -p lkjagent-runtime --test kernel_snapshot_adapter`: `KERNEL_SNAPSHOT_ADAPTER_EXIT=0`.
- `cargo test -p lkjagent-runtime --test authority_ledger_wiring`: `AUTHORITY_LEDGER_WIRING_EXIT=0`.
- `cargo fmt --check`: `FMT_CHECK_EXIT=0`.

Before this reconciliation, the latest recorded full verification covered
active-log ledger, model-log authority export, export-manifest missing-file
records, authority staleness facts, and recovery counter handling:

- `cargo fmt --check`: `FMT_CHECK_EXIT=0`.
- `cargo test -p lkjagent-runtime --test authority_ledger_wiring`:
  `AUTHORITY_LEDGER_WIRING_EXIT=0`; includes durable queue-head snapshot
  proof.
- `cargo test -p lkjagent-runtime --test turn_authority`:
  `TURN_AUTHORITY_EXIT=0`; includes shared-fact mission agreement with the
  kernel reducer.
- `cargo test -p lkjagent-runtime --test authority_examples`:
  `AUTHORITY_EXAMPLES_EXIT=0`; includes plan-first, structure-apply, and audit-gap examples.
- `cargo test -p lkjagent-tools --test doc_content_audit`:
  `DOC_CONTENT_EXIT=0`; includes bounded audit failure report evidence.
- `cargo test -p lkjagent-tools --test doc_tools`: `DOC_TOOLS_EXIT=0`.
- `cargo test -p lkjagent-tools --test graph_control_dispatch`:
  `GRAPH_CONTROL_EXIT=0`; includes graph-state authority overlay evidence.
- `cargo test -p lkjagent-runtime --test provider_exchange_log`:
  `PROVIDER_EXCHANGE_LOG_EXIT=0`; includes explicit `missing_files` export
  records and `provider_anomaly` store plus manifest status.
- `cargo test -p lkjagent-runtime --test provider_anomaly`:
  `PROVIDER_ANOMALY_EXIT=0`; includes retry-budget pause without parse-fault
  increments.
- `cargo test -p lkjagent-store --test provider_exchange`:
  `STORE_PROVIDER_EXCHANGE_EXIT=0`.
- `cargo test -p lkjagent-runtime --test kernel_admission`:
  `KERNEL_ADMISSION_EXIT=0`; includes fault, evidence, maintenance, and
  prompt-frame stale-action refusal.
- `cargo test -p lkjagent-runtime --test kernel_snapshot_adapter`:
  `KERNEL_SNAPSHOT_ADAPTER_EXIT=0`; includes active-mode staleness
  fingerprinting.
- `cargo test -p lkjagent-runtime --test step`: `STEP_EXIT=0`; parse faults
  clear after successful observation, not merely after parsing a new action.
- `cargo test -p lkjagent-runtime --test recovery_loop`:
  `RECOVERY_LOOP_EXIT=0`; repeated transient errors recover across tasks.
- `cargo test -p lkjagent-runtime --test current_model_run_fixture`:
  `CURRENT_MODEL_RUN_FIXTURE_EXIT=0`; fixture reads checked-in log bytes
  deterministically.
- `cargo test -p lkjagent-cli --test model_log`: `CLI_MODEL_LOG_EXIT=0`.
- `cargo test -p lkjagent-cli --test model_log_archive`:
  `CLI_MODEL_LOG_ARCHIVE_EXIT=0`.
- `cargo run -p lkjagent-xtask -- check-docs`: `CHECK_DOCS_EXIT=0`,
  `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `CHECK_LINES_EXIT=0`,
  `ok check-lines`.
- `cargo run -p lkjagent-xtask -- check-style`: `CHECK_STYLE_EXIT=0`,
  `ok check-style`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`,
  `ok verify`.
- `docker compose run --rm verify`: `DOCKER_VERIFY_EXIT=0`, `ok verify`.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
