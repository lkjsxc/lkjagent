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

The active `data/logs/current-model-run.md` shows a Chronos Fracture story-bible
run stuck in recovery. It does not prove file mutation. It proves repeated
missing action envelopes, empty provider content with nonzero completion tokens,
no touched paths, no useful evidence, and a replay manifest that lists missing
turn artifacts.

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
| Document audit basics | Audit checks README topology, links, catalog coverage, path hygiene, line limits, workspace briefs, structure-only pages, owner-term pages, and old generated boilerplate leaves. |
| Placeholder and payload refusal | `fs.write`, `fs.batch_write`, content audit, and check-docs reject common scaffold phrases and oversized payloads before mutation. |
| Audit-owned evidence guard | Direct `graph.evidence` cannot satisfy `artifact-readiness` or `document-structure`. |
| Hard compaction mode | A runtime-owned `Compaction` active mode exists and does not render `memory.save` as a model action. |
| Stale-action preemption seed | Cached actions are refused before dispatch when selected authority fields change. |
| Baseline benchmarks | The corpus includes owner-reported recovery, artifact, memory, accounting, model-log, batch-schema, compaction, and repeated-recovery signatures. |

## Partially Implemented Behavior

| Area | Current truth |
| --- | --- |
| Runtime authority | Pure active-mode selection, normalized authority snapshots, event and decision rows, prompt-card decision ids, pending-action admission rows, immutable admission-view refusal for pending actions, owner-queue stale-action refusal, and `agent.done` refusal exist. A standalone `kernel` module defines pure snapshot, event, decision, admission, effect, render, fault, adapter, and reducer records with invariant tests. The daemon still has parallel authority paths that can disagree. |
| State-transition contracts | Snapshot, event, decision, admission, transition, artifact ledger, compaction history, fan-out, and index-network contracts are documented. Full unified runtime wiring remains open. |
| Recovery controller | Fault notices, recovery graph routes, escape-tool visibility, repeat refusal, route metadata, pure recovery plans, SQLite retry counts, repeated batch-schema shape change to `artifact.next`, and payload-overflow routing to `artifact.next` for known artifacts exist. Provider empty-content anomaly handling and every-route shape-change proof remain open. |
| Schema repair | Safe alias normalization and registry examples exist for covered cases. `fs.batch_write` normalizes selected safe payloads and refuses unsafe shapes before mutation. Runtime route changes after repeated schema faults remain open beyond covered classes. |
| Artifact lifecycle | Scaffold, audit, fact-only `artifact.next`, empty-root identity batches, story semantic readiness checks, bounded write examples, root-scoped cursors, root/path address refusals, normalized artifact ledger and cursor APIs, invalid-root markers, and daemon `agent.done` refusal for unresolved ledger weak paths exist. Adoption repair and close-path proof remain incomplete. |
| Completion gates | A pure completion reducer returns completion kind, failed gates, missing evidence, existing evidence, current artifact, next action, valid example, blocked-handoff allowance, and status text. Every close path is not yet proven to call the same artifact-aware gate. |
| Compaction resumability | Compaction records graph, recovery, artifact, batch cursor, last-observation, and next-action fields in notices and writes pre/post graph compaction snapshot rows. Prompt-frame resume proof remains open. |
| Maintenance | Idle maintenance, owner queue preemption at turn boundaries, no-op cooldown, exact duplicate deletion, high-overlap merge, and low-signal rewrite pruning exist. Every dispatch and close path still needs unified kernel authority proof. |
| Provider exchange logging | Store schema, APIs, atomic file writer, per-turn export files, CLI list/show, raw-case inspection, sanitized replay export, raw turn-file copying, and focused tests exist. Manifest integrity proof for missing per-turn files remains open. |
| Benchmarks | Uploaded-run text signatures cover provider artifacts and repeated recovery. Live endpoint smoke remains open. |

## Active Failure Evidence

`data/logs/current-model-run.md` and the latest turn directory prove the next
reliability gap:

- snapshot: `daemon_state=working`, `active_case=1`,
  `active_node=recover-by-smaller-scope`, and `active_phase=recovery`;
- context: `19.36K/24.58K`, about 79 percent used;
- owner task: create a structured story bible at `stories/chronos-fracture`;
- touched paths: `none`;
- evidence ledger: `none`;
- fault ledger: repeated `parse fault: missing action envelope` after earlier
  `bad envelope prose before action envelope` faults;
- latest request: the system prompt still allowed `<think>` tags and preserved
  prior assistant turns shaped as `<think>...</think><action>...</action>`;
- latest response: `content` is empty, `finish_reason=stop`,
  `closure_mode=Unclosed`, and `completion_tokens=485`;
- latest parse: `status=fault`, `content_bytes=0`, and
  `error=MissingActionEnvelope`;
- latest export manifest lists `admission.json` and `observation.txt`, but both
  files are absent in the latest turn directory.

The checked-in active log does not prove JSON-in-`files` refusal, oversized
README refusal, or successful creation under the Chronos root. Those remain
historical failure classes only when backed by separate fixtures or logs.

## Active Target

The dependency queue is [execution/current-blockers.md](execution/current-blockers.md).
The first open target is current-state reconciliation plus transition-kernel
handover. The first implementation target after that is the persisted kernel:

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
dispatch, recovery, compaction, maintenance, or close attempts. The protocol and
LLM layers must prevent live prompts from teaching `<think>` output, must
sanitize invalid assistant history before replay, and must classify empty
content with nonzero completion tokens as a provider anomaly instead of an
ordinary parse fault loop.

## Latest Local Evidence

Documentation reconciliation gates for this handoff slice:

- `cargo run -p lkjagent-xtask -- check-docs`: `CHECK_DOCS_EXIT=0`, `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `CHECK_LINES_EXIT=0`, `ok check-lines`.

Prompt hygiene, provider anomaly, and export-manifest focused gates:

- `cargo fmt --check`: `FMT_CHECK_EXIT=0`.
- `cargo test -p lkjagent-protocol`: `PROTOCOL_EXIT=0`.
- `cargo test -p lkjagent-llm`: `LLM_EXIT=0`.
- `cargo test -p lkjagent-context --test assemble`: `CONTEXT_ASSEMBLE_EXIT=0`.
- `cargo test -p lkjagent-runtime --test prompt_hygiene`: `PROMPT_HYGIENE_EXIT=0`.
- `cargo test -p lkjagent-runtime --test provider_anomaly`: `PROVIDER_ANOMALY_EXIT=0`.
  Provider anomalies now set endpoint retry state without parse-fault increments.
  Protocol parsing now accepts opening parameter tags that start content on the
  same line, such as `<content># Premise`, until the matching close tag. Adjacent
  repeated `<files>...</files>` wrappers for `fs.batch_write` merge into one
  delimiter payload. Startup graph-prefix rendering now keeps persisted
  completion-guard lines inside the graph-state prefix budget instead of
  crashing during compaction restart.
- `cargo test -p lkjagent-runtime --test current_model_run_fixture`: `CURRENT_MODEL_RUN_FIXTURE_EXIT=0`.
- `cargo test -p lkjagent-runtime --test provider_exchange_log`: `PROVIDER_EXCHANGE_LOG_EXIT=0`.
- `cargo test -p lkjagent-runtime --test kernel_admission`: `KERNEL_ADMISSION_EXIT=0`.
- `cargo test -p lkjagent-runtime --test pending_action_authority`: `PENDING_ACTION_AUTHORITY_EXIT=0`.
- `cargo test -p lkjagent-runtime --test authority_ledger_wiring`: `AUTHORITY_LEDGER_WIRING_EXIT=0`.
- `cargo test -p lkjagent-runtime --test kernel_prompt_render`: `KERNEL_PROMPT_RENDER_EXIT=0`.
- `cargo test -p lkjagent-tools --test artifact_address`: `ARTIFACT_ADDRESS_EXIT=0`.
- `cargo test -p lkjagent-runtime --test owner_guidance`: `OWNER_GUIDANCE_EXIT=0`.
- `cargo test -p lkjagent-benchmark --test corpus`: `BENCHMARK_CORPUS_TEST_EXIT=0`.
- `cargo run -p lkjagent-xtask -- benchmark check-corpus`: `BENCHMARK_EXIT=0`, `ok benchmark-corpus`.
- `cargo run -p lkjagent-xtask -- check-style`: `CHECK_STYLE_EXIT=0`, `ok check-style`.
- `cargo test -p lkjagent-runtime`: `RUNTIME_EXIT=0`.
- `cargo test -p lkjagent-cli --test model_log`: `CLI_MODEL_LOG_EXIT=0`.
- `cargo test -p lkjagent-cli --test model_log_archive`: `CLI_MODEL_LOG_ARCHIVE_EXIT=0`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.
- `docker compose run --rm verify`: `DOCKER_VERIFY_EXIT=0`, `ok verify`.

Full focused gate sweep also passed with exit 0 for `lkjagent-protocol`,
`lkjagent-llm`, runtime `kernel_model`, `kernel_prompt_render`,
`kernel_admission`, `authority_reducer`, `recovery_controller`,
`recovery_shape_enforcement`, `authority_ledger_wiring`, tools
`registry_examples`, `batch_write_formats`, `artifact_ledger_tools`,
`artifact_tools`, `doc_tools`, CLI `model_log`, doc/style/line checks,
benchmark corpus, quiet verify, and Docker verify.

The live Chronos story smoke has not run in this slice.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
