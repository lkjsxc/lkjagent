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
completion remains unreliable.

Owner-reported failures are current evidence. The harness can generate
semantically poor documentation files such as part-001.md and can loop on
action parameter faults such as unknown params [path]. The code confirms the
generic document scaffold still creates sequence-named child files, and the
generic recursive seed still contains release-shaped API paths that violate the
documentation standards.

The daemon also lacks enough neutral multi-state progress, compact token
accounting, and a single synthesized GPT handoff log for a stronger external
model. Current docs that imply these features are complete are stale until the
implementation and tests prove them.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol` |
| Tool dispatcher | partially implemented | strict parameter errors still lack robust recovery |
| Document scaffold tool | broken | `doc.scaffold` emits sequence-named child files |
| Document audit tool | partially implemented | checks README and counts, not full topology graph |
| Recursive document seed | partially implemented | deterministic tree exists with stale API-shaped paths |
| State graph cases | partially implemented | one case exists, but ranked neutral tracks are missing |
| Owner objective normalization | partially implemented | raw owner text can dominate task framing |
| Runtime recovery | partially implemented | parse/tool recovery exists, parameter recovery is shallow |
| Context budgets | partially implemented | budget model exists, compact usage display is missing |
| Token usage ledger | not implemented | endpoint usage is not persisted as a ledger |
| Console/status accounting | partially implemented | status exists, token deck is incomplete |
| GPT handoff log | not implemented | no single current Markdown handoff file |
| Mechanical benchmarks | partially implemented | owner failure cases are not fully covered |

## Open Work

The dependency queue is
[execution/current-blockers.md](execution/current-blockers.md). The first
active slice is to make the documentation contract honest, then replace the
semantic document scaffold and audit behavior.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
