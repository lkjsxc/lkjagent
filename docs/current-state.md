# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
partial, and what is open. A behavior is implemented only when code, focused
tests, and gates prove it.

## Summary

lkjagent has a working Rust workspace with local gates, Docker Compose gates,
a tag action parser, tool dispatcher, typed graph model, context budgeting,
SQLite persistence, endpoint client, daemon loop, owner queue intake, status,
log, console, memory commands, and benchmark fixtures.

The open problem is controller reliability. Runtime authority is not yet the
single durable source for every active mode, dispatch decision, recovery route,
compaction snapshot, maintenance boundary, and completion gate. Uploaded run
logs remain active evidence until focused tests and Docker Compose verification
prove that their failures cannot recur.

## Implemented Behavior

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Diagnostic runtime output | `data/workspace` and `data/logs` are tracked for the current handoff; SQLite store files stay ignored. |
| Parser | `lkjagent-protocol` parses line-oriented, paired-tag, JSON envelope, and batch file action forms covered by focused fixtures. |
| Dispatcher registry | `lkjagent-tools` validates registered tools and renders registry examples for covered action families. |
| Graph model | `lkjagent-graph` stores typed cases, evidence requirements, ranked tracks, transitions, and completion decisions. |
| SQLite store | Queue, state, event, memory, and task summary surfaces exist in `lkjagent-store`. |
| Endpoint loop | The daemon calls a local endpoint, records token usage when present, and preserves unknown usage as unknown. |
| Model log | Status, console, and `lkjagent model-log` expose a provider-neutral current model run snapshot. |
| Document scaffold seed | Deterministic scaffold paths and compact `catalog.toml` metadata exist for project, multi-topic docs, story, and cookbook roots. |
| Document audit basics | Audit checks README topology, links, catalog coverage, path hygiene, line limits, and scaffold-only leaves. |
| Placeholder and payload refusal | `fs.write`, `fs.batch_write`, and content audit reject common scaffold phrases and oversized payloads before mutation. |
| Audit-owned evidence guard | Direct `graph.evidence` cannot satisfy `artifact-readiness` or `document-structure`. |
| Hard compaction mode | A runtime-owned `Compaction` active mode exists and does not render `memory.save` as a model action. |
| Baseline benchmarks | The corpus includes owner-reported recovery, artifact, memory, accounting, and model-log signatures. |

## Partially Implemented Behavior

| Area | Current truth |
| --- | --- |
| Runtime authority | Pure active-mode selection, `RuntimeMission` mapping, turn authority cards, adapter-built runtime snapshots, mission-derived active modes, data-first decision records, normalized authority snapshot, event, decision, transition, effect, and admission store APIs, turn-authority snapshot, event, decision, and transition persistence, prompt-card decision id and fingerprint rendering, pending-action admission persistence, pending-action immutable admission-view refusal, effective dispatch policy reuse, store-backed authority snapshot fields, stale maintenance-action refusal before dispatch, and `agent.done` refusal exist. These are partial authority pieces, not yet one transition kernel. Broader route coverage and pre-dispatch contradiction repair remain open. |
| State-transition contracts | Snapshot, event, decision, admission, transition, artifact ledger, compaction history, fan-out, and index-network contracts are documented. The transition-network contract is partially implemented where current tests prove it; normalized history exists for turn authority snapshots and decisions, while full unified runtime wiring remains open. |
| Recovery controller | Fault notices, recovery graph routes, escape-tool visibility, repeat refusal, closed fault classes, route metadata, a pure recovery plan table, recovery-plan examples that parse, validate, admit, and dispatch to local routes, and SQLite retry counts keyed by case, node, tool, parameter shape, and fault class exist. Live shape-change enforcement for every fault class remains open. |
| Schema repair | Safe alias normalization and registry examples exist for covered cases. Registry examples parse, validate, and dispatch to routes except heavyweight verification gates. Recovery-plan examples parse, validate, are admitted by recovery policy, and dispatch to local routes when model-authored. |
| Artifact lifecycle | Scaffold, audit, `artifact.next`, bounded write examples, root-scoped cursors, normalized artifact ledger and cursor store APIs, ledger writes from `artifact.plan`, `artifact.apply`, `artifact.audit`, and `artifact.next`, successful write-path cursor completion marking, audit output `artifact_ledger_id`, and daemon `agent.done` refusal for unresolved ledger weak paths exist. Full close-path coverage remains incomplete. |
| Completion gates | A pure `decide_completion` reducer returns completion kind, failed gates, missing and existing evidence, next action, valid example, blocked-handoff allowance, and status text. Runtime `agent.done` admission uses it, and daemon graph dispatch checks the artifact ledger before admitting completion. Every close path is not yet proven to use the same artifact-aware gate. |
| Compaction resumability | Compaction records graph, recovery, artifact, batch cursor, last-observation, and next-action fields in notices and writes pre/post graph compaction snapshot rows. Store reopen coverage and status rendering for latest snapshots remain open. |
| Maintenance | Idle maintenance, no-op cooldown, exact duplicate deletion, same-title high-overlap merge, and low-signal rewrite pruning exist. Pre-dispatch owner preemption proof remains open. |
| Status and console | Active graph state, active mode, authority snapshot fields, context pressure, token usage, and model-log paths display. Last successful observation is summarized from recent observations. |
| Benchmarks | Uploaded-run signatures are represented by deterministic fixtures. Runtime replay coverage and every completion path are not yet complete. |

## Open Failure Evidence

Uploaded run logs still stand for these failures:

- repeated parse faults: missing action block and unclosed action roots.
- invalid parameters: absolute `/docs`, invalid `graph.plan`, invalid note and memory kinds.
- repeated recovery actions: `graph.recover`, `graph.state`, and `graph.next` loops.
- blocked escape tools: `doc.scaffold`, `fs.write`, `shell.run`, and internal `agent.ask` attempts.
- contradictory policy: maintenance and graph policy rendering together.
- content failure: README-only or scaffold-only output treated as artifact success.
- artifact drift: empty cookbook roots, shallow dictionary roots, and topic drift.
- memory failure: duplicate or low-value maintenance rows.
- compaction failure: hard pressure depending on model-authored `memory.save`.
- completion failure: `agent.done` closing without audit, verification, or recovery evidence.

## Active Target

The active target is a runtime-owned authority layer above model output, graph
suggestions, maintenance, compaction, owner questions, verification, and
completion. Model output is intent. Graph transitions are guidance. The
runtime decides one active mode, one effective policy, one admitted action
surface, one recovery route, and one completion gate per turn.

## Dependency Queue

The implementation queue is
[execution/current-blockers.md](execution/current-blockers.md). Rows remain
open until the task contract names focused evidence and the same slice records
the gates that actually ran. Contract text alone never closes a row.

## Latest Local Evidence

Latest focused slice gates:

- `cargo test -p lkjagent-store --test memory_prune`: `STORE_MEMORY_PRUNE_EXIT=0`.
- `cargo test -p lkjagent-tools --test memory_prune`: `TOOLS_MEMORY_PRUNE_EXIT=0`.
- `cargo test -p lkjagent-store`: `STORE_TEST_EXIT=0`.
- `cargo test -p lkjagent-tools`: `TOOLS_TEST_EXIT=0`.
- `cargo fmt --check`: `FMT_EXIT=0`.
- `cargo run -p lkjagent-xtask -- check-docs`: `ok check-docs`, `DOCS_EXIT=0`.
- `cargo run -p lkjagent-xtask -- check-lines`: `ok check-lines`, `LINES_EXIT=0`.
- `cargo run -p lkjagent-xtask -- quiet verify`: `ok verify`, `VERIFY_EXIT=0`.

Recent committed slices also ran focused authority, recovery, artifact,
compaction, `cargo fmt --check`, `check-docs`, `check-lines`, and quiet verify
gates before commit. Docker Compose verification is not current for these slices.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
