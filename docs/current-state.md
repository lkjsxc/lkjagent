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
| Parser | `lkjagent-protocol` parses line-oriented, paired-tag, JSON envelope, and batch file action forms covered by focused fixtures. Provider stop-closure restoration records closure mode for parse logs. |
| Dispatcher registry | `lkjagent-tools` validates registered tools and renders registry examples for covered action families. |
| Graph model | `lkjagent-graph` stores typed cases, evidence requirements, ranked tracks, transitions, and completion decisions. |
| SQLite store | Queue, state, event, memory, task summary, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist in `lkjagent-store`. |
| Endpoint loop | The daemon calls a local endpoint, records token usage when present, and preserves unknown usage as unknown. |
| Model log | Status, console, and `lkjagent model-log` expose a provider-neutral current run snapshot. Provider exchange request, authority, response, timing, parse, admission, observation, and error records are written when the daemon has a log root. |
| Document scaffold seed | Deterministic scaffold paths, relation-first generic seeds, bounded slugs, compact `catalog.toml`, and creative writing profiles exist for project, multi-topic docs, story, novel, character, and cookbook roots. |
| Document audit basics | Audit checks README topology, links, catalog coverage, path hygiene, line limits, workspace briefs, structure-only pages, owner-term pages, and old generated boilerplate leaves. |
| Placeholder and payload refusal | `fs.write`, `fs.batch_write`, content audit, and check-docs reject common scaffold phrases and oversized payloads before mutation. |
| Audit-owned evidence guard | Direct `graph.evidence` cannot satisfy `artifact-readiness` or `document-structure`. |
| Hard compaction mode | A runtime-owned `Compaction` active mode exists and does not render `memory.save` as a model action. |
| Maintenance preemption seed | Cached maintenance actions are refused before dispatch when queued owner work changes current authority. |
| Baseline benchmarks | The corpus includes owner-reported recovery, artifact, memory, accounting, model-log, batch-schema, compaction, and repeated-recovery signatures. |

## Partially Implemented Behavior

| Area | Current truth |
| --- | --- |
| Runtime authority | Pure active-mode selection, `RuntimeMission` mapping, normalized authority snapshots, event and decision rows, prompt-card decision ids, pending-action admission rows, immutable admission-view refusal for pending actions, stale maintenance-action refusal, and `agent.done` refusal exist. A standalone `kernel` module defines pure snapshot, event, decision, admission, effect, render, fault, adapter, and reducer records with invariant tests. Kernel prompt rendering requires a persisted decision id and produces path-scoped batch examples. Kernel admission refuses stale, blocked, not-admitted, and completion-blocked tools before dispatch. Store rows now cover prompt frames and observations with foreign-key proof for admissions. These are partial authority pieces, not one daemon-wired transition kernel. |
| State-transition contracts | Snapshot, event, decision, admission, transition, artifact ledger, compaction history, fan-out, and index-network contracts are documented. Full unified runtime wiring remains open. |
| Recovery controller | Fault notices, recovery graph routes, escape-tool visibility, repeat refusal, route metadata, pure recovery plans, dispatcher-valid examples for covered routes, SQLite retry counts, and repeated batch-schema shape change to `artifact.next` exist. Live shape-change enforcement for every fault class remains open. |
| Schema repair | Safe alias normalization and registry examples exist for covered cases. `fs.batch_write` now normalizes safe path-shaped unknown parameters into `files` and refuses absolute, duplicate, or empty-content path parameters before mutation. Runtime route changes after repeated schema faults remain open. |
| Artifact lifecycle | Scaffold, audit, `artifact.next`, bounded write examples, root-scoped cursors, root/path address refusals, normalized artifact ledger and cursor APIs, invalid-root markers, and daemon `agent.done` refusal for unresolved ledger weak paths exist. Adoption repair and close-path proof remain incomplete. |
| Completion gates | A pure completion reducer returns completion kind, failed gates, missing evidence, existing evidence, next action, valid example, blocked-handoff allowance, and status text. Every close path is not yet proven to call the same artifact-aware gate. |
| Compaction resumability | Compaction records graph, recovery, artifact, batch cursor, last-observation, and next-action fields in notices and writes pre/post graph compaction snapshot rows. Status rendering and prompt-frame resume proof remain open. |
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
- the task then looped through missing `<act>` blocks and consecutive parse
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
decision id.

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

Post-edit gates for this documentation reconciliation slice:

- `cargo run -p lkjagent-xtask -- check-docs`: `DOCS_EXIT=0`, `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `LINES_EXIT=0`, `ok check-lines`.

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
