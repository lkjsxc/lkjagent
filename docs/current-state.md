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

Owner-reported failures are current evidence. The harness previously generated
semantically poor documentation files such as part-001.md and could loop on
action parameter faults such as unknown params [path]. The generic document
scaffold now has focused tests for semantic names, README indexes, exact-count
semantic files, graph manifests, and audit rejection of sequence-only names.
Safe action parameter drift now normalizes documented aliases and prints exact
valid examples for refusals.

The graph now has neutral ranked state tracks, an objective envelope that does
not copy raw owner text, SQLite track snapshots, graph notice rendering, and
status/console display tests. The transition-quality scoring exists as pure
code, but the runtime transition controller still does not consume it. Compact
token accounting and a single synthesized GPT handoff log remain open.

## Area Status

| Area | Status | Evidence |
| --- | --- | --- |
| Cargo workspace and gates | implemented | `Cargo.toml`; `crates/lkjagent-xtask` |
| Docker compose services | implemented | `docker-compose.yml` |
| Action parser | implemented | `crates/lkjagent-protocol` |
| Tool dispatcher | partially implemented | safe parameter normalization exists; runtime recovery is still shallow |
| Document scaffold tool | partially implemented | semantic scaffold tests pass; final gate not yet run |
| Document audit tool | partially implemented | topology and graph checks exist; final gate not yet run |
| Recursive document seed | partially implemented | deterministic tree uses semantic contract paths |
| State graph cases | partially implemented | ranked neutral tracks exist; controller integration remains shallow |
| Owner objective normalization | partially implemented | objective envelope exists; deeper multilingual extraction remains open |
| Runtime recovery | partially implemented | parse/tool recovery exists, parameter recovery is shallow |
| Context budgets | partially implemented | budget model exists, compact usage display is missing |
| Token usage ledger | not implemented | endpoint usage is not persisted as a ledger |
| Console/status accounting | partially implemented | ranked states display; token deck is incomplete |
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
