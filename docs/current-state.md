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
and use punctuation-safe search, but semantic maintenance merge and rewrite
pruning remain incomplete. Long content tasks route toward story and cookbook
scaffolds, but semantic artifact identity, adoption, repair, and
content-bearing completion need deeper runtime enforcement. The artifact
lifecycle contract is
[architecture/artifacts/lifecycle.md](architecture/artifacts/lifecycle.md).
Recovery can still block the tools required to escape a fault. Completion can
still be too close to planning or scaffold evidence unless artifact audit
gates are enforced at every close path. Visible objective rendering must not
show visible counter prefixes.

Owner-reported failures remain active evidence: repeated parse faults,
invalid `graph.plan`, blocked `doc.scaffold`, blocked `fs.write`, empty
cookbook roots, duplicate memory rows, FTS punctuation failures, policy
contradictions, false completion after scaffold-only output, repeated
`graph.next`, invalid note and memory kinds, compaction deadlocks, and owner
questions about internal tool uncertainty.

The graph now has neutral ranked state tracks, an objective envelope that does
not copy raw owner text, SQLite track snapshots, graph notice rendering,
transition selection, and status/console display tests. Runtime recovery and
normal post-event graph refresh routes consume the selector. Completion
refusals now produce structured partial handoffs instead of vague denials.
Compact token accounting is implemented for endpoint usage, status, and console.
A single synthesized GPT handoff log is implemented and exposed through status,
console, and `lkjagent gpt-log`. The benchmark corpus now includes
owner-reported documentation, action recovery, recovery-loop, graph-plan
example, FTS punctuation, duplicate-memory, active-policy contradiction,
invalid-note-kind, bread-cookbook false-completion, accounting, and GPT log
failure cases with known-good and known-bad fixtures. Final quiet verification
and Docker Compose verification passed for this redesign slice on 2026-06-20.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol` |
| Tool dispatcher | partially implemented | generated examples parse, validate, and dispatch for key graph, memory, fs, and doc tools; dispatch now checks one effective policy object before routing |
| Document scaffold tool | implemented | semantic project, story, and cookbook scaffold tests pass; artifact.apply reuses the planner and writer |
| Document audit tool | implemented | topology checks pass local gates; artifact.audit checks kind mismatch; content artifacts reject scaffold-only leaves |
| Recursive document seed | implemented | deterministic tree writes README indexes and `.lkj-doc-graph.md`; content-artifact routing now uses semantic roots for long stories and cookbooks |
| Memory save and find | partially implemented | accepted kinds, duplicate skip, same-title overlap update, punctuation-safe FTS queries, and exact duplicate prune have focused tests; semantic merge and rewrite pruning remain open |
| State graph cases | implemented | ranked neutral tracks and pure transition selection drive recovery and post-event graph refresh; refusal examples now use admitted transition targets |
| Owner objective normalization | partially implemented | objective envelope exists; deeper multilingual extraction remains open |
| Runtime recovery | partially implemented | pure active-mode selection exists, dispatch uses one effective policy per turn, internal `agent.ask` questions are refused, runtime close rechecks graph completion, and no-op maintenance defers restart; repeated invalid actions still need deterministic recovery control |
| Context budgets | partially implemented | budget model and compact context display exist; forced compaction is runtime-owned and preserves active graph and fault state; richer structured snapshots remain open |
| Token usage ledger | implemented | endpoint usage is parsed, persisted, and preserves unknown fields |
| Console/status accounting | partially implemented | ranked states plus compact context/token deck and GPT path display; last successful action is still shallow |
| GPT handoff log | implemented | runtime and CLI write one current Markdown snapshot |
| Mechanical benchmarks | partially implemented | uploaded loop fixtures and judges exist; corpus check passes; final quiet and Docker Compose verification must run for the current change |

## Open Work

The dependency queue is
[execution/current-blockers.md](execution/current-blockers.md). The open work is
structural, not only endpoint quality: endpoint-turn mode selection,
semantic maintenance pruning, compaction snapshots, artifact adoption/repair,
blocked handoffs, and final verification must move before the ledger can call
the loop redesign implemented.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
