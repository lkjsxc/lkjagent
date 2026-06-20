# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
broken, and what remains open. A behavior is implemented only when code,
focused tests, and gates prove it.

## Summary

The repository contains a Rust workspace, documentation contracts, local gates,
a tag-based action parser, graph case state, context budgeting, SQLite storage,
an endpoint client, tool dispatch, a daemon loop, and CLI surfaces for queue,
status, log, console, and memory. Those pieces exist, but owner-visible task
completion still depends on endpoint quality and graph-controller depth.

Owner-reported failures are current evidence. The harness previously generated
semantically poor documentation files such as part-001.md and could loop on
action parameter faults such as unknown params [path]. The generic document
scaffold now has focused tests for semantic names, README indexes, exact-count
semantic files, graph manifests, and audit rejection of sequence-only names.
Safe action parameter drift now normalizes documented aliases, prints exact
valid examples for refusals, and routes repeated parser-level parameter faults
through a dedicated `recover-params` node. A newer owner run shows broader
recovery failure: repeated parse faults, invalid recovery actions, invalid
`graph.note` kinds, blocked mutation during recovery, and compaction guidance
that conflicted with graph policy. Focused fixes now cover valid
`graph.note` examples, graph-selected recovery routing, diagnostic-loop
refusals, runtime-owned compaction, long-content document routing, oversized
write payload recovery, owner-question gating, and a benchmark regression for
the long-story recovery loop.

The graph now has neutral ranked state tracks, an objective envelope that does
not copy raw owner text, SQLite track snapshots, graph notice rendering,
transition selection, and status/console display tests. Runtime recovery and
normal post-event graph refresh routes consume the selector. Completion
refusals now produce structured partial handoffs instead of vague denials.
Compact token accounting is implemented for endpoint usage, status, and console. A
single synthesized GPT handoff log is implemented and exposed through status,
console, and `lkjagent gpt-log`. The benchmark corpus now includes the
owner-reported documentation, action recovery, recovery-loop, accounting, and
GPT log failure cases with known-good and known-bad fixtures. Quiet verification
and Docker Compose verification pass after the runtime recovery controller
changes. The optional live smoke run has not been rerun for those controller
changes.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol` |
| Tool dispatcher | partially implemented | safe parameter normalization, valid graph.note examples, and actionable graph-policy refusals exist; remaining policy depth depends on full controller integration |
| Document scaffold tool | implemented | semantic scaffold tests, quiet verify, compose smoke |
| Document audit tool | implemented | topology and graph checks pass local gates |
| Recursive document seed | implemented | deterministic tree writes README indexes and `.lkj-doc-graph.md` |
| State graph cases | implemented | ranked neutral tracks and pure transition selection drive recovery and post-event graph refresh |
| Owner objective normalization | partially implemented | objective envelope exists; deeper multilingual extraction remains open |
| Runtime recovery | implemented | parse/tool/repeat recovery, `recover-params`, selector routing, payload-risk recovery, repeated-diagnostic refusal, selector-driven post-event refresh, and partial completion handoff proof exist |
| Context budgets | partially implemented | budget model and compact context display exist; forced compaction is runtime-owned |
| Token usage ledger | implemented | endpoint usage is parsed, persisted, and preserves unknown fields |
| Console/status accounting | partially implemented | ranked states plus compact context/token deck and GPT path display; last successful action is still shallow |
| GPT handoff log | implemented | runtime and CLI write one current Markdown snapshot |
| Mechanical benchmarks | implemented | owner failure cases are covered by corpus fixtures and `benchmark check-corpus` passes |

## Open Work

The dependency queue is
[execution/current-blockers.md](execution/current-blockers.md). The remaining
open risk is live endpoint quality under real model output; repository gates and
Docker Compose verification pass for the deterministic controller changes.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
