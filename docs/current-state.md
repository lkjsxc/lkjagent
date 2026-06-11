# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
design-only, and what comes next. Every change that moves behavior updates
this file in the same commit. Statuses used across docs: implemented,
design-only, not implemented, out of scope, open question.

## Summary

The repository contains the complete documentation contract and no Rust code.
Every runtime behavior described under [architecture/](architecture/README.md)
and [product/](product/README.md) is design-only. The implementation queue is
[execution/current-blockers.md](execution/current-blockers.md).

## Area Status

| Area | Status | Contract |
| --- | --- | --- |
| Documentation tree and policies | implemented | [repository/](repository/README.md) |
| Vision and scope | implemented | [vision/](vision/README.md) |
| Decision records | implemented | [decisions/](decisions/README.md) |
| Agent manual and skills | implemented | [agent/](agent/README.md) |
| Execution queue and tasks | implemented | [execution/](execution/README.md) |
| Cargo workspace and crates | design-only | [repository/layout.md](repository/layout.md) |
| Verification xtask and quiet gates | design-only | [operations/verification.md](operations/verification.md) |
| Docker compose services | design-only | [operations/compose.md](operations/compose.md) |
| Daemon and agent loop | design-only | [architecture/runtime/](architecture/runtime/README.md) |
| Context engine and compaction | design-only | [architecture/context/](architecture/context/README.md) |
| Action protocol and parser | design-only | [architecture/protocol/](architecture/protocol/README.md) |
| Toolset | design-only | [architecture/tools/](architecture/tools/README.md) |
| SQLite store and memory | design-only | [architecture/memory/](architecture/memory/README.md) |
| Skill runtime and library | design-only | [architecture/skills/](architecture/skills/README.md) |
| LLM endpoint client | design-only | [architecture/llm/](architecture/llm/README.md) |
| Container and sandbox | design-only | [architecture/sandbox/](architecture/sandbox/README.md) |
| User message queue and CLI | design-only | [product/](product/README.md) |
| Self-maintenance loop | design-only | [architecture/runtime/self-maintenance.md](architecture/runtime/self-maintenance.md) |

## Out of Scope

Messaging channels, web UI, MCP, sub-agents, plan mode, heartbeat schedules,
and cron schedules. The boundaries are stated in [vision/scope.md](vision/scope.md).

## Next Step

Take the first open blocker in
[execution/current-blockers.md](execution/current-blockers.md):
bootstrap the cargo workspace per
[execution/tasks/bootstrap-workspace.md](execution/tasks/bootstrap-workspace.md).

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
