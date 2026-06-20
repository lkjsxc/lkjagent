# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
broken, and what remains open. A behavior is implemented only when code,
focused tests, and gates prove it.

## Summary

The repository contains a Rust workspace, documentation contracts, local gates,
a tag-based action parser, graph case state, context budgeting, SQLite storage,
an endpoint client, tool dispatch, a daemon loop, and CLI surfaces for queue,
status, log, console, and memory.

The parser, dispatcher, graph, context engine, store, endpoint client, and
queue exist. The implementation is not yet safe for the uploaded GPT-5.5-Pro
failure logs. Runtime completion remains unsafe for long content tasks,
maintenance can loop and duplicate records, recovery can suggest or allow
actions that active policy refuses, and compaction can contradict graph policy.
Structured record identity is not yet sufficient to prevent duplicate
knowledge artifacts.

Owner-reported failures are current evidence. The harness previously generated
semantically poor documentation files such as part-001.md and could loop on
action parameter faults such as unknown params [path]. Newer logs show broader
system failure: repeated parse faults, invalid parameter loops, invalid
`graph.note` kinds, invalid `memory.save` kinds, duplicate memory entries,
blocked `memory.save` during maintenance, blocked `doc.scaffold` during
recovery, repeated `graph.next`, premature `agent.done`, and content tasks that
completed without the requested story or cookbook artifact.

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
and Docker Compose verification have not been rerun for this broader redesign.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol` |
| Tool dispatcher | partially implemented | generated examples parse, validate, and dispatch for graph plan/note/evidence, memory save, fs.stat, and doc.scaffold; maintenance dispatch now suppresses graph policy, broader active-mode loop integration remains open |
| Document scaffold tool | implemented | semantic project, story, and cookbook scaffold tests pass; quiet verify from prior controller work |
| Document audit tool | implemented | topology checks pass local gates; runtime records document-structure only for passed audits |
| Recursive document seed | implemented | deterministic tree writes README indexes and `.lkj-doc-graph.md`; content-artifact routing now uses semantic roots for long stories and cookbooks |
| Memory save and find | partially implemented | accepted kinds, duplicate skip, and punctuation-safe FTS queries have focused tests; maintenance pruning remains open |
| State graph cases | implemented | ranked neutral tracks and pure transition selection drive recovery and post-event graph refresh; refusal examples now use admitted transition targets |
| Owner objective normalization | partially implemented | objective envelope exists; deeper multilingual extraction remains open |
| Runtime recovery | partially implemented | pure active-mode selection exists, maintenance/compaction modes do not render graph-policy refusals, internal `agent.ask` questions are refused, and runtime close rechecks graph completion; repeated invalid actions still need full deterministic recovery control |
| Context budgets | partially implemented | budget model and compact context display exist; forced compaction is runtime-owned and preserves active graph/fault state; richer structured snapshots remain open |
| Token usage ledger | implemented | endpoint usage is parsed, persisted, and preserves unknown fields |
| Console/status accounting | partially implemented | ranked states plus compact context/token deck and GPT path display; last successful action is still shallow |
| GPT handoff log | implemented | runtime and CLI write one current Markdown snapshot |
| Mechanical benchmarks | partially implemented | uploaded loop fixtures and judges exist and corpus passes; final Docker gate remains open |

## Open Work

The dependency queue is
[execution/current-blockers.md](execution/current-blockers.md). The open work is
structural, not only endpoint quality: active-mode selection, maintenance
idempotency, compaction ownership, semantic artifact planning, completion
readiness, and benchmark coverage must move before the ledger can call the
loop redesign implemented.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
