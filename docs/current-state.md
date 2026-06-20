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
that can conflict with graph policy.

The graph now has neutral ranked state tracks, an objective envelope that does
not copy raw owner text, SQLite track snapshots, graph notice rendering, and
status/console display tests. The transition-quality scoring exists as pure
code, but the runtime transition controller still does not consume it. Compact
token accounting is implemented for endpoint usage, status, and console. A
single synthesized GPT handoff log is implemented and exposed through status,
console, and `lkjagent gpt-log`. The benchmark corpus now includes the
owner-reported documentation, action recovery, accounting, and GPT log failure
cases with known-good and known-bad fixtures.
Docker Compose verification and a live smoke run previously passed for
semantic recursive documentation generation, graph manifest creation, compact
context/token status, and the single GPT handoff log. They do not prove the
newer runtime recovery controller behavior.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol` |
| Tool dispatcher | partially implemented | safe parameter normalization exists; graph.note examples and broader recovery refusals need correction |
| Document scaffold tool | implemented | semantic scaffold tests, quiet verify, compose smoke |
| Document audit tool | implemented | topology and graph checks pass local gates |
| Recursive document seed | implemented | deterministic tree writes README indexes and `.lkj-doc-graph.md` |
| State graph cases | partially implemented | ranked neutral tracks exist; controller integration remains shallow |
| Owner objective normalization | partially implemented | objective envelope exists; deeper multilingual extraction remains open |
| Runtime recovery | partially implemented | parse/tool/repeat recovery and `recover-params` exist; repeated diagnostics and controller-driven escape remain open |
| Context budgets | partially implemented | budget model and compact context display exist; forced compaction must stop depending on model-owned `memory.save` |
| Token usage ledger | implemented | endpoint usage is parsed, persisted, and preserves unknown fields |
| Console/status accounting | partially implemented | ranked states plus compact context/token deck and GPT path display; last successful action is still shallow |
| GPT handoff log | implemented | runtime and CLI write one current Markdown snapshot |
| Mechanical benchmarks | implemented | owner failure cases are covered by corpus fixtures and `benchmark check-corpus` passes |

## Open Work

The dependency queue is
[execution/current-blockers.md](execution/current-blockers.md). The main open
risk is runtime recovery controller integration: transition quality must drive
runtime state, repeated diagnostics must force a different route, compaction
must be policy-consistent, and large content tasks must use document
construction.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
