# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
broken, and what remains open. A behavior is implemented only when code,
focused tests, and gates prove it.

## Summary

The core crates, local gates, parser, dispatcher, graph model, transition
scoring, context budgeting, SQLite storage, endpoint client, daemon loop,
queue intake, status, log, console, and memory CLI surfaces exist.

The implementation is not yet fully safe for the uploaded run logs. Runtime
transition control remains insufficient. Active mode selection is documented
in [architecture/runtime/active-mode/](architecture/runtime/active-mode/README.md)
but is not yet authoritative enough across owner work, recovery, maintenance,
compaction, and closed idle. Maintenance and graph policy can still contradict
each other in some paths. Compaction and graph policy must stay runtime-owned
so hard pressure never requires a blocked model `memory.save`.

Memory writes now skip exact equivalents, update same-title high-overlap rows,
merge same-title and maintenance no-op prune groups, and use punctuation-safe
search, but rewrite pruning remains incomplete. Long content tasks route toward
story, cookbook, and dictionary artifacts, and `artifact.next` can plan bounded
write batches from weak content paths with a root-scoped SQLite cursor. Direct
file writes and batch writes now reject scaffold phrases before mutation, but
semantic artifact identity and adoption need deeper runtime enforcement. The
artifact lifecycle contract is
[architecture/artifacts/lifecycle.md](architecture/artifacts/lifecycle.md).
Recovery can still block the tools required to escape a fault. Completion can
still be too close to planning evidence on non-artifact close paths. Visible
objective rendering must not show visible counter prefixes.

Owner-reported failures remain active evidence: repeated parse faults,
invalid `graph.plan`, blocked `doc.scaffold`, blocked `fs.write`, empty
cookbook roots, duplicate memory rows, FTS punctuation failures, policy
contradictions, false completion after scaffold-only output, repeated
`graph.next`, invalid note and memory kinds, compaction deadlocks, and owner
questions about internal tool uncertainty.

The bread dictionary and cookbook logs are benchmark fixtures, not content
authoring tasks. They prove systemic failures in runtime authority, recovery,
artifact adoption and repair, write batching, content-bearing completion,
maintenance preemption, compaction resumability, and evidence accounting.

The graph now has neutral ranked state tracks, an objective envelope that does
not copy raw owner text, SQLite track snapshots, graph notice rendering,
transition selection, and status/console display tests. Runtime recovery and
normal post-event graph refresh routes consume the selector. Completion
refusals now produce structured partial handoffs instead of vague denials.
Audit-owned evidence requirements now stay audit-derived: `graph.evidence`
cannot satisfy `artifact-readiness` or `document-structure`, and completion
refusals point to `artifact.audit` or `doc.audit` for those gates.
Hard compaction now selects a single `Compaction` active mode before queued
owner intake, recovery, or maintenance, and runtime authority examples for
model-call modes come from the dispatcher registry renderer instead of separate
hand-written strings.
Compact token accounting is implemented for endpoint usage, status, and console.
A single synthesized GPT handoff log is implemented and exposed through status,
console, and `lkjagent gpt-log`. The benchmark corpus now includes
owner-reported documentation, action recovery, recovery-loop, graph-plan
example, FTS punctuation, duplicate-memory, active-policy contradiction,
invalid-note-kind, bread-cookbook false-completion, accounting, and GPT log
failure cases with known-good and known-bad fixtures. Final quiet verification
and Docker Compose verification passed for this redesign slice on 2026-06-21.

## Active Redesign Target

The current target is a runtime-owned authority layer above graph suggestions,
model actions, maintenance, compaction, verification, and completion. The
target outcomes are:

1. Runtime-owned active mode and tool admission.
2. Deterministic recovery controller.
3. Content-bearing artifact completion gates.
4. Runtime-owned compaction snapshots.
5. Maintenance defers owner work.
6. Uploaded run-log fixtures are covered by mechanical benchmarks.
7. Docker Compose verification passes after the implementation slice.

The documentation contract now names the required authority, artifact,
recovery, maintenance, compaction, schema-repair, and uploaded-run evaluation
pages. Those pages are implementation requirements, not proof that the runtime
behavior is complete.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol`; child-tag file tool parameters have focused fixtures |
| Tool dispatcher | partially implemented | generated examples parse, validate, and dispatch for key graph, memory, fs, and doc tools; dispatch now checks one effective policy object before routing, including `agent.done` completion refusal; schema repair emits one canonical example for covered parameter and evidence-kind faults; audit-owned evidence refuses direct `graph.evidence` |
| Document scaffold tool | implemented | semantic project, story, and cookbook scaffold tests pass; artifact.apply reuses the planner and writer |
| Document audit tool | implemented | topology checks pass local gates; artifact.audit checks kind mismatch; content artifacts reject scaffold-only leaves, weak cookbook/story leaves, and shallow dictionary term lists |
| Artifact next batch | partially implemented | `artifact.next` returns exact weak paths, content-bearing `fs.batch_write` examples, and root-scoped cursor advancement; scaffold-only cookbook, cursor, and meaningful cookbook tests pass |
| Recursive document seed | implemented | deterministic tree writes README indexes and `.lkj-doc-graph.md`; content-artifact routing now uses semantic roots for long stories and cookbooks |
| Memory save and find | partially implemented | accepted kinds, duplicate skip, same-title overlap update, punctuation-safe FTS queries, exact duplicate prune, same-title prune merge, and maintenance no-op lesson merge have focused tests; rewrite pruning remains open |
| State graph cases | implemented | ranked neutral tracks and pure transition selection drive recovery and post-event graph refresh; refusal examples now use admitted transition targets |
| Owner objective normalization | partially implemented | objective envelope exists; deeper multilingual extraction remains open |
| Runtime recovery | partially implemented | pure turn authority selection, pure reducer/admission helpers, authority snapshot/event/decision types, store-backed runtime snapshots, one active-mode endpoint card, cached dispatch authority, recovery escape-tool visibility, batch-oriented recovery examples, and effective dispatch repair admissions exist; `graph.recover` now derives next actions from legal transitions, admitted tools, and plan readiness; internal `agent.ask` questions are refused, runtime close rechecks graph completion, content artifacts require separate audit-derived artifact-readiness evidence, and no-op maintenance defers restart; fault-class-specific deterministic recovery control remains open |
| Context budgets | partially implemented | budget model and compact context display exist; forced compaction is runtime-owned and renders mission, evidence gaps, artifact root/id, recovery step, last failed action, admitted tools, exact next action, and completion blockers; artifact.next root cursors exist, while richer cursor fields and last-successful-observation snapshots remain open |
| Token usage ledger | implemented | endpoint usage is parsed, persisted, and preserves unknown fields |
| Console/status accounting | partially implemented | ranked states plus compact context/token deck and GPT path display; last successful action is still shallow |
| GPT handoff log | implemented | runtime and CLI write one current Markdown snapshot |
| Mechanical benchmarks | partially implemented | uploaded loop fixtures and judges exist, including bread `artifact.next` recovery and shallow dictionary readiness; corpus, quiet verify, and Docker Compose verify passed for this slice |

## Open Work

The dependency queue is
[execution/current-blockers.md](execution/current-blockers.md). The open work is
structural, not only endpoint quality: endpoint-turn mode selection,
artifact adoption/repair, durable batch cursors, last-observation compaction
snapshots, and blocked handoffs need deeper coverage before the ledger can call
the loop redesign implemented. Final verification must be rerun after each
later slice.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
