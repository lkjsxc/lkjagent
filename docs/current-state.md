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
| Parser | `lkjagent-protocol` parses paired-tag, line-oriented, and batch file action forms covered by focused fixtures. |
| Dispatcher registry | `lkjagent-tools` validates registered tools and renders registry examples for covered action families. |
| Graph model | `lkjagent-graph` stores typed cases, evidence requirements, ranked tracks, transitions, and completion decisions. |
| SQLite store | Queue, state, event, memory, and task summary surfaces exist in `lkjagent-store`. |
| Endpoint loop | The daemon calls a local endpoint, records token usage when present, and preserves unknown usage as unknown. |
| Model log | Status, console, and `lkjagent model-log` expose a provider-neutral current model run snapshot. |
| Document scaffold seed | Deterministic scaffold paths exist for project, lkjagent-model, multi-topic docs, story, and cookbook roots. |
| Document audit basics | Audit checks README topology, links, `.lkj-doc-graph.md`, line limits, and scaffold-only leaves. |
| Placeholder refusal | `fs.write`, `fs.batch_write`, and content audit reject common scaffold and placeholder phrase classes before mutation. |
| Audit-owned evidence guard | Direct `graph.evidence` cannot satisfy `artifact-readiness` or `document-structure`. |
| Hard compaction mode | A runtime-owned `Compaction` active mode exists and does not render `memory.save` as a model action. |
| Baseline benchmarks | The corpus includes owner-reported recovery, artifact, memory, accounting, and model-log signatures. |

## Partially Implemented Behavior

| Area | Current truth |
| --- | --- |
| Runtime authority | Pure active-mode selection, turn authority cards, effective dispatch policy reuse, stale maintenance-action refusal before dispatch, and `agent.done` refusal exist. Durable authority snapshots and broader pre-dispatch contradiction repair remain open. |
| Recovery controller | Fault notices, recovery graph routes, escape-tool visibility, repeat refusal, closed fault classes, route metadata, and a pure recovery plan table exist. Durable retry counts and live shape-change enforcement for every fault class remain open. |
| Schema repair | Safe alias normalization and registry examples exist for covered cases. Every rendered recovery example is not yet proven through one canonical registry validation path. |
| Artifact lifecycle | Scaffold, audit, `artifact.next`, bounded write examples, and root-scoped cursors exist. Artifact identity, adoption, weak-path repair state, and semantic readiness are incomplete. |
| Completion gates | Runtime completion refusals keep cases open for many artifact and evidence gaps. Every close path is not yet proven to use the same artifact-aware gate. |
| Compaction resumability | Compaction records graph, recovery, artifact, and next-action fields in notices. The durable snapshot still lacks richer last-successful observation and batch cursor state. |
| Maintenance | Idle maintenance, no-op cooldown, exact duplicate deletion, and same-title high-overlap merge exist. Rewrite pruning and pre-dispatch owner preemption remain open. |
| Status and console | Active graph state, context pressure, token usage, and model-log paths display. Last successful observation and exact authority snapshot details remain shallow. |
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

This session ran these gates after the documentation and recovery-model edits:

- `cargo run -p lkjagent-xtask -- check-docs`: `ok check-docs`.
- `cargo run -p lkjagent-xtask -- check-lines`: `ok check-lines`.
- `cargo run -p lkjagent-xtask -- check-style`: `ok check-style`.
- `cargo fmt --check`: passed.
- `cargo test -p lkjagent-runtime`: passed after the stale maintenance-action refusal test.
- `cargo run -p lkjagent-xtask -- quiet verify`: `ok verify`.
- `docker compose run --rm verify`: `ok verify`.

These gates prove the documentation shape, line cap, style scan, formatting,
runtime focused crate tests, workspace quiet verification, and Docker Compose
verification for this slice. They do not prove the full redesign; the blocker
queue remains open until each row has focused runtime coverage.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
