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

The latest `data/logs/current-model-run.md` remains active evidence. It shows a
Chronos Fracture story-bible run that entered recovery at
`document-completion-check`, recorded plan and directory-observation evidence,
but did not record document-structure or artifact-readiness evidence.

## Implemented Behavior

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Parser | `lkjagent-protocol` parses line-oriented, paired-tag, and batch file action forms covered by focused fixtures. The live parser uses `<action>`, rejects top-level JSON and prose outside the envelope, records implicit-envelope outcomes in provider logs and runtime notices, and emits dedicated attribute-like tag faults. Provider stop-closure restoration records closure mode for parse logs. |
| Dispatcher registry | `lkjagent-tools` validates registered tools, required-any groups, and renders registry examples for covered action families. `graph.plan` requires checks or paths before dispatch. |
| Graph model | `lkjagent-graph` stores typed cases, evidence requirements, ranked tracks, transitions, and completion decisions. |
| SQLite store | Queue, state, event, memory, task summary, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist in `lkjagent-store`. |
| Endpoint loop | The daemon calls a local endpoint, records token usage when present, and preserves unknown usage as unknown. |
| Model log | Status, console, and `lkjagent model-log` expose a provider-neutral current run snapshot. Provider exchange request, authority, response, timing, parse with envelope mode and text hash, admission, observation, error records, raw-case inspection, and replay export files are written when the daemon has a log root. |
| Document scaffold seed | Deterministic scaffold paths, relation-first generic seeds, bounded slugs, compact `catalog.toml`, and creative writing profiles exist for project, multi-topic docs, story, novel, character, and cookbook roots. |
| Document audit basics | Audit checks README topology, links, catalog coverage, path hygiene, line limits, workspace briefs, structure-only pages, owner-term pages, and old generated boilerplate leaves. |
| Placeholder and payload refusal | `fs.write`, `fs.batch_write`, content audit, and check-docs reject common scaffold phrases and oversized payloads before mutation. |
| Audit-owned evidence guard | Direct `graph.evidence` cannot satisfy `artifact-readiness` or `document-structure`. |
| Hard compaction mode | A runtime-owned `Compaction` active mode exists and does not render `memory.save` as a model action. |
| Stale-action preemption seed | Cached actions are refused before dispatch when owner queue, runtime-only, or maintenance authority fields change. |
| Baseline benchmarks | The corpus includes owner-reported recovery, artifact, memory, accounting, model-log, batch-schema, compaction, and repeated-recovery signatures. |

## Partially Implemented Behavior

| Area | Current truth |
| --- | --- |
| Runtime authority | Pure active-mode selection, `RuntimeMission` mapping, normalized authority snapshots, event and decision rows, prompt-card decision ids, pending-action admission rows, immutable admission-view refusal for pending actions, owner-queue stale-action refusal, and `agent.done` refusal exist. A standalone `kernel` module defines pure snapshot, event, decision, admission, effect, render, fault, adapter, and reducer records with invariant tests. Kernel prompt rendering requires persisted event and decision ids and produces path-scoped batch examples. Kernel admission refuses stale, blocked, not-admitted, and completion-blocked tools before dispatch. The daemon now records authority prompt frames and effect observations tied to decision/admission rows, and cached actions refuse on prompt-frame head changes. These are partial authority pieces, not one daemon-wired transition kernel. |
| State-transition contracts | Snapshot, event, decision, admission, transition, artifact ledger, compaction history, fan-out, and index-network contracts are documented. Full unified runtime wiring remains open. |
| Recovery controller | Fault notices, attribute-like tag repair examples, recovery graph routes, escape-tool visibility, repeat refusal, route metadata, pure recovery plans, dispatcher-valid examples for covered routes, SQLite retry counts, and repeated batch-schema shape change to `artifact.next` exist. Live shape-change enforcement for every fault class remains open. |
| Schema repair | Safe alias normalization and registry examples exist for covered cases. `fs.batch_write` now normalizes safe path-shaped unknown parameters into `files` and refuses absolute, duplicate, or empty-content path parameters before mutation. Runtime route changes after repeated schema faults remain open. |
| Artifact lifecycle | Scaffold, audit, fact-only `artifact.next`, story semantic readiness checks, bounded write examples, root-scoped cursors, root/path address refusals, explicit `.md` directory invalid-root repair output, normalized artifact ledger and cursor APIs, invalid-root markers, and daemon `agent.done` refusal for unresolved ledger weak paths exist. Adoption repair and close-path proof remain incomplete. |
| Completion gates | A pure completion reducer returns completion kind, failed gates, missing evidence, existing evidence, current artifact, next action, valid example, blocked-handoff allowance, and status text. Artifact-readiness refusal names the current artifact when authority supplies it; graph-only fallback now uses `graph.state` instead of generic roots. Every close path is not yet proven to call the same artifact-aware gate. |
| Compaction resumability | Compaction records graph, recovery, artifact, batch cursor, last-observation, and next-action fields in notices and writes pre/post graph compaction snapshot rows. CLI status now renders the latest compaction snapshot fields for the active case. Prompt-frame resume proof remains open. |
| Maintenance | Idle maintenance, no-op cooldown, exact duplicate deletion, high-overlap merge, and low-signal rewrite pruning exist. Owner preemption before endpoint and every dispatch path remains open. |
| Provider exchange logging | Store schema, APIs, atomic file writer, per-turn export files, CLI list/show, and focused tests exist. Sanitized archive export and raw case inspection remain open. |
| Benchmarks | Uploaded-run text signatures exist. Parser-level and runtime-level replay fixtures that assert productive next actions remain open. |

## Active Failure Evidence

`data/logs/current-model-run.md` proves the next reliability gap:

- the daemon refused a stale maintenance `memory.find` action after owner work
  appeared, so active-mode preemption is useful but narrow;
- the owner requested `stories/chronos-fracture` as a directory story artifact
  with README, catalog, child directories, line limits, small batches, and audits;
- the run created `stories/chronos-fracture` and
  `stories/chronos-fracture/bible`;
- the task then looped through missing action envelopes and consecutive parse
  faults reaching at least count 5;
- `fs.batch_write` was repeatedly refused because `files` was missing and a
  path-shaped unknown parameter such as
  `stories/chronos-fracture/catalog.toml` appeared instead; focused tool tests
  now cover safe normalization for that shape, but live runtime replay remains
  open;
- recovery examples were rendered, but the live route did not escape into a
  productive write or audit path;
- evidence contained plan and observation rows, not `document-structure` or
  `artifact-readiness`;
- no close path can be called implemented until quiet verify, Docker Compose
  verification, and a live or replayed Chronos story run prove this class does
  not recur.

## Active Target

The dependency queue is [execution/current-blockers.md](execution/current-blockers.md).
The first open target is documentation and current-state reconciliation. The
first implementation target after that is the transition-kernel contract and
data model: `RuntimeSnapshot + RuntimeEvent -> RuntimeDecision`, persisted
before prompt rendering or dispatch, with admission derived from the same
decision id. The protocol and LLM crates now use `<action>` as the singular live
action envelope; runtime authority wiring still needs to follow the kernel.

Route-level proof remains open for all admission paths. Stale-action refusal
must compare the full staleness fingerprint, not only maintenance mode.
Compaction and maintenance are runtime-owned decisions, not prompt inventions.
Completion must read the central completion reducer on every path.

## Latest Local Evidence

Baseline at the start of this reconciliation slice:

- `git status --short`: no output.
- `docs/current-state.md`: 200 lines before this rewrite.
- `data/logs/current-model-run.md`: 194 lines and still active evidence.
- `cargo run -p lkjagent-xtask -- check-docs`: `DOCS_EXIT=0`, `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `LINES_EXIT=0`, `ok check-lines`.

Protocol live-envelope focused gates:

- `cargo fmt --check`: `FMT_CHECK_EXIT=0`.
- `cargo test -p lkjagent-protocol`: `PROTOCOL_EXIT=0`.
- `cargo test -p lkjagent-llm`: `LLM_EXIT=0`.
- `cargo run -p lkjagent-xtask -- check-docs`: `CHECK_DOCS_EXIT=0`, `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `CHECK_LINES_EXIT=0`, `ok check-lines`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.
- Registry conditional slice: `cargo test -p lkjagent-protocol`: `PROTOCOL_EXIT=0`;
  `cargo test -p lkjagent-tools`: `TOOLS_EXIT=0`; focused registry tests passed.

Runtime-kernel data-model focused gates:

- `cargo fmt --check`: `FMT_CHECK_EXIT=0`.
- `cargo test -p lkjagent-runtime --test kernel_model`: `KERNEL_MODEL_EXIT=0`, 9 passed.
- `cargo test -p lkjagent-runtime --test authority_reducer`: `AUTHORITY_REDUCER_EXIT=0`, 7 passed.
- `cargo run -p lkjagent-xtask -- check-docs`: `CHECK_DOCS_EXIT=0`, `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `CHECK_LINES_EXIT=0`, `ok check-lines`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Snapshot-adapter focused gate:

- `cargo test -p lkjagent-runtime --test kernel_snapshot_adapter`: `KERNEL_SNAPSHOT_ADAPTER_EXIT=0`, 6 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Prompt-frame focused gate:

- `cargo test -p lkjagent-runtime --test kernel_prompt_render`: `KERNEL_PROMPT_RENDER_EXIT=0`, 4 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Admission focused gate:

- `cargo test -p lkjagent-runtime --test kernel_admission`: `KERNEL_ADMISSION_EXIT=0`, 5 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Batch-write schema focused gate:

- `cargo test -p lkjagent-tools --test batch_write_formats`: `BATCH_WRITE_FORMATS_EXIT=0`, 10 passed.
- `cargo test -p lkjagent-tools --test dispatch_normalize`: `DISPATCH_NORMALIZE_EXIT=0`, 6 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Recovery-shape focused gates:

- `cargo test -p lkjagent-runtime --test recovery_shape_enforcement`: `RECOVERY_SHAPE_EXIT=0`, 3 passed.
- `cargo test -p lkjagent-runtime --test authority_recovery_plan`: `AUTHORITY_RECOVERY_PLAN_EXIT=0`, 5 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Artifact-address focused gates:

- `cargo test -p lkjagent-tools --test artifact_address_invalid_root`: `ARTIFACT_ADDRESS_INVALID_ROOT_EXIT=0`, 1 passed.
- `cargo test -p lkjagent-tools --test artifact_address`: `ARTIFACT_ADDRESS_EXIT=0`, 6 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Completion-gate focused gates:

- `cargo test -p lkjagent-runtime --test completion_decision`: `COMPLETION_DECISION_EXIT=0`, 5 passed.
- `cargo test -p lkjagent-runtime --test authority_completion`: `AUTHORITY_COMPLETION_EXIT=0`, 5 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Status compaction focused gate:

- `cargo test -p lkjagent-cli --test status`: `CLI_STATUS_TEST_EXIT=0`, 6 passed.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

Latest broad local gates:

- `cargo run -p lkjagent-xtask -- check-style`: `CHECK_STYLE_EXIT=0`, `ok check-style`.
- `cargo run -p lkjagent-xtask -- benchmark check-corpus`: `BENCHMARK_EXIT=0`, `ok benchmark-corpus`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.
- `docker compose run --rm verify`: `DOCKER_VERIFY_EXIT=0`, `ok verify`.

Store-ledger focused gates:

- `cargo test -p lkjagent-store --test runtime_kernel_ledger`: `STORE_KERNEL_LEDGER_EXIT=0`, 3 passed.
- `cargo test -p lkjagent-store --test runtime_authority`: `STORE_AUTHORITY_EXIT=0`, 1 passed.
- `cargo test -p lkjagent-store`: `STORE_TEST_EXIT=0`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `QUIET_VERIFY_EXIT=0`, `ok verify`.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
